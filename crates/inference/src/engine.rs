use pxp_ast::{visitor::{walk_expression, Visitor}, *};
use pxp_bytestring::ByteStr;
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
            index: self.index
        };

        generator.visit(ast);
        map
    }
}

struct TypeMapGenerator<'a> {
    map: &'a mut TypeMap,
    index: &'a Index,
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
}
