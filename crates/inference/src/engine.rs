use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use pxp_ast::{
    visitor::{walk_expression, Visitor},
    *,
};
use pxp_bytestring::{ByteStr, ByteString};
use pxp_index::{Index, ReflectionClass, ReflectionFunctionLike};
use pxp_token::TokenKind;
use pxp_type::{ConstExpr, Type};
use visitor::{
    walk_array_expression, walk_concat_expression, walk_die_expression, walk_empty_expression,
    walk_error_suppress_expression, walk_eval_expression, walk_exit_expression,
    walk_function_call_expression, walk_function_closure_creation_expression,
    walk_function_statement, walk_include_expression, walk_include_once_expression,
    walk_instanceof_expression, walk_isset_expression, walk_method_call_expression,
    walk_method_closure_creation_expression, walk_new_expression,
    walk_nullsafe_method_call_expression, walk_parenthesized_expression, walk_print_expression,
    walk_reference_expression, walk_require_expression, walk_require_once_expression,
    walk_static_method_call_expression, walk_unset_expression,
};

use crate::TypeMap;

/// The `TypeEngine` is responsible for generating a `TypeMap` for a given AST.
/// It uses the provided `Index` to resolve types for method calls, property accesses, etc.
pub struct TypeEngine<'a> {
    index: &'a Index,
}

impl<'a> TypeEngine<'a> {
    /// Create a new `TypeEngine` with the provided `Index`.
    pub fn new(index: &'a Index) -> Self {
        TypeEngine { index }
    }

    /// Infer the types for the given AST and return a `TypeMap`.
    pub fn infer(&self, ast: &[Statement]) -> TypeMap {
        let mut map = TypeMap::new();

        let mut generator = TypeMapGenerator {
            map: &mut map,
            index: self.index,
            scopes: ScopeStack::new(),
        };

        generator.visit(ast);
        map
    }
}

struct TypeMapGenerator<'a> {
    map: &'a mut TypeMap,
    index: &'a Index,
    scopes: ScopeStack,
}

struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    fn start(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn start_enclosed(&mut self) {
        self.scopes.push(self.current().enclose());
    }

    fn end(&mut self) {
        self.scopes.pop();
    }

    fn current(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn current_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scope {
    variables: HashMap<ByteString, Type<ResolvedName>>,
    outer: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            outer: None,
        }
    }

    fn enclose(&self) -> Self {
        Scope {
            variables: HashMap::new(),
            outer: Some(Rc::new(RefCell::new(self.clone()))),
        }
    }

    fn set_variable(&mut self, variable: &SimpleVariable, ty: Type<ResolvedName>) {
        self.variables.insert(variable.symbol.clone(), ty);
    }

    fn get_variable(&self, variable: &SimpleVariable) -> Option<Type<ResolvedName>> {
        if let Some(ty) = self.variables.get(&variable.symbol) {
            return Some(ty.clone());
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get_variable(variable);
        }

        None
    }
}

impl<'a> TypeMapGenerator<'a> {
    fn unwrap_data_type(&self, data_type: Option<&'a DataType>) -> Type<ResolvedName> {
        data_type.map_or(Type::Mixed, |ty| ty.get_type().clone())
    }

    fn is_newable_string(&self, value: &ByteStr) -> bool {
        self.index.get_class(value).is_some()
    }

    fn is_callable_string(&self, name: &ByteStr) -> bool {
        let name: &ByteStr = name[1..name.len() - 1].into();

        if name.contains(b"::") {
            todo!();
        }

        self.index.get_function(name).is_some()
    }

    fn determine_function_call_target_return_type(
        &self,
        target: &Expression,
    ) -> Type<ResolvedName> {
        match &target.kind {
            ExpressionKind::Name(name) => {
                self.get_function_call_target_return_type_from_name(name.as_ref())
            }
            ExpressionKind::Parenthesized(inner) => {
                self.determine_function_call_target_return_type(&inner.expr)
            }
            ExpressionKind::Closure(inner) => inner
                .return_type
                .as_ref()
                .map(|t| t.data_type.get_type().clone())
                .unwrap_or_else(|| Type::Mixed),
            ExpressionKind::Literal(inner) => match inner.kind {
                LiteralKind::String if self.is_callable_string(inner.token.symbol.as_ref()) => self
                    .get_function_call_target_return_type_from_callable_string(
                        inner.token.symbol.as_ref(),
                    ),
                _ => Type::Mixed,
            },
            // FIXME: Support other callable types here.
            _ => Type::Mixed,
        }
    }

