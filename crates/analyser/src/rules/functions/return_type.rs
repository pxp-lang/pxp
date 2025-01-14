use pxp_ast::{visitor::Ancestors, Node};
use pxp_diagnostics::{DiagnosticLabel, Severity};
use pxp_index::{HasLocation, ReflectionFunctionLike};
use pxp_span::IsSpanned;

use crate::{rules::support::{ReturnTypeCheckResult, ReturnTypeChecker}, AnalyserContext, AnalyserDiagnostic, Rule};

pub struct ReturnTypeRule;

impl Rule for ReturnTypeRule {
    fn should_run(&self, node: &Node, _: &Ancestors) -> bool {
        node.is_return_statement()
    }

    fn run(&self, node: &Node, _: &Ancestors, context: &mut AnalyserContext) {
        let Some(function) = context.get_function() else {
            return
        };

        let Some(return_type) = function.get_return_type() else {
            return
        };

        let Some(node) = node.as_return_statement() else {
            unreachable!()
        };

        let result = ReturnTypeChecker::check_return_type(context, return_type, node);

        match result {
            ReturnTypeCheckResult::Empty => context.report(
                AnalyserDiagnostic::new()
                    .code("R0001")
                    .identifier("function/return-empty")
                    .help("Add a value to the return.")
                    .labels(vec![
                        DiagnosticLabel::secondary(function.location().span(), "Function declared here."),
                        DiagnosticLabel::primary(node.span, "Empty return statement here.")
                    ])
                    .message(format!("Function {}() should return {} but empty return statement found.", function.get_name(), return_type)),
                Severity::Error,
                node.span,
            ),
            ReturnTypeCheckResult::Void => context.report(
                AnalyserDiagnostic::new()
                    .code("R0002")
                    .identifier("function/return-void")
                    .help("Remove the value from the return.")
                    .labels(vec![
                        DiagnosticLabel::secondary(function.location().span(), "Function declared here."),
                        DiagnosticLabel::primary(node.span, "Return value is declared here.")
                    ])
                    .message(format!("Function {}() with return type void returns {}, but shouldn't return anything.", function.get_name(), context.get_type(node.value.as_ref().unwrap().id))),
                Severity::Error,
                node.span,
            ),
            ReturnTypeCheckResult::TypeMismatch => todo!(),
            ReturnTypeCheckResult::Never => context.report(
                AnalyserDiagnostic::new()
                    .code("R0004")
                    .identifier("function/return-never")
                    .help("Try to remove the return statement.")
                    .labels(vec![
                        DiagnosticLabel::secondary(function.location().span(), "Function declared here."),
                        DiagnosticLabel::primary(node.span, "Return statement is not allowed in this function.")
                    ])
                    .message(format!("Function {}() should never return but return statement found.", function.get_name())),
                Severity::Error,
                node.span,
            ),
            ReturnTypeCheckResult::Ok => {},
        }
    }

    fn name(&self) -> &'static str {
        "functions/return_type"
    }
}
