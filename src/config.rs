use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub build: Build,
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
