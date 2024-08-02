use std::path::PathBuf;

pub fn file_is_newer_than(file: PathBuf, other: PathBuf) -> bool {
    let file_metadata = std::fs::metadata(file).unwrap();
    let other_metadata = std::fs::metadata(other).unwrap();

    file_metadata.modified().unwrap() > other_metadata.modified().unwrap()
}
