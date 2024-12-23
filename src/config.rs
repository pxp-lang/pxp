use std::{env::current_dir, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) check: CheckConfig,
}

impl Config {
    pub(crate) fn load() -> anyhow::Result<Self> {
        let cwd = current_dir()?;
        let config_path = cwd.join("pxp.config.toml");

        if ! config_path.exists() {
            return Err(anyhow::anyhow!("Could not locate `pxp.config.toml` file in current directory."));
        }

        let config = std::fs::read_to_string(config_path)?;

        toml::from_str(&config).map_err(Into::into)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct CheckConfig {
    pub(crate) paths: Vec<PathBuf>,
}
