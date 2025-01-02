use std::path::{Path, PathBuf};

use indicatif::{ProgressDrawTarget, ProgressStyle};

pub(crate) struct ProgressBar {
    bar: indicatif::ProgressBar,
}

impl ProgressBar {
    pub(crate) fn new(show: bool, n: u64) -> Self {
        let bar = indicatif::ProgressBar::new(n).with_style(
            ProgressStyle::with_template("{wide_bar:.green} {pos:>7}/{len:7}\n{msg}").unwrap(),
        );

        if !show {
            bar.set_draw_target(ProgressDrawTarget::hidden());
        }

        Self { bar }
    }

    pub(crate) fn inc(&self, n: u64) {
        self.bar.inc(n);
    }

    pub(crate) fn set_message(&self, message: String) {
        self.bar.set_message(message);
    }

    pub(crate) fn finish_and_clear(&self) {
        self.bar.finish_and_clear();
    }
}

pub(crate) fn find_php_files_in_list(paths: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for path in paths {
        if path.is_dir() {
            files.append(&mut find_php_files_in(path)?);
        } else {
            files.push(path.clone());
        }
    }

    Ok(files)
}

pub(crate) fn find_php_files_in(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut find_php_files_in(&path)?);
        } else if path.extension().map_or(false, |ext| ext == "php") {
            files.push(path);
        }
    }

    Ok(files)
}

pub(crate) fn pxp_home_dir() -> anyhow::Result<PathBuf> {
    let Some(home) = homedir::my_home()? else {
        anyhow::bail!("Could not find home directory.");
    };

    let pxp = home.join(".pxp");

    if !pxp.exists() {
        std::fs::create_dir(&pxp)?;
    }

    Ok(pxp)
}
