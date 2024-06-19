use std::collections::HashMap;

use pxp_ast::{downcast, AssignmentOperationExpression, Expression, ExpressionKind, Literal, LiteralKind, Name, Node, SimpleVariable, Statement, Variable};
use pxp_index::Index;
use pxp_symbol::Symbol;
use pxp_type::Type;
use pxp_visitor::{NodeVisitor, NodeVisitorResult};

#[derive(Debug, Clone)]
pub struct InferenceEngine;

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine
    }

    pub fn infer(&self, index: &Index, ast: &[Statement], target: &dyn Node) -> Type<Name> {
        let mut visitor = ContextTrackingNodeVisitor::new(index, target);

        visitor.visit_ast(ast);
        visitor.get_type()
    }
}

struct ContextTrackingNodeVisitor<'a> {
    index: &'a Index,
    target: &'a dyn Node,
    resolved: Type<Name>,
    contexts: Vec<Context>,
}

struct Context {
    variables: HashMap<Symbol, Type<Name>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: Symbol) -> Option<Type<Name>> {
        self.variables.get(&name).cloned()
    }

    pub fn set_variable(&mut self, name: Symbol, r#type: Type<Name>) {
        self.variables.insert(name, r#type);
    }

    pub fn stripped(&self) -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn cloned(&self) -> Self {
        Context {
            variables: self.variables.clone(),
        }
    }
}

impl<'a> ContextTrackingNodeVisitor<'a> {
    pub fn new(index: &'a Index, target: &'a dyn Node) -> Self {
        ContextTrackingNodeVisitor {
            index,
            target,
            resolved: Type::Mixed,
            contexts: Vec::from([Context::new()]),
        }
    }

    pub fn get_type(&self) -> Type<Name> {
        self.resolved.clone()
    }

