use std::collections::VecDeque;

use crate::source::Source;
use pxp_bytestring::ByteStr;
use pxp_bytestring::ByteString;

use pxp_span::Span;
use pxp_token::OpenTagKind;
use pxp_token::OwnedToken;
use pxp_token::Token;
use pxp_token::TokenKind;

pub mod error;
pub mod macros;
pub mod source;

#[derive(Debug)]
pub struct Lexer<'a> {
    frames: VecDeque<StackFrame>,
    source: Source<'a>,

    current: Token<'a>,
    peek: Option<Token<'a>>,
    peek_again: Option<Token<'a>>,
}

#[derive(Debug)]
pub enum StackFrame {
    Initial,
    Scripting,
    Halted,
    DoubleQuote,
    ShellExec,
    DocString {
        kind: TokenKind,
        label: ByteString,
        expect_label: bool,
    },
    LookingForVarname,
    LookingForProperty,
    VarOffset,
    DocBlock,
}

impl<'a> Lexer<'a> {
    pub fn new<B: ?Sized + AsRef<[u8]>>(input: &'a B) -> Self {
        let mut this = Self {
            source: Source::new(input.as_ref()),
            frames: VecDeque::from([StackFrame::Initial]),

            current: Token::new(TokenKind::Eof, Span::default(), ByteStr::new(&[])),
            peek: None,
            peek_again: None,
        };

        this.next();
        this
    }

    pub fn new_in_immediate<B: ?Sized + AsRef<[u8]>>(input: &'a B) -> Self {
        let mut this = Self::new(input);

        this.replace(StackFrame::Scripting);
        this
    }

    pub fn collect(&'a mut self) -> Vec<OwnedToken> {
        let mut tokens = Vec::new();

        loop {
            let token = self.current();

            tokens.push(token.to_owned());

            if token.kind == TokenKind::Eof {
                break;
            }

            self.next();
        }

        tokens
    }

    pub fn current(&self) -> Token {
        self.current
    }

    pub fn peek(&mut self) -> Token {
        if self.peek.is_none() {
            self.peek = Some(self.read_next());
        }

        self.peek.unwrap()
    }

    pub fn peek_again(&mut self) -> Token {
        if self.peek_again.is_none() {
            self.peek_again = Some(self.read_next());
        }

        self.peek_again.unwrap()
    }

    pub fn set_peek(&mut self, token: Token<'a>) {
        self.peek = Some(token);
    }

    pub fn next(&mut self) {
        if self.peek.is_some() {
            self.current = self.peek.take().unwrap();
            self.peek = None;

            if self.peek_again.is_some() {
                self.peek = self.peek_again.take();
                self.peek_again = None;
            }

            return;
        }

        self.current = self.read_next();
    }

    fn read_next(&mut self) -> Token<'a> {
        if self.source.eof() {
            return Token::new_without_symbol(TokenKind::Eof, self.source.span());
        }

        self.source.start_token();

