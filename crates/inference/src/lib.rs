mod engine;
mod map;

pub use engine::TypeEngine;
pub use map::TypeMap;

#[cfg(test)]
mod tests {
    use pxp_ast::{HasId, ResolvedName, Statement, StatementKind};
    use pxp_index::{FileId, Index};
    use pxp_lexer::Lexer;
    use pxp_node_finder::NodeFinder;
    use pxp_parser::Parser;
    use pxp_type::{ConstExpr, Type};

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
        assert_eq!(
            infer("'Hello, world!'"),
            Type::LiteralString(b"Hello, world!".into())
        );
        assert_eq!(
            infer("\"Hello, world!\""),
            Type::LiteralString(b"Hello, world!".into())
        );
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
                Box::new(Type::Union(vec![
                    Type::LiteralString(b"a".into()),
                    Type::LiteralString(b"b".into())
                ])),
                Box::new(Type::Integer)
            )
        )
    }

    #[test]
    fn it_infers_type_of_mixed_keyed_array() {
        assert_eq!(
            infer(r#"$a = ['a' => 1, 2]"#),
            Type::TypedArray(
                Box::new(Type::Union(vec![
                    Type::LiteralString(b"a".into()),
                    Type::Integer
                ])),
                Box::new(Type::Integer)
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
            _ => panic!("Expected a named type 'A'."),
        }
    }

    #[test]
    fn it_infers_type_of_new_expression_on_class_string_literal() {
        let inferred = infer(
            r#"
        class A {}
        $a = 'A';
        new $a()"#,
        );

        assert!(inferred.is_named());

        match inferred {
            Type::Named(name) => assert_eq!(name.resolved, b"A"),
            _ => panic!("Expected a named type 'A'."),
        }
    }

    #[test]
    fn it_infers_types_of_function_parameters() {
        assert_eq!(
            infer_at(
                r#"
        function a(string $b) {
            $b^^
        }
        "#
            ),
            Type::String
        );
    }

    #[test]
    fn outer_variables_are_not_accessible_inside_of_functions() {
        assert_eq!(
            infer_at(
                r#"
        $a = 42;
        function a() {
            $a^^
        }
        "#
            ),
            Type::Mixed
        );
    }

    #[test]
    fn it_infers_type_of_variadic_parameters() {
        assert_eq!(
            infer_at(
                r#"
        function a(string ...$b) {
            $b^^
        }
        "#
            ),
            Type::TypedArray(Box::new(Type::Integer), Box::new(Type::String))
        );
    }

    #[test]
    fn it_infers_type_of_eval_expression() {
        assert_eq!(infer(r#"eval('42')"#), Type::Mixed);
    }

    #[test]
    fn it_infers_type_of_empty_expression() {
        assert_eq!(infer(r#"empty('')"#), Type::Boolean);
    }

    #[test]
    fn it_infers_type_of_die_expression() {
        assert_eq!(infer(r#"die('')"#), Type::Never);
    }

    #[test]
    fn it_infers_type_of_exit_expression() {
        assert_eq!(infer(r#"exit('')"#), Type::Never);
    }

    #[test]
    fn it_infers_type_of_isset_expression() {
        assert_eq!(infer(r#"isset('')"#), Type::Boolean);
    }

    #[test]
    fn it_infers_type_of_unset_expression() {
        assert_eq!(infer(r#"unset('')"#), Type::Void);
    }

    #[test]
    fn it_infers_type_of_print_expression() {
        assert_eq!(
            infer(r#"print('')"#),
            Type::ConstExpr(Box::new(ConstExpr::Integer(1.into())))
        );
    }

    #[test]
    fn it_infers_type_of_concat_expression() {
        assert_eq!(infer(r#"'a' . 'b'"#), Type::String);
    }

    #[test]
    fn it_infers_type_of_instanceof_expression() {
        assert_eq!(infer(r#"$a instanceof A"#), Type::Boolean);
    }

    #[test]
    fn it_infers_type_of_reference_expression() {
        assert_eq!(infer(r#"$b = 1; $a = &$b"#), Type::Integer);
    }

    #[test]
    fn it_infers_type_of_parenthesized_expression() {
        assert_eq!(infer(r#"(42)"#), Type::Integer);
    }

    #[test]
    fn it_infers_type_of_error_suppression_expression() {
        assert_eq!(infer(r#"function foo(): int {} @foo()"#), Type::Integer);
    }

    #[test]
    fn it_infers_type_of_include_expression() {
        assert_eq!(infer(r#"include 'file.php'"#), Type::Mixed);
    }

    #[test]
    fn it_infers_type_of_include_once_expression() {
        assert_eq!(infer(r#"include_once 'file.php'"#), Type::Mixed);
    }

    #[test]
    fn it_infers_type_of_require_expression() {
        assert_eq!(infer(r#"require 'file.php'"#), Type::Mixed);
    }

    #[test]
    fn it_infers_type_of_require_once_expression() {
        assert_eq!(infer(r#"require_once 'file.php'"#), Type::Mixed);
    }

    #[test]
    fn it_infers_type_of_function_closure_creation_expression() {
        assert_eq!(
            infer(r#"foo(...)"#),
            Type::Named(ResolvedName {
                resolved: b"Closure".into(),
                original: b"Closure".into(),
            })
        );
    }

    #[test]
    fn it_infers_type_of_method_call() {
        assert_eq!(
            infer(
                r#"
        class Foo {
            function bar(): int {}
        }

        (new Foo)->bar()
        "#
            ),
            Type::Integer
        );
    }

    #[test]
    fn it_infers_type_of_method_closure_creation_expression() {
        assert_eq!(
            infer(
                r#"
        class Foo {
            function bar(): int {}
        }

        (new Foo)->bar(...)
        "#
            ),
            Type::Named(ResolvedName {
                resolved: b"Closure".into(),
                original: b"Closure".into(),
            })
        );
    }

    #[test]
    fn it_infers_type_of_nullsafe_method_call() {
        assert_eq!(
            infer(
            r#"
            class Foo {
                function bar(): int {}
            }

            $foo = new Foo();
            $foo?->bar()
            "#
            ),
            Type::Union(vec![Type::Integer, Type::Null])
        );
    }

    /// Parse the given code, infer the types and return the type of the expression suffixed with a ^^ sequence.
    fn infer_at(code: &str) -> Type<ResolvedName> {
        let code = format!("<?php {};", code);
        let marker = code
            .find("^^")
            .expect("Code does not contain a ^^ sequence.");
        let code = code.replace("^^", "");
        let result = Parser::parse(Lexer::new(code.as_bytes()));

        let mut index = Index::new();
        index.index(FileId::new(0), &result.ast);

        let engine = TypeEngine::new(&index);
        let map = engine.infer(&result.ast);

        let Some((node, _)) = NodeFinder::find_at_byte_offset(&result.ast, marker) else {
            panic!("Could not find a node at the given marker.");
        };

        map.resolve(node.id).clone()
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
