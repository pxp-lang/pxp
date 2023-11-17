use crate::expected_token_err;
use crate::error;
use crate::error::ParseResult;
use crate::internal::arrays;
use crate::internal::attributes;
use crate::internal::classes;
use crate::internal::control_flow;
use crate::internal::functions;
use crate::internal::identifiers;
use crate::internal::parameters;
use crate::internal::precedences::Associativity;
use crate::internal::precedences::Precedence;
use crate::internal::strings;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use pxp_ast::Expression;
use pxp_ast::arguments::ArgumentPlaceholder;
use pxp_ast::comments::CommentGroup;
use pxp_ast::identifiers::DynamicIdentifier;
use pxp_ast::identifiers::Identifier;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::literals::Literal;
use pxp_ast::literals::LiteralFloat;
use pxp_ast::literals::LiteralInteger;
use pxp_ast::literals::LiteralString;
use pxp_ast::operators::ArithmeticOperationExpression;
use pxp_ast::operators::AssignmentOperationExpression;
use pxp_ast::operators::BitwiseOperationExpression;
use pxp_ast::operators::ComparisonOperationExpression;
use pxp_ast::operators::LogicalOperationExpression;
use pxp_ast::{
    ArrayIndexExpression, CoalesceExpression, ConcatExpression, ConstantFetchExpression,
    ExpressionKind, FunctionCallExpression, FunctionClosureCreationExpression, InstanceofExpression,
    MagicConstantExpression, MethodCallExpression, MethodClosureCreationExpression,
    NullsafeMethodCallExpression, NullsafePropertyFetchExpression, PropertyFetchExpression,
    ReferenceExpression, ShortTernaryExpression, StaticMethodCallExpression,
    StaticMethodClosureCreationExpression, StaticPropertyFetchExpression,
    StaticVariableMethodCallExpression, StaticVariableMethodClosureCreationExpression,
    TernaryExpression,
};
use pxp_span::Span;
use pxp_token::DocStringKind;
use pxp_token::TokenKind;

use pxp_ast::literals::LiteralStringKind;
use pxp_ast::BoolExpression;
use pxp_ast::CastExpression;
use pxp_ast::CloneExpression;
use pxp_ast::DieExpression;
use pxp_ast::EmptyExpression;
use pxp_ast::ErrorSuppressExpression;
use pxp_ast::EvalExpression;
use pxp_ast::ExitExpression;
use pxp_ast::IncludeExpression;
use pxp_ast::IncludeOnceExpression;
use pxp_ast::IssetExpression;
use pxp_ast::NewExpression;
use pxp_ast::ParenthesizedExpression;
use pxp_ast::PrintExpression;
use pxp_ast::RequireExpression;
use pxp_ast::RequireOnceExpression;
use pxp_ast::ThrowExpression;
use pxp_ast::UnsetExpression;
use pxp_ast::YieldExpression;
use pxp_ast::YieldFromExpression;

pub fn create(state: &mut State) -> ParseResult<Expression> {
    for_precedence(state, Precedence::Lowest)
}

fn null_coalesce_precedence(state: &mut State) -> ParseResult<Expression> {
    for_precedence(state, Precedence::NullCoalesce)
}

fn clone_or_new_precedence(state: &mut State) -> ParseResult<Expression> {
    for_precedence(state, Precedence::CloneOrNew)
}

