use crate::rule::Rule;

/// An API for collecting analysis rules into smaller groups.
pub trait RuleCollection {
    /// Returns a collection of analysis rules.
    fn rules(&self) -> Vec<Box<dyn Rule>>;

    /// Returns the name of the collection.
    fn name(&self) -> &'static str;
}
