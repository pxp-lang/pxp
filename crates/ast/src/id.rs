use crate::NodeId;

pub trait HasId {
    fn id(&self) -> NodeId;
}
