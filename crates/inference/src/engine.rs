use std::{cell::RefCell, collections::HashMap, rc::Rc};

use pxp_ast::{visitor::{walk_expression, Visitor}, *};
use pxp_bytestring::{ByteStr, ByteString};
use pxp_index::{Index, ReflectionFunctionLike};
use pxp_token::TokenKind;
use pxp_type::Type;
use visitor::walk_function_call_expression;

use crate::TypeMap;

/// The `TypeEngine` is responsible for generating a `TypeMap` for a given AST.
/// It uses the provided `Index` to resolve types for method calls, property accesses, etc.
pub struct TypeEngine<'a> {
    index: &'a Index
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
            scopes: vec![Scope::new()]
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
    variables: HashMap<ByteString, Type<Name>>,
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
            outer: Some(Rc::new(RefCell::new(self.clone())))
        }
    }

    fn set_variable(&mut self, variable: &SimpleVariable, ty: &Type<Name>) {
        self.variables.insert(variable.symbol.clone(), ty.clone());
    }

    fn get_variable(&self, variable: &SimpleVariable) -> Option<Type<Name>> {
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
    fn is_callable_string(&self, name: &ByteStr) -> bool {
        let name: &ByteStr = name[1..name.len() - 1].into();

        if name.contains(b"::") {
            todo!();
        }

        self.index.get_function(name).is_some()
    }

    fn determine_function_call_target_return_type(&self, target: &Expression) -> Type<Name> {
        match &target.kind {
            ExpressionKind::Name(name) => self.get_function_call_target_return_type_from_name(name.as_ref()),
            ExpressionKind::Parenthesized(inner) => self.determine_function_call_target_return_type(&inner.expr),
            ExpressionKind::Closure(inner) => inner.return_type.as_ref().map(|t| t.data_type.get_type().clone()).unwrap_or_else(|| Type::Mixed),
            ExpressionKind::Literal(inner) => match inner.kind {
                LiteralKind::String if self.is_callable_string(inner.token.symbol.as_ref()) => self.get_function_call_target_return_type_from_callable_string(inner.token.symbol.as_ref()),
                _ => Type::Mixed,
            }
            // FIXME: Support other callable types here.
            _ => Type::Mixed,
        }
    }

    fn get_function_call_target_return_type_from_callable_string(&self, name: &ByteStr) -> Type<Name> {
        let name: &ByteStr = name[1..name.len() - 1].into();

        // FIXME: Handle method calls.
        if name.contains(b"::") {
            return Type::Mixed;
        }

        match self.index.get_function(name) {
            Some(function) => function.get_return_type().unwrap_or_else(|| &Type::Mixed).clone(),
            None => Type::Mixed,
        }
    }

    fn get_function_call_target_return_type_from_name(&self, name: &Name) -> Type<Name> {
        match &name.kind {
            NameKind::Resolved(inner) => match self.index.get_function(inner.resolved.as_bytestr()) {
                Some(function) => function.get_return_type().unwrap_or_else(|| &Type::Mixed).clone(),
                None => Type::Mixed,
            },
            
            _ => todo!(),
        }
    }
}

impl<'a> Visitor for TypeMapGenerator<'a> {
    fn visit_expression(&mut self, node: &Expression) {
        walk_expression(self, node);

        let inner = self.map.resolve(node.kind.id()).clone();

        self.map.insert(node.id, inner);
    }

    fn visit_literal(&mut self, node: &Literal) {
        self.map.insert(node.id, match node.kind {
            LiteralKind::Integer => Type::Integer,
            LiteralKind::Float => Type::Float,
            LiteralKind::String => Type::String,
            LiteralKind::Missing => Type::Missing,
        })
    }

    fn visit_interpolated_string_expression(&mut self, node: &InterpolatedStringExpression) {
        self.map.insert(node.id, Type::String);
    }

    fn visit_bool_expression(&mut self, node: &BoolExpression) {
        self.map.insert(node.id, match node.value.kind {
            TokenKind::True => Type::True,
            TokenKind::False => Type::False,
            _ => Type::Boolean
        });
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
        self.map.insert(node.id, self.map.resolve(node.right.kind.id()).clone());

        // If the left-hand side is a variable, we can use that to assign the type in the current scope.
        match &node.left.kind {
            ExpressionKind::Variable(variable) if variable.is_simple() => {
                let variable = variable.to_simple();
                let resolved = self.map.resolve(node.right.kind.id());

                self.scopes.current_mut().set_variable(&variable, resolved);
                self.map.insert(variable.id, resolved.clone());
            },
            _ => return,
        }
    }
}
