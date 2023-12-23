use std::{path::{PathBuf, Path}, fs::read, collections::HashMap};

use discoverer::discover;
use pxp_ast::{functions::{FunctionStatement, ConcreteMethod, AbstractMethod, ConcreteConstructor, AbstractConstructor}, namespaces::{UnbracedNamespace, BracedNamespace}, classes::{ClassStatement, ClassExtends, ClassImplements}, UseStatement, Use, GroupUseStatement, UseKind, constant::ClassishConstant, modifiers::Visibility, properties::{Property, VariableProperty}, interfaces::{InterfaceStatement, InterfaceExtends}};
use pxp_parser::parse;
use pxp_span::Span;
use pxp_symbol::{SymbolTable, Symbol};
use pxp_type::Type;
use pxp_visitor::{Visitor, walk_function, walk_braced_namespace, walk_unbraced_namespace, walk_class, walk_use, walk_group_use, walk_concrete_method, walk_interface, walk_abstract_method, walk_concrete_constructor, walk_abstract_constructor};

use crate::{index::Index, FunctionEntity, ParameterEntity, Location, ClassLikeEntity, ClassishConstantEntity, PropertyEntity, MethodEntity};

#[derive(Debug, Clone)]
pub struct Indexer {
    index: Index,
    symbol_table: SymbolTable,
    scope: Scope,
}

#[derive(Debug, Clone, Default)]
struct Scope {
    file: String,
    namespace: Option<Symbol>,
    // HashMap<Alias | Name, (Normal | Function | Const, Name)>
    uses: HashMap<Symbol, (UseKind, Symbol)>,
    current_class_like: ClassLikeEntity,
}

impl Scope {
    pub fn namespace(&self) -> Option<&Symbol> {
        self.namespace.as_ref()
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn add_use(&mut self, alias_or_name: Symbol, maps_to: (UseKind, Symbol)) {
        self.uses.insert(alias_or_name, maps_to);
    }
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            index: Index::default(),
            symbol_table: SymbolTable::default(),
            scope: Scope::default(),
        }
    }

    pub fn index(&mut self, directories: Vec<PathBuf>) -> (Index, SymbolTable) {
        let files = discover(&["php"], &directories.iter().map(|d| d.to_str().unwrap()).collect::<Vec<&str>>()).unwrap();

        for file in files {
            self.index_file(file);
        }

        (self.index.clone(), self.symbol_table.clone())
    }

    fn index_file(&mut self, file: PathBuf) {
        let contents = read(&file).unwrap();
        let mut program = parse(&contents, &mut self.symbol_table);

        self.scope = Scope::default();
        self.scope.file = file.to_str().unwrap().to_string();
        self.visit(&mut program.ast);
    }

    fn qualify(&mut self, symbol: Symbol) -> Symbol {
        // FIXME: Check for uses here.

        if let Some(namespace) = self.scope.namespace() {
            self.symbol_table.coagulate(&[*namespace, symbol], Some(b"\\"))
        } else {
            symbol
        }
    }

    pub fn of(index: Index, symbol_table: SymbolTable) -> Self {
        Self { index, symbol_table, scope: Scope::default() }
    }
}

impl Visitor for Indexer {
    fn visit_unbraced_namespace(&mut self, node: &mut UnbracedNamespace) {
        self.scope.namespace = Some(node.name.token.symbol.unwrap());
        walk_unbraced_namespace(self, node);
        self.scope.namespace = None;
    }

    fn visit_braced_namespace(&mut self, node: &mut BracedNamespace) {
        if let Some(name) = &node.name {
            self.scope.namespace = Some(name.token.symbol.unwrap());
        }

        walk_braced_namespace(self, node);

        if node.name.is_some() {
            self.scope.namespace = None;
        }
    }

    fn visit_use(&mut self, node: &mut UseStatement) {
        for r#use in node.uses.iter() {
            let name = &r#use.name.token.symbol.unwrap();
            let kind = match &r#use.kind {
                Some(kind) => kind.clone(),
                None => node.kind.clone(),
            };

            self.scope.add_use(r#use.alias.clone().map_or(*name, |alias| alias.token.symbol.unwrap()), (kind, *name));
        }

