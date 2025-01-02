mod function;
mod parameters;

pub use function::FunctionEntity;
pub use parameters::{Parameters, Parameter};
use pxp_bytestring::ByteString;

#[derive(Debug, Clone)]
pub(crate) struct EntityRegistry {
    functions: Vec<FunctionEntity>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: FunctionEntity) {
        self.functions.push(function);
    }

    pub fn functions(&self) -> &[FunctionEntity] {
        &self.functions
    }

    pub fn get_function(&self, index: usize) -> Option<&FunctionEntity> {
        self.functions.get(index)
    }

    pub fn get_function_by_name(&self, name: impl Into<ByteString>) -> Option<&FunctionEntity> {
        let name = name.into();

        self.functions.iter().find(|f| f.name.symbol() == &name)
    }
}
