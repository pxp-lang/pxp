use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Hint,
    Information,
    Warning,
    Error,
}

impl Severity {
    pub fn is_error(&self) -> bool {
        self == &Severity::Error
    }

    pub fn is_warning(&self) -> bool {
        self == &Severity::Warning
    }

    pub fn is_information(&self) -> bool {
        self == &Severity::Information
    }

    pub fn is_hint(&self) -> bool {
        self == &Severity::Hint
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Hint => write!(f, "[hint]",),
            Severity::Information => write!(f, "[info]",),
            Severity::Warning => write!(f, "[warning]",),
            Severity::Error => write!(f, "[error]",),
        }
    }
}