    fn determine_class_from_type(&self, ty: &Type<ResolvedName>) -> Option<Vec<ReflectionClass>> {
        if !ty.is_object_like() {
            return None;
        }

        let mut classes = Vec::new();

        match ty {
            Type::Named(ResolvedName { resolved, .. }) => {
                match self.index.get_class(resolved.to_owned()) {
                    Some(class) => classes.push(class),
                    None => return None,
                }
            }
            Type::Nullable(inner) => return self.determine_class_from_type(inner),
            Type::Union(inners) | Type::Intersection(inners) => {
                classes.extend(
                    inners
                        .iter()
                        .filter_map(|inner| self.determine_class_from_type(inner))
                        .flatten(),
                );
            }
            Type::SelfReference | Type::StaticReference | Type::ParentReference => todo!(),
            _ => unreachable!(),
        };

        Some(classes)
    }

    fn get_function_call_target_return_type_from_callable_string(
        &self,
        name: &ByteStr,
    ) -> Type<ResolvedName> {
        let name: &ByteStr = name[1..name.len() - 1].into();

        // FIXME: Handle method calls.
        if name.contains(b"::") {
            return Type::Mixed;
        }

        match self.index.get_function(name) {
            Some(function) => function
                .get_return_type()
                .unwrap_or_else(|| &Type::Mixed)
                .clone(),
            None => Type::Mixed,
        }
    }

    fn get_function_call_target_return_type_from_name(&self, name: &Name) -> Type<ResolvedName> {
        match &name.kind {
            NameKind::Resolved(inner) => match self.index.get_function(inner.resolved.as_bytestr())
            {
                Some(function) => function
                    .get_return_type()
                    .unwrap_or_else(|| &Type::Mixed)
                    .clone(),
                None => Type::Mixed,
            },

            _ => todo!(),
        }
    }

    fn simplify_union(&self, mut types: Vec<Type<ResolvedName>>) -> Type<ResolvedName> {
        if types.len() == 1 {
            return types[0].clone();
        }

        let mut uniques = HashSet::new();

        types.retain(|ty| uniques.insert(ty.clone()));

        if types.len() == 1 {
            return types[0].clone();
        }

        Type::Union(types)
    }

    fn determine_array_type(&self, node: &ArrayExpression) -> Type<ResolvedName> {
        let value_types: Vec<Type<ResolvedName>> = node
            .items
            .iter()
            .filter_map(|item| -> Option<Type<ResolvedName>> {
                match item {
                    ArrayItem::Skipped(_) => None,
                    ArrayItem::Value(inner) => Some(self.map.resolve(inner.value.id).clone()),
                    ArrayItem::ReferencedValue(inner) => {
                        Some(self.map.resolve(inner.value.id).clone())
                    }
                    ArrayItem::SpreadValue(inner) => Some(self.map.resolve(inner.value.id).clone()),
                    ArrayItem::KeyValue(inner) => Some(self.map.resolve(inner.value.id).clone()),
                    ArrayItem::ReferencedKeyValue(inner) => {
                        Some(self.map.resolve(inner.value.id).clone())
                    }
                }
            })
            .collect();

        if node.is_list() {
            return Type::TypedArray(
                Box::new(Type::Integer),
                Box::new(self.simplify_union(value_types)),
            );
        }

        let key_types: Vec<Type<ResolvedName>> = node
            .items
            .iter()
            .map(|item| -> Type<ResolvedName> {
                match item {
                    ArrayItem::KeyValue(array_item_key_value) => {
                        self.map.resolve(array_item_key_value.key.id).clone()
                    }
                    ArrayItem::ReferencedKeyValue(array_item_referenced_key_value) => self
                        .map
                        .resolve(array_item_referenced_key_value.key.id)
                        .clone(),
                    _ => Type::Integer,
                }
            })
            .collect();

        Type::TypedArray(
            Box::new(self.simplify_union(key_types)),
            Box::new(self.simplify_union(value_types)),
        )
    }
}

