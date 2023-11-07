use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::ast::utils::CommaSeparated;
use crate::parser::ast::Ending;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::state::State;

pub fn skip_ending(state: &mut State) -> ParseResult<Ending> {
    let current = state.stream.current();

    if current.kind == TokenKind::CloseTag {
        state.stream.next();

        Ok(Ending::CloseTag(current.span))
    } else if current.kind == TokenKind::SemiColon {
        state.stream.next();

        Ok(Ending::Semicolon(current.span))
    } else {
        Err(error::unexpected_token(vec![";".to_string()], current))
    }
}

pub fn skip_semicolon(state: &mut State) -> ParseResult<Span> {
    let current = state.stream.current();

    if current.kind == TokenKind::SemiColon {
        state.stream.next();

        Ok(current.span)
    } else {
        Err(error::unexpected_token(vec!["`;`".to_string()], current))
    }
}

pub fn skip_left_brace(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftBrace)
}

pub fn skip_right_brace(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightBrace)
}

pub fn skip_left_parenthesis(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftParen)
}

pub fn skip_right_parenthesis(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightParen)
}

pub fn skip_left_bracket(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftBracket)
}

pub fn skip_right_bracket(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightBracket)
}

pub fn skip_double_arrow(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::DoubleArrow)
}

pub fn skip_double_colon(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::DoubleColon)
}

pub fn skip_colon(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::Colon)
}

pub fn skip(state: &mut State, kind: TokenKind) -> ParseResult<Span> {
    let current = state.stream.current();

    if current.kind == kind {
        let end = current.span;

        state.stream.next();

        Ok(end)
    } else {
        Err(error::unexpected_token(vec![kind.to_string()], current))
    }
}

pub fn skip_any_of(state: &mut State, kinds: &[TokenKind]) -> ParseResult<Span> {
    let current = state.stream.current();

    if kinds.contains(&current.kind) {
        let end = current.span;

        state.stream.next();

        Ok(end)
    } else {
        Err(error::unexpected_token(
            kinds.iter().map(|kind| kind.to_string()).collect(),
            current,
        ))
    }
}

/// Parse an item that is surrounded by parentheses.
///
/// This function will skip the left parenthesis, call the given function,
/// and then skip the right parenthesis.
pub fn parenthesized<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
) -> ParseResult<(Span, T, Span)> {
    let left_parenthesis = skip_left_parenthesis(state)?;
    let inner = func(state)?;
    let right_parenthesis = skip_right_parenthesis(state)?;

    Ok((left_parenthesis, inner, right_parenthesis))
}

/// Parse an item that is surrounded by braces.
///
/// This function will skip the left brace, call the given function,
/// and then skip the right brace.
pub fn braced<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
) -> ParseResult<(Span, T, Span)> {
    let left_brace = skip_left_brace(state)?;
    let inner = func(state)?;
    let right_brace = skip_right_brace(state)?;

    Ok((left_brace, inner, right_brace))
}

pub fn semicolon_terminated<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
) -> ParseResult<(Span, T)> {
    let inner = func(state)?;
    let semicolon = skip_semicolon(state)?;
    Ok((semicolon, inner))
}

/// Parse a comma-separated list of items, allowing a trailing comma.
pub fn comma_separated<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
    until: TokenKind,
) -> ParseResult<CommaSeparated<T>> {
    let mut inner: Vec<T> = vec![];
    let mut commas: Vec<Span> = vec![];
    let mut current = state.stream.current();

    while current.kind != until {
        inner.push(func(state)?);

        current = state.stream.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.span);

        state.stream.next();

        current = state.stream.current();
    }

    Ok(CommaSeparated { inner, commas })
}

/// Parse a comma-separated list of items, not allowing trailing commas.
pub fn comma_separated_no_trailing<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
    until: TokenKind,
) -> ParseResult<CommaSeparated<T>> {
    let mut inner: Vec<T> = vec![];
    let mut commas: Vec<Span> = vec![];
    let mut current = state.stream.current();

    while current.kind != until {
        inner.push(func(state)?);

        current = state.stream.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        // If the next token is the until token, we don't want to consume the comma.
        // This ensures that trailing commas are not allowed.
        if state.stream.peek().kind == until {
            break;
        }

        commas.push(current.span);

        state.stream.next();

        current = state.stream.current();
    }

    Ok(CommaSeparated { inner, commas })
}

/// Parse a comma-separated list of items, requiring at least one item, and not allowing trailing commas.
pub fn at_least_one_comma_separated_no_trailing<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
) -> ParseResult<CommaSeparated<T>> {
    let mut inner: Vec<T> = vec![];
    let mut commas: Vec<Span> = vec![];

    loop {
        inner.push(func(state)?);

        let current = state.stream.current();
        if current.kind != TokenKind::Comma {
            break;
        }

        commas.push(current.span);

        state.stream.next();
    }

    Ok(CommaSeparated { inner, commas })
}
