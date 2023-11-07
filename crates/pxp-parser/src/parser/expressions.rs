use crate::expected_token_err;
use crate::lexer::token::DocStringKind;
use crate::lexer::token::TokenKind;
use crate::parser::ast::arguments::ArgumentPlaceholder;
use crate::parser::ast::identifiers::DynamicIdentifier;
use crate::parser::ast::identifiers::Identifier;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::literals::Literal;
use crate::parser::ast::literals::LiteralFloat;
use crate::parser::ast::literals::LiteralInteger;
use crate::parser::ast::literals::LiteralString;
use crate::parser::ast::operators::ArithmeticOperationExpression;
use crate::parser::ast::operators::AssignmentOperationExpression;
use crate::parser::ast::operators::BitwiseOperationExpression;
use crate::parser::ast::operators::ComparisonOperationExpression;
use crate::parser::ast::operators::LogicalOperationExpression;
use crate::parser::ast::{
    ArrayIndexExpression, CoalesceExpression, ConcatExpression, ConstantFetchExpression,
    Expression, FunctionCallExpression, FunctionClosureCreationExpression, InstanceofExpression,
    MagicConstantExpression, MethodCallExpression, MethodClosureCreationExpression,
    NullsafeMethodCallExpression, NullsafePropertyFetchExpression, PropertyFetchExpression,
    ReferenceExpression, ShortTernaryExpression, StaticMethodCallExpression,
    StaticMethodClosureCreationExpression, StaticPropertyFetchExpression,
    StaticVariableMethodCallExpression, StaticVariableMethodClosureCreationExpression,
    TernaryExpression,
};
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::internal::arrays;
use crate::parser::internal::attributes;
use crate::parser::internal::classes;
use crate::parser::internal::control_flow;
use crate::parser::internal::functions;
use crate::parser::internal::identifiers;
use crate::parser::internal::parameters;
use crate::parser::internal::precedences::Associativity;
use crate::parser::internal::precedences::Precedence;
use crate::parser::internal::strings;
use crate::parser::internal::utils;
use crate::parser::internal::variables;
use crate::parser::state::State;

