use std::path::PathBuf;

use super::io::{error, success};

const CONFIG_STUB: &'static str = include_str!("../../stubs/pxp.toml");

pub fn init(force: bool) {
    let path = PathBuf::from("./pxp.toml");
    
    if path.exists() && !force {
        error("pxp.toml already exists in this directory.");
        return;
    }

    std::fs::write(path, CONFIG_STUB).unwrap();

    success("pxp.toml created successfully.");
}