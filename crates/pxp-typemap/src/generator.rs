use std::collections::HashSet;

use pxp_ast::{Expression, ExpressionKind, literals::LiteralKind, NodeId, Statement};
use pxp_span::Span;
use pxp_type::Type;
use pxp_visitor::{Visitor, walk_expression};

use crate::type_map::TypeMap;

#[derive(Debug, Clone, Default)]
pub struct TypeMapGenerator {
    type_map: TypeMap,
    // Vec<(FromNodeId, ToNodeId)>
    deferred: Vec<(NodeId, NodeId)>,
    array_refinements: Vec<(NodeId, Vec<NodeId>)>,
}

impl TypeMapGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&mut self, ast: &mut [Statement]) -> TypeMap {
        self.visit(ast);
        self.process_array_refinements();

        self.type_map()
    }

    fn process_array_refinements(&mut self) {
        for (target_id, child_ids) in self.array_refinements.iter() {
            let mut union = vec![];

            for child_id in child_ids.iter() {
                if let Some(child_type) = self.type_map.get_expr_type(*child_id) {
                    union.push(child_type.clone());
                }
            }

            if union.is_empty() {
                continue;
            }

            self.type_map.insert_expr_type(*target_id, Type::GenericArray(Box::new(Type::Integer), Box::new(Type::Union(self.simplify_union(&union)))));
        }
    }

    fn simplify_union(&self, union: &Vec<Type>) -> Vec<Type> {
        let set: HashSet<Type> = union.into_iter().cloned().collect();
        set.into_iter().collect()
    }

    pub fn type_map(&self) -> TypeMap {
        self.type_map.clone()
    }
}

