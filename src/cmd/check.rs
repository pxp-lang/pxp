use std::{collections::HashMap, path::Path};

use ariadne::{Label, Report, Source};
use clap::Parser as Args;
use colored::Colorize;
use indicatif::ProgressBar;
use pxp_diagnostics::Diagnostic;
use pxp_lexer::Lexer;
use pxp_parser::{Parser, ParserDiagnostic};

use crate::{config::{CheckConfig, Config}, utils::{find_php_files_in_list, severity_to_report_kind}};

#[derive(Args, Debug)]
#[command(version, about = "Perform static analysis on a file or directory.")]
pub struct Check {
    #[arg(short = 's', long, help = "Only perform high-level syntax checks.")]
    only_syntax: bool,
}

pub fn check(args: Check) -> anyhow::Result<()> {
    let config = Config::load()?;

    if args.only_syntax {
        return only_syntax(config.check);
    }

    Ok(())
}

fn only_syntax(config: CheckConfig) -> anyhow::Result<()> {
    let files = find_php_files_in_list(&config.paths)?;
    let pb = ProgressBar::new(files.len() as u64);
    let mut diagnostics: HashMap<&Path, Vec<Diagnostic<ParserDiagnostic>>> = HashMap::new();

    for file in files.iter() {
        pb.inc(1);

        let contents = std::fs::read(&file)?;
        let result = Parser::parse(Lexer::new(&contents));

        if result.diagnostics.is_empty() {
            continue;
        }

        diagnostics.insert(file, result.diagnostics);
    }

    pb.finish_and_clear();

    if diagnostics.is_empty() {
        println!("{}", "No syntax errors found!".green().bold());

        return Ok(())
    }

    for (file, collection) in diagnostics.iter() {
        println!("{}:", format!("{}", file.display()).yellow());

        for diagnostic in collection.iter() {
            Report::build(severity_to_report_kind(diagnostic.severity), (file.display().to_string(), diagnostic.span.to_range()))
                .with_label(
                    Label::new((file.display().to_string(), diagnostic.span.to_range()))
                        .with_message(diagnostic.kind.to_string())
                )
                .finish()
                .print((file.display().to_string(), Source::from(&std::fs::read_to_string(file)?)))?;
        }
    }

    Ok(())
}
