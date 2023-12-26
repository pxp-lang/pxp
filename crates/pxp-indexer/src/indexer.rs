use std::{
    collections::HashMap,
    fs::read,
    path::PathBuf,
};

use discoverer::discover;
use pxp_ast::{
    classes::{ClassExtends, ClassImplements, ClassStatement},
    constant::{ClassishConstant, ConstantStatement},
    enums::{BackedEnumCase, BackedEnumStatement, UnitEnumCase, UnitEnumStatement},
    functions::{
        AbstractConstructor, AbstractMethod, ConcreteConstructor, ConcreteMethod, FunctionStatement,
    },
    identifiers::{Identifier, SimpleIdentifier},
    interfaces::{InterfaceExtends, InterfaceStatement},
    literals::Literal,
    namespaces::{BracedNamespace, UnbracedNamespace},
    properties::{Property, VariableProperty},
    traits::{TraitStatement, TraitUsage},
    ExpressionKind, FunctionCallExpression, GroupUseStatement, UseKind, UseStatement,
};
use pxp_parser::parse;
use pxp_span::Span;
use pxp_symbol::{Symbol, SymbolTable};
use pxp_syntax::visibility::Visibility;
use pxp_token::{Token, TokenKind};
use pxp_type::Type;
use pxp_visitor::{
    walk_abstract_constructor, walk_abstract_method, walk_backed_enum, walk_braced_namespace,
    walk_class, walk_concrete_constructor, walk_concrete_method, walk_function, walk_group_use,
    walk_interface, walk_trait, walk_trait_usage, walk_unbraced_namespace, walk_unit_enum,
    walk_use, Visitor,
};

