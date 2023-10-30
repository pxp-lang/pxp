use pxp_ast::SimpleIdentifier;
use pxp_token::{TokenKind, Token};

use crate::{state::ParserState, result::{ParseError, ParseResult}};

use super::is_reserved_identifier;

pub fn skip(state: &mut ParserState, kind: TokenKind) -> Token {
    let current = state.stream.current();

    if current.kind == kind {
        state.stream.next();

        current.clone()
    } else {
        unexpected_token(state, &[kind]);

        Token::missing(current.span.with_start_as_end())
    }
}

pub fn unexpected_token(state: &mut ParserState, kinds: &[TokenKind]) {
    state.errors.push(ParseError::UnexpectedToken {
        token: state.stream.current().clone(),
        expected: kinds.iter().map(|k| k.to_string()).collect::<Vec<String>>(),
    });
}

pub fn skip_semicolon(state: &mut ParserState) -> Token {
    let current = state.stream.current();

    if current.kind == TokenKind::SemiColon {
        state.stream.next();
        current.clone()
    } else {
        unexpected_token(state, &[TokenKind::SemiColon]);
        
        Token::missing(current.span.with_start_as_end())
    }
}

pub fn skip_left_brace(state: &mut ParserState) -> Token {
    skip(state, TokenKind::LeftBrace)
}

pub fn skip_right_brace(state: &mut ParserState) -> Token {
    skip(state, TokenKind::RightBrace)
}

pub fn full_type_name(state: &mut ParserState) -> SimpleIdentifier {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Identifier
        | TokenKind::QualifiedIdentifier
        | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;

            state.stream.next();

            SimpleIdentifier {
                span,
                value: current.clone()
            }
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;
            let token = Token::new(TokenKind::Identifier, span, current.literal().clone());

            state.stream.next();

            SimpleIdentifier { span, value: token }
        }
        TokenKind::Static => {
            let span = current.span;
            let token = Token::new(TokenKind::Identifier, span, current.literal().clone());

            state.stream.next();

            SimpleIdentifier { span, value: token }
        }
        t if is_reserved_identifier(t) => {
            state.errors.push(ParseError::ReservedKeywordInTypeName {
                span: current.span,
                token: current.clone(),
            });

            let span = current.span;

            state.stream.next();

            SimpleIdentifier { span, value: current.clone() }
        }
        _ => {
            unexpected_token(state, &[TokenKind::Identifier]);
            SimpleIdentifier::missing(current.span.with_start_as_end())
        },
    }
}

pub fn type_identifier(state: &mut ParserState) -> SimpleIdentifier {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Identifier => {
            let span = current.span;

            state.stream.next();

            SimpleIdentifier {
                span,
                value: current.clone(),
            }
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;
            let token = Token::new(TokenKind::Identifier, span, current.literal().clone());

            state.stream.next();

            SimpleIdentifier { span, value: token }
        }
        TokenKind::Static => {
            let span = current.span;
            let token = Token::new(TokenKind::Identifier, span, current.literal().clone());

            state.stream.next();

            SimpleIdentifier { span, value: token }
        }
        t if is_reserved_identifier(t) => {
            state.errors.push(ParseError::ReservedKeywordInTypeName {
                span: current.span,
                token: current.clone(),
            });

            let span = current.span;

            state.stream.next();

            SimpleIdentifier { span, value: current.clone() }
        }
        _ => {
            unexpected_token(state, &[TokenKind::Identifier]);
            SimpleIdentifier::missing(current.span.with_start_as_end())
        },
    }
}