fn for_precedence(state: &mut State, precedence: Precedence) -> ParseResult<Expression> {
    let mut left = left(state, &precedence)?;

    loop {
        let current = state.stream.current();
        let span = current.span;
        let kind = &current.kind;

        if matches!(current.kind, TokenKind::SemiColon | TokenKind::Eof) {
            break;
        }

        if is_postfix(kind) {
            let lpred = Precedence::postfix(kind);

            if lpred < precedence {
                break;
            }

            left = postfix(state, left, kind)?;
            continue;
        }

        if is_infix(kind) {
            let rpred = Precedence::infix(kind);

            if rpred < precedence {
                break;
            }

            if rpred == precedence && matches!(rpred.associativity(), Some(Associativity::Left)) {
                break;
            }

            if rpred == precedence && matches!(rpred.associativity(), Some(Associativity::Non)) {
                return Err(error::unexpected_token(vec![], current));
            }

            state.stream.next();

            let op = state.stream.current();
            let start_span = op.span;
            let kind = match kind {
                TokenKind::Question => {
                    // this happens due to a comment, or whitespaces between the ? and the :
                    // we consider `foo() ? : bar()` a ternary expression, with `then` being a noop
                    // however, this must behave like a short ternary at runtime.
                    if op.kind == TokenKind::Colon {
                        state.stream.next();

                        let r#else = create(state)?;

                        ExpressionKind::Ternary(TernaryExpression {
                            condition: Box::new(left),
                            question: span,
                            then: Box::new(Expression::noop(start_span)),
                            colon: op.span,
                            r#else: Box::new(r#else),
                        })
                    } else {
                        let then = create(state)?;
                        let colon = utils::skip_colon(state)?;
                        let r#else = create(state)?;

                        ExpressionKind::Ternary(TernaryExpression {
                            condition: Box::new(left),
                            question: span,
                            then: Box::new(then),
                            colon,
                            r#else: Box::new(r#else),
                        })
                    }
                }
                TokenKind::QuestionColon => {
                    let r#else = create(state)?;
                    ExpressionKind::ShortTernary(ShortTernaryExpression {
                        condition: Box::new(left),
                        question_colon: span,
                        r#else: Box::new(r#else),
                    })
                }
                TokenKind::Equals if op.kind == TokenKind::Ampersand => {
                    state.stream.next();

                    // FIXME: You should only be allowed to assign a referencable variable,
                    //        here, not any old expression.
                    let right = Box::new(for_precedence(state, rpred)?);
                    let right_span = right.span;

                    ExpressionKind::AssignmentOperation(AssignmentOperationExpression::Assign {
                        left: Box::new(left),
                        equals: span,
                        right: Box::new(Expression::new(
                        ExpressionKind::Reference(ReferenceExpression {
                                ampersand: op.span,
                                right,
                            }),
                            Span::new(start_span.start, right_span.end),
                            CommentGroup::default()
                        )),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Self_ => {
                    state.stream.next();

                    ExpressionKind::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::new(
                            ExpressionKind::Self_,
                            start_span,
                            CommentGroup::default()
                        )),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Parent => {
                    state.stream.next();

                    ExpressionKind::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::new(
                            ExpressionKind::Parent,
                            start_span,
                            CommentGroup::default()
                        )),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Static => {
                    state.stream.next();

                    ExpressionKind::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::new(
                            ExpressionKind::Static,
                            start_span,
                            CommentGroup::default()
                        )),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Enum => {
                    let enum_span = op.span;
                    state.stream.next();

                    ExpressionKind::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::new(
                            ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                                SimpleIdentifier {
                                    span: enum_span,
                                    value: "enum".into(),
                                },
                            )),
                            Span::new(start_span.start, enum_span.end),
                            CommentGroup::default()
                        )),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::From => {
                    let from_span = op.span;
                    state.stream.next();

                    ExpressionKind::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::new(
                            ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                                SimpleIdentifier {
                                    span: from_span,
                                    value: "from".into(),
                                },
                            )),
                            Span::new(start_span.start, from_span.end),
                            CommentGroup::default()
                        )),
                    })
                }
                _ => {
                    let left = Box::new(left);
                    let right = Box::new(for_precedence(state, rpred)?);

                    match kind {
                        TokenKind::Plus => ExpressionKind::ArithmeticOperation(
                            ArithmeticOperationExpression::Addition {
                                left,
                                plus: span,
                                right,
                            },
                        ),
                        TokenKind::Minus => ExpressionKind::ArithmeticOperation(
                            ArithmeticOperationExpression::Subtraction {
                                left,
                                minus: span,
                                right,
                            },
                        ),
                        TokenKind::Asterisk => ExpressionKind::ArithmeticOperation(
                            ArithmeticOperationExpression::Multiplication {
                                left,
                                asterisk: span,
                                right,
                            },
                        ),
                        TokenKind::Slash => ExpressionKind::ArithmeticOperation(
                            ArithmeticOperationExpression::Division {
                                left,
                                slash: span,
                                right,
                            },
                        ),
                        TokenKind::Percent => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::Modulo {
                                left,
                                percent: span,
                                right,
                            })
                        }
                        TokenKind::Pow => ExpressionKind::ArithmeticOperation(
                            ArithmeticOperationExpression::Exponentiation {
                                left,
                                pow: span,
                                right,
                            },
                        ),
                        TokenKind::Equals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression::Assign {
                                left,
                                equals: span,
                                right,
                            })
                        }
                        TokenKind::PlusEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::Addition {
                                left,
                                plus_equals: span,
                                right,
                            },
                        ),
                        TokenKind::MinusEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::Subtraction {
                                left,
                                minus_equals: span,
                                right,
                            },
                        ),
                        TokenKind::AsteriskEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::Multiplication {
                                left,
                                asterisk_equals: span,
                                right,
                            },
                        ),
                        TokenKind::SlashEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::Division {
                                left,
                                slash_equals: span,
                                right,
                            },
                        ),
                        TokenKind::PercentEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression::Modulo {
                                left,
                                percent_equals: span,
                                right,
                            })
                        }
                        TokenKind::PowEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::Exponentiation {
                                left,
                                pow_equals: span,
                                right,
                            },
                        ),
                        TokenKind::AmpersandEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::BitwiseAnd {
                                left,
                                ampersand_equals: span,
                                right,
                            },
                        ),
                        TokenKind::PipeEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::BitwiseOr {
                                left,
                                pipe_equals: span,
                                right,
                            },
                        ),
                        TokenKind::CaretEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::BitwiseXor {
                                left,
                                caret_equals: span,
                                right,
                            },
                        ),
                        TokenKind::LeftShiftEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::LeftShift {
                                left,
                                left_shift_equals: span,
                                right,
                            },
                        ),
                        TokenKind::RightShiftEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::RightShift {
                                left,
                                right_shift_equals: span,
                                right,
                            },
                        ),
                        TokenKind::DoubleQuestionEquals => ExpressionKind::AssignmentOperation(
                            AssignmentOperationExpression::Coalesce {
                                left,
                                coalesce_equals: span,
                                right,
                            },
                        ),
                        TokenKind::DotEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression::Concat {
                                left,
                                dot_equals: span,
                                right,
                            })
                        }
                        TokenKind::Ampersand => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression::And {
                                left,
                                and: span,
                                right,
                            })
                        }
                        TokenKind::Pipe => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression::Or {
                                left,
                                or: span,
                                right,
                            })
                        }
                        TokenKind::Caret => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression::Xor {
                                left,
                                xor: span,
                                right,
                            })
                        }
                        TokenKind::LeftShift => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression::LeftShift {
                                left,
                                left_shift: span,
                                right,
                            })
                        }
                        TokenKind::RightShift => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression::RightShift {
                                left,
                                right_shift: span,
                                right,
                            })
                        }
                        TokenKind::DoubleEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression::Equal {
                                left,
                                double_equals: span,
                                right,
                            })
                        }
                        TokenKind::TripleEquals => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::Identical {
                                left,
                                triple_equals: span,
                                right,
                            },
                        ),
                        TokenKind::BangEquals => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::NotEqual {
                                left,
                                bang_equals: span,
                                right,
                            },
                        ),
                        TokenKind::AngledLeftRight => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::AngledNotEqual {
                                left,
                                angled_left_right: span,
                                right,
                            },
                        ),
                        TokenKind::BangDoubleEquals => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::NotIdentical {
                                left,
                                bang_double_equals: span,
                                right,
                            },
                        ),
                        TokenKind::LessThan => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::LessThan {
                                left,
                                less_than: span,
                                right,
                            },
                        ),
                        TokenKind::GreaterThan => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::GreaterThan {
                                left,
                                greater_than: span,
                                right,
                            },
                        ),
                        TokenKind::LessThanEquals => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::LessThanOrEqual {
                                left,
                                less_than_equals: span,
                                right,
                            },
                        ),
                        TokenKind::GreaterThanEquals => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::GreaterThanOrEqual {
                                left,
                                greater_than_equals: span,
                                right,
                            },
                        ),
                        TokenKind::Spaceship => ExpressionKind::ComparisonOperation(
                            ComparisonOperationExpression::Spaceship {
                                left,
                                spaceship: span,
                                right,
                            },
                        ),
                        TokenKind::BooleanAnd => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression::And {
                                left,
                                double_ampersand: span,
                                right,
                            })
                        }
                        TokenKind::BooleanOr => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression::Or {
                                left,
                                double_pipe: span,
                                right,
                            })
                        }
                        TokenKind::LogicalAnd => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression::LogicalAnd {
                                left,
                                and: span,
                                right,
                            })
                        }
                        TokenKind::LogicalOr => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression::LogicalOr {
                                left,
                                or: span,
                                right,
                            })
                        }
                        TokenKind::LogicalXor => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression::LogicalXor {
                                left,
                                xor: span,
                                right,
                            })
                        }
                        TokenKind::Dot => ExpressionKind::Concat(ConcatExpression {
                            left,
                            dot: span,
                            right,
                        }),
                        TokenKind::Instanceof => ExpressionKind::Instanceof(InstanceofExpression {
                            left,
                            instanceof: span,
                            right,
                        }),
                        _ => todo!(),
                    }
                }
            };

            let end_span = state.stream.previous().span;

            left = Expression::new(
                kind,
                Span::new(start_span.start, end_span.end),
                CommentGroup::default()
            );

            continue;
        }

        break;
    }

    Ok(left)
}

