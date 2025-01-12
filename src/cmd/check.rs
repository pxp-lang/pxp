use std::{collections::HashMap, path::Path};

use clap::Parser as Args;
use codespan_reporting::diagnostic::LabelStyle;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::diagnostic::Diagnostic as CodespanDiagnostic;
use codespan_reporting::diagnostic::Severity as CodespanSeverity;
use codespan_reporting::diagnostic::Label as CodespanLabel;
use codespan_reporting::term::termcolor::ColorChoice;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::Config as CodespanConfig;
use colored::Colorize;
use pxp_analyser::{AnalyserDiagnostic, Reporter, Runner};
use pxp_diagnostics::DiagnosticKind;
use pxp_diagnostics::DiagnosticLabel;
use pxp_diagnostics::DiagnosticLabelStyle;
use pxp_diagnostics::{Diagnostic, Severity};
use pxp_index::Index;
use pxp_inference::TypeEngine;
use pxp_lexer::Lexer;
use pxp_parser::{Parser, ParserDiagnostic};
use pxp_span::Spanned;

use crate::utils::find_php_files_in_cwd;
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

    let mut index = Index::new();
    let index_files = find_php_files_in_cwd()?;

    println!("Indexing files...");

    for file in index_files.iter() {
        index.index_file(&file);
    }

    let type_engine = TypeEngine::new(&index);
    let mut reporter = Reporter::new();
    let mut runner = Runner::new(&type_engine);

    println!("Analysing codebase...");

    let files = find_php_files_in_list(&config.check.paths)?;
    let mut file_repository = SimpleFiles::new();
    
    for file in files {
        let contents = std::fs::read(&file)?;
        let file_id = file_repository.add(file.display().to_string(), String::from_utf8_lossy(&contents).to_string());
        let result = Parser::parse(Lexer::new(&contents));

        if !result.diagnostics.is_empty() {
            process_parser_diagnostics(file_id, &result.diagnostics, &mut reporter);

            // If there are any errors, we don't want to run the analyser since the AST
            // is only partially complete and the results could be inaccurate.
            if result.diagnostics.iter().any(|d| d.severity == Severity::Error) {
                continue;
            }
        }

        runner.run(file_id, &mut reporter, &result.ast);
    }

    report(&file_repository, &reporter);

    Ok(())
}

fn report(files: &SimpleFiles<String, String>, reporter: &Reporter) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = CodespanConfig::default();

    for (file, diagnostics) in reporter.all() {
        for diagnostic in diagnostics {
            let mut codespan_diagnostic = CodespanDiagnostic::<usize>::new(severity_to_codespan(diagnostic.severity))
                .with_message(diagnostic.kind.get_message())
                .with_code(diagnostic.kind.get_code())
                .with_labels(labels_to_codespan(*file, &diagnostic.kind.get_labels()));

            if let Some(help) = diagnostic.kind.get_help() {
                codespan_diagnostic = codespan_diagnostic.with_notes(vec![
                    help.to_string()
                ]);
            }

            codespan_reporting::term::emit(&mut writer.lock(), &config, files, &codespan_diagnostic).unwrap();
        }
    }
}

fn labels_to_codespan(file: usize, labels: &[DiagnosticLabel]) -> Vec<CodespanLabel<usize>> {
    labels.iter().map(|label| {
        CodespanLabel::new(label_style_to_codespan(label.style), file, label.span.to_range())
            .with_message(label.message.clone())
    }).collect()
}

fn label_style_to_codespan(style: DiagnosticLabelStyle) -> LabelStyle {
    match style {
        DiagnosticLabelStyle::Primary => LabelStyle::Primary,
        DiagnosticLabelStyle::Secondary => LabelStyle::Secondary,
    }
}

// FIXME: Abstract all of this into the diagnostics crate, or alternatively
// an abstract crate that wraps `codespan_reporting`.
fn severity_to_codespan(severity: Severity) -> CodespanSeverity {
    match severity {
        Severity::Error => CodespanSeverity::Error,
        Severity::Warning => CodespanSeverity::Warning,
        Severity::Information => CodespanSeverity::Note,
        Severity::Hint => CodespanSeverity::Help,
    }
}

fn process_parser_diagnostics(file: usize, diagnostics: &[Diagnostic<ParserDiagnostic>], reporter: &mut Reporter) {
    for diagnostic in diagnostics.iter() {
        reporter.report(
            file,
            AnalyserDiagnostic::new()
                .code(&diagnostic.kind.get_code())
                .identifier(&diagnostic.kind.get_identifier())
                .message(&diagnostic.kind.get_message()),
            diagnostic.severity,
            diagnostic.span,
        );
    }
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
                diagnostic.kind.get_code().underline(),
                diagnostic.kind.get_message(),
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
