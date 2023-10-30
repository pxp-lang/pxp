use pxp_ast::{Statement, UseKind, StatementKind, UseStatement, Use, SimpleIdentifier, GroupUse};
use pxp_span::Span;
use pxp_token::{TokenKind, Token};

use crate::{state::ParserState, parse::utils::{unexpected_token, skip, skip_right_brace}, result::ParseError};

use super::utils::{full_type_name, type_identifier, skip_semicolon};

pub fn r#use(state: &mut ParserState) -> Statement {
    let r#use = skip(state, TokenKind::Use);

    let kind = match state.stream.current().kind {
        TokenKind::Function => {
            state.stream.next();
            UseKind::Function
        },
        TokenKind::Const => {
            state.stream.next();
            UseKind::Const
        },
        _ => UseKind::Normal,
    };

    // 1. If we encounter a semi-colon right here, we can take a fast path for the tolerance handling, since we know we're looking for an identifier.
    //    This will have a good performance improvement when we need to parse invalid use statements inside of the language server.
    let current = state.stream.current();
    if current.kind == TokenKind::SemiColon {
        unexpected_token(state, &[TokenKind::Identifier]);
        state.stream.next();

        return Statement::new(
            StatementKind::Use(UseStatement { kind, uses: vec![
                Use { name: SimpleIdentifier::missing(current.span.with_start_as_end()), alias: None }
            ] }),
            Span::new(r#use.span.start, current.span.end),
        )
    }

    let peek = state.stream.peek();
    let peek_1 = state.stream.lookahead(1);

    // 2. If we see a left brace or a namespace separator, we're going to try and parse a group use statement
    //    as long as the token after that is not a semi-colon.
    //
    // e.g.
    // valid:   use Foo\{
    // valid:   use Foo\Bar\{
    // invalid: use Foo\;
    //     - this would be a partial single use statement.
    // invalid: use Foo\
    //     - this would be an unterminated single use statement.
    if (peek.kind == TokenKind::NamespaceSeparator && peek_1.kind == TokenKind::LeftBrace) || peek.kind == TokenKind::LeftBrace {
        return group_use(state, kind, r#use);
    }

    // 3. Otherwise, we're going to try and parse a regular use statement.
    let mut uses = Vec::new();

    if state.stream.is_eof() {
        state.errors.push(ParseError::UnexpectedEndOfFile { span: current.span });
    }

    while !state.stream.is_eof() {
        let name = full_type_name(state);

        // If we can see a NamespaceSeparator here, we want to patch up the identifier since it represents a partial identifier
        // but we also want to report the error, as we're really expecting a semi-colon.
        if state.stream.current().kind == TokenKind::NamespaceSeparator {
            if state.stream.peek().kind == TokenKind::SemiColon {
                state.stream.next();
                unexpected_token(state, &[TokenKind::LeftBrace]);
            } else {
                state.stream.next();
                unexpected_token(state, &[TokenKind::SemiColon, TokenKind::Identifier, TokenKind::As]);
            }
        }

        let mut alias = None;

        if state.stream.current().kind == TokenKind::As {
            state.stream.next();

            // This will allow Foo as _ to be parsed as a valid use statement with a missing alias.
            alias = Some(type_identifier(state));
        }
        
        uses.push(Use { name, alias });

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
            continue;
        }

        skip_semicolon(state);
        break;
    }

    let span = Span::new(r#use.span.start, state.stream.previous().span.end);

    Statement::new(
        StatementKind::Use(UseStatement {
            kind,
            uses
        }),
        span
    )
}

fn group_use(state: &mut ParserState, kind: UseKind, start: Token) -> Statement {
    let prefix = full_type_name(state);
    let namespace_separator = skip(state, TokenKind::NamespaceSeparator);
    let left_brace = skip(state, TokenKind::LeftBrace);

    let mut uses = Vec::new();

    while state.stream.current().kind != TokenKind::RightBrace {
        let use_kind = match state.stream.current().kind {
            TokenKind::Function => {
                if kind != UseKind::Normal {
                    unexpected_token(state, &[TokenKind::Identifier]);
                }

                state.stream.next();
                Some(UseKind::Function)
            },
            TokenKind::Const => {
                if kind != UseKind::Normal {
                    unexpected_token(state, &[TokenKind::Identifier]);
                }

                state.stream.next();
                Some(UseKind::Const)
            },
            _ => None,
        };

        let name = full_type_name(state);

        // This would be a partially completed name.
        if state.stream.current().kind == TokenKind::NamespaceSeparator {
            unexpected_token(state, &[TokenKind::Comma, TokenKind::RightBrace]);
            state.stream.next();
        }

        let mut alias = None;

        if state.stream.current().kind == TokenKind::As {
            state.stream.next();
            alias = Some(type_identifier(state));
        }

        uses.push(GroupUse { name, alias, kind: use_kind });

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
            continue;
        }
    }

    if uses.is_empty() && state.stream.current().kind == TokenKind::RightBrace {
        unexpected_token(state, &[TokenKind::Identifier, TokenKind::QualifiedIdentifier, TokenKind::Function, TokenKind::Const])
    }
    
    skip_right_brace(state);
    skip_semicolon(state);

    let span = Span::new(start.span.start, state.stream.previous().span.end);

    Statement::new(
        StatementKind::GroupUse(pxp_ast::GroupUseStatement { prefix, kind, uses }),
        span
    )
}