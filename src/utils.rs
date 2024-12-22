use std::path::{Path, PathBuf};

pub(crate) fn find_php_files_in(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut find_php_files_in(&path)?);
        } else if path.extension().map_or(false, |ext| ext == "php") {
            files.push(path);
        }
    }

    Ok(files)
}
