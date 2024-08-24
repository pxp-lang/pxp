use std::collections::HashMap;

use pxp_bytestring::ByteString;
use pxp_type::Type;

#[derive(Debug)]
pub struct Scope {
    pub(crate) variables: HashMap<ByteString, Type<ByteString>>,
}

impl Scope {
    pub(crate) fn insert(&mut self, variable: ByteString, ty: Type<ByteString>) {
        self.variables.insert(variable, ty);
    }

    pub(crate) fn get(&self, variable: &ByteString) -> &Type<ByteString> {
        self.variables.get(variable).unwrap_or_else(|| &Type::Mixed)
    }
}