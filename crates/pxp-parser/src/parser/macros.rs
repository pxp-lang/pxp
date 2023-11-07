#[macro_export]
macro_rules! peek_token {
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? => $out:expr),+ $(,)? ], $state:expr, [ $($message:literal),+ $(,)? ]) => {{
        match &$state.stream.current().kind {
            $(
                $( $pattern )|+ $( if $guard )? => $out,
            )+
            _ => {
                return $crate::expected_token_err!([ $($message,)+ ], $state);
            }
        }
    }};
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?),+ $(,)? ], $state:expr, [ $($message:literal),+ $(,)? ]) => {{
        if !matches!($state.stream.current().kind, $( $pattern )|+ $( if $guard )?) {
            return $crate::expected_token_err!([ $($message,)+ ], $state);
        }
    }};
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? => $out:expr),+ $(,)? ], $state:expr, $message:literal) => {
        $crate::peek_token!([ $($( $pattern )|+ $( if $guard )? => $out,)+ ], $state, [$message])
    };
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?),+ $(,)? ], $state:expr, $message:literal) => {
        $crate::peek_token!([ $($( $pattern )|+ $( if $guard )?,)+ ], $state, [$message])
    };
}

#[macro_export]
macro_rules! expect_token {
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? => $out:expr),+ $(,)? ], $state:expr, [ $($message:literal),+ $(,)? ]) => {{
        let token = $state.stream.current();
        $state.stream.next();
        match token.kind {
            $(
                $( $pattern )|+ $( if $guard )? => {
                    $out
                },
            )+
            _ => {
                return Err($crate::parser::error::unexpected_token(
                    vec![$($message.into(),)+],
                    token,
                ))
            }
        }
    }};
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? => $out:expr),+ $(,)? ], $state:expr, $message:literal) => {
        $crate::expect_token!([ $($( $pattern )|+ $( if $guard )? => $out,)+ ], $state, [$message])
    };
}

#[macro_export]
macro_rules! expect_literal {
    ($state:expr) => {{
        let current = $state.stream.current();

        match &current.kind {
            TokenKind::LiteralInteger => {
                $state.stream.next();

                $crate::parser::ast::literals::Literal::Integer(
                    $crate::parser::ast::literals::LiteralInteger {
                        span: current.span,
                        value: current.value.clone(),
                    },
                )
            }
            TokenKind::LiteralFloat => {
                $state.stream.next();

                $crate::parser::ast::literals::Literal::Float(
                    $crate::parser::ast::literals::LiteralFloat {
                        span: current.span,
                        value: current.value.clone(),
                    },
                )
            }
            TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => {
                $state.stream.next();

                $crate::parser::ast::literals::Literal::String(
                    $crate::parser::ast::literals::LiteralString {
                        span: current.span,
                        value: current.value.clone(),
                        kind: if matches!(current.kind, TokenKind::LiteralSingleQuotedString) {
                            $crate::parser::ast::literals::LiteralStringKind::SingleQuoted
                        } else {
                            $crate::parser::ast::literals::LiteralStringKind::DoubleQuoted
                        },
                    },
                )
            }
            _ => {
                return $crate::expected_token_err!(["a literal"], $state);
            }
        }
    }};
}

#[macro_export]
macro_rules! expected_token_err {
    ([ $($expected:literal),+ $(,)? ], $state:expr $(,)?) => {{
        Err($crate::expected_token!([$($expected),+], $state))
    }};

    ($expected:literal, $state:expr $(,)?) => {
        $crate::expected_token_err!([$expected], $state)
    };
}

#[macro_export]
macro_rules! expected_token {
    ([ $($expected:literal),+ $(,)? ], $state:expr $(,)?) => {{
        $crate::parser::error::unexpected_token(
            vec![$($expected.into()),+],
            $state.stream.current(),
        )
    }};

    ($expected:literal, $state:expr $(,)?) => {
        $crate::expected_token!([$expected], $state)
    };
}

#[macro_export]
macro_rules! expected_scope {
    ([ $($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? => $out:expr),+ $(,)? ], $state:expr) => {{
        match $state.scope().cloned()? {
            $(
                $( $pattern )|+ $( if $guard )? => $out,
            )+
            _ => {
                return Err($crate::parser::error::reached_unpredictable_state($state.stream.current().span));
            }
        }
    }};
}

#[macro_export]
macro_rules! scoped {
    ($state:expr, $scope:expr, $block:block) => {{
        $state.enter($scope);

        let result = $block;

        $state.exit();

        result
    }};
}
