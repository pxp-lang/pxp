use std::{collections::HashMap, path::Path};

use clap::Parser as Args;
use colored::Colorize;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_lexer::Lexer;
use pxp_parser::{Parser, ParserDiagnostic};
use pxp_diagnostics::DiagnosticKind;
use pxp_span::Spanned;

use crate::{
    config::{CheckConfig, Config},
    utils::{find_php_files_in_list, ProgressBar},
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

    #[arg(short = 'p', long, help = "Do not show progress bar.")]
    no_progress: bool,
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
    let bar = ProgressBar::new(!args.no_progress, files.len() as u64);
    let mut diagnostics: HashMap<&Path, Vec<Diagnostic<ParserDiagnostic>>> = HashMap::new();

    for file in files.iter() {
        // println!("{}", file.display().to_string());

        bar.set_message(file.display().to_string());

        let contents = std::fs::read(file)?;
        let result = Parser::parse(Lexer::new(&contents));

        if result.diagnostics.is_empty() {
            bar.inc(1);

            continue;
        }

        diagnostics.insert(file, result.diagnostics);

        bar.inc(1);
    }

    bar.finish_and_clear();

    if diagnostics.is_empty() {
        println!("{}", "No syntax errors found!".green().bold());

        return Ok(());
    }

    for (file, collection) in diagnostics.iter() {
        let collection = collection
            .iter()
            .filter(|d| should_show_diagnostic(&args, d.severity))
            .collect::<Vec<_>>();

        let filename = file.display().to_string();
        let contents = std::fs::read(file)?;

        if collection.is_empty() {
            continue;
        }

        println!("{}", filename.bold());

        for diagnostic in collection.iter() {
            let line = diagnostic.span.start_line(&contents);
            let column = diagnostic.span.start_column(&contents);

            println!(
                "[{}]: {} on line {}, column {}",
                diagnostic.kind.code().underline(),
                diagnostic.kind.message(),
                line,
                column
            );
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