fn left(state: &mut State, precedence: &Precedence) -> ParseResult<Expression> {
    if state.stream.is_eof() {
        return Err(error::unexpected_token(vec![], state.stream.current()));
    }

    attributes(state, precedence)
}

macro_rules! expressions {
    (
        using($state:ident):

        $(
            #[before($else:ident), $(precedence($precedence:expr),)? current($(|)? $( $current:pat_param )|+) $(, peek($(|)? $( $peek:pat_param )|+))?]
            $expr:ident($out:tt)
        )+
    ) => {
        $(
            pub(in crate) fn $expr($state: &mut State, precedence: &Precedence) -> ParseResult<Expression> {
                $(
                    if &$precedence < precedence {
                        return $else($state, precedence);
                    }
                )?

                match &$state.stream.current().kind {
                    $( $current )|+ $( if matches!(&$state.stream.peek().kind, $( $peek )|+ ))? => $out,
                    _ => $else($state, precedence),
                }
            }
        )+
    };
}

expressions! {
    using(state):

    #[before(static_arrow_function), current(TokenKind::Attribute)]
    attributes({
        attributes::gather_attributes(state)?;

        let current = state.stream.current();

        match &current.kind {
            TokenKind::Static if state.stream.peek().kind == TokenKind::Function => {
                functions::anonymous_function(state)
            }
            TokenKind::Static if state.stream.peek().kind == TokenKind::Fn => {
                functions::arrow_function(state)
            }
            TokenKind::Function => functions::anonymous_function(state),
            TokenKind::Fn => functions::arrow_function(state),
            _ => {
                Err(error::missing_item_definition_after_attributes(
                    &state.attributes,
                    current,
                ))
            }
        }
    })

    #[before(static_anonymous_function), current(TokenKind::Static), peek(TokenKind::Fn)]
    static_arrow_function({
        functions::arrow_function(state)
    })

    #[before(arrow_function), current(TokenKind::Static), peek(TokenKind::Function)]
    static_anonymous_function({
        functions::anonymous_function(state)
    })

    #[before(anonymous_function), current(TokenKind::Fn)]
    arrow_function({
        functions::arrow_function(state)
    })

    #[before(eval), current(TokenKind::Function)]
    anonymous_function({
        functions::anonymous_function(state)
    })

    #[before(empty), current(TokenKind::Eval), peek(TokenKind::LeftParen)]
    eval({
        let start_span = state.stream.current().span;
        let eval = state.stream.current().span;
        state.stream.next();

        let argument = Box::new(parameters::single_argument(state, true, true).unwrap()?);
        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Eval(EvalExpression { eval, argument }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(die), current(TokenKind::Empty), peek(TokenKind::LeftParen)]
    empty({
        let start_span = state.stream.current().span;
        let empty = state.stream.current().span;
        state.stream.next();

        let argument = Box::new(parameters::single_argument(state, true, true).unwrap()?);
        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Empty(EmptyExpression { empty, argument }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(exit), current(TokenKind::Die)]
    die({
        let start_span = state.stream.current().span;
        let die = state.stream.current().span;
        state.stream.next();

        let argument = match parameters::single_argument(state, false, true) {
            Some(arg) => Some(Box::new(arg?)),
            None => None,
        };

        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Die(DieExpression { die, argument }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(isset), current(TokenKind::Exit)]
    exit({
        let start_span = state.stream.current().span;
        let exit = state.stream.current().span;
        state.stream.next();

        let argument = match parameters::single_argument(state, false, true) {
            Some(arg) => Some(Box::new(arg?)),
            None => None,
        };

        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Exit(ExitExpression { exit, argument }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(unset), current(TokenKind::Isset), peek(TokenKind::LeftParen)]
    isset({
        let start_span = state.stream.current().span;
        let isset = state.stream.current().span;
        state.stream.next();
        let arguments = parameters::argument_list(state)?;
        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Isset(IssetExpression { isset, arguments }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(print), current(TokenKind::Unset), peek(TokenKind::LeftParen)]
    unset({
        let start_span = state.stream.current().span;
        let unset = state.stream.current().span;
        state.stream.next();
        let arguments = parameters::argument_list(state)?;
        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Unset(UnsetExpression { unset, arguments }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(reserved_identifier_function_call), current(TokenKind::Print)]
    print({
        let start_span = state.stream.current().span;
        let print = state.stream.current().span;
        state.stream.next();

        let mut value = None;
        let mut argument = None;

        if let Some(arg) = parameters::single_argument(state, false, true) {
            argument = Some(Box::new(arg?));
        } else {
            value = Some(Box::new(create(state)?));
        }

        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Print(PrintExpression { print, value, argument }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(reserved_identifier_static_call), precedence(Precedence::CallDim), current(
        | TokenKind::True       | TokenKind::False | TokenKind::Null
        | TokenKind::Readonly   | TokenKind::Self_ | TokenKind::Parent
        | TokenKind::Enum       | TokenKind::From
    ), peek(TokenKind::LeftParen)]
    reserved_identifier_function_call({
        let span = state.stream.current().span;
        let ident = identifiers::identifier_maybe_soft_reserved(state)?;
        let lhs = Expression::new(
            ExpressionKind::Identifier(Identifier::SimpleIdentifier(ident)),
            span,
            CommentGroup::default()
        );

        postfix(state, lhs, &TokenKind::LeftParen)
    })

    #[before(list), current(TokenKind::Enum | TokenKind::From), peek(TokenKind::DoubleColon)]
    reserved_identifier_static_call({
        let span = state.stream.current().span;
        let ident = identifiers::type_identifier(state)?;
        let lhs = Expression::new(
            ExpressionKind::Identifier(Identifier::SimpleIdentifier(ident)),
            span,
            CommentGroup::default()
        );

        postfix(state, lhs, &TokenKind::DoubleColon)
    })

    #[before(anonymous_class), current(TokenKind::List)]
    list({
        arrays::list_expression(state)
    })

    #[before(throw), current(TokenKind::New), peek(TokenKind::Class | TokenKind::Attribute)]
    anonymous_class({
        classes::parse_anonymous(state, None)
    })

    #[before(r#yield), current(TokenKind::Throw)]
    throw({
        let start_span = state.stream.current().span;
        state.stream.next();
        let exception = for_precedence(state, Precedence::Lowest)?;
        let exception_span = exception.span;

        Ok(Expression::new(
            ExpressionKind::Throw(ThrowExpression {
                value: Box::new(exception),
            }),
            Span::new(start_span.start, exception_span.end),
            CommentGroup::default()
        ))
    })

    #[before(clone), current(TokenKind::Yield)]
    r#yield({
        let start_span = state.stream.current().span;
        state.stream.next();
        if state.stream.current().kind == TokenKind::SemiColon || state.stream.current().kind == TokenKind::RightParen {
            Ok(Expression::new(
                ExpressionKind::Yield(YieldExpression {
                    key: None,
                    value: None,
                }),
                start_span,
                CommentGroup::default()
            ))
        } else {
            let mut from = false;

            if state.stream.current().kind == TokenKind::From {
                state.stream.next();
                from = true;
            }

            let mut key = None;
            let mut value = Box::new(for_precedence(
                state,
                if from {
                    Precedence::YieldFrom
                } else {
                    Precedence::Yield
                },
            )?);

            if state.stream.current().kind == TokenKind::DoubleArrow && !from {
                state.stream.next();
                key = Some(value.clone());
                value = Box::new(for_precedence(state, Precedence::Yield)?);
            }

            let end_span = state.stream.previous().span;

            if from {
                Ok(Expression::new(
                    ExpressionKind::YieldFrom(YieldFromExpression { value }),
                    Span::new(start_span.start, end_span.end),
                    CommentGroup::default()
                ))
            } else {
                Ok(Expression::new(
                    ExpressionKind::Yield(YieldExpression { key, value: Some(value) }),
                    Span::new(start_span.start, end_span.end),
                    CommentGroup::default()
                ))
            }
        }
    })

    #[before(r#true), current(TokenKind::Clone)]
    clone({
        let start_span = state.stream.current().span;
        state.stream.next();

        let target = for_precedence(state, Precedence::CloneOrNew)?;

        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            ExpressionKind::Clone(CloneExpression {
                target: Box::new(target),
            }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(r#false), current(TokenKind::True)]
    r#true({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::Bool(BoolExpression { value: true }),
            span,
            CommentGroup::default()
        ))
    })

    #[before(null), current(TokenKind::False)]
    r#false({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::Bool(BoolExpression { value: false }),
            span,
            CommentGroup::default()
        ))
    })

    #[before(literal_integer), current(TokenKind::Null)]
    null({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::Null,
            span,
            CommentGroup::default()
        ))
    })

    #[before(literal_float), current(TokenKind::LiteralInteger)]
    literal_integer({
        let span = state.stream.current().span;
        let current = state.stream.current();

        if let TokenKind::LiteralInteger = &current.kind {
            state.stream.next();

            Ok(Expression::new(
                ExpressionKind::Literal(Literal::Integer(
                    LiteralInteger {
                        span: current.span,
                        value: current.value.clone()
                    }
                )),
                span,
                CommentGroup::default()
            ))
        } else {
            unreachable!("{}:{}", file!(), line!());
        }
    })

    #[before(literal_string), current(TokenKind::LiteralFloat)]
    literal_float({
        let span = state.stream.current().span;
        let current = state.stream.current();

        if let TokenKind::LiteralFloat = &current.kind {
            state.stream.next();

            Ok(Expression::new(
                ExpressionKind::Literal(Literal::Float(
                    LiteralFloat {
                        span: current.span,
                        value: current.value.clone()
                    }
                )),
                span,
                CommentGroup::default()
            ))
        } else {
            unreachable!("{}:{}", file!(), line!());
        }
    })

    #[before(string_part), current(TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString)]
    literal_string({
        let span = state.stream.current().span;
        let current = state.stream.current();

        if let TokenKind::LiteralSingleQuotedString = &current.kind {
            state.stream.next();

            Ok(Expression::new(
                ExpressionKind::Literal(Literal::String(
                    LiteralString {
                        span: current.span,
                        value: current.value.clone(),
                        kind: LiteralStringKind::SingleQuoted,
                    }
                )),
                span,
                CommentGroup::default()
            ))
        } else if let TokenKind::LiteralDoubleQuotedString = &current.kind {
            state.stream.next();

            Ok(Expression::new(
                ExpressionKind::Literal(Literal::String(
                    LiteralString {
                        span: current.span,
                        value: current.value.clone(),
                        kind: LiteralStringKind::DoubleQuoted,
                    }
                )),
                span,
                CommentGroup::default()
            ))
        } else {
            unreachable!("{}:{}", file!(), line!());
        }
    })

    #[before(heredoc), current(TokenKind::StringPart)]
    string_part({
        strings::interpolated(state)
    })

    #[before(nowdoc), current(TokenKind::StartDocString(DocStringKind::Heredoc))]
    heredoc({
        strings::heredoc(state)
    })

    #[before(backtick), current(TokenKind::StartDocString(DocStringKind::Nowdoc))]
    nowdoc({
        strings::nowdoc(state)
    })

    #[before(identifier), current(TokenKind::Backtick)]
    backtick({
        strings::shell_exec(state)
    })

    #[before(static_postfix), current(TokenKind::Identifier | TokenKind::QualifiedIdentifier | TokenKind::FullyQualifiedIdentifier)]
    identifier({
        let identifier = identifiers::full_name(state)?;
        let identifier_span = identifier.span;
        
        Ok(Expression::new(
            ExpressionKind::Identifier(Identifier::SimpleIdentifier(identifier)),
            identifier_span,
            CommentGroup::default()
        ))
    })

    #[before(self_identifier), current(TokenKind::Static)]
    static_postfix({
        let span = state.stream.current().span;
        state.stream.next();
        let expression = Expression::new(
            ExpressionKind::Static,
            span,
            CommentGroup::default()
        );

        postfix(state, expression, &TokenKind::DoubleColon)
    })

    #[before(parent_identifier), current(TokenKind::Self_)]
    self_identifier({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::Self_,
            span,
            CommentGroup::default()
        ))
    })

    #[before(left_parenthesis), current(TokenKind::Parent)]
    parent_identifier({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::Parent,
            span,
            CommentGroup::default()
        ))
    })

    #[before(r#match), current(TokenKind::LeftParen)]
    left_parenthesis({
        let start = state.stream.current().span;
        state.stream.next();

        let expr = create(state)?;

        let end = utils::skip_right_parenthesis(state)?;

        Ok(Expression::new(
            ExpressionKind::Parenthesized(ParenthesizedExpression { start, expr: Box::new(expr), end }),
            Span::new(start.start, end.end),
            CommentGroup::default()
        ))
    })

    #[before(array), current(TokenKind::Match)]
    r#match({
        control_flow::match_expression(state)
    })

    #[before(left_bracket), current(TokenKind::Array)]
    array({
        arrays::array_expression(state)
    })

    #[before(new), current(TokenKind::LeftBracket)]
    left_bracket({
        arrays::short_array_expression(state)
    })

    #[before(directory_magic_constant), current(TokenKind::New)]
    new({
        let new = state.stream.current().span;

        state.stream.next();

        if state.stream.current().kind == TokenKind::Class || state.stream.current().kind == TokenKind::Attribute {
            return classes::parse_anonymous(state, Some(new));
        };

        let current_span = state.stream.current().span;
        let target = match state.stream.current().kind {
            TokenKind::Self_ => {
                state.stream.next();

                Expression::new(
                    ExpressionKind::Self_,
                    current_span,
                    CommentGroup::default()
                )
            }
            TokenKind::Static => {
                state.stream.next();

                Expression::new(
                    ExpressionKind::Static,
                    current_span,
                    CommentGroup::default()
                )
            }
            TokenKind::Parent => {
                state.stream.next();

                Expression::new(
                    ExpressionKind::Parent,
                    current_span,
                    CommentGroup::default()
                )
            }
            TokenKind::Enum => {
                let span = state.stream.current().span;

                state.stream.next();

                Expression::new(
                    ExpressionKind::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { span, value: "enum".into() })),
                    span,
                    CommentGroup::default()
                )
            }
            TokenKind::From => {
                let span = state.stream.current().span;

                state.stream.next();

                Expression::new(
                    ExpressionKind::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { span, value: "from".into() })),
                    span,
                    CommentGroup::default()
                )
            }
            _ => clone_or_new_precedence(state)?,
        };

        let arguments = if state.stream.current().kind == TokenKind::LeftParen {
            Some(parameters::argument_list(state)?)
        } else {
            None
        };

        Ok(Expression::new(
            ExpressionKind::New(NewExpression {
                target: Box::new(target),
                new,
                arguments,
            }),
            Span::new(new.start, current_span.end),
            CommentGroup::default()
        ))
    })

    #[before(file_magic_constant), current(TokenKind::DirConstant)]
    directory_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Directory(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(line_magic_constant), current(TokenKind::FileConstant)]
    file_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::File(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(function_magic_constant), current(TokenKind::LineConstant)]
    line_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Line(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(class_magic_constant), current(TokenKind::FunctionConstant)]
    function_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Function(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(method_magic_constant), current(TokenKind::ClassConstant)]
    class_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Class(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(namespace_magic_constant), current(TokenKind::MethodConstant)]
    method_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Method(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(trait_magic_constant), current(TokenKind::NamespaceConstant)]
    namespace_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Namespace(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(compiler_halt_offset_magic_constant), current(TokenKind::TraitConstant)]
    trait_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::Trait(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(include), current(TokenKind::CompilerHaltOffsetConstant)]
    compiler_halt_offset_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::new(
            ExpressionKind::MagicConstant(MagicConstantExpression::CompilerHaltOffset(span)),
            span,
            CommentGroup::default()
        ))
    })

    #[before(cast_prefix), current(TokenKind::Include | TokenKind::IncludeOnce | TokenKind::Require | TokenKind::RequireOnce)]
    include({
        let start_span = state.stream.current().span;
        let current = state.stream.current();
        let span = current.span;

        state.stream.next();

        let path = Box::new(create(state)?);

        let kind = match current.kind {
            TokenKind::Include => ExpressionKind::Include(IncludeExpression { include: span, path }),
            TokenKind::IncludeOnce => ExpressionKind::IncludeOnce(IncludeOnceExpression { include_once: span, path }),
            TokenKind::Require => ExpressionKind::Require(RequireExpression { require: span, path }),
            TokenKind::RequireOnce => ExpressionKind::RequireOnce(RequireOnceExpression { require_once: span, path }),
            _ => unreachable!()
        };

        let end_span = state.stream.previous().span;

        Ok(Expression::new(
            kind,
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(numeric_prefix), current(
        | TokenKind::StringCast     | TokenKind::BinaryCast     | TokenKind::ObjectCast
        | TokenKind::BoolCast       | TokenKind::BooleanCast    | TokenKind::IntCast
        | TokenKind::IntegerCast    | TokenKind::FloatCast      | TokenKind::DoubleCast
        | TokenKind::RealCast       | TokenKind::UnsetCast      | TokenKind::ArrayCast
    )]
    cast_prefix({
        let current = state.stream.current();

        let span = current.span;
        let kind = current.kind.clone().into();

        state.stream.next();

        let rhs = for_precedence(state, Precedence::Prefix)?;
        let rhs_span = rhs.span;

        Ok(Expression::new(
            ExpressionKind::Cast(CastExpression {
                cast: span,
                kind,
                value: Box::new(rhs),
            }),
            Span::new(span.start, rhs_span.end),
            CommentGroup::default()
        ))
    })

    #[before(bang_prefix), current(TokenKind::Decrement | TokenKind::Increment | TokenKind::Minus | TokenKind::Plus)]
    numeric_prefix({
        let start_span = state.stream.current().span;
        let current = state.stream.current();

        let span = current.span;
        let op = current.kind.clone();

        state.stream.next();

        let right = Box::new(for_precedence(state, Precedence::Prefix)?);
        let right_span = right.span;
        let expr = match op {
            TokenKind::Minus => ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::Negative { minus: span, right }),
            TokenKind::Plus => ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::Positive { plus: span, right }),
            TokenKind::Decrement => ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::PreDecrement { decrement: span, right }),
            TokenKind::Increment => ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::PreIncrement { increment: span, right }),
            _ => unreachable!(),
        };

        Ok(Expression::new(
            expr,
            Span::new(start_span.start, right_span.end),
            CommentGroup::default()
        ))
    })

    #[before(at_prefix), current(TokenKind::Bang)]
    bang_prefix({
        let start_span = state.stream.current().span;
        let bang = state.stream.current().span;

        state.stream.next();

        let rhs = for_precedence(state, Precedence::Bang)?;
        let end_span = rhs.span;

        Ok(Expression::new(
            ExpressionKind::LogicalOperation(LogicalOperationExpression::Not { bang, right: Box::new(rhs) }),
            Span::new(start_span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(bitwise_prefix), current(TokenKind::At)]
    at_prefix({
        let span = state.stream.current().span;

        state.stream.next();

        let rhs = for_precedence(state, Precedence::Prefix)?;
        let end_span = rhs.span;

        Ok(Expression::new(
            ExpressionKind::ErrorSuppress(ErrorSuppressExpression {
                at: span,
                expr: Box::new(rhs)
            }),
            Span::new(span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(variable), current(TokenKind::BitwiseNot)]
    bitwise_prefix({
        let span = state.stream.current().span;

        state.stream.next();

        let right = Box::new(for_precedence(state, Precedence::Prefix)?);
        let end_span = right.span;

        Ok(Expression::new(
            ExpressionKind::BitwiseOperation(BitwiseOperationExpression::Not { not: span, right }),
            Span::new(span.start, end_span.end),
            CommentGroup::default()
        ))
    })

    #[before(unexpected_token), current(TokenKind::Dollar | TokenKind::DollarLeftBrace | TokenKind::Variable)]
    variable({
        let span = state.stream.current().span;

        Ok(Expression::new(
            ExpressionKind::Variable(variables::dynamic_variable(state)?),
            span,
            CommentGroup::default()
        ))
    })
}

fn unexpected_token(state: &mut State, _: &Precedence) -> ParseResult<Expression> {
    let current = state.stream.current();

    Err(error::unexpected_token(vec![], current))
}

fn postfix(state: &mut State, lhs: Expression, op: &TokenKind) -> ParseResult<Expression> {
    let start_span = state.stream.current().span;
    let kind = match op {
        TokenKind::DoubleQuestion => {
            let double_question = state.stream.current().span;
            state.stream.next();

            let rhs = null_coalesce_precedence(state)?;

            ExpressionKind::Coalesce(CoalesceExpression {
                lhs: Box::new(lhs),
                double_question,
                rhs: Box::new(rhs),
            })
        }
        TokenKind::LeftParen => {
            // `(...)` closure creation
            if state.stream.lookahead(0).kind == TokenKind::Ellipsis
                && state.stream.lookahead(1).kind == TokenKind::RightParen
            {
                let start = utils::skip(state, TokenKind::LeftParen)?;
                let ellipsis = utils::skip(state, TokenKind::Ellipsis)?;
                let end = utils::skip(state, TokenKind::RightParen)?;

                let placeholder = ArgumentPlaceholder {
                    comments: state.stream.comments(),
                    left_parenthesis: start,
                    ellipsis,
                    right_parenthesis: end,
                };

                ExpressionKind::FunctionClosureCreation(FunctionClosureCreationExpression {
                    target: Box::new(lhs),
                    placeholder,
                })
            } else {
                let arguments = parameters::argument_list(state)?;

                ExpressionKind::FunctionCall(FunctionCallExpression {
                    target: Box::new(lhs),
                    arguments,
                })
            }
        }
        TokenKind::LeftBracket => ExpressionKind::ArrayIndex(ArrayIndexExpression {
            array: Box::new(lhs),
            left_bracket: utils::skip_left_bracket(state)?,
            index: if state.stream.current().kind == TokenKind::RightBracket {
                None
            } else {
                Some(create(state).map(Box::new)?)
            },
            right_bracket: utils::skip_right_bracket(state)?,
        }),
        TokenKind::DoubleColon => {
            let span = utils::skip_double_colon(state)?;

            let current = state.stream.current();

            let property = match current.kind {
                TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
                    ExpressionKind::Variable(variables::dynamic_variable(state)?)
                }
                _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
                    ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                        identifiers::identifier_maybe_reserved(state)?,
                    ))
                }
                TokenKind::LeftBrace => {
                    state.stream.next();

                    ExpressionKind::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
                        start: current.span,
                        expr: Box::new(create(state)?),
                        end: utils::skip_right_brace(state)?,
                    }))
                }
                TokenKind::Class => {
                    state.stream.next();

                    ExpressionKind::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
                        span: current.span,
                        value: "class".into(),
                    }))
                }
                _ => {
                    return expected_token_err!(["`{`", "`$`", "an identifier"], state);
                }
            };

            let lhs = Box::new(lhs);

            if state.stream.current().kind == TokenKind::LeftParen {
                if state.stream.lookahead(0).kind == TokenKind::Ellipsis
                    && state.stream.lookahead(1).kind == TokenKind::RightParen
                {
                    let start = utils::skip(state, TokenKind::LeftParen)?;
                    let ellipsis = utils::skip(state, TokenKind::Ellipsis)?;
                    let end = utils::skip(state, TokenKind::RightParen)?;

                    let placeholder = ArgumentPlaceholder {
                        comments: state.stream.comments(),
                        left_parenthesis: start,
                        ellipsis,
                        right_parenthesis: end,
                    };

                    match property {
                        ExpressionKind::Identifier(identifier) => {
                            ExpressionKind::StaticMethodClosureCreation(
                                StaticMethodClosureCreationExpression {
                                    target: lhs,
                                    double_colon: span,
                                    method: identifier,
                                    placeholder,
                                },
                            )
                        }
                        ExpressionKind::Variable(variable) => {
                            ExpressionKind::StaticVariableMethodClosureCreation(
                                StaticVariableMethodClosureCreationExpression {
                                    target: lhs,
                                    double_colon: span,
                                    method: variable,
                                    placeholder,
                                },
                            )
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let arguments = parameters::argument_list(state)?;

                    match property {
                        ExpressionKind::Identifier(identifier) => {
                            ExpressionKind::StaticMethodCall(StaticMethodCallExpression {
                                target: lhs,
                                double_colon: span,
                                method: identifier,
                                arguments,
                            })
                        }
                        ExpressionKind::Variable(variable) => ExpressionKind::StaticVariableMethodCall(
                            StaticVariableMethodCallExpression {
                                target: lhs,
                                double_colon: span,
                                method: variable,
                                arguments,
                            },
                        ),
                        _ => unreachable!(),
                    }
                }
            } else {
                match property {
                    ExpressionKind::Identifier(identifier) => {
                        ExpressionKind::ConstantFetch(ConstantFetchExpression {
                            target: lhs,
                            double_colon: span,
                            constant: identifier,
                        })
                    }
                    ExpressionKind::Variable(variable) => {
                        ExpressionKind::StaticPropertyFetch(StaticPropertyFetchExpression {
                            target: lhs,
                            double_colon: span,
                            property: variable,
                        })
                    }
                    _ => unreachable!(),
                }
            }
        }
        TokenKind::Arrow | TokenKind::QuestionArrow => {
            let span = state.stream.current().span;
            state.stream.next();

            let property = match state.stream.current().kind {
                TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
                    let start_span = state.stream.current().span;
                    let kind = ExpressionKind::Variable(variables::dynamic_variable(state)?);
                    let end_span = state.stream.previous().span;

                    Expression::new(kind, Span::new(start_span.start, end_span.end), CommentGroup::default())
                }
                _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
                    let start_span = state.stream.current().span;
                    let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                        identifiers::identifier_maybe_reserved(state)?,
                    ));
                    let end_span = state.stream.previous().span;

                    Expression::new(kind, Span::new(start_span.start, end_span.end), CommentGroup::default())
                }
                TokenKind::LeftBrace => {
                    let start = state.stream.current().span;
                    state.stream.next();

                    let name = create(state)?;

                    let end = utils::skip_right_brace(state)?;

                    Expression::new(
                        ExpressionKind::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
                            start,
                            expr: Box::new(name),
                            end,
                        })),
                        Span::new(start.start, end.end),
                        CommentGroup::default()
                    )
                }
                _ => {
                    return expected_token_err!(["`{`", "`$`", "an identifier"], state);
                }
            };

            if state.stream.current().kind == TokenKind::LeftParen {
                if op == &TokenKind::QuestionArrow {
                    let arguments = parameters::argument_list(state)?;

                    ExpressionKind::NullsafeMethodCall(NullsafeMethodCallExpression {
                        target: Box::new(lhs),
                        method: Box::new(property),
                        question_arrow: span,
                        arguments,
                    })
                } else {
                    // `(...)` closure creation
                    if state.stream.lookahead(0).kind == TokenKind::Ellipsis
                        && state.stream.lookahead(1).kind == TokenKind::RightParen
                    {
                        let start = utils::skip(state, TokenKind::LeftParen)?;
                        let ellipsis = utils::skip(state, TokenKind::Ellipsis)?;
                        let end = utils::skip(state, TokenKind::RightParen)?;

                        let placeholder = ArgumentPlaceholder {
                            comments: state.stream.comments(),
                            left_parenthesis: start,
                            ellipsis,
                            right_parenthesis: end,
                        };

                        ExpressionKind::MethodClosureCreation(MethodClosureCreationExpression {
                            target: Box::new(lhs),
                            method: Box::new(property),
                            arrow: span,
                            placeholder,
                        })
                    } else {
                        let arguments = parameters::argument_list(state)?;

                        ExpressionKind::MethodCall(MethodCallExpression {
                            target: Box::new(lhs),
                            method: Box::new(property),
                            arrow: span,
                            arguments,
                        })
                    }
                }
            } else if op == &TokenKind::QuestionArrow {
                ExpressionKind::NullsafePropertyFetch(NullsafePropertyFetchExpression {
                    target: Box::new(lhs),
                    question_arrow: span,
                    property: Box::new(property),
                })
            } else {
                ExpressionKind::PropertyFetch(PropertyFetchExpression {
                    target: Box::new(lhs),
                    arrow: span,
                    property: Box::new(property),
                })
            }
        }
        TokenKind::Increment => {
            let span = state.stream.current().span;
            state.stream.next();

            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::PostIncrement {
                left: Box::new(lhs),
                increment: span,
            })
        }
        TokenKind::Decrement => {
            let span = state.stream.current().span;
            state.stream.next();

            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression::PostDecrement {
                left: Box::new(lhs),
                decrement: span,
            })
        }
        _ => todo!("postfix: {:?}", op),
    };

    let end_span = state.stream.previous().span;

    Ok(Expression::new(
        kind,
        Span::new(start_span.start, end_span.end),
        CommentGroup::default()
    ))
}

