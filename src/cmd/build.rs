use pxp::visitors::variable::VariableVisitor;
use pxp_parser::{parse, traverser::Visitor};

use super::BuildCommand;

pub fn run(args: BuildCommand) {
    let file = args.file.unwrap();
    let contents = std::fs::read(&file).unwrap();
    let mut ast = parse(&contents).unwrap();
    let mut visitor = VariableVisitor::new();
    for node in ast.iter_mut() {
        visitor.visit_node(node);
    }
    dbg!(ast);
}