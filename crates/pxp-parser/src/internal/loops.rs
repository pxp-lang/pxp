use crate::expressions;
use crate::internal::blocks;
use crate::internal::utils;
use crate::state::State;
use crate::statement;
use pxp_ast::StatementKind;
use pxp_ast::*;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::Token;
use pxp_token::TokenKind;

pub fn foreach_statement(state: &mut State) -> StatementKind {
    let foreach = utils::skip(state, TokenKind::Foreach);

    let (left_parenthesis, iterator, right_parenthesis) =
        utils::parenthesized(state, &|state: &mut State| {
            let expression = expressions::create(state);

            let r#as = utils::skip(state, TokenKind::As);

            let current = state.stream.current();
            let ampersand = if current.kind == TokenKind::Ampersand {
                state.stream.next();
                Some(current.span)
            } else {
                None
            };

            let mut value = expressions::create(state);

            let current = state.stream.current();
            if current.kind == TokenKind::DoubleArrow {
                state.stream.next();
                let arrow = current.span;

                let current = state.stream.current();
                let ampersand = if current.kind == TokenKind::Ampersand {
                    state.stream.next();
                    Some(current.span)
                } else {
                    None
                };

                let mut key = expressions::create(state);

                std::mem::swap(&mut value, &mut key);

                ForeachStatementIterator::KeyAndValue(ForeachStatementIteratorKeyAndValue {
                    id: state.id(),
                    span: Span::combine(expression.span, value.span),
                    expression,
                    r#as,
                    key,
                    double_arrow: arrow,
                    ampersand,
                    value,
                })
            } else {
                ForeachStatementIterator::Value(ForeachStatementIteratorValue {
                    id: state.id(),
                    span: Span::combine(expression.span, value.span),
                    expression,
                    r#as,
                    ampersand,
                    value,
                })
            }
        });

    let body = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let statements = blocks::multiple_statements_until(state, &TokenKind::EndForeach);
        let endforeach = utils::skip(state, TokenKind::EndForeach);
        let ending = utils::skip_ending(state);

        ForeachStatementBody::Block(ForeachStatementBodyBlock {
            id: state.id(),
            span: Span::combine(colon, ending.span()),
            colon,
            statements,
            endforeach,
            ending,
        })
    } else {
        let statement = statement(state);

        ForeachStatementBody::Statement(ForeachStatementBodyStatement {
            id: state.id(),
            span: statement.span,
            statement: Box::new(statement),
        })
    };

    StatementKind::Foreach(ForeachStatement {
        id: state.id(),
        span: Span::combine(foreach, body.span()),
        foreach,
        left_parenthesis,
        iterator,
        right_parenthesis,
        body,
    })
}

