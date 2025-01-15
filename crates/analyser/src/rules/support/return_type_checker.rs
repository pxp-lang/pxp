use pxp_ast::{ResolvedName, ReturnStatement, ReturnType};
use pxp_type::Type;

use crate::AnalyserContext;

pub(crate) struct ReturnTypeChecker;

pub(crate) enum ReturnTypeCheckResult {
    Ok,
    Empty,
    Void,
    TypeMismatch,
    Never,
}

impl ReturnTypeChecker {
    pub(crate) fn check_return_type(
        context: &AnalyserContext,
        expected: &Type<ResolvedName>,
        node: &ReturnStatement,
    ) -> ReturnTypeCheckResult {
        if expected.is_never() {
            return ReturnTypeCheckResult::Never;
        }

        if !expected.is_void() && node.value.is_none() {
            return ReturnTypeCheckResult::Empty;
        }

        if expected.is_void() && node.value.is_some() {
            return ReturnTypeCheckResult::Void;
        }

        ReturnTypeCheckResult::Ok
    }
}
