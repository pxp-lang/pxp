use crate::Language;

#[derive(Debug)]
pub struct SourceFile {
    pub name: Option<String>,
    pub language: Language,
    pub source: Vec<u8>,
}

impl SourceFile {
    pub fn new(name: Option<String>, language: Language, source: Vec<u8>) -> Self {
        Self { name, language, source }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn source(&self) -> &[u8] {
        &self.source[..]
    }
}

impl AsRef<[u8]> for SourceFile {
    fn as_ref(&self) -> &[u8] {
        self.source()
    }
}