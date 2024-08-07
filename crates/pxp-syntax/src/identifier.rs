use pxp_token::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierQualification {
    FullyQualified,
    Qualified,
    Unqualified,
}

impl IdentifierQualification {
    pub fn is_fully_qualified(&self) -> bool {
        matches!(self, Self::FullyQualified)
    }

    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::Qualified)
    }

    pub fn is_unqualified(&self) -> bool {
        matches!(self, Self::Unqualified)
    }
}

impl From<TokenKind> for IdentifierQualification {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Identifier => Self::Unqualified,
            TokenKind::QualifiedIdentifier => Self::Qualified,
            TokenKind::FullyQualifiedIdentifier => Self::FullyQualified,
            _ => unreachable!(),
        }
    }
}
