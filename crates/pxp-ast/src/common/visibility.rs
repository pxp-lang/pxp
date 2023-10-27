#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Protected,
    Private
}

impl Visibility {
    pub fn is_public(&self) -> bool {
        matches!(self, Visibility::Public)
    }

    pub fn is_protected(&self) -> bool {
        matches!(self, Visibility::Protected)
    }

    pub fn is_private(&self) -> bool {
        matches!(self, Visibility::Private)
    }
}