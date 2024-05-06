use pxp_visitor::VisitorMut;

#[derive(Debug)]
pub struct NameResolvingVisitor {
    context: NameResolvingContext,
}

impl NameResolvingVisitor {
    pub fn new() -> Self {
        NameResolvingVisitor {
            context: NameResolvingContext {},
        }
    }
}

#[derive(Debug)]
struct NameResolvingContext {

}

impl VisitorMut for NameResolvingVisitor {

}