        walk_use(self, node)
    }

    fn visit_group_use(&mut self, node: &mut GroupUseStatement) {
        walk_group_use(self, node)
    }

    fn visit_function(&mut self, node: &mut FunctionStatement) {
        let mut function = FunctionEntity::default();

        let short_name = node.name.token.symbol.unwrap();
        function.name = self.qualify(short_name);
        function.short_name = short_name;

        let mut parameters = Vec::new();

        for parameter in node.parameters.iter() {
            let mut p = ParameterEntity::default();
            p.name = parameter.name.token.symbol.unwrap();
            p.reference = parameter.ampersand.is_some();
            p.variadic = parameter.ellipsis.is_some();
            p.optional = parameter.default.is_some();
            p.r#type = if let Some(r#type) = &parameter.data_type {
                r#type.clone()
            } else {
                Type::Mixed(Span::default())
            };

            parameters.push(p);
        }

        function.parameters = parameters;
        function.return_type = if let Some(return_type) = &node.return_type {
            return_type.data_type.clone()
        } else {
            Type::Mixed(Span::default())
        };

        function.location = Location::new(self.scope.file().to_string(), Span::new(node.name.token.span.start, node.body.right_brace.end));

        self.index.add_function(function);

        walk_function(self, node);
    }

    fn visit_interface(&mut self, node: &mut InterfaceStatement) {
        let mut interface = ClassLikeEntity::default();
        interface.is_interface = true;

        let name = node.name.token.symbol.unwrap();
        interface.name = self.qualify(name);
        interface.short_name = name;
        interface.r#final = false;
        interface.r#abstract = false;
        interface.r#readonly = false;

        if let Some(InterfaceExtends { parents, .. }) = &node.extends {
            interface.extends = parents.iter().map(|p| self.qualify(p.token.symbol.unwrap())).collect();
        }

        self.scope.current_class_like = interface;
        walk_interface(self, node);

        let mut interface = self.scope.current_class_like.clone();
        interface.location = Location::new(self.scope.file().to_string(), Span::new(node.name.token.span.start, node.body.right_brace.end));

        self.index.add_class_like(interface);
    }

    fn visit_class(&mut self, node: &mut ClassStatement) {
        let mut class = ClassLikeEntity::default();
        class.is_class = true;

        let name = node.name.token.symbol.unwrap();
        class.name = self.qualify(name);
        class.short_name = name;
        class.r#final = node.modifiers.has_final();
        class.r#abstract = node.modifiers.has_abstract();
        class.r#readonly = node.modifiers.has_readonly();

        if let Some(ClassExtends { parent, .. }) = &node.extends {
            class.extends = vec![self.qualify(parent.token.symbol.unwrap())];
        }

        if let Some(ClassImplements { interfaces, .. }) = &node.implements {
            class.implements = interfaces.iter().map(|i| self.qualify(i.token.symbol.unwrap())).collect();
        }

        self.scope.current_class_like = class;
        walk_class(self, node);

        let mut class = self.scope.current_class_like.clone();
        class.location = Location::new(self.scope.file().to_string(), Span::new(node.name.token.span.start, node.body.right_brace.end));
        self.index.add_class_like(class);
    }

    fn visit_classish_constant(&mut self, node: &mut ClassishConstant) {
        let r#final = node.modifiers.has_final();
        let visibility = node.modifiers.visibility();

        for entry in node.entries.iter() {
            let mut entity = ClassishConstantEntity::default();

            entity.name = entry.name.token.symbol.unwrap();
            entity.r#final = r#final;
            entity.visibility = visibility;

            self.scope.current_class_like.constants.push(entity);
        }
    }

    fn visit_property(&mut self, node: &mut Property) {
        let visibility = node.modifiers.visibility();
        let r#static = node.modifiers.has_static();
        let r#type = node.r#type.clone().unwrap_or_default();

        for property in node.entries.iter() {
            let mut entity = PropertyEntity::default();
            entity.name = property.variable().token.symbol.unwrap();
            entity.visibility = visibility;
            entity.r#static = r#static;
            entity.r#type = r#type.clone();
            entity.default = property.is_initialized();

            self.scope.current_class_like.properties.push(entity);
        }
    }

    fn visit_abstract_method(&mut self, node: &mut AbstractMethod) {
        let mut entity = MethodEntity::default();
        entity.name = node.name.token.symbol.unwrap();
        entity.visibility = node.modifiers.visibility();
        entity.r#static = node.modifiers.has_static();
        entity.r#abstract = false;
        entity.r#virtual = self.scope.current_class_like.is_interface;
        entity.r#final = node.modifiers.has_final();

        let mut parameters = Vec::new();

        for parameter in node.parameters.iter() {
            let mut p = ParameterEntity::default();
            p.name = parameter.name.token.symbol.unwrap();
            p.reference = parameter.ampersand.is_some();
            p.variadic = parameter.ellipsis.is_some();
            p.optional = parameter.default.is_some();
            p.r#type = if let Some(r#type) = &parameter.data_type {
                r#type.clone()
            } else {
                Type::Mixed(Span::default())
            };

            parameters.push(p);
        }

        entity.parameters = parameters;

        entity.return_type = if let Some(return_type) = &node.return_type {
            return_type.data_type.clone()
        } else {
            Type::Mixed(Span::default())
        };

        self.scope.current_class_like.methods.push(entity);

        walk_abstract_method(self, node);
    }

    fn visit_concrete_method(&mut self, node: &mut ConcreteMethod) {
        let mut entity = MethodEntity::default();
        entity.name = node.name.token.symbol.unwrap();
        entity.visibility = node.modifiers.visibility();
        entity.r#static = node.modifiers.has_static();
        entity.r#abstract = false;
        entity.r#final = node.modifiers.has_final();

        let mut parameters = Vec::new();

        for parameter in node.parameters.iter() {
            let mut p = ParameterEntity::default();
            p.name = parameter.name.token.symbol.unwrap();
            p.reference = parameter.ampersand.is_some();
            p.variadic = parameter.ellipsis.is_some();
            p.optional = parameter.default.is_some();
            p.r#type = if let Some(r#type) = &parameter.data_type {
                r#type.clone()
            } else {
                Type::Mixed(Span::default())
            };

            parameters.push(p);
        }

        entity.parameters = parameters;

        entity.return_type = if let Some(return_type) = &node.return_type {
            return_type.data_type.clone()
        } else {
            Type::Mixed(Span::default())
        };

        self.scope.current_class_like.methods.push(entity);

        walk_concrete_method(self, node);
    }

    fn visit_concrete_constructor(&mut self, node: &mut ConcreteConstructor) {
        let mut entity = MethodEntity::default();
        entity.name = node.name.token.symbol.unwrap();
        entity.visibility = node.modifiers.visibility();
        entity.r#static = node.modifiers.has_static();
        entity.r#abstract = false;
        entity.r#final = node.modifiers.has_final();

        let mut parameters = Vec::new();

        for parameter in node.parameters.parameters.iter() {
            let mut p = ParameterEntity::default();
            p.name = parameter.name.token.symbol.unwrap();
            p.reference = parameter.ampersand.is_some();
            p.variadic = parameter.ellipsis.is_some();
            p.optional = parameter.default.is_some();
            p.r#type = if let Some(r#type) = &parameter.data_type {
                r#type.clone()
            } else {
                Type::Mixed(Span::default())
            };

            // Indicates that this is a promoted property.
            if !parameter.modifiers.is_empty() {
                let mut property = PropertyEntity::default();
                property.name = parameter.name.token.symbol.unwrap();
                property.visibility = Visibility::Public;
                property.r#static = false;
                property.r#type = p.r#type.clone();
                property.default = true;

                self.scope.current_class_like.properties.push(property);
            }

            parameters.push(p);
        }

        entity.parameters = parameters;
        entity.return_type = Type::Void(Span::default());

        self.scope.current_class_like.methods.push(entity);

        walk_concrete_constructor(self, node);
    }

    fn visit_abstract_constructor(&mut self, node: &mut AbstractConstructor) {
        let mut entity = MethodEntity::default();
        entity.name = node.name.token.symbol.unwrap();
        entity.visibility = node.modifiers.visibility();
        entity.r#static = node.modifiers.has_static();
        entity.r#abstract = !self.scope.current_class_like.is_interface;
        entity.r#virtual = self.scope.current_class_like.is_interface;
        entity.r#final = node.modifiers.has_final();

        let mut parameters = Vec::new();

        for parameter in node.parameters.parameters.iter() {
            let mut p = ParameterEntity::default();
            p.name = parameter.name.token.symbol.unwrap();
            p.reference = parameter.ampersand.is_some();
            p.variadic = parameter.ellipsis.is_some();
            p.optional = parameter.default.is_some();
            p.r#type = if let Some(r#type) = &parameter.data_type {
                r#type.clone()
            } else {
                Type::Mixed(Span::default())
            };

            // Indicates that this is a promoted property.
            if !parameter.modifiers.is_empty() {
                let mut property = PropertyEntity::default();
                property.name = parameter.name.token.symbol.unwrap();
                property.visibility = Visibility::Public;
                property.r#static = false;
                property.r#type = p.r#type.clone();
                property.default = true;

                self.scope.current_class_like.properties.push(property);
            }

            parameters.push(p);
        }

        entity.parameters = parameters;
        entity.return_type = Type::Void(Span::default());

        self.scope.current_class_like.methods.push(entity);

        walk_abstract_constructor(self, node);
    }

    fn visit_variable_property(&mut self, node: &mut VariableProperty) {
        let visibility = Visibility::Public;
        let r#static = false;
        let r#type = node.r#type.clone().unwrap_or_default();

        for property in node.entries.iter() {
            let mut entity = PropertyEntity::default();
            entity.name = property.variable().token.symbol.unwrap();
            entity.visibility = visibility;
            entity.r#static = r#static;
            entity.r#type = r#type.clone();
            entity.default = property.is_initialized();

            self.scope.current_class_like.properties.push(entity);
        }
    }

    // FIXME: Walk rest of classish members.
    // FIXME: Walk enum cases too.
}
