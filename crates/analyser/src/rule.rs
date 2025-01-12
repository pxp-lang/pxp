use pxp_ast::{visitor::Ancestors, Node};

use crate::AnalyserContext;

pub trait Rule {
    /// Determine if the rule should be execute for the given node and ancestor tree.
    fn should_run(&self, node: &Node, ancestors: &Ancestors) -> bool;

    /// Execute the rule for the given node and ancestor tree.
    fn run(&self, node: &Node, ancestors: &Ancestors, context: &mut AnalyserContext);

    /// Returns the name of the rule.
    fn name(&self) -> &'static str;
}
