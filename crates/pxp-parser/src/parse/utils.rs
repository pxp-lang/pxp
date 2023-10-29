use pxp_token::{TokenKind, Token};

use crate::{state::ParserState, result::{ParseError, ParseResult}};

pub fn skip(state: &mut ParserState, kind: TokenKind) -> Token {
    let current = state.stream.current();

    if current.kind == kind {
        state.stream.next();

        current.clone()
    } else {
        unexpected_token(state, kind);

        Token::missing(current.span.with_start_as_end())
    }
}

pub fn unexpected_token(state: &mut ParserState, kind: TokenKind) {
    state.errors.push(ParseError::UnexpectedToken {
        token: state.stream.current().clone(),
        expected: vec![kind.to_string()],
    });
}

pub fn skip_semicolon(state: &mut ParserState) -> Token {
    let current = state.stream.current();

    if current.kind == TokenKind::SemiColon {
        state.stream.next();
        current.clone()
    } else {
        unexpected_token(state, TokenKind::SemiColon);
        
        Token::missing(current.span.with_start_as_end())
    }
}

pub fn skip_left_brace(state: &mut ParserState) -> Token {
    skip(state, TokenKind::LeftBrace)
}

pub fn skip_right_brace(state: &mut ParserState) -> Token {
    skip(state, TokenKind::RightBrace)
}