use std::collections::HashMap;

use pxp_ast::{*, visitor::Visitor};
use pxp_bytestring::ByteString;
use pxp_index::Index;
use pxp_type::Type;
use visitor::{walk_assignment_operation_expression, walk_expression, walk_function_statement, walk_parenthesized_expression};

use crate::TypeMap;

/// An internal set of methods for generating a `TypeMap` from an AST.
/// 
/// This is used internally by the `InferenceEngine` to generate a `TypeMap` from an AST.
pub(super) struct TypeMapGenerator<'i> {
    index: &'i Index,
    map: TypeMap,
    scopes: ScopeStack,
}

#[derive(Debug)]
struct ScopeStack(Vec<Scope>);

impl ScopeStack {
    fn new() -> Self {
        ScopeStack(Vec::new())
    }

    fn push(&mut self) {
        self.0.push(Scope {
            variables: HashMap::new(),
        });
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn scope(&self) -> &Scope {
        self.0.last().unwrap()
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.0.last_mut().unwrap()
    }
}

#[derive(Debug)]
struct Scope {
    variables: HashMap<ByteString, Type<ByteString>>,
}

impl Scope {
    fn insert(&mut self, variable: ByteString, ty: Type<ByteString>) {
        self.variables.insert(variable, ty);
    }

    fn get(&self, variable: &ByteString) -> &Type<ByteString> {
        self.variables.get(variable).unwrap_or_else(|| &Type::Mixed)
    }
}

impl<'i> TypeMapGenerator<'i> {
    pub fn new(index: &'i Index) -> Self {
        // We initialise the ScopeStack with a single `Scope` to
        // represent the global scope. This scope should never be popped.
        let mut scopes = ScopeStack::new();
        scopes.push();

        TypeMapGenerator {
            index,
            map: TypeMap::new(),
            scopes,
        }
    }

    pub fn generate(&mut self, ast: &[Statement]) -> TypeMap {
        // FIXME: The `Visitor` trait itself needs to accept a slice
        //        rather than a reference to a `Vec<Statement>`.
        let ast = ast.to_vec();

        self.visit(&ast);

        self.map.clone()
    }

    fn scoped(&mut self, f: impl FnOnce(&mut Self)) {
        self.scopes.push();
        f(self);
        self.scopes.pop();
    }

    fn bytestring_type(&self, ty: &Type<Name>) -> Type<ByteString> {
        match ty {
            Type::Named(inner) => Type::Named(inner.symbol().clone()),
            Type::Nullable(inner) => Type::Nullable(Box::new(self.bytestring_type(inner))),
            Type::Union(tys) => Type::Union(tys.iter().map(|t| self.bytestring_type(t)).collect()),
            Type::Intersection(tys) => Type::Intersection(tys.iter().map(|t| self.bytestring_type(t)).collect()),
            Type::Void => Type::Void,
            Type::Null => Type::Null,
            Type::True => Type::True,
            Type::False => Type::False,
            Type::Never => Type::Never,
            Type::Float => Type::Float,
            Type::Boolean => Type::Boolean,
            Type::Integer => Type::Integer,
            Type::String => Type::String,
            Type::Array => Type::Array,
            Type::Object => Type::Object,
            Type::Mixed => Type::Mixed,
            Type::Callable => Type::Callable,
            Type::Iterable => Type::Iterable,
            Type::StaticReference => Type::StaticReference,
            Type::SelfReference => Type::SelfReference,
            Type::ParentReference => Type::ParentReference,
            Type::Missing => Type::Missing,
        }
    }
}

/// Handles traversing the AST and generating a `TypeMap`.
impl Visitor for TypeMapGenerator<'_> {
    // All top-level expressions have the same type as their child.
    fn visit_expression(&mut self, node: &Expression) {
        walk_expression(self, node);

        self.map.insert(node.id(), self.map.resolve(node.kind.id()).clone());
    }

    fn visit_literal(&mut self, node: &Literal) {
        self.map.insert(node.id(), match node.kind {
            LiteralKind::String => Type::String,
            LiteralKind::Integer => Type::Integer,
            LiteralKind::Float => Type::Float,
            _ => Type::Mixed,
        });
    }

    fn visit_bool_expression(&mut self, node: &BoolExpression) {
        self.map.insert(node.id(), Type::Boolean);
    }

    fn visit_simple_variable(&mut self, node: &SimpleVariable) {
        let ty = self.scopes.scope().get(&node.symbol);

        self.map.insert(node.id(), ty.clone());
    }

    fn visit_assignment_operation_expression(&mut self, node: &AssignmentOperationExpression) {
        walk_assignment_operation_expression(self, node);

        let target = node.kind.left();

        // We can only track the types for simple variables in the current scope.
        // Dynamic variable tracking is far more complex.
        if ! matches!(target.kind, ExpressionKind::Variable(Variable::SimpleVariable(_))) {
            return;
        }

        let variable = match &target.kind {
            ExpressionKind::Variable(Variable::SimpleVariable(SimpleVariable { symbol, .. })) => symbol,
            _ => unreachable!(),
        };

        let value = node.kind.right();
        let ty = self.map.resolve(value.id());

        self.scopes.scope_mut().insert(variable.clone(), ty.clone());
        self.map.insert(node.id(), ty.clone());
    }

    fn visit_function_statement(&mut self, node: &FunctionStatement) {
        self.scoped(|this| {
            // Insert function parameters into the current scope.
            for parameter in node.parameters.iter() {
                // FIXME: Make this look nicer...
                let ty = parameter.data_type.as_ref().map(|d| this.bytestring_type(d.get_type())).unwrap_or_else(|| Type::Mixed);

                this.scopes.scope_mut().insert(parameter.name.symbol.clone(), ty);
            }

            walk_function_statement(this, node);
        });
    }

    fn visit_function_call_expression(&mut self, node: &FunctionCallExpression) {
        // FIXME: Add support for calling `Closure` objects and `__invoke`able objects.
        if ! matches!(node.target.kind, ExpressionKind::Name(_)) {
            return;
        }

        let name = match &node.target.kind {
            ExpressionKind::Name(name) => name,
            _ => unreachable!(),
        };

        let return_type = if name.is_resolved() {
            let symbol = &name.as_resolved().unwrap().resolved;

            self.index.get_function(symbol).map(|f| f.get_return_type().clone()).unwrap_or_else(|| Type::Mixed)
        } else {
            todo!("do checks for resolved and unresolved names");
        };

        self.map.insert(node.id, self.bytestring_type(&return_type));
    }

    fn visit_parenthesized_expression(&mut self, node: &ParenthesizedExpression) {
        walk_parenthesized_expression(self, node);

        let ty = self.map.resolve(node.expr.id);

        self.map.insert(node.id, ty.clone());
    }

    fn visit_new_expression(&mut self, node: &NewExpression) {
        let target = node.target.as_ref();

        // FIXME: Add support for "new"ing up objects from local variables of known "objectable" types.
        if ! matches!(target.kind, ExpressionKind::Name(_)) {
            return;
        }

        let name = match &target.kind {
            ExpressionKind::Name(name) => name,
            _ => unreachable!()
        };

        let ty = if let Some(resolved) = name.as_resolved() {
            if self.index.has_class(&resolved.resolved) {
                Type::Named(resolved.resolved.clone())
            } else {
                Type::Mixed
            }
        } else if name.is_unresolved() {
            Type::Mixed
        } else if name.is_special() {
            Type::Mixed
        } else {
            unreachable!()
        };

        self.map.insert(node.id, ty);
    }
}