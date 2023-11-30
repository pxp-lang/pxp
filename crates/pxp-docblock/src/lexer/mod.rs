use pxp_span::{Position, Span};
use pxp_symbol::SymbolTable;

use self::token::{Token, TokenKind};

pub(crate) mod token;
#[macro_use]
pub(crate) mod macros;

pub struct Lexer<'a, 'b> {
    input: &'a [u8],
    start: Position,
    position: Position,
    symbol_table: &'b mut SymbolTable<'a>,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn new<B: ?Sized + AsRef<[u8]>>(input: &'a B, symbol_table: &'b mut SymbolTable<'a>) -> Self {
        Self {
            input: input.as_ref(),
            start: Position::new(0, 1, 0),
            position: Position::new(0, 1, 0),
            symbol_table
        }
    }

    pub fn tokenize(&'b mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        self.skip_whitespace();
        while !self.is_eof() {
            self.skip_whitespace();
            self.start();

            match self.current() {
                b'\'' => {
                    self.next();

                    while self.current() != b'\'' {
                        self.next();

                        if self.is_eof() {
                            break;
                        }
                    }

                    self.next();

                    let span = self.span();
                    let symbol = self.symbol_table.intern(self.range(span.start.offset, span.end.offset));

                    tokens.push(Token::new_with_symbol(TokenKind::SingleQuotedString, span, symbol));
                },
                b'\"' => {
                    self.next();

                    while self.current() != b'\"' {
                        self.next();

                        if self.is_eof() {
                            break;
                        }
                    }

                    self.next();

                    let span = self.span();
                    let symbol = self.symbol_table.intern(self.range(span.start.offset, span.end.offset));

                    tokens.push(Token::new_with_symbol(TokenKind::DoubleQuotedString, span, symbol));
                },
                b'|' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Union, self.span()));
                },
                b'&' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Intersection, self.span()));
                },
                b'?' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Nullable, self.span()));
                },
                b'!' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Negated, self.span()));
                },
                b'(' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::OpenParen, self.span()));
                },
                b')' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::CloseParen, self.span()));
                },
                b'<' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::OpenAngle, self.span()));
                },
                b'>' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::CloseAngle, self.span()));
                },
                b'[' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::OpenSquare, self.span()));
                },
                b']' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::CloseSquare, self.span()));
                },
                b'{' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::OpenCurly, self.span()));
                },
                b'}' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::CloseCurly, self.span()));
                },
                b',' => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Comma, self.span()));
                },
                b'=' if self.peek() != Some(b'>') => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Equal, self.span()));
                },
                b':' if self.peek() != Some(b':') => {
                    self.next();

                    tokens.push(Token::new_without_symbol(TokenKind::Colon, self.span()));
                },
                _ => match self.range_n(2) {
                    [b'@', b'a'..=b'z', ..] => {
                        self.skip(2);

                        while let tag!() = self.current() {
                            self.next();

                            if self.is_eof() {
                                break;
                            }
                        }

                        let span = self.span();
                        let symbol = self.symbol_table.intern(self.range(span.start.offset, span.end.offset));

                        tokens.push(Token::new_with_symbol(TokenKind::Tag, span, symbol));
                    },
                    [b'$', ident_start!(), ..] if self.range_n(5) != b"$this" => {
                        self.skip(2);

                        while let ident!() = self.current() {
                            self.next();

                            if self.is_eof() {
                                break;
                            }
                        }

                        let span = self.span();
                        let symbol = self.symbol_table.intern(self.range(span.start.offset, span.end.offset));

                        tokens.push(Token::new_with_symbol(TokenKind::Variable, span, symbol));
                    },
                    [b':', b':', ..] => {
                        self.skip(2);

                        tokens.push(Token::new_without_symbol(TokenKind::DoubleColon, self.span()));
                    },
                    [b'=', b'>', ..] => {
                        self.skip(2);

                        tokens.push(Token::new_without_symbol(TokenKind::DoubleArrow, self.span()));
                    },
                    [b'-', b'>', ..] => {
                        self.skip(2);

                        tokens.push(Token::new_without_symbol(TokenKind::Arrow, self.span()));
                    },
                    _ => match self.range_n(3) {
                        [b'0', b'b' | b'B', b'0' | b'1', ..] => {
                            self.skip(3);

                            while let b'0' | b'1' | b'_' = self.current() {
                                self.next();

                                if self.is_eof() {
                                    break;
                                }
                            }

                            tokens.push(Token::new_without_symbol(TokenKind::Integer, self.span()));
                        },
                        [b'0', b @ (b'o' | b'O'), b'0'..=b'7', ..] | [b'0', b @ (b'0'..=b'7'), ..] => {
                            if let b'o' | b'O' = b {
                                self.skip(3);
                            } else {
                                self.skip(2);
                            }

                            while let b'0'..=b'7' | b'_' = self.current() {
                                self.next();

                                if self.is_eof() {
                                    break;
                                }
                            }

                            tokens.push(Token::new_without_symbol(TokenKind::Integer, self.span()));
                        },
                        [b'0', b'x' | b'X', b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F', ..] => {
                            self.skip(3);

                            while let b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'_' = self.current() {
                                self.next();

                                if self.is_eof() {
                                    break;
                                }
                            }

                            tokens.push(Token::new_without_symbol(TokenKind::Integer, self.span()));
                        },
                        [b'0'..=b'9', ..] => {
                            self.next();

                            let mut kind = if self.current() == b'.' { self.next(); TokenKind::Float } else { TokenKind::Integer };

                            while let b'0'..=b'9' | b'_' = self.current() {
                                self.next();

                                if self.is_eof() {
                                    break;
                                }

                                if self.current() == b'.' {
                                    if kind == TokenKind::Float {
                                        break;
                                    }

                                    kind = TokenKind::Float;
                                    self.next();
                                }
                            }

                            tokens.push(Token::new_without_symbol(kind, self.span()));
                        },
                        [b'.', b'.', b'.', ..] => {
                            self.skip(3);

                            tokens.push(Token::new_without_symbol(TokenKind::Variadic, self.span()));
                        },
                        _ => match self.range_n(4) {
                            [b'/', b'*', b'*', whitespace!()] => {
                                self.skip(3);
                                self.skip_whitespace();

                                tokens.push(Token::new_without_symbol(TokenKind::OpenDoc, self.span()));
                            },
                            [ident_start!(), ..] | [b'\\', ident_start!(), ..] => {
                                let mut kind = if self.current() == b'\\' {
                                    self.next();

                                    TokenKind::FullyQualifiedIdentifier
                                } else {
                                    TokenKind::Identifier
                                };

                                self.next();

                                while let ident!() = self.current() {
                                    self.next();

                                    if self.is_eof() {
                                        break;
                                    }

                                    if self.current() == b'\\' {
                                        self.next();

                                        if kind == TokenKind::Identifier {
                                            kind = TokenKind::QualifiedIdentifier;
                                        }
                                    }
                                }

                                let span = self.span();
                                let symbol = self.symbol_table.intern(self.range(span.start.offset, span.end.offset));

                                tokens.push(Token::new_with_symbol(kind, span, symbol));
                            },
                            [b'*', b'/', ..] => {
                                self.skip(2);

                                tokens.push(Token::new_without_symbol(TokenKind::CloseDoc, self.span()));
                                break;
                            },
                            _ => match self.range_n(5) {
                                [b'$', b't', b'h', b'i', b's', ..] => {
                                    self.skip(5);

                                    tokens.push(Token::new_without_symbol(TokenKind::ThisVariable, self.span()));
                                },
                                _ => unimplemented!("{:?}", self.current() as char),
                            }
                        }
                    }
                }
            }
        }

        return tokens;
    }

    fn start(&mut self) {
        self.start = self.position;
    }

    fn span(&self) -> Span {
        Span::new(self.start, self.position)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.current().is_ascii_whitespace() {
            self.next();
        }
    }

    fn range(&self, start: usize, end: usize) -> &'a [u8] {
        let end = if end > self.input.len() {
            self.input.len()
        } else {
            end
        };

        &self.input[start..end]
    }

    fn range_n(&self, end: usize) -> &'a [u8] {
        self.range(self.position.offset, self.position.offset + end)
    }

    fn current(&self) -> u8 {
        self.input[self.position.offset]
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.position.offset + 1).copied()
    }

    fn peek_offset(&self, start: usize, offset: usize) -> u8 {
        let offset = start + offset;

        if offset >= self.input.len() {
            return self.input[self.input.len() - 1];
        }

        self.input[offset]
    }

    fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    fn next(&mut self) {
        self.position.offset += 1;

        if self.is_eof() {
            return;
        }

        if self.current() == b'\n' {
            self.position.line += 1;
            self.position.column = 0;
        } else {
            self.position.column += 1;
        }
    }

    fn is_eof(&self) -> bool {
        self.position.offset >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use pxp_symbol::SymbolTable;
    use super::{Lexer, token::{TokenKind, Token}};

    macro_rules! assert_kinds {
        ($tokens:expr, $kinds:expr) => {
            // assert_eq!($tokens.len(), $kinds.len());

            for (token, kind) in $tokens.iter().zip($kinds.iter()) {
                assert_eq!(token.kind, *kind);
            }
        };
    }

    #[test]
    fn it_can_tokenize_open_doc() {
        let tokens = tokenise("/** ");

        assert_kinds!(tokens, vec![TokenKind::OpenDoc]);
    }

    #[test]
    fn it_can_tokenize_close_doc() {
        let tokens = tokenise("*/");

        assert_kinds!(tokens, vec![TokenKind::CloseDoc]);
    }

    #[test]
    fn it_can_tokenize_identifiers() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("example", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::Identifier,
        ]);

        assert_eq!(symbol_table.resolve(tokens[0].symbol.unwrap()).unwrap(), b"example");
    }

    #[test]
    fn it_can_tokenize_qualified_identifiers() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("example\\foo", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::QualifiedIdentifier,
        ]);

        assert_eq!(symbol_table.resolve(tokens[0].symbol.unwrap()).unwrap(), b"example\\foo");
    }

    #[test]
    fn it_can_tokenize_fully_qualified_identifiers() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("\\example\\foo", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::FullyQualifiedIdentifier,
        ]);

        assert_eq!(symbol_table.resolve(tokens[0].symbol.unwrap()).unwrap(), b"\\example\\foo");
    }

    #[test]
    fn it_can_tokenize_variables() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("$example", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::Variable,
        ]);

        assert_eq!(symbol_table.resolve(tokens[0].symbol.unwrap()).unwrap(), b"$example");
    }

    #[test]
    fn it_can_tokenize_this_variables() {
        let tokens = tokenise("$this");

        assert_kinds!(tokens, vec![
            TokenKind::ThisVariable,
        ]);
    }

    #[test]
    fn it_can_tokenize_tags() {
        let tokens = tokenise("@example @example-tag @example:tag @example\\tag");

        assert_kinds!(tokens, vec![
            TokenKind::Tag,
            TokenKind::Tag,
            TokenKind::Tag,
            TokenKind::Tag,
        ]);
    }

    #[test]
    fn it_can_tokenize_single_quoted_strings() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("'example'", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::SingleQuotedString,
        ]);

        assert_eq!(symbol_table.resolve(tokens[0].symbol.unwrap()).unwrap(), b"'example'");
    }

    #[test]
    fn it_can_tokenize_double_quoted_strings() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("\"example\"", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::DoubleQuotedString,
        ]);

        assert_eq!(symbol_table.resolve(tokens[0].symbol.unwrap()).unwrap(), b"\"example\"");
    }

    #[test]
    fn it_can_tokenize_integers() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("100 0123 0o123 0x1A 0b11111111 1_234_567", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::Integer,
            TokenKind::Integer,
            TokenKind::Integer,
            TokenKind::Integer,
            TokenKind::Integer,
            TokenKind::Integer,
        ]);
    }

    #[test]
    fn it_can_tokenize_floats() {
        let mut symbol_table = SymbolTable::new();
        let tokens = tokenise_through("1.2345 100.5 100_250.5", &mut symbol_table);

        assert_kinds!(tokens, vec![
            TokenKind::Float,
            TokenKind::Float,
            TokenKind::Float,
        ]);
    }

    #[test]
    fn it_can_tokenize_symbols() {
        let tokens = tokenise("| & ? ! () <> [] {} , ... :: => -> = :");

        assert_kinds!(tokens, vec![
            TokenKind::Union,
            TokenKind::Intersection,
            TokenKind::Nullable,
            TokenKind::Negated,
            TokenKind::OpenParen,
            TokenKind::CloseParen,
            TokenKind::OpenAngle,
            TokenKind::CloseAngle,
            TokenKind::OpenSquare,
            TokenKind::CloseSquare,
            TokenKind::OpenCurly,
            TokenKind::CloseCurly,
            TokenKind::Comma,
            TokenKind::Variadic,
            TokenKind::DoubleColon,
            TokenKind::DoubleArrow,
            TokenKind::Arrow,
            TokenKind::Equal,
            TokenKind::Colon,
        ]);
    }

    fn tokenise_through<'a>(input: &'a str, symbol_table: &mut SymbolTable<'a>) -> Vec<Token> {
        let mut lexer = Lexer::new(input, symbol_table);

        lexer.tokenize()
    }

    fn tokenise(input: &str) -> Vec<Token> {
        let mut symbol_table = SymbolTable::new();
        let mut lexer = Lexer::new(input, &mut symbol_table);

        lexer.tokenize()
    }
}