    fn resolve_type(&self, node: &dyn Node) -> Type<Name> {
        // FIXME: This is disgusting.
        if let Some(Expression { kind, .. }) = downcast(node) {
            return match kind {
                ExpressionKind::Missing => Type::Mixed,
                ExpressionKind::Eval(node) => self.resolve_type(node),
                ExpressionKind::Empty(node) => self.resolve_type(node),
                ExpressionKind::Die(node) => self.resolve_type(node),
                ExpressionKind::Exit(node) => self.resolve_type(node),
                ExpressionKind::Isset(node) => self.resolve_type(node),
                ExpressionKind::Unset(node) => self.resolve_type(node),
                ExpressionKind::Print(node) => self.resolve_type(node),
                ExpressionKind::Literal(node) => self.resolve_type(node),
                ExpressionKind::ArithmeticOperation(node) => self.resolve_type(node),
                ExpressionKind::AssignmentOperation(node) => self.resolve_type(node),
                ExpressionKind::BitwiseOperation(node) => self.resolve_type(node),
                ExpressionKind::ComparisonOperation(node) => self.resolve_type(node),
                ExpressionKind::LogicalOperation(node) => self.resolve_type(node),
                ExpressionKind::Concat(node) => self.resolve_type(node),
                ExpressionKind::Instanceof(node) => self.resolve_type(node),
                ExpressionKind::Reference(node) => self.resolve_type(node),
                ExpressionKind::Parenthesized(node) => self.resolve_type(node),
                ExpressionKind::ErrorSuppress(node) => self.resolve_type(node),
                ExpressionKind::Identifier(node) => self.resolve_type(node),
                ExpressionKind::Variable(node) => self.resolve_type(node),
                ExpressionKind::Include(node) => self.resolve_type(node),
                ExpressionKind::IncludeOnce(node) => self.resolve_type(node),
                ExpressionKind::Require(node) => self.resolve_type(node),
                ExpressionKind::RequireOnce(node) => self.resolve_type(node),
                ExpressionKind::FunctionCall(node) => self.resolve_type(node),
                ExpressionKind::FunctionClosureCreation(node) => self.resolve_type(node),
                ExpressionKind::MethodCall(node) => self.resolve_type(node),
                ExpressionKind::MethodClosureCreation(node) => self.resolve_type(node),
                ExpressionKind::NullsafeMethodCall(node) => self.resolve_type(node),
                ExpressionKind::StaticMethodCall(node) => self.resolve_type(node),
                ExpressionKind::StaticVariableMethodCall(node) => self.resolve_type(node),
                ExpressionKind::StaticMethodClosureCreation(node) => self.resolve_type(node),
                ExpressionKind::StaticVariableMethodClosureCreation(node) => self.resolve_type(node),
                ExpressionKind::PropertyFetch(node) => self.resolve_type(node),
                ExpressionKind::NullsafePropertyFetch(node) => self.resolve_type(node),
                ExpressionKind::StaticPropertyFetch(node) => self.resolve_type(node),
                ExpressionKind::ConstantFetch(node) => self.resolve_type(node),
                ExpressionKind::Static => self.resolve_type(node),
                ExpressionKind::Self_ => self.resolve_type(node),
                ExpressionKind::Parent => self.resolve_type(node),
                ExpressionKind::ShortArray(node) => self.resolve_type(node),
                ExpressionKind::Array(node) => self.resolve_type(node),
                ExpressionKind::List(node) => self.resolve_type(node),
                ExpressionKind::Closure(node) => self.resolve_type(node),
                ExpressionKind::ArrowFunction(node) => self.resolve_type(node),
                ExpressionKind::New(node) => self.resolve_type(node),
                ExpressionKind::InterpolatedString(node) => self.resolve_type(node),
                ExpressionKind::Heredoc(node) => self.resolve_type(node),
                ExpressionKind::Nowdoc(node) => self.resolve_type(node),
                ExpressionKind::ShellExec(node) => self.resolve_type(node),
                ExpressionKind::AnonymousClass(node) => self.resolve_type(node),
                ExpressionKind::Bool(node) => self.resolve_type(node),
                ExpressionKind::ArrayIndex(node) => self.resolve_type(node),
                ExpressionKind::Null => self.resolve_type(node),
                ExpressionKind::MagicConstant(node) => self.resolve_type(node),
                ExpressionKind::ShortTernary(node) => self.resolve_type(node),
                ExpressionKind::Ternary(node) => self.resolve_type(node),
                ExpressionKind::Coalesce(node) => self.resolve_type(node),
                ExpressionKind::Clone(node) => self.resolve_type(node),
                ExpressionKind::Match(node) => self.resolve_type(node),
                ExpressionKind::Throw(node) => self.resolve_type(node),
                ExpressionKind::Yield(node) => self.resolve_type(node),
                ExpressionKind::YieldFrom(node) => self.resolve_type(node),
                ExpressionKind::Cast(node) => self.resolve_type(node),
                ExpressionKind::Name(node) => self.resolve_type(node),
                ExpressionKind::Noop => Type::Never,
            };
        }

        // Simple Literal Types
        if let Some(Literal { kind, .. }) = downcast::<Literal>(node) {
            return match kind {
                LiteralKind::Integer => Type::Integer,
                LiteralKind::Float => Type::Float,
                LiteralKind::String => Type::String,
                LiteralKind::Missing => Type::Mixed,
            };
        }

        // Variable Resolution
        if let Some(SimpleVariable { symbol, .. }) = downcast::<SimpleVariable>(node) {
            if let Some(r#type) = self.context().get_variable(*symbol) {
                return r#type;
            }
        }

        Type::Mixed
    }

    fn process_node(&mut self, node: &dyn Node) {
        if let Some(assignment) = downcast::<AssignmentOperationExpression>(node) {
            if assignment.targets_variable() {
                self.process_assignment(assignment);
            }
        }
    }

    fn process_assignment(&mut self, assignment: &AssignmentOperationExpression) {
        let name = match assignment.target() {
            Expression { kind: ExpressionKind::Variable(Variable::SimpleVariable(SimpleVariable { symbol, .. })), .. } => symbol,
            _ => return,
        };

        let r#type = self.resolve_type(assignment.right());

        self.context_mut().set_variable(*name, r#type);
    }

    fn push_context(&mut self) {
        self.contexts.push(Context::new());
    }

    fn pop_context(&mut self) {
        self.contexts.pop();
    }

    fn context_mut(&mut self) -> &mut Context {
        self.contexts.last_mut().unwrap()
    }

    fn context(&self) -> &Context {
        self.contexts.last().unwrap()
    }
}

impl<'a> NodeVisitor<'a> for ContextTrackingNodeVisitor<'a> {
    fn visit(&mut self, node: &'a dyn Node) -> NodeVisitorResult {
        if std::ptr::eq(node, self.target) {
            self.resolved = self.resolve_type(node);

            return NodeVisitorResult::Stop;
        }

        self.process_node(node);
               
        NodeVisitorResult::Continue
    }
}