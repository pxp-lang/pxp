#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum NameQualification {
    Unqualified,
    Qualified,
    FullyQualified,
}