impl Visitor for TypeMapGenerator {
    fn visit_expression(&mut self, node: &mut Expression) {        
        self.type_map.insert_expr_type(node.id, match &node.kind {
            ExpressionKind::Missing => Type::Missing,
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
                LiteralKind::Missing => Type::Missing,
            },
            // FIXME: Use type information from the left & right operands to
            //        determine the correct type.
            ExpressionKind::ArithmeticOperation(_) => Type::Mixed,
            ExpressionKind::AssignmentOperation(_) => Type::Mixed,
            ExpressionKind::BitwiseOperation(_) => Type::Mixed,
            ExpressionKind::ComparisonOperation(_) => Type::Mixed,
            ExpressionKind::LogicalOperation(_) => Type::Mixed,
            ExpressionKind::Concat(_) => Type::String,
            ExpressionKind::Instanceof(_) => Type::Boolean,
            // FIXME: Map this to the type of the variable on the right-hand side.
            ExpressionKind::Reference(_) => Type::Mixed,
            // FIXME: This is not correct, it should map to the type of the
            // expression inside the parentheses.
            ExpressionKind::Parenthesized(inner) => {
                self.deferred.push((node.id, inner.expr.id));

                Type::Mixed
            },
            // FIXME: This is not correct, it should map to the type of the
            // expression inside the parentheses.
            ExpressionKind::ErrorSuppress(inner) => {
                self.deferred.push((node.id, inner.expr.id));

                Type::Mixed
            },
            // FIXME: We can check an Index for a constant with the given name
            // and return the type of that constant.
            ExpressionKind::Identifier(_) => Type::Mixed,
            // FIXME: We can check the current scope for a variable with the name
            //        and return the type of that variable.
            ExpressionKind::Variable(_) => Type::Mixed,
            ExpressionKind::Include(_) => Type::Mixed,
            ExpressionKind::IncludeOnce(_) => Type::Mixed,
            ExpressionKind::Require(_) => Type::Mixed,
            ExpressionKind::RequireOnce(_) => Type::Mixed,
            // FIXME: We can get this information from the Index.
            ExpressionKind::FunctionCall(_) => Type::Mixed,
            // FIXME: This should return a Closure type, with information
            //        about the arguments retrieved from the function definition.
            ExpressionKind::FunctionClosureCreation(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the method with the given name.
            ExpressionKind::MethodCall(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the method with the given name, returning a Closure
            //        with the correct arguments.
            ExpressionKind::MethodClosureCreation(_) => Type::Callable,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the method with the given name.
            ExpressionKind::NullsafeMethodCall(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the method with the given name.
            ExpressionKind::StaticMethodCall(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the method with the given name.
            ExpressionKind::StaticVariableMethodCall(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the method with the given name, returning a Closure
            //        with the correct arguments.
            ExpressionKind::StaticMethodClosureCreation(_) => Type::Callable,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the property with the given name, returning a Closure
            //        with the correct arguments.
            ExpressionKind::StaticVariableMethodClosureCreation(_) => Type::Callable,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the property with the given name.
            ExpressionKind::PropertyFetch(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the property with the given name.
            ExpressionKind::NullsafePropertyFetch(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the property with the given name.
            ExpressionKind::StaticPropertyFetch(_) => Type::Mixed,
            // FIXME: We can get this information by getting the type of the left-hand side,
            //        and then looking up the property with the given name.
            ExpressionKind::ConstantFetch(_) => Type::Mixed,
            // FIXME: We can get this information by looking at the current scope.
            ExpressionKind::Static => Type::Object,
            // FIXME: We can get this information by looking at the current scope.
            ExpressionKind::Self_ => Type::Object,
            // FIXME: We can get this information by looking at the current scope.
            ExpressionKind::Parent => Type::Object,
            ExpressionKind::ShortArray(inner) => {
                let mut refinements = Vec::new();

                for item in inner.items.iter() {
                    if let Some(expr) = item.value() {
                        refinements.push(expr.id);
                    }
                }

                self.array_refinements.push((node.id, refinements));

                Type::Array
            },
            ExpressionKind::Array(_) => Type::Array,
            ExpressionKind::List(_) => Type::Array,
            // FIXME: This should really be a named Closure type, with the correct arguments.
            ExpressionKind::Closure(_) => Type::Callable,
            // FIXME: This should really be a named Closure type, with the correct arguments.
            ExpressionKind::ArrowFunction(_) => Type::Callable,
            // FIXME: This should return a Named type if we can get the class name,
            //        otherwise it should return a generic Object type for now.
            ExpressionKind::New(_) => Type::Object,
            ExpressionKind::InterpolatedString(_) => Type::String,
            ExpressionKind::Heredoc(_) => Type::String,
            ExpressionKind::Nowdoc(_) => Type::String,
            ExpressionKind::ShellExec(_) => Type::String,
            // FIXME: We should create a known generic object that stores information
            //        about the anonymous class, since we have that information present.
            ExpressionKind::AnonymousClass(_) => Type::Object,
            ExpressionKind::Bool(_) => Type::Boolean,
            // FIXME: If we know the type of the array, i.e. it's a generic array, 
            //        we can return the type of the array item.
            ExpressionKind::ArrayIndex(_) => Type::Mixed,
            ExpressionKind::Null => Type::Null,
            ExpressionKind::MagicConstant(_) => Type::String,
            // FIXME: If we know for the sure the left-hand side is truthy, we can
            //        return the correct types.
            ExpressionKind::ShortTernary(_) => Type::Mixed,
            // FIXME: If we know for the sure the left-hand side is truthy, we can
            //        return the correct types.
            ExpressionKind::Ternary(_) => Type::Mixed,
            // FIXME: If we know the types of both sides, we can return a valid union.
            //        If we know that one of them is definitely set, we can return the
            //        type of the left-side.
            ExpressionKind::Coalesce(_) => Type::Mixed,
            // FIXME: If we know what we're cloning, we can return a valid Named type.
            ExpressionKind::Clone(_) => Type::Object,
            // FIXME: If we can figure out the return types of all the arms, we can 
            //        return a valid union.
            ExpressionKind::Match(_) => Type::Mixed,
            ExpressionKind::Throw(_) => Type::Never,
            ExpressionKind::Yield(_) => Type::Mixed,
            ExpressionKind::YieldFrom(_) => Type::Mixed,
            // FIXME: Inspect the cast type and return a valid type.
            ExpressionKind::Cast(_) => Type::Mixed,
            ExpressionKind::Noop => Type::Void,
        });

        walk_expression(self, node);
    }
}