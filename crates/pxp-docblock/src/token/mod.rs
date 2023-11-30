use pxp_span::Span;
use pxp_symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
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