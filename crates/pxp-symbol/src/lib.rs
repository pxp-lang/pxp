use std::collections::HashMap;

use pxp_bytestring::ByteStr;

pub type Symbol = usize;

#[derive(Default, Debug, Clone)]
pub struct SymbolTable<'s> {
    map: HashMap<&'s [u8], Symbol>,
    vec: Vec<&'s [u8]>,
}

impl<'s> SymbolTable<'s> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, contents: &'s [u8]) -> Symbol {
        if let Some(symbol) = self.map.get(contents) {
            return *symbol
        }

        let symbol = self.vec.len();
        
        self.map.insert(contents, symbol);
        self.vec.push(contents);

        symbol
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<ByteStr<'s>> {
        self.vec.get(symbol).map(|s| ByteStr::new(s))
    }
}

#[cfg(test)]
mod tests {
    use pxp_bytestring::ByteStr;

    use super::SymbolTable;

    #[test]
    fn it_can_be_created() {
        let _ = SymbolTable::new();
    }

    #[test]
    fn it_can_intern_a_symbol() {
        let mut symbols = SymbolTable::new();
        let sample_text = b"Hello, world!";

        assert_eq!(symbols.intern(sample_text), 0);
    }

    #[test]
    fn it_can_resolve_a_symbol() {
        let mut symbols = SymbolTable::new();
        let sample_text = b"Hello, world!";

        let symbol = symbols.intern(sample_text);

        assert_eq!(symbols.resolve(symbol), Some(ByteStr::new(sample_text)));
    }
}