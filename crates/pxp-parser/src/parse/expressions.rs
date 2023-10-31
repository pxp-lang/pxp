use pxp_ast::{Expression, ExpressionKind, TernaryExpression, ShortTernaryExpression, InfixExpression, InfixOperator, ReferenceExpression, SimpleVariable, Identifier, TrueExpression, FalseExpression, NullExpression, LiteralInteger, LiteralFloat, LiteralString};
use pxp_token::TokenKind;

use crate::{state::ParserState, result::ParseError};

use super::utils::{skip_colon, skip_left_bracket, skip_right_bracket, skip_double_colon, skip_right_brace, skip, self};

pub fn create(state: &mut ParserState) -> Expression {
    for_precedence(state, Precedence::Lowest)
}

fn null_coalesce_precedence(state: &mut ParserState) -> Expression {
    for_precedence(state, Precedence::NullCoalesce)
}

fn clone_or_new_precedence(state: &mut ParserState) -> Expression {
    for_precedence(state, Precedence::CloneOrNew)
}

fn for_precedence(state: &mut ParserState, precedence: Precedence) -> Expression {
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
                utils::unexpected_token(state, &["right-associative expression"]);

                return Expression::missing(state.stream.current().span);
            }

            state.stream.next();

            let op = state.stream.current();

            left = match kind {
                TokenKind::Question => {
                    // this happens due to a comment, or whitespaces between the  and the :
                    // we consider `foo()  : bar()` a ternary expression, with `then` being a noop
                    // however, this must behave like a short ternary at runtime.
                    if op.kind == TokenKind::Colon {
                        state.stream.next();

                        let r#else = create(state);
                        let span = (left.span.start, r#else.span.end).into();

                        Expression::new(
                            ExpressionKind::Ternary(TernaryExpression {
                                condition: Box::new(left),
                                // question: span,
                                then: Box::new(Expression::noop(span)),
                                // colon: op.span,
                                r#else: Box::new(r#else),
                            }),
                            span
                        )
                    } else {
                        let then = create(state);
                        let colon = skip_colon(state);
                        let r#else = create(state);

                        let span = (left.span.start, r#else.span.end).into();

                        Expression::new(
                            ExpressionKind::Ternary(TernaryExpression {
                                condition: Box::new(left),
                                // question: span,
                                then: Box::new(then),
                                // colon: op.span,
                                r#else: Box::new(r#else),
                            }),
                            span,
                        )
                    }
                }
                TokenKind::QuestionColon => {
                    let r#else = create(state);
                    let span = (left.span.start, r#else.span.end).into();

                    Expression::new(
                        ExpressionKind::ShortTernary(ShortTernaryExpression {
                            left: Box::new(left),
                            // colon: op.span,
                            right: Box::new(r#else),
                        }),
                        span
                    )
                }
                TokenKind::Equals if op.kind == TokenKind::BitwiseAnd => {
                    state.stream.next();

                    let current = state.stream.current();

                    match current.kind {
                        TokenKind::Variable => {
                            state.stream.next();

                            Expression::new(
                                ExpressionKind::Reference(ReferenceExpression {
                                    right: SimpleVariable {
                                        name: current.clone(),
                                        span: current.span,
                                    }
                                }),
                                (left.span.start, current.span.end).into(),
                            )
                        },
                        _ => {
                            utils::unexpected_token(state, &[TokenKind::Variable]);

                            Expression::missing(current.span.with_start_as_end())
                        }
                    }
                }
                TokenKind::Instanceof if op.kind == TokenKind::Static => {
                    state.stream.next();

                    let span = (left.span.start, op.span.end).into();

                    Expression::new(
                        ExpressionKind::Infix(InfixExpression {
                            kind: InfixOperator::Instanceof(state.stream.previous().clone()),
                            left: Box::new(left),
                            right: Box::new(Expression::new(ExpressionKind::Static(op.span), op.span)),
                        }),
                        span
                    )
                }
                TokenKind::Instanceof if op.kind == TokenKind::Enum => {
                    state.stream.next();

                    let span = (left.span.start, op.span.end).into();

                    // FIXME: Convert the ::Enum token into a Identifier one.
                    Expression::new(
                        ExpressionKind::Infix(InfixExpression {
                            kind: InfixOperator::Instanceof(state.stream.previous().clone()),
                            left: Box::new(left),
                            right: Box::new(Expression::new(ExpressionKind::Identifier(Identifier::simple(op.clone(), op.span)), op.span)),
                        }),
                        span
                    )
                }
                TokenKind::Instanceof if op.kind == TokenKind::From => {
                    state.stream.next();

                    let span = (left.span.start, op.span.end).into();

                    // FIXME: Convert the ::From token into a Identifier one.
                    Expression::new(
                        ExpressionKind::Infix(InfixExpression {
                            kind: InfixOperator::Instanceof(state.stream.previous().clone()),
                            left: Box::new(left),
                            right: Box::new(Expression::new(ExpressionKind::Identifier(Identifier::simple(op.clone(), op.span)), op.span)),
                        }),
                        span
                    )
                }
                _ => {
                    let left = Box::new(left);
                    let right = Box::new(for_precedence(state, rpred));

                    let span = (left.span.start, right.span.end).into();

                    Expression::new(
                        ExpressionKind::Infix(InfixExpression {
                            kind: InfixOperator::from(op.clone()),
                            left,
                            right
                        }),
                        span
                    )
                }
            };

            continue;
        }

        break;
    }

    left
}

fn left(state: &mut ParserState, precedence: &Precedence) -> Expression {
    if state.stream.is_eof() {
        state.errors.push(ParseError::UnexpectedEndOfFile { span: state.stream.current().span });

        return Expression::missing(state.stream.current().span)
    }

    // attributes(state, precedence)
    r#true(state, precedence)
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
            pub(in crate) fn $expr($state: &mut ParserState, precedence: &Precedence) -> Expression {
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

    // #[before(static_arrow_function), current(TokenKind::Attribute)]
    // attributes({
    //     attributes::gather_attributes(state);

    //     let current = state.stream.current();

    //     match &current.kind {
    //         TokenKind::Static if state.stream.peek().kind == TokenKind::Function => {
    //             functions::anonymous_function(state)
    //         }
    //         TokenKind::Static if state.stream.peek().kind == TokenKind::Fn => {
    //             functions::arrow_function(state)
    //         }
    //         TokenKind::Function => functions::anonymous_function(state),
    //         TokenKind::Fn => functions::arrow_function(state),
    //         _ => {
    //             Err(error::missing_item_definition_after_attributes(
    //                 &state.attributes,
    //                 current,
    //             ))
    //         }
    //     }
    // })

    // #[before(static_anonymous_function), current(TokenKind::Static), peek(TokenKind::Fn)]
    // static_arrow_function({
    //     functions::arrow_function(state)
    // })

    // #[before(arrow_function), current(TokenKind::Static), peek(TokenKind::Function)]
    // static_anonymous_function({
    //     functions::anonymous_function(state)
    // })

    // #[before(anonymous_function), current(TokenKind::Fn)]
    // arrow_function({
    //     functions::arrow_function(state)
    // })

    // #[before(eval), current(TokenKind::Function)]
    // anonymous_function({
    //     functions::anonymous_function(state)
    // })

    // #[before(empty), current(TokenKind::Eval), peek(TokenKind::LeftParen)]
    // eval({
    //     let eval = state.stream.current().span;
    //     state.stream.next();

    //     let argument = Box::new(parameters::single_argument(state, true, true).unwrap());

    //     Ok(Expression::Eval(EvalExpression { eval, argument }))
    // })

    // #[before(die), current(TokenKind::Empty), peek(TokenKind::LeftParen)]
    // empty({
    //     let empty = state.stream.current().span;
    //     state.stream.next();

    //     let argument = Box::new(parameters::single_argument(state, true, true).unwrap());

    //     Ok(Expression::Empty(EmptyExpression { empty, argument }))
    // })

    // #[before(exit), current(TokenKind::Die)]
    // die({
    //     let die = state.stream.current().span;
    //     state.stream.next();

    //     let argument = match parameters::single_argument(state, false, true) {
    //         Some(arg) => Some(Box::new(arg)),
    //         None => None,
    //     };

    //     Ok(Expression::Die(DieExpression { die, argument }))
    // })

    // #[before(isset), current(TokenKind::Exit)]
    // exit({
    //     let exit = state.stream.current().span;
    //     state.stream.next();

    //     let argument = match parameters::single_argument(state, false, true) {
    //         Some(arg) => Some(Box::new(arg)),
    //         None => None,
    //     };

    //     Ok(Expression::Exit(ExitExpression { exit, argument }))
    // })

    // #[before(unset), current(TokenKind::Isset), peek(TokenKind::LeftParen)]
    // isset({
    //     let isset = state.stream.current().span;
    //     state.stream.next();
    //     let arguments = parameters::argument_list(state);

    //     Ok(Expression::Isset(IssetExpression { isset, arguments}))
    // })

    // #[before(print), current(TokenKind::Unset), peek(TokenKind::LeftParen)]
    // unset({
    //     let unset = state.stream.current().span;
    //     state.stream.next();
    //     let arguments = parameters::argument_list(state);

    //     Ok(Expression::Unset(UnsetExpression { unset, arguments}))
    // })

    // #[before(reserved_identifier_function_call), current(TokenKind::Print)]
    // print({
    //     let print = state.stream.current().span;
    //     state.stream.next();

    //     let mut value = None;
    //     let mut argument = None;

    //     if let Some(arg) = parameters::single_argument(state, false, true) {
    //         argument = Some(Box::new(arg));
    //     } else {
    //         value = Some(Box::new(create(state)));
    //     }

    //     Ok(Expression::Print(PrintExpression { print, value, argument }))
    // })

    // #[before(reserved_identifier_static_call), precedence(Precedence::CallDim), current(
    //     | TokenKind::True       | TokenKind::False | TokenKind::Null
    //     | TokenKind::Readonly   | TokenKind::Self_ | TokenKind::Parent
    //     | TokenKind::Enum       | TokenKind::From
    // ), peek(TokenKind::LeftParen)]
    // reserved_identifier_function_call({
    //     let ident = identifiers::identifier_maybe_soft_reserved(state);
    //     let lhs = Expression::Identifier(Identifier::SimpleIdentifier(ident));

    //     postfix(state, lhs, &TokenKind::LeftParen)
    // })

    // #[before(list), current(TokenKind::Enum | TokenKind::From), peek(TokenKind::DoubleColon)]
    // reserved_identifier_static_call({
    //     let ident = identifiers::type_identifier(state);
    //     let lhs = Expression::Identifier(Identifier::SimpleIdentifier(ident));

    //     postfix(state, lhs, &TokenKind::DoubleColon)
    // })

    // #[before(anonymous_class), current(TokenKind::List)]
    // list({
    //     arrays::list_expression(state)
    // })

    // #[before(throw), current(TokenKind::New), peek(TokenKind::Class | TokenKind::Attribute)]
    // anonymous_class({
    //     classes::parse_anonymous(state, None)
    // })

    // #[before(r#yield), current(TokenKind::Throw)]
    // throw({
    //     state.stream.next();

    //     Ok(Expression::Throw(ThrowExpression {
    //         value: Box::new(for_precedence(state, Precedence::Lowest))
    //     }))
    // })

    // #[before(clone), current(TokenKind::Yield)]
    // r#yield({
    //     state.stream.next();
    //     if state.stream.current().kind == TokenKind::SemiColon || state.stream.current().kind == TokenKind::RightParen {
    //         Ok(Expression::Yield(YieldExpression {
    //             key: None,
    //             value: None,
    //         }))
    //     } else {
    //         let mut from = false;

    //         if state.stream.current().kind == TokenKind::From {
    //             state.stream.next();
    //             from = true;
    //         }

    //         let mut key = None;
    //         let mut value = Box::new(for_precedence(
    //             state,
    //             if from {
    //                 Precedence::YieldFrom
    //             } else {
    //                 Precedence::Yield
    //             },
    //         ));

    //         if state.stream.current().kind == TokenKind::DoubleArrow && !from {
    //             state.stream.next();
    //             key = Some(value.clone());
    //             value = Box::new(for_precedence(state, Precedence::Yield));
    //         }

    //         if from {
    //             Ok(Expression::YieldFrom(YieldFromExpression { value }))
    //         } else {
    //             Ok(Expression::Yield(YieldExpression {
    //                 key,
    //                 value: Some(value),
    //             }))
    //         }
    //     }
    // })

    // #[before(r#true), current(TokenKind::Clone)]
    // clone({
    //     state.stream.next();

    //     let target = for_precedence(state, Precedence::CloneOrNew);

    //     Ok(Expression::Clone(CloneExpression {
    //         target: Box::new(target),
    //     }))
    // })

    #[before(r#false), current(TokenKind::True)]
    r#true({
        let token = state.stream.current().clone();
        let span = token.span;

        state.stream.next();

        Expression::new(
            ExpressionKind::True(TrueExpression { token }),
            span
        )
    })

    #[before(null), current(TokenKind::False)]
    r#false({
        let token = state.stream.current().clone();
        let span = token.span;

        state.stream.next();

        Expression::new(
            ExpressionKind::False(FalseExpression { token }),
            span
        )
    })

    #[before(literal_integer), current(TokenKind::Null)]
    null({
        let token = state.stream.current().clone();
        let span = token.span;

        state.stream.next();

        Expression::new(
            ExpressionKind::Null(NullExpression { token }),
            span
        )
    })

    #[before(literal_float), current(TokenKind::Integer)]
    literal_integer({
        let current = state.stream.current();

        state.stream.next();

        Expression::new(
            ExpressionKind::Integer(LiteralInteger { value: current.clone(), span: current.span }),
            current.span
        )
    })

    #[before(literal_string), current(TokenKind::Float)]
    literal_float({
        let current = state.stream.current();

        state.stream.next();

        Expression::new(
            ExpressionKind::Float(LiteralFloat { value: current.clone(), span: current.span }),
            current.span
        )
    })

    #[before(unexpected_token), current(TokenKind::SingleQuotedString | TokenKind::DoubleQuotedString)]
    literal_string({
        let current = state.stream.current();

        state.stream.next();

        Expression::new(
            ExpressionKind::String(LiteralString { value: current.clone(), span: current.span }),
            current.span
        )
    })

    // #[before(heredoc), current(TokenKind::StringPart)]
    // string_part({
    //     strings::interpolated(state)
    // })

    // #[before(nowdoc), current(TokenKind::StartDocString(DocStringKind::Heredoc))]
    // heredoc({
    //     strings::heredoc(state)
    // })

    // #[before(backtick), current(TokenKind::StartDocString(DocStringKind::Nowdoc))]
    // nowdoc({
    //     strings::nowdoc(state)
    // })

    // #[before(identifier), current(TokenKind::Backtick)]
    // backtick({
    //     strings::shell_exec(state)
    // })

    // #[before(static_postfix), current(TokenKind::Identifier | TokenKind::QualifiedIdentifier | TokenKind::FullyQualifiedIdentifier)]
    // identifier({
    //     Ok(Expression::Identifier(Identifier::SimpleIdentifier(identifiers::full_name(state))))
    // })

    // #[before(self_identifier), current(TokenKind::Static)]
    // static_postfix({
    //     state.stream.next();

    //     postfix(state, Expression::Static, &TokenKind::DoubleColon)
    // })

    // #[before(parent_identifier), current(TokenKind::Self_)]
    // self_identifier({
    //     state.stream.next();

    //     Ok(Expression::Self_)
    // })

    // #[before(left_parenthesis), current(TokenKind::Parent)]
    // parent_identifier({
    //     state.stream.next();

    //     Ok(Expression::Parent)
    // })

    // #[before(r#match), current(TokenKind::LeftParen)]
    // left_parenthesis({
    //     let start = state.stream.current().span;
    //     state.stream.next();

    //     let expr = create(state);

    //     let end = skip_right_parenthesis(state);

    //     Ok(Expression::Parenthesized(ParenthesizedExpression { start, expr: Box::new(expr), end }))
    // })

    // #[before(array), current(TokenKind::Match)]
    // r#match({
    //     control_flow::match_expression(state)
    // })

    // #[before(left_bracket), current(TokenKind::Array)]
    // array({
    //     arrays::array_expression(state)
    // })

    // #[before(new), current(TokenKind::LeftBracket)]
    // left_bracket({
    //     arrays::short_array_expression(state)
    // })

    // #[before(directory_magic_constant), current(TokenKind::New)]
    // new({
    //     let new = state.stream.current().span;

    //     state.stream.next();

    //     if state.stream.current().kind == TokenKind::Class || state.stream.current().kind == TokenKind::Attribute {
    //         return classes::parse_anonymous(state, Some(new));
    //     };

    //     let target = match state.stream.current().kind {
    //         TokenKind::Self_ => {
    //             state.stream.next();

    //             Expression::Self_
    //         }
    //         TokenKind::Static => {
    //             state.stream.next();

    //             Expression::Static
    //         }
    //         TokenKind::Parent => {
    //             state.stream.next();

    //             Expression::Parent
    //         }
    //         TokenKind::Enum => {
    //             let span = state.stream.current().span;

    //             state.stream.next();

    //             Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { span, value: "enum".into() }))
    //         }
    //         TokenKind::From => {
    //             let span = state.stream.current().span;

    //             state.stream.next();

    //             Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { span, value: "from".into() }))
    //         }
    //         _ => clone_or_new_precedence(state),
    //     };

    //     let arguments = if state.stream.current().kind == TokenKind::LeftParen {
    //         Some(parameters::argument_list(state))
    //     } else {
    //         None
    //     };

    //     Ok(Expression::New(NewExpression {
    //         target: Box::new(target),
    //         new,
    //         arguments,
    //     }))
    // })

    // #[before(file_magic_constant), current(TokenKind::DirConstant)]
    // directory_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Directory(span)))
    // })

    // #[before(line_magic_constant), current(TokenKind::FileConstant)]
    // file_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::File(span)))
    // })

    // #[before(function_magic_constant), current(TokenKind::LineConstant)]
    // line_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Line(span)))
    // })

    // #[before(class_magic_constant), current(TokenKind::FunctionConstant)]
    // function_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Function(span)))
    // })

    // #[before(method_magic_constant), current(TokenKind::ClassConstant)]
    // class_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Class(span)))
    // })

    // #[before(namespace_magic_constant), current(TokenKind::MethodConstant)]
    // method_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Method(span)))
    // })

    // #[before(trait_magic_constant), current(TokenKind::NamespaceConstant)]
    // namespace_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Namespace(span)))
    // })

    // #[before(compiler_halt_offset_magic_constant), current(TokenKind::TraitConstant)]
    // trait_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::Trait(span)))
    // })

    // #[before(include), current(TokenKind::CompilerHaltOffsetConstant)]
    // compiler_halt_offset_magic_constant({
    //     let span = state.stream.current().span;
    //     state.stream.next();

    //     Ok(Expression::MagicConstant(MagicConstantExpression::CompilerHaltOffset(span)))
    // })

    // #[before(cast_prefix), current(TokenKind::Include | TokenKind::IncludeOnce | TokenKind::Require | TokenKind::RequireOnce)]
    // include({
    //     let current = state.stream.current();
    //     let span = current.span;

    //     state.stream.next();

    //     let path = Box::new(create(state));

    //     Ok(match current.kind {
    //         TokenKind::Include => Expression::Include(IncludeExpression { include: span, path }),
    //         TokenKind::IncludeOnce => Expression::IncludeOnce(IncludeOnceExpression { include_once: span, path }),
    //         TokenKind::Require => Expression::Require(RequireExpression { require: span, path }),
    //         TokenKind::RequireOnce => Expression::RequireOnce(RequireOnceExpression { require_once: span, path }),
    //         _ => unreachable!()
    //     })
    // })

    // #[before(numeric_prefix), current(
    //     | TokenKind::StringCast     | TokenKind::BinaryCast     | TokenKind::ObjectCast
    //     | TokenKind::BoolCast       | TokenKind::BooleanCast    | TokenKind::IntCast
    //     | TokenKind::IntegerCast    | TokenKind::FloatCast      | TokenKind::DoubleCast
    //     | TokenKind::RealCast       | TokenKind::UnsetCast      | TokenKind::ArrayCast
    // )]
    // cast_prefix({
    //     let current = state.stream.current();

    //     let span = current.span;
    //     let kind = current.kind.clone().into();

    //     state.stream.next();

    //     let rhs = for_precedence(state, Precedence::Prefix);

    //     Ok(Expression::Cast(CastExpression {
    //         cast: span,
    //         kind,
    //         value: Box::new(rhs),
    //     }))
    // })

    // #[before(bang_prefix), current(TokenKind::Decrement | TokenKind::Increment | TokenKind::Minus | TokenKind::Plus)]
    // numeric_prefix({
    //     let current = state.stream.current();

    //     let span = current.span;
    //     let op = current.kind.clone();

    //     state.stream.next();

    //     let right = Box::new(for_precedence(state, Precedence::Prefix));
    //     let expr = match op {
    //         TokenKind::Minus => Expression::ArithmeticOperation(ArithmeticOperationExpression::Negative { minus: span, right }),
    //         TokenKind::Plus => Expression::ArithmeticOperation(ArithmeticOperationExpression::Positive { plus: span, right }),
    //         TokenKind::Decrement => Expression::ArithmeticOperation(ArithmeticOperationExpression::PreDecrement { decrement: span, right }),
    //         TokenKind::Increment => Expression::ArithmeticOperation(ArithmeticOperationExpression::PreIncrement { increment: span, right }),
    //         _ => unreachable!(),
    //     };

    //     Ok(expr)
    // })

    // #[before(at_prefix), current(TokenKind::Bang)]
    // bang_prefix({
    //     let bang = state.stream.current().span;

    //     state.stream.next();

    //     let rhs = for_precedence(state, Precedence::Bang);

    //     Ok(Expression::LogicalOperation(LogicalOperationExpression::Not {
    //         bang,
    //         right: Box::new(rhs)
    //     }))
    // })

    // #[before(bitwise_prefix), current(TokenKind::At)]
    // at_prefix({
    //     let span = state.stream.current().span;

    //     state.stream.next();

    //     let rhs = for_precedence(state, Precedence::Prefix);

    //     Ok(Expression::ErrorSuppress(ErrorSuppressExpression {
    //         at: span,
    //         expr: Box::new(rhs)
    //     }))
    // })

    // #[before(variable), current(TokenKind::BitwiseNot)]
    // bitwise_prefix({
    //     let span = state.stream.current().span;

    //     state.stream.next();

    //     let right = Box::new(for_precedence(state, Precedence::Prefix));

    //     Ok(Expression::BitwiseOperation(BitwiseOperationExpression::Not { not: span, right }))
    // })

    // #[before(unexpected_token), current(TokenKind::Dollar | TokenKind::DollarLeftBrace | TokenKind::Variable)]
    // variable({
    //     Ok(Expression::Variable(variables::dynamic_variable(state)))
    // })
}

