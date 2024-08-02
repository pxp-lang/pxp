use pxp_token::TokenKind;

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum NameQualification {
    Unqualified,
    Qualified,
    FullyQualified,
}

impl From<TokenKind> for NameQualification {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::QualifiedIdentifier => NameQualification::Qualified,
            TokenKind::FullyQualifiedIdentifier => NameQualification::FullyQualified,
            _ => NameQualification::Unqualified,
        }
    }
}
