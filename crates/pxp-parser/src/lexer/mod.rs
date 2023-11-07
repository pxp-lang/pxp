use crate::ident;
use crate::ident_start;
use crate::lexer::byte_string::ByteString;
use crate::lexer::error::SyntaxError;
use crate::lexer::error::SyntaxResult;
use crate::lexer::state::source::Source;
use crate::lexer::state::StackFrame;
use crate::lexer::state::State;
use crate::lexer::token::DocStringIndentationKind;
use crate::lexer::token::DocStringKind;
use crate::lexer::token::OpenTagKind;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

pub mod byte_string;
pub mod error;
pub mod stream;
pub mod token;

mod state;

mod macros;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Lexer;

impl Lexer {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn tokenize<B: ?Sized + AsRef<[u8]>>(&self, input: &B) -> SyntaxResult<Vec<Token>> {
        let mut state = State::new(Source::new(input.as_ref()));
        let mut tokens = Vec::new();

        while !state.source.eof() {
            match state.frame()? {
                // The "Initial" state is used to parse inline HTML. It is essentially a catch-all
                // state that will build up a single token buffer until it encounters an open tag
                // of some description.
                StackFrame::Initial => self.initial(&mut state, &mut tokens)?,
                // The scripting state is entered when an open tag is encountered in the source code.
                // This tells the lexer to start analysing characters at PHP tokens instead of inline HTML.
                StackFrame::Scripting => {
                    self.skip_whitespace(&mut state);

                    // If we have consumed whitespace and then reached the end of the file, we should break.
                    if state.source.eof() {
                        break;
                    }

                    tokens.push(self.scripting(&mut state)?);
                }
                // The "Halted" state is entered when the `__halt_compiler` token is encountered.
                // In this state, all the text that follows is no longer parsed as PHP as is collected
                // into a single "InlineHtml" token (kind of cheating, oh well).
                StackFrame::Halted => {
                    tokens.push(Token {
                        kind: TokenKind::InlineHtml,
                        span: state.source.span(),
                        value: state.source.read_remaining().into(),
                    });
                    break;
                }
                // The double quote state is entered when inside a double-quoted string that
                // contains variables.
                StackFrame::DoubleQuote => self.double_quote(&mut state, &mut tokens)?,
                // The shell exec state is entered when inside of a execution string (`).
                StackFrame::ShellExec => self.shell_exec(&mut state, &mut tokens)?,
                // The doc string state is entered when tokenizing heredocs and nowdocs.
                StackFrame::DocString(kind, label, ..) => {
                    let label = label.clone();

                    match kind {
                        DocStringKind::Heredoc => self.heredoc(&mut state, &mut tokens, label)?,
                        DocStringKind::Nowdoc => self.nowdoc(&mut state, &mut tokens, label)?,
                    }
                }
                // LookingForProperty is entered inside double quotes,
                // backticks, or a heredoc, expecting a variable name.
                // If one isn't found, it switches to scripting.
                StackFrame::LookingForVarname => {
                    if let Some(token) = self.looking_for_varname(&mut state)? {
                        tokens.push(token);
                    }
                }
                // LookingForProperty is entered inside double quotes,
                // backticks, or a heredoc, expecting an arrow followed by a
                // property name.
                StackFrame::LookingForProperty => {
                    tokens.push(self.looking_for_property(&mut state)?);
                }
                StackFrame::VarOffset => {
                    if state.source.eof() {
                        break;
                    }

                    tokens.push(self.var_offset(&mut state)?);
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: state.source.span(),
            value: ByteString::default(),
        });

        Ok(tokens)
    }

    fn skip_whitespace(&self, state: &mut State) {
        while let Some(true) = state.source.current().map(|u: &u8| u.is_ascii_whitespace()) {
            state.source.next();
        }
    }

    fn read_and_skip_whitespace(&self, state: &mut State) -> Vec<u8> {
        let mut buffer = Vec::new();
        while let Some(true) = state.source.current().map(|u: &u8| u.is_ascii_whitespace()) {
            buffer.push(*state.source.current().unwrap());
            state.source.next();
        }
        buffer
    }

    fn initial(&self, state: &mut State, tokens: &mut Vec<Token>) -> SyntaxResult<()> {
        let inline_span = state.source.span();
        let mut buffer = Vec::new();
        while let Some(char) = state.source.current() {
            if state.source.at_case_insensitive(b"<?php", 5) {
                let tag_span = state.source.span();

                let tag = state.source.read_and_skip(5);
                state.replace(StackFrame::Scripting);

                if !buffer.is_empty() {
                    tokens.push(Token {
                        kind: TokenKind::InlineHtml,
                        span: inline_span,
                        value: buffer.into(),
                    });
                }

                tokens.push(Token {
                    kind: TokenKind::OpenTag(OpenTagKind::Full),
                    span: tag_span,
                    value: tag.into(),
                });

                return Ok(());
            } else if state.source.at_case_insensitive(b"<?=", 3) {
                let tag_span = state.source.span();

                state.source.skip(3);
                state.replace(StackFrame::Scripting);

                if !buffer.is_empty() {
                    tokens.push(Token {
                        kind: TokenKind::InlineHtml,
                        span: inline_span,
                        value: buffer.into(),
                    });
                }

                tokens.push(Token {
                    kind: TokenKind::OpenTag(OpenTagKind::Echo),
                    span: tag_span,
                    value: b"<?=".into(),
                });

                return Ok(());
            } else if state.source.at_case_insensitive(b"<?", 2) {
                let tag_span = state.source.span();

                state.source.skip(2);
                state.replace(StackFrame::Scripting);

                if !buffer.is_empty() {
                    tokens.push(Token {
                        kind: TokenKind::InlineHtml,
                        span: inline_span,
                        value: buffer.into(),
                    });
                }

                tokens.push(Token {
                    kind: TokenKind::OpenTag(OpenTagKind::Short),
                    span: tag_span,
                    value: b"<?".into(),
                });

                return Ok(());
            }

            state.source.next();
            buffer.push(*char);
        }

        tokens.push(Token {
            kind: TokenKind::InlineHtml,
            span: inline_span,
            value: buffer.into(),
        });

        Ok(())
    }