pub fn for_statement(state: &mut State) -> StatementKind {
    let r#for = utils::skip(state, TokenKind::For);

    let (left_parenthesis, iterator, right_parenthesis) = utils::parenthesized(state, &|state| {
        let (initializations_semicolon, initializations) =
            utils::semicolon_terminated(state, &|state| {
                utils::comma_separated_no_trailing(
                    state,
                    &expressions::create,
                    TokenKind::SemiColon,
                )
            });

        let (conditions_semicolon, conditions) = utils::semicolon_terminated(state, &|state| {
            utils::comma_separated_no_trailing(state, &expressions::create, TokenKind::SemiColon)
        });

        let r#loop =
            utils::comma_separated_no_trailing(state, &expressions::create, TokenKind::RightParen);

        ForStatementIterator {
            id: state.id(),
            span: Span::combine(initializations.span(), r#loop.span()),
            initializations,
            initializations_semicolon,
            conditions,
            conditions_semicolon,
            r#loop,
        }
    });

    let body = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let statements = blocks::multiple_statements_until(state, &TokenKind::EndFor);
        let endfor = utils::skip(state, TokenKind::EndFor);
        let ending = utils::skip_ending(state);

        ForStatementBody::Block(ForStatementBodyBlock {
            id: state.id(),
            span: Span::combine(colon, ending.span()),
            colon,
            statements,
            endfor,
            ending,
        })
    } else {
        let x = statement(state);

        ForStatementBody::Statement(ForStatementBodyStatement {
            id: state.id(),
            span: x.span,
            statement: Box::new(x),
        })
    };

    StatementKind::For(ForStatement {
        id: state.id(),
        span: Span::combine(r#for, body.span()),
        r#for,
        left_parenthesis,
        iterator,
        right_parenthesis,
        body,
    })
}

pub fn do_while_statement(state: &mut State) -> StatementKind {
    let r#do = utils::skip(state, TokenKind::Do);

    let body = Box::new(statement(state));

    let r#while = utils::skip(state, TokenKind::While);

    let (semicolon, (left_parenthesis, condition, right_parenthesis)) =
        utils::semicolon_terminated(state, &|state| {
            utils::parenthesized(state, &expressions::create)
        });

    StatementKind::DoWhile(DoWhileStatement {
        id: state.id(),
        span: Span::combine(r#do, right_parenthesis),
        r#do,
        body,
        r#while,
        left_parenthesis,
        condition,
        right_parenthesis,
        semicolon,
    })
}

pub fn while_statement(state: &mut State) -> StatementKind {
    let r#while = utils::skip(state, TokenKind::While);

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create);

    let body = if state.stream.current().kind == TokenKind::Colon {
        let colon = utils::skip_colon(state);
        let statements = blocks::multiple_statements_until(state, &TokenKind::EndWhile);
        let endwhile = utils::skip(state, TokenKind::EndWhile);
        let ending = utils::skip_ending(state);

        WhileStatementBody::Block(WhileStatementBodyBlock {
            id: state.id(),
            span: Span::combine(colon, ending.span()),
            colon,
            statements,
            endwhile,
            ending,
        })
    } else {
        let x = statement(state);

        WhileStatementBody::Statement(WhileStatementBodyStatement {
            id: state.id(),
            span: x.span,
            statement: Box::new(x),
        })
    };

    StatementKind::While(WhileStatement {
        id: state.id(),
        span: Span::combine(r#while, body.span()),
        r#while,
        left_parenthesis,
        condition,
        right_parenthesis,
        body,
    })
}

pub fn continue_statement(state: &mut State) -> StatementKind {
    let r#continue = utils::skip(state, TokenKind::Continue);
    let level = maybe_loop_level(state);
    let ending = utils::skip_ending(state);

    StatementKind::Continue(ContinueStatement {
        id: state.id(),
        span: Span::combine(r#continue, ending.span()),
        r#continue,
        level,
        ending,
    })
}

pub fn break_statement(state: &mut State) -> StatementKind {
    let r#break = utils::skip(state, TokenKind::Break);
    let level = maybe_loop_level(state);
    let ending = utils::skip_ending(state);

    StatementKind::Break(BreakStatement {
        id: state.id(),
        span: Span::combine(r#break, ending.span()),
        r#break,
        level,
        ending,
    })
}

fn maybe_loop_level(state: &mut State) -> Option<Level> {
    let current = &state.stream.current().kind;

    if current == &TokenKind::SemiColon || current == &TokenKind::CloseTag {
        None
    } else {
        Some(loop_level(state))
    }
}

fn loop_level(state: &mut State) -> Level {
    let current = state.stream.current();

    if let Token {
        kind: TokenKind::LiteralInteger,
        ..
    } = current
    {
        state.stream.next();

        return Level::Literal(LiteralLevel {
            id: state.id(),
            literal: Literal::new(state.id(), LiteralKind::Integer, *current, current.span),
        });
    }

    let (left_parenthesis, level, right_parenthesis) =
        utils::parenthesized(state, &|state| Box::new(loop_level(state)));

    Level::Parenthesized(ParenthesizedLevel {
        id: state.id(),
        span: Span::combine(left_parenthesis, right_parenthesis),
        left_parenthesis,
        level,
        right_parenthesis,
    })
}
