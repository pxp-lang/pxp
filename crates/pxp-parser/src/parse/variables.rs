use pxp_ast::{SimpleVariable, Variable, BracedVariable, VariableKind, VariableVariable};
use pxp_token::TokenKind;

use crate::state::ParserState;

use super::{utils::{unexpected_token, self}, expressions};

pub fn simple_variable(state: &mut ParserState) -> SimpleVariable {
    let current = state.stream.current();
    if let TokenKind::Variable = &current.kind {
        let span = current.span;
        
        state.stream.next();

        return SimpleVariable { name: current.clone(), span }
    }

    unexpected_token(state, &[TokenKind::Variable]);
    SimpleVariable::missing(current.span.with_start_as_end())
}

pub fn dynamic_variable(state: &mut ParserState) -> Variable {
    let current = state.stream.current();
    match &current.kind {
        TokenKind::Variable => {
            let name = state.stream.current().clone();
            let span = name.span;
            state.stream.next();

            Variable { kind: VariableKind::Simple(SimpleVariable { name, span }), span }
        }
        TokenKind::DollarLeftBrace => {
            state.stream.next();

            let expr = expressions::create(state);
            let span = expr.span;

            utils::skip_right_brace(state);

            Variable { kind: VariableKind::Braced(BracedVariable { name: Box::new(expr), span }), span }
        }
        // todo(azjezz): figure out why the lexer does this.
        TokenKind::Dollar if state.stream.peek().kind == TokenKind::LeftBrace => {
            state.stream.next();
            state.stream.next();

            let expr = expressions::create(state);
            let span = expr.span;
            utils::skip_right_brace(state);

            Variable { kind: VariableKind::Braced(BracedVariable { name: Box::new(expr), span }), span }
        }
        TokenKind::Dollar => {
            state.stream.next();

            let variable = dynamic_variable(state);
            let span = variable.span;

            Variable {
                kind: VariableKind::Variable(VariableVariable { name: Box::new(variable), span }),
                span
            }
        }
        _ => {
            let span = current.span;
            unexpected_token(state, &[TokenKind::Variable, TokenKind::DollarLeftBrace, TokenKind::Dollar]);
            Variable { kind: VariableKind::Simple(SimpleVariable::missing(span)), span }
        }
    }
}
