use crate::internal::arrays;
use crate::internal::attributes;
use crate::internal::classes;
use crate::internal::control_flow;
use crate::internal::functions;
use crate::internal::identifiers;
use crate::internal::names;
use crate::internal::parameters;
use crate::internal::precedences::Associativity;
use crate::internal::precedences::Precedence;
use crate::internal::strings;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::Expression;
use pxp_ast::*;
use pxp_ast::{
    ArrayIndexExpression, CoalesceExpression, ConcatExpression, ConstantFetchExpression,
    ExpressionKind, FunctionCallExpression, FunctionClosureCreationExpression,
    InstanceofExpression, MagicConstantExpression, MethodCallExpression,
    MethodClosureCreationExpression, NullsafeMethodCallExpression, NullsafePropertyFetchExpression,
    PropertyFetchExpression, ReferenceExpression, ShortTernaryExpression,
    StaticMethodCallExpression, StaticMethodClosureCreationExpression,
    StaticPropertyFetchExpression, StaticVariableMethodCallExpression,
    StaticVariableMethodClosureCreationExpression, TernaryExpression,
};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

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

pub fn create(state: &mut State) -> Expression {
    for_precedence(state, Precedence::Lowest)
}

fn null_coalesce_precedence(state: &mut State) -> Expression {
    for_precedence(state, Precedence::NullCoalesce)
}

fn clone_or_new_precedence(state: &mut State) -> Expression {
    for_precedence(state, Precedence::CloneOrNew)
}

