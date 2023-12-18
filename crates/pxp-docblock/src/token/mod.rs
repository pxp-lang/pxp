use std::fmt::Debug;

use pxp_span::Span;
use pxp_symbol::{Symbol, SymbolTable};

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn with_symbol_table<'a>(&self, symbol_table: &'a SymbolTable) -> TokenWithSymbolTable<'a> {
        TokenWithSymbolTable {
            token: *self,
            symbol_table,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    Float(Symbol),
    Integer(Symbol),
    SingleQuotedString(Symbol),
    DoubleQuotedString(Symbol),
    Identifier(Symbol),
    QualifiedIdentifier(Symbol),
    FullyQualifiedIdentifier(Symbol),
    ThisVariable(Symbol),
    Variable(Symbol),
    HorizontalWhitespace(Symbol),
    PhpdocEol(Symbol),
    Other,
    End,
    Colon,
    Wildcard,
    Negated,
    Arrow,
}

impl TokenKind {
    pub fn get_symbol(&self) -> Option<Symbol> {
        match self {
            TokenKind::Float(symbol) => Some(*symbol),
            TokenKind::Integer(symbol) => Some(*symbol),
            TokenKind::SingleQuotedString(symbol) => Some(*symbol),
            TokenKind::DoubleQuotedString(symbol) => Some(*symbol),
            TokenKind::Identifier(symbol) => Some(*symbol),
            TokenKind::QualifiedIdentifier(symbol) => Some(*symbol),
            TokenKind::FullyQualifiedIdentifier(symbol) => Some(*symbol),
            TokenKind::ThisVariable(symbol) => Some(*symbol),
            TokenKind::Variable(symbol) => Some(*symbol),
            TokenKind::HorizontalWhitespace(symbol) => Some(*symbol),
            TokenKind::PhpdocEol(symbol) => Some(*symbol),
            _ => None,
        }
    }
}

pub struct TokenWithSymbolTable<'a> {
    pub token: Token,
    pub symbol_table: &'a SymbolTable,
}

impl<'a> Debug for TokenWithSymbolTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(symbol) = self.token.kind.get_symbol() {
            write!(f, "{:?} (Symbol: {:?})", self.token.kind, self.symbol_table.resolve(symbol).unwrap())
        } else {
            write!(f, "{:?}", self.token.kind)
        }
    }
}