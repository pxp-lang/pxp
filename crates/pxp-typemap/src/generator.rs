use std::collections::HashMap;

use pxp_ast::{Statement, ExpressionKind, literals::LiteralKind, BoolExpression, CastExpression, CastKind};
use pxp_indexer::Index;
use pxp_symbol::{Symbol, SymbolTable};
use pxp_type::Type;
use pxp_visitor::{Visitor, walk_expression};

use crate::TypeMap;

#[derive(Debug, Clone)]
pub struct TypeMapGenerator<'a> {
    index: &'a Index,
    symbol_table: &'a SymbolTable,
    scopes: Vec<Scope>,
    map: TypeMap,
}

impl<'a> TypeMapGenerator<'a> {
    pub fn new(index: &'a Index, symbol_table: &'a SymbolTable) -> Self {
        Self { index, symbol_table, scopes: Vec::new(), map: TypeMap::new() }
    }

    pub fn generate(&mut self, ast: &mut [Statement]) -> TypeMap {
        // We can use the same TypeMapGenerator for multiple files, so we need to reset the state
        // before we start generating the type map for a new file.
        self.map = TypeMap::new();
        self.scopes = vec![Scope::default()];

        self.visit(ast);

        self.map.clone()
    }
}

impl<'a> Visitor for TypeMapGenerator<'a> {
    fn visit_expression(&mut self, node: &mut pxp_ast::Expression) {
        let r#type = match &node.kind {
            ExpressionKind::Missing => Type::Mixed,
            ExpressionKind::Eval(_) => Type::Mixed,
            ExpressionKind::Empty(_) => Type::Boolean,
            ExpressionKind::Die(_) => Type::Never,
            ExpressionKind::Exit(_) => Type::Never,
            ExpressionKind::Isset(_) => Type::Boolean,
            ExpressionKind::Unset(_) => Type::Void,
            ExpressionKind::Print(_) => Type::Integer,
            ExpressionKind::Literal(literal) => match &literal.kind {
                LiteralKind::Integer => Type::Integer,
                LiteralKind::Float => Type::Float,
                LiteralKind::String => Type::String,
                LiteralKind::Missing => Type::Mixed,
            },
            ExpressionKind::ArithmeticOperation(_) => Type::Mixed,
            ExpressionKind::AssignmentOperation(_) => Type::Mixed,
            ExpressionKind::BitwiseOperation(_) => Type::Mixed,
            ExpressionKind::ComparisonOperation(_) => Type::Mixed,
            ExpressionKind::LogicalOperation(_) => Type::Mixed,
            ExpressionKind::Concat(_) => Type::String,
            ExpressionKind::Instanceof(_) => Type::Boolean,
            ExpressionKind::Reference(_) => Type::Mixed,
            // FIXME: This should return the same type as whatever the inner expression  is.
            ExpressionKind::Parenthesized(_) => Type::Mixed,
            // FIXME: This should return the same type as whatever the inner expression  is.
            ExpressionKind::ErrorSuppress(_) => Type::Mixed,
            ExpressionKind::Identifier(_) => Type::Mixed,
            ExpressionKind::Variable(_) => Type::Mixed,
            ExpressionKind::Include(_) => Type::Mixed,
            ExpressionKind::IncludeOnce(_) => Type::Mixed,
            ExpressionKind::Require(_) => Type::Mixed,
            ExpressionKind::RequireOnce(_) => Type::Mixed,
            ExpressionKind::FunctionCall(_) => Type::Mixed,
            ExpressionKind::FunctionClosureCreation(_) => Type::Mixed,
            ExpressionKind::MethodCall(_) => Type::Mixed,
            ExpressionKind::MethodClosureCreation(_) => Type::Mixed,
            ExpressionKind::NullsafeMethodCall(_) => Type::Mixed,
            ExpressionKind::StaticMethodCall(_) => Type::Mixed,
            ExpressionKind::StaticVariableMethodCall(_) => Type::Mixed,
            ExpressionKind::StaticMethodClosureCreation(_) => Type::Mixed,
            ExpressionKind::StaticVariableMethodClosureCreation(_) => Type::Mixed,
            ExpressionKind::PropertyFetch(_) => Type::Mixed,
            ExpressionKind::NullsafePropertyFetch(_) => Type::Mixed,
            ExpressionKind::StaticPropertyFetch(_) => Type::Mixed,
            ExpressionKind::ConstantFetch(_) => Type::Mixed,
            ExpressionKind::Static => Type::Mixed,
            ExpressionKind::Self_ => Type::Mixed,
            ExpressionKind::Parent => Type::Mixed,
            ExpressionKind::ShortArray(_) => Type::Array,
            ExpressionKind::Array(_) => Type::Array,
            ExpressionKind::List(_) => Type::Array,
            // FIXME: This should be of Type::Named, where the name is `\Closure`.
            ExpressionKind::Closure(_) => Type::Callable,
            // FIXME: This should be of Type::Named, where the name is `\Closure`.
            ExpressionKind::ArrowFunction(_) => Type::Callable,
            ExpressionKind::New(_) => Type::Mixed,
            ExpressionKind::InterpolatedString(_) => Type::Mixed,
            ExpressionKind::Heredoc(_) => Type::String,
            ExpressionKind::Nowdoc(_) => Type::String,
            ExpressionKind::ShellExec(_) => Type::String,
            // FIXME: This should be of Type::Named, where we generate a unique name for each
            // anonymous class.
            ExpressionKind::AnonymousClass(_) => Type::Object,
            ExpressionKind::Bool(BoolExpression { value }) => if *value { Type::True } else { Type::False },
            // FIXME: If we know the type of array we're accessing, we can be more specific here
            // and just return the inner type of the array.
            ExpressionKind::ArrayIndex(_) => Type::Mixed,
            ExpressionKind::Null => Type::Null,
            // FIXME: Since we know which constant is being referenced, we can be more specific
            // here, specifically for things like __CLASS__ etc.
            ExpressionKind::MagicConstant(_) => Type::String,
            ExpressionKind::ShortTernary(_) => Type::Mixed,
            ExpressionKind::Ternary(_) => Type::Mixed,
            ExpressionKind::Coalesce(_) => Type::Mixed,
            // FIXME: If we know the type of value we're cloning, we can be more specific here
            // and just return that same type again.
            ExpressionKind::Clone(_) => Type::Object,
            ExpressionKind::Match(_) => Type::Mixed,
            ExpressionKind::Throw(_) => Type::Never,
            ExpressionKind::Yield(_) => Type::Mixed,
            ExpressionKind::YieldFrom(_) => Type::Mixed,
            ExpressionKind::Cast(CastExpression { kind, .. }) => match kind {
                CastKind::Int => Type::Integer,
                CastKind::Bool => Type::Boolean,
                CastKind::Float => Type::Float,
                CastKind::String => Type::String,
                CastKind::Array => Type::Array,
                CastKind::Object => Type::Object,
                CastKind::Unset => Type::Never,
            },
            ExpressionKind::Noop => Type::Mixed,
        };

        self.map.insert(node.id, r#type);

        walk_expression(self, node);
    }
}

#[derive(Default, Debug, Clone)]
struct Scope {
    namespace: Option<Symbol>, 
    imports: HashMap<Symbol, Symbol>,
    variables: HashMap<Symbol, Type>,
}