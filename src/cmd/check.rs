use std::{collections::HashMap, path::Path};

use ariadne::{Label, Report, Source};
use clap::Parser as Args;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_lexer::Lexer;
use pxp_parser::{Parser, ParserDiagnostic};

use crate::{
    config::{CheckConfig, Config},
    utils::{find_php_files_in_list, severity_to_report_kind},
};

#[derive(Args, Debug)]
#[command(version, about = "Perform static analysis on a file or directory.")]
pub struct Check {
    #[arg(short = 's', long, help = "Only perform high-level syntax checks.")]
    only_syntax: bool,

    #[arg(short = 'e', long, help = "Only show errors.")]
    only_errors: bool,

    #[arg(short = 'w', long, help = "Only show warnings.")]
    only_warnings: bool,
}

pub fn check(args: Check) -> anyhow::Result<()> {
    let config = Config::load()?;

    if args.only_syntax {
        return only_syntax(args, config.check);
    }

    Ok(())
}

fn only_syntax(args: Check, config: CheckConfig) -> anyhow::Result<()> {
    let files = find_php_files_in_list(&config.paths)?;
    let style = ProgressStyle::with_template("{wide_bar:.green} {pos:>7}/{len:7}\n{msg}")?;
    let pb = ProgressBar::new(files.len() as u64).with_style(style);
    let mut diagnostics: HashMap<&Path, Vec<Diagnostic<ParserDiagnostic>>> = HashMap::new();

    for file in files.iter() {
        pb.set_message(file.display().to_string());

        let contents = std::fs::read(&file)?;
        let result = Parser::parse(Lexer::new(&contents));

        if result.diagnostics.is_empty() {
            pb.inc(1);

            continue;
        }

        diagnostics.insert(file, result.diagnostics);

        pb.inc(1);
    }

    pb.finish_and_clear();

    if diagnostics.is_empty() {
        println!("{}", "No syntax errors found!".green().bold());

        return Ok(());
    }

    for (file, collection) in diagnostics.iter() {
        let collection = collection
            .iter()
            .filter(|d| should_show_diagnostic(&args, d.severity))
            .collect::<Vec<_>>();

        if collection.is_empty() {
            continue;
        }

        for diagnostic in collection.iter() {
            Report::build(
                severity_to_report_kind(diagnostic.severity),
                (file.display().to_string(), diagnostic.span.to_range()),
            )
            .with_label(
                Label::new((file.display().to_string(), diagnostic.span.to_range()))
                    .with_message(diagnostic.kind.to_string()),
            )
            .finish()
            .print((
                file.display().to_string(),
                Source::from(&std::fs::read_to_string(file)?),
            ))?;
        }
    }

    Ok(())
}

fn should_show_diagnostic(args: &Check, severity: Severity) -> bool {
    if args.only_errors && severity != Severity::Error {
        return false;
    }

    if args.only_warnings && severity != Severity::Warning {
        return false;
    }

    true
}