fn unexpected_token(state: &mut ParserState, _: &Precedence) -> Expression {
    let current = state.stream.current();

    utils::unexpected_token(state, &["expression"]);

    Expression::missing(current.span)
}

fn postfix(state: &mut ParserState, lhs: Expression, op: &TokenKind) -> Expression {
    todo!();

    // match op {
    //     TokenKind::DoubleQuestion => {
    //         let double_question = state.stream.current().span;
    //         state.stream.next();

    //         let rhs = null_coalesce_precedence(state);

    //         Expression::Coalesce(CoalesceExpression {
    //             lhs: Box::new(lhs),
    //             double_question,
    //             rhs: Box::new(rhs),
    //         })
    //     }
    //     TokenKind::LeftParen => {
    //         // `(...)` closure creation
    //         if state.stream.lookahead(0).kind == TokenKind::Ellipsis
    //             && state.stream.lookahead(1).kind == TokenKind::RightParen
    //         {
    //             let start = skip(state, TokenKind::LeftParen);
    //             let ellipsis = skip(state, TokenKind::Ellipsis);
    //             let end = skip(state, TokenKind::RightParen);

    //             let placeholder = ArgumentPlaceholder {
    //                 comments: state.stream.comments(),
    //                 left_parenthesis: start,
    //                 ellipsis,
    //                 right_parenthesis: end,
    //             };

    //             Expression::FunctionClosureCreation(FunctionClosureCreationExpression {
    //                 target: Box::new(lhs),
    //                 placeholder,
    //             })
    //         } else {
    //             let arguments = parameters::argument_list(state);

    //             Expression::FunctionCall(FunctionCallExpression {
    //                 target: Box::new(lhs),
    //                 arguments,
    //             })
    //         }
    //     }
    //     TokenKind::LeftBracket => Expression::ArrayIndex(ArrayIndexExpression {
    //         array: Box::new(lhs),
    //         left_bracket: skip_left_bracket(state),
    //         index: if state.stream.current().kind == TokenKind::RightBracket {
    //             None
    //         } else {
    //             Some(create(state).map(Box::new))
    //         },
    //         right_bracket: skip_right_bracket(state),
    //     }),
    //     TokenKind::DoubleColon => {
    //         let span = skip_double_colon(state);

    //         let current = state.stream.current();

    //         let property = match current.kind {
    //             TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
    //                 Expression::Variable(variables::dynamic_variable(state))
    //             }
    //             _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
    //                 Expression::Identifier(Identifier::SimpleIdentifier(
    //                     identifiers::identifier_maybe_reserved(state),
    //                 ))
    //             }
    //             TokenKind::LeftBrace => {
    //                 state.stream.next();

    //                 Expression::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
    //                     start: current.span,
    //                     expr: Box::new(create(state)),
    //                     end: skip_right_brace(state),
    //                 }))
    //             }
    //             TokenKind::Class => {
    //                 state.stream.next();

    //                 Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier {
    //                     span: current.span,
    //                     value: "class".into(),
    //                 }))
    //             }
    //             _ => {
    //                 return expected_token_err!(["`{`", "`$`", "an identifier"], state);
    //             }
    //         };

    //         let lhs = Box::new(lhs);

    //         if state.stream.current().kind == TokenKind::LeftParen {
    //             if state.stream.lookahead(0).kind == TokenKind::Ellipsis
    //                 && state.stream.lookahead(1).kind == TokenKind::RightParen
    //             {
    //                 let start = skip(state, TokenKind::LeftParen);
    //                 let ellipsis = skip(state, TokenKind::Ellipsis);
    //                 let end = skip(state, TokenKind::RightParen);

    //                 let placeholder = ArgumentPlaceholder {
    //                     comments: state.stream.comments(),
    //                     left_parenthesis: start,
    //                     ellipsis,
    //                     right_parenthesis: end,
    //                 };

    //                 match property {
    //                     Expression::Identifier(identifier) => {
    //                         Expression::StaticMethodClosureCreation(
    //                             StaticMethodClosureCreationExpression {
    //                                 target: lhs,
    //                                 double_colon: span,
    //                                 method: identifier,
    //                                 placeholder,
    //                             },
    //                         )
    //                     }
    //                     Expression::Variable(variable) => {
    //                         Expression::StaticVariableMethodClosureCreation(
    //                             StaticVariableMethodClosureCreationExpression {
    //                                 target: lhs,
    //                                 double_colon: span,
    //                                 method: variable,
    //                                 placeholder,
    //                             },
    //                         )
    //                     }
    //                     _ => unreachable!(),
    //                 }
    //             } else {
    //                 let arguments = parameters::argument_list(state);

    //                 match property {
    //                     Expression::Identifier(identifier) => {
    //                         Expression::StaticMethodCall(StaticMethodCallExpression {
    //                             target: lhs,
    //                             double_colon: span,
    //                             method: identifier,
    //                             arguments,
    //                         })
    //                     }
    //                     Expression::Variable(variable) => Expression::StaticVariableMethodCall(
    //                         StaticVariableMethodCallExpression {
    //                             target: lhs,
    //                             double_colon: span,
    //                             method: variable,
    //                             arguments,
    //                         },
    //                     ),
    //                     _ => unreachable!(),
    //                 }
    //             }
    //         } else {
    //             match property {
    //                 Expression::Identifier(identifier) => {
    //                     Expression::ConstantFetch(ConstantFetchExpression {
    //                         target: lhs,
    //                         double_colon: span,
    //                         constant: identifier,
    //                     })
    //                 }
    //                 Expression::Variable(variable) => {
    //                     Expression::StaticPropertyFetch(StaticPropertyFetchExpression {
    //                         target: lhs,
    //                         double_colon: span,
    //                         property: variable,
    //                     })
    //                 }
    //                 _ => unreachable!(),
    //             }
    //         }
    //     }
    //     TokenKind::Arrow | TokenKind::QuestionArrow => {
    //         let span = state.stream.current().span;
    //         state.stream.next();

    //         let property = match state.stream.current().kind {
    //             TokenKind::Variable | TokenKind::Dollar | TokenKind::DollarLeftBrace => {
    //                 Expression::Variable(variables::dynamic_variable(state))
    //             }
    //             _ if identifiers::is_identifier_maybe_reserved(&state.stream.current().kind) => {
    //                 Expression::Identifier(Identifier::SimpleIdentifier(
    //                     identifiers::identifier_maybe_reserved(state),
    //                 ))
    //             }
    //             TokenKind::LeftBrace => {
    //                 let start = state.stream.current().span;
    //                 state.stream.next();

    //                 let name = create(state);

    //                 let end = skip_right_brace(state);

    //                 Expression::Identifier(Identifier::DynamicIdentifier(DynamicIdentifier {
    //                     start,
    //                     expr: Box::new(name),
    //                     end,
    //                 }))
    //             }
    //             _ => {
    //                 return expected_token_err!(["`{`", "`$`", "an identifier"], state);
    //             }
    //         };

    //         if state.stream.current().kind == TokenKind::LeftParen {
    //             if op == &TokenKind::QuestionArrow {
    //                 let arguments = parameters::argument_list(state);

    //                 Expression::NullsafeMethodCall(NullsafeMethodCallExpression {
    //                     target: Box::new(lhs),
    //                     method: Box::new(property),
    //                     question_arrow: span,
    //                     arguments,
    //                 })
    //             } else {
    //                 // `(...)` closure creation
    //                 if state.stream.lookahead(0).kind == TokenKind::Ellipsis
    //                     && state.stream.lookahead(1).kind == TokenKind::RightParen
    //                 {
    //                     let start = skip(state, TokenKind::LeftParen);
    //                     let ellipsis = skip(state, TokenKind::Ellipsis);
    //                     let end = skip(state, TokenKind::RightParen);

    //                     let placeholder = ArgumentPlaceholder {
    //                         comments: state.stream.comments(),
    //                         left_parenthesis: start,
    //                         ellipsis,
    //                         right_parenthesis: end,
    //                     };

    //                     Expression::MethodClosureCreation(MethodClosureCreationExpression {
    //                         target: Box::new(lhs),
    //                         method: Box::new(property),
    //                         arrow: span,
    //                         placeholder,
    //                     })
    //                 } else {
    //                     let arguments = parameters::argument_list(state);

    //                     Expression::MethodCall(MethodCallExpression {
    //                         target: Box::new(lhs),
    //                         method: Box::new(property),
    //                         arrow: span,
    //                         arguments,
    //                     })
    //                 }
    //             }
    //         } else if op == &TokenKind::QuestionArrow {
    //             Expression::NullsafePropertyFetch(NullsafePropertyFetchExpression {
    //                 target: Box::new(lhs),
    //                 question_arrow: span,
    //                 property: Box::new(property),
    //             })
    //         } else {
    //             Expression::PropertyFetch(PropertyFetchExpression {
    //                 target: Box::new(lhs),
    //                 arrow: span,
    //                 property: Box::new(property),
    //             })
    //         }
    //     }
    //     TokenKind::Increment => {
    //         let span = state.stream.current().span;
    //         state.stream.next();

    //         Expression::ArithmeticOperation(ArithmeticOperationExpression::PostIncrement {
    //             left: Box::new(lhs),
    //             increment: span,
    //         })
    //     }
    //     TokenKind::Decrement => {
    //         let span = state.stream.current().span;
    //         state.stream.next();

    //         Expression::ArithmeticOperation(ArithmeticOperationExpression::PostDecrement {
    //             left: Box::new(lhs),
    //             decrement: span,
    //         })
    //     }
    //     _ => todo!("postfix: {:}", op),
    // }
}

