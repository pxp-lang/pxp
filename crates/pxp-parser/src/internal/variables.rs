use crate::error::ParseResult;
use crate::expected_token_err;
use crate::expressions;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::variables::BracedVariableVariable;
use pxp_ast::variables::SimpleVariable;
use pxp_ast::variables::Variable;
use pxp_ast::variables::VariableVariable;
use pxp_token::TokenKind;

pub fn simple_variable(state: &mut State) -> ParseResult<SimpleVariable> {
    let current = state.stream.current();
    if let TokenKind::Variable = &current.kind {
        state.stream.next();

        return Ok(SimpleVariable { token: *current });
    }

    expected_token_err!("a variable", state)
}

pub fn dynamic_variable(state: &mut State) -> ParseResult<Variable> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Variable => {
            state.stream.next();

            Ok(Variable::SimpleVariable(SimpleVariable { token: *current }))
        }
        TokenKind::DollarLeftBrace => {
            let start = current.span;
            state.stream.next();

            let expr = expressions::create(state)?;

            let end = utils::skip_right_brace(state)?;

            Ok(Variable::BracedVariableVariable(BracedVariableVariable {
                start,
                variable: Box::new(expr),
                end,
            }))
        }
        // FIXME: figure out why the lexer does this.
        TokenKind::Dollar if state.stream.peek().kind == TokenKind::LeftBrace => {
            let start = current.span;
            state.stream.next();
            state.stream.next();

            let expr = expressions::create(state)?;

            let end = utils::skip_right_brace(state)?;

            Ok(Variable::BracedVariableVariable(BracedVariableVariable {
                start,
                variable: Box::new(expr),
                end,
            }))
        }
        TokenKind::Dollar => {
            let span = current.span;
            state.stream.next();

            let variable = dynamic_variable(state)?;

            Ok(Variable::VariableVariable(VariableVariable {
                span,
                variable: Box::new(variable),
            }))
        }
        _ => {
            expected_token_err!("a variable", state)
        }
    }
}
