use crate::NodeId;

pub trait HasId {
    fn id(&self) -> NodeId;
}

impl<T: HasId> HasId for Box<T> {
    fn id(&self) -> NodeId {
        self.as_ref().id()
    }
}
