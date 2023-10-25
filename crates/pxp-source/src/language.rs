#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Php,
    Pxp,
}

impl Language {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Php => "php",
            Self::Pxp => "pxp",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "php" => Some(Self::Php),
            "pxp" => Some(Self::Pxp),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_produce_correct_extensions() {
        assert_eq!(Language::Php.extension(), "php");
        assert_eq!(Language::Pxp.extension(), "pxp");
    }

    #[test]
    fn it_can_be_constructed_from_extensions() {
        assert_eq!(Language::from_extension("php"), Some(Language::Php));
        assert_eq!(Language::from_extension("pxp"), Some(Language::Pxp));
        assert_eq!(Language::from_extension("foo"), None);
    }
}