use super::ast::literals::LiteralStringKind;
use super::ast::BoolExpression;
use super::ast::CastExpression;
use super::ast::CloneExpression;
use super::ast::DieExpression;
use super::ast::EmptyExpression;
use super::ast::ErrorSuppressExpression;
use super::ast::EvalExpression;
use super::ast::ExitExpression;
use super::ast::IncludeExpression;
use super::ast::IncludeOnceExpression;
use super::ast::IssetExpression;
use super::ast::NewExpression;
use super::ast::ParenthesizedExpression;
use super::ast::PrintExpression;
use super::ast::RequireExpression;
use super::ast::RequireOnceExpression;
use super::ast::ThrowExpression;
use super::ast::UnsetExpression;
use super::ast::YieldExpression;
use super::ast::YieldFromExpression;

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

            left = match kind {
                TokenKind::Question => {
                    // this happens due to a comment, or whitespaces between the ? and the :
                    // we consider `foo() ? : bar()` a ternary expression, with `then` being a noop
                    // however, this must behave like a short ternary at runtime.
                    if op.kind == TokenKind::Colon {
                        state.stream.next();

                        let r#else = create(state)?;

                        Expression::Ternary(TernaryExpression {
                            condition: Box::new(left),
                            question: span,
                            then: Box::new(Expression::Noop),
                            colon: op.span,
                            r#else: Box::new(r#else),
                        })
                    } else {
                        let then = create(state)?;
                        let colon = utils::skip_colon(state)?;
                        let r#else = create(state)?;

                        Expression::Ternary(TernaryExpression {
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
                    Expression::ShortTernary(ShortTernaryExpression {
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

                    Expression::AssignmentOperation(AssignmentOperationExpression::Assign {
                        left: Box::new(left),
                        equals: span,
                        right: Box::new(Expression::Reference(ReferenceExpression {
                            ampersand: op.span,
                            right,
                        })),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Self_ => {
                    state.stream.next();

                    Expression::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::Self_),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Parent => {
                    state.stream.next();

                    Expression::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::Parent),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Static => {
                    state.stream.next();

                    Expression::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::Static),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::Enum => {
                    let enum_span = op.span;
                    state.stream.next();

                    Expression::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::Identifier(Identifier::SimpleIdentifier(
                            SimpleIdentifier {
                                span: enum_span,
                                value: "enum".into(),
                            },
                        ))),
                    })
                }
                TokenKind::Instanceof if op.kind == TokenKind::From => {
                    let from_span = op.span;
                    state.stream.next();

                    Expression::Instanceof(InstanceofExpression {
                        left: Box::new(left),
                        instanceof: span,
                        right: Box::new(Expression::Identifier(Identifier::SimpleIdentifier(
                            SimpleIdentifier {
                                span: from_span,
                                value: "from".into(),
                            },
                        ))),
                    })
                }
                _ => {
                    let left = Box::new(left);
                    let right = Box::new(for_precedence(state, rpred)?);

                    match kind {
                        TokenKind::Plus => Expression::ArithmeticOperation(
                            ArithmeticOperationExpression::Addition {
                                left,
                                plus: span,
                                right,
                            },
                        ),
                        TokenKind::Minus => Expression::ArithmeticOperation(
                            ArithmeticOperationExpression::Subtraction {
                                left,
                                minus: span,
                                right,
                            },
                        ),
                        TokenKind::Asterisk => Expression::ArithmeticOperation(
                            ArithmeticOperationExpression::Multiplication {
                                left,
                                asterisk: span,
                                right,
                            },
                        ),
                        TokenKind::Slash => Expression::ArithmeticOperation(
                            ArithmeticOperationExpression::Division {
                                left,
                                slash: span,
                                right,
                            },
                        ),
                        TokenKind::Percent => {
                            Expression::ArithmeticOperation(ArithmeticOperationExpression::Modulo {
                                left,
                                percent: span,
                                right,
                            })
                        }
                        TokenKind::Pow => Expression::ArithmeticOperation(
                            ArithmeticOperationExpression::Exponentiation {
                                left,
                                pow: span,
                                right,
                            },
                        ),
                        TokenKind::Equals => {
                            Expression::AssignmentOperation(AssignmentOperationExpression::Assign {
                                left,
                                equals: span,
                                right,
                            })
                        }
                        TokenKind::PlusEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::Addition {
                                left,
                                plus_equals: span,
                                right,
                            },
                        ),
                        TokenKind::MinusEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::Subtraction {
                                left,
                                minus_equals: span,
                                right,
                            },
                        ),
                        TokenKind::AsteriskEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::Multiplication {
                                left,
                                asterisk_equals: span,
                                right,
                            },
                        ),
                        TokenKind::SlashEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::Division {
                                left,
                                slash_equals: span,
                                right,
                            },
                        ),
                        TokenKind::PercentEquals => {
                            Expression::AssignmentOperation(AssignmentOperationExpression::Modulo {
                                left,
                                percent_equals: span,
                                right,
                            })
                        }
                        TokenKind::PowEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::Exponentiation {
                                left,
                                pow_equals: span,
                                right,
                            },
                        ),
                        TokenKind::AmpersandEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::BitwiseAnd {
                                left,
                                ampersand_equals: span,
                                right,
                            },
                        ),
                        TokenKind::PipeEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::BitwiseOr {
                                left,
                                pipe_equals: span,
                                right,
                            },
                        ),
                        TokenKind::CaretEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::BitwiseXor {
                                left,
                                caret_equals: span,
                                right,
                            },
                        ),
                        TokenKind::LeftShiftEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::LeftShift {
                                left,
                                left_shift_equals: span,
                                right,
                            },
                        ),
                        TokenKind::RightShiftEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::RightShift {
                                left,
                                right_shift_equals: span,
                                right,
                            },
                        ),
                        TokenKind::DoubleQuestionEquals => Expression::AssignmentOperation(
                            AssignmentOperationExpression::Coalesce {
                                left,
                                coalesce_equals: span,
                                right,
                            },
                        ),
                        TokenKind::DotEquals => {
                            Expression::AssignmentOperation(AssignmentOperationExpression::Concat {
                                left,
                                dot_equals: span,
                                right,
                            })
                        }
                        TokenKind::Ampersand => {
                            Expression::BitwiseOperation(BitwiseOperationExpression::And {
                                left,
                                and: span,
                                right,
                            })
                        }
                        TokenKind::Pipe => {
                            Expression::BitwiseOperation(BitwiseOperationExpression::Or {
                                left,
                                or: span,
                                right,
                            })
                        }
                        TokenKind::Caret => {
                            Expression::BitwiseOperation(BitwiseOperationExpression::Xor {
                                left,
                                xor: span,
                                right,
                            })
                        }
                        TokenKind::LeftShift => {
                            Expression::BitwiseOperation(BitwiseOperationExpression::LeftShift {
                                left,
                                left_shift: span,
                                right,
                            })
                        }
                        TokenKind::RightShift => {
                            Expression::BitwiseOperation(BitwiseOperationExpression::RightShift {
                                left,
                                right_shift: span,
                                right,
                            })
                        }
                        TokenKind::DoubleEquals => {
                            Expression::ComparisonOperation(ComparisonOperationExpression::Equal {
                                left,
                                double_equals: span,
                                right,
                            })
                        }
                        TokenKind::TripleEquals => Expression::ComparisonOperation(
                            ComparisonOperationExpression::Identical {
                                left,
                                triple_equals: span,
                                right,
                            },
                        ),
                        TokenKind::BangEquals => Expression::ComparisonOperation(
                            ComparisonOperationExpression::NotEqual {
                                left,
                                bang_equals: span,
                                right,
                            },
                        ),
                        TokenKind::AngledLeftRight => Expression::ComparisonOperation(
                            ComparisonOperationExpression::AngledNotEqual {
                                left,
                                angled_left_right: span,
                                right,
                            },
                        ),
                        TokenKind::BangDoubleEquals => Expression::ComparisonOperation(
                            ComparisonOperationExpression::NotIdentical {
                                left,
                                bang_double_equals: span,
                                right,
                            },
                        ),
                        TokenKind::LessThan => Expression::ComparisonOperation(
                            ComparisonOperationExpression::LessThan {
                                left,
                                less_than: span,
                                right,
                            },
                        ),
                        TokenKind::GreaterThan => Expression::ComparisonOperation(
                            ComparisonOperationExpression::GreaterThan {
                                left,
                                greater_than: span,
                                right,
                            },
                        ),
                        TokenKind::LessThanEquals => Expression::ComparisonOperation(
                            ComparisonOperationExpression::LessThanOrEqual {
                                left,
                                less_than_equals: span,
                                right,
                            },
                        ),
                        TokenKind::GreaterThanEquals => Expression::ComparisonOperation(
                            ComparisonOperationExpression::GreaterThanOrEqual {
                                left,
                                greater_than_equals: span,
                                right,
                            },
                        ),
                        TokenKind::Spaceship => Expression::ComparisonOperation(
                            ComparisonOperationExpression::Spaceship {
                                left,
                                spaceship: span,
                                right,
                            },
                        ),
                        TokenKind::BooleanAnd => {
                            Expression::LogicalOperation(LogicalOperationExpression::And {
                                left,
                                double_ampersand: span,
                                right,
                            })
                        }
                        TokenKind::BooleanOr => {
                            Expression::LogicalOperation(LogicalOperationExpression::Or {
                                left,
                                double_pipe: span,
                                right,
                            })
                        }
                        TokenKind::LogicalAnd => {
                            Expression::LogicalOperation(LogicalOperationExpression::LogicalAnd {
                                left,
                                and: span,
                                right,
                            })
                        }
                        TokenKind::LogicalOr => {
                            Expression::LogicalOperation(LogicalOperationExpression::LogicalOr {
                                left,
                                or: span,
                                right,
                            })
                        }
                        TokenKind::LogicalXor => {
                            Expression::LogicalOperation(LogicalOperationExpression::LogicalXor {
                                left,
                                xor: span,
                                right,
                            })
                        }
                        TokenKind::Dot => Expression::Concat(ConcatExpression {
                            left,
                            dot: span,
                            right,
                        }),
                        TokenKind::Instanceof => Expression::Instanceof(InstanceofExpression {
                            left,
                            instanceof: span,
                            right,
                        }),
                        _ => todo!(),
                    }
                }
            };

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
            pub(in crate::parser) fn $expr($state: &mut State, precedence: &Precedence) -> ParseResult<Expression> {
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
        let eval = state.stream.current().span;
        state.stream.next();

        let argument = Box::new(parameters::single_argument(state, true, true).unwrap()?);

        Ok(Expression::Eval(EvalExpression { eval, argument }))
    })

    #[before(die), current(TokenKind::Empty), peek(TokenKind::LeftParen)]
    empty({
        let empty = state.stream.current().span;
        state.stream.next();

        let argument = Box::new(parameters::single_argument(state, true, true).unwrap()?);

        Ok(Expression::Empty(EmptyExpression { empty, argument }))
    })

    #[before(exit), current(TokenKind::Die)]
    die({
        let die = state.stream.current().span;
        state.stream.next();

        let argument = match parameters::single_argument(state, false, true) {
            Some(arg) => Some(Box::new(arg?)),
            None => None,
        };

        Ok(Expression::Die(DieExpression { die, argument }))
    })

    #[before(isset), current(TokenKind::Exit)]
    exit({
        let exit = state.stream.current().span;
        state.stream.next();

        let argument = match parameters::single_argument(state, false, true) {
            Some(arg) => Some(Box::new(arg?)),
            None => None,
        };

        Ok(Expression::Exit(ExitExpression { exit, argument }))
    })

    #[before(unset), current(TokenKind::Isset), peek(TokenKind::LeftParen)]
    isset({
        let isset = state.stream.current().span;
        state.stream.next();
        let arguments = parameters::argument_list(state)?;

        Ok(Expression::Isset(IssetExpression { isset, arguments}))
    })

    #[before(print), current(TokenKind::Unset), peek(TokenKind::LeftParen)]
    unset({
        let unset = state.stream.current().span;
        state.stream.next();
        let arguments = parameters::argument_list(state)?;

        Ok(Expression::Unset(UnsetExpression { unset, arguments}))
    })

    #[before(reserved_identifier_function_call), current(TokenKind::Print)]
    print({
        let print = state.stream.current().span;
        state.stream.next();

        let mut value = None;
        let mut argument = None;

        if let Some(arg) = parameters::single_argument(state, false, true) {
            argument = Some(Box::new(arg?));
        } else {
            value = Some(Box::new(create(state)?));
        }

        Ok(Expression::Print(PrintExpression { print, value, argument }))
    })

    #[before(reserved_identifier_static_call), precedence(Precedence::CallDim), current(
        | TokenKind::True       | TokenKind::False | TokenKind::Null
        | TokenKind::Readonly   | TokenKind::Self_ | TokenKind::Parent
        | TokenKind::Enum       | TokenKind::From
    ), peek(TokenKind::LeftParen)]
    reserved_identifier_function_call({
        let ident = identifiers::identifier_maybe_soft_reserved(state)?;
        let lhs = Expression::Identifier(Identifier::SimpleIdentifier(ident));

        postfix(state, lhs, &TokenKind::LeftParen)
    })

    #[before(list), current(TokenKind::Enum | TokenKind::From), peek(TokenKind::DoubleColon)]
    reserved_identifier_static_call({
        let ident = identifiers::type_identifier(state)?;
        let lhs = Expression::Identifier(Identifier::SimpleIdentifier(ident));

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
        state.stream.next();

        Ok(Expression::Throw(ThrowExpression {
            value: Box::new(for_precedence(state, Precedence::Lowest)?)
        }))
    })

    #[before(clone), current(TokenKind::Yield)]
    r#yield({
        state.stream.next();
        if state.stream.current().kind == TokenKind::SemiColon || state.stream.current().kind == TokenKind::RightParen {
            Ok(Expression::Yield(YieldExpression {
                key: None,
                value: None,
            }))
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

            if from {
                Ok(Expression::YieldFrom(YieldFromExpression { value }))
            } else {
                Ok(Expression::Yield(YieldExpression {
                    key,
                    value: Some(value),
                }))
            }
        }
    })

    #[before(r#true), current(TokenKind::Clone)]
    clone({
        state.stream.next();

        let target = for_precedence(state, Precedence::CloneOrNew)?;

        Ok(Expression::Clone(CloneExpression {
            target: Box::new(target),
        }))
    })

    #[before(r#false), current(TokenKind::True)]
    r#true({
        state.stream.next();

        Ok(Expression::Bool(BoolExpression { value: true }))
    })

    #[before(null), current(TokenKind::False)]
    r#false({
        state.stream.next();

        Ok(Expression::Bool(BoolExpression { value: false }))
    })

    #[before(literal_integer), current(TokenKind::Null)]
    null({
        state.stream.next();

        Ok(Expression::Null)
    })

    #[before(literal_float), current(TokenKind::LiteralInteger)]
    literal_integer({
        let current = state.stream.current();

        if let TokenKind::LiteralInteger = &current.kind {
            state.stream.next();

            Ok(Expression::Literal(Literal::Integer(
                LiteralInteger {
                    span: current.span,
                    value: current.value.clone()
                }
            )))
        } else {
            unreachable!("{}:{}", file!(), line!());
        }
    })

    #[before(literal_string), current(TokenKind::LiteralFloat)]
    literal_float({
        let current = state.stream.current();

        if let TokenKind::LiteralFloat = &current.kind {
            state.stream.next();

            Ok(Expression::Literal(
                Literal::Float(LiteralFloat {
                    span: current.span,
                    value: current.value.clone()
                })
            ))
        } else {
            unreachable!("{}:{}", file!(), line!());
        }
    })

    #[before(string_part), current(TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString)]
    literal_string({
        let current = state.stream.current();

        if let TokenKind::LiteralSingleQuotedString = &current.kind {
            state.stream.next();

            Ok(Expression::Literal(
                Literal::String(LiteralString {
                    span: current.span,
                    value: current.value.clone(),
                    kind: LiteralStringKind::SingleQuoted,
                })
            ))
        } else if let TokenKind::LiteralDoubleQuotedString = &current.kind {
            state.stream.next();

            Ok(Expression::Literal(
                Literal::String(LiteralString {
                    span: current.span,
                    value: current.value.clone(),
                    kind: LiteralStringKind::DoubleQuoted,
                })
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
        Ok(Expression::Identifier(Identifier::SimpleIdentifier(identifiers::full_name(state)?)))
    })

    #[before(self_identifier), current(TokenKind::Static)]
    static_postfix({
        state.stream.next();

        postfix(state, Expression::Static, &TokenKind::DoubleColon)
    })

    #[before(parent_identifier), current(TokenKind::Self_)]
    self_identifier({
        state.stream.next();

        Ok(Expression::Self_)
    })

    #[before(left_parenthesis), current(TokenKind::Parent)]
    parent_identifier({
        state.stream.next();

        Ok(Expression::Parent)
    })

    #[before(r#match), current(TokenKind::LeftParen)]
    left_parenthesis({
        let start = state.stream.current().span;
        state.stream.next();

        let expr = create(state)?;

        let end = utils::skip_right_parenthesis(state)?;

        Ok(Expression::Parenthesized(ParenthesizedExpression { start, expr: Box::new(expr), end }))
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

        let target = match state.stream.current().kind {
            TokenKind::Self_ => {
                state.stream.next();

                Expression::Self_
            }
            TokenKind::Static => {
                state.stream.next();

                Expression::Static
            }
            TokenKind::Parent => {
                state.stream.next();

                Expression::Parent
            }
            TokenKind::Enum => {
                let span = state.stream.current().span;

                state.stream.next();

                Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { span, value: "enum".into() }))
            }
            TokenKind::From => {
                let span = state.stream.current().span;

                state.stream.next();

                Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { span, value: "from".into() }))
            }
            _ => clone_or_new_precedence(state)?,
        };

        let arguments = if state.stream.current().kind == TokenKind::LeftParen {
            Some(parameters::argument_list(state)?)
        } else {
            None
        };

        Ok(Expression::New(NewExpression {
            target: Box::new(target),
            new,
            arguments,
        }))
    })

    #[before(file_magic_constant), current(TokenKind::DirConstant)]
    directory_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Directory(span)))
    })

    #[before(line_magic_constant), current(TokenKind::FileConstant)]
    file_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::File(span)))
    })

    #[before(function_magic_constant), current(TokenKind::LineConstant)]
    line_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Line(span)))
    })

    #[before(class_magic_constant), current(TokenKind::FunctionConstant)]
    function_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Function(span)))
    })

    #[before(method_magic_constant), current(TokenKind::ClassConstant)]
    class_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Class(span)))
    })

    #[before(namespace_magic_constant), current(TokenKind::MethodConstant)]
    method_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Method(span)))
    })

    #[before(trait_magic_constant), current(TokenKind::NamespaceConstant)]
    namespace_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Namespace(span)))
    })

    #[before(compiler_halt_offset_magic_constant), current(TokenKind::TraitConstant)]
    trait_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::Trait(span)))
    })

    #[before(include), current(TokenKind::CompilerHaltOffsetConstant)]
    compiler_halt_offset_magic_constant({
        let span = state.stream.current().span;
        state.stream.next();

        Ok(Expression::MagicConstant(MagicConstantExpression::CompilerHaltOffset(span)))
    })

    #[before(cast_prefix), current(TokenKind::Include | TokenKind::IncludeOnce | TokenKind::Require | TokenKind::RequireOnce)]
    include({
        let current = state.stream.current();
        let span = current.span;

        state.stream.next();

        let path = Box::new(create(state)?);

        Ok(match current.kind {
            TokenKind::Include => Expression::Include(IncludeExpression { include: span, path }),
            TokenKind::IncludeOnce => Expression::IncludeOnce(IncludeOnceExpression { include_once: span, path }),
            TokenKind::Require => Expression::Require(RequireExpression { require: span, path }),
            TokenKind::RequireOnce => Expression::RequireOnce(RequireOnceExpression { require_once: span, path }),
            _ => unreachable!()
        })
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

        Ok(Expression::Cast(CastExpression {
            cast: span,
            kind,
            value: Box::new(rhs),
        }))
    })

    #[before(bang_prefix), current(TokenKind::Decrement | TokenKind::Increment | TokenKind::Minus | TokenKind::Plus)]
    numeric_prefix({
        let current = state.stream.current();

        let span = current.span;
        let op = current.kind.clone();

        state.stream.next();

        let right = Box::new(for_precedence(state, Precedence::Prefix)?);
        let expr = match op {
            TokenKind::Minus => Expression::ArithmeticOperation(ArithmeticOperationExpression::Negative { minus: span, right }),
            TokenKind::Plus => Expression::ArithmeticOperation(ArithmeticOperationExpression::Positive { plus: span, right }),
            TokenKind::Decrement => Expression::ArithmeticOperation(ArithmeticOperationExpression::PreDecrement { decrement: span, right }),
            TokenKind::Increment => Expression::ArithmeticOperation(ArithmeticOperationExpression::PreIncrement { increment: span, right }),
            _ => unreachable!(),
        };

        Ok(expr)
    })

    #[before(at_prefix), current(TokenKind::Bang)]
    bang_prefix({
        let bang = state.stream.current().span;

        state.stream.next();

        let rhs = for_precedence(state, Precedence::Bang)?;

        Ok(Expression::LogicalOperation(LogicalOperationExpression::Not {
            bang,
            right: Box::new(rhs)
        }))
    })

    #[before(bitwise_prefix), current(TokenKind::At)]
    at_prefix({
        let span = state.stream.current().span;

        state.stream.next();

        let rhs = for_precedence(state, Precedence::Prefix)?;

        Ok(Expression::ErrorSuppress(ErrorSuppressExpression {
            at: span,
            expr: Box::new(rhs)
        }))
    })

    #[before(variable), current(TokenKind::BitwiseNot)]
    bitwise_prefix({
        let span = state.stream.current().span;

        state.stream.next();

        let right = Box::new(for_precedence(state, Precedence::Prefix)?);

        Ok(Expression::BitwiseOperation(BitwiseOperationExpression::Not { not: span, right }))
    })

    #[before(unexpected_token), current(TokenKind::Dollar | TokenKind::DollarLeftBrace | TokenKind::Variable)]
    variable({
        Ok(Expression::Variable(variables::dynamic_variable(state)?))
    })
}

