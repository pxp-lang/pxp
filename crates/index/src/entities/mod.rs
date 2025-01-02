mod function;
mod parameters;
mod class;

pub use function::FunctionEntity;
pub use parameters::{Parameters, Parameter};
pub use class::{ClassEntity, ClassEntityKind};
use pxp_bytestring::ByteString;

#[derive(Debug, Clone)]
pub(crate) struct EntityRegistry {
    functions: Vec<FunctionEntity>,
    classes: Vec<ClassEntity>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            classes: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: FunctionEntity) {
        self.functions.push(function);
    }

    pub fn functions(&self) -> &[FunctionEntity] {
        &self.functions
    }

    pub fn get_function(&self, name: impl Into<ByteString>) -> Option<&FunctionEntity> {
        let name = name.into();

        self.functions.iter().find(|f| &f.name.resolved == &name)
    }

    pub fn add_class(&mut self, class: ClassEntity) {
        self.classes.push(class);
    }

    pub fn classes(&self) -> &[ClassEntity] {
        &self.classes
    }

    pub fn get_class(&self, name: impl Into<ByteString>) -> Option<&ClassEntity> {
        let name = name.into();

        self.classes.iter().find(|c| &c.name.resolved == &name)
    }
}
