use std::fmt::Display;

#[derive(Debug)]
pub enum Severity {
    Warning,
    Error,
}

impl Severity {
    pub fn with_ascii(&self) -> AsciiSeverity {
        AsciiSeverity::new(self)
    }
}

pub struct AsciiSeverity<'a> {
    severity: &'a Severity,
}

impl<'a> AsciiSeverity<'a> {
    pub fn new(severity: &'a Severity) -> Self {
        Self { severity }
    }

    fn get_ascii_color(&self) -> &'static str {
        match self.severity {
            Severity::Warning => "\x1b[33;1m",
            Severity::Error => "\x1b[31;1m",
        }
    }

    pub fn get_ascii_reset(&self) -> &'static str {
        "\x1b[0m"
    }
}

impl<'a> Display for AsciiSeverity<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.get_ascii_color(), self.severity, self.get_ascii_reset())
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Warning => write!(
                f,
                "[warning]",
            ),
            Severity::Error => write!(
                f,
                "[error]",
            ),
        }
    }
}