    fn scripting(&self, state: &mut State) -> SyntaxResult<Token> {
        let span = state.source.span();
        let (kind, value): (TokenKind, ByteString) = match state.source.read(3) {
            [b'!', b'=', b'='] => {
                state.source.skip(3);

                (TokenKind::BangDoubleEquals, b"!==".into())
            }
            [b'?', b'?', b'='] => {
                state.source.skip(3);
                (TokenKind::DoubleQuestionEquals, b"??=".into())
            }
            [b'?', b'-', b'>'] => {
                state.source.skip(3);
                (TokenKind::QuestionArrow, b"?->".into())
            }
            [b'=', b'=', b'='] => {
                state.source.skip(3);
                (TokenKind::TripleEquals, b"===".into())
            }
            [b'.', b'.', b'.'] => {
                state.source.skip(3);
                (TokenKind::Ellipsis, b"...".into())
            }
            [b'`', ..] => {
                state.source.next();
                state.replace(StackFrame::ShellExec);
                (TokenKind::Backtick, b"`".into())
            }
            [b'@', ..] => {
                state.source.next();
                (TokenKind::At, b"@".into())
            }
            [b'!', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::BangEquals, b"!=".into())
            }
            [b'!', ..] => {
                state.source.next();
                (TokenKind::Bang, b"!".into())
            }
            [b'&', b'&', ..] => {
                state.source.skip(2);
                (TokenKind::BooleanAnd, b"&&".into())
            }
            [b'&', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::AmpersandEquals, b"&=".into())
            }
            [b'&', ..] => {
                state.source.next();
                (TokenKind::Ampersand, b"&".into())
            }
            [b'?', b'>', ..] => {
                // This is a close tag, we can enter "Initial" mode again.
                state.source.skip(2);

                state.replace(StackFrame::Initial);

                (TokenKind::CloseTag, b"?>".into())
            }
            [b'?', b'?', ..] => {
                state.source.skip(2);
                (TokenKind::DoubleQuestion, b"??".into())
            }
            [b'?', b':', ..] => {
                state.source.skip(2);
                (TokenKind::QuestionColon, b"?:".into())
            }
            [b'?', ..] => {
                state.source.next();
                (TokenKind::Question, b"?".into())
            }
            [b'=', b'>', ..] => {
                state.source.skip(2);
                (TokenKind::DoubleArrow, b"=>".into())
            }
            [b'=', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::DoubleEquals, b"==".into())
            }
            [b'=', ..] => {
                state.source.next();
                (TokenKind::Equals, b"=".into())
            }
            // Single quoted string.
            [b'\'', ..] => {
                state.source.skip(1);
                self.tokenize_single_quote_string(state)?
            }
            [b'b' | b'B', b'\'', ..] => {
                state.source.skip(2);
                self.tokenize_single_quote_string(state)?
            }
            [b'"', ..] => {
                state.source.skip(1);
                self.tokenize_double_quote_string(state)?
            }
            [b'b' | b'B', b'"', ..] => {
                state.source.skip(2);
                self.tokenize_double_quote_string(state)?
            }
            [b'$', ident_start!(), ..] => self.tokenize_variable(state),
            [b'$', ..] => {
                state.source.next();
                (TokenKind::Dollar, b"$".into())
            }
            [b'.', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::DotEquals, b".=".into())
            }
            [b'0'..=b'9', ..] => self.tokenize_number(state)?,
            [b'.', b'0'..=b'9', ..] => self.tokenize_number(state)?,
            [b'.', ..] => {
                state.source.next();
                (TokenKind::Dot, b".".into())
            }
            [b'\\', ident_start!(), ..] => {
                state.source.next();

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
                state.source.next();
                (TokenKind::NamespaceSeparator, b"\\".into())
            }
            [b'/', b'*', ..] => {
                state.source.next();
                let mut buffer = vec![b'/'];

                loop {
                    match state.source.read(2) {
                        [b'*', b'/'] => {
                            state.source.skip(2);
                            buffer.extend_from_slice(b"*/");
                            break;
                        }
                        &[t, ..] => {
                            state.source.next();
                            buffer.push(t);
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if buffer.starts_with(b"/**") {
                    (TokenKind::DocumentComment, buffer.into())
                } else {
                    (TokenKind::MultiLineComment, buffer.into())
                }
            }
            [b'#', b'[', ..] => {
                state.source.skip(2);
                (TokenKind::Attribute, b"#[".into())
            }
            [ch @ b'/', b'/', ..] | [ch @ b'#', ..] => {
                let mut buffer = if *ch == b'/' {
                    state.source.skip(2);
                    b"//".to_vec()
                } else {
                    state.source.next();
                    b"#".to_vec()
                };

                while let Some(c) = state.source.current() {
                    if *c == b'\n' {
                        state.source.next();
                        break;
                    }

                    if state.source.read(2) == [b'?', b'>'] {
                        break;
                    }

                    buffer.push(*c);
                    state.source.next();
                }

                if buffer.starts_with(b"#") {
                    (TokenKind::HashMarkComment, buffer.into())
                } else {
                    (TokenKind::SingleLineComment, buffer.into())
                }
            }
            [b'/', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::SlashEquals, b"/=".into())
            }
            [b'/', ..] => {
                state.source.next();
                (TokenKind::Slash, b"/".into())
            }
            [b'*', b'*', b'=', ..] => {
                state.source.skip(3);
                (TokenKind::PowEquals, b"**=".into())
            }
            [b'<', b'<', b'='] => {
                state.source.skip(3);

                (TokenKind::LeftShiftEquals, b"<<=".into())
            }
            [b'<', b'=', b'>'] => {
                state.source.skip(3);
                (TokenKind::Spaceship, b"<=>".into())
            }
            [b'>', b'>', b'='] => {
                state.source.skip(3);
                (TokenKind::RightShiftEquals, b">>=".into())
            }
            [b'<', b'<', b'<'] => {
                state.source.skip(3);
                let mut buffer = b"<<<".to_vec();
                buffer.extend(self.read_and_skip_whitespace(state));

                let doc_string_kind = match state.source.read(1) {
                    [b'\''] => {
                        buffer.push(b'\'');
                        state.source.next();
                        DocStringKind::Nowdoc
                    }
                    [b'"'] => {
                        buffer.push(b'"');
                        state.source.next();
                        DocStringKind::Heredoc
                    }
                    [_, ..] => DocStringKind::Heredoc,
                    [] => {
                        return Err(SyntaxError::UnexpectedEndOfFile(state.source.span()));
                    }
                };

                let label: ByteString = match self.peek_identifier(state) {
                    Some(_) => self.consume_identifier(state).into(),
                    None => {
                        return match state.source.current() {
                            Some(c) => {
                                Err(SyntaxError::UnexpectedCharacter(*c, state.source.span()))
                            }
                            None => Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
                        }
                    }
                };

                buffer.extend_from_slice(&label);

                if doc_string_kind == DocStringKind::Nowdoc {
                    match state.source.current() {
                        Some(b'\'') => {
                            buffer.push(b'\'');
                            state.source.next();
                        }
                        _ => {
                            // TODO(azjezz) this is most likely a bug, what if current is none?
                            return Err(SyntaxError::UnexpectedCharacter(
                                *state.source.current().unwrap(),
                                state.source.span(),
                            ));
                        }
                    };
                } else if let Some(b'"') = state.source.current() {
                    buffer.push(b'"');
                    state.source.next();
                }

                if !matches!(state.source.current(), Some(b'\n')) {
                    return Err(SyntaxError::UnexpectedCharacter(
                        *state.source.current().unwrap(),
                        state.source.span(),
                    ));
                }

                state.source.next();
                state.replace(StackFrame::DocString(
                    doc_string_kind.clone(),
                    label.clone(),
                    DocStringIndentationKind::None,
                    0,
                ));

                (TokenKind::StartDocString(doc_string_kind), buffer.into())
            }
            [b'*', b'*', ..] => {
                state.source.skip(2);
                (TokenKind::Pow, b"**".into())
            }
            [b'*', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::AsteriskEquals, b"*=".into())
            }
            [b'*', ..] => {
                state.source.next();
                (TokenKind::Asterisk, b"*".into())
            }
            [b'|', b'|', ..] => {
                state.source.skip(2);
                (TokenKind::BooleanOr, b"||".into())
            }
            [b'|', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::PipeEquals, b"|=".into())
            }
            [b'|', ..] => {
                state.source.next();
                (TokenKind::Pipe, b"|".into())
            }
            [b'^', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::CaretEquals, b"^=".into())
            }
            [b'^', ..] => {
                state.source.next();
                (TokenKind::Caret, b"^".into())
            }
            [b'{', ..] => {
                state.source.next();
                state.enter(StackFrame::Scripting);
                (TokenKind::LeftBrace, b"{".into())
            }
            [b'}', ..] => {
                state.source.next();
                state.exit();
                (TokenKind::RightBrace, b"}".into())
            }
            [b'(', ..] => {
                state.source.next();
                let mut buffer = b"(".to_vec();

                // Inlined so we can add whitespace to the buffer.
                while let Some(true) = state.source.current().map(|u: &u8| u.is_ascii_whitespace())
                {
                    buffer.push(*state.source.current().unwrap());
                    state.source.next();
                }

                if state.source.at_case_insensitive(b"int", 3) {
                    if state.source.at_case_insensitive(b"integer", 7)
                        && state.source.peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        buffer.extend(state.source.read_and_skip(7));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::IntegerCast, buffer.into())
                    } else if state.source.peek_ignoring_whitespace(3, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(3));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::IntCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"bool", 4) {
                    if state.source.at_case_insensitive(b"boolean", 7)
                        && state.source.peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        buffer.extend(state.source.read_and_skip(7));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::BooleanCast, buffer.into())
                    } else if state.source.peek_ignoring_whitespace(4, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(4));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::BoolCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"float", 5) {
                    if state.source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::FloatCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"double", 6) {
                    if state.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::DoubleCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"real", 4) {
                    if state.source.peek_ignoring_whitespace(4, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(4));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::RealCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"string", 6) {
                    if state.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::StringCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"binary", 6) {
                    if state.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::BinaryCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"array", 5) {
                    if state.source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::ArrayCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"object", 6) {
                    if state.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(6));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::ObjectCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else if state.source.at_case_insensitive(b"unset", 5) {
                    if state.source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        buffer.extend(state.source.read_and_skip(5));
                        buffer.extend(self.read_and_skip_whitespace(state));
                        buffer.extend(state.source.read_and_skip(1));

                        (TokenKind::UnsetCast, buffer.into())
                    } else {
                        (TokenKind::LeftParen, buffer.into())
                    }
                } else {
                    (TokenKind::LeftParen, buffer.into())
                }
            }
            [b')', ..] => {
                state.source.next();
                (TokenKind::RightParen, b")".into())
            }
            [b';', ..] => {
                state.source.next();
                (TokenKind::SemiColon, b";".into())
            }
            [b'+', b'+', ..] => {
                state.source.skip(2);
                (TokenKind::Increment, b"++".into())
            }
            [b'+', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::PlusEquals, b"+=".into())
            }
            [b'+', ..] => {
                state.source.next();
                (TokenKind::Plus, b"+".into())
            }
            [b'%', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::PercentEquals, b"%=".into())
            }
            [b'%', ..] => {
                state.source.next();
                (TokenKind::Percent, b"%".into())
            }
            [b'-', b'-', ..] => {
                state.source.skip(2);
                (TokenKind::Decrement, b"--".into())
            }
            [b'-', b'>', ..] => {
                state.source.skip(2);
                (TokenKind::Arrow, b"->".into())
            }
            [b'-', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::MinusEquals, b"-=".into())
            }
            [b'-', ..] => {
                state.source.next();
                (TokenKind::Minus, b"-".into())
            }
            [b'<', b'<', ..] => {
                state.source.skip(2);
                (TokenKind::LeftShift, b"<<".into())
            }
            [b'<', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::LessThanEquals, b"<=".into())
            }
            [b'<', b'>', ..] => {
                state.source.skip(2);
                (TokenKind::AngledLeftRight, b"<>".into())
            }
            [b'<', ..] => {
                state.source.next();
                (TokenKind::LessThan, b"<".into())
            }
            [b'>', b'>', ..] => {
                state.source.skip(2);
                (TokenKind::RightShift, b">>".into())
            }
            [b'>', b'=', ..] => {
                state.source.skip(2);
                (TokenKind::GreaterThanEquals, b">=".into())
            }
            [b'>', ..] => {
                state.source.next();
                (TokenKind::GreaterThan, b">".into())
            }
            [b',', ..] => {
                state.source.next();
                (TokenKind::Comma, b",".into())
            }
            [b'[', ..] => {
                state.source.next();
                (TokenKind::LeftBracket, b"[".into())
            }
            [b']', ..] => {
                state.source.next();
                (TokenKind::RightBracket, b"]".into())
            }
            [b':', b':', ..] => {
                state.source.skip(2);
                (TokenKind::DoubleColon, b"::".into())
            }
            [b':', ..] => {
                state.source.next();
                (TokenKind::Colon, b":".into())
            }
            [b'~', ..] => {
                state.source.next();
                (TokenKind::BitwiseNot, b"~".into())
            }
            [b @ ident_start!(), ..] => {
                state.source.next();
                let mut qualified = false;
                let mut last_was_slash = false;

                let mut buffer = vec![*b];
                while let Some(next @ ident!() | next @ b'\\') = state.source.current() {
                    if matches!(next, ident!()) {
                        buffer.push(*next);
                        state.source.next();
                        last_was_slash = false;
                        continue;
                    }

                    if *next == b'\\' && !last_was_slash {
                        qualified = true;
                        last_was_slash = true;
                        buffer.push(*next);
                        state.source.next();
                        continue;
                    }

                    break;
                }

                if qualified {
                    (TokenKind::QualifiedIdentifier, buffer.into())
                } else {
                    let kind = identifier_to_keyword(&buffer).unwrap_or(TokenKind::Identifier);

                    if kind == TokenKind::HaltCompiler {
                        match state.source.read(3) {
                            [b'(', b')', b';'] => {
                                state.source.skip(3);
                                state.replace(StackFrame::Halted);
                            }
                            _ => return Err(SyntaxError::InvalidHaltCompiler(state.source.span())),
                        }
                    }

                    (kind, buffer.into())
                }
            }
            [b, ..] => unimplemented!(
                "<scripting> char: {}, line: {}, col: {}",
                *b as char,
                state.source.span().line,
                state.source.span().column
            ),
            // We should never reach this point since we have the empty checks surrounding
            // the call to this function, but it's better to be safe than sorry.
            [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
        };

        Ok(Token { kind, span, value })
    }

    fn double_quote(&self, state: &mut State, tokens: &mut Vec<Token>) -> SyntaxResult<()> {
        let span = state.source.span();
        let mut buffer = Vec::new();
        let (kind, value) = loop {
            match state.source.read(3) {
                [b'$', b'{', ..] => {
                    state.source.skip(2);
                    state.enter(StackFrame::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, b"${".into());
                }
                [b'{', b'$', ..] => {
                    // Intentionally only consume the left brace.
                    state.source.next();
                    state.enter(StackFrame::Scripting);
                    break (TokenKind::LeftBrace, b"{".into());
                }
                [b'"', ..] => {
                    state.source.next();
                    state.replace(StackFrame::Scripting);
                    break (TokenKind::DoubleQuote, b'"'.into());
                }
                &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                    state.source.skip(2);
                    buffer.push(b);
                }
                &[b'\\', b'n', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\n');
                }
                &[b'\\', b'r', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\r');
                }
                &[b'\\', b't', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\t');
                }
                &[b'\\', b'v', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x0b');
                }
                &[b'\\', b'e', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x1b');
                }
                &[b'\\', b'f', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x0c');
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    state.source.skip(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source.current()
                    {
                        state.source.next();
                        hex.push(*b as char);
                    }

                    let b = u8::from_str_radix(&hex, 16).unwrap();
                    buffer.push(b);
                }
                &[b'\\', b'u', b'{'] => {
                    state.source.skip(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source.current()
                    {
                        state.source.next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || state.source.current() != Some(&b'}') {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    }
                    state.source.next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    };

                    if let Some(c) = char::from_u32(c) {
                        let mut tmp = [0; 4];
                        let bytes = c.encode_utf8(&mut tmp);
                        buffer.extend(bytes.as_bytes());
                    } else {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    state.source.skip(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = state.source.current() {
                        state.source.next();
                        octal.push(*b as char);
                    }
                    if let Some(b @ b'0'..=b'7') = state.source.current() {
                        state.source.next();
                        octal.push(*b as char);
                    }

                    if let Ok(b) = u8::from_str_radix(&octal, 8) {
                        buffer.push(b);
                    } else {
                        return Err(SyntaxError::InvalidOctalEscape(state.source.span()));
                    }
                }
                [b'$', ident_start!(), ..] => {
                    let mut var = state.source.read_and_skip(1).to_vec();
                    var.extend(self.consume_identifier(state));

                    match state.source.read(4) {
                        [b'[', ..] => state.enter(StackFrame::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            state.enter(StackFrame::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, var.into());
                }
                &[b, ..] => {
                    state.source.next();
                    buffer.push(b);
                }
                [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
            }
        };

        if !buffer.is_empty() {
            tokens.push(Token {
                kind: TokenKind::StringPart,
                span,
                value: buffer.into(),
            })
        }

        tokens.push(Token { kind, span, value });
        Ok(())
    }

    fn shell_exec(&self, state: &mut State, tokens: &mut Vec<Token>) -> SyntaxResult<()> {
        let span = state.source.span();
        let mut buffer = Vec::new();
        let (kind, value) = loop {
            match state.source.read(2) {
                [b'$', b'{'] => {
                    state.source.skip(2);
                    state.enter(StackFrame::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, b"${".into());
                }
                [b'{', b'$'] => {
                    // Intentionally only consume the left brace.
                    state.source.next();
                    state.enter(StackFrame::Scripting);
                    break (TokenKind::LeftBrace, b"{".into());
                }
                [b'`', ..] => {
                    state.source.next();
                    state.replace(StackFrame::Scripting);
                    break (TokenKind::Backtick, b"`".into());
                }
                [b'$', ident_start!()] => {
                    let mut var = state.source.read_and_skip(1).to_vec();
                    var.extend(self.consume_identifier(state));

                    match state.source.read(4) {
                        [b'[', ..] => state.enter(StackFrame::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            state.enter(StackFrame::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, var.into());
                }
                &[b, ..] => {
                    state.source.next();
                    buffer.push(b);
                }
                [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
            }
        };

        if !buffer.is_empty() {
            tokens.push(Token {
                kind: TokenKind::StringPart,
                span,
                value: buffer.into(),
            })
        }

        tokens.push(Token { kind, span, value });

        Ok(())
    }

    fn heredoc(
        &self,
        state: &mut State,
        tokens: &mut Vec<Token>,
        label: ByteString,
    ) -> SyntaxResult<()> {
        let span = state.source.span();
        let mut buffer: Vec<u8> = Vec::new();

        let (kind, value) = loop {
            match state.source.read(3) {
                [b'$', b'{', ..] => {
                    state.source.skip(2);
                    state.enter(StackFrame::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, b"${".into());
                }
                [b'{', b'$', ..] => {
                    // Intentionally only consume the left brace.
                    state.source.next();
                    state.enter(StackFrame::Scripting);
                    break (TokenKind::LeftBrace, b"{".into());
                }
                &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                    state.source.skip(2);
                    buffer.push(b);
                }
                &[b'\\', b'n', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\n');
                }
                &[b'\\', b'r', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\r');
                }
                &[b'\\', b't', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\t');
                }
                &[b'\\', b'v', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x0b');
                }
                &[b'\\', b'e', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x1b');
                }
                &[b'\\', b'f', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x0c');
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    state.source.skip(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source.current()
                    {
                        state.source.next();
                        hex.push(*b as char);
                    }

                    let b = u8::from_str_radix(&hex, 16).unwrap();
                    buffer.push(b);
                }
                &[b'\\', b'u', b'{'] => {
                    state.source.skip(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source.current()
                    {
                        state.source.next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || state.source.current() != Some(&b'}') {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    }
                    state.source.next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    };

                    if let Some(c) = char::from_u32(c) {
                        let mut tmp = [0; 4];
                        let bytes = c.encode_utf8(&mut tmp);
                        buffer.extend(bytes.as_bytes());
                    } else {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    state.source.skip(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = state.source.current() {
                        state.source.next();
                        octal.push(*b as char);
                    }
                    if let Some(b @ b'0'..=b'7') = state.source.current() {
                        state.source.next();
                        octal.push(*b as char);
                    }

                    if let Ok(b) = u8::from_str_radix(&octal, 8) {
                        buffer.push(b);
                    } else {
                        return Err(SyntaxError::InvalidOctalEscape(state.source.span()));
                    }
                }
                [b'$', ident_start!(), ..] => {
                    let mut var = state.source.read_and_skip(1).to_vec();
                    var.extend(self.consume_identifier(state));

                    match state.source.read(4) {
                        [b'[', ..] => state.enter(StackFrame::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            state.enter(StackFrame::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, var.into());
                }
                // If we find a new-line, we can start to check if we can see the EndHeredoc token.
                [b'\n', ..] => {
                    buffer.push(b'\n');
                    state.source.next();

                    // Check if we can see the closing label right here.
                    if state.source.at(&label, label.len()) {
                        state.source.skip(label.len());
                        state.replace(StackFrame::Scripting);
                        break (
                            TokenKind::EndDocString(DocStringIndentationKind::None, 0),
                            label,
                        );
                    }

                    // Check if there's any whitespace first.
                    let (whitespace_kind, whitespace_amount) = match state.source.read(1) {
                        [b' '] => {
                            let mut amount = 0;
                            while state.source.read(1) == [b' '] {
                                amount += 1;
                                state.source.next();
                            }
                            (DocStringIndentationKind::Space, amount)
                        }
                        [b'\t'] => {
                            let mut amount = 0;
                            while state.source.read(1) == [b'\t'] {
                                amount += 1;
                                state.source.next();
                            }
                            (DocStringIndentationKind::Tab, amount)
                        }
                        _ => (DocStringIndentationKind::None, 0),
                    };

                    // We've figured out what type of whitespace was being used
                    // at the start of the line.
                    // We should now check for any extra whitespace, of any kind.
                    let mut extra_whitespace_buffer = Vec::new();
                    while let [b @ b' ' | b @ b'\t'] = state.source.read(1) {
                        extra_whitespace_buffer.push(b);
                        state.source.next();
                    }

                    // We've consumed all leading whitespace on this line now,
                    // so let's try to read the label again.
                    if state.source.at(&label, label.len()) {
                        // We've found the label, finally! We need to do 1 last
                        // check to make sure there wasn't a mixture of indentation types.
                        if whitespace_kind != DocStringIndentationKind::None
                            && !extra_whitespace_buffer.is_empty()
                        {
                            return Err(SyntaxError::InvalidDocIndentation(state.source.span()));
                        }

                        // If we get here, only 1 type of indentation was found. We can move
                        // the process along by reading over the label and breaking out
                        // with the EndHeredoc token, storing the kind and amount of whitespace.
                        state.source.skip(label.len());
                        state.replace(StackFrame::Scripting);
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
                    state.source.next();
                    buffer.push(b);
                }
                [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
            }
        };

        // Any trailing line breaks should be removed from the final heredoc.
        if buffer.last() == Some(&b'\n') {
            buffer.pop();
        }

        if !buffer.is_empty() {
            tokens.push(Token {
                kind: TokenKind::StringPart,
                span,
                value: buffer.into(),
            })
        }

        tokens.push(Token { kind, span, value });

        Ok(())
    }

    fn nowdoc(
        &self,
        state: &mut State,
        tokens: &mut Vec<Token>,
        label: ByteString,
    ) -> SyntaxResult<()> {
        let span = state.source.span();
        let mut buffer: Vec<u8> = Vec::new();

        let (kind, value) = loop {
            match state.source.read(3) {
                // If we find a new-line, we can start to check if we can see the EndHeredoc token.
                [b'\n', ..] => {
                    buffer.push(b'\n');
                    state.source.next();

                    // Check if we can see the closing label right here.
                    if state.source.at(&label, label.len()) {
                        state.source.skip(label.len());
                        state.replace(StackFrame::Scripting);
                        break (
                            TokenKind::EndDocString(DocStringIndentationKind::None, 0),
                            label,
                        );
                    }

                    // Check if there's any whitespace first.
                    let (whitespace_kind, whitespace_amount) = match state.source.read(1) {
                        [b' '] => {
                            let mut amount = 0;
                            while state.source.read(1) == [b' '] {
                                amount += 1;
                                state.source.next();
                            }
                            (DocStringIndentationKind::Space, amount)
                        }
                        [b'\t'] => {
                            let mut amount = 0;
                            while state.source.read(1) == [b'\t'] {
                                amount += 1;
                                state.source.next();
                            }
                            (DocStringIndentationKind::Tab, amount)
                        }
                        _ => (DocStringIndentationKind::None, 0),
                    };

                    // We've figured out what type of whitespace was being used
                    // at the start of the line.
                    // We should now check for any extra whitespace, of any kind.
                    let mut extra_whitespace_buffer = Vec::new();
                    while let [b @ b' ' | b @ b'\t'] = state.source.read(1) {
                        extra_whitespace_buffer.push(b);
                        state.source.next();
                    }

                    // We've consumed all leading whitespace on this line now,
                    // so let's try to read the label again.
                    if state.source.at(&label, label.len()) {
                        // We've found the label, finally! We need to do 1 last
                        // check to make sure there wasn't a mixture of indentation types.
                        if whitespace_kind != DocStringIndentationKind::None
                            && !extra_whitespace_buffer.is_empty()
                        {
                            return Err(SyntaxError::InvalidDocIndentation(state.source.span()));
                        }

                        // If we get here, only 1 type of indentation was found. We can move
                        // the process along by reading over the label and breaking out
                        // with the EndHeredoc token, storing the kind and amount of whitespace.
                        state.source.skip(label.len());
                        state.replace(StackFrame::Scripting);
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
                    state.source.next();
                    buffer.push(b);
                }
                [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
            }
        };

        // Any trailing line breaks should be removed from the final heredoc.
        if buffer.last() == Some(&b'\n') {
            buffer.pop();
        }

        if !buffer.is_empty() {
            tokens.push(Token {
                kind: TokenKind::StringPart,
                span,
                value: buffer.into(),
            })
        }

        tokens.push(Token { kind, span, value });

        Ok(())
    }

    fn looking_for_varname(&self, state: &mut State) -> SyntaxResult<Option<Token>> {
        let identifier = self.peek_identifier(state);

        if let Some(ident) = identifier {
            if let [b'[' | b'}'] = state.source.peek(ident.len(), 1) {
                let ident = ident.to_vec();
                let span = state.source.span();
                state.source.skip(ident.len());
                state.replace(StackFrame::Scripting);
                return Ok(Some(Token {
                    kind: TokenKind::Identifier,
                    span,
                    value: ident.into(),
                }));
            }
        }

        state.replace(StackFrame::Scripting);

        Ok(None)
    }

    fn looking_for_property(&self, state: &mut State) -> SyntaxResult<Token> {
        let span = state.source.span();
        let (kind, value) = match state.source.read(3) {
            [b'?', b'-', b'>'] => {
                state.source.skip(3);
                (TokenKind::QuestionArrow, b"?->".into())
            }
            [b'-', b'>', ..] => {
                state.source.skip(2);
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

        Ok(Token { kind, span, value })
    }

    fn var_offset(&self, state: &mut State) -> SyntaxResult<Token> {
        let span = state.source.span();
        let (kind, value) = match state.source.read(2) {
            [b'$', ident_start!()] => self.tokenize_variable(state),
            [b'0'..=b'9', ..] => {
                // TODO: all integer literals are allowed, but only decimal integers with no underscores
                // are actually treated as numbers. Others are treated as strings.
                // Float literals are not allowed, but that could be handled in the parser.
                self.tokenize_number(state)?
            }
            [b'[', ..] => {
                state.source.next();
                (TokenKind::LeftBracket, b"[".into())
            }
            [b'-', ..] => {
                state.source.next();
                (TokenKind::Minus, b"-".into())
            }
            [b']', ..] => {
                state.source.next();
                state.exit();
                (TokenKind::RightBracket, b"]".into())
            }
            &[ident_start!(), ..] => {
                let label = self.consume_identifier(state);
                (TokenKind::Identifier, label.into())
            }
            &[b, ..] => return Err(SyntaxError::UnrecognisedToken(b, state.source.span())),
            [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
        };
        Ok(Token { kind, span, value })
    }

    fn tokenize_single_quote_string(
        &self,
        state: &mut State,
    ) -> SyntaxResult<(TokenKind, ByteString)> {
        let mut buffer = vec![];

        loop {
            match state.source.read(2) {
                [b'\'', ..] => {
                    state.source.next();
                    break;
                }
                &[b'\\', b @ b'\'' | b @ b'\\'] => {
                    state.source.skip(2);
                    buffer.push(b);
                }
                &[b, ..] => {
                    state.source.next();
                    buffer.push(b);
                }
                [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
            }
        }

        Ok((TokenKind::LiteralSingleQuotedString, buffer.into()))
    }

    fn tokenize_double_quote_string(
        &self,
        state: &mut State,
    ) -> SyntaxResult<(TokenKind, ByteString)> {
        let mut buffer = vec![];

        let constant = loop {
            match state.source.read(3) {
                [b'"', ..] => {
                    state.source.next();
                    break true;
                }
                &[b'\\', b @ (b'"' | b'\\' | b'$'), ..] => {
                    state.source.skip(2);
                    buffer.push(b);
                }
                &[b'\\', b'n', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\n');
                }
                &[b'\\', b'r', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\r');
                }
                &[b'\\', b't', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\t');
                }
                &[b'\\', b'v', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x0b');
                }
                &[b'\\', b'e', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x1b');
                }
                &[b'\\', b'f', ..] => {
                    state.source.skip(2);
                    buffer.push(b'\x0c');
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    state.source.skip(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source.current()
                    {
                        state.source.next();
                        hex.push(*b as char);
                    }

                    let b = u8::from_str_radix(&hex, 16).unwrap();
                    buffer.push(b);
                }
                &[b'\\', b'u', b'{'] => {
                    state.source.skip(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        state.source.current()
                    {
                        state.source.next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || state.source.current() != Some(&b'}') {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    }
                    state.source.next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    };

                    if let Some(c) = char::from_u32(c) {
                        let mut tmp = [0; 4];
                        let bytes = c.encode_utf8(&mut tmp);
                        buffer.extend(bytes.as_bytes());
                    } else {
                        return Err(SyntaxError::InvalidUnicodeEscape(state.source.span()));
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    state.source.skip(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = state.source.current() {
                        state.source.next();
                        octal.push(*b as char);
                    }

                    if let Some(b @ b'0'..=b'7') = state.source.current() {
                        state.source.next();
                        octal.push(*b as char);
                    }

                    if let Ok(b) = u8::from_str_radix(&octal, 8) {
                        buffer.push(b);
                    } else {
                        return Err(SyntaxError::InvalidOctalEscape(state.source.span()));
                    }
                }
                [b'$', ident_start!(), ..] | [b'{', b'$', ..] | [b'$', b'{', ..] => {
                    break false;
                }
                &[b, ..] => {
                    state.source.next();
                    buffer.push(b);
                }
                [] => return Err(SyntaxError::UnexpectedEndOfFile(state.source.span())),
            }
        };

        Ok(if constant {
            (TokenKind::LiteralDoubleQuotedString, buffer.into())
        } else {
            state.replace(StackFrame::DoubleQuote);
            (TokenKind::StringPart, buffer.into())
        })
    }

    fn peek_identifier<'a>(&'a self, state: &'a State) -> Option<&'a [u8]> {
        let mut size = 0;

        if let [ident_start!()] = state.source.read(1) {
            size += 1;
            while let [ident!()] = state.source.peek(size, 1) {
                size += 1;
            }

            Some(state.source.read(size))
        } else {
            None
        }
    }

    fn consume_identifier(&self, state: &mut State) -> Vec<u8> {
        let ident = self.peek_identifier(state).unwrap().to_vec();
        state.source.skip(ident.len());

        ident
    }

    fn tokenize_variable(&self, state: &mut State) -> (TokenKind, ByteString) {
        let mut var = state.source.read_and_skip(1).to_vec();
        var.extend(self.consume_identifier(state));
        (TokenKind::Variable, var.into())
    }

    fn tokenize_number(&self, state: &mut State) -> SyntaxResult<(TokenKind, ByteString)> {
        let mut buffer = Vec::new();

        let (base, kind) = match state.source.read(2) {
            [a @ b'0', b @ b'B' | b @ b'b'] => {
                buffer.push(*a);
                buffer.push(*b);
                state.source.skip(2);
                (2, NumberKind::Int)
            }
            [a @ b'0', b @ b'O' | b @ b'o'] => {
                buffer.push(*a);
                buffer.push(*b);
                state.source.skip(2);
                (8, NumberKind::Int)
            }
            [a @ b'0', b @ b'X' | b @ b'x'] => {
                buffer.push(*a);
                buffer.push(*b);
                state.source.skip(2);
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
            state.source.read(3),
            [b'.', ..] | [b'e' | b'E', b'-' | b'+', b'0'..=b'9'] | [b'e' | b'E', b'0'..=b'9', ..]
        );

        if !is_float {
            return parse_int(&buffer);
        }

        if let Some(b'.') = state.source.current() {
            buffer.push(b'.');
            state.source.next();
            self.read_digits(state, &mut buffer, 10);
        }

        if let Some(b'e' | b'E') = state.source.current() {
            buffer.push(b'e');
            state.source.next();
            if let Some(b @ (b'-' | b'+')) = state.source.current() {
                buffer.push(*b);
                state.source.next();
            }
            self.read_digits(state, &mut buffer, 10);
        }

        Ok((TokenKind::LiteralFloat, buffer.into()))
    }

    fn read_digits(&self, state: &mut State, buffer: &mut Vec<u8>, base: usize) {
        if base == 16 {
            self.read_digits_fn(state, buffer, u8::is_ascii_hexdigit);
        } else {
            let max = b'0' + base as u8;
            self.read_digits_fn(state, buffer, |b| (b'0'..max).contains(b));
        };
    }

    fn read_digits_fn<F: Fn(&u8) -> bool>(
        &self,
        state: &mut State,
        buffer: &mut Vec<u8>,
        is_digit: F,
    ) {
        if let Some(b) = state.source.current() {
            if is_digit(b) {
                state.source.next();
                buffer.push(*b);
            } else {
                return;
            }
        }

        loop {
            match state.source.read(2) {
                [b, ..] if is_digit(b) => {
                    state.source.next();
                    buffer.push(*b);
                }
                [b'_', b] if is_digit(b) => {
                    state.source.next();
                    state.source.next();
                    buffer.push(*b);
                }
                _ => {
                    break;
                }
            }
        }
    }
}

// Parses an integer literal in the given base and converts errors to SyntaxError.
// It returns a float token instead on overflow.
fn parse_int(buffer: &[u8]) -> SyntaxResult<(TokenKind, ByteString)> {
    Ok((TokenKind::LiteralInteger, buffer.into()))
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
        b"__dir__" => TokenKind::DirConstant,
        b"__file__" => TokenKind::FileConstant,
        b"__line__" => TokenKind::LineConstant,
        b"__function__" => TokenKind::FunctionConstant,
        b"__class__" => TokenKind::ClassConstant,
        b"__method__" => TokenKind::MethodConstant,
        b"__trait__" => TokenKind::TraitConstant,
        b"__namespace__" => TokenKind::NamespaceConstant,
        b"__compiler_halt_offset__" => TokenKind::CompilerHaltOffsetConstant,
        b"while" => TokenKind::While,
        b"insteadof" => TokenKind::Insteadof,
        b"list" => TokenKind::List,
        b"self" => TokenKind::Self_,
        b"parent" => TokenKind::Parent,
        _ => return None,
    })
}

#[derive(Debug, Eq, PartialEq)]
enum NumberKind {
    Int,
    Float,
    IntOrFloat,
    OctalOrFloat,
}
