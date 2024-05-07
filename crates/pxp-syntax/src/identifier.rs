use pxp_token::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameQualification {
    FullyQualified,
    Qualified,
    Unqualified,
}

impl NameQualification {
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

impl From<TokenKind> for NameQualification {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Identifier => Self::Unqualified,
            TokenKind::QualifiedIdentifier => Self::Qualified,
            TokenKind::FullyQualifiedIdentifier => Self::FullyQualified,
            _ => unreachable!(),
        }
    }
}