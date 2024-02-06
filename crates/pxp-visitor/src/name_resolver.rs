use crate::Visitor;

#[derive(Debug, Default)]
pub struct NameResolvingVisitor {
    // ...
}

impl NameResolvingVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Visitor for NameResolvingVisitor {
    
}