use std::path::{Path, PathBuf};

use ariadne::ReportKind;
use pxp_diagnostics::Severity;

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
