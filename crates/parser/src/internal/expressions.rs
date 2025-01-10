use crate::internal::diagnostics::ParserDiagnostic;
use crate::internal::precedences::Associativity;
use crate::internal::precedences::Precedence;
use crate::Parser;
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

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self) -> Expression {
        self.for_precedence(Precedence::Lowest)
    }

    fn null_coalesce_precedence(&mut self) -> Expression {
        self.for_precedence(Precedence::NullCoalesce)
    }

    fn clone_or_new_precedence(&mut self) -> Expression {
        self.for_precedence(Precedence::CloneOrNew)
    }

    fn for_precedence(&mut self, precedence: Precedence) -> Expression {
        let mut left = self.left(&precedence);

        loop {
            let span = self.current_span();
            let kind = self.current_kind();

            if matches!(kind, TokenKind::SemiColon | TokenKind::Eof) {
                break;
            }

            if self.is_postfix(kind) {
                let lpred = Precedence::postfix(kind);

                if lpred < precedence {
                    break;
                }

                left = self.postfix(left, kind);
                continue;
            }

            if self.is_infix(kind) {
                let rpred = Precedence::infix(kind);

                if rpred < precedence {
                    break;
                }

                if rpred == precedence && matches!(rpred.associativity(), Some(Associativity::Left))
                {
                    break;
                }

                if rpred == precedence && matches!(rpred.associativity(), Some(Associativity::Non))
                {
                    self.diagnostic(
                        ParserDiagnostic::UnexpectedToken {
                            token: self.current().to_owned(),
                        },
                        Severity::Error,
                        self.current_span(),
                    );
                }

                self.next();

                let op = self.current().to_owned();
                let start_span = self.current_span();

                let kind = match kind {
                    TokenKind::Question => {
                        // this happens due to a comment, or whitespaces between the  and the :
                        // we consider `foo()  : bar()` a ternary expression, with `then` being a noop
                        // however, this must behave like a short ternary at runtime.
                        if op.kind == TokenKind::Colon {
                            self.next();

                            let r#else = self.parse_expression();

                            ExpressionKind::Ternary(Box::new(TernaryExpression {
                                id: self.id(),
                                span: Span::combine(left.span, r#else.span),
                                condition: Box::new(left),
                                question: span,
                                then: Box::new(Expression::noop(self.id(), start_span)),
                                colon: op.span,
                                r#else: Box::new(r#else),
                            }))
                        } else {
                            let then = self.parse_expression();
                            let colon = self.skip_colon();
                            let r#else = self.parse_expression();

                            ExpressionKind::Ternary(Box::new(TernaryExpression {
                                id: self.id(),
                                span: Span::combine(left.span, r#else.span),
                                condition: Box::new(left),
                                question: span,
                                then: Box::new(then),
                                colon,
                                r#else: Box::new(r#else),
                            }))
                        }
                    }
                    TokenKind::QuestionColon => {
                        let r#else = self.parse_expression();
                        ExpressionKind::ShortTernary(Box::new(ShortTernaryExpression {
                            id: self.id(),
                            span: Span::combine(left.span, r#else.span),
                            condition: Box::new(left),
                            question_colon: span,
                            r#else: Box::new(r#else),
                        }))
                    }
                    TokenKind::Equals if op.kind == TokenKind::Ampersand => {
                        self.next();

                        let right = Box::new(self.for_precedence(rpred));
                        let right_span = right.span;
                        let expr_span = Span::combine(left.span, right_span);
                        let reference_span = Span::combine(op.span, right_span);

                        ExpressionKind::AssignmentOperation(Box::new(
                            AssignmentOperationExpression {
                                id: self.id(),
                                span: expr_span,
                                left: Box::new(left),
                                kind: AssignmentOperationKind::Assign(span),
                                right: Box::new(Expression::new(
                                    self.id(),
                                    ExpressionKind::Reference(Box::new(ReferenceExpression {
                                        id: self.id(),
                                        span: reference_span,
                                        ampersand: op.span,
                                        right,
                                    })),
                                    Span::new(start_span.start, right_span.end),
                                    CommentGroup::default(),
                                )),
                            },
                        ))
                    }
                    TokenKind::Instanceof if op.kind == TokenKind::Self_ => {
                        let self_span = op.span;
                        self.next();
                        let right = Expression::new(
                            self.id(),
                            ExpressionKind::Self_(Box::new(SelfExpression {
                                id: self.id(),
                                span: self_span,
                            })),
                            self_span,
                            CommentGroup::default(),
                        );
                        let span = Span::combine(left.span, right.span);

                        ExpressionKind::Instanceof(Box::new(InstanceofExpression {
                            id: self.id(),
                            span,
                            left: Box::new(left),
                            instanceof: span,
                            right: Box::new(right),
                        }))
                    }
                    TokenKind::Instanceof if op.kind == TokenKind::Parent => {
                        self.next();
                        let right = Expression::new(
                            self.id(),
                            ExpressionKind::Parent(Box::new(ParentExpression {
                                id: self.id(),
                                span: op.span,
                            })),
                            op.span,
                            CommentGroup::default(),
                        );
                        let span = Span::combine(left.span, right.span);

                        ExpressionKind::Instanceof(Box::new(InstanceofExpression {
                            id: self.id(),
                            span,
                            left: Box::new(left),
                            instanceof: span,
                            right: Box::new(right),
                        }))
                    }
                    TokenKind::Instanceof if op.kind == TokenKind::Static => {
                        let instanceof = span;
                        self.next();
                        let right = Expression::new(
                            self.id(),
                            ExpressionKind::Static(Box::new(StaticExpression {
                                id: self.id(),
                                span: op.span,
                            })),
                            op.span,
                            CommentGroup::default(),
                        );

                        ExpressionKind::Instanceof(Box::new(InstanceofExpression {
                            id: self.id(),
                            span: Span::combine(left.span, right.span),
                            left: Box::new(left),
                            instanceof,
                            right: Box::new(right),
                        }))
                    }
                    TokenKind::Instanceof if op.kind == TokenKind::Enum => {
                        let enum_span = op.span;
                        self.next();

                        let right = Expression::new(
                            self.id(),
                            ExpressionKind::Identifier(Box::new(Identifier::SimpleIdentifier(
                                SimpleIdentifier::new(self.id(), op.symbol, enum_span),
                            ))),
                            enum_span,
                            CommentGroup::default(),
                        );

                        ExpressionKind::Instanceof(Box::new(InstanceofExpression {
                            id: self.id(),
                            span: Span::combine(left.span, right.span),
                            left: Box::new(left),
                            instanceof: span,
                            right: Box::new(right),
                        }))
                    }
                    TokenKind::Instanceof if op.kind == TokenKind::From => {
                        let from_span = op.span;
                        self.next();
                        let right = Expression::new(
                            self.id(),
                            ExpressionKind::Identifier(Box::new(Identifier::SimpleIdentifier(
                                SimpleIdentifier::new(self.id(), op.symbol, op.span),
                            ))),
                            Span::new(start_span.start, from_span.end),
                            CommentGroup::default(),
                        );

                        ExpressionKind::Instanceof(Box::new(InstanceofExpression {
                            id: self.id(),
                            span: Span::combine(left.span, right.span),
                            left: Box::new(left),
                            instanceof: span,
                            right: Box::new(right),
                        }))
                    }
                    _ => {
                        let op_span = span;
                        let left = Box::new(left);
                        let right = Box::new(self.for_precedence(rpred));
                        let span = Span::combine(left.span, right.span);

                        match kind {
                            TokenKind::Plus => ExpressionKind::ArithmeticOperation(Box::new(
                                ArithmeticOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ArithmeticOperationKind::Addition {
                                        id: self.id(),
                                        left,
                                        plus: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Minus => ExpressionKind::ArithmeticOperation(Box::new(
                                ArithmeticOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ArithmeticOperationKind::Subtraction {
                                        id: self.id(),
                                        left,
                                        minus: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Asterisk => ExpressionKind::ArithmeticOperation(Box::new(
                                ArithmeticOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ArithmeticOperationKind::Multiplication {
                                        id: self.id(),
                                        left,
                                        asterisk: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Slash => ExpressionKind::ArithmeticOperation(Box::new(
                                ArithmeticOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ArithmeticOperationKind::Division {
                                        id: self.id(),
                                        left,
                                        slash: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Percent => ExpressionKind::ArithmeticOperation(Box::new(
                                ArithmeticOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ArithmeticOperationKind::Modulo {
                                        id: self.id(),
                                        left,
                                        percent: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Pow => ExpressionKind::ArithmeticOperation(Box::new(
                                ArithmeticOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ArithmeticOperationKind::Exponentiation {
                                        id: self.id(),
                                        left,
                                        pow: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Equals => ExpressionKind::AssignmentOperation(Box::new(
                                AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Assign(op_span),
                                    right,
                                },
                            )),
                            TokenKind::PlusEquals => ExpressionKind::AssignmentOperation(Box::new(
                                AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Addition(op_span),
                                    right,
                                },
                            )),
                            TokenKind::MinusEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Subtraction(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::AsteriskEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Multiplication(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::SlashEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Division(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::PercentEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Modulo(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::PowEquals => ExpressionKind::AssignmentOperation(Box::new(
                                AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Exponentiation(op_span),
                                    right,
                                },
                            )),
                            TokenKind::AmpersandEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::BitwiseAnd(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::PipeEquals => ExpressionKind::AssignmentOperation(Box::new(
                                AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::BitwiseOr(op_span),
                                    right,
                                },
                            )),
                            TokenKind::CaretEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::BitwiseXor(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::LeftShiftEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::LeftShift(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::RightShiftEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::RightShift(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::DoubleQuestionEquals => ExpressionKind::AssignmentOperation(
                                Box::new(AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Coalesce(op_span),
                                    right,
                                }),
                            ),
                            TokenKind::DotEquals => ExpressionKind::AssignmentOperation(Box::new(
                                AssignmentOperationExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    kind: AssignmentOperationKind::Concat(op_span),
                                    right,
                                },
                            )),
                            TokenKind::Ampersand => ExpressionKind::BitwiseOperation(Box::new(
                                BitwiseOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: BitwiseOperationKind::And {
                                        id: self.id(),
                                        left,
                                        and: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Pipe => ExpressionKind::BitwiseOperation(Box::new(
                                BitwiseOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: BitwiseOperationKind::Or {
                                        id: self.id(),
                                        left,
                                        or: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Caret => ExpressionKind::BitwiseOperation(Box::new(
                                BitwiseOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: BitwiseOperationKind::Xor {
                                        id: self.id(),
                                        left,
                                        xor: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::LeftShift => ExpressionKind::BitwiseOperation(Box::new(
                                BitwiseOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: BitwiseOperationKind::LeftShift {
                                        id: self.id(),
                                        left,
                                        left_shift: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::RightShift => ExpressionKind::BitwiseOperation(Box::new(
                                BitwiseOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: BitwiseOperationKind::RightShift {
                                        id: self.id(),
                                        left,
                                        right_shift: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::DoubleEquals => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::Equal {
                                        id: self.id(),
                                        left,
                                        double_equals: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::TripleEquals => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::Identical {
                                        id: self.id(),
                                        left,
                                        triple_equals: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::BangEquals => ExpressionKind::ComparisonOperation(Box::new(
                                ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::NotEqual {
                                        id: self.id(),
                                        left,
                                        bang_equals: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::AngledLeftRight => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::AngledNotEqual {
                                        id: self.id(),
                                        left,
                                        angled_left_right: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::BangDoubleEquals => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::NotIdentical {
                                        id: self.id(),
                                        left,
                                        bang_double_equals: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::LessThan => ExpressionKind::ComparisonOperation(Box::new(
                                ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::LessThan {
                                        id: self.id(),
                                        left,
                                        less_than: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::GreaterThan => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::GreaterThan {
                                        id: self.id(),
                                        left,
                                        greater_than: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::LessThanEquals => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::LessThanOrEqual {
                                        id: self.id(),
                                        left,
                                        less_than_equals: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::GreaterThanEquals => ExpressionKind::ComparisonOperation(
                                Box::new(ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::GreaterThanOrEqual {
                                        id: self.id(),
                                        left,
                                        greater_than_equals: op_span,
                                        right,
                                    },
                                }),
                            ),
                            TokenKind::Spaceship => ExpressionKind::ComparisonOperation(Box::new(
                                ComparisonOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: ComparisonOperationKind::Spaceship {
                                        id: self.id(),
                                        left,
                                        spaceship: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::BooleanAnd => ExpressionKind::LogicalOperation(Box::new(
                                LogicalOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: LogicalOperationKind::And {
                                        id: self.id(),
                                        left,
                                        double_ampersand: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::BooleanOr => ExpressionKind::LogicalOperation(Box::new(
                                LogicalOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: LogicalOperationKind::Or {
                                        id: self.id(),
                                        left,
                                        double_pipe: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::LogicalAnd => ExpressionKind::LogicalOperation(Box::new(
                                LogicalOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: LogicalOperationKind::LogicalAnd {
                                        id: self.id(),
                                        left,
                                        and: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::LogicalOr => ExpressionKind::LogicalOperation(Box::new(
                                LogicalOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: LogicalOperationKind::LogicalOr {
                                        id: self.id(),
                                        left,
                                        or: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::LogicalXor => ExpressionKind::LogicalOperation(Box::new(
                                LogicalOperationExpression {
                                    id: self.id(),
                                    span,
                                    kind: LogicalOperationKind::LogicalXor {
                                        id: self.id(),
                                        left,
                                        xor: op_span,
                                        right,
                                    },
                                },
                            )),
                            TokenKind::Dot => ExpressionKind::Concat(Box::new(ConcatExpression {
                                id: self.id(),
                                span,
                                left,
                                dot: op_span,
                                right,
                            })),
                            TokenKind::Instanceof => {
                                ExpressionKind::Instanceof(Box::new(InstanceofExpression {
                                    id: self.id(),
                                    span,
                                    left,
                                    instanceof: op_span,
                                    right,
                                }))
                            }
                            _ => unreachable!(),
                        }
                    }
                };

                let span = Span::combine(start_span, kind.span());

                left = Expression::new(self.id(), kind, span, CommentGroup::default());

                self.maybe_shift_assignment_operands(&mut left);

                continue;
            }

            break;
        }

        left
    }

    fn should_shift_assignment_operands(&self, expr: &Expression) -> bool {
        match &expr.kind {
            ExpressionKind::AssignmentOperation(inner) => matches!(
                &inner.left.kind,
                ExpressionKind::ComparisonOperation(_)
                    | ExpressionKind::BitwiseOperation(_)
                    | ExpressionKind::ArithmeticOperation(_)
                    | ExpressionKind::LogicalOperation(_)
            ),
            _ => false,
        }
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
    fn maybe_shift_assignment_operands(&self, expr: &mut Expression) {
        if !self.should_shift_assignment_operands(expr) {
            return;
        }

        // At this point, we know that the left-hand side of the expression is an assignment.
        let (id, assignment_left, kind, assignment_right) = match &expr.kind {
            ExpressionKind::AssignmentOperation(inner) => (
                &inner.id,
                inner.left.as_ref(),
                &inner.kind,
                inner.right.as_ref(),
            ),
            _ => unreachable!(),
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

        // Then we need to get the right-hand side of the comparison, since
        // this is the real assignment target.
        let real_assignment_target = match &assignment_left.kind {
            ExpressionKind::ComparisonOperation(inner) => Some(inner.kind.right()),
            ExpressionKind::BitwiseOperation(inner) => Some(inner.kind.right()),
            ExpressionKind::ArithmeticOperation(inner) => inner.kind.right(),
            ExpressionKind::LogicalOperation(inner) => Some(inner.kind.right()),
            _ => todo!(),
        };

        if real_assignment_target.is_none() {
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
            ExpressionKind::AssignmentOperation(Box::new(AssignmentOperationExpression {
                id: *id,
                span: Span::default(),
                left: Box::new(real_assignment_target.cloned().unwrap()),
                kind: *kind,
                right: Box::new(assignment_right.clone()),
            })),
            Span::default(),
            CommentGroup::default(),
        );

        // Then we need to create the new binary operation, which will replace
        // the existing assignment operation.
        let mut new_expression = assignment_left.clone();

        match &mut new_expression.kind {
            ExpressionKind::ComparisonOperation(inner) => inner.kind.set_right(Box::new(new_right)),
            ExpressionKind::BitwiseOperation(inner) => inner.kind.set_right(Box::new(new_right)),
            ExpressionKind::ArithmeticOperation(inner) => inner.kind.set_right(Box::new(new_right)),
            ExpressionKind::LogicalOperation(inner) => inner.kind.set_right(Box::new(new_right)),
            _ => unreachable!(),
        };

        *expr = new_expression;
    }

    pub fn attributes(&mut self) -> Expression {
        self.gather_attributes();

        match self.current_kind() {
            TokenKind::Static if self.peek_kind() == TokenKind::Function => {
                self.parse_anonymous_function()
            }
            TokenKind::Static if self.peek_kind() == TokenKind::Fn => self.parse_arrow_function(),
            TokenKind::Function => self.parse_anonymous_function(),
            TokenKind::Fn => self.parse_arrow_function(),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::InvalidTargetForAttributes,
                    Severity::Error,
                    self.current_span(),
                );

                Expression::missing(self.id(), self.current_span())
            }
        }
    }

    fn left(&mut self, precedence: &Precedence) -> Expression {
        if self.is_eof() {
            self.diagnostic(
                ParserDiagnostic::UnexpectedEndOfFile,
                Severity::Error,
                self.current().span,
            );

            return Expression::missing(self.id(), self.current().span);
        }

        match (self.current_kind(), self.peek_kind()) {
            (TokenKind::Attribute, _) => self.attributes(),

            (TokenKind::Static, TokenKind::Fn) => self.parse_arrow_function(),

            (TokenKind::Static, TokenKind::Function) => self.parse_anonymous_function(),

            (TokenKind::Fn, _) => self.parse_arrow_function(),

            (TokenKind::Function, _) => self.parse_anonymous_function(),

            (TokenKind::Eval, TokenKind::LeftParen) => {
                let eval = self.next();

                let argument = Box::new(self.parse_single_argument(true, true).unwrap());

                let span = Span::combine(eval, argument.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Eval(Box::new(EvalExpression {
                        id: self.id(),
                        span,
                        eval,
                        argument,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Empty, TokenKind::LeftParen) => {
                let empty = self.next();

                let argument = Box::new(self.parse_single_argument(true, true).unwrap());
                let span = Span::combine(empty, argument.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Empty(Box::new(EmptyExpression {
                        id: self.id(),
                        span,
                        empty,
                        argument,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Die, _) => {
                let die = self.next();

                let argument = self.parse_single_argument(false, true).map(Box::new);

                let span = Span::combine(die, argument.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Die(Box::new(DieExpression {
                        id: self.id(),
                        span,
                        die,
                        argument,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Exit, _) => {
                let exit = self.next();

                let argument = self.parse_single_argument(false, true).map(Box::new);

                let span = Span::combine(exit, argument.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Exit(Box::new(ExitExpression {
                        id: self.id(),
                        span,
                        exit,
                        argument,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Isset, TokenKind::LeftParen) => {
                let isset = self.next();
                let arguments = self.parse_argument_list();
                let span = Span::combine(isset, arguments.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Isset(Box::new(IssetExpression {
                        id: self.id(),
                        span,
                        isset,
                        arguments,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Unset, TokenKind::LeftParen) => {
                let unset = self.next();
                let arguments = self.parse_argument_list();
                let span = Span::combine(unset, arguments.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Unset(Box::new(UnsetExpression {
                        id: self.id(),
                        span,
                        unset,
                        arguments,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Print, _) => {
                let print = self.next();

                let mut value = None;
                let mut argument = None;

                if let Some(arg) = self.parse_single_argument(false, true) {
                    argument = Some(Box::new(arg));
                } else {
                    value = Some(Box::new(self.parse_expression()));
                }

                let span = if let Some(argument) = &argument {
                    Span::combine(print, argument.span())
                } else {
                    Span::combine(print, value.as_ref().unwrap().span())
                };

                Expression::new(
                    self.id(),
                    ExpressionKind::Print(Box::new(PrintExpression {
                        id: self.id(),
                        span,
                        print,
                        value,
                        argument,
                    })),
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
                let name = self.parse_name_maybe_soft_reserved(UseKind::Function);
                let span = name.span;

                let lhs = Expression::new(
                    self.id(),
                    ExpressionKind::Name(Box::new(name)),
                    span,
                    CommentGroup::default(),
                );

                self.postfix(lhs, TokenKind::LeftParen)
            }

            (TokenKind::Enum | TokenKind::From, TokenKind::DoubleColon) => {
                let name = self.parse_full_name_including_self();
                let span = name.span;

                let lhs = Expression::new(
                    self.id(),
                    ExpressionKind::Name(Box::new(name)),
                    span,
                    CommentGroup::default(),
                );

                self.postfix(lhs, TokenKind::DoubleColon)
            }

            (TokenKind::List, _) => self.parse_list_expression(),

            (TokenKind::New, TokenKind::Class | TokenKind::Attribute) => {
                self.parse_anonymous_class(None)
            }

            (TokenKind::Throw, _) => {
                let start_span = self.current().span;
                self.next();
                let exception = self.for_precedence(Precedence::Lowest);
                let exception_span = exception.span;
                let span = Span::combine(start_span, exception_span);

                Expression::new(
                    self.id(),
                    ExpressionKind::Throw(Box::new(ThrowExpression {
                        id: self.id(),
                        span,
                        value: Box::new(exception),
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Yield, _) => {
                let start_span = self.next();

                if self.current_kind() == TokenKind::SemiColon
                    || self.current_kind() == TokenKind::RightParen
                {
                    Expression::new(
                        self.id(),
                        ExpressionKind::Yield(Box::new(YieldExpression {
                            id: self.id(),
                            r#yield: start_span,
                            span: start_span,
                            key: None,
                            value: None,
                        })),
                        start_span,
                        CommentGroup::default(),
                    )
                } else {
                    let mut from = Span::default();

                    if self.current_kind() == TokenKind::From {
                        from = self.next();
                    }

                    let mut key = None;
                    let mut value = Box::new(self.parse_expression());

                    if self.current_kind() == TokenKind::DoubleArrow && from.is_empty() {
                        self.next();
                        key = Some(value.clone());
                        value = Box::new(self.parse_expression());
                    }

                    let span = Span::combine(start_span, value.span());

                    if !from.is_empty() {
                        Expression::new(
                            self.id(),
                            ExpressionKind::YieldFrom(Box::new(YieldFromExpression {
                                id: self.id(),
                                r#yield: start_span,
                                from,
                                span,
                                value,
                            })),
                            span,
                            CommentGroup::default(),
                        )
                    } else {
                        Expression::new(
                            self.id(),
                            ExpressionKind::Yield(Box::new(YieldExpression {
                                id: self.id(),
                                span,
                                r#yield: start_span,
                                key,
                                value: Some(value),
                            })),
                            span,
                            CommentGroup::default(),
                        )
                    }
                }
            }

            (TokenKind::Clone, _) => {
                let start_span = self.next();

                let target = self.for_precedence(Precedence::CloneOrNew);

                let span = Span::combine(start_span, target.span());

                Expression::new(
                    self.id(),
                    ExpressionKind::Clone(Box::new(CloneExpression {
                        id: self.id(),
                        span,
                        clone: start_span,
                        target: Box::new(target),
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::True, _) => {
                let value = self.current().to_owned();
                let span = self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::Bool(Box::new(BoolExpression {
                        id: self.id(),
                        span,
                        value,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::False, _) => {
                let value = self.current().to_owned();
                let span = self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::Bool(Box::new(BoolExpression {
                        id: self.id(),
                        span,
                        value,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Null, _) => {
                let span = self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::Null(span),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::LiteralInteger, _) => {
                if self.current_kind() == TokenKind::LiteralInteger {
                    self.next_but_first(|parser| {
                        Expression::new(
                            parser.id(),
                            ExpressionKind::Literal(Box::new(Literal::new(
                                parser.id(),
                                LiteralKind::Integer,
                                parser.current().to_owned(),
                                parser.current_span(),
                            ))),
                            parser.current_span(),
                            CommentGroup::default(),
                        )
                    })
                } else {
                    unreachable!("{}:{}", file!(), line!());
                }
            }

            (TokenKind::LiteralFloat, _) => {
                if self.current_kind() == TokenKind::LiteralFloat {
                    self.next_but_first(|parser| {
                        Expression::new(
                            parser.id(),
                            ExpressionKind::Literal(Box::new(Literal::new(
                                parser.id(),
                                LiteralKind::Float,
                                parser.current().to_owned(),
                                parser.current_span(),
                            ))),
                            parser.current_span(),
                            CommentGroup::default(),
                        )
                    })
                } else {
                    unreachable!("{}:{}", file!(), line!());
                }
            }

            (TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString, _) => {
                if matches!(
                    self.current_kind(),
                    TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString
                ) {
                    self.next_but_first(|parser| {
                        Expression::new(
                            parser.id(),
                            ExpressionKind::Literal(Box::new(Literal::new(
                                parser.id(),
                                LiteralKind::String,
                                parser.current().to_owned(),
                                parser.current_span(),
                            ))),
                            parser.current_span(),
                            CommentGroup::default(),
                        )
                    })
                } else {
                    unreachable!("{}:{}", file!(), line!());
                }
            }

            (TokenKind::StringPart, _) => self.parse_interpolated_string(),

            (TokenKind::StartHeredoc, _) => self.parse_heredoc(),

            (TokenKind::StartNowdoc, _) => self.parse_nowdoc(),

            (TokenKind::Backtick, _) => self.parse_shell_exec_string(),

            (
                TokenKind::Identifier
                | TokenKind::QualifiedIdentifier
                | TokenKind::FullyQualifiedIdentifier,
                _,
            ) => {
                let kind = match self.peek_kind() {
                    TokenKind::LeftParen => UseKind::Function,
                    TokenKind::DoubleColon => UseKind::Normal,
                    _ => UseKind::Const,
                };

                let name = self.parse_full_name(kind);

                let span = name.span;

                Expression::new(
                    self.id(),
                    ExpressionKind::Name(Box::new(name)),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Static, _) => {
                let span = self.next();

                let expression = Expression::new(
                    self.id(),
                    ExpressionKind::Static(Box::new(StaticExpression {
                        id: self.id(),
                        span,
                    })),
                    span,
                    CommentGroup::default(),
                );

                self.postfix(expression, TokenKind::DoubleColon)
            }

            (TokenKind::Self_, _) => {
                let span = self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::Self_(Box::new(SelfExpression {
                        id: self.id(),
                        span,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Parent, _) => {
                let span = self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::Parent(Box::new(ParentExpression {
                        id: self.id(),
                        span,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::LeftParen, _) => {
                let start = self.next();

                let expr = self.parse_expression();

                let end = self.skip_right_parenthesis();
                let span = Span::combine(start, end);

                Expression::new(
                    self.id(),
                    ExpressionKind::Parenthesized(Box::new(ParenthesizedExpression {
                        id: self.id(),
                        span,
                        start,
                        expr: Box::new(expr),
                        end,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::Match, _) => self.parse_match_expression(),

            (TokenKind::Array, _) => self.parse_array_expression(),

            (TokenKind::LeftBracket, _) => self.parse_short_array_expression(),

            (TokenKind::New, _) => {
                let new = self.next();

                if self.current_kind() == TokenKind::Class
                    || self.current_kind() == TokenKind::Attribute
                {
                    return self.parse_anonymous_class(Some(new));
                };

                let target = match self.current_kind() {
                    TokenKind::Self_ => {
                        let token = self.current().to_owned();

                        self.next();

                        let span = token.span;

                        Expression::new(
                            self.id(),
                            ExpressionKind::Name(Box::new(Name::special(
                                self.id(),
                                SpecialNameKind::Self_,
                                token.symbol,
                                span,
                            ))),
                            span,
                            CommentGroup::default(),
                        )
                    }
                    TokenKind::Static => {
                        let token = self.current().to_owned();

                        self.next();

                        let span = token.span;

                        Expression::new(
                            self.id(),
                            ExpressionKind::Name(Box::new(Name::special(
                                self.id(),
                                SpecialNameKind::Static,
                                token.symbol,
                                span,
                            ))),
                            span,
                            CommentGroup::default(),
                        )
                    }
                    TokenKind::Parent => {
                        let token = self.current().to_owned();

                        self.next();

                        let span = token.span;

                        Expression::new(
                            self.id(),
                            ExpressionKind::Name(Box::new(Name::special(
                                self.id(),
                                SpecialNameKind::Parent,
                                token.symbol,
                                span,
                            ))),
                            span,
                            CommentGroup::default(),
                        )
                    }
                    TokenKind::FullyQualifiedIdentifier => {
                        let symbol = self.current_symbol_as_bytestring();
                        let span = self.current_span();

                        self.next();

                        let resolved = self.strip_leading_namespace_qualifier(&symbol);

                        Expression::new(
                            self.id(),
                            ExpressionKind::Name(Box::new(Name::resolved(
                                self.id(),
                                resolved,
                                symbol,
                                span,
                            ))),
                            span,
                            CommentGroup::default(),
                        )
                    }
                    TokenKind::Identifier
                    | TokenKind::QualifiedIdentifier
                    | TokenKind::Enum
                    | TokenKind::From => self.next_but_first(|parser| {
                        let id = parser.id();

                        Expression::new(
                            parser.id(),
                            ExpressionKind::Name(Box::new(parser.maybe_resolve_identifier(
                                id,
                                &parser.current(),
                                UseKind::Normal,
                            ))),
                            parser.current_span(),
                            CommentGroup::default(),
                        )
                    }),
                    TokenKind::Variable => {
                        let variable = self.parse_simple_variable();
                        let span = variable.span;

                        Expression::new(
                            self.id(),
                            ExpressionKind::Variable(Box::new(Variable::SimpleVariable(variable))),
                            span,
                            CommentGroup::default(),
                        )
                    }
                    _ => self.clone_or_new_precedence(),
                };

                let arguments = if self.current_kind() == TokenKind::LeftParen {
                    Some(self.parse_argument_list())
                } else {
                    None
                };

                let span = if arguments.is_some() {
                    Span::combine(new, arguments.span())
                } else {
                    Span::combine(new, target.span())
                };

                Expression::new(
                    self.id(),
                    ExpressionKind::New(Box::new(NewExpression {
                        id: self.id(),
                        span,
                        target: Box::new(target),
                        new,
                        arguments,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::DirConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Directory,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::FileConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::File,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::LineConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Line,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::FunctionConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Function,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::ClassConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Class,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::MethodConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Method,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::NamespaceConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Namespace,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::TraitConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::Trait,
                    })),
                    span,
                    CommentGroup::default(),
                )
            }

            (TokenKind::CompilerHaltOffsetConstant, _) => {
                let span = self.current().span;
                self.next();

                Expression::new(
                    self.id(),
                    ExpressionKind::MagicConstant(Box::new(MagicConstantExpression {
                        id: self.id(),
                        span,
                        kind: MagicConstantKind::CompilerHaltOffset,
                    })),
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
                let kind = self.current_kind();
                let keyword_span = self.next();

                let path = self.parse_expression();
                let span = Span::combine(keyword_span, path.span);
                let path = Box::new(path);

                let kind = match kind {
                    TokenKind::Include => ExpressionKind::Include(Box::new(IncludeExpression {
                        id: self.id(),
                        span,
                        include: keyword_span,
                        path,
                    })),
                    TokenKind::IncludeOnce => {
                        ExpressionKind::IncludeOnce(Box::new(IncludeOnceExpression {
                            id: self.id(),
                            span,
                            include_once: keyword_span,
                            path,
                        }))
                    }
                    TokenKind::Require => ExpressionKind::Require(Box::new(RequireExpression {
                        id: self.id(),
                        span,
                        require: keyword_span,
                        path,
                    })),
                    TokenKind::RequireOnce => {
                        ExpressionKind::RequireOnce(Box::new(RequireOnceExpression {
                            id: self.id(),
                            span,
                            require_once: keyword_span,
                            path,
                        }))
                    }
                    _ => unreachable!(),
                };

                let span = kind.span();

                Expression::new(self.id(), kind, span, CommentGroup::default())
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
                let span = self.current_span();
                let kind = self.current().into();

                self.next();

                let rhs = self.for_precedence(Precedence::Prefix);
                let rhs_span = rhs.span;

                Expression::new(
                    self.id(),
                    ExpressionKind::Cast(Box::new(CastExpression {
                        id: self.id(),
                        span,
                        kind,
                        value: Box::new(rhs),
                    })),
                    Span::new(span.start, rhs_span.end),
                    CommentGroup::default(),
                )
            }

            (
                TokenKind::Decrement | TokenKind::Increment | TokenKind::Minus | TokenKind::Plus,
                _,
            ) => {
                let start_span = self.current().span;
                let current = self.current();

                let op_span = current.span;
                let op = current.kind;

                self.next();

                let right = Box::new(self.for_precedence(Precedence::Prefix));
                let right_span = right.span;
                let span = Span::combine(start_span, right_span);

                let expr = match op {
                    TokenKind::Minus => ExpressionKind::ArithmeticOperation(Box::new(
                        ArithmeticOperationExpression {
                            id: self.id(),
                            span,
                            kind: ArithmeticOperationKind::Negative {
                                id: self.id(),
                                minus: op_span,
                                right,
                            },
                        },
                    )),
                    TokenKind::Plus => ExpressionKind::ArithmeticOperation(Box::new(
                        ArithmeticOperationExpression {
                            id: self.id(),
                            span,
                            kind: ArithmeticOperationKind::Positive {
                                id: self.id(),
                                plus: op_span,
                                right,
                            },
                        },
                    )),
                    TokenKind::Decrement => ExpressionKind::ArithmeticOperation(Box::new(
                        ArithmeticOperationExpression {
                            id: self.id(),
                            span,
                            kind: ArithmeticOperationKind::PreDecrement {
                                id: self.id(),
                                decrement: op_span,
                                right,
                            },
                        },
                    )),
                    TokenKind::Increment => ExpressionKind::ArithmeticOperation(Box::new(
                        ArithmeticOperationExpression {
                            id: self.id(),
                            span,
                            kind: ArithmeticOperationKind::PreIncrement {
                                id: self.id(),
                                increment: op_span,
                                right,
                            },
                        },
                    )),
                    _ => unreachable!(),
                };

                Expression::new(
                    self.id(),
                    expr,
                    Span::new(start_span.start, right_span.end),
                    CommentGroup::default(),
                )
            }

            (TokenKind::Bang, _) => {
                let start_span = self.current().span;
                let bang = self.current().span;

                self.next();

                let rhs = self.for_precedence(Precedence::Bang);
                let end_span = rhs.span;
                let span = Span::combine(start_span, end_span);

                Expression::new(
                    self.id(),
                    ExpressionKind::LogicalOperation(Box::new(LogicalOperationExpression {
                        id: self.id(),
                        span,
                        kind: LogicalOperationKind::Not {
                            id: self.id(),
                            bang,
                            right: Box::new(rhs),
                        },
                    })),
                    Span::new(start_span.start, end_span.end),
                    CommentGroup::default(),
                )
            }

            (TokenKind::At, _) => {
                let span = self.current().span;

                self.next();

                let rhs = self.for_precedence(Precedence::Prefix);
                let end_span = rhs.span;
                let span = Span::combine(span, end_span);

                Expression::new(
                    self.id(),
                    ExpressionKind::ErrorSuppress(Box::new(ErrorSuppressExpression {
                        id: self.id(),
                        span,
                        at: span,
                        expr: Box::new(rhs),
                    })),
                    Span::new(span.start, end_span.end),
                    CommentGroup::default(),
                )
            }

            (TokenKind::BitwiseNot, _) => {
                let span = self.current().span;

                self.next();

                let right = Box::new(self.for_precedence(Precedence::Prefix));
                let end_span = right.span;
                let span = Span::combine(span, end_span);

                Expression::new(
                    self.id(),
                    ExpressionKind::BitwiseOperation(Box::new(BitwiseOperationExpression {
                        span,
                        kind: BitwiseOperationKind::Not {
                            id: self.id(),
                            not: span,
                            right,
                        },
                        id: self.id(),
                    })),
                    Span::new(span.start, end_span.end),
                    CommentGroup::default(),
                )
            }

            (TokenKind::Dollar | TokenKind::DollarLeftBrace | TokenKind::Variable, _) => {
                let span = self.current().span;

                Expression::new(
                    self.id(),
                    ExpressionKind::Variable(Box::new(self.parse_dynamic_variable())),
                    span,
                    CommentGroup::default(),
                )
            }

            _ => self.unexpected_token(precedence),
        }
    }

    fn unexpected_token(&mut self, _: &Precedence) -> Expression {
        let kind = self.current_kind();
        let span = self.current_span();

        self.diagnostic(
            ParserDiagnostic::UnexpectedToken {
                token: self.current().to_owned(),
            },
            Severity::Error,
            span,
        );

        // This is a common case where we don't want to consume the right-brace as it might close a structure.
        if kind != TokenKind::RightBrace {
            self.next();
        }

        Expression::missing(self.id(), span)
    }

    fn postfix(&mut self, lhs: Expression, op: TokenKind) -> Expression {
        let start_span = self.current().span;
        let kind = match op {
            TokenKind::DoubleQuestion => {
                let double_question = self.current().span;
                self.next();

                let rhs = self.null_coalesce_precedence();
                let span = Span::combine(lhs.span, rhs.span);

                ExpressionKind::Coalesce(Box::new(CoalesceExpression {
                    id: self.id(),
                    span,
                    lhs: Box::new(lhs),
                    double_question,
                    rhs: Box::new(rhs),
                }))
            }
            TokenKind::LeftParen => {
                // `(...)` closure creation
                if self.peek_kind() == TokenKind::Ellipsis
                    && self.peek_again_kind() == TokenKind::RightParen
                {
                    let start = self.skip(TokenKind::LeftParen);
                    let ellipsis = self.skip(TokenKind::Ellipsis);
                    let end = self.skip(TokenKind::RightParen);
                    let span = Span::combine(start, end);

                    let placeholder = ArgumentPlaceholder {
                        id: self.id(),
                        span,
                        comments: self.comments(),
                        left_parenthesis: start,
                        ellipsis,
                        right_parenthesis: end,
                    };

                    let span = Span::combine(lhs.span, span);

                    ExpressionKind::FunctionClosureCreation(Box::new(
                        FunctionClosureCreationExpression {
                            id: self.id(),
                            span,
                            target: Box::new(lhs),
                            placeholder,
                        },
                    ))
                } else {
                    let arguments = self.parse_argument_list();
                    let span = Span::combine(lhs.span, arguments.span);

                    ExpressionKind::FunctionCall(Box::new(FunctionCallExpression {
                        id: self.id(),
                        span,
                        target: Box::new(lhs),
                        arguments,
                    }))
                }
            }
            TokenKind::LeftBracket => {
                let left_bracket = self.skip_left_bracket();
                let index = if self.current_kind() == TokenKind::RightBracket {
                    None
                } else {
                    Some(Box::new(self.parse_expression()))
                };
                let right_bracket = self.skip_right_bracket();
                let span = Span::combine(lhs.span, right_bracket);

                ExpressionKind::ArrayIndex(Box::new(ArrayIndexExpression {
                    id: self.id(),
                    span,
                    array: Box::new(lhs),
                    left_bracket,
                    index,
                    right_bracket,
                }))
            }
            TokenKind::DoubleColon => {
                let double_colon = self.skip_double_colon();

                let property = match self.current_kind() {
                    TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
                        ExpressionKind::Variable(Box::new(self.parse_dynamic_variable()))
                    }
                    _ if self.is_identifier_maybe_reserved(self.current_kind()) => {
                        ExpressionKind::Identifier(Box::new(Identifier::SimpleIdentifier(
                            self.parse_identifier_maybe_reserved(),
                        )))
                    }
                    TokenKind::LeftBrace => {
                        let start = self.next();

                        let expr = Box::new(self.parse_expression());
                        let end = self.skip_right_brace();

                        let span = Span::new(start.start, end.end);

                        ExpressionKind::Identifier(Box::new(Identifier::DynamicIdentifier(
                            DynamicIdentifier {
                                id: self.id(),
                                span,
                                expr,
                            },
                        )))
                    }
                    TokenKind::Class => {
                        let symbol = self.current_symbol_as_bytestring();
                        let span = self.next();

                        ExpressionKind::Identifier(Box::new(Identifier::SimpleIdentifier(
                            SimpleIdentifier::new(self.id(), symbol, span),
                        )))
                    }
                    _ => {
                        self.diagnostic(
                            ParserDiagnostic::ExpectedToken {
                                expected: vec![
                                    TokenKind::LeftBrace,
                                    TokenKind::Dollar,
                                    TokenKind::Identifier,
                                ],
                                found: self.current().to_owned(),
                            },
                            Severity::Error,
                            self.current().span,
                        );

                        let span = self.current_span();

                        self.next();

                        ExpressionKind::Missing(MissingExpression { id: 0, span })
                    }
                };

                let lhs = Box::new(lhs);

                if self.current_kind() == TokenKind::LeftParen {
                    if self.peek_kind() == TokenKind::Ellipsis
                        && self.peek_again_kind() == TokenKind::RightParen
                    {
                        let start = self.skip(TokenKind::LeftParen);
                        let ellipsis = self.skip(TokenKind::Ellipsis);
                        let end = self.skip(TokenKind::RightParen);
                        let span = Span::combine(start, end);

                        let placeholder = ArgumentPlaceholder {
                            id: self.id(),
                            span,
                            comments: self.comments(),
                            left_parenthesis: start,
                            ellipsis,
                            right_parenthesis: end,
                        };

                        match property {
                            ExpressionKind::Identifier(identifier) => {
                                ExpressionKind::StaticMethodClosureCreation(Box::new(
                                    StaticMethodClosureCreationExpression {
                                        id: self.id(),
                                        span: Span::combine(lhs.span, placeholder.span),
                                        target: lhs,
                                        double_colon,
                                        method: *identifier,
                                        placeholder,
                                    },
                                ))
                            }
                            ExpressionKind::Variable(variable) => {
                                ExpressionKind::StaticVariableMethodClosureCreation(Box::new(
                                    StaticVariableMethodClosureCreationExpression {
                                        id: self.id(),
                                        span: Span::combine(lhs.span, placeholder.span),
                                        target: lhs,
                                        double_colon,
                                        method: *variable,
                                        placeholder,
                                    },
                                ))
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        let arguments = self.parse_argument_list();

                        match property {
                            ExpressionKind::Identifier(identifier) => {
                                ExpressionKind::StaticMethodCall(Box::new(
                                    StaticMethodCallExpression {
                                        id: self.id(),
                                        span: Span::combine(lhs.span, arguments.span),
                                        target: lhs,
                                        double_colon,
                                        method: *identifier,
                                        arguments,
                                    },
                                ))
                            }
                            ExpressionKind::Variable(variable) => {
                                ExpressionKind::StaticVariableMethodCall(Box::new(
                                    StaticVariableMethodCallExpression {
                                        id: self.id(),
                                        span: Span::combine(lhs.span, arguments.span),
                                        target: lhs,
                                        double_colon,
                                        method: *variable,
                                        arguments,
                                    },
                                ))
                            }
                            _ => unreachable!(),
                        }
                    }
                } else {
                    match property {
                        ExpressionKind::Identifier(identifier) => {
                            ExpressionKind::ConstantFetch(Box::new(ConstantFetchExpression {
                                id: self.id(),
                                span: Span::combine(lhs.span, identifier.span()),
                                target: lhs,
                                double_colon,
                                constant: *identifier,
                            }))
                        }
                        ExpressionKind::Variable(variable) => ExpressionKind::StaticPropertyFetch(
                            Box::new(StaticPropertyFetchExpression {
                                id: self.id(),
                                span: Span::combine(lhs.span, variable.span()),
                                target: lhs,
                                double_colon,
                                property: *variable,
                            }),
                        ),
                        _ => {
                            let span = Span::combine(lhs.span, double_colon);

                            ExpressionKind::ConstantFetch(Box::new(ConstantFetchExpression {
                                id: self.id(),
                                span,
                                target: lhs,
                                double_colon,
                                constant: Identifier::missing(
                                    self.id(),
                                    Span::flat(double_colon.end),
                                ),
                            }))
                        }
                    }
                }
            }
            TokenKind::Arrow | TokenKind::QuestionArrow => {
                let span = self.next();

                let property = match self.current_kind() {
                    TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
                        let start_span = self.current_span();
                        let kind =
                            ExpressionKind::Variable(Box::new(self.parse_dynamic_variable()));
                        let span = Span::combine(start_span, kind.span());

                        Expression::new(self.id(), kind, span, CommentGroup::default())
                    }
                    _ if self.is_identifier_maybe_reserved(self.current_kind()) => {
                        let start_span = self.current_span();
                        let kind = ExpressionKind::Identifier(Box::new(
                            Identifier::SimpleIdentifier(self.parse_identifier_maybe_reserved()),
                        ));
                        let span = Span::combine(start_span, kind.span());

                        Expression::new(self.id(), kind, span, CommentGroup::default())
                    }
                    TokenKind::LeftBrace => {
                        let start = self.current().span;
                        self.next();

                        let name = self.parse_expression();

                        let end = self.skip_right_brace();
                        let span = Span::new(start.start, end.end);

                        Expression::new(
                            self.id(),
                            ExpressionKind::Identifier(Box::new(Identifier::DynamicIdentifier(
                                DynamicIdentifier {
                                    id: self.id(),
                                    span,
                                    expr: Box::new(name),
                                },
                            ))),
                            Span::new(start.start, end.end),
                            CommentGroup::default(),
                        )
                    }
                    _ => {
                        let span = self.current_span();

                        self.diagnostic(
                            ParserDiagnostic::ExpectedToken {
                                expected: vec![
                                    TokenKind::LeftBrace,
                                    TokenKind::Dollar,
                                    TokenKind::Identifier,
                                ],
                                found: self.current().to_owned(),
                            },
                            Severity::Error,
                            span,
                        );

                        Expression::missing(self.id(), span)
                    }
                };

                if self.current_kind() == TokenKind::LeftParen {
                    if op == TokenKind::QuestionArrow {
                        let arguments = self.parse_argument_list();

                        ExpressionKind::NullsafeMethodCall(Box::new(NullsafeMethodCallExpression {
                            id: self.id(),
                            span: Span::combine(lhs.span, arguments.span),
                            target: Box::new(lhs),
                            method: Box::new(property),
                            question_arrow: span,
                            arguments,
                        }))
                    } else {
                        // `(...)` closure creation
                        if self.peek_kind() == TokenKind::Ellipsis
                            && self.peek_again_kind() == TokenKind::RightParen
                        {
                            let start = self.skip(TokenKind::LeftParen);
                            let ellipsis = self.skip(TokenKind::Ellipsis);
                            let end = self.skip(TokenKind::RightParen);
                            let span = Span::combine(start, end);

                            let placeholder = ArgumentPlaceholder {
                                id: self.id(),
                                span,
                                comments: self.comments(),
                                left_parenthesis: start,
                                ellipsis,
                                right_parenthesis: end,
                            };

                            ExpressionKind::MethodClosureCreation(Box::new(
                                MethodClosureCreationExpression {
                                    id: self.id(),
                                    span: Span::combine(lhs.span, placeholder.span),
                                    target: Box::new(lhs),
                                    method: Box::new(property),
                                    arrow: span,
                                    placeholder,
                                },
                            ))
                        } else {
                            let arguments = self.parse_argument_list();

                            ExpressionKind::MethodCall(Box::new(MethodCallExpression {
                                id: self.id(),
                                span: Span::combine(lhs.span, arguments.span),
                                target: Box::new(lhs),
                                method: Box::new(property),
                                arrow: span,
                                arguments,
                            }))
                        }
                    }
                } else if op == TokenKind::QuestionArrow {
                    ExpressionKind::NullsafePropertyFetch(Box::new(
                        NullsafePropertyFetchExpression {
                            id: self.id(),
                            span: Span::combine(lhs.span, property.span),
                            target: Box::new(lhs),
                            question_arrow: span,
                            property: Box::new(property),
                        },
                    ))
                } else {
                    ExpressionKind::PropertyFetch(Box::new(PropertyFetchExpression {
                        id: self.id(),
                        span: Span::combine(lhs.span, property.span),
                        target: Box::new(lhs),
                        arrow: span,
                        property: Box::new(property),
                    }))
                }
            }
            TokenKind::Increment => {
                let op = self.current().span;
                self.next();

                ExpressionKind::ArithmeticOperation(Box::new(ArithmeticOperationExpression {
                    id: self.id(),
                    span: Span::combine(lhs.span, op),
                    kind: ArithmeticOperationKind::PostIncrement {
                        id: self.id(),
                        left: Box::new(lhs),
                        increment: op,
                    },
                }))
            }
            TokenKind::Decrement => {
                let op = self.current().span;
                self.next();

                ExpressionKind::ArithmeticOperation(Box::new(ArithmeticOperationExpression {
                    id: self.id(),
                    span: Span::combine(lhs.span, op),
                    kind: ArithmeticOperationKind::PostDecrement {
                        id: self.id(),
                        left: Box::new(lhs),
                        decrement: op,
                    },
                }))
            }
            _ => unreachable!(),
        };

        let span = Span::combine(start_span, kind.span());

        Expression::new(self.id(), kind, span, CommentGroup::default())
    }

    fn is_infix(&self, t: TokenKind) -> bool {
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
    fn is_postfix(&self, t: TokenKind) -> bool {
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
}
