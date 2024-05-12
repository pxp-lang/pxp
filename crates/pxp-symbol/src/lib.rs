use std::{collections::HashMap, fmt::{Debug, Display}, mem::MaybeUninit, sync::Once};

use pxp_bytestring::ByteStr;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Symbol {
    pub id: u32,
    pub len: u32,
}

impl Symbol {
    pub fn new(id: u32, len: u32) -> Self {
        Self { id, len }
    }

    #[inline]
    pub fn missing() -> Self {
        Self::new(0, 0)
    }

    #[inline]
    pub const fn is_missing(&self) -> bool {
        self.id == 0
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(contents) = SymbolTable::the().resolve(*self) {
            write!(f, "Symbol(\"{}\")", contents)
        } else {
            write!(f, "Symbol({})", self.id)
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(contents) = SymbolTable::the().resolve(*self) {
            write!(f, "{}", contents)
        } else {
            write!(f, "Symbol({})", self.id)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SymbolTable {
    map: HashMap<Vec<u8>, Symbol>,
    vec: Vec<Vec<u8>>,
}

fn singleton() -> &'static mut SymbolTable {
    static mut SINGLETON: MaybeUninit<SymbolTable> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            SINGLETON.write(SymbolTable::new());
        });

        &mut *SINGLETON.as_mut_ptr()
    }
}

impl SymbolTable {
    pub fn the() -> &'static mut Self {
        singleton()
    }

    pub fn new() -> Self {
        let mut table = Self {
            map: HashMap::new(),
            vec: Vec::new(),
        };

        // This will reserve Symbol(0) for internal use (missing tokens, etc.)
        table.intern(b"");
        table
    }

    pub fn intern(&mut self, contents: &[u8]) -> Symbol {
        if let Some(symbol) = self.map.get(contents) {
            return *symbol;
        }

        let symbol = self.vec.len() as u32;

        self.map.insert(contents.to_vec(), Symbol::new(symbol, contents.len() as u32));
        self.vec.push(contents.to_vec());

        Symbol::new(symbol, contents.len() as u32)
    }

    pub fn find(&self, contents: &[u8]) -> Option<Symbol> {
        self.map.get(contents).copied()
    }

    pub fn must_find(&self, contents: &[u8]) -> Symbol {
        self.find(contents).unwrap_or_else(|| panic!("Symbol for {} not found", ByteStr::from(contents)))
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<ByteStr> {
        self.vec.get(symbol.id as usize).map(|s| ByteStr::new(s))
    }

    pub fn must_resolve(&self, symbol: Symbol) -> ByteStr {
        self.resolve(symbol).unwrap_or_else(|| panic!("Symbol {} not found", symbol))
    }

    pub fn coagulate(&mut self, symbols: &[Symbol], with: Option<&[u8]>) -> Symbol {
        let mut contents = Vec::new();

        for (i, symbol) in symbols.iter().enumerate() {
            contents.extend_from_slice(&self.resolve(*symbol).unwrap());

            if i < symbols.len() - 1 {
                if let Some(with) = with {
                    contents.extend_from_slice(with);
                }
            }
        }

        self.intern(&contents)
    }
}

#[cfg(test)]
mod tests {
    use pxp_bytestring::ByteStr;

    use crate::Symbol;

    use super::SymbolTable;

    #[test]
    fn it_can_be_created() {
        let _ = SymbolTable::new();
    }

    #[test]
    fn it_can_intern_a_symbol() {
        let mut symbols = SymbolTable::new();
        let sample_text = b"Hello, world!";

        assert_eq!(symbols.intern(sample_text), Symbol {
            id: 1,
            len: 13,
        });
    }

    #[test]
    fn it_can_resolve_a_symbol() {
        let mut symbols = SymbolTable::new();
        let sample_text = b"Hello, world!";

        let symbol = symbols.intern(sample_text);

        assert_eq!(symbols.resolve(symbol), Some(ByteStr::new(sample_text)));
    }
}
