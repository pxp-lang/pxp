mod function;
mod parameters;

pub use function::ReflectionFunction;
pub use parameters::ReflectionParameter;

pub trait Reflector {
    type Reflection;

    fn reflect(&self) -> Self::Reflection;
}
