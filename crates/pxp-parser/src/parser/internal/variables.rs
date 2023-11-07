use crate::expected_token_err;
use crate::lexer::token::TokenKind;
use crate::parser::ast::variables::BracedVariableVariable;
use crate::parser::ast::variables::SimpleVariable;
use crate::parser::ast::variables::Variable;
use crate::parser::ast::variables::VariableVariable;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::utils;
use crate::parser::state::State;

pub fn simple_variable(state: &mut State) -> ParseResult<SimpleVariable> {
    let current = state.stream.current();
    if let TokenKind::Variable = &current.kind {
        let span = current.span;
        let name = current.value.clone();
        state.stream.next();

        return Ok(SimpleVariable { span, name });
    }

    expected_token_err!("a variable", state)
}

pub fn dynamic_variable(state: &mut State) -> ParseResult<Variable> {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Variable => {
            let span = current.span;
            let name = current.value.clone();
            state.stream.next();

            Ok(Variable::SimpleVariable(SimpleVariable { span, name }))
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
        // todo(azjezz): figure out why the lexer does this.
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
