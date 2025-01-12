use pxp_ast::{visitor::Ancestors, Node};

use crate::Rule;

pub struct ReturnTypeRule;

impl Rule for ReturnTypeRule {
    fn should_run(&self, node: &Node, ancestors: &Ancestors) -> bool {
        false
    }

    fn run(&self, node: &Node, ancestors: &Ancestors, context: &mut crate::AnalyserContext) {
        
    }

    fn name(&self) -> &'static str {
        "functions/return_type"
    }
}