fn unexpected_token(state: &mut State, _: &Precedence) -> ParseResult<Expression> {
    let current = state.stream.current();

    Err(error::unexpected_token(vec![], current))
}

fn postfix(state: &mut State, lhs: Expression, op: &TokenKind) -> ParseResult<Expression> {
    Ok(match op {
        TokenKind::DoubleQuestion => {
            let double_question = state.stream.current().span;
            state.stream.next();

            let rhs = null_coalesce_precedence(state)?;

            Expression::Coalesce(CoalesceExpression {
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

                Expression::FunctionClosureCreation(FunctionClosureCreationExpression {
                    target: Box::new(lhs),
                    placeholder,
                })
            } else {
                let arguments = parameters::argument_list(state)?;

                Expression::FunctionCall(FunctionCallExpression {
                    target: Box::new(lhs),
                    arguments,
                })
            }
        }
        TokenKind::LeftBracket => Expression::ArrayIndex(ArrayIndexExpression {
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
                    Expression::Variable(variables::dynamic_variable(state)?)
                }
                _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
                    Expression::Identifier(Identifier::SimpleIdentifier(
                        identifiers::identifier_maybe_reserved(state)?,
                    ))
                }
                TokenKind::LeftBrace => {
                    state.stream.next();

                    Expression::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
                        start: current.span,
                        expr: Box::new(create(state)?),
                        end: utils::skip_right_brace(state)?,
                    }))
                }
                TokenKind::Class => {
                    state.stream.next();

                    Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
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
                        Expression::Identifier(identifier) => {
                            Expression::StaticMethodClosureCreation(
                                StaticMethodClosureCreationExpression {
                                    target: lhs,
                                    double_colon: span,
                                    method: identifier,
                                    placeholder,
                                },
                            )
                        }
                        Expression::Variable(variable) => {
                            Expression::StaticVariableMethodClosureCreation(
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
                        Expression::Identifier(identifier) => {
                            Expression::StaticMethodCall(StaticMethodCallExpression {
                                target: lhs,
                                double_colon: span,
                                method: identifier,
                                arguments,
                            })
                        }
                        Expression::Variable(variable) => Expression::StaticVariableMethodCall(
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
                    Expression::Identifier(identifier) => {
                        Expression::ConstantFetch(ConstantFetchExpression {
                            target: lhs,
                            double_colon: span,
                            constant: identifier,
                        })
                    }
                    Expression::Variable(variable) => {
                        Expression::StaticPropertyFetch(StaticPropertyFetchExpression {
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
                    Expression::Variable(variables::dynamic_variable(state)?)
                }
                _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
                    Expression::Identifier(Identifier::SimpleIdentifier(
                        identifiers::identifier_maybe_reserved(state)?,
                    ))
                }
                TokenKind::LeftBrace => {
                    let start = state.stream.current().span;
                    state.stream.next();

                    let name = create(state)?;

                    let end = utils::skip_right_brace(state)?;

                    Expression::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
                        start,
                        expr: Box::new(name),
                        end,
                    }))
                }
                _ => {
                    return expected_token_err!(["`{`", "`$`", "an identifier"], state);
                }
            };

            if state.stream.current().kind == TokenKind::LeftParen {
                if op == &TokenKind::QuestionArrow {
                    let arguments = parameters::argument_list(state)?;

                    Expression::NullsafeMethodCall(NullsafeMethodCallExpression {
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

                        Expression::MethodClosureCreation(MethodClosureCreationExpression {
                            target: Box::new(lhs),
                            method: Box::new(property),
                            arrow: span,
                            placeholder,
                        })
                    } else {
                        let arguments = parameters::argument_list(state)?;

                        Expression::MethodCall(MethodCallExpression {
                            target: Box::new(lhs),
                            method: Box::new(property),
                            arrow: span,
                            arguments,
                        })
                    }
                }
            } else if op == &TokenKind::QuestionArrow {
                Expression::NullsafePropertyFetch(NullsafePropertyFetchExpression {
                    target: Box::new(lhs),
                    question_arrow: span,
                    property: Box::new(property),
                })
            } else {
                Expression::PropertyFetch(PropertyFetchExpression {
                    target: Box::new(lhs),
                    arrow: span,
                    property: Box::new(property),
                })
            }
        }
        TokenKind::Increment => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::ArithmeticOperation(ArithmeticOperationExpression::PostIncrement {
                left: Box::new(lhs),
                increment: span,
            })
        }
        TokenKind::Decrement => {
            let span = state.stream.current().span;
            state.stream.next();

            Expression::ArithmeticOperation(ArithmeticOperationExpression::PostDecrement {
                left: Box::new(lhs),
                decrement: span,
            })
        }
        _ => todo!("postfix: {:?}", op),
    })
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