use crate::{
    index::Index, ClassLikeEntity, ClassishConstantEntity, ConstantEntity, FunctionEntity,
    Location, MethodEntity, ParameterEntity, PropertyEntity,
};

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

    pub fn index(&mut self, directory: &PathBuf) -> (Index, SymbolTable) {
        let files = discover(
            &["php"],
            &[directory.to_str().unwrap().to_string().as_str()]
        )
        .unwrap();

        for file in files {
            self.index_file(file);
        }

        (self.index.clone(), self.symbol_table.clone())
    }

    fn index_file(&mut self, file: PathBuf) {
        if !self.index.should_index_file(&file) {
            return;
        }

        let contents = read(&file).unwrap();
        let mut program = parse(&contents, &mut self.symbol_table);

        self.scope = Scope::default();
        self.scope.file = file.to_str().unwrap().to_string();
        self.visit(&mut program.ast);

        self.index.add_file(file);
    }

    fn qualify(&mut self, symbol: Symbol, token: Token) -> Symbol {
        // Fully-qualified identifiers don't need qualification, so we can
        // just return the symbol as-is.
        if token.kind == TokenKind::FullyQualifiedIdentifier {
            return symbol;
        }

        // If the symbol isn't qualified, i.e. Foo, then we need to check all of the
        // "uses" in the current scope to see if any of them map to Foo. If they do, then
        // we can return the symbol mapped to the "use".
        //
        // This only works for unqualified identifiers as qualified identifiers require some byte manipulation.
        // It's quite naive but does the job.
        if token.kind == TokenKind::Identifier {
            if let Some((_, qualified)) = self.scope.uses.get(&symbol) {
                return *qualified;
            }
        }

        // If we have a qualified identifier, i.e. A\B\C, then we need to check if the first part of the
        // identifier is a "use" in the current scope, i.e. use F\A. If it is, then we can join the
        // two symbols together to form the fully-qualified symbol.
        if token.kind == TokenKind::QualifiedIdentifier {
            // We first grab the symbol that we're trying to qualify, i.e. A\B\C.
            let bytes = self.symbol_table.resolve(symbol).unwrap().to_bytestring();
            // We then need to split the symbol into its constituent parts, i.e. [A, B, C].
            let split = bytes.split(|b| *b == b'\\').collect::<Vec<&[u8]>>();
            // We then need to grab the first part of the identifier, i.e. A.
            // This is the part that we want to check against the "uses" in the current scope.
            let first = split.first().unwrap();
            // We can try to intern the first part of the identifier to get a symbol.
            // Use statements should be interned before any other statements can use them,
            // so we should always get a symbol back.
            let first_symbol = self.symbol_table.intern(first);

            // If the first part of the identifier is a "use" in the current scope, i.e. use F\A, then
            // we can join the two symbols together (omitting the first part of the identifier) to form
            // the fully-qualified symbol.
            if let Some((_, qualified)) = self.scope.uses.get(&first_symbol) {
                // We can grab the string that represents the import.
                let mut qualified = self
                    .symbol_table
                    .resolve(*qualified)
                    .unwrap()
                    .to_bytestring();
                // We can then grab the rest of the identifier, i.e. [B, C].
                let rest = split
                    .iter()
                    .skip(1)
                    .map(|b| b.to_vec())
                    .collect::<Vec<Vec<u8>>>()
                    .join(&b"\\"[..]);
                // We can then append the rest of the identifier to the import.
                qualified.extend(b"\\");
                qualified.extend(&rest);
                // We can then intern the fully-qualified identifier to get a symbol.
                return self.symbol_table.intern(&qualified);
            }
        }

        if let Some(namespace) = self.scope.namespace() {
            self.symbol_table
                .coagulate(&[*namespace, symbol], Some(b"\\"))
        } else {
            symbol
        }
    }

    pub fn with_symbol_table(symbol_table: SymbolTable) -> Self {
        Self {
            index: Index::default(),
            symbol_table,
            scope: Scope::default(),
        }
    }

    pub fn of(index: Index, symbol_table: SymbolTable) -> Self {
        Self {
            index,
            symbol_table,
            scope: Scope::default(),
        }
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

            // If there is an alias present, we can use that since that will be a SimpleIdentifier.
            // If there's no alias, then we need to generate a SimpleIdentifier from the imported name, i.e. A\B\C => C.
            let key = if let Some(alias) = &r#use.alias {
                alias.token.symbol.unwrap()
            } else {
                // We grab the name from the symbol table. This does involve heap-allocation
                // the ByteStr into a ByteString, but it'll do for now.
                let symbol = self.symbol_table.resolve(*name).unwrap().to_bytestring();
                // We split the name on the backslash and grab the last part, i.e. A\B\C => C.
                let split = symbol.split(|b| *b == b'\\');
                let last = split.last().unwrap();
                // We create a new symbol from the last part.
                self.symbol_table.intern(last)
            };

            self.scope.add_use(key, (kind, *name));
        }

        walk_use(self, node)
    }

    fn visit_group_use(&mut self, node: &mut GroupUseStatement) {
        walk_group_use(self, node)
    }

    fn visit_constant(&mut self, node: &mut ConstantStatement) {
        for entry in node.entries.iter() {
            let mut constant = ConstantEntity::default();
            constant.name = entry.name.token.symbol.unwrap();
            // FIXME: Add some simple type inference here.
            constant.r#type = Type::Mixed(Span::default());
            constant.location = Location::new(
                self.scope.file().to_string(),
                Span::new(entry.name.token.span.start, entry.value.span.end),
            );

            self.index.add_constant(constant);
        }
    }

    fn visit_function_call(&mut self, node: &mut FunctionCallExpression) {
        // We only care about calls to the define() function with at least one argument.
        if node.arguments.arguments.is_empty() {
            return;
        }

        // We only care about calls to the define() function.
        if let ExpressionKind::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
            token,
        })) = &node.target.kind
        {
            let symbol = self.symbol_table.resolve(token.symbol.unwrap()).unwrap();

            if symbol != b"define" {
                return;
            }

            let mut constant = ConstantEntity::default();
            let name_argument = node.arguments.arguments.first().unwrap();

            if let ExpressionKind::Literal(Literal { token, .. }) = name_argument.get_value().kind {
                let name = self
                    .symbol_table
                    .resolve(token.symbol.unwrap())
                    .unwrap()
                    .to_bytestring();
                // We need to remove the quotes from the name.
                // FIXME: This is a bit of a hack, but it'll do for now.
                let name = &name[1..name.len() - 1];
                let name_symbol = self.symbol_table.intern(name);

                constant.name = name_symbol;
                constant.short_name = name_symbol;
                constant.r#type = Type::Mixed(Span::default());
                constant.location = Location::new(
                    self.scope.file().to_string(),
                    Span::new(node.target.span.start, node.arguments.right_parenthesis.end),
                );

                self.index.add_constant(constant);
            } else {
                return;
            }
        }
    }

    fn visit_function(&mut self, node: &mut FunctionStatement) {
        let mut function = FunctionEntity::default();

        let short_name = node.name.token.symbol.unwrap();
        function.name = self.qualify(short_name, node.name.token);
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

        function.location = Location::new(
            self.scope.file().to_string(),
            Span::new(node.name.token.span.start, node.body.right_brace.end),
        );

        self.index.add_function(function);

        walk_function(self, node);
    }

    fn visit_unit_enum(&mut self, node: &mut UnitEnumStatement) {
        let mut enumeration = ClassLikeEntity::default();
        enumeration.is_enum = true;

        let name = node.name.token.symbol.unwrap();
        enumeration.name = self.qualify(name, node.name.token);
        enumeration.short_name = name;

        for implements in node.implements.iter() {
            let name = self.qualify(implements.token.symbol.unwrap(), implements.token);
            enumeration.implements.push(name);
        }

        self.scope.current_class_like = enumeration;
        walk_unit_enum(self, node);

        let mut enumeration = self.scope.current_class_like.clone();
        enumeration.location = Location::new(
            self.scope.file().to_string(),
            Span::new(node.name.token.span.start, node.body.right_brace.end),
        );

        self.index.add_class_like(enumeration);
    }

    fn visit_unit_enum_case(&mut self, node: &mut UnitEnumCase) {
        self.scope
            .current_class_like
            .cases
            .push(node.name.token.symbol.unwrap());
    }

    fn visit_backed_enum(&mut self, node: &mut BackedEnumStatement) {
        let mut enumeration = ClassLikeEntity::default();
        enumeration.is_enum = true;
        enumeration.backing_type = node.backed_type.clone();

        let name = node.name.token.symbol.unwrap();
        enumeration.name = self.qualify(name, node.name.token);
        enumeration.short_name = name;

        for implements in node.implements.iter() {
            let name = self.qualify(implements.token.symbol.unwrap(), implements.token);
            enumeration.implements.push(name);
        }

        self.scope.current_class_like = enumeration;
        walk_backed_enum(self, node);

        let mut enumeration = self.scope.current_class_like.clone();
        enumeration.location = Location::new(
            self.scope.file().to_string(),
            Span::new(node.name.token.span.start, node.body.right_brace.end),
        );

        self.index.add_class_like(enumeration);
    }

    fn visit_backed_enum_case(&mut self, node: &mut BackedEnumCase) {
        self.scope
            .current_class_like
            .cases
            .push(node.name.token.symbol.unwrap());
    }

    fn visit_trait(&mut self, node: &mut TraitStatement) {
        let mut trait_ = ClassLikeEntity::default();
        trait_.is_trait = true;

        let name = node.name.token.symbol.unwrap();
        trait_.name = self.qualify(name, node.name.token);
        trait_.short_name = name;
        trait_.r#final = false;
        trait_.r#abstract = false;
        trait_.r#readonly = false;

        self.scope.current_class_like = trait_;
        walk_trait(self, node);

        let mut trait_ = self.scope.current_class_like.clone();
        trait_.location = Location::new(
            self.scope.file().to_string(),
            Span::new(node.name.token.span.start, node.body.right_brace.end),
        );

        self.index.add_class_like(trait_);
    }

    fn visit_interface(&mut self, node: &mut InterfaceStatement) {
        let mut interface = ClassLikeEntity::default();
        interface.is_interface = true;

        let name = node.name.token.symbol.unwrap();
        interface.name = self.qualify(name, node.name.token);
        interface.short_name = name;
        interface.r#final = false;
        interface.r#abstract = false;
        interface.r#readonly = false;

        if let Some(InterfaceExtends { parents, .. }) = &node.extends {
            interface.extends = parents
                .iter()
                .map(|p| self.qualify(p.token.symbol.unwrap(), p.token))
                .collect();
        }

        self.scope.current_class_like = interface;
        walk_interface(self, node);

        let mut interface = self.scope.current_class_like.clone();
        interface.location = Location::new(
            self.scope.file().to_string(),
            Span::new(node.name.token.span.start, node.body.right_brace.end),
        );

        self.index.add_class_like(interface);
    }

    fn visit_class(&mut self, node: &mut ClassStatement) {
        let mut class = ClassLikeEntity::default();
        class.is_class = true;

        let name = node.name.token.symbol.unwrap();
        class.name = self.qualify(name, node.name.token);
        class.short_name = name;
        class.r#final = node.modifiers.has_final();
        class.r#abstract = node.modifiers.has_abstract();
        class.r#readonly = node.modifiers.has_readonly();

        if let Some(ClassExtends { parent, .. }) = &node.extends {
            class.extends = vec![self.qualify(parent.token.symbol.unwrap(), parent.token)];
        }

        if let Some(ClassImplements { interfaces, .. }) = &node.implements {
            class.implements = interfaces
                .iter()
                .map(|i| self.qualify(i.token.symbol.unwrap(), i.token))
                .collect();
        }

        self.scope.current_class_like = class;
        walk_class(self, node);

        let mut class = self.scope.current_class_like.clone();
        class.location = Location::new(
            self.scope.file().to_string(),
            Span::new(node.name.token.span.start, node.body.right_brace.end),
        );
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

    fn visit_trait_usage(&mut self, node: &mut TraitUsage) {
        for r#use in node.traits.iter() {
            let name = self.qualify(r#use.token.symbol.unwrap(), r#use.token);

            self.scope.current_class_like.uses.push(name);
        }

        walk_trait_usage(self, node);
    }

    // FIXME: Walk rest of classish members.
    // FIXME: Walk enum cases too.
}