fn for_precedence(state: &mut State, precedence: Precedence) -> Expression {
    let mut left = left(state, &precedence);

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

            left = postfix(state, left, kind);
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
                state.diagnostic(
                    ParserDiagnostic::UnexpectedToken {
                        token: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );
            }

            state.stream.next();

            let op = state.stream.current();
            let start_span = op.span;
            let kind = match kind {
                TokenKind::Question => {
                    // this happens due to a comment, or whitespaces between the  and the :
                    // we consider `foo()  : bar()` a ternary expression, with `then` being a noop
                    // however, this must behave like a short ternary at runtime.
                    if op.kind == TokenKind::Colon {
                        state.stream.next();

                        let r#else = create(state);

                        ExpressionKind::Ternary(TernaryExpression {
                            id: state.id(),
                            span: Span::combine(left.span, r#else.span),
                            condition: Box::new(left),
                            question: span,
                            then: Box::new(Expression::noop(state.id(), start_span)),
                            colon: op.span,
                            r#else: Box::new(r#else),
                        })
                    } else {
                        let then = create(state);
                        let colon = utils::skip_colon(state);
                        let r#else = create(state);

                        ExpressionKind::Ternary(TernaryExpression {
                            id: state.id(),
                            span: Span::combine(left.span, r#else.span),
                            condition: Box::new(left),
                            question: span,
                            then: Box::new(then),
                            colon,
                            r#else: Box::new(r#else),
                        })
                    }
                }
                TokenKind::QuestionColon => {
                    let r#else = create(state);
                    ExpressionKind::ShortTernary(ShortTernaryExpression {
                        id: state.id(),
                        span: Span::combine(left.span, r#else.span),
                        condition: Box::new(left),
                        question_colon: span,
                        r#else: Box::new(r#else),
                    })
                }
                TokenKind::Equals if op.kind == TokenKind::Ampersand => {
                    state.stream.next();

                    // FIXME: You should only be allowed to assign a referencable variable,
                    //        here, not any old expression.
                    let right = Box::new(for_precedence(state, rpred));
                    let right_span = right.span;
                    let span = Span::combine(left.span, right_span);
                    let reference_span = Span::combine(op.span, right_span);

                    ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                        id: state.id(),
                        span,
                        kind: AssignmentOperationKind::Assign {
                            id: state.id(),
                            left: Box::new(left),
                            equals: span,
                            right: Box::new(Expression::new(
                                state.id(),
                                ExpressionKind::Reference(ReferenceExpression {
                                    id: state.id(),
                                    span: reference_span,
                                    ampersand: op.span,
                                    right,
                                }),
                                Span::new(start_span.start, right_span.end),
                                CommentGroup::default(),
                            )),
                        },
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Self_ => {
                    let self_span = op.span;
                    state.stream.next();
                    let right = Expression::new(
                        state.id(),
                        ExpressionKind::Self_(SelfExpression {
                            id: state.id(),
                            span: self_span,
                        }),
                        self_span,
                        CommentGroup::default(),
                    );
                    let span = Span::combine(left.span, right.span);

                    ExpressionKind::Instanceof(InstanceofExpression {
                        id: state.id(),
                        span,
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(right),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Parent => {
                    state.stream.next();
                    let right = Expression::new(
                        state.id(),
                        ExpressionKind::Parent(ParentExpression {
                            id: state.id(),
                            span: op.span,
                        }),
                        op.span,
                        CommentGroup::default(),
                    );
                    let span = Span::combine(left.span, right.span);

                    ExpressionKind::Instanceof(InstanceofExpression {
                        id: state.id(),
                        span,
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(right),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Static => {
                    let instanceof = span;
                    state.stream.next();
                    let right = Expression::new(
                        state.id(),
                        ExpressionKind::Static(StaticExpression {
                            id: state.id(),
                            span: op.span,
                        }),
                        op.span,
                        CommentGroup::default(),
                    );

                    ExpressionKind::Instanceof(InstanceofExpression {
                        id: state.id(),
                        span: Span::combine(left.span, right.span),
                        left: Box::new(left),
                        instanceof,
                        right: Box::new(right),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Enum => {
                    let enum_span = op.span;
                    state.stream.next();

                    let right = Expression::new(
                        state.id(),
                        ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                            SimpleIdentifier::new(
                                state.id(),
                                op.symbol.as_ref().unwrap().clone(),
                                enum_span,
                            ),
                        )),
                        enum_span,
                        CommentGroup::default(),
                    );

                    ExpressionKind::Instanceof(InstanceofExpression {
                        id: state.id(),
                        span: Span::combine(left.span, right.span),
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(right),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::From => {
                    let from_span = op.span;
                    state.stream.next();
                    let right = Expression::new(
                        state.id(),
                        ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                            SimpleIdentifier::new(
                                state.id(),
                                op.symbol.as_ref().unwrap().clone(),
                                op.span,
                            ),
                        )),
                        Span::new(start_span.start, from_span.end),
                        CommentGroup::default(),
                    );

                    ExpressionKind::Instanceof(InstanceofExpression {
                        id: state.id(),
                        span: Span::combine(left.span, right.span),
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(right),
                    })
                }
                _ => {
                    let op_span = span;
                    let left = Box::new(left);
                    let right = Box::new(for_precedence(state, rpred));
                    let span = Span::combine(left.span, right.span);

                    match kind {
                        TokenKind::Plus => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                id: state.id(),
                                span,
                                kind: ArithmeticOperationKind::Addition {
                                    id: state.id(),
                                    left,
                                    plus: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Minus => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                id: state.id(),
                                span,
                                kind: ArithmeticOperationKind::Subtraction {
                                    id: state.id(),
                                    left,
                                    minus: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Asterisk => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                id: state.id(),
                                span,
                                kind: ArithmeticOperationKind::Multiplication {
                                    id: state.id(),
                                    left,
                                    asterisk: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Slash => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                id: state.id(),
                                span,
                                kind: ArithmeticOperationKind::Division {
                                    id: state.id(),
                                    left,
                                    slash: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Percent => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                id: state.id(),
                                span,
                                kind: ArithmeticOperationKind::Modulo {
                                    id: state.id(),
                                    left,
                                    percent: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Pow => {
                            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                                id: state.id(),
                                span,
                                kind: ArithmeticOperationKind::Exponentiation {
                                    id: state.id(),
                                    left,
                                    pow: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Equals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Assign {
                                    id: state.id(),
                                    left,
                                    equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::PlusEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Addition {
                                    id: state.id(),
                                    left,
                                    plus_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::MinusEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Subtraction {
                                    id: state.id(),
                                    left,
                                    minus_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::AsteriskEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Multiplication {
                                    id: state.id(),
                                    left,
                                    asterisk_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::SlashEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Division {
                                    id: state.id(),
                                    left,
                                    slash_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::PercentEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Modulo {
                                    id: state.id(),
                                    left,
                                    percent_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::PowEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Exponentiation {
                                    id: state.id(),
                                    left,
                                    pow_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::AmpersandEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::BitwiseAnd {
                                    id: state.id(),
                                    left,
                                    ampersand_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::PipeEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::BitwiseOr {
                                    id: state.id(),
                                    left,
                                    pipe_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::CaretEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::BitwiseXor {
                                    id: state.id(),
                                    left,
                                    caret_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LeftShiftEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::LeftShift {
                                    id: state.id(),
                                    left,
                                    left_shift_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::RightShiftEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::RightShift {
                                    id: state.id(),
                                    left,
                                    right_shift_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::DoubleQuestionEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Coalesce {
                                    id: state.id(),
                                    left,
                                    coalesce_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::DotEquals => {
                            ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
                                id: state.id(),
                                span,
                                kind: AssignmentOperationKind::Concat {
                                    id: state.id(),
                                    left,
                                    dot_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Ampersand => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression {
                                id: state.id(),
                                span,
                                kind: BitwiseOperationKind::And {
                                    id: state.id(),
                                    left,
                                    and: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Pipe => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression {
                                id: state.id(),
                                span,
                                kind: BitwiseOperationKind::Or {
                                    id: state.id(),
                                    left,
                                    or: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Caret => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression {
                                id: state.id(),
                                span,
                                kind: BitwiseOperationKind::Xor {
                                    id: state.id(),
                                    left,
                                    xor: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LeftShift => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression {
                                id: state.id(),
                                span,
                                kind: BitwiseOperationKind::LeftShift {
                                    id: state.id(),
                                    left,
                                    left_shift: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::RightShift => {
                            ExpressionKind::BitwiseOperation(BitwiseOperationExpression {
                                id: state.id(),
                                span,
                                kind: BitwiseOperationKind::RightShift {
                                    id: state.id(),
                                    left,
                                    right_shift: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::DoubleEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::Equal {
                                    id: state.id(),
                                    left,
                                    double_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::TripleEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::Identical {
                                    id: state.id(),
                                    left,
                                    triple_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::BangEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::NotEqual {
                                    id: state.id(),
                                    left,
                                    bang_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::AngledLeftRight => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::AngledNotEqual {
                                    id: state.id(),
                                    left,
                                    angled_left_right: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::BangDoubleEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::NotIdentical {
                                    id: state.id(),
                                    left,
                                    bang_double_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LessThan => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::LessThan {
                                    id: state.id(),
                                    left,
                                    less_than: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::GreaterThan => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::GreaterThan {
                                    id: state.id(),
                                    left,
                                    greater_than: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LessThanEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::LessThanOrEqual {
                                    id: state.id(),
                                    left,
                                    less_than_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::GreaterThanEquals => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::GreaterThanOrEqual {
                                    id: state.id(),
                                    left,
                                    greater_than_equals: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Spaceship => {
                            ExpressionKind::ComparisonOperation(ComparisonOperationExpression {
                                id: state.id(),
                                span,
                                kind: ComparisonOperationKind::Spaceship {
                                    id: state.id(),
                                    left,
                                    spaceship: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::BooleanAnd => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression {
                                id: state.id(),
                                span,
                                kind: LogicalOperationKind::And {
                                    id: state.id(),
                                    left,
                                    double_ampersand: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::BooleanOr => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression {
                                id: state.id(),
                                span,
                                kind: LogicalOperationKind::Or {
                                    id: state.id(),
                                    left,
                                    double_pipe: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LogicalAnd => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression {
                                id: state.id(),
                                span,
                                kind: LogicalOperationKind::LogicalAnd {
                                    id: state.id(),
                                    left,
                                    and: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LogicalOr => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression {
                                id: state.id(),
                                span,
                                kind: LogicalOperationKind::LogicalOr {
                                    id: state.id(),
                                    left,
                                    or: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::LogicalXor => {
                            ExpressionKind::LogicalOperation(LogicalOperationExpression {
                                id: state.id(),
                                span,
                                kind: LogicalOperationKind::LogicalXor {
                                    id: state.id(),
                                    left,
                                    xor: op_span,
                                    right,
                                },
                            })
                        }
                        TokenKind::Dot => ExpressionKind::Concat(ConcatExpression {
                            id: state.id(),
                            span,
                            left,
                            dot: op_span,
                            right,
                        }),
                        TokenKind::Instanceof => ExpressionKind::Instanceof(InstanceofExpression {
                            id: state.id(),
                            span,
                            left,
                            instanceof: op_span,
                            right,
                        }),
                        _ => unreachable!(),
                    }
                }
            };

            let end_span = state.stream.previous().span;

            left = Expression::new(
                state.id(),
                kind,
                Span::new(start_span.start, end_span.end),
                CommentGroup::default(),
            );

            maybe_shift_assignment_operands(&mut left);

            continue;
        }

        break;
    }

    left
}

fn should_shift_assignment_operands(expr: &Expression) -> bool {
    let is_assignment = matches!(expr.kind, ExpressionKind::AssignmentOperation(_));

    if !is_assignment {
        return false;
    }

    let ExpressionKind::AssignmentOperation(AssignmentOperationExpression { kind, .. }) =
        &expr.kind
    else {
        unreachable!()
    };

    matches!(
        kind.left().kind,
        ExpressionKind::ComparisonOperation(_)
            | ExpressionKind::BitwiseOperation(_)
            | ExpressionKind::ArithmeticOperation(_)
            | ExpressionKind::LogicalOperation(_)
    )
}

// This is a workaround for a problem somebody reported, but something that
// I've found in other PHP parsers too.
//
// Given the following code:
//     true !== $a = true
// The precedence system interprets this as:
//     (true !== $a) = true
// which isn't a valid assignment target.
//
// This seems to be a downfall of the precedence system and a side effect
// of PHP treating assignment as an expression rather than a statement.
//
// I found a similar piece of logic in the `microsoft/tolerant-php-parser` project,
// where they check to see if the expression being created is an assignment operation
// and if the left-hand side if a binary operation.
//
// If it is, then they shift the operands around to fake parentheses, so that the expression
// is instead interpreted as:
//     true !== ($a = true)
//
// This is a real mega hack, but it seems to work and should be the only place where
// we need to do this sort of trickery-bobbery.
fn maybe_shift_assignment_operands(expr: &mut Expression) {
    if !should_shift_assignment_operands(expr) {
        return;
    }

    // At this point, we know that the left-hand side of the expression is an assignment.
    let ExpressionKind::AssignmentOperation(AssignmentOperationExpression { id, kind, .. }) =
        &expr.kind
    else {
        unreachable!()
    };

    // Given the following AST:
    // AssignmentOperation {
    //     left: ComparisonOperation {
    //         left: true,
    //         op: !==,
    //         right: $a
    //     },
    //     right: true
    // }
    //
    // We need to transform it into:
    // ComparisonOperation {
    //     left: true,
    //     op: !==,
    //     right: AssignmentOperation {
    //         left: $a,
    //         right: true
    //     }
    // }

    // So we first need to get the left-hand side of the assignment.
    // Which in the example above will be the ComparisonOperation.
    let assignment_left = kind.left();

    // We also need the right-hand side of the assignment since
    // that will be our new right-hand side too.
    let assignment_right = kind.right();

    // Then we need to get the right-hand side of the comparison, since
    // this is the real assignment target.
    let real_assignment_target = match &assignment_left.kind {
        ExpressionKind::ComparisonOperation(ComparisonOperationExpression { kind, .. }) => {
            Some(kind.right())
        }
        ExpressionKind::BitwiseOperation(BitwiseOperationExpression { kind, .. }) => {
            Some(kind.right())
        }
        ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression { kind, .. }) => {
            kind.right()
        }
        ExpressionKind::LogicalOperation(LogicalOperationExpression { kind, .. }) => {
            Some(kind.right())
        }
        _ => todo!(),
    };

    if let None = real_assignment_target {
        // This is a bit of a hack, but we can't really do anything about it.
        // If we can't find the real assignment target, then we can't shift the operands.
        return;
    }

    // Then we can create the new right-hand side of the comparison, which will
    // be an assignment expression.
    //
    // This is a bit lengthy since we need to match against the existing assignment to
    // make sure it's the right type of assignment.
    let new_right = Expression::new(
        expr.id,
        ExpressionKind::AssignmentOperation(AssignmentOperationExpression {
            id: *id,
            span: Span::default(),
            kind: match kind {
                AssignmentOperationKind::Assign { id, equals, .. } => {
                    AssignmentOperationKind::Assign {
                        id: *id,
                        left: Box::new(real_assignment_target.cloned().unwrap()),
                        equals: *equals,
                        right: Box::new(assignment_right.clone()),
                    }
                }
                AssignmentOperationKind::Addition {
                    id, plus_equals, ..
                } => AssignmentOperationKind::Addition {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    plus_equals: *plus_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::Subtraction {
                    id, minus_equals, ..
                } => AssignmentOperationKind::Subtraction {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    minus_equals: *minus_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::Multiplication {
                    id,
                    asterisk_equals,
                    ..
                } => AssignmentOperationKind::Multiplication {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    asterisk_equals: *asterisk_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::Division {
                    id, slash_equals, ..
                } => AssignmentOperationKind::Division {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    slash_equals: *slash_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::Modulo {
                    id, percent_equals, ..
                } => AssignmentOperationKind::Modulo {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    percent_equals: *percent_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::Exponentiation { id, pow_equals, .. } => {
                    AssignmentOperationKind::Exponentiation {
                        id: *id,
                        left: Box::new(real_assignment_target.cloned().unwrap()),
                        pow_equals: *pow_equals,
                        right: Box::new(assignment_right.clone()),
                    }
                }
                AssignmentOperationKind::Concat { id, dot_equals, .. } => {
                    AssignmentOperationKind::Concat {
                        id: *id,
                        left: Box::new(real_assignment_target.cloned().unwrap()),
                        dot_equals: *dot_equals,
                        right: Box::new(assignment_right.clone()),
                    }
                }
                AssignmentOperationKind::BitwiseAnd {
                    id,
                    ampersand_equals,
                    ..
                } => AssignmentOperationKind::BitwiseAnd {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    ampersand_equals: *ampersand_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::BitwiseOr {
                    id, pipe_equals, ..
                } => AssignmentOperationKind::BitwiseOr {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    pipe_equals: *pipe_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::BitwiseXor {
                    id, caret_equals, ..
                } => AssignmentOperationKind::BitwiseXor {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    caret_equals: *caret_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::LeftShift {
                    id,
                    left_shift_equals,
                    ..
                } => AssignmentOperationKind::LeftShift {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    left_shift_equals: *left_shift_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::RightShift {
                    id,
                    right_shift_equals,
                    ..
                } => AssignmentOperationKind::RightShift {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    right_shift_equals: *right_shift_equals,
                    right: Box::new(assignment_right.clone()),
                },
                AssignmentOperationKind::Coalesce {
                    id,
                    coalesce_equals,
                    ..
                } => AssignmentOperationKind::Coalesce {
                    id: *id,
                    left: Box::new(real_assignment_target.cloned().unwrap()),
                    coalesce_equals: *coalesce_equals,
                    right: Box::new(assignment_right.clone()),
                },
            },
        }),
        Span::default(),
        CommentGroup::default(),
    );

    // Then we need to create the new binary operation, which will replace
    // the existing assignment operation.
    let mut new_expression = assignment_left.clone();

    match &mut new_expression.kind {
        ExpressionKind::ComparisonOperation(ComparisonOperationExpression { kind, .. }) => {
            kind.set_right(Box::new(new_right))
        }
        ExpressionKind::BitwiseOperation(BitwiseOperationExpression { kind, .. }) => {
            kind.set_right(Box::new(new_right))
        }
        ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression { kind, .. }) => {
            kind.set_right(Box::new(new_right))
        }
        ExpressionKind::LogicalOperation(LogicalOperationExpression { kind, .. }) => {
            kind.set_right(Box::new(new_right))
        }
        _ => unreachable!(),
    };

    *expr = new_expression;
}

pub fn attributes(state: &mut State) -> Expression {
    attributes::gather_attributes(state);

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
            state.diagnostic(
                ParserDiagnostic::InvalidTargetForAttributes,
                Severity::Error,
                current.span,
            );

            Expression::missing(state.id(), current.span)
        }
    }
}

fn left(state: &mut State, precedence: &Precedence) -> Expression {
    if state.stream.is_eof() {
        state.diagnostic(
            ParserDiagnostic::UnexpectedEndOfFile,
            Severity::Error,
            state.stream.current().span,
        );

        return Expression::missing(state.id(), state.stream.current().span);
    }

    let current = state.stream.current();
    let peek = state.stream.peek();

    match (&current.kind, &peek.kind) {
        (TokenKind::Attribute, _) => attributes(state),

        (TokenKind::Static, TokenKind::Fn) => functions::arrow_function(state),

        (TokenKind::Static, TokenKind::Function) => functions::anonymous_function(state),

        (TokenKind::Fn, _) => functions::arrow_function(state),

        (TokenKind::Function, _) => functions::anonymous_function(state),

        (TokenKind::Eval, TokenKind::LeftParen) => {
            let start_span = state.stream.current().span;
            let eval = state.stream.current().span;
            state.stream.next();

            let argument = Box::new(parameters::single_argument(state, true, true).unwrap());
            let end_span = state.stream.previous().span;

            Expression::new(
                state.id(),
                ExpressionKind::Eval(EvalExpression {
                    id: state.id(),
                    span: Span::combine(start_span, end_span),
                    eval,
                    argument,
                }),
                Span::new(start_span.start, end_span.end),
                CommentGroup::default(),
            )
        }

        (TokenKind::Empty, TokenKind::LeftParen) => {
            let start_span = state.stream.current().span;
            let empty = state.stream.current().span;
            state.stream.next();

            let argument = Box::new(parameters::single_argument(state, true, true).unwrap());
            let end_span = state.stream.previous().span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::Empty(EmptyExpression {
                    id: state.id(),
                    span,
                    empty,
                    argument,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Die, _) => {
            let start_span = state.stream.current().span;
            let die = state.stream.current().span;
            state.stream.next();

            let argument = parameters::single_argument(state, false, true).map(Box::new);

            let end_span = state.stream.previous().span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::Die(DieExpression {
                    id: state.id(),
                    span,
                    die,
                    argument,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Exit, _) => {
            let start_span = state.stream.current().span;
            let exit = state.stream.current().span;
            state.stream.next();

            let argument = parameters::single_argument(state, false, true).map(Box::new);

            let end_span = state.stream.previous().span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::Exit(ExitExpression {
                    id: state.id(),
                    span,
                    exit,
                    argument,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Isset, TokenKind::LeftParen) => {
            let start_span = state.stream.current().span;
            let isset = state.stream.current().span;
            state.stream.next();
            let arguments = parameters::argument_list(state);
            let end_span = state.stream.previous().span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::Isset(IssetExpression {
                    id: state.id(),
                    span,
                    isset,
                    arguments,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Unset, TokenKind::LeftParen) => {
            let start_span = state.stream.current().span;
            let unset = state.stream.current().span;
            state.stream.next();
            let arguments = parameters::argument_list(state);
            let end_span = state.stream.previous().span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::Unset(UnsetExpression {
                    id: state.id(),
                    span,
                    unset,
                    arguments,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Print, _) => {
            let start_span = state.stream.current().span;
            let print = state.stream.current().span;
            state.stream.next();

            let mut value = None;
            let mut argument = None;

            if let Some(arg) = parameters::single_argument(state, false, true) {
                argument = Some(Box::new(arg));
            } else {
                value = Some(Box::new(create(state)));
            }

            let end_span = state.stream.previous().span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::Print(PrintExpression {
                    id: state.id(),
                    span,
                    print,
                    value,
                    argument,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (
            TokenKind::True
            | TokenKind::False
            | TokenKind::Null
            | TokenKind::Readonly
            | TokenKind::Self_
            | TokenKind::Parent
            | TokenKind::Enum
            | TokenKind::From,
            TokenKind::LeftParen,
        ) => {
            let name = names::name_maybe_soft_reserved(state, UseKind::Function);
            let span = name.span;

            let lhs = Expression::new(
                state.id(),
                ExpressionKind::Name(name),
                span,
                CommentGroup::default(),
            );

            postfix(state, lhs, &TokenKind::LeftParen)
        }

        (TokenKind::Enum | TokenKind::From, TokenKind::DoubleColon) => {
            let name = names::full_name_including_self(state);
            let span = name.span;

            let lhs = Expression::new(
                state.id(),
                ExpressionKind::Name(name),
                span,
                CommentGroup::default(),
            );

            postfix(state, lhs, &TokenKind::DoubleColon)
        }

        (TokenKind::List, _) => arrays::list_expression(state),

        (TokenKind::New, TokenKind::Class | TokenKind::Attribute) => {
            classes::parse_anonymous(state, None)
        }

        (TokenKind::Throw, _) => {
            let start_span = state.stream.current().span;
            state.stream.next();
            let exception = for_precedence(state, Precedence::Lowest);
            let exception_span = exception.span;
            let span = Span::combine(start_span, exception_span);

            Expression::new(
                state.id(),
                ExpressionKind::Throw(ThrowExpression {
                    id: state.id(),
                    span,
                    value: Box::new(exception),
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Yield, _) => {
            let start_span = state.stream.current().span;
            state.stream.next();
            if state.stream.current().kind == TokenKind::SemiColon
                || state.stream.current().kind == TokenKind::RightParen
            {
                Expression::new(
                    state.id(),
                    ExpressionKind::Yield(YieldExpression {
                        id: state.id(),
                        r#yield: start_span,
                        span: start_span,
                        key: None,
                        value: None,
                    }),
                    start_span,
                    CommentGroup::default(),
                )
            } else {
                let mut from = Span::default();

                if state.stream.current().kind == TokenKind::From {
                    from = state.stream.current().span;
                    state.stream.next();
                }

                let mut key = None;
                let mut value = Box::new(create(state));

                if state.stream.current().kind == TokenKind::DoubleArrow && !from.is_empty() {
                    state.stream.next();
                    key = Some(value.clone());
                    value = Box::new(create(state));
                }

                let end_span = state.stream.previous().span;
                let span = Span::new(start_span.start, end_span.end);

                if !from.is_empty() {
                    Expression::new(
                        state.id(),
                        ExpressionKind::YieldFrom(YieldFromExpression {
                            id: state.id(),
                            r#yield: start_span,
                            from,
                            span,
                            value,
                        }),
                        span,
                        CommentGroup::default(),
                    )
                } else {
                    Expression::new(
                        state.id(),
                        ExpressionKind::Yield(YieldExpression {
                            id: state.id(),
                            span,
                            r#yield: start_span,
                            key,
                            value: Some(value),
                        }),
                        span,
                        CommentGroup::default(),
                    )
                }
            }
        }

        (TokenKind::Clone, _) => {
            let start_span = state.stream.current().span;
            state.stream.next();

            let target = for_precedence(state, Precedence::CloneOrNew);

            let end_span = state.stream.previous().span;
            let span = Span::new(start_span.start, end_span.end);

            Expression::new(
                state.id(),
                ExpressionKind::Clone(CloneExpression {
                    id: state.id(),
                    span,
                    clone: start_span,
                    target: Box::new(target),
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::True, _) => {
            let span = state.stream.current().span;
            let value = state.stream.current().clone();
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::Bool(BoolExpression {
                    id: state.id(),
                    span,
                    value,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::False, _) => {
            let span = state.stream.current().span;
            let value = state.stream.current().clone();
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::Bool(BoolExpression {
                    id: state.id(),
                    span,
                    value,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Null, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::Null(span),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::LiteralInteger, _) => {
            let span = state.stream.current().span;
            let current = state.stream.current();

            if let TokenKind::LiteralInteger = &current.kind {
                state.stream.next();

                Expression::new(
                    state.id(),
                    ExpressionKind::Literal(Literal::new(
                        state.id(),
                        LiteralKind::Integer,
                        current.clone(),
                        span,
                    )),
                    span,
                    CommentGroup::default(),
                )
            } else {
                unreachable!("{}:{}", file!(), line!());
            }
        }

        (TokenKind::LiteralFloat, _) => {
            let span = state.stream.current().span;
            let current = state.stream.current();

            if let TokenKind::LiteralFloat = &current.kind {
                state.stream.next();

                Expression::new(
                    state.id(),
                    ExpressionKind::Literal(Literal::new(
                        state.id(),
                        LiteralKind::Float,
                        current.clone(),
                        span,
                    )),
                    span,
                    CommentGroup::default(),
                )
            } else {
                unreachable!("{}:{}", file!(), line!());
            }
        }

        (TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString, _) => {
            let span = state.stream.current().span;
            let current = state.stream.current();

            if let TokenKind::LiteralSingleQuotedString = &current.kind {
                state.stream.next();

                Expression::new(
                    state.id(),
                    ExpressionKind::Literal(Literal::new(
                        state.id(),
                        LiteralKind::String,
                        current.clone(),
                        span,
                    )),
                    span,
                    CommentGroup::default(),
                )
            } else if let TokenKind::LiteralDoubleQuotedString = &current.kind {
                state.stream.next();

                Expression::new(
                    state.id(),
                    ExpressionKind::Literal(Literal::new(
                        state.id(),
                        LiteralKind::String,
                        current.clone(),
                        span,
                    )),
                    span,
                    CommentGroup::default(),
                )
            } else {
                unreachable!("{}:{}", file!(), line!());
            }
        }

        (TokenKind::StringPart, _) => strings::interpolated(state),

        (TokenKind::StartHeredoc, _) => strings::heredoc(state),

        (TokenKind::StartNowdoc, _) => strings::nowdoc(state),

        (TokenKind::Backtick, _) => strings::shell_exec(state),

        (
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier,
            _,
        ) => {
            let name = names::full_name(
                state,
                match state.stream.peek().kind {
                    TokenKind::LeftParen => UseKind::Function,
                    TokenKind::DoubleColon => UseKind::Normal,
                    _ => UseKind::Const,
                },
            );

            let span = name.span;

            Expression::new(
                state.id(),
                ExpressionKind::Name(name),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Static, _) => {
            let span = state.stream.current().span;
            state.stream.next();
            let expression = Expression::new(
                state.id(),
                ExpressionKind::Static(StaticExpression {
                    id: state.id(),
                    span,
                }),
                span,
                CommentGroup::default(),
            );

            postfix(state, expression, &TokenKind::DoubleColon)
        }

        (TokenKind::Self_, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::Self_(SelfExpression {
                    id: state.id(),
                    span,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Parent, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::Parent(ParentExpression {
                    id: state.id(),
                    span,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::LeftParen, _) => {
            let start = state.stream.current().span;
            state.stream.next();

            let expr = create(state);

            let end = utils::skip_right_parenthesis(state);
            let span = Span::combine(start, end);

            Expression::new(
                state.id(),
                ExpressionKind::Parenthesized(ParenthesizedExpression {
                    id: state.id(),
                    span,
                    start,
                    expr: Box::new(expr),
                    end,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::Match, _) => control_flow::match_expression(state),

        (TokenKind::Array, _) => arrays::array_expression(state),

        (TokenKind::LeftBracket, _) => arrays::short_array_expression(state),

        (TokenKind::New, _) => {
            let new = state.stream.current().span;

            state.stream.next();

            if state.stream.current().kind == TokenKind::Class
                || state.stream.current().kind == TokenKind::Attribute
            {
                return classes::parse_anonymous(state, Some(new));
            };

            let target = match state.stream.current().kind {
                TokenKind::Self_ => {
                    let token = state.stream.current();

                    state.stream.next();

                    Expression::new(
                        state.id(),
                        ExpressionKind::Name(Name::special(
                            state.id(),
                            SpecialNameKind::Self_(token.span),
                            token.symbol.as_ref().unwrap().clone(),
                            token.span,
                        )),
                        token.span,
                        CommentGroup::default(),
                    )
                }
                TokenKind::Static => {
                    let token = state.stream.current();

                    state.stream.next();

                    Expression::new(
                        state.id(),
                        ExpressionKind::Name(Name::special(
                            state.id(),
                            SpecialNameKind::Static(token.span),
                            token.symbol.as_ref().unwrap().clone(),
                            token.span,
                        )),
                        token.span,
                        CommentGroup::default(),
                    )
                }
                TokenKind::Parent => {
                    let token = state.stream.current();

                    state.stream.next();

                    Expression::new(
                        state.id(),
                        ExpressionKind::Name(Name::special(
                            state.id(),
                            SpecialNameKind::Parent(token.span),
                            token.symbol.as_ref().unwrap().clone(),
                            token.span,
                        )),
                        token.span,
                        CommentGroup::default(),
                    )
                }
                TokenKind::FullyQualifiedIdentifier => {
                    let token = state.stream.current();

                    let span = token.span;
                    let symbol = token.symbol.as_ref().unwrap().clone();
                    let resolved = state.strip_leading_namespace_qualifier(&symbol);

                    state.stream.next();

                    Expression::new(
                        state.id(),
                        ExpressionKind::Name(Name::resolved(state.id(), resolved, symbol, span)),
                        span,
                        CommentGroup::default(),
                    )
                }
                TokenKind::Identifier
                | TokenKind::QualifiedIdentifier
                | TokenKind::Enum
                | TokenKind::From => {
                    let token = state.stream.current();

                    state.stream.next();

                    Expression::new(
                        state.id(),
                        ExpressionKind::Name(
                            state.maybe_resolve_identifier(token, UseKind::Normal),
                        ),
                        token.span,
                        CommentGroup::default(),
                    )
                }
                _ => clone_or_new_precedence(state),
            };

            let arguments = if state.stream.current().kind == TokenKind::LeftParen {
                Some(parameters::argument_list(state))
            } else {
                None
            };

            let span = Span::combine(new, state.stream.previous().span);

            Expression::new(
                state.id(),
                ExpressionKind::New(NewExpression {
                    id: state.id(),
                    span,
                    target: Box::new(target),
                    new,
                    arguments,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::DirConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Directory,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::FileConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::File,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::LineConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Line,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::FunctionConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Function,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::ClassConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Class,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::MethodConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Method,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::NamespaceConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Namespace,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::TraitConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::Trait,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (TokenKind::CompilerHaltOffsetConstant, _) => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::new(
                state.id(),
                ExpressionKind::MagicConstant(MagicConstantExpression {
                    id: state.id(),
                    span,
                    kind: MagicConstantKind::CompilerHaltOffset,
                }),
                span,
                CommentGroup::default(),
            )
        }

        (
            TokenKind::Include
            | TokenKind::IncludeOnce
            | TokenKind::Require
            | TokenKind::RequireOnce,
            _,
        ) => {
            let start_span = state.stream.current().span;
            let current = state.stream.current();
            let keyword_span = current.span;

            state.stream.next();

            let path = create(state);
            let span = Span::combine(start_span, path.span);
            let path = Box::new(path);

            let kind = match current.kind {
                TokenKind::Include => ExpressionKind::Include(IncludeExpression {
                    id: state.id(),
                    span,
                    include: keyword_span,
                    path,
                }),
                TokenKind::IncludeOnce => ExpressionKind::IncludeOnce(IncludeOnceExpression {
                    id: state.id(),
                    span,
                    include_once: keyword_span,
                    path,
                }),
                TokenKind::Require => ExpressionKind::Require(RequireExpression {
                    id: state.id(),
                    span,
                    require: keyword_span,
                    path,
                }),
                TokenKind::RequireOnce => ExpressionKind::RequireOnce(RequireOnceExpression {
                    id: state.id(),
                    span,
                    require_once: keyword_span,
                    path,
                }),
                _ => unreachable!(),
            };

            let end_span = state.stream.previous().span;

            Expression::new(
                state.id(),
                kind,
                Span::new(start_span.start, end_span.end),
                CommentGroup::default(),
            )
        }

        (
            TokenKind::StringCast
            | TokenKind::BinaryCast
            | TokenKind::ObjectCast
            | TokenKind::BoolCast
            | TokenKind::BooleanCast
            | TokenKind::IntCast
            | TokenKind::IntegerCast
            | TokenKind::FloatCast
            | TokenKind::DoubleCast
            | TokenKind::RealCast
            | TokenKind::UnsetCast
            | TokenKind::ArrayCast,
            _,
        ) => {
            let current = state.stream.current();

            let span = current.span;
            let kind = current.clone().into();

            state.stream.next();

            let rhs = for_precedence(state, Precedence::Prefix);
            let rhs_span = rhs.span;

            Expression::new(
                state.id(),
                ExpressionKind::Cast(CastExpression {
                    id: state.id(),
                    span,
                    kind,
                    value: Box::new(rhs),
                }),
                Span::new(span.start, rhs_span.end),
                CommentGroup::default(),
            )
        }

        (TokenKind::Decrement | TokenKind::Increment | TokenKind::Minus | TokenKind::Plus, _) => {
            let start_span = state.stream.current().span;
            let current = state.stream.current();

            let op_span = current.span;
            let op = current.kind;

            state.stream.next();

            let right = Box::new(for_precedence(state, Precedence::Prefix));
            let right_span = right.span;
            let span = Span::combine(start_span, right_span);

            let expr = match op {
                TokenKind::Minus => {
                    ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                        id: state.id(),
                        span,
                        kind: ArithmeticOperationKind::Negative {
                            id: state.id(),
                            minus: op_span,
                            right,
                        },
                    })
                }
                TokenKind::Plus => {
                    ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                        id: state.id(),
                        span,
                        kind: ArithmeticOperationKind::Positive {
                            id: state.id(),
                            plus: op_span,
                            right,
                        },
                    })
                }
                TokenKind::Decrement => {
                    ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                        id: state.id(),
                        span,
                        kind: ArithmeticOperationKind::PreDecrement {
                            id: state.id(),
                            decrement: op_span,
                            right,
                        },
                    })
                }
                TokenKind::Increment => {
                    ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                        id: state.id(),
                        span,
                        kind: ArithmeticOperationKind::PreIncrement {
                            id: state.id(),
                            increment: op_span,
                            right,
                        },
                    })
                }
                _ => unreachable!(),
            };

            Expression::new(
                state.id(),
                expr,
                Span::new(start_span.start, right_span.end),
                CommentGroup::default(),
            )
        }

        (TokenKind::Bang, _) => {
            let start_span = state.stream.current().span;
            let bang = state.stream.current().span;

            state.stream.next();

            let rhs = for_precedence(state, Precedence::Bang);
            let end_span = rhs.span;
            let span = Span::combine(start_span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::LogicalOperation(LogicalOperationExpression {
                    id: state.id(),
                    span,
                    kind: LogicalOperationKind::Not {
                        id: state.id(),
                        bang,
                        right: Box::new(rhs),
                    },
                }),
                Span::new(start_span.start, end_span.end),
                CommentGroup::default(),
            )
        }

        (TokenKind::At, _) => {
            let span = state.stream.current().span;

            state.stream.next();

            let rhs = for_precedence(state, Precedence::Prefix);
            let end_span = rhs.span;
            let span = Span::combine(span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::ErrorSuppress(ErrorSuppressExpression {
                    id: state.id(),
                    span,
                    at: span,
                    expr: Box::new(rhs),
                }),
                Span::new(span.start, end_span.end),
                CommentGroup::default(),
            )
        }

        (TokenKind::BitwiseNot, _) => {
            let span = state.stream.current().span;

            state.stream.next();

            let right = Box::new(for_precedence(state, Precedence::Prefix));
            let end_span = right.span;
            let span = Span::combine(span, end_span);

            Expression::new(
                state.id(),
                ExpressionKind::BitwiseOperation(BitwiseOperationExpression {
                    span,
                    kind: BitwiseOperationKind::Not {
                        id: state.id(),
                        not: span,
                        right,
                    },
                    id: state.id(),
                }),
                Span::new(span.start, end_span.end),
                CommentGroup::default(),
            )
        }

        (TokenKind::Dollar | TokenKind::DollarLeftBrace | TokenKind::Variable, _) => {
            let span = state.stream.current().span;

            Expression::new(
                state.id(),
                ExpressionKind::Variable(variables::dynamic_variable(state)),
                span,
                CommentGroup::default(),
            )
        }

        _ => unexpected_token(state, precedence),
    }
}

fn unexpected_token(state: &mut State, _: &Precedence) -> Expression {
    let current = state.stream.current();

    state.diagnostic(
        ParserDiagnostic::UnexpectedToken {
            token: current.clone(),
        },
        Severity::Error,
        current.span,
    );

    // This is a common case where we don't want to consume the right-brace as it might close a structure.
    if current.kind != TokenKind::RightBrace {
        state.stream.next();
    }

    Expression::missing(state.id(), current.span)
}

fn postfix(state: &mut State, lhs: Expression, op: &TokenKind) -> Expression {
    let start_span = state.stream.current().span;
    let kind = match op {
        TokenKind::DoubleQuestion => {
            let double_question = state.stream.current().span;
            state.stream.next();

            let rhs = null_coalesce_precedence(state);
            let span = Span::combine(lhs.span, rhs.span);

            ExpressionKind::Coalesce(CoalesceExpression {
                id: state.id(),
                span,
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
                let start = utils::skip(state, TokenKind::LeftParen);
                let ellipsis = utils::skip(state, TokenKind::Ellipsis);
                let end = utils::skip(state, TokenKind::RightParen);
                let span = Span::combine(start, end);

                let placeholder = ArgumentPlaceholder {
                    id: state.id(),
                    span,
                    comments: state.comments(),
                    left_parenthesis: start,
                    ellipsis,
                    right_parenthesis: end,
                };

                let span = Span::combine(lhs.span, span);

                ExpressionKind::FunctionClosureCreation(FunctionClosureCreationExpression {
                    id: state.id(),
                    span,
                    target: Box::new(lhs),
                    placeholder,
                })
            } else {
                let arguments = parameters::argument_list(state);
                let span = Span::combine(lhs.span, arguments.span);

                ExpressionKind::FunctionCall(FunctionCallExpression {
                    id: state.id(),
                    span,
                    target: Box::new(lhs),
                    arguments,
                })
            }
        }
        TokenKind::LeftBracket => {
            let left_bracket = utils::skip_left_bracket(state);
            let index = if state.stream.current().kind == TokenKind::RightBracket {
                None
            } else {
                Some(Box::new(create(state)))
            };
            let right_bracket = utils::skip_right_bracket(state);
            let span = Span::combine(lhs.span, right_bracket);

            ExpressionKind::ArrayIndex(ArrayIndexExpression {
                id: state.id(),
                span,
                array: Box::new(lhs),
                left_bracket,
                index,
                right_bracket,
            })
        }
        TokenKind::DoubleColon => {
            let double_colon = utils::skip_double_colon(state);
            let current = state.stream.current();

            let property = match current.kind {
                TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
                    ExpressionKind::Variable(variables::dynamic_variable(state))
                }
                _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
                    ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                        identifiers::identifier_maybe_reserved(state),
                    ))
                }
                TokenKind::LeftBrace => {
                    let start = current.span;

                    state.stream.next();

                    let expr = Box::new(create(state));
                    let end = utils::skip_right_brace(state);

                    let span = Span::new(start.start, end.end);

                    ExpressionKind::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
                        id: state.id(),
                        span,
                        expr,
                    }))
                }
                TokenKind::Class => {
                    state.stream.next();

                    let symbol = current.symbol.as_ref().unwrap().clone();

                    ExpressionKind::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier::new(
                        state.id(),
                        symbol,
                        current.span,
                    )))
                }
                _ => {
                    state.diagnostic(
                        ParserDiagnostic::ExpectedToken {
                            expected: vec![
                                TokenKind::LeftBrace,
                                TokenKind::Dollar,
                                TokenKind::Identifier,
                            ],
                            found: current.clone(),
                        },
                        Severity::Error,
                        current.span,
                    );

                    state.stream.next();

                    ExpressionKind::Missing(MissingExpression {
                        id: 0,
                        span: current.span,
                    })
                }
            };

            let lhs = Box::new(lhs);

            if state.stream.current().kind == TokenKind::LeftParen {
                if state.stream.lookahead(0).kind == TokenKind::Ellipsis
                    && state.stream.lookahead(1).kind == TokenKind::RightParen
                {
                    let start = utils::skip(state, TokenKind::LeftParen);
                    let ellipsis = utils::skip(state, TokenKind::Ellipsis);
                    let end = utils::skip(state, TokenKind::RightParen);
                    let span = Span::combine(start, end);

                    let placeholder = ArgumentPlaceholder {
                        id: state.id(),
                        span,
                        comments: state.comments(),
                        left_parenthesis: start,
                        ellipsis,
                        right_parenthesis: end,
                    };

                    match property {
                        ExpressionKind::Identifier(identifier) => {
                            ExpressionKind::StaticMethodClosureCreation(
                                StaticMethodClosureCreationExpression {
                                    id: state.id(),
                                    span: Span::combine(lhs.span, placeholder.span),
                                    target: lhs,
                                    double_colon,
                                    method: identifier,
                                    placeholder,
                                },
                            )
                        }
                        ExpressionKind::Variable(variable) => {
                            ExpressionKind::StaticVariableMethodClosureCreation(
                                StaticVariableMethodClosureCreationExpression {
                                    id: state.id(),
                                    span: Span::combine(lhs.span, placeholder.span),
                                    target: lhs,
                                    double_colon,
                                    method: variable,
                                    placeholder,
                                },
                            )
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let arguments = parameters::argument_list(state);

                    match property {
                        ExpressionKind::Identifier(identifier) => {
                            ExpressionKind::StaticMethodCall(StaticMethodCallExpression {
                                id: state.id(),
                                span: Span::combine(lhs.span, arguments.span),
                                target: lhs,
                                double_colon,
                                method: identifier,
                                arguments,
                            })
                        }
                        ExpressionKind::Variable(variable) => {
                            ExpressionKind::StaticVariableMethodCall(
                                StaticVariableMethodCallExpression {
                                    id: state.id(),
                                    span: Span::combine(lhs.span, arguments.span),
                                    target: lhs,
                                    double_colon,
                                    method: variable,
                                    arguments,
                                },
                            )
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                match property {
                    ExpressionKind::Identifier(identifier) => {
                        ExpressionKind::ConstantFetch(ConstantFetchExpression {
                            id: state.id(),
                            span: Span::combine(lhs.span, identifier.span()),
                            target: lhs,
                            double_colon,
                            constant: identifier,
                        })
                    }
                    ExpressionKind::Variable(variable) => {
                        ExpressionKind::StaticPropertyFetch(StaticPropertyFetchExpression {
                            id: state.id(),
                            span: Span::combine(lhs.span, variable.span()),
                            target: lhs,
                            double_colon,
                            property: variable,
                        })
                    }
                    _ => {
                        let span = Span::combine(lhs.span, double_colon);

                        ExpressionKind::ConstantFetch(ConstantFetchExpression {
                            id: state.id(),
                            span,
                            target: lhs,
                            double_colon,
                            constant: Identifier::missing(state.id(), Span::flat(double_colon.end)),
                        })
                    }
                }
            }
        }
        TokenKind::Arrow | TokenKind::QuestionArrow => {
            let span = state.stream.current().span;
            state.stream.next();

            let property = match state.stream.current().kind {
                TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
                    let start_span = state.stream.current().span;
                    let kind = ExpressionKind::Variable(variables::dynamic_variable(state));
                    let end_span = state.stream.previous().span;

                    Expression::new(
                        state.id(),
                        kind,
                        Span::new(start_span.start, end_span.end),
                        CommentGroup::default(),
                    )
                }
                _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
                    let start_span = state.stream.current().span;
                    let kind = ExpressionKind::Identifier(Identifier::SimpleIdentifier(
                        identifiers::identifier_maybe_reserved(state),
                    ));
                    let end_span = state.stream.previous().span;

                    Expression::new(
                        state.id(),
                        kind,
                        Span::new(start_span.start, end_span.end),
                        CommentGroup::default(),
                    )
                }
                TokenKind::LeftBrace => {
                    let start = state.stream.current().span;
                    state.stream.next();

                    let name = create(state);

                    let end = utils::skip_right_brace(state);
                    let span = Span::new(start.start, end.end);

                    Expression::new(
                        state.id(),
                        ExpressionKind::Identifier(Identifier::DynamicIdentifier(
                            DynamicIdentifier {
                                id: state.id(),
                                span,
                                expr: Box::new(name),
                            },
                        )),
                        Span::new(start.start, end.end),
                        CommentGroup::default(),
                    )
                }
                _ => {
                    let span = state.stream.current().span;

                    state.diagnostic(
                        ParserDiagnostic::ExpectedToken {
                            expected: vec![
                                TokenKind::LeftBrace,
                                TokenKind::Dollar,
                                TokenKind::Identifier,
                            ],
                            found: state.stream.current().clone(),
                        },
                        Severity::Error,
                        span,
                    );

                    Expression::missing(state.id(), span)
                }
            };

            if state.stream.current().kind == TokenKind::LeftParen {
                if op == &TokenKind::QuestionArrow {
                    let arguments = parameters::argument_list(state);

                    ExpressionKind::NullsafeMethodCall(NullsafeMethodCallExpression {
                        id: state.id(),
                        span: Span::combine(lhs.span, arguments.span),
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
                        let start = utils::skip(state, TokenKind::LeftParen);
                        let ellipsis = utils::skip(state, TokenKind::Ellipsis);
                        let end = utils::skip(state, TokenKind::RightParen);
                        let span = Span::combine(start, end);

                        let placeholder = ArgumentPlaceholder {
                            id: state.id(),
                            span,
                            comments: state.comments(),
                            left_parenthesis: start,
                            ellipsis,
                            right_parenthesis: end,
                        };

                        ExpressionKind::MethodClosureCreation(MethodClosureCreationExpression {
                            id: state.id(),
                            span: Span::combine(lhs.span, placeholder.span),
                            target: Box::new(lhs),
                            method: Box::new(property),
                            arrow: span,
                            placeholder,
                        })
                    } else {
                        let arguments = parameters::argument_list(state);

                        ExpressionKind::MethodCall(MethodCallExpression {
                            id: state.id(),
                            span: Span::combine(lhs.span, arguments.span),
                            target: Box::new(lhs),
                            method: Box::new(property),
                            arrow: span,
                            arguments,
                        })
                    }
                }
            } else if op == &TokenKind::QuestionArrow {
                ExpressionKind::NullsafePropertyFetch(NullsafePropertyFetchExpression {
                    id: state.id(),
                    span: Span::combine(lhs.span, property.span),
                    target: Box::new(lhs),
                    question_arrow: span,
                    property: Box::new(property),
                })
            } else {
                ExpressionKind::PropertyFetch(PropertyFetchExpression {
                    id: state.id(),
                    span: Span::combine(lhs.span, property.span),
                    target: Box::new(lhs),
                    arrow: span,
                    property: Box::new(property),
                })
            }
        }
        TokenKind::Increment => {
            let op = state.stream.current().span;
            state.stream.next();

            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                id: state.id(),
                span: Span::combine(lhs.span, op),
                kind: ArithmeticOperationKind::PostIncrement {
                    id: state.id(),
                    left: Box::new(lhs),
                    increment: op,
                },
            })
        }
        TokenKind::Decrement => {
            let op = state.stream.current().span;
            state.stream.next();

            ExpressionKind::ArithmeticOperation(ArithmeticOperationExpression {
                id: state.id(),
                span: Span::combine(lhs.span, op),
                kind: ArithmeticOperationKind::PostDecrement {
                    id: state.id(),
                    left: Box::new(lhs),
                    decrement: op,
                },
            })
        }
        _ => unreachable!(),
    };

    let end_span = state.stream.previous().span;

    Expression::new(
        state.id(),
        kind,
        Span::new(start_span.start, end_span.end),
        CommentGroup::default(),
    )
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
