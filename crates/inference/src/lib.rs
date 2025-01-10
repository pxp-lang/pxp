mod engine;
mod map;

pub use engine::TypeEngine;
pub use map::TypeMap;

#[cfg(test)]
mod tests {
    use pxp_ast::{HasId, Name, ResolvedName, Statement, StatementKind};
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

    #[test]
    fn it_infers_type_of_function_calls() {
        assert_eq!(
            infer(
                r#"
        function a(): int {}
        a()
        "#
            ),
            Type::Integer
        );
    }

    #[test]
    fn it_infers_type_of_iife() {
        assert_eq!(
            infer(
                r#"
        (function (): int {
            return 42;
        })()
        "#
            ),
            Type::Integer
        );
    }

    #[test]
    fn it_infers_type_of_function_calls_on_callable_string() {
        assert_eq!(
            infer(
                r#"
        function a(): string {}
        'a'()
        "#
            ),
            Type::String
        );
    }

    #[test]
    fn it_infers_type_of_variable() {
        assert_eq!(
            infer(
                r#"
        $a = 42;
        $a
        "#
            ),
            Type::Integer
        );
    }

    #[test]
    fn it_infers_type_of_assignment_expression() {
        assert_eq!(infer(r#"$a = 100"#), Type::Integer);
    }

    #[test]
    fn it_tracks_types_through_assignments() {
        assert_eq!(
            infer(
                r#"
        $a = 42;
        $b = $a;
        $c = $b;
        $c
        "#
            ),
            Type::Integer
        );
    }

    #[test]
    fn it_infers_type_of_arrays() {
        assert_eq!(
            infer(r#"$a = [1, 2, 3]"#),
            Type::TypedArray(Box::new(Type::Integer), Box::new(Type::Integer))
        );
    }

    #[test]
    fn it_infers_type_of_keyed_array() {
        assert_eq!(
            infer(r#"$a = ['a' => 1, 'b' => 2]"#),
            Type::TypedArray(
                Box::new(Type::String),
                Box::new(Type::Integer),
            )
        )
    }

    #[test]
    fn it_infers_type_of_mixed_keyed_array() {
        assert_eq!(
            infer(r#"$a = ['a' => 1, 2]"#),
            Type::TypedArray(
                Box::new(Type::array_key_types()),
                Box::new(Type::Integer),
            ),
        )
    }

    #[test]
    fn it_infers_type_of_new_expression() {
        let inferred = infer(
            r#"
        class A {}
        new A()
        "#,
        );

        assert!(inferred.is_named());

        match inferred {
            Type::Named(name) => assert_eq!(name.resolved, b"A"),
            _ => panic!("Expected a named type."),
        }
    }

    /// Parse the given code, infer the types and return the type of the last expression in the code.
    fn infer(code: &str) -> Type<ResolvedName> {
        // Parse the code.
        let result = Parser::parse(Lexer::new(format!("<?php {};", code).as_bytes()));

        // Create an index and index the generated AST.
        let mut index = Index::new();
        index.index(FileId::new(0), &result.ast);

        // Create a `TypeEngine` and infer the types.
        let engine = TypeEngine::new(&index);
        let map = engine.infer(&result.ast);

        // Get the last expression in the code.
        let Some(Statement {
            kind: StatementKind::Expression(statement),
            ..
        }) = result.ast.last()
        else {
            panic!("The code must end with an expression statement.");
        };

        let expression_id = statement.expression.id();

        // Get the type of the last expression.
        map.resolve(expression_id).clone()
    }
}