fn is_infix(t: &TokenKind) -> bool {
    matches!(
        t,
        TokenKind::Pow
            | TokenKind::RightShiftAssign
            | TokenKind::LeftShiftAssign
            | TokenKind::BitwiseXorAssign
            | TokenKind::BitwiseAndAssign
            | TokenKind::BitwiseOrAssign
            | TokenKind::ModuloAssign
            | TokenKind::PowAssign
            | TokenKind::LogicalAnd
            | TokenKind::LogicalOr
            | TokenKind::LogicalXor
            | TokenKind::Spaceship
            | TokenKind::LeftShift
            | TokenKind::RightShift
            | TokenKind::BitwiseAnd
            | TokenKind::BitwiseOr
            | TokenKind::BitwiseXor
            | TokenKind::Modulo
            | TokenKind::Instanceof
            | TokenKind::Multiply
            | TokenKind::Divide
            | TokenKind::Add
            | TokenKind::Subtract
            | TokenKind::Concat
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThanOrEqual
            | TokenKind::Equals
            | TokenKind::Identical
            | TokenKind::NotEqual
            | TokenKind::NotIdentical
            | TokenKind::Question
            | TokenKind::QuestionColon
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Assign
            | TokenKind::AddAssign
            | TokenKind::SubtractAssign
            | TokenKind::ConcatAssign
            | TokenKind::NullCoalesceAssign
            | TokenKind::MultiplyAssign
            | TokenKind::DivideAssign
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
            | TokenKind::NullsafeArrow
            | TokenKind::DoubleColon
            | TokenKind::NullCoalesce
    )
}

