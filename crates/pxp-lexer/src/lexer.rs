use pxp_bytestring::ByteString;
use pxp_source::{SourceFile, Source};
use pxp_span::Span;
use pxp_token::{Token, TokenKind, DocStringIndentationKind, DocStringKind};

use crate::{LexerResult, state::{StateMachine, State}, LexerError, ident_start, ident};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash, Copy)]
pub struct Lexer;

impl Lexer {
    pub const fn new() -> Self {
        Self
    }

    fn initial(&self, state: &mut StateMachine, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let start_position = state.source_mut().position();
        let mut buffer = Vec::new();

        while let Some(char) = state.source_mut().current() {
            if state.source_mut().matches_n(b"<?php", 5) {
                let tag_position = state.source_mut().position();
                let tag = state.source_mut().read_and_skip_n(5);

                state.replace(State::Scripting);

                if !buffer.is_empty() {
                    tokens.push(Token::new(
                        TokenKind::InlineHtml,
                        (start_position, tag_position - 1).into(),
                        buffer.into(),
                    ));
                }

                tokens.push(Token::new(
                    TokenKind::FullOpenTag,
                    (tag_position, tag_position + 4).into(),
                    ByteString::from(tag),
                ));

                return Ok(())
            }

            state.source_mut().next();
            buffer.push(*char);
        }

        tokens.push(Token::new(
            TokenKind::InlineHtml,
            (start_position, state.source_mut().position()).into(),
            buffer.into(),
        ));

        Ok(())
    }

