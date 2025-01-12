use pxp_ast::{ReturnStatement, Statement};

mod return_finder;

pub(crate) fn find_returns_in_block<'a>(block: &'a [Statement]) -> Vec<&'a ReturnStatement> {
    return_finder::ReturnFinder::find(block)
}
