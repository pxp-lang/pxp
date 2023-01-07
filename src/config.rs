use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = Config::read();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub build: Build,
    pub lint: Lint,
}

impl Config {
    pub fn read() -> Self {
        let path = PathBuf::from("./pxp.toml");
        // FIXME: don't panic, output nice error instead.
        if !path.exists() {
            panic!("pxp.toml not found");
        }
        let contents = std::fs::read_to_string(path).unwrap();
        toml::from_str(&contents).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Build {
    pub paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Lint {
    pub variables: Option<Case>,
    pub functions: Option<Case>,
    pub methods: Option<Case>,
    pub identifiers: Option<Case>,
    pub property_types: bool,
    pub parameter_types: bool,
    pub return_types: bool,
    pub include_php: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Case {
    Snake,
    Camel,
    Pascal,
}
