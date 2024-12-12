use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use crate::{expressions, Parser};
use pxp_ast::*;

use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_optional_simple_variable(&mut self) -> Option<SimpleVariable> {
        let current = state.current();

        if TokenKind::Variable == current.kind {
            Some(parse_simple_variable(state))
        } else {
            None
        }
    }

    pub fn parse_simple_variable(&mut self) -> SimpleVariable {
        let current = state.current();

        match &current.kind {
            TokenKind::Variable => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();

                let name = symbol.clone();
                let stripped = ByteString::from(&name[1..]);

                SimpleVariable {
                    id: state.id(),
                    symbol: current.symbol.as_ref().unwrap().clone(),
                    stripped,
                    span: current.span,
                }
            }
            TokenKind::Dollar => {
                state.next();

                state.diagnostic(
                    ParserDiagnostic::DynamicVariableNotAllowed,
                    Severity::Error,
                    current.span,
                );

                SimpleVariable {
                    id: state.id(),
                    symbol: ByteString::empty(),
                    stripped: ByteString::empty(),
                    span: current.span,
                }
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                SimpleVariable::missing(state.id(), current.span)
            }
        }
    }

    pub fn parse_dynamic_variable(&mut self) -> Variable {
        let current = state.current();
        match &current.kind {
            TokenKind::Variable => {
                state.next();

                let symbol = current.symbol.as_ref().unwrap();
                let name = symbol.clone();
                let stripped = ByteString::from(&name[1..]);

                Variable::SimpleVariable(SimpleVariable {
                    id: state.id(),
                    symbol: current.symbol.as_ref().unwrap().clone(),
                    stripped,
                    span: current.span,
                })
            }
            TokenKind::DollarLeftBrace => {
                let start = current.span;
                state.next();

                let expr = expressions::create(state);

                let end = utils::skip_right_brace(state);

                Variable::BracedVariableVariable(BracedVariableVariable {
                    id: state.id(),
                    span: Span::combine(start, end),
                    start,
                    variable: Box::new(expr),
                    end,
                })
            }
            TokenKind::Dollar if state.peek().kind == TokenKind::LeftBrace => {
                let start = current.span;
                state.next();
                state.next();

                let expr = expressions::create(state);

                let end = utils::skip_right_brace(state);

                Variable::BracedVariableVariable(BracedVariableVariable {
                    id: state.id(),
                    span: Span::combine(start, end),
                    start,
                    variable: Box::new(expr),
                    end,
                })
            }
            TokenKind::Dollar => {
                let span = current.span;
                state.next();

                match state.current().kind {
                    TokenKind::Dollar | TokenKind::Variable => {
                        let variable = parse_dynamic_variable(state);

                        Variable::VariableVariable(VariableVariable {
                            id: state.id(),
                            span,
                            variable: Box::new(variable),
                        })
                    }
                    // This allows us to handle standalone $ tokens, i.e. incomplete variables.
                    _ => {
                        state.diagnostic(
                            ParserDiagnostic::ExpectedToken {
                                expected: vec![TokenKind::Variable],
                                found: current.clone(),
                            },
                            Severity::Error,
                            current.span,
                        );

                        Variable::SimpleVariable(SimpleVariable::missing(state.id(), current.span))
                    }
                }
            }
            _ => {
                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                Variable::SimpleVariable(SimpleVariable::missing(state.id(), current.span))
            }
        }
    }
}
