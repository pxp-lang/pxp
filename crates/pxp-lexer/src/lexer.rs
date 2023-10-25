use pxp_bytestring::ByteString;
use pxp_source::{SourceFile, Source};
use pxp_span::Span;
use pxp_token::{Token, TokenKind};

use crate::{LexerResult, state::{StateMachine, State}, LexerError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash, Copy)]
pub struct Lexer;

impl Lexer {
    pub const fn new() -> Self {
        Self
    }

    fn initial(&self, state: &mut StateMachine, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let start_position = state.source().position();
        let source = state.source();
        let mut buffer = Vec::new();

        while let Some(char) = source.current() {
            if source.matches_n(b"<?php", 5) {
                let tag_position = source.position();
                let tag = source.read_and_skip_n(5);

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

            source.next();
            buffer.push(*char);
        }

        tokens.push(Token::new(
            TokenKind::InlineHtml,
            (start_position, source.position()).into(),
            buffer.into(),
        ));

        Ok(())
    }

    fn scripting(&self, state: &mut StateMachine, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let mut source = state.source();
        let start_position = source.position();
        source.skip_whitespace();

        if source.is_eof() {
            return Ok(());
        }

        let (kind, value): (TokenKind, ByteString) = match source.read_n(3) {
            [b'!', b'=', b'='] => {
                source.skip_n(3);

                (TokenKind::NotIdentical, b"!==".into())
            }
            [b'?', b'?', b'='] => {
                source.skip_n(3);
                (TokenKind::NullCoalesceAssign, b"??=".into())
            }
            [b'?', b'-', b'>'] => {
                source.skip_n(3);
                (TokenKind::NullsafeArrow, b"?->".into())
            }
            [b'=', b'=', b'='] => {
                source.skip_n(3);
                (TokenKind::Identical, b"===".into())
            }
            [b'.', b'.', b'.'] => {
                source.skip_n(3);
                (TokenKind::Ellipsis, b"...".into())
            }
            [b'`', ..] => {
                source.next();
                state.replace(State::ShellExec);
                (TokenKind::Backtick, b"`".into())
            }
            [b'@', ..] => {
                source.next();
                (TokenKind::ErrorControl, b"@".into())
            }
            [b'!', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::NotEqual, b"!=".into())
            }
            [b'!', ..] => {
                source.next();
                (TokenKind::Not, b"!".into())
            }
            [b'&', b'&', ..] => {
                source.skip_n(2);
                (TokenKind::And, b"&&".into())
            }
            [b'&', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::BitwiseAndAssign, b"&=".into())
            }
            [b'&', ..] => {
                source.next();
                (TokenKind::BitwiseAnd, b"&".into())
            }
            [b'?', b'>', ..] => {
                // This is a close tag, we can enter "Initial" mode again.
                source.skip_n(2);

                state.replace(State::Initial);

                (TokenKind::CloseTag, b"?>".into())
            }
            [b'?', b'?', ..] => {
                source.skip_n(2);
                (TokenKind::NullCoalesce, b"??".into())
            }
            [b'?', b':', ..] => {
                source.skip_n(2);
                (TokenKind::QuestionColon, b"?:".into())
            }
            [b'?', ..] => {
                source.next();
                (TokenKind::Question, b"?".into())
            }
            [b'=', b'>', ..] => {
                source.skip_n(2);
                (TokenKind::DoubleArrow, b"=>".into())
            }
            [b'=', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::Equals, b"==".into())
            }
            [b'=', ..] => {
                source.next();
                (TokenKind::Equals, b"=".into())
            }
            // Single quoted string.
            [b'\'', ..] => {
                source.skip_n(1);
                self.tokenize_single_quote_string(state)?
            }
            [b'b' | b'B', b'\'', ..] => {
                source.skip_n(2);
                self.tokenize_single_quote_string(state)?
            }
            [b'"', ..] => {
                source.skip_n(1);
                self.tokenize_double_quote_string(state)?
            }
            [b'b' | b'B', b'"', ..] => {
                source.skip_n(2);
                self.tokenize_double_quote_string(state)?
            }
            [b'$', ident_start!(), ..] => self.tokenize_variable(state),
            [b'$', ..] => {
                source.next();
                (TokenKind::Dollar, b"$".into())
            }
            [b'.', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::ConcatAssign, b".=".into())
            }
            [b'0'..=b'9', ..] => self.tokenize_number(state)?,
            [b'.', b'0'..=b'9', ..] => self.tokenize_number(state)?,
            [b'.', ..] => {
                source.next();
                (TokenKind::Concat, b".".into())
            }
            [b'\\', ident_start!(), ..] => {
                source.next();

                match self.scripting(state)? {
                    Token {
                        kind: TokenKind::Identifier | TokenKind::QualifiedIdentifier,
                        value,
                        ..
                    } => {
                        let mut bytes = value;
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
                source.next();
                (TokenKind::NamespaceSeparator, b"\\".into())
            }
            [b'/', b'*', ..] => {
                source.next();
                let mut buffer = vec![b'/'];

                loop {
                    match source.read_n(2) {
                        [b'*', b'/'] => {
                            source.skip_n(2);
                            buffer.extend_from_slice(b"*/");
                            break;
                        }
                        &[t, ..] => {
                            source.next();
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
                source.skip_n(2);
                (TokenKind::Attribute, b"#[".into())
            }
            [ch @ b'/', b'/', ..] | [ch @ b'#', ..] => {
                let mut buffer = if *ch == b'/' {
                    source.skip_n(2);
                    b"//".to_vec()
                } else {
                    source.next();
                    b"#".to_vec()
                };

                while let Some(c) = source.current() {
                    if *c == b'\n' {
                        source.next();
                        break;
                    }

                    if source.read_n(2) == [b'?', b'>'] {
                        break;
                    }

                    buffer.push(*c);
                    source.next();
                }

                if buffer.starts_with(b"#") {
                    (TokenKind::HashComment, buffer.into())
                } else {
                    (TokenKind::SlashComment, buffer.into())
                }
            }
            [b'/', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::DivideAssign, b"/=".into())
            }
            [b'/', ..] => {
                source.next();
                (TokenKind::Divide, b"/".into())
            }
            [b'*', b'*', b'=', ..] => {
                source.skip_n(3);
                (TokenKind::PowAssign, b"**=".into())
            }
            [b'<', b'<', b'='] => {
                source.skip_n(3);

                (TokenKind::LeftShiftAssign, b"<<=".into())
            }
            [b'<', b'=', b'>'] => {
                source.skip_n(3);
                (TokenKind::Spaceship, b"<=>".into())
            }
            [b'>', b'>', b'='] => {
                source.skip_n(3);
                (TokenKind::RightShiftAssign, b">>=".into())
            }
            [b'<', b'<', b'<'] => {
                source.skip_n(3);
                let mut buffer = b"<<<".to_vec();
                buffer.extend(self.read_and_skip_whitespace(state));

                let doc_string_kind = match source.read_n(1) {
                    [b'\''] => {
                        buffer.push(b'\'');
                        source.next();
                        DocStringKind::Nowdoc
                    }
                    [b'"'] => {
                        buffer.push(b'"');
                        source.next();
                        DocStringKind::Heredoc
                    }
                    [_, ..] => DocStringKind::Heredoc,
                    [] => {
                        return Err(SyntaxError::UnexpectedEndOfFile(source.span()));
                    }
                };

                let label: ByteString = match self.peek_identifier(state) {
                    Some(_) => self.consume_identifier(state).into(),
                    None => {
                        return match source.current() {
                            Some(c) => {
                                Err(SyntaxError::UnexpectedCharacter(*c, source.span()))
                            }
                            None => Err(SyntaxError::UnexpectedEndOfFile(source.span())),
                        }
                    }
                };

                buffer.extend_from_slice(&label);

                if doc_string_kind == DocStringKind::Nowdoc {
                    match source.current() {
                        Some(b'\'') => {
                            buffer.push(b'\'');
                            source.next();
                        }
                        _ => {
                            // TODO(azjezz) this is most likely a bug, what if current is none?
                            return Err(SyntaxError::UnexpectedCharacter(
                                *source.current().unwrap(),
                                source.span(),
                            ));
                        }
                    };
                } else if let Some(b'"') = source.current() {
                    buffer.push(b'"');
                    source.next();
                }

                if !matches!(source.current(), Some(b'\n')) {
                    return Err(SyntaxError::UnexpectedCharacter(
                        *source.current().unwrap(),
                        source.span(),
                    ));
                }

                source.next();
                state.replace(StackFrame::DocString(
                    doc_string_kind.clone(),
                    label.clone(),
                    DocStringIndentationKind::None,
                    0,
                ));

                (TokenKind::StartDocString(doc_string_kind), buffer.into())
            }
            [b'*', b'*', ..] => {
                source.skip_n(2);
                (TokenKind::Pow, b"**".into())
            }
            [b'*', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::MultiplyAssign, b"*=".into())
            }
            [b'*', ..] => {
                source.next();
                (TokenKind::Multiply, b"*".into())
            }
            [b'|', b'|', ..] => {
                source.skip_n(2);
                (TokenKind::Or, b"||".into())
            }
            [b'|', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::BitwiseOrAssign, b"|=".into())
            }
            [b'|', ..] => {
                source.next();
                (TokenKind::BitwiseOr, b"|".into())
            }
            [b'^', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::BitwiseXorAssign, b"^=".into())
            }
            [b'^', ..] => {
                source.next();
                (TokenKind::BitwiseXor, b"^".into())
            }
            [b'{', ..] => {
                source.next();
                state.enter(State::Scripting);
                (TokenKind::LeftBrace, b"{".into())
            }
            [b'}', ..] => {
                source.next();
                state.exit();
                (TokenKind::RightBrace, b"}".into())
            }
            [b'(', ..] => {
                source.next();
                let mut buffer = b"(".to_vec();

                // Inlined so we can add whitespace to the buffer.
                while let Some(true) = source.current().map(|u: &u8| u.is_ascii_whitespace())
                {
                    buffer.push(*source.current().unwrap());
                    source.next();
                }

                if source.matches_n(b"int", 3) {
                    if source.matches_n(b"integer", 7)
                        && source.peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        buffer.extend(source.read_and_skip_n(7));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::IntCast, buffer.into())
                    } else if source.peek_ignoring_whitespace(3, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(3));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::IntCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"bool", 4) {
                    if source.at_case_insensitive(b"boolean", 7)
                        && source.peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        buffer.extend(source.read_and_skip_n(7));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::BoolCast, buffer.into())
                    } else if source.peek_ignoring_whitespace(4, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(4));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::BoolCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"float", 5) {
                    if source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"double", 6) {
                    if source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"real", 4) {
                    if source.peek_ignoring_whitespace(4, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(4));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"string", 6) {
                    if source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::StringCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"binary", 6) {
                    if source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::BinaryCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"array", 5) {
                    if source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::ArrayCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"object", 6) {
                    if source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::ObjectCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if source.at_case_insensitive(b"unset", 5) {
                    if source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(source.read_and_skip_n(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(source.read_and_skip_n(1));

                        (TokenKind::UnsetCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else {
                    (TokenKind::LeftParen, buffer.into())
                }
            }
            [b')', ..] => {
                source.next();
                (TokenKind::RightParen, b")".into())
            }
            [b';', ..] => {
                source.next();
                (TokenKind::SemiColon, b";".into())
            }
            [b'+', b'+', ..] => {
                source.skip_n(2);
                (TokenKind::Increment, b"++".into())
            }
            [b'+', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::AddAssign, b"+=".into())
            }
            [b'+', ..] => {
                source.next();
                (TokenKind::Add, b"+".into())
            }
            [b'%', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::ModuloAssign, b"%=".into())
            }
            [b'%', ..] => {
                source.next();
                (TokenKind::Modulo, b"%".into())
            }
            [b'-', b'-', ..] => {
                source.skip_n(2);
                (TokenKind::Decrement, b"--".into())
            }
            [b'-', b'>', ..] => {
                source.skip_n(2);
                (TokenKind::Arrow, b"->".into())
            }
            [b'-', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::SubtractAssign, b"-=".into())
            }
            [b'-', ..] => {
                source.next();
                (TokenKind::Subtract, b"-".into())
            }
            [b'<', b'<', ..] => {
                source.skip_n(2);
                (TokenKind::LeftShift, b"<<".into())
            }
            [b'<', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::LessThanOrEqual, b"<=".into())
            }
            [b'<', b'>', ..] => {
                source.skip_n(2);
                (TokenKind::NotEqual, b"<>".into())
            }
            [b'<', ..] => {
                source.next();
                (TokenKind::LessThan, b"<".into())
            }
            [b'>', b'>', ..] => {
                source.skip_n(2);
                (TokenKind::RightShift, b">>".into())
            }
            [b'>', b'=', ..] => {
                source.skip_n(2);
                (TokenKind::GreaterThanOrEqual, b">=".into())
            }
            [b'>', ..] => {
                source.next();
                (TokenKind::GreaterThan, b">".into())
            }
            [b',', ..] => {
                source.next();
                (TokenKind::Comma, b",".into())
            }
            [b'[', ..] => {
                source.next();
                (TokenKind::LeftBracket, b"[".into())
            }
            [b']', ..] => {
                source.next();
                (TokenKind::RightBracket, b"]".into())
            }
            [b':', b':', ..] => {
                source.skip_n(2);
                (TokenKind::DoubleColon, b"::".into())
            }
            [b':', ..] => {
                source.next();
                (TokenKind::Colon, b":".into())
            }
            [b'~', ..] => {
                source.next();
                (TokenKind::BitwiseNot, b"~".into())
            }
            [b @ ident_start!(), ..] => {
                source.next();
                let mut qualified = false;
                let mut last_was_slash = false;

                let mut buffer = vec![*b];
                while let Some(next @ ident!() | next @ b'\\') = source.current() {
                    if matches!(next, ident!()) {
                        buffer.push(*next);
                        source.next();
                        last_was_slash = false;
                        continue;
                    }

                    if *next == b'\\' && !last_was_slash {
                        qualified = true;
                        last_was_slash = true;
                        buffer.push(*next);
                        source.next();
                        continue;
                    }

                    break;
                }

                if qualified {
                    (TokenKind::QualifiedIdentifier, buffer.into())
                } else {
                    let kind = identifier_to_keyword(&buffer).unwrap_or(TokenKind::Identifier);

                    if kind == TokenKind::HaltCompiler {
                        match source.read_n(3) {
                            [b'(', b')', b';'] => {
                                source.skip_n(3);
                                state.replace(State::Halted);
                            }
                            _ => return Err(LexerError::InvalidHaltCompiler(source.position())),
                        }
                    }

                    (kind, buffer.into())
                }
            }
            [b, ..] => unimplemented!(
                "<scripting> char: {}, position: {}",
                *b as char,
                source.position(),
            ),
            // We should never reach this point since we have the empty checks surrounding
            // the call to this function, but it's better to be safe than sorry.
            [] => return Err(LexerError::UnexpectedEndOfFile(source.position())),
        };

        Ok(())
    }

    pub fn tokenize(&self, file: &SourceFile) -> LexerResult<Vec<Token>> {
        let mut state = StateMachine::new(Source::new(file.source()));
        let mut tokens = Vec::new();

        while !state.source().is_eof() {
            match state.state()? {
                State::Initial => self.initial(&mut state, &mut tokens)?,
                State::Scripting => self.scripting(&mut state, &mut tokens)?,
                // State::Halted => self.halted(&mut state, &mut tokens)?,
                // State::DoubleQuotedString => self.double_quoted_string(&mut state, &mut tokens)?,
                // State::ShellExec => self.shell_exec(&mut state, &mut tokens)?,
                // State::LookingForVarname => self.looking_for_varname(&mut state, &mut tokens)?,
                // State::LookingForProperty => self.looking_for_property(&mut state, &mut tokens)?,
                // State::VarOffset => self.var_offset(&mut state, &mut tokens)?,
                _ => return Err(LexerError::UnpredictableState(state.source().position())),
            }
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            Span::new(state.source().position(), state.source().position()),
            ByteString::default(),
        ));

        Ok(tokens)
    }
}