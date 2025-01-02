use crate::Parameter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionParameter<'a, O: CanReflectParameters> {
    entity: &'a Parameter,
    owner: O,
}

impl<'a, O: CanReflectParameters> ReflectionParameter<'a, O> {
    pub fn new(entity: &'a Parameter, owner: O) -> Self {
        Self { entity, owner }
    }

    pub fn is_optional(&self) -> bool {
        todo!()
    }

    pub fn is_variadic(&self) -> bool {
        todo!()
    }
}

pub(crate) trait CanReflectParameters {}

pub trait ReflectsParameters<'a, O: CanReflectParameters> : CanReflectParameters {
    fn get_parameters(&self) -> Vec<ReflectionParameter<'a, O>>;
    
    fn get_number_of_parameters(&self) -> usize {
        self.get_parameters().len()
    }

    fn get_number_of_required_parameters(&self) -> usize {
        self.get_parameters().iter().filter(|p| !p.is_optional()).count()
    }

    fn is_variadic(&self) -> bool {
        self.get_parameters().iter().any(|p| p.is_variadic())
    }
}