impl<'a> Visitor for TypeMapGenerator<'a> {
    fn visit_expression(&mut self, node: &Expression) {
        walk_expression(self, node);

        let inner = self.map.resolve(node.kind.id()).clone();

        self.map.insert(node.id, inner);
    }

    fn visit_literal(&mut self, node: &Literal) {
        self.map.insert(
            node.id,
            match node.kind {
                LiteralKind::Integer => Type::Integer,
                LiteralKind::Float => Type::Float,
                LiteralKind::String => Type::LiteralString(
                    node.token
                        .symbol
                        .as_bytestr()
                        .strip_string_quotes()
                        .to_bytestring(),
                ),
                LiteralKind::Missing => Type::Missing,
            },
        )
    }

    fn visit_interpolated_string_expression(&mut self, node: &InterpolatedStringExpression) {
        self.map.insert(node.id, Type::String);
    }

    fn visit_bool_expression(&mut self, node: &BoolExpression) {
        self.map.insert(
            node.id,
            match node.value.kind {
                TokenKind::True => Type::True,
                TokenKind::False => Type::False,
                _ => Type::Boolean,
            },
        );
    }

    fn visit_function_call_expression(&mut self, node: &FunctionCallExpression) {
        // We need to walk the function call expression first to ensure all arguments are resolved
        // for conditional return types and resolving generics.
        //
        // We also need the information for the `node.target` to be available in the map.
        walk_function_call_expression(self, node);

        // FIXME: Once we've got this information, we can resolve generics based on the arguments.
        let return_type = self.determine_function_call_target_return_type(&node.target);

        self.map.insert(node.id, return_type);
    }

    fn visit_simple_variable(&mut self, node: &SimpleVariable) {
        if let Some(ty) = self.scopes.current().get_variable(node) {
            self.map.insert(node.id(), ty);
        }
    }

    fn visit_assignment_operation_expression(&mut self, node: &AssignmentOperationExpression) {
        // Walk the right-hand side of the assignment first to ensure the type is resolved.
        walk_expression(self, &node.right);

        // Assignment expressions are always resolved to the type of the right-hand side.
        self.map
            .insert(node.id, self.map.resolve(node.right.kind.id()).clone());

        // If the left-hand side is a variable, we can use that to assign the type in the current scope.
        match &node.left.kind {
            ExpressionKind::Variable(variable) if variable.is_simple() => {
                let variable = variable.to_simple();
                let resolved = self.map.resolve(node.right.kind.id());

                self.scopes
                    .current_mut()
                    .set_variable(&variable, resolved.clone());
                self.map.insert(variable.id, resolved.clone());
            }
            _ => return,
        }
    }

    fn visit_new_expression(&mut self, node: &NewExpression) {
        walk_new_expression(self, node);

        self.map.insert(
            node.id,
            match &node.target.kind {
                ExpressionKind::Name(name) => match true {
                    _ if name.is_resolved() => Type::Named(name.to_resolved().clone()),
                    _ => Type::Mixed,
                },
                _ => match self.map.resolve(node.target.id) {
                    Type::LiteralString(value) if self.is_newable_string(value.as_ref()) => {
                        Type::Named(ResolvedName {
                            resolved: value.clone(),
                            original: value.clone(),
                        })
                    }
                    _ => Type::Object,
                },
            },
        );
    }

    fn visit_array_expression(&mut self, node: &ArrayExpression) {
        walk_array_expression(self, node);

        // We've walked the array expression, so we can now figure out a more specific type for
        // the array, rather than it just returning `Type::Array`.
        self.map.insert(node.id, self.determine_array_type(node));
    }

    fn visit_function_statement(&mut self, node: &FunctionStatement) {
        self.scopes.start();
        walk_function_statement(self, node);
        self.scopes.end();
    }