fn is_infix(t: &TokenKind) -> bool {
    matches!(
        t,
        TokenKind::Pow
            | TokenKind::RightShiftEquals
            | TokenKind::LeftShiftEquals
            | TokenKind::CaretEquals
            | TokenKind::AmpersandEquals
            | TokenKind::PipeEquals
            | TokenKind::PercentEquals
            | TokenKind::PowEquals
            | TokenKind::LogicalAnd
            | TokenKind::LogicalOr
            | TokenKind::LogicalXor
            | TokenKind::Spaceship
            | TokenKind::LeftShift
            | TokenKind::RightShift
            | TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::Percent
            | TokenKind::Instanceof
            | TokenKind::Asterisk
            | TokenKind::Slash
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Dot
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThanEquals
            | TokenKind::DoubleEquals
            | TokenKind::TripleEquals
            | TokenKind::BangEquals
            | TokenKind::BangDoubleEquals
            | TokenKind::AngledLeftRight
            | TokenKind::Question
            | TokenKind::QuestionColon
            | TokenKind::BooleanAnd
            | TokenKind::BooleanOr
            | TokenKind::Equals
            | TokenKind::PlusEquals
            | TokenKind::MinusEquals
            | TokenKind::DotEquals
            | TokenKind::DoubleQuestionEquals
            | TokenKind::AsteriskEquals
            | TokenKind::SlashEquals
    )
}

#[inline(always)]
fn is_postfix(t: &TokenKind) -> bool {
    matches!(
        t,
        TokenKind::Increment
            | TokenKind::Decrement
            | TokenKind::LeftParen
            | TokenKind::LeftBracket
            | TokenKind::Arrow
            | TokenKind::QuestionArrow
            | TokenKind::DoubleColon
            | TokenKind::DoubleQuestion
    )
}