pub enum Associativity {
    Non,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Precedence {
    Lowest,
    Print,
    Yield,
    YieldFrom,
    IncDec,
    KeyOr,
    KeyXor,
    KeyAnd,
    Assignment,
    Ternary,
    NullCoalesce,
    Or,
    And,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equality,
    LtGt,
    Concat,
    BitShift,
    AddSub,
    MulDivMod,
    Bang,
    Instanceof,
    Prefix,
    Pow,
    CallDim,
    ObjectAccess,
    CloneOrNew,
}

impl Precedence {
    pub fn infix(kind: &TokenKind) -> Self {
        use TokenKind::*;

        match kind {
            Pow => Self::Pow,
            Instanceof => Self::Instanceof,
            Multiply | Divide | Modulo => Self::MulDivMod,
            Add | Subtract => Self::AddSub,
            LeftShift | RightShift => Self::BitShift,
            Concat => Self::Concat,
            LessThan | LessThanOrEqual | GreaterThan | GreaterThanOrEqual => Self::LtGt,
            Equals | NotEqual | Identical | NotIdentical
            | Spaceship => Self::Equality,
            BitwiseAnd => Self::BitwiseAnd,
            BitwiseXor => Self::BitwiseXor,
            BitwiseOr => Self::BitwiseOr,
            And => Self::And,
            Or => Self::Or,
            NullCoalesce => Self::NullCoalesce,
            Question | QuestionColon => Self::Ternary,
            Assign | AddAssign | SubtractAssign | MultiplyAssign | PowAssign | DivideAssign
            | ConcatAssign | BitwiseAndAssign | NullCoalesceAssign | ModuloAssign
            | BitwiseOrAssign | BitwiseXorAssign | LeftShiftAssign | RightShiftAssign => Self::Assignment,
            Yield => Self::Yield,
            LogicalAnd => Self::KeyAnd,
            LogicalOr => Self::KeyOr,
            LogicalXor => Self::KeyXor,
            _ => unimplemented!("precedence for op {:}", kind),
        }
    }

    pub fn postfix(kind: &TokenKind) -> Self {
        use TokenKind::*;

        match kind {
            DoubleQuestion => Self::NullCoalesce,
            Increment | Decrement => Self::IncDec,
            LeftParen | LeftBracket => Self::CallDim,
            Arrow | NullsafeArrow | DoubleColon => Self::ObjectAccess,
            _ => unimplemented!("postfix precedence for op {:}", kind),
        }
    }

    pub fn associativity(&self) -> Option<Associativity> {
        Some(match self {
            Self::Instanceof
            | Self::MulDivMod
            | Self::AddSub
            | Self::BitShift
            | Self::Concat
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::And
            | Self::Or
            | Self::KeyAnd
            | Self::KeyOr
            | Self::KeyXor => Associativity::Left,
            Self::Pow | Self::NullCoalesce | Self::Assignment => Associativity::Right,
            Self::Ternary | Self::Equality | Self::LtGt => Associativity::Non,
            _ => return None,
        })
    }
}
