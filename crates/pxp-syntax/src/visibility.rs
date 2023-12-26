use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Serialize, Deserialize)]

pub enum Visibility {
    #[default]
    Public,
    Protected,
    Private,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Protected => write!(f, "protected"),
            Visibility::Private => write!(f, "private"),
        }
    }
}