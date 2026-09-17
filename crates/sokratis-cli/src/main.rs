//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI)
//! `clap` derive: struktura JE dokumentacija naredbe (`--help` se generira). `anyhow::Result<i32>`
//! u `run()`: svaka greška (io ili parse) ide `?`-om do `main`, koji je JEDINO mjesto s
//! `process::exit` — izlazni kod je ugovor prema preflightu.
mod table;

use clap::{Parser, Subcommand};
use sokratis_core::{Report, Severity, build_report};
use sokratis_io::Project;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "sokratis",
    version,
    about = "Statistika rada, čistoća docs-a i signali smjera iz gita"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Cijeli izvještaj (tempo, vrste, pokazatelji, faze, vizije, docs, signali)
    Report {
        path: Option<PathBuf>,
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        table: bool,
    },
    /// Samo čistoća dokumentacije
    Docs {
        path: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Samo signali; izlazni kod 0 nema · 1 Warn · 2 Alert
    Signals {
        path: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

fn report_for(path: Option<PathBuf>, since: Option<&str>) -> anyhow::Result<Report> {
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    let project = Project::open(&path)?;
    let input = project.input(since)?;
    Ok(build_report(&input, &project.profile)?)
}

fn run() -> anyhow::Result<i32> {
    match Cli::parse().cmd {
        Cmd::Report {
            path,
            since,
            json,
            table,
        } => {
            let r = report_for(path, since.as_deref())?;
            if json || !table {
                println!("{}", serde_json::to_string_pretty(&r)?);
            } else {
                println!("{}", table::render(&r));
            }
            Ok(0)
        }
        Cmd::Docs { path, json } => {
            let r = report_for(path, None)?;
            match (&r.docs, json) {
                (Some(d), true) => println!("{}", serde_json::to_string_pretty(d)?),
                (Some(d), false) => println!("{}", table::render_docs(d)),
                (None, _) => println!("docs: nema mape s dokumentacijom (n/a)"),
            }
            Ok(0)
        }
        Cmd::Signals { path, json } => {
            let r = report_for(path, None)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&r.signals)?);
            } else {
                println!("{}", table::render_signals(&r.signals));
            }
            Ok(match r.signals.iter().map(|s| s.severity).max() {
                Some(Severity::Alert) => 2,
                Some(Severity::Warn) => 1,
                _ => 0,
            })
        }
    }
}

fn main() {
    let code = match run() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("sokratis: {e:#}");
            3
        }
    };
    std::process::exit(code);
}
