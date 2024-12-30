use std::path::{Path, PathBuf};

use ariadne::ReportKind;
use indicatif::{ProgressDrawTarget, ProgressStyle};
use pxp_diagnostics::Severity;

pub(crate) struct ProgressBar {
    show: bool,
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

        Self { show, bar }
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

pub(crate) fn severity_to_report_kind(severity: Severity) -> ReportKind<'static> {
    match severity {
        Severity::Hint => ReportKind::Advice,
        Severity::Information => ReportKind::Advice,
        Severity::Warning => ReportKind::Warning,
        Severity::Error => ReportKind::Error,
    }
}
