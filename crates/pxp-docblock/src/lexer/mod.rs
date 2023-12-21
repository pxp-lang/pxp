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

                    state.push(Token::new(TokenKind::OpenPhpdoc, span));
                },
                [b'*', b'/', ..] => {
                    state.skip(2);

                    let span = state.span();

                    state.push(Token::new(TokenKind::ClosePhpdoc, span));
                },
                [b'\x09' | b'\x20', ..] => {
                    state.skip_horizontal_whitespace();

                    let span = state.span();
                    let symbol = self.symbol_table.intern(state.range(span.start.offset, span.end.offset));

                    state.push(Token::new(TokenKind::HorizontalWhitespace(symbol), span))
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

                        state.push(Token::new(TokenKind::PhpdocEol(self.symbol_table.intern(state.range(span.start.offset, span.end.offset))), span));
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

                    state.push(Token::new(TokenKind::PhpdocEol(symbol), span));
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
                        TokenKind::FullyQualifiedIdentifier(symbol)
                    } else if qualified {
                        TokenKind::QualifiedIdentifier(symbol)
                    } else {
                        TokenKind::Identifier(symbol)
                    }, span));
                },
                [b'.', b'.', b'.', ..] => {
                    state.skip(3);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Variadic, span));
                },
                [b'=', b'>', ..] => {
                    state.skip(2);

                    let span = state.span();

                    state.push(Token::new(TokenKind::DoubleArrow, span));
                },
                [b':', b':', ..] => {
                    state.skip(2);

                    let span = state.span();

                    state.push(Token::new(TokenKind::DoubleColon, span));
                },
                [b'&', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Intersection, span));
                },
                [b'|', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Union, span));
                },
                [b'(', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenParen, span));
                },
                [b')', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseParen, span));
                },
                [b'{', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenBrace, span));
                },
                [b'}', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseBrace, span));
                },
                [b'[', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenBracket, span));
                },
                [b']', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseBracket, span));
                },
                [b'<', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::OpenAngle, span));
                },
                [b'>', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::CloseAngle, span));
                },
                [b'=', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Equal, span));
                },
                [b',', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Comma, span));
                },
                [b':', ..] => {
                    state.skip(1);

                    let span = state.span();

                    state.push(Token::new(TokenKind::Colon, span));
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
                        state.push(Token::new(TokenKind::Float(symbol), span));
                    } else {
                        state.push(Token::new(TokenKind::Integer(symbol), span));
                    }
                },
                [fb @ b'\r', b'\n', ..] | [fb @ b'\n', ..] => {
                    if *fb == b'\r' {
                        state.skip(2);
                    } else {
                        state.skip(1);
                    }

                    let span = state.span();

                    state.push(Token::new(TokenKind::Eol, span));
                },
                _ => unimplemented!("{:?}", state.current() as char)
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
