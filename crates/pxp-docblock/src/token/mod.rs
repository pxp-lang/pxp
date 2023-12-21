use std::fmt::Debug;

use pxp_span::Span;
use pxp_symbol::{Symbol, SymbolTable};

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub symbol: Symbol,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, symbol: Symbol) -> Self {
        Self { kind, span, symbol }
    }

    pub fn with_symbol_table<'a>(&self, symbol_table: &'a SymbolTable) -> TokenWithSymbolTable<'a> {
        TokenWithSymbolTable {
            token: *self,
            symbol_table,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Reference,
    Union,
    Intersection,
    Nullable,
    OpenParen,
    CloseParen,
    OpenAngle,
    CloseAngle,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Variadic,
    DoubleColon,
    DoubleArrow,
    Equal,
    OpenPhpdoc,
    ClosePhpdoc,
    PhpdocTag,
    Float,
    Integer,
    SingleQuotedString,
    DoubleQuotedString,
    Identifier,
    QualifiedIdentifier,
    FullyQualifiedIdentifier,
    ThisVariable,
    Variable,
    HorizontalWhitespace,
    PhpdocEol,
    Other,
    End,
    Colon,
    Wildcard,
    Negated,
    Arrow,
    Eol,
}

pub struct TokenWithSymbolTable<'a> {
    pub token: Token,
    pub symbol_table: &'a SymbolTable,
}

impl<'a> Debug for TokenWithSymbolTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token.kind {
            TokenKind::Float => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::Integer => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::SingleQuotedString => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::DoubleQuotedString => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::Identifier => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::QualifiedIdentifier => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::FullyQualifiedIdentifier => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::ThisVariable => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::Variable => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::PhpdocEol => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            TokenKind::PhpdocTag => write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(self.token.symbol).unwrap()),
            _ => write!(f, "{:?}", self.token.kind),
        }
    }
}