    fn visit_function_parameter_list(&mut self, node: &FunctionParameterList) {
        for parameter in node.parameters.iter() {
            let mut r#type = self.unwrap_data_type(parameter.data_type.as_ref());

            if parameter.is_variadic() {
                r#type = Type::TypedArray(Box::new(Type::Integer), Box::new(r#type));
            }

            self.scopes
                .current_mut()
                .set_variable(&parameter.name, r#type);
        }
    }

    fn visit_missing_expression(&mut self, node: &MissingExpression) {
        self.map.insert(node.id, Type::Missing);
    }

    fn visit_eval_expression(&mut self, node: &EvalExpression) {
        walk_eval_expression(self, node);

        self.map.insert(node.id, Type::Mixed);
    }

    fn visit_empty_expression(&mut self, node: &EmptyExpression) {
        walk_empty_expression(self, node);

        // FIXME: We should be able to determine an exact true or false
        // here depending on the type of the value passed through.
        self.map.insert(node.id, Type::Boolean);
    }

    fn visit_die_expression(&mut self, node: &DieExpression) {
        walk_die_expression(self, node);

        self.map.insert(node.id, Type::Never);
    }

    fn visit_exit_expression(&mut self, node: &ExitExpression) {
        walk_exit_expression(self, node);

        self.map.insert(node.id, Type::Never);
    }

    fn visit_isset_expression(&mut self, node: &IssetExpression) {
        walk_isset_expression(self, node);

        // FIXME: We should be able to determine an exact true or false
        // depending on the expressions passed through.
        self.map.insert(node.id, Type::Boolean);
    }

    fn visit_unset_expression(&mut self, node: &UnsetExpression) {
        walk_unset_expression(self, node);

        self.map.insert(node.id, Type::Void);
    }

    fn visit_print_expression(&mut self, node: &PrintExpression) {
        walk_print_expression(self, node);

        self.map.insert(
            node.id,
            Type::ConstExpr(Box::new(ConstExpr::Integer(1.into()))),
        );
    }

    fn visit_concat_expression(&mut self, node: &ConcatExpression) {
        walk_concat_expression(self, node);

        // FIXME: We can be more precise here by checking the types on the
        // left and right-hand side of the expression, e.g. empty strings, etc.
        self.map.insert(node.id, Type::String);
    }

    fn visit_instanceof_expression(&mut self, node: &InstanceofExpression) {
        walk_instanceof_expression(self, node);

        // FIXME: Can we do some smart stuff here to determine the
        // real true / false state based on the left-hand side?
        self.map.insert(node.id, Type::Boolean);
    }

    fn visit_reference_expression(&mut self, node: &ReferenceExpression) {
        walk_reference_expression(self, node);

        self.map
            .insert(node.id, self.map.resolve(node.right.id).clone());
    }

    fn visit_parenthesized_expression(&mut self, node: &ParenthesizedExpression) {
        walk_parenthesized_expression(self, node);

        self.map
            .insert(node.id, self.map.resolve(node.expr.id).clone());
    }

    fn visit_error_suppress_expression(&mut self, node: &ErrorSuppressExpression) {
        walk_error_suppress_expression(self, node);

        self.map
            .insert(node.id, self.map.resolve(node.expr.id).clone());
    }

    fn visit_include_expression(&mut self, node: &IncludeExpression) {
        walk_include_expression(self, node);

        self.map.insert(node.id, Type::Mixed);
    }

    fn visit_include_once_expression(&mut self, node: &IncludeOnceExpression) {
        walk_include_once_expression(self, node);

        self.map.insert(node.id, Type::Mixed);
    }

    fn visit_require_expression(&mut self, node: &RequireExpression) {
        walk_require_expression(self, node);

        self.map.insert(node.id, Type::Mixed);
    }

    fn visit_require_once_expression(&mut self, node: &RequireOnceExpression) {
        walk_require_once_expression(self, node);

        self.map.insert(node.id, Type::Mixed);
    }

    fn visit_function_closure_creation_expression(
        &mut self,
        node: &FunctionClosureCreationExpression,
    ) {
        walk_function_closure_creation_expression(self, node);

        // FIXME: If the target is a function or if we can resolve the target to
        // something that resembles a callable, we can produce a better type here.
        self.map.insert(
            node.id,
            Type::Named(ResolvedName {
                resolved: ByteString::from("Closure"),
                original: ByteString::from("Closure"),
            }),
        );
    }

    fn visit_method_call_expression(&mut self, node: &MethodCallExpression) {
        walk_method_call_expression(self, node);

        let method_name = match &node.method.kind {
            ExpressionKind::Identifier(identifier) if identifier.is_simple() => {
                identifier.to_simple().symbol.as_bytestr()
            }
            // FIXME: Can we support dynamic method names here if we know the value of the expression?
            _ => {
                self.map.insert(node.id, Type::Mixed);

                return;
            }
        };

        let target = self.map.resolve(node.target.id);

        if !target.is_object_like() {
            self.map.insert(node.id, Type::Invalid);

            return;
        }

        if target.is_object() {
            self.map.insert(node.id, Type::Mixed);

            return;
        }

        // If we can't figure out what class-like thing we're calling the method on,
        // we'll just return a mixed type and continue on.
        let Some(classes) = self.determine_class_from_type(target) else {
            self.map.insert(node.id, Type::Mixed);
            return;
        };

        let methods = classes
            .iter()
            .filter_map(|class| class.get_method(method_name))
            .collect::<Vec<_>>();

        if methods.is_empty() {
            self.map.insert(node.id, Type::Mixed);

            return;
        }

        let return_type = self.simplify_union(
            methods
                .iter()
                .filter_map(|method| method.get_return_type().cloned())
                .collect::<Vec<Type<ResolvedName>>>(),
        );

        self.map.insert(node.id, return_type);
    }

    fn visit_method_closure_creation_expression(&mut self, node: &MethodClosureCreationExpression) {
        walk_method_closure_creation_expression(self, node);

        // FIXME: If we know what method is being called, we can determine a better type here.
        self.map.insert(
            node.id,
            Type::Named(ResolvedName {
                resolved: b"Closure".into(),
                original: b"Closure".into(),
            }),
        );
    }

    fn visit_nullsafe_method_call_expression(&mut self, node: &NullsafeMethodCallExpression) {
        walk_nullsafe_method_call_expression(self, node);

        let method_name = match &node.method.kind {
            ExpressionKind::Identifier(identifier) if identifier.is_simple() => {
                identifier.to_simple().symbol.as_bytestr()
            }
            // FIXME: Can we support dynamic method names here if we know the value of the expression?
            _ => {
                self.map.insert(node.id, Type::Mixed);

                return;
            }
        };

        let target = self.map.resolve(node.target.id);

        if !target.is_object_like() {
            self.map.insert(node.id, Type::Invalid);

            return;
        }

        if target.is_object() {
            self.map.insert(node.id, Type::Mixed);

            return;
        }

        // If we can't figure out what class-like thing we're calling the method on,
        // we'll just return a mixed type and continue on.
        let Some(classes) = self.determine_class_from_type(target) else {
            self.map.insert(node.id, Type::Mixed);
            return;
        };

        let methods = classes
            .iter()
            .filter_map(|class| class.get_method(method_name))
            .collect::<Vec<_>>();

        if methods.is_empty() {
            self.map.insert(node.id, Type::Mixed);

            return;
        }

        let return_type = self.simplify_union(
            methods
                .iter()
                .filter_map(|method| method.get_return_type().cloned())
                .collect::<Vec<Type<ResolvedName>>>(),
        );

        // FIXME: If we can determine that the thing we're calling isn't nullable, we can
        // omit the null type from the union.
        let return_type = self.simplify_union(vec![return_type, Type::Null]);

        self.map.insert(node.id, return_type);
    }

    fn visit_static_method_call_expression(&mut self, node: &StaticMethodCallExpression) {
        walk_static_method_call_expression(self, node);

        // FIXME: If we know that the target is a class-like thing, we can determine a better type here.
        let target = match &node.target.kind {
            ExpressionKind::Name(name) if name.is_resolved() => {
                name.to_resolved().resolved.as_ref()
            }
            _ => {
                self.map.insert(node.id, Type::Mixed);

                return;
            }
        };

        let Identifier::SimpleIdentifier(SimpleIdentifier {
            symbol: method_name,
            ..
        }) = &node.method
        else {
            self.map.insert(node.id, Type::Mixed);

            return;
        };

        let Some(class) = self.index.get_class(target) else {
            self.map.insert(node.id, Type::Invalid);

            return;
        };

        let Some(method) = class.get_static_method(method_name.as_ref()) else {
            self.map.insert(node.id, Type::Invalid);

            return;
        };

        let return_type = method
            .get_return_type()
            .cloned()
            .unwrap_or_else(|| Type::Mixed);

        self.map.insert(node.id, return_type);
    }
}
