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
        let current = self.current();

        if TokenKind::Variable == current.kind {
            Some(parse_simple_variable())
        } else {
            None
        }
    }

    pub fn parse_simple_variable(&mut self) -> SimpleVariable {
        let current = self.current();

        match &current.kind {
            TokenKind::Variable => {
                self.next();

                let symbol = current.symbol.as_ref().unwrap();

                let name = symbol.clone();
                let stripped = ByteString::from(&name[1..]);

                SimpleVariable {
                    id: self.state.id(),
                    symbol: current.symbol.as_ref().unwrap().clone(),
                    stripped,
                    span: current.span,
                }
            }
            TokenKind::Dollar => {
                self.next();

                self.diagnostic(
                    ParserDiagnostic::DynamicVariableNotAllowed,
                    Severity::Error,
                    current.span,
                );

                SimpleVariable {
                    id: self.state.id(),
                    symbol: ByteString::empty(),
                    stripped: ByteString::empty(),
                    span: current.span,
                }
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                SimpleVariable::missing(self.state.id(), current.span)
            }
        }
    }

    pub fn parse_dynamic_variable(&mut self) -> Variable {
        let current = self.current();
        match &current.kind {
            TokenKind::Variable => {
                self.next();

                let symbol = current.symbol.as_ref().unwrap();
                let name = symbol.clone();
                let stripped = ByteString::from(&name[1..]);

                Variable::SimpleVariable(SimpleVariable {
                    id: self.state.id(),
                    symbol: current.symbol.as_ref().unwrap().clone(),
                    stripped,
                    span: current.span,
                })
            }
            TokenKind::DollarLeftBrace => {
                let start = current.span;
                self.next();

                let expr = self.parse_expression();

                let end = self.skip_right_brace();

                Variable::BracedVariableVariable(BracedVariableVariable {
                    id: self.state.id(),
                    span: Span::combine(start, end),
                    start,
                    variable: Box::new(expr),
                    end,
                })
            }
            TokenKind::Dollar if state.peek().kind == TokenKind::LeftBrace => {
                let start = current.span;
                self.next();
                self.next();

                let expr = self.parse_expression();

                let end = self.skip_right_brace();

                Variable::BracedVariableVariable(BracedVariableVariable {
                    id: self.state.id(),
                    span: Span::combine(start, end),
                    start,
                    variable: Box::new(expr),
                    end,
                })
            }
            TokenKind::Dollar => {
                let span = current.span;
                self.next();

                match self.current_kind() {
                    TokenKind::Dollar | TokenKind::Variable => {
                        let variable = parse_dynamic_variable();

                        Variable::VariableVariable(VariableVariable {
                            id: self.state.id(),
                            span,
                            variable: Box::new(variable),
                        })
                    }
                    // This allows us to handle standalone $ tokens, i.e. incomplete variables.
                    _ => {
                        self.diagnostic(
                            ParserDiagnostic::ExpectedToken {
                                expected: vec![TokenKind::Variable],
                                found: current.clone(),
                            },
                            Severity::Error,
                            current.span,
                        );

                        Variable::SimpleVariable(SimpleVariable::missing(self.state.id(), current.span))
                    }
                }
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: current.clone(),
                    },
                    Severity::Error,
                    current.span,
                );

                Variable::SimpleVariable(SimpleVariable::missing(self.state.id(), current.span))
            }
        }
    }
}
