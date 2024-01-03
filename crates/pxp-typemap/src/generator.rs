use std::collections::HashMap;

use indexmap::IndexSet;
use pxp_ast::{Statement, ExpressionKind, literals::LiteralKind, BoolExpression, CastExpression, CastKind, operators::AssignmentOperationExpression, variables::{Variable, SimpleVariable}, ArrayIndexExpression, ShortArrayExpression, ArrayExpression, ArrayItem, utils::CommaSeparated};
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

    fn scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn simplify_union_of_types(&self, types: &[Type]) -> Vec<Type> {
        let set: IndexSet<Type> = types.iter().cloned().collect();
        set.into_iter().collect()
    }

    fn generate_array_type(&self, items: &CommaSeparated<ArrayItem>) -> Type {
        if items.len() == 0 {
            return Type::EmptyArray;
        }

        let mut types = Vec::new();

        for item in items.iter() {
            match item.value() {
                Some(value) => types.push(self.map.get(value.id).cloned().unwrap_or(Type::Mixed)),
                None => {},
            }
        }

        let simplified = self.simplify_union_of_types(&types);

        // FIXME: We should also be handling cases where the keys are strings (not integers).
        //        Perhaps we need a `key_and_value()` helper on `ArrayItem` that returns a tuple.
        if simplified.len() == 1 {
            Type::GenericArray(Box::new(Type::Integer), Box::new(simplified[0].clone()))
        } else {
            Type::GenericArray(Box::new(Type::Integer), Box::new(Type::Union(simplified)))
        }
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
        // We pre-walk the expression so that we can use type information of sub-expressions when
        // determining the type of the current expression.
        walk_expression(self, node);

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
            ExpressionKind::AssignmentOperation(operation) => match operation {
                AssignmentOperationExpression::Assign { left, right, .. } if matches!(left.kind, ExpressionKind::Variable(Variable::SimpleVariable(_))) => {
                    let variable = match &left.kind {
                        ExpressionKind::Variable(Variable::SimpleVariable(SimpleVariable { token })) => token.symbol.unwrap(),
                        _ => unreachable!(),
                    };

                    let r#type = self.map.get(right.id).cloned().unwrap_or(Type::Mixed);

                    self.scope_mut().insert_variable(variable, r#type.clone());

                    r#type
                },
                // This is to handle cases such as $items[] = 1, where we need to update the type of the array to include the new value.
                AssignmentOperationExpression::Assign { left, right, .. } if matches!(&left.kind, ExpressionKind::ArrayIndex(ArrayIndexExpression { index, array, .. }) if index.is_none() && matches!(&array.kind, ExpressionKind::Variable(Variable::SimpleVariable(_)))) => {
                    let variable = match &left.kind {
                        ExpressionKind::ArrayIndex(ArrayIndexExpression { array, .. }) => match &array.kind {
                            ExpressionKind::Variable(Variable::SimpleVariable(SimpleVariable { token })) => token.symbol.unwrap(),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };

                    let right_type = self.map.get(right.id).cloned().unwrap_or(Type::Mixed);

                    // If we have a variable that we know the type of...
                    if let Some(variable_type) = self.scope().get_variable(variable).cloned() {
                        // and it's a generically typed array, or an empty array...
                        let types = match variable_type {
                            Type::GenericArray(key_type, value_type) => Some((key_type.as_ref().clone(), value_type.as_ref().clone())),
                            Type::EmptyArray => Some((Type::Integer, Type::Mixed)),
                            _ => None,
                        };

                        // FIXME: We should also check that the key types are the same.
                        if let Some((key_type, value_type)) = types {
                            // then we can update the type of the array to include the new value.
                            let mut types = match value_type {
                                Type::Union(types) => types.clone(),
                                Type::Mixed => vec![],
                                _ => vec![value_type.clone()],
                            };

                            types.push(right_type.clone());

                            let simplified = self.simplify_union_of_types(&types);
                            let new_inner_type = if simplified.len() == 1 { simplified[0].clone() } else { Type::Union(simplified) };

                            // and then update the type of the variable to be the new array type.
                            self.scope_mut().insert_variable(variable, Type::GenericArray(Box::new(key_type), Box::new(new_inner_type)));
                        }
                    }

                    right_type
                },
                _ => Type::Mixed,
            },
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
            ExpressionKind::Variable(variable) => match variable {
                Variable::SimpleVariable(SimpleVariable { token }) => {
                    let variable = token.symbol.unwrap();
                    self.scope().get_variable(variable).cloned().unwrap_or(Type::Mixed)
                },
                // FIXME: There might be cases where we can be more specific here, e.g. if we know
                // what the value of the dynamic variable is.
                _ => Type::Mixed,
            },
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
            ExpressionKind::ShortArray(ShortArrayExpression { items, .. }) => self.generate_array_type(items),
            ExpressionKind::Array(ArrayExpression { items, .. }) => self.generate_array_type(items),
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
    }
}

#[derive(Default, Debug, Clone)]
struct Scope {
    namespace: Option<Symbol>, 
    imports: HashMap<Symbol, Symbol>,
    variables: HashMap<Symbol, Type>,
}

impl Scope {
    fn get_variable(&self, name: Symbol) -> Option<&Type> {
        self.variables.get(&name)
    }

    fn insert_variable(&mut self, name: Symbol, ty: Type) {
        self.variables.insert(name, ty);
    }
}