use std::path::PathBuf;

use pxp_symbol::SymbolTable;

use crate::Index;

pub fn try_load_index_from_cache(directory: &PathBuf) -> Option<(Index, SymbolTable)> {
    // Try to load a serialised version of the index from the system cache directory.
    // If the file doesn't exist, or if it fails to load, return None.
    // Otherwise, return the loaded index.
    //
    // The cache directory is ~/.cache/pxp-indexer.
    // The cache file is ~/.cache/pxp-indexer/{directory}.index

    let cache_dir = dirs::cache_dir().unwrap().join("pxp-indexer");
    let cache_file = cache_dir.join(directory.file_name().unwrap()).with_extension("index");

    if !cache_file.exists() {
        return None;
    }

    match std::fs::read(cache_file) {
        Ok(contents) => match bincode::deserialize::<(Index, SymbolTable)>(&contents) {
            Ok(index) => Some(index),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn write_index_to_cache(result: (&Index, &SymbolTable), directory: &PathBuf) {
    let cache_dir = dirs::cache_dir().unwrap().join("pxp-indexer");
    let cache_file = cache_dir.join(directory.file_name().unwrap()).with_extension("index");

    if !cache_dir.exists() {
        std::fs::create_dir_all(cache_dir).unwrap();
    }

    let contents = bincode::serialize(&result).unwrap();

    std::fs::write(cache_file, contents).unwrap();
}

pub fn serialize_index(result: (&Index, &SymbolTable)) -> Vec<u8> {
    bincode::serialize(&result).unwrap()
}

pub fn deserialize_index(contents: &[u8]) -> (Index, SymbolTable) {
    bincode::deserialize::<(Index, SymbolTable)>(contents).unwrap()
}
