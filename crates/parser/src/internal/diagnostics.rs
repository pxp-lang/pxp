use pxp_diagnostics::Severity;
use pxp_token::TokenKind;

use crate::{Parser, ParserDiagnostic};

impl<'a> Parser<'a> {
    pub(crate) fn unexpected_end_of_file(&mut self) {
        self.diagnostic(ParserDiagnostic::UnexpectedEndOfFile, Severity::Error, self.current_span());
    }

    pub(crate) fn expected_token(&mut self, kind: TokenKind) {
        self.diagnostic(ParserDiagnostic::ExpectedToken { expected: kind, found: self.current().to_owned() }, Severity::Error, self.current_span());
    }

    pub(crate) fn expected_any_of_tokens(&mut self, kinds: &[TokenKind]) {
        self.diagnostic(ParserDiagnostic::ExpectedOneOfTokens { expected: kinds.to_vec(), found: self.current().to_owned() }, Severity::Error, self.current_span());
    }

    pub(crate) fn cannot_use_reserved_keyword_as_type_name(&mut self){ 
        self.diagnostic(ParserDiagnostic::CannotUseReservedKeywordAsTypeName, Severity::Error, self.current_span());
    }
}
