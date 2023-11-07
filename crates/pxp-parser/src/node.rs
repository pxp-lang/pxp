use std::any::Any;

pub trait Node: Any {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![]
    }
}
