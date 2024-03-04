use pxp_indexer::{Index, deserialize_index};
use pxp_symbol::SymbolTable;

pub const PHP_80_RAW_INDEX: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/8.0.index"));
pub const PHP_81_RAW_INDEX: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/8.1.index"));
pub const PHP_82_RAW_INDEX: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/8.2.index"));
pub const PHP_83_RAW_INDEX: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/8.3.index"));

pub fn php_80_index() -> (Index, SymbolTable) {
    deserialize_index(PHP_80_RAW_INDEX)
}

pub fn php_81_index() -> (Index, SymbolTable) {
    deserialize_index(PHP_81_RAW_INDEX)
}

pub fn php_82_index() -> (Index, SymbolTable) {
    deserialize_index(PHP_82_RAW_INDEX)
}

pub fn php_83_index() -> (Index, SymbolTable) {
    deserialize_index(PHP_83_RAW_INDEX)
}

pub fn php_latest_index() -> (Index, SymbolTable) {
    php_83_index()
}