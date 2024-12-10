use pxp_ast::Name;
use pxp_span::Spanned;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_optional_name(&mut self) -> Option<Name> {
        todo!()
    }

    pub(crate) fn parse_use_name(&mut self) -> Name {
        let identifier = self.parse_full_type_identifier();

        if identifier.is_missing() {
            return Name::missing(self.id(), identifier.span);
        }

        Name::resolved(self.id(), identifier.symbol.clone(), identifier.symbol, identifier.span)
    }
}
