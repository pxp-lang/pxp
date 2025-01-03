mod engine;
mod map;

pub use map::TypeMap;
pub use engine::TypeEngine;

#[cfg(test)]
mod tests {
    use pxp_ast::{HasId, Name, Statement, StatementKind};
    use pxp_index::{FileId, Index};
    use pxp_lexer::Lexer;
    use pxp_parser::Parser;
    use pxp_type::Type;

    use crate::TypeEngine;

    #[test]
    fn it_infers_integer_literals() {
        assert_eq!(infer("42"), Type::Integer);
    }

    #[test]
    fn it_infers_float_literals() {
        assert_eq!(infer("42.0"), Type::Float);
    }

    #[test]
    fn it_infers_string_literals() {
        assert_eq!(infer("'Hello, world!'"), Type::String);
        assert_eq!(infer("\"Hello, world!\""), Type::String);
    }

    #[test]
    fn it_infers_interpolated_strings() {
        assert_eq!(infer("\"Hello, $name!\""), Type::String);
    }

    #[test]
    fn it_infers_boolean_literals() {
        assert_eq!(infer("true"), Type::True);
        assert_eq!(infer("false"), Type::False);
    }

    /// Parse the given code, infer the types and return the type of the last expression in the code.
    fn infer(code: &str) -> Type<Name> {
        // Parse the code.
        let result = Parser::parse(Lexer::new(format!("<?php {};", code).as_bytes()));

        // Create an index and index the generated AST.
        let mut index = Index::new();
        index.index(FileId::new(0), &result.ast);

        // Create a `TypeEngine` and infer the types.
        let engine = TypeEngine::new(&index);
        let map = engine.infer(&result.ast);

        // Get the last expression in the code.
        let Some(Statement { kind: StatementKind::Expression(statement), .. }) = result.ast.last() else {
            panic!("The code must end with an expression statement.");
        };

        let expression_id = statement.expression.id();

        // Get the type of the last expression.
        map.resolve(expression_id).clone()
    }
}
