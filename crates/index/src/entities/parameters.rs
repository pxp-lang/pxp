use pxp_ast::{Name, SimpleVariable};
use pxp_type::Type;

use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    parameters: Vec<Parameter>,
}

impl Parameters {
    pub fn new(parameters: Vec<Parameter>) -> Self {
        Self {
            parameters
        }
    }

    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub(crate) name: SimpleVariable,
    pub(crate) r#type: Type<Name>,
    pub(crate) optional: bool,
    pub(crate) location: Location,
}
