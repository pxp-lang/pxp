use std::fmt::Display;

#[derive(Debug)]
pub enum Severity {
    Warning,
    Error,
}

impl Severity {
    fn get_ascii_color(&self) -> &'static str {
        match self {
            Severity::Warning => "\x1b[33m",
            Severity::Error => "\x1b[31m",
        }
    }

    pub fn get_ascii_reset(&self) -> &'static str {
        "\x1b[0m"
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Warning => write!(
                f,
                "{}[warning]{}",
                self.get_ascii_color(),
                self.get_ascii_reset()
            ),
            Severity::Error => write!(
                f,
                "{}[error]{}",
                self.get_ascii_color(),
                self.get_ascii_reset()
            ),
        }
    }
}
