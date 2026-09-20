//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI)
//! `clap` derive: struktura JE dokumentacija naredbe (`--help` se generira). `anyhow::Result<i32>`
//! u `run()`: svaka greška (io ili parse) ide `?`-om do `main`, koji je JEDINO mjesto s
//! `process::exit` — izlazni kod je ugovor prema preflightu.
//! (cigla M2/19 — `--until`) `Option<String>` iz `clap`-a postaje `Option<&str>` kroz
//! `as_deref()`: `report_for` posuđuje niz umjesto da ga kopira, isti obrazac kao `since`.
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
    /// Cijeli izvještaj (tempo, vrste, pokazatelji, faze, vizije, docs, signali) [--since YYYY-MM-DD] [--until YYYY-MM-DD]
    Report {
        path: Option<PathBuf>,
        #[arg(long)]
        since: Option<String>,
        /// Gornja granica razdoblja (uključivo, cijeli dan) — zrcalo `--since` (S-011/S-012).
        #[arg(long)]
        until: Option<String>,
        /// `conflicts_with`: dva oblika ispisa odjednom su pogrešna uporaba (izlaz 3), a ne
        /// „zadnji pobjeđuje" — prije je `--json --table` tiho ispisao JSON (nalaz M10).
        #[arg(long, conflicts_with = "table")]
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

fn report_for(
    path: Option<PathBuf>,
    since: Option<&str>,
    until: Option<&str>,
) -> anyhow::Result<Report> {
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    let project = Project::open(&path)?;
    let input = project.input_between(since, until)?;
    Ok(build_report(&input, &project.profile)?)
}

fn run() -> anyhow::Result<i32> {
    // `try_parse` umjesto `parse`: `parse` bi sam izašao s kodom **2**, a 2 je naš kod za ALERT
    // (nalaz I4). `clap` i `--help`/`--version` vraća kao `Err`, pa razliku presuđuje
    // `use_stderr()`: greška uporabe ide na stderr → 3, pomoć/verzija na stdout → 0.
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            let _ = e.print();
            return Ok(if e.use_stderr() { 3 } else { 0 });
        }
    };
    match cli.cmd {
        Cmd::Report {
            path,
            since,
            until,
            json,
            table,
        } => {
            let r = report_for(path, since.as_deref(), until.as_deref())?;
            if json || !table {
                println!("{}", serde_json::to_string_pretty(&r)?);
            } else {
                println!("{}", table::render(&r));
            }
            Ok(0)
        }
        Cmd::Docs { path, json } => {
            let r = report_for(path, None, None)?;
            match (&r.docs, json) {
                (Some(d), true) => println!("{}", serde_json::to_string_pretty(d)?),
                (Some(d), false) => println!("{}", table::render_docs(d)),
                (None, _) => println!("docs: nema mape s dokumentacijom (n/a)"),
            }
            Ok(0)
        }
        Cmd::Signals { path, json } => {
            let r = report_for(path, None, None)?;
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
