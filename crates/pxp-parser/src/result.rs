use std::fmt::Debug;

use pxp_ast::Program;
use pxp_span::Span;
use pxp_token::Token;

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub program: Program,
    pub errors: Vec<ParseError>,
}

#[derive(Clone)]
pub enum ParseError {
    UnexpectedToken { token: Token, expected: Vec<String> },
    UnbracedNamespaceWithoutName { span: Span },
    ReservedKeywordInTypeName { span: Span, token: Token },
    UnexpectedEndOfFile { span: Span },
}   

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken { token, expected } => {
                write!(f, "Unexpected {:?} at {:?}, expected one of: {:?}", token, token.span, expected)
            },
            Self::UnbracedNamespaceWithoutName { span } => {
                write!(f, "Unbraced namespace without name at {:?}", span)
            },
            Self::ReservedKeywordInTypeName { span, token } => {
                write!(f, "Cannot use reserved keyword {:?} as type name at {:?}", token, span)
            },
            Self::UnexpectedEndOfFile { span } => {
                write!(f, "Unexpected end of file at {:?}", span)
            }
        }
    }
}