        match self.frame() {
            // The "Initial" state is used to parse inline HTML. It is essentially a catch-all
            // state that will build up a single token buffer until it encounters an open tag
            // of some description.
            StackFrame::Initial => self.initial().unwrap_or_else(|| self.scripting()),
            // The scripting state is entered when an open tag is encountered in the source code.
            // This tells the lexer to start analysing characters at PHP tokens instead of inline HTML.
            StackFrame::Scripting => {
                self.skip_whitespace();

                // If we have consumed whitespace, we should restart the token's position tracking
                // to ensure we accurately track the span of the token.
                self.source.start_token();

                // If we have consumed whitespace and then reached the end of the file, we should break.
                if self.source.eof() {
                    return Token::new_without_symbol(TokenKind::Eof, self.source.span());
                }

                self.scripting()
            }
            // The "Halted" state is entered when the `__halt_compiler` token is encountered.
            // In this state, all the text that follows is no longer parsed as PHP as is collected
            // into a single "InlineHtml" token (kind of cheating, oh well).
            StackFrame::Halted => {
                let symbol = self.source.read_remaining();

                Token::new(TokenKind::InlineHtml, self.source.span(), symbol)
            }
            // The double quote state is entered when inside a double-quoted string that
            // contains variables.
            StackFrame::DoubleQuote => self.double_quote(),
            // The shell exec state is entered when inside of a execution string (`).
            StackFrame::ShellExec => self.shell_exec(),
            // The doc string state is entered when tokenizing heredocs and nowdocs.
            StackFrame::DocString { kind, label, expect_label } => {
                let label = label.clone();

                match kind {
                    TokenKind::StartHeredoc => self.heredoc(label, *expect_label),
                    TokenKind::StartNowdoc => self.nowdoc(label, *expect_label),
                    _ => unreachable!(),
                }
            }
            // LookingForProperty is entered inside double quotes,
            // backticks, or a heredoc, expecting a variable name.
            // If one isn't found, it switches to scripting.
            StackFrame::LookingForVarname => {
                if let Some(token) = self.looking_for_varname() {
                    token
                } else {
                    self.scripting()
                }
            }
            // LookingForProperty is entered inside double quotes,
            // backticks, or a heredoc, expecting an arrow followed by a
            // property name.
            StackFrame::LookingForProperty => self.looking_for_property(),
            StackFrame::VarOffset => {
                if self.source.eof() {
                    Token::new_without_symbol(TokenKind::Eof, self.source.span())
                } else {
                    self.var_offset()
                }
            }
            // DocBlock is entered when parsing a DocBlock comment.
            // The lexer does this extra work to ensure that the comment
            // is in a usable state for the parser.
            StackFrame::DocBlock => self.docblock(),
        }
    }

    pub fn frame(&self) -> &StackFrame {
        self.frames
            .back()
            .unwrap_or_else(|| panic!("The lexer has reached an invalid state. This shouldn't happen, but somehow it has."))
    }

    pub fn frame_mut(&mut self) -> &mut StackFrame {
        self.frames
            .back_mut()
            .unwrap_or_else(|| panic!("The lexer has reached an invalid state. This shouldn't happen, but somehow it has."))
    }

    pub fn replace(&mut self, state: StackFrame) {
        let i = self.frames.len() - 1;

        self.frames[i] = state;
    }

    pub fn enter(&mut self, state: StackFrame) {
        self.frames.push_back(state);
    }

    pub fn exit(&mut self) {
        self.frames.pop_back();
    }

    // pub fn tokenize(&'a mut self) -> SyntaxResult<Vec<Token<'a>>> {
    //     let mut tokens = Vec::new();

    //     while !self.source.eof() {
    //         self.source.start_token();

    //         match self.frame()? {
    //             // The "Initial" state is used to parse inline HTML. It is essentially a catch-all
    //             // state that will build up a single token buffer until it encounters an open tag
    //             // of some description.
    //             StackFrame::Initial => self.initial(&mut tokens)?,
    //             // The scripting state is entered when an open tag is encountered in the source code.
    //             // This tells the lexer to start analysing characters at PHP tokens instead of inline HTML.
    //             StackFrame::Scripting => {
    //                 self.skip_whitespace();

    //                 // If we have consumed whitespace, we should restart the token's position tracking
    //                 // to ensure we accurately track the span of the token.
    //                 self.source.start_token();

    //                 // If we have consumed whitespace and then reached the end of the file, we should break.
    //                 if self.source.eof() {
    //                     break;
    //                 }

    //                 tokens.push(self.scripting()?);
    //             }
    //             // The "Halted" state is entered when the `__halt_compiler` token is encountered.
    //             // In this state, all the text that follows is no longer parsed as PHP as is collected
    //             // into a single "InlineHtml" token (kind of cheating, oh well).
    //             StackFrame::Halted => {
    //                 let symbol = self.source.read_remaining();

    //                 tokens.push(Token::new(
    //                     TokenKind::InlineHtml,
    //                     self.source.span(),
    //                     symbol,
    //                 ));
    //                 break;
    //             }
    //             // The double quote state is entered when inside a double-quoted string that
    //             // contains variables.
    //             StackFrame::DoubleQuote => self.double_quote(&mut tokens)?,
    //             // The shell exec state is entered when inside of a execution string (`).
    //             StackFrame::ShellExec => self.shell_exec(&mut tokens)?,
    //             // The doc string state is entered when tokenizing heredocs and nowdocs.
    //             StackFrame::DocString(kind, label) => {
    //                 let label = label.clone();

    //                 match kind {
    //                     TokenKind::StartHeredoc => self.heredoc(&mut tokens, label)?,
    //                     TokenKind::StartNowdoc => self.nowdoc(&mut tokens, label)?,
    //                     _ => unreachable!(),
    //                 }
    //             }
    //             // LookingForProperty is entered inside double quotes,
    //             // backticks, or a heredoc, expecting a variable name.
    //             // If one isn't found, it switches to scripting.
    //             StackFrame::LookingForVarname => {
    //                 if let Some(token) = self.looking_for_varname()? {
    //                     tokens.push(token);
    //                 }
    //             }
    //             // LookingForProperty is entered inside double quotes,
    //             // backticks, or a heredoc, expecting an arrow followed by a
    //             // property name.
    //             StackFrame::LookingForProperty => {
    //                 tokens.push(self.looking_for_property()?);
    //             }
    //             StackFrame::VarOffset => {
    //                 if self.source.eof() {
    //                     break;
    //                 }

    //                 tokens.push(self.var_offset()?);
    //             }
    //             // DocBlock is entered when parsing a DocBlock comment.
    //             // The lexer does this extra work to ensure that the comment
    //             // is in a usable state for the parser.
    //             StackFrame::DocBlock => self.docblock(&mut tokens)?,
    //         }
    //     }

    //     tokens.push(Token::new_without_symbol(
    //         TokenKind::Eof,
    //         self.source.span(),
    //     ));

    //     Ok(tokens)
    // }

    fn skip_horizontal_whitespace(&mut self) {
        while let Some(true) = self
            .source
            .current()
            .map(|u: &u8| u == &b' ' || u == &b'\t')
        {
            self.source.next();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(true) = self.source.current().map(|u: &u8| u.is_ascii_whitespace()) {
            self.source.next();
        }
    }

    fn read_and_skip_whitespace(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        while let Some(true) = self.source.current().map(|u: &u8| u.is_ascii_whitespace()) {
            buffer.push(*self.source.current().unwrap());
            self.source.next();
        }
        buffer
    }

    fn docblock_eol(&mut self) -> Token<'a> {
        // We've already skipped the line break at this point.
        // We need to consume horizontal whitespace.
        self.skip_horizontal_whitespace();

        if matches!(self.source.current(), Some(b'*'))
            && !matches!(self.source.read(2), [b'*', b'/', ..])
        {
            self.source.next();

            // We also want to skip the next space character here.
            if let Some(b' ') = self.source.current() {
                self.source.next();
            }
        }

        let span = self.source.span();
        let symbol = self.source.span_range(span);

        Token::new(TokenKind::PhpDocEol, span, symbol)
    }

    fn docblock(&mut self) -> Token<'a> {
        self.source.start_token();

        if matches!(self.source.read(2), [b'\r', b'\n', ..] | [b'\n', ..]) {
            let b = self.source.current().unwrap();

            if b == &b'\r' {
                self.source.skip(2);
            } else {
                self.source.skip(1);
            }

            return self.docblock_eol();
        }

        match &self.source.read(2) {
            [b'@', ident_start!(), ..] => {
                self.source.skip(2);

                while let Some(ident_start!() | b'\\') = self.source.current() {
                    self.source.next();
                }

                let span = self.source.span();
                let symbol = self.source.span_range(span);

                Token::new(TokenKind::PhpDocTag, span, symbol)
            }
            [b'$', ident_start!(), ..] => {
                let variable = self.tokenize_variable();
                let span = self.source.span();
                let symbol = self.source.span_range(span);

                Token::new(variable, span, symbol)
            }
            [b'\\', ident_start!(), ..] => {
                self.source.next();

                let mut span = self.source.span();

                let kind = match self.scripting() {
                    Token {
                        kind: TokenKind::Identifier | TokenKind::QualifiedIdentifier,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    Token {
                        kind: TokenKind::True,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    Token {
                        kind: TokenKind::False,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    Token {
                        kind: TokenKind::Null,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    s => unreachable!("{:?}", s),
                };

                Token::new(kind, span, self.source.span_range(span))
            }
            [b @ ident_start!(), ..] => {
                self.source.next();
                let mut qualified = false;
                let mut last_was_slash = false;

                let mut buffer = vec![*b];
                while let Some(next @ ident!() | next @ b'\\') = self.source.current() {
                    if matches!(next, ident!()) {
                        buffer.push(*next);
                        self.source.next();
                        last_was_slash = false;
                        continue;
                    }

                    if *next == b'\\' && !last_was_slash {
                        qualified = true;
                        last_was_slash = true;
                        buffer.push(*next);
                        self.source.next();
                        continue;
                    }

                    break;
                }

                let kind = if qualified {
                    TokenKind::QualifiedIdentifier
                } else {
                    identifier_to_keyword(&buffer).unwrap_or(TokenKind::Identifier)
                };

                let span = self.source.span();
                let symbol = self.source.span_range(span);

                Token::new(kind, span, symbol)
            }
            [b'|', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Pipe, span, self.source.span_range(span))
            }
            [b'&', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Ampersand, span, self.source.span_range(span))
            }
            [b'!', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Bang, span, self.source.span_range(span))
            }
            [b'?', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Question, span, self.source.span_range(span))
            }
            [b'(', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::LeftParen, span, self.source.span_range(span))
            }
            [b')', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::RightParen, span, self.source.span_range(span))
            }
            [b'[', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::LeftBracket, span, self.source.span_range(span))
            }
            [b']', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::RightBracket, span, self.source.span_range(span))
            }
            [b'{', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::LeftBrace, span, self.source.span_range(span))
            }
            [b'}', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::RightBrace, span, self.source.span_range(span))
            }
            [b'<', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::LessThan, span, self.source.span_range(span))
            }
            [b'>', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::GreaterThan, span, self.source.span_range(span))
            }
            [b'.', b'.', b'.', ..] => {
                self.source.skip(3);

                let span = self.source.span();

                Token::new(TokenKind::Ellipsis, span, self.source.span_range(span))
            }
            [b'=', b'>', ..] => {
                self.source.skip(2);

                let span = self.source.span();

                Token::new(TokenKind::DoubleArrow, span, self.source.span_range(span))
            }
            [b'-', b'>', ..] => {
                self.source.skip(2);

                let span = self.source.span();

                Token::new(TokenKind::Arrow, span, self.source.span_range(span))
            }
            [b'=', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Equals, span, self.source.span_range(span))
            }
            [b':', b':', ..] => {
                self.source.skip(2);

                let span = self.source.span();

                Token::new(TokenKind::DoubleColon, span, self.source.span_range(span))
            }
            [b':', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Colon, span, self.source.span_range(span))
            }
            [b',', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Comma, span, self.source.span_range(span))
            }
            [b'0'..=b'9', ..] => {
                let number = self.tokenize_number();
                let span = self.source.span();
                let symbol = self.source.span_range(span);

                Token::new(number, span, symbol)
            }
            // We only need to consider these things strings if they are closed before the end of the line.
            [b'\'', ..] => {
                // First we can grab the current offset, in case we need to backtrack.
                let offset = self.source.offset();

                self.source.next();

                let is_single_quoted_string = loop {
                    let Some(c) = self.source.current() else {
                        break false;
                    };

                    // If we encounter a single quote, we can break out of the loop since we've found the end of the string.
                    if *c == b'\'' {
                        self.source.next();
                        break true;
                    }

                    // If we encounter the end of a line, we need to backtrack and treat the single quote as a single character.
                    if *c == b'\n' {
                        break false;
                    }

                    self.source.next();
                };

                if is_single_quoted_string {
                    let span = self.source.span();
                    let symbol = self.source.span_range(span);

                    Token::new(TokenKind::LiteralSingleQuotedString, span, symbol)
                } else {
                    self.source.goto(offset);
                    self.source.next();

                    let span = self.source.span();
                    let symbol = self.source.span_range(span);

                    Token::new(TokenKind::PhpDocOther, span, symbol)
                }
            }
            [b'"', ..] => {
                let offset = self.source.offset();

                self.source.next();

                let is_single_quoted_string = loop {
                    let Some(c) = self.source.current() else {
                        break false;
                    };

                    // If we encounter a single quote, we can break out of the loop since we've found the end of the string.
                    if *c == b'"' {
                        self.source.next();
                        break true;
                    }

                    // If we encounter the end of a line, we need to backtrack and treat the single quote as a single character.
                    if *c == b'\n' {
                        break false;
                    }

                    self.source.next();
                };

                if is_single_quoted_string {
                    let span = self.source.span();
                    let symbol = self.source.span_range(span);

                    Token::new(TokenKind::LiteralDoubleQuotedString, span, symbol)
                } else {
                    self.source.goto(offset);
                    self.source.next();

                    let span = self.source.span();
                    let symbol = self.source.span_range(span);

                    Token::new(TokenKind::PhpDocOther, span, symbol)
                }
            }
            [b'*', b'/', ..] => {
                self.source.skip(2);
                self.exit();

                let span = self.source.span();

                Token::new(
                    TokenKind::ClosePhpDoc,
                    self.source.span(),
                    self.source.span_range(span),
                )
            }
            [b'*', ..] => {
                self.source.next();

                let span = self.source.span();

                Token::new(TokenKind::Asterisk, span, self.source.span_range(span))
            }
            [b' ' | b'\t', ..] => {
                self.skip_horizontal_whitespace();

                let span = self.source.span();
                let symbol = self.source.span_range(span);

                Token::new(TokenKind::PhpDocHorizontalWhitespace, span, symbol)
            }
            _ => {
                self.source.next();

                let span = self.source.span();
                let symbol = self.source.span_range(span);

                Token::new(TokenKind::PhpDocOther, span, symbol)
            }
        }
    }

    fn initial(&mut self) -> Option<Token<'a>> {
        while self.source.current().is_some() {
            if self.source.at_case_insensitive(b"<?php", 5)
                || self.source.at_case_insensitive(b"<?=", 3)
                || self.source.at_case_insensitive(b"<?", 2)
            {
                let inline_span = self.source.span();

                self.replace(StackFrame::Scripting);

                if !inline_span.is_empty() {
                    return Some(Token::new(
                        TokenKind::InlineHtml,
                        inline_span,
                        self.source.span_range(inline_span),
                    ));
                } else {
                    return None;
                }
            }

            self.source.next();
        }

        let inline_span = self.source.span();

        Some(Token::new(
            TokenKind::InlineHtml,
            inline_span,
            self.source.span_range(inline_span),
        ))
    }

    fn scripting(&mut self) -> Token<'a> {
        if &self.source.read(5) == b"<?php" {
            self.source.skip(5);

            let span = self.source.span();

            return Token::new(
                TokenKind::OpenTag(OpenTagKind::Full),
                span,
                self.source.span_range(span),
            );
        }

        let kind = match self.source.read(3) {
            [b'!', b'=', b'='] => {
                self.source.skip(3);

                TokenKind::BangDoubleEquals
            }
            [b'?', b'?', b'='] => {
                self.source.skip(3);
                TokenKind::DoubleQuestionEquals
            }
            [b'?', b'-', b'>'] => {
                self.source.skip(3);
                TokenKind::QuestionArrow
            }
            [b'=', b'=', b'='] => {
                self.source.skip(3);
                TokenKind::TripleEquals
            }
            [b'.', b'.', b'.'] => {
                self.source.skip(3);
                TokenKind::Ellipsis
            }
            [b'`', ..] => {
                self.source.next();
                self.replace(StackFrame::ShellExec);
                TokenKind::Backtick
            }
            [b'@', ..] => {
                self.source.next();
                TokenKind::At
            }
            [b'!', b'=', ..] => {
                self.source.skip(2);
                TokenKind::BangEquals
            }
            [b'!', ..] => {
                self.source.next();
                TokenKind::Bang
            }
            [b'&', b'&', ..] => {
                self.source.skip(2);
                TokenKind::BooleanAnd
            }
            [b'&', b'=', ..] => {
                self.source.skip(2);
                TokenKind::AmpersandEquals
            }
            [b'&', ..] => {
                self.source.next();
                TokenKind::Ampersand
            }
            [b'?', b'>', ..] => {
                // This is a close tag, we can enter "Initial" mode again.
                self.source.skip(2);

                self.replace(StackFrame::Initial);

                TokenKind::CloseTag
            }
            [b'?', b'?', ..] => {
                self.source.skip(2);
                TokenKind::DoubleQuestion
            }
            [b'?', b':', ..] => {
                self.source.skip(2);
                TokenKind::QuestionColon
            }
            [b'?', ..] => {
                self.source.next();
                TokenKind::Question
            }
            [b'=', b'>', ..] => {
                self.source.skip(2);
                TokenKind::DoubleArrow
            }
            [b'=', b'=', ..] => {
                self.source.skip(2);
                TokenKind::DoubleEquals
            }
            [b'=', ..] => {
                self.source.next();
                TokenKind::Equals
            }
            // Single quoted string.
            [b'\'', ..] => {
                self.source.skip(1);
                self.tokenize_single_quote_string()
            }
            [b'b' | b'B', b'\'', ..] => {
                self.source.skip(2);
                self.tokenize_single_quote_string()
            }
            [b'"', ..] => {
                self.source.skip(1);
                self.tokenize_double_quote_string()
            }
            [b'b' | b'B', b'"', ..] => {
                self.source.skip(2);
                self.tokenize_double_quote_string()
            }
            [b'$', ident_start!(), ..] => self.tokenize_variable(),
            [b'$', ..] => {
                self.source.next();
                TokenKind::Dollar
            }
            [b'.', b'=', ..] => {
                self.source.skip(2);
                TokenKind::DotEquals
            }
            [b'0'..=b'9', ..] => self.tokenize_number(),
            [b'.', b'0'..=b'9', ..] => self.tokenize_number(),
            [b'.', ..] => {
                self.source.next();
                TokenKind::Dot
            }
            [b'\\', ident_start!(), ..] => {
                self.source.next();

                let mut span = self.source.span();

                match self.scripting() {
                    Token {
                        kind: TokenKind::Identifier | TokenKind::QualifiedIdentifier,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    Token {
                        kind: TokenKind::True,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    Token {
                        kind: TokenKind::False,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    Token {
                        kind: TokenKind::Null,
                        span: ident_span,
                        ..
                    } => {
                        span.end = ident_span.end;

                        TokenKind::FullyQualifiedIdentifier
                    }
                    s => unreachable!("{:?}", s),
                }
            }
            [b'\\', ..] => {
                self.source.next();
                TokenKind::NamespaceSeparator
            }
            [b'/', b'*', ..] => {
                self.source.next();

                let mut kind = TokenKind::MultiLineComment;
                #[allow(unused)]
                let mut with_symbol = true;

                loop {
                    match self.source.read(2) {
                        #[cfg(feature = "docblocks")]
                        [b'*', b'*', ..] => {
                            self.source.skip(2);

                            kind = TokenKind::OpenPhpDoc;
                            with_symbol = false;

                            self.enter(StackFrame::DocBlock);

                            break;
                        }
                        [b'*', b'/', ..] => {
                            self.source.skip(2);
                            break;
                        }
                        #[cfg(not(feature = "docblocks"))]
                        [b'*', b'*', ..] if kind != TokenKind::DocBlockComment => {
                            self.source.skip(2);
                            kind = TokenKind::DocBlockComment;
                        }
                        &[..] => {
                            self.source.next();
                        }
                    }
                }

                kind
            }
            [b'#', b'[', ..] => {
                self.source.skip(2);
                TokenKind::Attribute
            }
            [ch @ b'/', b'/', ..] | [ch @ b'#', ..] => {
                let kind = if *ch == b'/' {
                    self.source.skip(2);
                    TokenKind::SingleLineComment
                } else {
                    self.source.next();
                    TokenKind::HashMarkComment
                };

                while let Some(c) = self.source.current() {
                    if *c == b'\n' {
                        self.source.next();
                        break;
                    }

                    if self.source.read(2) == [b'?', b'>'] {
                        break;
                    }

                    self.source.next();
                }

                kind
            }
            [b'/', b'=', ..] => {
                self.source.skip(2);
                TokenKind::SlashEquals
            }
            [b'/', ..] => {
                self.source.next();
                TokenKind::Slash
            }
            [b'*', b'*', b'=', ..] => {
                self.source.skip(3);
                TokenKind::PowEquals
            }
            [b'<', b'<', b'='] => {
                self.source.skip(3);

                TokenKind::LeftShiftEquals
            }
            [b'<', b'=', b'>'] => {
                self.source.skip(3);
                TokenKind::Spaceship
            }
            [b'>', b'>', b'='] => {
                self.source.skip(3);
                TokenKind::RightShiftEquals
            }
            [b'<', b'<', b'<'] => {
                self.source.skip(3);
                let mut buffer = b"<<<".to_vec();
                buffer.extend(self.read_and_skip_whitespace());

                let kind = match self.source.read(1) {
                    [b'\''] => {
                        buffer.push(b'\'');
                        self.source.next();
                        TokenKind::StartNowdoc
                    }
                    [b'"'] => {
                        buffer.push(b'"');
                        self.source.next();
                        TokenKind::StartHeredoc
                    }
                    [_, ..] => TokenKind::StartHeredoc,
                    [] => {
                        // FIXME: Push diagnostics for unexpected end of file.
                        todo!()
                    }
                };

                let label: ByteString = match self.peek_identifier() {
                    Some(_) => self.consume_identifier().into(),
                    None => {
                        #[allow(unreachable_code)]
                        return match self.source.current() {
                            Some(_c) => {
                                // FIXME: Push diagnostics for unexpected character.
                                todo!()
                            }
                            // FIXME: Push diagnostics for unexpected end of file.
                            None => todo!(),
                        };
                    }
                };

                buffer.extend_from_slice(&label);

                if kind == TokenKind::StartNowdoc {
                    match self.source.current() {
                        Some(b'\'') => {
                            buffer.push(b'\'');
                            self.source.next();
                        }
                        _ => {
                            // FIXME: Push diagnostics for unexpected character / no character.
                            todo!()
                        }
                    };
                } else if let Some(b'"') = self.source.current() {
                    buffer.push(b'"');
                    self.source.next();
                }

                if !matches!(self.source.current(), Some(b'\n')) {
                    // FIXME: Push diagnostics for unexpected character.
                    todo!()
                }

                self.source.next();
                self.replace(StackFrame::DocString { kind, label: label.clone(), expect_label: false });

                kind
            }
            [b'*', b'*', ..] => {
                self.source.skip(2);
                TokenKind::Pow
            }
            [b'*', b'=', ..] => {
                self.source.skip(2);
                TokenKind::AsteriskEquals
            }
            [b'*', ..] => {
                self.source.next();
                TokenKind::Asterisk
            }
            [b'|', b'|', ..] => {
                self.source.skip(2);
                TokenKind::BooleanOr
            }
            [b'|', b'=', ..] => {
                self.source.skip(2);
                TokenKind::PipeEquals
            }
            [b'|', ..] => {
                self.source.next();
                TokenKind::Pipe
            }
            [b'^', b'=', ..] => {
                self.source.skip(2);
                TokenKind::CaretEquals
            }
            [b'^', ..] => {
                self.source.next();
                TokenKind::Caret
            }
            [b'{', ..] => {
                self.source.next();
                self.enter(StackFrame::Scripting);
                TokenKind::LeftBrace
            }
            [b'}', ..] => {
                self.source.next();
                self.exit();
                TokenKind::RightBrace
            }
            [b'(', ..] => {
                self.source.next();

                // Inlined so we can add whitespace to the buffer.
                while let Some(true) = self.source.current().map(|u: &u8| u.is_ascii_whitespace()) {
                    self.source.next();
                }

                if self.source.at_case_insensitive(b"int", 3) {
                    if self.source.at_case_insensitive(b"integer", 7)
                        && self.source.peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        self.source.read_and_skip(7);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::IntegerCast
                    } else if self.source.peek_ignoring_whitespace(3, 1) == [b')'] {
                        self.source.read_and_skip(3);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::IntCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"bool", 4) {
                    if self.source.at_case_insensitive(b"boolean", 7)
                        && self.source.peek_ignoring_whitespace(7, 1) == [b')']
                    {
                        self.source.read_and_skip(7);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::BooleanCast
                    } else if self.source.peek_ignoring_whitespace(4, 1) == [b')'] {
                        self.source.read_and_skip(4);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::BoolCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"float", 5) {
                    if self.source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        self.source.read_and_skip(5);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::FloatCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"double", 6) {
                    if self.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        self.source.read_and_skip(6);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::DoubleCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"real", 4) {
                    if self.source.peek_ignoring_whitespace(4, 1) == [b')'] {
                        self.source.read_and_skip(4);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::RealCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"string", 6) {
                    if self.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        self.source.read_and_skip(6);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::StringCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"binary", 6) {
                    if self.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        self.source.read_and_skip(6);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::BinaryCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"array", 5) {
                    if self.source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        self.source.read_and_skip(5);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::ArrayCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"object", 6) {
                    if self.source.peek_ignoring_whitespace(6, 1) == [b')'] {
                        self.source.read_and_skip(6);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::ObjectCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else if self.source.at_case_insensitive(b"unset", 5) {
                    if self.source.peek_ignoring_whitespace(5, 1) == [b')'] {
                        self.source.read_and_skip(5);
                        self.read_and_skip_whitespace();
                        self.source.read_and_skip(1);

                        TokenKind::UnsetCast
                    } else {
                        TokenKind::LeftParen
                    }
                } else {
                    TokenKind::LeftParen
                }
            }
            [b')', ..] => {
                self.source.next();
                TokenKind::RightParen
            }
            [b';', ..] => {
                self.source.next();
                TokenKind::SemiColon
            }
            [b'+', b'+', ..] => {
                self.source.skip(2);
                TokenKind::Increment
            }
            [b'+', b'=', ..] => {
                self.source.skip(2);
                TokenKind::PlusEquals
            }
            [b'+', ..] => {
                self.source.next();
                TokenKind::Plus
            }
            [b'%', b'=', ..] => {
                self.source.skip(2);
                TokenKind::PercentEquals
            }
            [b'%', ..] => {
                self.source.next();
                TokenKind::Percent
            }
            [b'-', b'-', ..] => {
                self.source.skip(2);
                TokenKind::Decrement
            }
            [b'-', b'>', ..] => {
                self.source.skip(2);
                TokenKind::Arrow
            }
            [b'-', b'=', ..] => {
                self.source.skip(2);
                TokenKind::MinusEquals
            }
            [b'-', ..] => {
                self.source.next();
                TokenKind::Minus
            }
            [b'<', b'<', ..] => {
                self.source.skip(2);
                TokenKind::LeftShift
            }
            [b'<', b'=', ..] => {
                self.source.skip(2);
                TokenKind::LessThanEquals
            }
            [b'<', b'>', ..] => {
                self.source.skip(2);
                TokenKind::AngledLeftRight
            }
            [b'<', ..] => {
                self.source.next();
                TokenKind::LessThan
            }
            [b'>', b'>', ..] => {
                self.source.skip(2);
                TokenKind::RightShift
            }
            [b'>', b'=', ..] => {
                self.source.skip(2);
                TokenKind::GreaterThanEquals
            }
            [b'>', ..] => {
                self.source.next();
                TokenKind::GreaterThan
            }
            [b',', ..] => {
                self.source.next();
                TokenKind::Comma
            }
            [b'[', ..] => {
                self.source.next();
                TokenKind::LeftBracket
            }
            [b']', ..] => {
                self.source.next();
                TokenKind::RightBracket
            }
            [b':', b':', ..] => {
                self.source.skip(2);
                TokenKind::DoubleColon
            }
            [b':', ..] => {
                self.source.next();
                TokenKind::Colon
            }
            [b'~', ..] => {
                self.source.next();
                TokenKind::BitwiseNot
            }
            [b @ ident_start!(), ..] => {
                self.source.next();
                let mut qualified = false;
                let mut last_was_slash = false;

                let mut buffer = vec![*b];
                while let Some(next @ ident!() | next @ b'\\') = self.source.current() {
                    if matches!(next, ident!()) {
                        buffer.push(*next);
                        self.source.next();
                        last_was_slash = false;
                        continue;
                    }

                    if *next == b'\\' && !last_was_slash {
                        qualified = true;
                        last_was_slash = true;
                        buffer.push(*next);
                        self.source.next();
                        continue;
                    }

                    break;
                }

                if qualified {
                    TokenKind::QualifiedIdentifier
                } else {
                    let kind = identifier_to_keyword(&buffer).unwrap_or(TokenKind::Identifier);

                    if kind == TokenKind::HaltCompiler {
                        match self.source.read(3) {
                            [b'(', b')', b';'] => {
                                self.source.skip(3);
                                self.replace(StackFrame::Halted);
                            }
                            // FIXME: Push diagnostics for invalid halt compiler.
                            _ => todo!(),
                        }
                    }

                    kind
                }
            }
            [b, ..] => unimplemented!(
                "<scripting> {} at offset: {}",
                *b as char,
                self.source.offset(),
            ),
            // We should never reach this point since we have the empty checks surrounding
            // the call to this function, but it's better to be safe than sorry.
            // FIXME: Push diagnostics for unexpected end of file.
            [] => todo!(),
        };

        let mut span = self.source.span();

        // FIXME: This is a bit hacky, but it works for now.
        //        We're doing this so that the closing double quote isn't included in the span.
        if kind == TokenKind::LiteralDoubleQuotedString {
            span.end -= 1;
        }

        Token::new(kind, span, self.source.span_range(span))
    }

    fn double_quote(&mut self) -> Token<'a> {
        #[allow(unused_assignments)]
        let mut buffer_span = None;

        let (kind, span) = loop {
            match self.source.read(3) {
                [b'$', b'{', ..] => {
                    buffer_span = Some(self.source.span());
                    self.source.start_token();
                    self.source.skip(2);
                    self.enter(StackFrame::LookingForVarname);
                    break (TokenKind::DollarLeftBrace, self.source.span());
                }
                [b'{', b'$', ..] => {
                    buffer_span = Some(self.source.span());
                    self.source.start_token();
                    // Intentionally only consume the left brace.
                    self.source.next();
                    self.enter(StackFrame::Scripting);
                    break (TokenKind::LeftBrace, self.source.span());
                }
                [b'"', ..] => {
                    buffer_span = Some(self.source.span());
                    self.source.start_token();
                    self.source.next();
                    self.replace(StackFrame::Scripting);
                    break (TokenKind::DoubleQuote, self.source.span());
                }
                &[b'\\', b'"' | b'\\' | b'$', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'n', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'r', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b't', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'v', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'e', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'f', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    self.source.skip(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        self.source.current()
                    {
                        self.source.next();
                        hex.push(*b as char);
                    }
                }
                &[b'\\', b'u', b'{'] => {
                    self.source.skip(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        self.source.current()
                    {
                        self.source.next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || self.source.current() != Some(&b'}') {
                        // FIXME: Push diagnostics for invalid unicode escape.
                        todo!();
                    }
                    self.source.next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        // FIXME: Push diagnostics for invalid unicode escape.
                        todo!();
                    };

                    if char::from_u32(c).is_none() {
                        // FIXME: Push diagnostics for invalid unicode escape.
                        todo!();
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    self.source.skip(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = self.source.current() {
                        self.source.next();
                        octal.push(*b as char);
                    }
                    if let Some(b @ b'0'..=b'7') = self.source.current() {
                        self.source.next();
                        octal.push(*b as char);
                    }

                    if u8::from_str_radix(&octal, 8).is_err() {
                        // FIXME: Push diagnostics for invalid octal escape.
                        todo!();
                    }
                }
                [b'$', ident_start!(), ..] => {
                    buffer_span = Some(self.source.span());
                    self.source.start_token();
                    let mut var = self.source.read_and_skip(1).to_vec();
                    var.extend(self.consume_identifier());

                    match self.source.read(4) {
                        [b'[', ..] => self.enter(StackFrame::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            self.enter(StackFrame::LookingForProperty)
                        }
                        _ => {}
                    }

                    break (TokenKind::Variable, self.source.span());
                }
                &[_, ..] => {
                    self.source.next();
                }
                // FIXME: Push diagnostics for unexpected end of file.
                [] => todo!(),
            }
        };

        let buffer_span = match buffer_span {
            Some(span) => span,
            None => self.source.span(),
        };

        if buffer_span.is_empty() {
            return Token::new(kind, span, self.source.span_range(span));
        }

        self.set_peek(Token::new(kind, span, self.source.span_range(span)));

        Token::new(
            TokenKind::StringPart,
            buffer_span,
            self.source.span_range(buffer_span),
        )
    }

    fn shell_exec(&mut self) -> Token<'a> {
        let mut buffer_span = None;

        let kind = loop {
            match self.source.read(2) {
                [b'$', b'{'] => {
                    buffer_span = Some(self.source.span());
                    self.source.start_token();
                    self.source.skip(2);
                    self.enter(StackFrame::LookingForVarname);
                    break TokenKind::DollarLeftBrace;
                }
                [b'{', b'$'] => {
                    buffer_span = Some(self.source.span());
                    self.source.start_token();
                    // Intentionally only consume the left brace.
                    self.source.next();
                    self.enter(StackFrame::Scripting);
                    break TokenKind::LeftBrace;
                }
                [b'`', ..] => {
                    self.source.next();
                    self.replace(StackFrame::Scripting);
                    break TokenKind::Backtick;
                }
                [b'$', ident_start!()] => {
                    let mut var = self.source.read_and_skip(1).to_vec();
                    var.extend(self.consume_identifier());

                    match self.source.read(4) {
                        [b'[', ..] => self.enter(StackFrame::VarOffset),
                        [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                            self.enter(StackFrame::LookingForProperty)
                        }
                        _ => {}
                    }

                    break TokenKind::Variable;
                }
                &[_, ..] => {
                    self.source.next();
                }
                // FIXME: Push diagnostics for unexpected end of file.
                [] => todo!(),
            }
        };

        let buffer_span = match buffer_span {
            Some(span) => span,
            None => self.source.span(),
        };

        let span = self.source.span();

        if buffer_span.is_empty() {
            return Token::new(kind, span, self.source.span_range(span));
        }

        self.set_peek(Token::new(kind, span, self.source.span_range(span)));

        Token::new(
            TokenKind::StringPart,
            buffer_span,
            self.source.span_range(buffer_span),
        )
    }

    fn heredoc(&mut self, label: ByteString, is_expecting_label: bool) -> Token<'a> {
        // If we're expecting a label, we should check for it here.
        // The second part of the condition isn't really needed, but it's better to be safe.
        if is_expecting_label && self.source.at(&label, label.len()) {
            self.source.skip(label.len());
            self.replace(StackFrame::Scripting);

            let span = self.source.span();

            return Token::new(TokenKind::EndHeredoc, span, self.source.span_range(span));
        }

        // Now we can check for interpolation starters. These are going to produce
        // their own tokens and then change the stack frame to the appropriate state.
        match self.source.read(2) {
            [b'$', b'{', ..] => {
                self.source.skip(2);
                
                self.enter(StackFrame::LookingForVarname);
                
                let span = self.source.span();

                return Token::new(TokenKind::DollarLeftBrace, span, self.source.span_range(span));
            }
            [b'{', b'$', ..] => {
                self.source.next();

                self.enter(StackFrame::Scripting);

                let span = self.source.span();

                return Token::new(TokenKind::LeftBrace, span, self.source.span_range(span));
            }
            [b'$', ident_start!(), ..] => {
                let mut var = self.source.read_and_skip(1).to_vec();

                var.extend(self.consume_identifier());

                match self.source.read(4) {
                    [b'[', ..] => self.enter(StackFrame::VarOffset),
                    [b'-', b'>', ident_start!(), ..] | [b'?', b'-', b'>', ident_start!()] => {
                        self.enter(StackFrame::LookingForProperty)
                    }
                    _ => {}
                };

                let span = self.source.span();

                return Token::new(TokenKind::Variable, span, self.source.span_range(span));
            },
            _ => {},
        };

        let should_expect_label = loop {
            if self.source.eof() {
                break false;
            }

            match self.source.read(3) {
                [b'\\', b'"' | b'\\' | b'$', ..] => {
                    self.source.skip(2);
                }
                // These characters start interpolation sequences, so if we find them
                // here we need to break out of the loop and let them get picked up
                // in the next iteration of the lexer.
                [b'$', b'{', ..] | [b'{', b'$', ..] | [b'$', ident_start!(), ..] => {
                    break false;
                },
                [b'\n', ..] => {
                    self.source.next();
                    self.skip_horizontal_whitespace();

                    // Check if we can see the closing label right here.
                    if self.source.at(&label, label.len()) {
                        // We've found the label so can update the stack frame
                        // so it is consumed the next time.
                        break true;
                    }
                }
                _ => self.source.next(),
            }
        };

        let span = self.source.span();

        match self.frame_mut() {
            StackFrame::DocString { expect_label, .. } => *expect_label = should_expect_label,
            _ => unreachable!(),
        };

        Token::new(
            TokenKind::StringPart,
            span,
            self.source.span_range(span),
        )
    }

    fn nowdoc(&mut self, label: ByteString, is_expecting_label: bool) -> Token<'a> {
        if is_expecting_label && self.source.at(&label, label.len()) {
            self.source.skip(label.len());
            self.replace(StackFrame::Scripting);

            let span = self.source.span();
            
            return Token::new(TokenKind::EndNowdoc, span, self.source.span_range(span));
        }

        let should_expect_label = loop {
            // If we've reached the end of the input, we need to break otherwise
            // we'll be here forever.
            if self.source.eof() {
                break false;
            }

            match self.source.read(1) {
                // If we find a new-line, we can start to check if we can see the EndHeredoc token.
                [b'\n', ..] => {
                    // Skip over the line break.
                    self.source.next();

                    // Skip over any leading whitespace.
                    self.skip_horizontal_whitespace();

                    // Check if we can see the closing label right here.
                    if self.source.at(&label, label.len()) {
                        // If we can, we need to break so that the next time we try to read a 
                        // token from this method we produce the EndNowdoc token.
                        break true;
                    }
                }
                _ => self.source.next(),
            }
        };

        let span = self.source.span();

        match self.frame_mut() {
            StackFrame::DocString { expect_label, .. } => *expect_label = should_expect_label,
            _ => unreachable!(),
        };

        Token::new(
            TokenKind::StringPart,
            span,
            self.source.span_range(span),
        )
    }

    fn looking_for_varname(&mut self) -> Option<Token<'a>> {
        let identifier = self.peek_identifier();

        if let Some(ident) = identifier {
            if let [b'[' | b'}'] = self.source.peek(ident.len(), 1) {
                self.source.skip(ident.len());
                let span = self.source.span();
                self.replace(StackFrame::Scripting);
                return Some(Token::new(
                    TokenKind::Identifier,
                    span,
                    self.source.span_range(span),
                ));
            }
        }

        self.replace(StackFrame::Scripting);

        None
    }

    fn looking_for_property(&mut self) -> Token<'a> {
        let kind = match self.source.read(3) {
            [b'?', b'-', b'>'] => {
                self.source.skip(3);
                TokenKind::QuestionArrow
            }
            [b'-', b'>', ..] => {
                self.source.skip(2);
                TokenKind::Arrow
            }
            &[ident_start!(), ..] => {
                self.consume_identifier();
                self.exit();
                TokenKind::Identifier
            }
            // Should be impossible as we already looked ahead this far inside double_quote.
            _ => unreachable!(),
        };

        let span = self.source.span();

        Token::new(kind, span, self.source.span_range(span))
    }

    fn var_offset(&mut self) -> Token<'a> {
        let kind = match self.source.read(2) {
            [b'$', ident_start!()] => self.tokenize_variable(),
            // FIXME: all integer literals are allowed, but only decimal integers with no underscores
            // are actually treated as numbers. Others are treated as strings.
            // Float literals are not allowed, but that could be handled in the parser.
            [b'0'..=b'9', ..] => self.tokenize_number(),
            [b'[', ..] => {
                self.source.next();
                TokenKind::LeftBracket
            }
            [b'-', ..] => {
                self.source.next();
                TokenKind::Minus
            }
            [b']', ..] => {
                self.source.next();
                self.exit();
                TokenKind::RightBracket
            }
            &[ident_start!(), ..] => {
                self.consume_identifier();
                TokenKind::Identifier
            }
            // FIXME: Produce "Invalid" token type and push diagnostics for unexpected character.
            &[_b, ..] => todo!(),
            // FIXME: Push diagnostics for unexpected end of file.
            [] => todo!(),
        };

        let span = self.source.span();

        Token::new(kind, span, self.source.span_range(span))
    }

    fn tokenize_single_quote_string(&mut self) -> TokenKind {
        loop {
            match self.source.read(2) {
                [b'\'', ..] => {
                    self.source.next();
                    break;
                }
                &[b'\\', b'\'' | b'\\'] => {
                    self.source.skip(2);
                }
                &[_, ..] => {
                    self.source.next();
                }
                // FIXME: Push some diagnostics here for unexpected end of file.
                [] => break,
            }
        }

        TokenKind::LiteralSingleQuotedString
    }

    fn tokenize_double_quote_string(&mut self) -> TokenKind {
        self.source.start_token();

        let constant = loop {
            match self.source.read(3) {
                [b'"', ..] => {
                    self.source.next();
                    break true;
                }
                &[b'\\', b'"' | b'\\' | b'$', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'n', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'r', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b't', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'v', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'e', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'f', ..] => {
                    self.source.skip(2);
                }
                &[b'\\', b'x', b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')] => {
                    self.source.skip(3);

                    let mut hex = String::from(b as char);
                    if let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        self.source.current()
                    {
                        self.source.next();
                        hex.push(*b as char);
                    }
                }
                &[b'\\', b'u', b'{'] => {
                    self.source.skip(3);

                    let mut code_point = String::new();
                    while let Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) =
                        self.source.current()
                    {
                        self.source.next();
                        code_point.push(*b as char);
                    }

                    if code_point.is_empty() || self.source.current() != Some(&b'}') {
                        // FIXME: Push some diagnostics here for invalid unicode escape.
                        todo!()
                    }

                    self.source.next();

                    let c = if let Ok(c) = u32::from_str_radix(&code_point, 16) {
                        c
                    } else {
                        // FIXME: Push some diagnostics here for invalid unicode escape.
                        todo!()
                    };

                    if char::from_u32(c).is_none() {
                        // FIXME: Push some diagnostics here for invalid unicode escape.
                        todo!()
                    }
                }
                &[b'\\', b @ b'0'..=b'7', ..] => {
                    self.source.skip(2);

                    let mut octal = String::from(b as char);
                    if let Some(b @ b'0'..=b'7') = self.source.current() {
                        self.source.next();
                        octal.push(*b as char);
                    }

                    if let Some(b @ b'0'..=b'7') = self.source.current() {
                        self.source.next();
                        octal.push(*b as char);
                    }

                    if u8::from_str_radix(&octal, 8).is_err() {
                        // FIXME: Push some diagnostics here for invalid octal escape.
                        todo!()
                    }
                }
                [b'$', ident_start!(), ..] | [b'{', b'$', ..] | [b'$', b'{', ..] => {
                    break false;
                }
                &[_, ..] => {
                    self.source.next();
                }
                // FIXME: Push some diagnostics here for unexpected end of file.
                [] => todo!(),
            }
        };

        if constant {
            TokenKind::LiteralDoubleQuotedString
        } else {
            self.replace(StackFrame::DoubleQuote);
            TokenKind::StringPart
        }
    }

    fn peek_identifier(&self) -> Option<&[u8]> {
        let mut size = 0;

        if let [ident_start!()] = self.source.read(1) {
            size += 1;
            while let [ident!()] = self.source.peek(size, 1) {
                size += 1;
            }

            Some(self.source.read(size))
        } else {
            None
        }
    }

    fn consume_identifier(&mut self) -> Vec<u8> {
        let ident = self.peek_identifier().unwrap().to_vec();
        self.source.skip(ident.len());

        ident
    }

    fn tokenize_variable(&mut self) -> TokenKind {
        self.source.skip(1);
        self.consume_identifier();

        TokenKind::Variable
    }

    fn tokenize_number(&mut self) -> TokenKind {
        let (base, kind) = match self.source.read(2) {
            [b'0', b'B' | b'b'] => {
                self.source.skip(2);
                (2, NumberKind::Int)
            }
            [b'0', b'O' | b'o'] => {
                self.source.skip(2);
                (8, NumberKind::Int)
            }
            [b'0', b'X' | b'x'] => {
                self.source.skip(2);
                (16, NumberKind::Int)
            }
            [b'0', ..] => (10, NumberKind::OctalOrFloat),
            [b'.', ..] => (10, NumberKind::Float),
            _ => (10, NumberKind::IntOrFloat),
        };

        if kind != NumberKind::Float {
            self.read_digits(base);
            if kind == NumberKind::Int {
                return TokenKind::LiteralInteger;
            }
        }

        // Remaining cases: decimal integer, legacy octal integer, or float.
        let is_float = matches!(
            self.source.read(3),
            [b'.', ..] | [b'e' | b'E', b'-' | b'+', b'0'..=b'9'] | [b'e' | b'E', b'0'..=b'9', ..]
        );

        if !is_float {
            return TokenKind::LiteralInteger;
        }

        if let Some(b'.') = self.source.current() {
            self.source.next();
            self.read_digits(10);
        }

        if let Some(b'e' | b'E') = self.source.current() {
            self.source.next();

            if let Some(b'-' | b'+') = self.source.current() {
                self.source.next();
            }

            self.read_digits(10);
        }

        TokenKind::LiteralFloat
    }

    fn read_digits(&mut self, base: usize) {
        if base == 16 {
            self.read_digits_fn(u8::is_ascii_hexdigit);
        } else {
            let max = b'0' + base as u8;
            self.read_digits_fn(|b| (b'0'..max).contains(b));
        };
    }

    fn read_digits_fn<F: Fn(&u8) -> bool>(&mut self, is_digit: F) {
        if let Some(b) = self.source.current() {
            if is_digit(b) {
                self.source.next();
            } else {
                return;
            }
        }

        loop {
            match self.source.read(2) {
                [b, ..] if is_digit(b) => {
                    self.source.next();
                }
                [b'_', b] if is_digit(b) => {
                    self.source.next();
                    self.source.next();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

const KEYWORDS: [(&[u8], TokenKind); 85] = [
    (b"eval", TokenKind::Eval),
    (b"die", TokenKind::Die),
    (b"empty", TokenKind::Empty),
    (b"isset", TokenKind::Isset),
    (b"unset", TokenKind::Unset),
    (b"exit", TokenKind::Exit),
    (b"enddeclare", TokenKind::EndDeclare),
    (b"endswitch", TokenKind::EndSwitch),
    (b"endfor", TokenKind::EndFor),
    (b"endwhile", TokenKind::EndWhile),
    (b"endforeach", TokenKind::EndForeach),
    (b"endif", TokenKind::EndIf),
    (b"from", TokenKind::From),
    (b"and", TokenKind::LogicalAnd),
    (b"or", TokenKind::LogicalOr),
    (b"xor", TokenKind::LogicalXor),
    (b"print", TokenKind::Print),
    (b"__halt_compiler", TokenKind::HaltCompiler),
    (b"readonly", TokenKind::Readonly),
    (b"global", TokenKind::Global),
    (b"match", TokenKind::Match),
    (b"abstract", TokenKind::Abstract),
    (b"array", TokenKind::Array),
    (b"as", TokenKind::As),
    (b"break", TokenKind::Break),
    (b"case", TokenKind::Case),
    (b"catch", TokenKind::Catch),
    (b"class", TokenKind::Class),
    (b"clone", TokenKind::Clone),
    (b"continue", TokenKind::Continue),
    (b"const", TokenKind::Const),
    (b"declare", TokenKind::Declare),
    (b"default", TokenKind::Default),
    (b"do", TokenKind::Do),
    (b"echo", TokenKind::Echo),
    (b"else", TokenKind::Else),
    (b"elseif", TokenKind::ElseIf),
    (b"enum", TokenKind::Enum),
    (b"extends", TokenKind::Extends),
    (b"false", TokenKind::False),
    (b"final", TokenKind::Final),
    (b"finally", TokenKind::Finally),
    (b"fn", TokenKind::Fn),
    (b"for", TokenKind::For),
    (b"foreach", TokenKind::Foreach),
    (b"function", TokenKind::Function),
    (b"goto", TokenKind::Goto),
    (b"if", TokenKind::If),
    (b"include", TokenKind::Include),
    (b"include_once", TokenKind::IncludeOnce),
    (b"implements", TokenKind::Implements),
    (b"interface", TokenKind::Interface),
    (b"instanceof", TokenKind::Instanceof),
    (b"namespace", TokenKind::Namespace),
    (b"new", TokenKind::New),
    (b"null", TokenKind::Null),
    (b"private", TokenKind::Private),
    (b"protected", TokenKind::Protected),
    (b"public", TokenKind::Public),
    (b"require", TokenKind::Require),
    (b"require_once", TokenKind::RequireOnce),
    (b"return", TokenKind::Return),
    (b"static", TokenKind::Static),
    (b"switch", TokenKind::Switch),
    (b"throw", TokenKind::Throw),
    (b"trait", TokenKind::Trait),
    (b"true", TokenKind::True),
    (b"try", TokenKind::Try),
    (b"use", TokenKind::Use),
    (b"var", TokenKind::Var),
    (b"yield", TokenKind::Yield),
    (b"__dir__", TokenKind::DirConstant),
    (b"__file__", TokenKind::FileConstant),
    (b"__line__", TokenKind::LineConstant),
    (b"__function__", TokenKind::FunctionConstant),
    (b"__class__", TokenKind::ClassConstant),
    (b"__method__", TokenKind::MethodConstant),
    (b"__trait__", TokenKind::TraitConstant),
    (b"__namespace__", TokenKind::NamespaceConstant),
    (
        b"__compiler_halt_offset__",
        TokenKind::CompilerHaltOffsetConstant,
    ),
    (b"while", TokenKind::While),
    (b"insteadof", TokenKind::Insteadof),
    (b"list", TokenKind::List),
    (b"self", TokenKind::Self_),
    (b"parent", TokenKind::Parent),
];

#[inline(always)]
fn identifier_to_keyword(ident: &[u8]) -> Option<TokenKind> {
    KEYWORDS.iter().find_map(|(keyword, kind)| {
        if ident.eq_ignore_ascii_case(keyword) {
            Some(*kind)
        } else {
            None
        }
    })
}

#[derive(Debug, Eq, PartialEq)]
enum NumberKind {
    Int,
    Float,
    IntOrFloat,
    OctalOrFloat,
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    use pxp_token::{OpenTagKind, TokenKind};

    #[test]
    fn it_can_tokenize_keywords() {
        use TokenKind::*;

        let tokens = Lexer::new("<?php die self parent from print readonly global abstract as break case catch class clone const continue declare default do echo else elseif empty enddeclare endfor endforeach endif endswitch endwhile enum extends false final finally fn for foreach function goto if implements include include_once instanceof insteadof eval exit unset isset list interface match namespace new null private protected public require require_once return static switch throw trait true try use var yield while and or xor").collect().iter().map(|t| t.kind).collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                OpenTag(OpenTagKind::Full),
                Die,
                Self_,
                Parent,
                From,
                Print,
                Readonly,
                Global,
                Abstract,
                As,
                Break,
                Case,
                Catch,
                Class,
                Clone,
                Const,
                Continue,
                Declare,
                Default,
                Do,
                Echo,
                Else,
                ElseIf,
                Empty,
                EndDeclare,
                EndFor,
                EndForeach,
                EndIf,
                EndSwitch,
                EndWhile,
                Enum,
                Extends,
                False,
                Final,
                Finally,
                Fn,
                For,
                Foreach,
                Function,
                Goto,
                If,
                Implements,
                Include,
                IncludeOnce,
                Instanceof,
                Insteadof,
                Eval,
                Exit,
                Unset,
                Isset,
                List,
                Interface,
                Match,
                Namespace,
                New,
                Null,
                Private,
                Protected,
                Public,
                Require,
                RequireOnce,
                Return,
                Static,
                Switch,
                Throw,
                Trait,
                True,
                Try,
                Use,
                Var,
                Yield,
                While,
                LogicalAnd,
                LogicalOr,
                LogicalXor,
                Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_casts() {
        use TokenKind::*;

        let tokens = Lexer::new("<?php (int) (integer) (bool) (boolean) (float) (double) (real) (string) (array) (object) (unset)").collect().iter().map(|t| t.kind).collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                OpenTag(OpenTagKind::Full),
                IntCast,
                IntegerCast,
                BoolCast,
                BooleanCast,
                FloatCast,
                DoubleCast,
                RealCast,
                StringCast,
                ArrayCast,
                ObjectCast,
                UnsetCast,
                Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_casts_with_excess_whitespace() {
        use TokenKind::*;

        let tokens = Lexer::new("<?php (int    ) (integer  ) (bool  ) (boolean) (float ) (double   ) (real    ) (string ) (array   ) (object   ) (  unset  )").collect().iter().map(|t| t.kind).collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                OpenTag(OpenTagKind::Full),
                IntCast,
                IntegerCast,
                BoolCast,
                BooleanCast,
                FloatCast,
                DoubleCast,
                RealCast,
                StringCast,
                ArrayCast,
                ObjectCast,
                UnsetCast,
                Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_operators() {
        use TokenKind::*;

        let tokens = Lexer::new("<?php + - * / % ** = += -= *= /= .= %= **= &= |= ^= <<= >>= <=> == === != <> !== > < >= <= <=> ?? ! && || ??= and or xor . -> :: ++ -- ?? ! and or xor").collect().iter().map(|t| t.kind).collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                OpenTag(OpenTagKind::Full),
                Plus,
                Minus,
                Asterisk,
                Slash,
                Percent,
                Pow,
                Equals,
                PlusEquals,
                MinusEquals,
                AsteriskEquals,
                SlashEquals,
                DotEquals,
                PercentEquals,
                PowEquals,
                AmpersandEquals,
                PipeEquals,
                CaretEquals,
                LeftShiftEquals,
                RightShiftEquals,
                Spaceship,
                DoubleEquals,
                TripleEquals,
                BangEquals,
                AngledLeftRight,
                BangDoubleEquals,
                GreaterThan,
                LessThan,
                GreaterThanEquals,
                LessThanEquals,
                Spaceship,
                DoubleQuestion,
                Bang,
                BooleanAnd,
                BooleanOr,
                DoubleQuestionEquals,
                LogicalAnd,
                LogicalOr,
                LogicalXor,
                Dot,
                Arrow,
                DoubleColon,
                Increment,
                Decrement,
                DoubleQuestion,
                Bang,
                LogicalAnd,
                LogicalOr,
                LogicalXor,
                Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_single_quoted_strings() {
        let tokens = Lexer::new("<?php 'foo' 'foo\\'bar'")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::LiteralSingleQuotedString,
                TokenKind::LiteralSingleQuotedString,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_double_quoted_strings() {
        let tokens = Lexer::new("<?php \"foo\" \"foo\\\"bar\"")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::LiteralDoubleQuotedString,
                TokenKind::LiteralDoubleQuotedString,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_heredocs() {
        let tokens = Lexer::new("<?php <<<EOD\n    foo\n    EOD")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::StartHeredoc,
                TokenKind::StringPart,
                TokenKind::EndHeredoc,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_nowdocs() {
        let tokens = Lexer::new("<?php <<<'EOD'\n    foo\n    EOD")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::StartNowdoc,
                TokenKind::StringPart,
                TokenKind::EndNowdoc,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_integers() {
        let tokens = Lexer::new("<?php 100 0123 0o123 0x1A 0b11111111 1_234_567")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::LiteralInteger,
                TokenKind::LiteralInteger,
                TokenKind::LiteralInteger,
                TokenKind::LiteralInteger,
                TokenKind::LiteralInteger,
                TokenKind::LiteralInteger,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_floats() {
        let tokens = Lexer::new("<?php 1.234 1.2e3 7E-10 1_234.567")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::LiteralFloat,
                TokenKind::LiteralFloat,
                TokenKind::LiteralFloat,
                TokenKind::LiteralFloat,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn it_can_tokenize_identifiers() {
        let tokens = Lexer::new("<?php hello \\hello hello\\world")
            .collect()
            .iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::Identifier,
                TokenKind::FullyQualifiedIdentifier,
                TokenKind::QualifiedIdentifier,
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn it_can_tokenize_heredocs_with_escapes() {
        let mut lexer = Lexer::new("<?php <<<EOD\n\\$foo\nEOD;");

        let tokens = lexer.collect().iter().map(|t| t.kind).collect::<Vec<_>>();

        assert_eq!(
            &tokens,
            &[
                TokenKind::OpenTag(OpenTagKind::Full),
                TokenKind::StartHeredoc,
                TokenKind::StringPart,
                TokenKind::EndHeredoc,
                TokenKind::SemiColon,
                TokenKind::Eof,
            ]
        );
    }
}
