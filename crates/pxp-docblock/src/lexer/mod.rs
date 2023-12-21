use pxp_symbol::SymbolTable;

use crate::token::{Token, TokenKind};

use self::state::State;

mod state;
mod macros;

#[derive(Debug)]
pub struct Lexer<'a> {
    symbol_table: &'a mut SymbolTable,
}

impl<'a> Lexer<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn tokenize<'b>(&mut self, input: &'b [u8]) -> Result<Vec<Token>, LexerError> {
        let mut state = State::new(input);

        while ! state.is_eof() {
            state.start_token();

            match state.peek_n(4) {
                [b'/', b'*', b'*', ..] => {
                    state.skip(3);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenPhpdoc, span, self.symbol_table.intern(b"/**")));
                },
                [b'*', b'/', ..] => {
                    state.skip(2);

                    let span = state.span();

                    state.push(Token::new(TokenKind::ClosePhpdoc, span, self.symbol_table.intern(b"*/")));
                },
                [b'\x09' | b'\x20', ..] => {
                    state.skip_horizontal_whitespace();

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::HorizontalWhitespace, span, symbol))
                },
                [fb @ b'\r', b'\n', b'\x09' | b'\x20', ..] | [fb @ b'\n', b'\x09' | b'\x20', ..] => {
                    // Consume a carriage return if it exists.
                    if *fb == b'\r' {
                        state.skip(1);
                    }

                    // Consume the newline.
                    state.skip(1);

                    // Consume any horizontal whitespace.
                    state.skip_horizontal_whitespace();

                    if state.current() != b'*' {
                        return Err(LexerError::UnexpectedCharacter(state.current()));
                    }

                    if state.peek() == b'/' {
                        let span = state.span();

                        state.push(Token::new(TokenKind::PhpdocEol, span, self.symbol_table.intern(state.range(span.start.offset, span.end.offset))));
                        continue;
                    }

                    // Consume the asterisk.
                    state.skip(1);

                    // Consume an optional space.
                    if state.current() == b'\x20' {
                        state.skip(1);
                    }

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::PhpdocEol, span, symbol));
                },
                [b'\\', b'a'..=b'z' | b'A'..=b'Z' | b'\x80'..=b'\xFF' | b'_', ..] | [b'a'..=b'z' | b'A'..=b'Z' | b'\x80'..=b'\xFF' | b'_', ..] => {
                    let fully_qualified = state.current() == b'\\';
                    let mut qualified = false;

                    if fully_qualified {
                        state.next();
                    }

                    fn consume_identifier_part(state: &mut State) {
                        // [A-Za-z_\x80-\xFF]
                        while let b'a'..=b'z' | b'A'..=b'Z' | b'\x80'..=b'\xFF' | b'_' = state.current() {
                            state.next();

                            if state.is_eof() {
                                return;
                            }
                        }

                        // [0-9A-Za-z_\x80-\xFF-]
                        while let b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'\x80'..=b'\xFF' | b'_' | b'-' = state.current() {
                            state.next();

                            if state.is_eof() {
                                return;
                            }
                        }
                    }

                    consume_identifier_part(&mut state);

                    while state.current() == b'\\' {
                        state.next();
                        qualified = true;

                        consume_identifier_part(&mut state);
                    }

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(if fully_qualified {
                        TokenKind::FullyQualifiedIdentifier
                    } else if qualified {
                        TokenKind::QualifiedIdentifier
                    } else {
                        TokenKind::Identifier
                    }, span, symbol));
                },
                [b'@', b'a'..=b'z' | b'A'..=b'Z', ..] => {
                    state.skip(1);

                    while let b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'\\' | b'-' = state.current() {
                        state.next();

                        if state.is_eof() {
                            break;
                        }
                    }

                    if state.current() == b':' {
                        state.next();

                        while let b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'\\' | b'-' = state.current() {
                            state.next();

                            if state.is_eof() {
                                break;
                            }
                        }
                    }

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::PhpdocTag, span, symbol));
                },
                [b'.', b'.', b'.', ..] => {
                    state.skip(3);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Variadic, span, self.symbol_table.intern(b"...")));
                },
                [b'=', b'>', ..] => {
                    state.skip(2);

                    let span = state.span();

                    state.push(Token::new(TokenKind::DoubleArrow, span, self.symbol_table.intern(b"=>")));
                },
                [b':', b':', ..] => {
                    state.skip(2);

                    let span = state.span();

                    state.push(Token::new(TokenKind::DoubleColon, span, self.symbol_table.intern(b"::")));
                },
                [b'&', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Intersection, span, self.symbol_table.intern(b"&")));
                },
                [b'|', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Union, span, self.symbol_table.intern(b"|")));
                },
                [b'(', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenParen, span, self.symbol_table.intern(b"(")));
                },
                [b')', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseParen, span, self.symbol_table.intern(b")")));
                },
                [b'{', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenBrace, span, self.symbol_table.intern(b"{")));
                },
                [b'}', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseBrace, span, self.symbol_table.intern(b"}")));
                },
                [b'[', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenBracket, span, self.symbol_table.intern(b"[")));
                },
                [b']', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseBracket, span, self.symbol_table.intern(b"]")));
                },
                [b'<', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenAngle, span, self.symbol_table.intern(b"<")));
                },
                [b'>', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseAngle, span, self.symbol_table.intern(b">")));
                },
                [b'=', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Equal, span, self.symbol_table.intern(b"=")));
                },
                [b',', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Comma, span, self.symbol_table.intern(b",")));
                },
                [b':', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Colon, span, self.symbol_table.intern(b":")));
                },
                [fb @ b'+' | fb @ b'-', b'0'..=b'9', ..] | [fb @ b'0'..=b'9', ..] => {
                    if *fb == b'+' || *fb == b'-' {
                        state.skip(1);
                    }

                    let mut is_float = false;

                    while let b'0'..=b'9' = state.current() {
                        state.next();

                        if state.is_eof() {
                            break;
                        }
                    }

                    if state.current() == b'.' {
                        is_float = true;

                        state.next();

                        while let b'0'..=b'9' = state.current() {
                            state.next();

                            if state.is_eof() {
                                break;
                            }
                        }
                    }

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    if is_float {
                        state.push(Token::new(TokenKind::Float, span, symbol));
                    } else {
                        state.push(Token::new(TokenKind::Integer, span, symbol));
                    }
                },
                [b'$', b'a'..=b'z' | b'A'..=b'Z' | b'\x80'..=b'\xFF' | b'_', ..] => {
                    state.skip(1);

                    let mut bytes = vec![];
                    let mut this = false;

                    // Since we are only inside of this block if the first non-$ characters are valid, we can also
                    // tokenise numeric characters here as they definitely won't be at the start of the variable name.
                    while let b'a'..=b'z' | b'A'..=b'Z' | b'\x80'..=b'\xFF' | b'_' | b'0'..=b'9' = state.current() {
                        if this {
                            this = false;
                        }

                        bytes.push(state.current());
                        this = bytes == b"this";

                        state.next();

                        if state.is_eof() {
                            break;
                        }
                    }

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    if this {
                        state.push(Token::new(TokenKind::ThisVariable, span, symbol));
                    } else {
                        state.push(Token::new(TokenKind::Variable, span, symbol));
                    }
                }
                [fb @ b'\r', b'\n', ..] | [fb @ b'\n', ..] => {
                    if *fb == b'\r' {
                        state.skip(2);
                    } else {
                        state.skip(1);
                    }

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::Eol, span, symbol));
                },
                [b'\'', ..] => {
                    state.skip(1);

                    while state.current() != b'\'' {
                        state.next();

                        if state.is_eof() {
                            return Err(LexerError::UnexpectedEndOfInput);
                        }
                    }

                    state.skip(1);

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::SingleQuotedString, span, symbol));
                },
                [b'"', ..] => {
                    state.skip(1);

                    while state.current() != b'"' {
                        state.next();

                        if state.is_eof() {
                            return Err(LexerError::UnexpectedEndOfInput);
                        }
                    }

                    state.skip(1);

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::DoubleQuotedString, span, symbol));
                },
                _ => {
                    state.next();

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::Other, span, symbol));
                }
            }
        }

        Ok(state.get_tokens())
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(u8),
    UnexpectedEndOfInput,
}
