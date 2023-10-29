use pxp_ast::{Statement, SimpleIdentifier, Block, StatementKind, NamespaceStatement, UnbracedNamespace, BracedNamespace};
use pxp_span::Span;
use pxp_token::{TokenKind, Token};

use crate::{state::ParserState, parse::{utils::skip, optional_name, top_level_statement}, result::ParseError};

use super::utils::{skip_semicolon, skip_left_brace, skip_right_brace};

pub fn namespace(state: &mut ParserState) -> Statement {
    let namespace = skip(state, TokenKind::Namespace);
    let name = optional_name(state);
    let current = state.stream.current();
    
    match current.kind {
        // 1. If the current token is a semi-colon, then:
        TokenKind::SemiColon => match name {
            // a) if we have a name, parse an unbraced namespace with the given name.
            Some(name) => {
                unbraced_namespace(state, namespace.span, name)
            },
            // b) otherwise, parse an unbraced namespace with a missing name and mark as error.
            None => {
                state.errors.push(ParseError::UnbracedNamespaceWithoutName { span: current.span.with_start_as_end() });
                unbraced_namespace(state, namespace.span, SimpleIdentifier::missing(current.span.with_start_as_end()))
            },
        },
        // 2. If the current token is a left brace, then parse a braced namespace.
        TokenKind::LeftBrace => todo!(),
        // 3. Check if we have a name, then:
        _ => match name {
            // a) if we have a name, parse an unbraced namespace with the given name and error on missing semi-colon.
            Some(name) => {
                skip_semicolon(state);
                unbraced_namespace(state, namespace.span, name)
            },
            // b) parse an unbraced namespace without a name.
            None => {
                braced_namespace(state, namespace.span, None)
            },
        },
    }
}

fn unbraced_namespace(
    state: &mut ParserState,
    start: Span,
    name: SimpleIdentifier,
) -> Statement {
    // TODO: Store the semi-colon in the AST.
    let semi_colon = skip_semicolon(state);
    let mut body = Block::new();

    while !state.stream.is_eof() {
        body.push(top_level_statement(state));
    }

    let span = Span::new(start.start, state.stream.previous().span.end);

    Statement::new(
        StatementKind::Namespace(NamespaceStatement {
            kind: pxp_ast::NamespaceKind::Unbraced(UnbracedNamespace { name, body }),
            span,
        }),
        span
    )
}

fn braced_namespace(
    state: &mut ParserState,
    start: Span,
    name: Option<SimpleIdentifier>,
) -> Statement {
    // FIXME: Store left brace in the AST.
    let left_brace = skip_left_brace(state);
    let mut body = Block::new();

    while !state.stream.is_eof() {
        body.push(top_level_statement(state));
    }

    // FIXME: Store right brace in the AST.
    let right_brace = skip_right_brace(state);
    let span = Span::new(start.start, right_brace.span.end);

    Statement::new(
        StatementKind::Namespace(NamespaceStatement {
            kind: pxp_ast::NamespaceKind::Braced(BracedNamespace { name, body }),
            span
        }),
        span
    )
}