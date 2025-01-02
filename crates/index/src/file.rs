use std::{collections::HashMap, path::{Path, PathBuf}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(usize);

pub trait HasFileId {
    fn file_id(&self) -> FileId;
}

impl HasFileId for FileId {
    fn file_id(&self) -> FileId {
        *self
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FileRegistry {
    files: HashMap<PathBuf, FileId>,
}

impl FileRegistry {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn get(&self, path: &PathBuf) -> FileId {
        *self.files.get(path).unwrap()
    }

    pub fn get_file_path(&self, id: FileId) -> Option<&Path> {
        self.files.iter().find_map(|(path, &file_id)| -> Option<&Path> {
            if file_id == id {
                Some(path)
            } else {
                None
            }
        })
    }

    pub fn get_file_path_unchecked(&self, id: FileId) -> &Path {
        self.get_file_path(id).unwrap()
    }

    pub fn get_or_insert(&mut self, path: &Path) -> FileId {
        if let Some(&id) = self.files.get(path) {
            id
        } else {
            let id = FileId(self.files.len());
            self.files.insert(path.to_path_buf(), id);
            id
        }
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }
}
