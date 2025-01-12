use std::{path::Path, task::Context};

use pxp_ast::{visitor::{Ancestors, NodeVisitor, NodeVisitorEscapeHatch}, Node, Statement};
use pxp_inference::TypeEngine;

use crate::{AnalyserContext, Reporter, Rule, RuleCollection};

pub struct Runner<'a> {
    rules: Vec<Box<dyn Rule>>,
    type_engine: &'a TypeEngine<'a>,
}

impl<'a> Runner<'a> {
    pub fn new(type_engine: &'a TypeEngine) -> Self {
        Self {
            rules: Vec::new(),
            type_engine,
        }
    }

    pub fn add_rule(&mut self, rule: impl Rule + 'static) {
        self.rules.push(Box::new(rule));
    }

    pub fn add_collection(&mut self, collection: impl RuleCollection + 'static) {
        for rule in collection.rules() {
            self.rules.push(rule);
        }
    }

    pub fn run(&self, file: usize, reporter: &mut Reporter, ast: &[Statement]) {
        let types = self.type_engine.infer(ast);
        let mut context = AnalyserContext::new(reporter, types, file);
        let mut visitor = AnalyserVisitor::new(self, &mut context);

        visitor.traverse(ast);
    }
}

struct AnalyserVisitor<'a> {
    runner: &'a Runner<'a>,
    context: &'a mut AnalyserContext<'a>,
}

impl<'a> AnalyserVisitor<'a> {
    fn new(runner: &'a Runner, context: &'a mut AnalyserContext<'a>) -> Self {
        Self { runner, context }
    }
}

impl<'a> NodeVisitor<'a> for AnalyserVisitor<'a> {
    fn enter(&mut self, node: Node<'a>, ancestors: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        for rule in &self.runner.rules {
            if rule.should_run(&node, ancestors) {
                rule.run(&node, ancestors, self.context);
            }
        }

        NodeVisitorEscapeHatch::Continue
    }
}
