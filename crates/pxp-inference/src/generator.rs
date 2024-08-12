use std::collections::HashMap;

use pxp_ast::{*, visitor::Visitor};
use pxp_index::Index;
use pxp_symbol::Symbol;
use pxp_type::Type;
use visitor::{walk_assignment_operation_expression, walk_expression, walk_function_statement, walk_variable};

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
    variables: HashMap<Symbol, Type<Name>>,
}

impl Scope {
    fn insert(&mut self, variable: Symbol, ty: Type<Name>) {
        self.variables.insert(variable, ty);
    }

    fn get(&self, variable: Symbol) -> &Type<Name> {
        self.variables.get(&variable).unwrap_or_else(|| &Type::Mixed)
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
        let ty = self.scopes.scope().get(node.symbol);

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

        let variable = match target.kind {
            ExpressionKind::Variable(Variable::SimpleVariable(SimpleVariable { symbol, .. })) => symbol,
            _ => unreachable!(),
        };

        let value = node.kind.right();
        let ty = self.map.resolve(value.id());

        self.scopes.scope_mut().insert(variable, ty.clone());
        self.map.insert(node.id(), ty.clone());
    }

    fn visit_function_statement(&mut self, node: &FunctionStatement) {
        self.scoped(|this| {
            // Insert function parameters into the current scope.
            for parameter in node.parameters.iter() {
                // FIXME: Make this look nicer...
                this.scopes.scope_mut().insert(parameter.name.symbol, parameter.data_type.as_ref().map(|d| d.get_type().clone()).unwrap_or_else(|| Type::Mixed));
            }

            walk_function_statement(this, node);
        });
    }

    fn visit_function_call_expression(&mut self, node: &FunctionCallExpression) {
        // FIXME: Add support for calling `Closure` objects and `__invoke`able objects.
        if ! matches!(node.target.kind, ExpressionKind::Name(_)) {
            return;
        }

        let name = match node.target.kind {
            ExpressionKind::Name(name) => name,
            _ => unreachable!(),
        };

        let return_type = if name.is_resolved() {
            let symbol = name.as_resolved().unwrap().resolved;

            self.index.get_function(symbol).map(|f| f.get_return_type().clone()).unwrap_or_else(|| Type::Mixed)
        } else {
            todo!("do checks for resolved and unresolved names");
        };

        self.map.insert(node.id, return_type);
    }
}