    fn scripting(&self, state: &mut StateMachine) -> LexerResult<Token> {
        let start_position = state.source_mut().position();

        if state.source_mut().is_eof() {
            return Err(LexerError::UnexpectedEndOfFile(state.source_mut().position()));
        }

        let (kind, value): (TokenKind, ByteString) = match state.source_mut().read_n(3) {
            [b'!', b'=', b'='] => {
                state.source_mut().skip_n(3);

                (TokenKind::NotIdentical, b"!==".into())
            }
            [b'?', b'?', b'='] => {
                state.source_mut().skip_n(3);
                (TokenKind::NullCoalesceAssign, b"??=".into())
            }
            [b'?', b'-', b'>'] => {
                state.source_mut().skip_n(3);
                (TokenKind::NullsafeArrow, b"?->".into())
            }
            [b'=', b'=', b'='] => {
                state.source_mut().skip_n(3);
                (TokenKind::Identical, b"===".into())
            }
            [b'.', b'.', b'.'] => {
                state.source_mut().skip_n(3);
                (TokenKind::Ellipsis, b"...".into())
            }
            [b'`', ..] => {
                state.source_mut().next();
                state.replace(State::ShellExec);
                (TokenKind::Backtick, b"`".into())
            }
            [b'@', ..] => {
                state.source_mut().next();
                (TokenKind::ErrorControl, b"@".into())
            }
            [b'!', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::NotEqual, b"!=".into())
            }
            [b'!', ..] => {
                state.source_mut().next();
                (TokenKind::Not, b"!".into())
            }
            [b'&', b'&', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::And, b"&&".into())
            }
            [b'&', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::BitwiseAndAssign, b"&=".into())
            }
            [b'&', ..] => {
                state.source_mut().next();
                (TokenKind::BitwiseAnd, b"&".into())
            }
            [b'?', b'>', ..] => {
                // This is a close tag, we can enter "Initial" mode again.
                state.source_mut().skip_n(2);

                state.replace(State::Initial);

                (TokenKind::CloseTag, b"?>".into())
            }
            [b'?', b'?', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::NullCoalesce, b"??".into())
            }
            [b'?', b':', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::QuestionColon, b"?:".into())
            }
            [b'?', ..] => {
                state.source_mut().next();
                (TokenKind::Question, b"?".into())
            }
            [b'=', b'>', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::DoubleArrow, b"=>".into())
            }
            [b'=', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Equals, b"==".into())
            }
            [b'=', ..] => {
                state.source_mut().next();
                (TokenKind::Assign, b"=".into())
            }
            // Single quoted string.
            [b'\'', ..] => {
                state.source_mut().skip_n(1);
                self.tokenize_single_quote_string(state)?
            }
            [b'b' | b'B', b'\'', ..] => {
                state.source_mut().skip_n(2);
                self.tokenize_single_quote_string(state)?
            }
            [b'"', ..] => {
                state.source_mut().skip_n(1);
                self.tokenize_double_quote_string(state)?
            }
            [b'b' | b'B', b'"', ..] => {
                state.source_mut().skip_n(2);
                self.tokenize_double_quote_string(state)?
            }
            [b'$', ident_start!(), ..] => self.tokenize_variable(state),
            [b'$', ..] => {
                state.source_mut().next();
                (TokenKind::Dollar, b"$".into())
            }
            [b'.', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::ConcatAssign, b".=".into())
            }
            [b'0'..=b'9', ..] => self.tokenize_number(state)?,
            [b'.', b'0'..=b'9', ..] => self.tokenize_number(state)?,
            [b'.', ..] => {
                state.source_mut().next();
                (TokenKind::Concat, b".".into())
            }
            [b'\\', ident_start!(), ..] => {
                state.source_mut().next();

                match self.scripting(state)? {
                    Token {
                        kind: TokenKind::Identifier | TokenKind::QualifiedIdentifier,
                        literal,
                        ..
                    } => {
                        let mut bytes = literal;
                        bytes.insert(0, b'\\');

                        (TokenKind::FullyQualifiedIdentifier, bytes)
                    }
                    Token {
                        kind: TokenKind::True,
                        ..
                    } => (TokenKind::FullyQualifiedIdentifier, b"\\true".into()),
                    Token {
                        kind: TokenKind::False,
                        ..
                    } => (TokenKind::FullyQualifiedIdentifier, b"\\false".into()),
                    Token {
                        kind: TokenKind::Null,
                        ..
                    } => (TokenKind::FullyQualifiedIdentifier, b"\\null".into()),
                    s => unreachable!("{:?}", s),
                }
            }
            [b'\\', ..] => {
                state.source_mut().next();
                (TokenKind::NamespaceSeparator, b"\\".into())
            }
            [b'/', b'*', ..] => {
                state.source_mut().next();
                let mut buffer = vec![b'/'];

                loop {
                    match state.source_mut().read_n(2) {
                        [b'*', b'/'] => {
                            state.source_mut().skip_n(2);
                            buffer.extend_from_slice(b"*/");
                            break;
                        }
                        &[t, ..] => {
                            state.source_mut().next();
                            buffer.push(t);
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if buffer.starts_with(b"/**") {
                    (TokenKind::DocBlockComment, buffer.into())
                } else {
                    (TokenKind::BlockComment, buffer.into())
                }
            }
            [b'#', b'[', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Attribute, b"#[".into())
            }
            [ch @ b'/', b'/', ..] | [ch @ b'#', ..] => {
                let mut buffer = if *ch == b'/' {
                    state.source_mut().skip_n(2);
                    b"//".to_vec()
                } else {
                    state.source_mut().next();
                    b"#".to_vec()
                };

                while let Some(c) = state.source_mut().current() {
                    if *c == b'\n' {
                        state.source_mut().next();
                        break;
                    }

                    if state.source_mut().read_n(2) == [b'?', b'>'] {
                        break;
                    }

                    buffer.push(*c);
                    state.source_mut().next();
                }

                if buffer.starts_with(b"#") {
                    (TokenKind::HashComment, buffer.into())
                } else {
                    (TokenKind::SlashComment, buffer.into())
                }
            }
            [b'/', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::DivideAssign, b"/=".into())
            }
            [b'/', ..] => {
                state.source_mut().next();
                (TokenKind::Divide, b"/".into())
            }
            [b'*', b'*', b'=', ..] => {
                state.source_mut().skip_n(3);
                (TokenKind::PowAssign, b"**=".into())
            }
            [b'<', b'<', b'='] => {
                state.source_mut().skip_n(3);

                (TokenKind::LeftShiftAssign, b"<<=".into())
            }
            [b'<', b'=', b'>'] => {
                state.source_mut().skip_n(3);
                (TokenKind::Spaceship, b"<=>".into())
            }
            [b'>', b'>', b'='] => {
                state.source_mut().skip_n(3);
                (TokenKind::RightShiftAssign, b">>=".into())
            }
            [b'<', b'<', b'<'] => {
                state.source_mut().skip_n(3);
                let mut buffer = b"<<<".to_vec();
                buffer.extend(self.read_and_skip_whitespace(state));

                let doc_string_kind = match state.source_mut().read_n(1) {
                    [b'\''] => {
                        buffer.push(b'\'');
                        state.source_mut().next();
                        DocStringKind::Nowdoc
                    }
                    [b'"'] => {
                        buffer.push(b'"');
                        state.source_mut().next();
                        DocStringKind::Heredoc
                    }
                    [_, ..] => DocStringKind::Heredoc,
                    [] => {
                        return Err(LexerError::UnexpectedEndOfFile(state.source_mut().position()));
                    }
                };

                let label: ByteString = match self.peek_identifier(state) {
                    Some(_) => self.consume_identifier(state).into(),
                    None => {
                        return match state.source_mut().current() {
                            Some(c) => {
                                Err(LexerError::UnexpectedCharacter(*c, state.source_mut().position()))
                            }
                            None => Err(LexerError::UnexpectedEndOfFile(state.source_mut().position())),
                        }
                    }
                };

                buffer.extend_from_slice(&label);

                if doc_string_kind == DocStringKind::Nowdoc {
                    match state.source_mut().current() {
                        Some(b'\'') => {
                            buffer.push(b'\'');
                            state.source_mut().next();
                        }
                        _ => {
                            // TODO(azjezz) this is most likely a bug, what if current is none?
                            return Err(LexerError::UnexpectedCharacter(
                                *state.source_mut().current().unwrap(),
                                state.source_mut().position(),
                            ));
                        }
                    };
                } else if let Some(b'"') = state.source_mut().current() {
                    buffer.push(b'"');
                    state.source_mut().next();
                }

                if !matches!(state.source_mut().current(), Some(b'\n')) {
                    return Err(LexerError::UnexpectedCharacter(
                        *state.source_mut().current().unwrap(),
                        state.source_mut().position(),
                    ));
                }

                state.source_mut().next();
                state.replace(State::DocString(
                    doc_string_kind,
                    label.clone(),
                    DocStringIndentationKind::None,
                    0,
                ));

                (TokenKind::StartDocString(doc_string_kind), buffer.into())
            }
            [b'*', b'*', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Pow, b"**".into())
            }
            [b'*', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::MultiplyAssign, b"*=".into())
            }
            [b'*', ..] => {
                state.source_mut().next();
                (TokenKind::Multiply, b"*".into())
            }
            [b'|', b'|', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Or, b"||".into())
            }
            [b'|', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::BitwiseOrAssign, b"|=".into())
            }
            [b'|', ..] => {
                state.source_mut().next();
                (TokenKind::BitwiseOr, b"|".into())
            }
            [b'^', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::BitwiseXorAssign, b"^=".into())
            }
            [b'^', ..] => {
                state.source_mut().next();
                (TokenKind::BitwiseXor, b"^".into())
            }
            [b'{', ..] => {
                state.source_mut().next();
                state.enter(State::Scripting);
                (TokenKind::LeftBrace, b"{".into())
            }
            [b'}', ..] => {
                state.source_mut().next();
                state.exit();
                (TokenKind::RightBrace, b"}".into())
            }
            [b'(', ..] => {
                state.source_mut().next();
                let mut buffer = b"(".to_vec();

                // Inlined so we can add whitespace to the buffer.
                while let Some(true) = state.source_mut().current().map(|u: &u8| u.is_ascii_whitespace())
                {
                    buffer.push(*state.source_mut().current().unwrap());
                    state.source_mut().next();
                }

                if state.source_mut().matches_n(b"int", 3) {
                    if state.source_mut().matches_n(b"integer", 7)
                        && state.source_mut().peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        buffer.extend(state.source_mut().read_and_skip_n(7));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::IntCast, buffer.into())
                    } else if state.source_mut().peek_ignoring_whitespace(3, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(3));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::IntCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"bool", 4) {
                    if state.source_mut().matches_n(b"boolean", 7)
                        && state.source_mut().peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        buffer.extend(state.source_mut().read_and_skip_n(7));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::BoolCast, buffer.into())
                    } else if state.source_mut().peek_ignoring_whitespace(4, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(4));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::BoolCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"float", 5) {
                    if state.source_mut().peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"double", 6) {
                    if state.source_mut().peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"real", 4) {
                    if state.source_mut().peek_ignoring_whitespace(4, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(4));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"string", 6) {
                    if state.source_mut().peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::StringCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"binary", 6) {
                    if state.source_mut().peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::BinaryCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"array", 5) {
                    if state.source_mut().peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::ArrayCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"object", 6) {
                    if state.source_mut().peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::ObjectCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source_mut().matches_n(b"unset", 5) {
                    if state.source_mut().peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(state.source_mut().read_and_skip_n(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source_mut().read_and_skip_n(1));

                        (TokenKind::UnsetCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else {
                    (TokenKind::LeftParen, buffer.into())
                }
            }
            [b')', ..] => {
                state.source_mut().next();
                (TokenKind::RightParen, b")".into())
            }
            [b';', ..] => {
                state.source_mut().next();
                (TokenKind::SemiColon, b";".into())
            }
            [b'+', b'+', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Increment, b"++".into())
            }
            [b'+', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::AddAssign, b"+=".into())
            }
            [b'+', ..] => {
                state.source_mut().next();
                (TokenKind::Add, b"+".into())
            }
            [b'%', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::ModuloAssign, b"%=".into())
            }
            [b'%', ..] => {
                state.source_mut().next();
                (TokenKind::Modulo, b"%".into())
            }
            [b'-', b'-', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Decrement, b"--".into())
            }
            [b'-', b'>', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Arrow, b"->".into())
            }
            [b'-', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::SubtractAssign, b"-=".into())
            }
            [b'-', ..] => {
                state.source_mut().next();
                (TokenKind::Subtract, b"-".into())
            }
            [b'<', b'<', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::LeftShift, b"<<".into())
            }
            [b'<', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::LessThanOrEqual, b"<=".into())
            }
            [b'<', b'>', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::NotEqual, b"<>".into())
            }
            [b'<', ..] => {
                state.source_mut().next();
                (TokenKind::LessThan, b"<".into())
            }
            [b'>', b'>', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::RightShift, b">>".into())
            }
            [b'>', b'=', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::GreaterThanOrEqual, b">=".into())
            }
            [b'>', ..] => {
                state.source_mut().next();
                (TokenKind::GreaterThan, b">".into())
            }
            [b',', ..] => {
                state.source_mut().next();
                (TokenKind::Comma, b",".into())
            }
            [b'[', ..] => {
                state.source_mut().next();
                (TokenKind::LeftBracket, b"[".into())
            }
            [b']', ..] => {
                state.source_mut().next();
                (TokenKind::RightBracket, b"]".into())
            }
            [b':', b':', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::DoubleColon, b"::".into())
            }
            [b':', ..] => {
                state.source_mut().next();
                (TokenKind::Colon, b":".into())
            }
            [b'~', ..] => {
                state.source_mut().next();
                (TokenKind::BitwiseNot, b"~".into())
            }
            [b @ ident_start!(), ..] => {
                state.source_mut().next();
                let mut qualified = false;
                let mut last_was_slash = false;

                let mut buffer = vec![*b];
                while let Some(next @ ident!() | next @ b'\\') = state.source_mut().current() {
                    if matches!(next, ident!()) {
                        buffer.push(*next);
                        state.source_mut().next();
                        last_was_slash = false;
                        continue;
                    }

                    if *next == b'\\' && !last_was_slash {
                        qualified = true;
                        last_was_slash = true;
                        buffer.push(*next);
                        state.source_mut().next();
                        continue;
                    }

                    break;
                }

                if qualified {
                    (TokenKind::QualifiedIdentifier, buffer.into())
                } else {
                    let kind = identifier_to_keyword(&buffer).unwrap_or(TokenKind::Identifier);

                    if kind == TokenKind::HaltCompiler {
                        match state.source_mut().read_n(3) {
                            [b'(', b')', b';'] => {
                                state.source_mut().skip_n(3);
                                state.replace(State::Halted);
                            }
                            _ => return Err(LexerError::InvalidHaltCompiler(state.source_mut().position())),
                        }
                    }

                    (kind, buffer.into())
                }
            }
            [b, ..] => unimplemented!(
                "<scripting> char: {}, position: {}",
                *b as char,
                state.source_mut().position(),
            ),
            // We should never reach this point since we have the empty checks surrounding
            // the call to this function, but it's better to be safe than sorry.
            [] => return Err(LexerError::UnexpectedEndOfFile(state.source_mut().position())),
        };

        Ok(Token::new(kind, (start_position, state.source_mut().position()).into(), value))
    }

    fn shell_exec(&self, state: &mut StateMachine, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let position = state.source().position();
        let mut buffer = Vec::new();

        let (kind, value) = loop {
            match state.source_mut().read_n(2) {
                [b'$', b'{'] => {
                    state.source_mut().skip_n(2);
                    state.enter(State::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, b"${".into());
                }
                [b'{', b'$'] => {
                    // Intentionally only consume the left brace.
                    state.source_mut().next();
                    state.enter(State::Scripting);
                    break (TokenKind::LeftBrace, b"{".into());
                }
                [b'`', ..] => {
                    state.source_mut().next();
                    state.replace(State::Scripting);
                    break (TokenKind::Backtick, b"`".into());
                }
                [b'$', ident_start!()] => {
                    let mut var = state.source_mut().read_and_skip_n(1).to_vec();
                    var.extend(self.consume_identifier(state));

                    match state.source_mut().read_n(4) {
                        [b'[', ..] => state.enter(State::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            state.enter(State::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, var.into());
                }
                &[b, ..] => {
                    state.source_mut().next();
                    buffer.push(b);
                }
                [] => return Err(LexerError::UnexpectedEndOfFile(state.source().position())),
            }
        };

        if !buffer.is_empty() {
            tokens.push(Token::new(
                TokenKind::InterpolatedStringPart,
                (position, position).into(),
                buffer.into(),
            ))
        }

        tokens.push(Token::new(kind, (position, state.source().position()).into(), value));

        Ok(())
    }

    fn double_quoted_string(&self, state: &mut StateMachine, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let position = state.source().position();
        let mut buffer = Vec::new();

        let (kind, value) = loop {
            match state.source().read_n(3) {
                [b'$', b'{', ..] => {
                    state.source_mut().skip_n(2);
                    state.enter(State::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, b"${".into());
                }
                [b'{', b'$', ..] => {
                    // Intentionally only consume the left brace.
                    state.source_mut().next();
                    state.enter(State::Scripting);
                    break (TokenKind::LeftBrace, b"{".into());
                }
                [b'"', ..] => {
                    state.source_mut().next();
                    state.replace(State::Scripting);
                    break (TokenKind::DoubleQuote, b'"'.into());
                }
                &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b);
                }
                &[b'\\', b'n', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\n');
                }
                &[b'\\', b'r', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\r');
                }
                &[b'\\', b't', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\t');
                }
                &[b'\\', b'v', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x0b');
                }
                &[b'\\', b'e', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x1b');
                }
                &[b'\\', b'f', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x0c');
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    state.source_mut().skip_n(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source().current()
                    {
                        state.source_mut().next();
                        hex.push(*b as char);
                    }

                    let b = u8::from_str_radix(&hex, 16).unwrap();
                    buffer.push(b);
                }
                &[b'\\', b'u', b'{'] => {
                    state.source_mut().skip_n(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source().current()
                    {
                        state.source_mut().next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || state.source().current() != Some(&b'}') {
                        return Err(LexerError::InvalidUnicodeEscape(state.source().position()));
                    }
                    state.source_mut().next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        return Err(LexerError::InvalidUnicodeEscape(state.source().position()));
                    };

                    if let Some(c) = char::from_u32(c) {
                        let mut tmp = [0; 4];
                        let bytes = c.encode_utf8(&mut tmp);
                        buffer.extend(bytes.as_bytes());
                    } else {
                        return Err(LexerError::InvalidUnicodeEscape(state.source().position()));
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    state.source_mut().skip_n(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = state.source().current() {
                        state.source_mut().next();
                        octal.push(*b as char);
                    }
                    if let Some(b @ b'0'..=b'7') = state.source().current() {
                        state.source_mut().next();
                        octal.push(*b as char);
                    }

                    if let Ok(b) = u8::from_str_radix(&octal, 8) {
                        buffer.push(b);
                    } else {
                        return Err(LexerError::InvalidOctalEscape(state.source().position()));
                    }
                }
                [b'$', ident_start!(), ..] => {
                    let mut var = state.source_mut().read_and_skip_n(1).to_vec();
                    var.extend(self.consume_identifier(state));

                    match state.source_mut().read_n(4) {
                        [b'[', ..] => state.enter(State::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            state.enter(State::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, var.into());
                }
                &[b, ..] => {
                    state.source_mut().next();
                    buffer.push(b);
                }
                [] => return Err(LexerError::UnexpectedEndOfFile(state.source().position())),
            }
        };

        if !buffer.is_empty() {
            tokens.push(Token::new(TokenKind::InterpolatedStringPart, (position, position).into(), buffer.into()))
        }

        tokens.push(Token::new(kind, (position, state.source().position() - 1).into(), value));
        Ok(())
    }

    fn looking_for_varname(&self, state: &mut StateMachine) -> LexerResult<Option<Token>> {
        let position = state.source().position();
        let identifier = self.peek_identifier(state);

        if let Some(ident) = identifier {
            if let [b'[' | b'}'] = state.source().peek_range(ident.len(), 1) {
                let ident = ident.to_vec();
                let end_position = state.source().position();
                state.source_mut().skip_n(ident.len());
                state.replace(State::Scripting);
                return Ok(Some(Token::new(
                    TokenKind::Identifier,
                    (position, end_position).into(),
                    ident.into(),
                )));
            }
        }

        state.replace(State::Scripting);

        Ok(None)
    }

fn looking_for_property(&self, state: &mut StateMachine) -> LexerResult<Token> {
        let position = state.source().position();
        let (kind, value) = match state.source_mut().read_n(3) {
            [b'?', b'-', b'>'] => {
                state.source_mut().skip_n(3);
                (TokenKind::NullsafeArrow, b"?->".into())
            }
            [b'-', b'>', ..] => {
                state.source_mut().skip_n(2);
                (TokenKind::Arrow, b"->".into())
            }
            &[ident_start!(), ..] => {
                let buffer = self.consume_identifier(state);
                state.exit();
                (TokenKind::Identifier, buffer.into())
            }
            // Should be impossible as we already looked ahead this far inside double_quote.
            _ => unreachable!(),
        };

        Ok(Token::new(kind, (position, state.source().position()).into(), value))
    }

    fn var_offset(&self, state: &mut StateMachine) -> LexerResult<Token> {
        let position = state.source().position();

        let (kind, value) = match state.source_mut().read_n(2) {
            [b'$', ident_start!()] => self.tokenize_variable(state),
            [b'0'..=b'9', ..] => {
                // TODO: all integer literals are allowed, but only decimal integers with no underscores
                // are actually treated as numbers. Others are treated as strings.
                // Float literals are not allowed, but that could be handled in the parser.
                self.tokenize_number(state)?
            }
            [b'[', ..] => {
                state.source_mut().next();
                (TokenKind::LeftBracket, b"[".into())
            }
            [b'-', ..] => {
                state.source_mut().next();
                (TokenKind::Subtract, b"-".into())
            }
            [b']', ..] => {
                state.source_mut().next();
                state.exit();
                (TokenKind::RightBracket, b"]".into())
            }
            &[ident_start!(), ..] => {
                let label = self.consume_identifier(state);
                (TokenKind::Identifier, label.into())
            }
            &[b, ..] => return Err(LexerError::UnrecognisedToken(b, state.source().position())),
            [] => return Err(LexerError::UnexpectedEndOfFile(state.source().position())),
        };

        Ok(Token::new(kind, (position, state.source().position()).into(), value))
    }

    fn halted(&self, state: &mut StateMachine, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let position = state.source().position();
        
        tokens.push(Token::new(
            TokenKind::InlineHtml,
            (position, state.source().position()).into(),
            state.source_mut().read_remaining().into(),
        ));

        Ok(())
    }

    fn heredoc(
        &self,
        state: &mut StateMachine,
        tokens: &mut Vec<Token>,
        label: ByteString,
    ) -> LexerResult<()> {
        let position = state.source().position();
        let mut buffer: Vec<u8> = Vec::new();

        let (kind, value) = loop {
            match state.source_mut().read_n(3) {
                [b'$', b'{', ..] => {
                    state.source_mut().skip_n(2);
                    state.enter(State::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, b"${".into());
                }
                [b'{', b'$', ..] => {
                    // Intentionally only consume the left brace.
                    state.source_mut().next();
                    state.enter(State::Scripting);
                    break (TokenKind::LeftBrace, b"{".into());
                }
                &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b);
                }
                &[b'\\', b'n', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\n');
                }
                &[b'\\', b'r', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\r');
                }
                &[b'\\', b't', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\t');
                }
                &[b'\\', b'v', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x0b');
                }
                &[b'\\', b'e', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x1b');
                }
                &[b'\\', b'f', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x0c');
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    state.source_mut().skip_n(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source_mut().current()
                    {
                        state.source_mut().next();
                        hex.push(*b as char);
                    }

                    let b = u8::from_str_radix(&hex, 16).unwrap();
                    buffer.push(b);
                }
                &[b'\\', b'u', b'{'] => {
                    state.source_mut().skip_n(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source_mut().current()
                    {
                        state.source_mut().next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || state.source_mut().current() != Some(&b'}') {
                        return Err(LexerError::InvalidUnicodeEscape(state.source().position()));
                    }
                    state.source_mut().next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        return Err(LexerError::InvalidUnicodeEscape(state.source().position()));
                    };

                    if let Some(c) = char::from_u32(c) {
                        let mut tmp = [0; 4];
                        let bytes = c.encode_utf8(&mut tmp);
                        buffer.extend(bytes.as_bytes());
                    } else {
                        return Err(LexerError::InvalidUnicodeEscape(state.source().position()));
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    state.source_mut().skip_n(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = state.source_mut().current() {
                        state.source_mut().next();
                        octal.push(*b as char);
                    }
                    if let Some(b @ b'0'..=b'7') = state.source_mut().current() {
                        state.source_mut().next();
                        octal.push(*b as char);
                    }

                    if let Ok(b) = u8::from_str_radix(&octal, 8) {
                        buffer.push(b);
                    } else {
                        return Err(LexerError::InvalidOctalEscape(state.source().position()));
                    }
                }
                [b'$', ident_start!(), ..] => {
                    let mut var = state.source_mut().read_and_skip_n(1).to_vec();
                    var.extend(self.consume_identifier(state));

                    match state.source_mut().read_n(4) {
                        [b'[', ..] => state.enter(State::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            state.enter(State::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, var.into());
                }
                // If we find a new-line, we can start to check if we can see the EndHeredoc token.
                [b'\n', ..] => {
                    buffer.push(b'\n');
                    state.source_mut().next();

                    // Check if we can see the closing label right here.
                    if state.source_mut().matches_n(&label, label.len()) {
                        state.source_mut().skip_n(label.len());
                        state.replace(State::Scripting);
                        break (
                            TokenKind::EndDocString(DocStringIndentationKind::None, 0),
                            label,
                        );
                    }

                    // Check if there's any whitespace first.
                    let (whitespace_kind, whitespace_amount) = match state.source_mut().read_n(1) {
                        [b' '] => {
                            let mut amount = 0;
                            while state.source_mut().read_n(1) == [b' '] {
                                amount += 1;
                                state.source_mut().next();
                            }
                            (DocStringIndentationKind::Space, amount)
                        }
                        [b'\t'] => {
                            let mut amount = 0;
                            while state.source_mut().read_n(1) == [b'\t'] {
                                amount += 1;
                                state.source_mut().next();
                            }
                            (DocStringIndentationKind::Tab, amount)
                        }
                        _ => (DocStringIndentationKind::None, 0),
                    };

                    // We've figured out what type of whitespace was being used
                    // at the start of the line.
                    // We should now check for any extra whitespace, of any kind.
                    let mut extra_whitespace_buffer = Vec::new();
                    while let [b @ b' ' | b @ b'\t'] = state.source_mut().read_n(1) {
                        extra_whitespace_buffer.push(b);
                        state.source_mut().next();
                    }

                    // We've consumed all leading whitespace on this line now,
                    // so let's try to read the label again.
                    if state.source_mut().matches_n(&label, label.len()) {
                        // We've found the label, finally! We need to do 1 last
                        // check to make sure there wasn't a mixture of indentation types.
                        if whitespace_kind != DocStringIndentationKind::None
                            && !extra_whitespace_buffer.is_empty()
                        {
                            return Err(LexerError::InvalidDocIndentation(state.source().position()));
                        }

                        // If we get here, only 1 type of indentation was found. We can move
                        // the process along by reading over the label and breaking out
                        // with the EndHeredoc token, storing the kind and amount of whitespace.
                        state.source_mut().skip_n(label.len());
                        state.replace(State::Scripting);
                        break (
                            TokenKind::EndDocString(whitespace_kind, whitespace_amount),
                            label,
                        );
                    } else {
                        // We didn't find the label. The buffer still needs to know about
                        // the whitespace, so let's extend the buffer with the whitespace
                        // and let the loop run again to handle the rest of the line.
                        if whitespace_kind != DocStringIndentationKind::None {
                            let whitespace_char: u8 = whitespace_kind.into();
                            for _ in 0..whitespace_amount {
                                buffer.push(whitespace_char);
                            }
                        }

                        buffer.extend(extra_whitespace_buffer);
                    }
                }
                &[b, ..] => {
                    state.source_mut().next();
                    buffer.push(b);
                }
                [] => return Err(LexerError::UnexpectedEndOfFile(state.source().position())),
            }
        };

        // Any trailing line breaks should be removed from the final heredoc.
        if buffer.last() == Some(&b'\n') {
            buffer.pop();
        }

        if !buffer.is_empty() {
            tokens.push(Token::new(
                TokenKind::InterpolatedStringPart,
                // FIXME: Actually track the position here correctly.
                (position, position).into(),
                buffer.into(),
            ));
        }

        // FIXME: Make sure the positions here are correct.
        tokens.push(Token::new(kind, (position, state.source().position()).into(), value ));

        Ok(())
    }

    fn nowdoc(
        &self,
        state: &mut StateMachine,
        tokens: &mut Vec<Token>,
        label: ByteString,
    ) -> LexerResult<()> {
        let position = state.source().position();
        let mut buffer: Vec<u8> = Vec::new();

        let (kind, value) = loop {
            match state.source_mut().read_n(3) {
                // If we find a new-line, we can start to check if we can see the EndHeredoc token.
                [b'\n', ..] => {
                    buffer.push(b'\n');
                    state.source_mut().next();

                    // Check if we can see the closing label right here.
                    if state.source_mut().matches_n(&label, label.len()) {
                        state.source_mut().skip_n(label.len());
                        state.replace(State::Scripting);
                        break (
                            TokenKind::EndDocString(DocStringIndentationKind::None, 0),
                            label,
                        );
                    }

                    // Check if there's any whitespace first.
                    let (whitespace_kind, whitespace_amount) = match state.source_mut().read_n(1) {
                        [b' '] => {
                            let mut amount = 0;
                            while state.source_mut().read_n(1) == [b' '] {
                                amount += 1;
                                state.source_mut().next();
                            }
                            (DocStringIndentationKind::Space, amount)
                        }
                        [b'\t'] => {
                            let mut amount = 0;
                            while state.source_mut().read_n(1) == [b'\t'] {
                                amount += 1;
                                state.source_mut().next();
                            }
                            (DocStringIndentationKind::Tab, amount)
                        }
                        _ => (DocStringIndentationKind::None, 0),
                    };

                    // We've figured out what type of whitespace was being used
                    // at the start of the line.
                    // We should now check for any extra whitespace, of any kind.
                    let mut extra_whitespace_buffer = Vec::new();
                    while let [b @ b' ' | b @ b'\t'] = state.source_mut().read_n(1) {
                        extra_whitespace_buffer.push(b);
                        state.source_mut().next();
                    }

                    // We've consumed all leading whitespace on this line now,
                    // so let's try to read the label again.
                    if state.source_mut().matches_n(&label, label.len()) {
                        // We've found the label, finally! We need to do 1 last
                        // check to make sure there wasn't a mixture of indentation types.
                        if whitespace_kind != DocStringIndentationKind::None
                            && !extra_whitespace_buffer.is_empty()
                        {
                            return Err(LexerError::InvalidDocIndentation(state.source().position()));
                        }

                        // If we get here, only 1 type of indentation was found. We can move
                        // the process along by reading over the label and breaking out
                        // with the EndHeredoc token, storing the kind and amount of whitespace.
                        state.source_mut().skip_n(label.len());
                        state.replace(State::Scripting);
                        break (
                            TokenKind::EndDocString(whitespace_kind, whitespace_amount),
                            label,
                        );
                    } else {
                        // We didn't find the label. The buffer still needs to know about
                        // the whitespace, so let's extend the buffer with the whitespace
                        // and let the loop run again to handle the rest of the line.
                        if whitespace_kind != DocStringIndentationKind::None {
                            let whitespace_char: u8 = whitespace_kind.into();
                            for _ in 0..whitespace_amount {
                                buffer.push(whitespace_char);
                            }
                        }

                        buffer.extend(extra_whitespace_buffer);
                    }
                }
                &[b, ..] => {
                    state.source_mut().next();
                    buffer.push(b);
                }
                [] => return Err(LexerError::UnexpectedEndOfFile(state.source().position())),
            }
        };

        // Any trailing line breaks should be removed from the final heredoc.
        if buffer.last() == Some(&b'\n') {
            buffer.pop();
        }

        if !buffer.is_empty() {
            tokens.push(Token::new(
                TokenKind::InterpolatedStringPart,
                // FIXME: Make sure positions are correct.
                (position, position).into(),
                buffer.into(),
            ));
        }

        tokens.push(Token::new(kind, (position, state.source().position()).into(), value ));

        Ok(())
    }

    pub fn tokenise(&self, file: &SourceFile) -> LexerResult<Vec<Token>> {
        self.tokenize(file)
    }

    pub fn tokenize(&self, file: &SourceFile) -> LexerResult<Vec<Token>> {
        let mut state = StateMachine::new(Source::new(file.source()));
        let mut tokens = Vec::new();

        while !state.source_mut().is_eof() {
            dbg!(&tokens);

            match state.state()? {
                State::Initial => self.initial(&mut state, &mut tokens)?,
                State::Scripting => {
                    state.source_mut().skip_whitespace();

                    if state.source_mut().is_eof() {
                        break;
                    }

                    tokens.push(self.scripting(&mut state)?);
                },
                State::Halted => self.halted(&mut state, &mut tokens)?,
                State::DoubleQuotedString => self.double_quoted_string(&mut state, &mut tokens)?,
                State::ShellExec => self.shell_exec(&mut state, &mut tokens)?,
                State::LookingForVarname => {
                    if let Some(token) = self.looking_for_varname(&mut state)? {
                        tokens.push(token);
                    }
                },
                State::LookingForProperty => {
                    tokens.push(self.looking_for_property(&mut state)?)
                },
                State::VarOffset => {
                    if state.source().is_eof() {
                        break;
                    }

                    tokens.push(self.var_offset(&mut state)?);
                },
                State::DocString(kind, label, ..) => {
                    let label = label.clone();

                    match kind {
                        DocStringKind::Heredoc => self.heredoc(&mut state, &mut tokens, label)?,
                        DocStringKind::Nowdoc => self.nowdoc(&mut state, &mut tokens, label)?,
                    }
                }
            }
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            Span::new(state.source_mut().position(), state.source_mut().position()),
            ByteString::default(),
        ));

        Ok(tokens)
    }

    fn tokenize_single_quote_string(
        &self,
        state: &mut StateMachine,
    ) -> LexerResult<(TokenKind, ByteString)> {
        let mut buffer = vec![];

        loop {
            match state.source_mut().read_n(2) {
                [b'\'', ..] => {
                    state.source_mut().next();
                    break;
                }
                &[b'\\', b @ b'\'' | b @ b'\\'] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b);
                }
                &[b, ..] => {
                    state.source_mut().next();
                    buffer.push(b);
                }
                [] => return Err(LexerError::UnexpectedEndOfFile(state.source_mut().position())),
            }
        }

        Ok((TokenKind::SingleQuotedString, buffer.into()))
    }

    fn tokenize_double_quote_string(
        &self,
        state: &mut StateMachine,
    ) -> LexerResult<(TokenKind, ByteString)> {
        let mut buffer = vec![];

        let constant = loop {
            match state.source_mut().read_n(3) {
                [b'"', ..] => {
                    state.source_mut().next();
                    break true;
                }
                &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b);
                }
                &[b'\\', b'n', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\n');
                }
                &[b'\\', b'r', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\r');
                }
                &[b'\\', b't', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\t');
                }
                &[b'\\', b'v', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x0b');
                }
                &[b'\\', b'e', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x1b');
                }
                &[b'\\', b'f', ..] => {
                    state.source_mut().skip_n(2);
                    buffer.push(b'\x0c');
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    state.source_mut().skip_n(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source_mut().current()
                    {
                        state.source_mut().next();
                        hex.push(*b as char);
                    }

                    let b = u8::from_str_radix(&hex, 16).unwrap();
                    buffer.push(b);
                }
                &[b'\\', b'u', b'{'] => {
                    state.source_mut().skip_n(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source_mut().current()
                    {
                        state.source_mut().next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || state.source_mut().current() != Some(&b'}') {
                        return Err(LexerError::InvalidUnicodeEscape(state.source_mut().position()));
                    }
                    state.source_mut().next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        return Err(LexerError::InvalidUnicodeEscape(state.source_mut().position()));
                    };

                    if let Some(c) = char::from_u32(c) {
                        let mut tmp = [0; 4];
                        let bytes = c.encode_utf8(&mut tmp);
                        buffer.extend(bytes.as_bytes());
                    } else {
                        return Err(LexerError::InvalidUnicodeEscape(state.source_mut().position()));
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    state.source_mut().skip_n(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = state.source_mut().current() {
                        state.source_mut().next();
                        octal.push(*b as char);
                    }

                    if let Some(b @ b'0'..=b'7') = state.source_mut().current() {
                        state.source_mut().next();
                        octal.push(*b as char);
                    }

                    if let Ok(b) = u8::from_str_radix(&octal, 8) {
                        buffer.push(b);
                    } else {
                        return Err(LexerError::InvalidOctalEscape(state.source_mut().position()));
                    }
                }
                [b'$', ident_start!(), ..] | [b'{', b'$', ..] | [b'$', b'{', ..] => {
                    break false;
                }
                &[b, ..] => {
                    state.source_mut().next();
                    buffer.push(b);
                }
                [] => return Err(LexerError::UnexpectedEndOfFile(state.source_mut().position())),
            }
        };

        Ok(if constant {
            (TokenKind::DoubleQuotedString, buffer.into())
        } else {
            state.replace(State::DoubleQuotedString);
            (TokenKind::InterpolatedStringPart, buffer.into())
        })
    }

    fn tokenize_variable(&self, state: &mut StateMachine) -> (TokenKind, ByteString) {
        let mut var = state.source_mut().read_and_skip_n(1).to_vec();
        var.extend(self.consume_identifier(state));
        (TokenKind::Variable, var.into())
    }

    fn consume_identifier(&self, state: &mut StateMachine) -> Vec<u8> {
        let ident = self.peek_identifier(state).unwrap().to_vec();
        state.source_mut().skip_n(ident.len());

        ident
    }

    fn peek_identifier<'a>(&'a self, state: &'a StateMachine) -> Option<&'a [u8]> {
        let mut size = 0;

        if let [ident_start!()] = state.source().read_n(1) {
            size += 1;
            while let [ident!()] = state.source().peek_range(size, 1) {
                size += 1;
            }

            Some(state.source().read_n(size))
        } else {
            None
        }
    }

    fn tokenize_number(&self, state: &mut StateMachine) -> LexerResult<(TokenKind, ByteString)> {
        let mut buffer = Vec::new();

        let (base, kind) = match state.source_mut().read_n(2) {
            [a @ b'0', b @ b'B' | b @ b'b'] => {
                buffer.push(*a);
                buffer.push(*b);
                state.source_mut().skip_n(2);
                (2, NumberKind::Int)
            }
            [a @ b'0', b @ b'O' | b @ b'o'] => {
                buffer.push(*a);
                buffer.push(*b);
                state.source_mut().skip_n(2);
                (8, NumberKind::Int)
            }
            [a @ b'0', b @ b'X' | b @ b'x'] => {
                buffer.push(*a);
                buffer.push(*b);
                state.source_mut().skip_n(2);
                (16, NumberKind::Int)
            }
            [b'0', ..] => (10, NumberKind::OctalOrFloat),
            [b'.', ..] => (10, NumberKind::Float),
            _ => (10, NumberKind::IntOrFloat),
        };

        if kind != NumberKind::Float {
            self.read_digits(state, &mut buffer, base);
            if kind == NumberKind::Int {
                return parse_int(&buffer);
            }
        }

        // Remaining cases: decimal integer, legacy octal integer, or float.
        let is_float = matches!(
            state.source_mut().read_n(3),
            [b'.', ..] | [b'e' | b'E', b'-' | b'+', b'0'..=b'9'] | [b'e' | b'E', b'0'..=b'9', ..]
        );

        if !is_float {
            return parse_int(&buffer);
        }

        if let Some(b'.') = state.source_mut().current() {
            buffer.push(b'.');
            state.source_mut().next();
            self.read_digits(state, &mut buffer, 10);
        }

        if let Some(b'e' | b'E') = state.source_mut().current() {
            buffer.push(b'e');
            state.source_mut().next();
            if let Some(b @ (b'-' | b'+')) = state.source_mut().current() {
                buffer.push(*b);
                state.source_mut().next();
            }
            self.read_digits(state, &mut buffer, 10);
        }

        Ok((TokenKind::Float, buffer.into()))
    }

    fn read_digits(&self, state: &mut StateMachine, buffer: &mut Vec<u8>, base: usize) {
        if base == 16 {
            self.read_digits_fn(state, buffer, u8::is_ascii_hexdigit);
        } else {
            let max = b'0' + base as u8;
            self.read_digits_fn(state, buffer, |b| (b'0'..max).contains(b));
        };
    }

    fn read_digits_fn<F: Fn(&u8) -> bool>(
        &self,
        state: &mut StateMachine,
        buffer: &mut Vec<u8>,
        is_digit: F,
    ) {
        if let Some(b) = state.source_mut().current() {
            if is_digit(b) {
                state.source_mut().next();
                buffer.push(*b);
            } else {
                return;
            }
        }

        loop {
            match state.source_mut().read_n(2) {
                [b, ..] if is_digit(b) => {
                    state.source_mut().next();
                    buffer.push(*b);
                }
                [b'_', b] if is_digit(b) => {
                    state.source_mut().next();
                    state.source_mut().next();
                    buffer.push(*b);
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn read_and_skip_whitespace(&self, state: &mut StateMachine) -> Vec<u8> {
        let mut buffer = Vec::new();

        while let Some(true) = state.source_mut().current().map(|u: &u8| u.is_ascii_whitespace()) {
            buffer.push(*state.source_mut().current().unwrap());
            state.source_mut().next();
        }
        
        buffer
    }
}

fn parse_int(buffer: &[u8]) -> LexerResult<(TokenKind, ByteString)> {
    Ok((TokenKind::Integer, buffer.into()))
}

#[derive(Debug, Eq, PartialEq)]
enum NumberKind {
    Int,
    Float,
    IntOrFloat,
    OctalOrFloat,
}

#[inline(always)]
fn identifier_to_keyword(ident: &[u8]) -> Option<TokenKind> {
    Some(match ident.to_ascii_lowercase().as_slice() {
        b"eval" => TokenKind::Eval,
        b"die" => TokenKind::Die,
        b"empty" => TokenKind::Empty,
        b"isset" => TokenKind::Isset,
        b"unset" => TokenKind::Unset,
        b"exit" => TokenKind::Exit,
        b"enddeclare" => TokenKind::EndDeclare,
        b"endswitch" => TokenKind::EndSwitch,
        b"endfor" => TokenKind::EndFor,
        b"endwhile" => TokenKind::EndWhile,
        b"endforeach" => TokenKind::EndForeach,
        b"endif" => TokenKind::EndIf,
        b"from" => TokenKind::From,
        b"and" => TokenKind::LogicalAnd,
        b"or" => TokenKind::LogicalOr,
        b"xor" => TokenKind::LogicalXor,
        b"print" => TokenKind::Print,
        b"__halt_compiler" => TokenKind::HaltCompiler,
        b"readonly" => TokenKind::Readonly,
        b"global" => TokenKind::Global,
        b"match" => TokenKind::Match,
        b"abstract" => TokenKind::Abstract,
        b"array" => TokenKind::Array,
        b"as" => TokenKind::As,
        b"break" => TokenKind::Break,
        b"case" => TokenKind::Case,
        b"catch" => TokenKind::Catch,
        b"class" => TokenKind::Class,
        b"clone" => TokenKind::Clone,
        b"continue" => TokenKind::Continue,
        b"const" => TokenKind::Const,
        b"declare" => TokenKind::Declare,
        b"default" => TokenKind::Default,
        b"do" => TokenKind::Do,
        b"echo" => TokenKind::Echo,
        b"else" => TokenKind::Else,
        b"elseif" => TokenKind::ElseIf,
        b"enum" => TokenKind::Enum,
        b"extends" => TokenKind::Extends,
        b"false" => TokenKind::False,
        b"final" => TokenKind::Final,
        b"finally" => TokenKind::Finally,
        b"fn" => TokenKind::Fn,
        b"for" => TokenKind::For,
        b"foreach" => TokenKind::Foreach,
        b"function" => TokenKind::Function,
        b"goto" => TokenKind::Goto,
        b"if" => TokenKind::If,
        b"include" => TokenKind::Include,
        b"include_once" => TokenKind::IncludeOnce,
        b"implements" => TokenKind::Implements,
        b"interface" => TokenKind::Interface,
        b"instanceof" => TokenKind::Instanceof,
        b"namespace" => TokenKind::Namespace,
        b"new" => TokenKind::New,
        b"null" => TokenKind::Null,
        b"private" => TokenKind::Private,
        b"protected" => TokenKind::Protected,
        b"public" => TokenKind::Public,
        b"require" => TokenKind::Require,
        b"require_once" => TokenKind::RequireOnce,
        b"return" => TokenKind::Return,
        b"static" => TokenKind::Static,
        b"switch" => TokenKind::Switch,
        b"throw" => TokenKind::Throw,
        b"trait" => TokenKind::Trait,
        b"true" => TokenKind::True,
        b"try" => TokenKind::Try,
        b"use" => TokenKind::Use,
        b"var" => TokenKind::Var,
        b"yield" => TokenKind::Yield,
        b"__dir__" => TokenKind::__DIR__,
        b"__file__" => TokenKind::__FILE__,
        b"__line__" => TokenKind::__LINE__,
        b"__function__" => TokenKind::__FUNCTION__,
        b"__class__" => TokenKind::__CLASS__,
        b"__method__" => TokenKind::__METHOD__,
        b"__trait__" => TokenKind::__TRAIT__,
        b"__namespace__" => TokenKind::__NAMESPACE__,
        b"__compiler_halt_offset__" => TokenKind::__COMPILER_HALT_OFFSET__,
        b"while" => TokenKind::While,
        b"insteadof" => TokenKind::Insteadof,
        b"list" => TokenKind::List,
        _ => return None,
    })
}