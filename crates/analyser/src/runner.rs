use std::{path::Path, task::Context};

use pxp_ast::{
    visitor::{Ancestors, NodeVisitor, NodeVisitorEscapeHatch},
    FunctionStatement, Node, Statement,
};
use pxp_index::Index;
use pxp_inference::TypeEngine;

use crate::{AnalyserContext, Reporter, Rule, RuleCollection};

pub struct Runner<'a> {
    rules: Vec<Box<dyn Rule>>,
    type_engine: &'a TypeEngine<'a>,
    index: &'a Index,
}

impl<'a> Runner<'a> {
    pub fn new(type_engine: &'a TypeEngine, index: &'a Index) -> Self {
        Self {
            rules: Vec::new(),
            type_engine,
            index,
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
        let mut context = AnalyserContext::new(reporter, types, &self.index, file);
        let mut visitor = AnalyserVisitor::new(self, &mut context);

        visitor.traverse(ast);
    }
}

struct AnalyserVisitor<'a> {
    runner: &'a Runner<'a>,
    context: &'a mut AnalyserContext<'a, 'a>,
}

impl<'a> AnalyserVisitor<'a> {
    fn new(runner: &'a Runner, context: &'a mut AnalyserContext<'a, 'a>) -> Self {
        Self { runner, context }
    }

    fn enter_function(&mut self, node: &FunctionStatement) {
        self.context.scope.function = Some(
            self.context
                .index
                .get_function(node.name.symbol().to_owned())
                .unwrap(),
        );
    }

    fn leave_function(&mut self) {
        self.context.scope.function = None;
    }
}

impl<'a> NodeVisitor<'a> for AnalyserVisitor<'a> {
    fn enter(&mut self, node: Node<'a>, ancestors: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        if node.is_function_statement() {
            self.enter_function(node.as_function_statement().unwrap());
        }

        for rule in &self.runner.rules {
            if rule.should_run(&node, ancestors) {
                rule.run(&node, ancestors, self.context);
            }
        }

        NodeVisitorEscapeHatch::Continue
    }

    fn leave(&mut self, node: Node<'a>, _: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        if node.is_function_statement() {
            self.leave_function();
        }

        NodeVisitorEscapeHatch::Continue
    }
}
