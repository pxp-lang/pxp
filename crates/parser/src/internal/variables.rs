use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::*;

use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_optional_simple_variable(&mut self) -> Option<SimpleVariable> {
        if self.current_kind() == TokenKind::Variable {
            Some(self.parse_simple_variable())
        } else {
            None
        }
    }

    pub fn parse_simple_variable(&mut self) -> SimpleVariable {
        match self.current_kind() {
            TokenKind::Variable => {
                let symbol = self.current_symbol_as_bytestring();
                let span = self.current_span();

                self.next();

                let stripped = ByteString::from(&symbol[1..]);

                SimpleVariable {
                    id: self.id(),
                    symbol,
                    stripped,
                    span,
                }
            }
            TokenKind::Dollar => {
                let span = self.next();

                self.diagnostic(
                    ParserDiagnostic::DynamicVariableNotAllowed,
                    Severity::Error,
                    span,
                );

                SimpleVariable {
                    id: self.id(),
                    symbol: ByteString::empty(),
                    stripped: ByteString::empty(),
                    span,
                }
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                SimpleVariable::missing(self.id(), self.current_span())
            }
        }
    }

    pub fn parse_dynamic_variable(&mut self) -> Variable {
        match self.current_kind() {
            TokenKind::Variable => {
                let symbol = self.current_symbol_as_bytestring();
                let span = self.next();
                let stripped = ByteString::from(&symbol[1..]);

                Variable::SimpleVariable(SimpleVariable {
                    id: self.id(),
                    symbol,
                    stripped,
                    span,
                })
            }
            TokenKind::DollarLeftBrace => {
                let start = self.next();
                let expr = self.parse_expression();
                let end = self.skip_right_brace();

                Variable::BracedVariableVariable(BracedVariableVariable {
                    id: self.id(),
                    span: Span::combine(start, end),
                    start,
                    variable: Box::new(expr),
                    end,
                })
            }
            TokenKind::Dollar if self.peek_kind() == TokenKind::LeftBrace => {
                let start = self.next();
                self.next();

                let expr = self.parse_expression();

                let end = self.skip_right_brace();

                Variable::BracedVariableVariable(BracedVariableVariable {
                    id: self.id(),
                    span: Span::combine(start, end),
                    start,
                    variable: Box::new(expr),
                    end,
                })
            }
            TokenKind::Dollar => {
                let span = self.next();

                match self.current_kind() {
                    TokenKind::Dollar | TokenKind::Variable => {
                        let variable = self.parse_dynamic_variable();

                        Variable::VariableVariable(VariableVariable {
                            id: self.id(),
                            span,
                            variable: Box::new(variable),
                        })
                    }
                    // This allows us to handle standalone $ tokens, i.e. incomplete variables.
                    _ => {
                        self.diagnostic(
                            ParserDiagnostic::ExpectedToken {
                                expected: vec![TokenKind::Variable],
                                found: self.current().to_owned(),
                            },
                            Severity::Error,
                            self.current_span(),
                        );

                        Variable::SimpleVariable(SimpleVariable::missing(
                            self.id(),
                            self.current_span(),
                        ))
                    }
                }
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                Variable::SimpleVariable(SimpleVariable::missing(
                    self.id(),
                    self.current_span(),
                ))
            }
        }
    }
}
