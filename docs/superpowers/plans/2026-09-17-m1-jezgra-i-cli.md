# Milestone 1 — Jezgra + CLI: plan implementacije

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rust CLI `sokratis` koji nad Sokrat Studyjem daje iste brojke kao `RAD.xlsx` (s ispravnim satima), ocjenu čistoće dokumentacije i dva signala s dokazom.

**Architecture:** Cargo workspace s tri cratea. `sokratis-core` je čisti Rust bez I/O-a (prima tekst i strukture, vraća `Report`); `sokratis-io` zove `git` proces, čita profil i ručne datoteke; `sokratis-cli` je tanka `clap` binarna. Tokovi rada su odvojeni po datotekama pa se grade paralelno na više grana i spajaju bez sudara.

**Tech Stack:** Rust stable (edition 2024, MSVC) · serde/serde_json · regex · thiserror/anyhow · clap · chrono (samo io) · tempfile/insta (dev).

**Spec:** `docs/plan/ARHITEKTURA_M1.md` · definicija: `docs/product/PRD.md` · odluke: `docs/records/DECISIONS.md` · konvencije: `docs/workflow/RUST.md` · testovi: `docs/workflow/TESTING.md` · agenti: `docs/workflow/AGENTI.md`

## Global Constraints

- `sokratis-core` **ne smije** koristiti `std::fs`, `std::process`, `std::env` ni `std::time` (S-002). Sve dolazi kroz `ReportInput`.
- Identifikatori u kodu i JSON-u su **engleski, `snake_case`** (S-008); enumi se serijaliziraju `rename_all = "snake_case"`.
- Zadane vrijednosti profila = konvencije Sokrat Studyja **doslovno** kako stoje u T1 (S-005). Nepoznato polje profila = greška.
- Sati: sortiranje po `author_time` uzlazno, prag 2,0 h, početak 0,5 h (S-007).
- **Ovisnosti samo one iz T1.** Tok koji treba novu ovisnost STAJE i javlja orkestratoru; ne mijenja `Cargo.toml`.
- **Vlasništvo datoteka po toku** (tablica dolje). Tok ne dira tuđe datoteke; ako mora, STAJE i javlja.
- Svaka cigla: `cargo fmt --check` · `cargo clippy --all-targets -- -D warnings` · `cargo test` zeleni prije commita; zaglavlje `//! ZAŠTO RUST OVAKO (cigla M1/N — naziv)` 2–5 redaka u svakoj novoj datoteci; novi pojam = redak u `docs/workflow/RUST.md` §4 (to radi čuvar dokumentacije nakon spajanja, na temelju zaglavlja).
- Commit poruka: `M1/N: <što> -- <zašto u pola rečenice>`; bez pusha; nikad na `main` (spaja orkestrator).
- Bez `unwrap()`/`expect()` izvan `#[cfg(test)]` i `tests/`.

---

## Struktura datoteka i vlasništvo po tokovima

| tok | grana | worktree | zadaci | vlasništvo (smije mijenjati SAMO ovo) |
|---|---|---|---|---|
| **KOSTUR** (orkestrator) | `main` | `sokratis` | T1 | sve |
| **FIXTURE** | `feat/fixtures` | `sokratis.fixtures` | T2 | `crates/sokratis-core/tests/fixtures/sokratstudy-*` |
| **PARSE** | `feat/core-parse` | `sokratis.parse` | T3–T6 | `crates/sokratis-core/src/parse/**`, `src/classify.rs` |
| **METRIKE** | `feat/core-metrics` | `sokratis.metrics` | T7–T11 | `crates/sokratis-core/src/metrics/**`, `src/civil.rs` |
| **DOCS+PRAVILA** | `feat/core-rules` | `sokratis.rules` | T12–T14 | `crates/sokratis-core/src/docs.rs`, `src/rules/**` |
| **IO** | `feat/io` | `sokratis.io` | T15–T17 | `crates/sokratis-io/**` |
| **CLI** | `feat/cli` | `sokratis.cli` | T18–T19 | `crates/sokratis-cli/**` |
| **INTEGRACIJA** (nakon spajanja svih) | `feat/integracija` | `sokratis.integracija` | T20–T22 | `crates/sokratis-core/src/report.rs`, `tests/parity.rs`, `.sokratis/`, `docs/` |

Redoslijed spajanja u `main`: T1 → (T2 · T3–T6 · T7–T11 · T12–T14 · T15–T17 · T18–T19 paralelno, spajaju se kako završe) → T20–T22.

```
sokratis/
  Cargo.toml                                   # workspace + workspace.dependencies (T1)
  crates/sokratis-core/
    Cargo.toml
    src/lib.rs                                 # moduli + re-export (T1)
    src/model.rs                               # SVI tipovi ugovora (T1)
    src/profile.rs                             # Profile + Default (Sokrat Study) + Patterns (T1)
    src/error.rs                               # ParseError (T1)
    src/civil.rs                               # days_between("YYYY-MM-DD") (T7)
    src/parse/mod.rs  gitlog.rs  diary.rs  plan.rs   # (T1 stub → T3, T4, T5)
    src/classify.rs                            # (T1 stub → T6)
    src/metrics/mod.rs  hours.rs  days.rs  kinds.rs  phases.rs  indicators.rs  # (T1 stub → T8–T11)
    src/docs.rs                                # DocsHealth (T1 stub → T12)
    src/rules/mod.rs  unmerged_branches.rs  docs_lag.rs   # (T1 stub → T13, T14)
    src/report.rs                              # build_report (T1 stub → T20)
    tests/fixtures/                            # (T2, T3–T5)
    tests/parity.rs                            # (T21)
  crates/sokratis-io/
    Cargo.toml
    src/lib.rs  error.rs  git.rs  project.rs   # (T1 stub → T15, T16, T17)
    tests/git_cli.rs  project.rs               # (T15–T17)
  crates/sokratis-cli/
    Cargo.toml
    src/main.rs  table.rs                      # (T1 stub → T18, T19)
    tests/cli.rs                               # (T19)
  .sokratis/profile.json                       # dogfooding (T22)
```

---

### Task 1: M0 toolchain + kostur workspacea s kompletnim ugovorom tipova

**Tko:** orkestrator, na `main`. Sve ostalo ovisi o ovome.

**Files:**
- Create: `Cargo.toml`, `crates/sokratis-core/{Cargo.toml,src/lib.rs,src/model.rs,src/profile.rs,src/error.rs}`, stubovi: `src/parse/{mod.rs,gitlog.rs,diary.rs,plan.rs}`, `src/classify.rs`, `src/metrics/{mod.rs,hours.rs,days.rs,kinds.rs,phases.rs,indicators.rs}`, `src/docs.rs`, `src/rules/{mod.rs,unmerged_branches.rs,docs_lag.rs}`, `src/report.rs`, `src/civil.rs`
- Create: `crates/sokratis-io/{Cargo.toml,src/lib.rs,src/error.rs,src/git.rs,src/project.rs}`
- Create: `crates/sokratis-cli/{Cargo.toml,src/main.rs,src/table.rs}`

**Interfaces:** Produces sve tipove iz `model.rs`, `profile.rs`, `error.rs` i potpise svih javnih funkcija (tijela `todo!()`). Kasniji zadaci **ne mijenjaju potpise**; ako moraju, javljaju orkestratoru.

- [ ] **Step 1: M0 — toolchain (traži Leonov OK jer mijenja sustav; ~4 GB)**

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --id Rustlang.Rustup
rustup default stable
cargo --version ; rustc --version ; cargo clippy --version ; cargo fmt --version
```
Expected: sve četiri naredbe ispišu verziju (novi terminal nakon instalacije zbog PATH-a).

- [ ] **Step 2: workspace `Cargo.toml`**

```toml
[workspace]
resolver = "3"
members = ["crates/sokratis-core", "crates/sokratis-io", "crates/sokratis-cli"]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["Leon Kreso"]
publish = false

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regex = "1"
thiserror = "2"
anyhow = "1"
clap = { version = "4", features = ["derive"] }
chrono = "0.4"
tempfile = "3"
insta = { version = "1", features = ["json", "redactions"] }
```

`crates/sokratis-core/Cargo.toml`:
```toml
[package]
name = "sokratis-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
regex.workspace = true
thiserror.workspace = true
```

`crates/sokratis-io/Cargo.toml`:
```toml
[package]
name = "sokratis-io"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish.workspace = true

[dependencies]
sokratis-core = { path = "../sokratis-core" }
serde_json.workspace = true
thiserror.workspace = true
chrono.workspace = true

[dev-dependencies]
tempfile.workspace = true
```

`crates/sokratis-cli/Cargo.toml`:
```toml
[package]
name = "sokratis-cli"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish.workspace = true

[[bin]]
name = "sokratis"
path = "src/main.rs"

[dependencies]
sokratis-core = { path = "../sokratis-core" }
sokratis-io = { path = "../sokratis-io" }
serde_json.workspace = true
anyhow.workspace = true
clap.workspace = true

[dev-dependencies]
tempfile.workspace = true
insta.workspace = true
```

- [ ] **Step 3: `src/lib.rs` i `src/error.rs` jezgre**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur jezgre)
//! Jezgra nema I/O: ni `std::fs`, ni `std::process`. Sve što treba dolazi kao `&str` ili
//! struktura kroz `ReportInput`. To je granica S-002 i razlog zašto se testira bez gita.
//! `pub mod` = modul je datoteka; `pub use` = kraći put do tipova za pozivatelja.
pub mod civil;
pub mod classify;
pub mod docs;
pub mod error;
pub mod metrics;
pub mod model;
pub mod parse;
pub mod profile;
pub mod report;
pub mod rules;

pub use error::ParseError;
pub use model::*;
pub use profile::{Patterns, Profile};
pub use report::build_report;
```

`src/error.rs`:
```rust
//! ZAŠTO RUST OVAKO (cigla M1/1 — greške jezgre)
//! `thiserror` pretvara enum u pravi `Error` s porukom po varijanti. Pozivatelj dobiva TIP
//! greške (koji redak, koji broj), ne string — i može odlučiti što s njom (`?` je propagira).
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("redak {line}: neispravan zapis commita: {text}")]
    BadLine { line: usize, text: String },
    #[error("redak {line}: nije broj: {text}")]
    BadNumber { line: usize, text: String },
    #[error("neispravan regex u profilu: {0}")]
    Regex(#[from] regex::Error),
}
```

- [ ] **Step 4: `src/model.rs` — ugovor tipova (ovo je JEDINI izvor imena za sve tokove)**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/1 — model)
//! Sve su strukture `pub` s `pub` poljima i `derive(Serialize, Deserialize)`: to je ugovor
//! prema CLI-ju i sučelju (JSON). Enumi s `rename_all = "snake_case"` daju `"debugging"`, ne
//! `"Debugging"` (S-008). `Clone` je namjeran — jezgra radi s podacima koje posjeduje.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub added: u64,
    pub deleted: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub author_time: i64,
    pub commit_time: i64,
    /// lokalni datum autora `YYYY-MM-DD` (git `%ad` uz `--date=format:%Y-%m-%d`) — dan u tablici
    pub date: String,
    /// lokalni datum commita `YYYY-MM-DD` (`%cd`) — po njemu git filtrira `--since`
    pub commit_date: String,
    pub subject: String,
    pub files: Vec<FileChange>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkKind { Planning, Documentation, Execution, Polish, Debugging }

impl WorkKind {
    pub const ALL: [WorkKind; 5] = [
        WorkKind::Planning, WorkKind::Documentation, WorkKind::Execution, WorkKind::Polish, WorkKind::Debugging,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubKind { Brick, GateOrMeasure, Deploy, Other }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delivery {
    pub date: String,
    pub model: String,
    pub title: String,
    pub kind: WorkKind,
    pub deploy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseState { Planned, Running, Closed }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phase {
    pub id: String,
    pub name: String,
    pub state: PhaseState,
    pub total_bricks: u32,
    pub done_bricks: u32,
    pub from: Option<String>,
    pub to: Option<String>,
    pub days: Option<i64>,
    pub commits: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vision {
    pub title: String,
    pub source: String,
    pub state: String,
    pub percent: Option<u8>,
    pub note: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Touched {
    pub commits: usize,
    pub lines: u64,
    pub files: usize,
    pub skipped_lines: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DayStats {
    pub date: String,
    pub commits: u32,
    pub commits_cumulative: u32,
    pub lines: u64,
    pub hours: f64,
    pub deliveries: u32,
    pub deploys: u32,
    pub test_lines: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KindStats {
    pub kind: WorkKind,
    pub commits: u32,
    pub share: f64,
    pub lines: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorKind { Measure, Proxy }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Indicator {
    pub id: String,
    pub value: f64,
    pub kind: IndicatorKind,
    pub formula: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub check: String,
    pub path: String,
    pub line: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocsHealth {
    pub score: u8,
    pub findings: Vec<Finding>,
    pub lag_days: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity { Info, Warn, Alert }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signal {
    pub rule: String,
    pub severity: Severity,
    pub title_key: String,
    pub evidence: Vec<String>,
    pub since: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub last_commit_time: i64,
    pub ahead_of_default: u32,
    pub merged: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocFile {
    /// relativno od korijena repoa, s `/`
    pub path: String,
    pub content: String,
    pub last_change_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub generated_at: i64,
    pub since: String,
    pub branch: String,
    pub touched: Touched,
    pub days: Vec<DayStats>,
    pub kinds: Vec<KindStats>,
    pub indicators: Vec<Indicator>,
    pub phases: Vec<Phase>,
    pub visions: Vec<Vision>,
    pub docs: Option<DocsHealth>,
    pub signals: Vec<Signal>,
}

/// Sve što jezgra dobiva izvana. Puni ga `sokratis-io`, testovi ga pune ručno.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportInput {
    pub git_log: String,
    pub diary: Option<String>,
    pub plan: Option<String>,
    pub docs: Vec<DocFile>,
    pub branches: Vec<BranchInfo>,
    pub overrides: HashMap<String, WorkKind>,
    pub visions: Vec<Vision>,
    pub now: i64,
    /// današnji lokalni datum `YYYY-MM-DD`
    pub today: String,
    pub since: String,
    pub branch: String,
}

/// Sve što pravilo smije vidjeti — u vlasništvu, bez lifetimeova (gradi se jednom u `build_report`).
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub profile: crate::profile::Profile,
    pub now: i64,
    pub commits: Vec<Commit>,
    pub branches: Vec<BranchInfo>,
    pub docs: Vec<DocFile>,
    pub last_code_commit: Option<Commit>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_kind_serializes_snake_case() {
        assert_eq!(serde_json::to_string(&WorkKind::Debugging).unwrap(), "\"debugging\"");
        assert_eq!(serde_json::to_string(&SubKind::GateOrMeasure).unwrap(), "\"gate_or_measure\"");
        let s: Severity = serde_json::from_str("\"alert\"").unwrap();
        assert_eq!(s, Severity::Alert);
        assert!(Severity::Alert > Severity::Warn && Severity::Warn > Severity::Info);
    }
}
```

- [ ] **Step 5: `src/profile.rs` — zadano = Sokrat Study, doslovno; `Patterns` = kompilirani regexi**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/1 — profil)
//! `#[serde(default)]` na strukturi: polje koje u JSON-u nedostaje uzima vrijednost iz
//! `impl Default` — a taj Default JE Sokrat Study (S-005). `deny_unknown_fields`: tipfeler u
//! profilu je greška, ne tiho ignoriranje. `Patterns` drži kompilirane regexe odvojeno od
//! profila jer `Regex` nije `Serialize`; kompilira se jednom, koristi tisuću puta.
use crate::model::WorkKind;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassifierRule {
    pub kind: WorkKind,
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedPhase {
    pub name: String,
    pub from: String,
    pub to: String,
    pub tag_pattern: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DocsWeights {
    pub dead_link: u8,
    pub not_indexed: u8,
    pub multiple_plans: u8,
    pub no_active_plan: u8,
    pub diary_in_definition: u8,
    pub lag: u8,
    pub key_file_budget: u8,
}

impl Default for DocsWeights {
    fn default() -> Self {
        Self { dead_link: 5, not_indexed: 3, multiple_plans: 15, no_active_plan: 10, diary_in_definition: 5, lag: 10, key_file_budget: 5 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Profile {
    pub default_branch: String,
    pub since: String,
    pub diary_path: String,
    pub changelog_path: String,
    pub plan_path: String,
    pub docs_dir: String,
    pub docs_index: String,
    pub plan_dir: String,
    pub plan_dir_ignore: Vec<String>,
    pub paused_marker: String,
    pub product_dir: String,
    pub key_file: String,
    pub key_file_budget_bytes: u64,
    pub diary_heading: String,
    pub diary_deploy_pattern: String,
    pub plan_brick: String,
    pub plan_phase_name: String,
    pub phase_tag: String,
    pub classifier: Vec<ClassifierRule>,
    pub gate_pattern: String,
    pub deploy_pattern: String,
    pub ci_fix_pattern: String,
    pub owner_name: String,
    pub test_path_prefixes: Vec<String>,
    pub test_path_contains: Vec<String>,
    pub test_path_suffixes: Vec<String>,
    pub code_exclude_prefixes: Vec<String>,
    pub code_exclude_suffixes: Vec<String>,
    pub session_gap_hours: f64,
    pub session_start_hours: f64,
    pub closed_phases: Vec<ClosedPhase>,
    pub include_unmerged: bool,
    pub unmerged_warn_days: i64,
    pub unmerged_alert_days: i64,
    pub unmerged_alert_count: usize,
    pub docs_lag_warn_days: i64,
    pub docs_lag_alert_days: i64,
    pub docs_weights: DocsWeights,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            default_branch: "main".into(),
            since: "2026-08-29".into(),
            diary_path: "docs/records/PROGRESS.md".into(),
            changelog_path: "docs/records/CHANGELOG.md".into(),
            plan_path: "docs/plan/RASPORED.md".into(),
            docs_dir: "docs".into(),
            docs_index: "docs/README.md".into(),
            plan_dir: "docs/plan".into(),
            plan_dir_ignore: vec!["ROADMAP.md".into()],
            paused_marker: r"^\s*\*\*Status:\*\*\s*⏸️\s*PAUZIRAN\b".into(),
            product_dir: "docs/product".into(),
            key_file: "CLAUDE.md".into(),
            key_file_budget_bytes: 40_000,
            diary_heading: r"^## (\d{4}-\d{2}-\d{2})(?:\s*\(([^)]*)\))?\s*[—-]+\s*(.+)$".into(),
            diary_deploy_pattern: "deploy|main na `".into(),
            plan_brick: r"^\| \*\*(F\d)/(\d+)\*\*\s*(✅?)".into(),
            plan_phase_name: r"^### (F\d) · (.+)$".into(),
            phase_tag: r"^(F\d/\d|C\d[ab]?(?:/\d\w*)?|MREZA[- ]?[A-E]\d?(?: \(\d/\d\))?|R\d(?:/[A-Z0-9+]+)?|T\d|ALAT-\d|BUG-\d+|U\d)".into(),
            classifier: vec![
                ClassifierRule { kind: WorkKind::Planning, pattern: "raspored|dobiva svoj spec|tracnice|tračnice|mjera za c|adr-0|odluk|presud|revizija je dala|sto sada|što sada|zadatka za sljedecu|zadatak za sljedecu".into() },
                ClassifierRule { kind: WorkKind::Documentation, pattern: r"^docs|compact|revizija pred|^r\d/docs|ishod --|deploy-zapis|zapis uz".into() },
                ClassifierRule { kind: WorkKind::Debugging, pattern: r"^fix|bug-\d|^ci:|popravak ci|kvar|obara|^test:|lagati|laze|laže|rupe|bomba|vise ne postoji|više ne postoji".into() },
                ClassifierRule { kind: WorkKind::Polish, pattern: r"^c[4-7]|paleta|mrtv|selektorski|\bvan\b|audit|node 24|migrira|utility|ljestv|raspusten|raspušten|skuplj|refactor|^alat|ugasen|ugašen|dug placen|dug plaćen".into() },
            ],
            gate_pattern: r"check:|brana|gate|^alat|^test|sonda|probe|mjera".into(),
            deploy_pattern: "na produkciji|deploy".into(),
            ci_fix_pattern: r"^ci:|popravak ci|job je otkazan".into(),
            owner_name: "leon".into(),
            test_path_prefixes: vec!["tests/".into()],
            test_path_contains: vec!["/check-".into()],
            test_path_suffixes: vec![".test.js".into(), ".spec.js".into()],
            code_exclude_prefixes: vec!["docs/".into()],
            code_exclude_suffixes: vec![".md".into()],
            session_gap_hours: 2.0,
            session_start_hours: 0.5,
            closed_phases: vec![
                ClosedPhase { name: "Osobni UGC-graditelj F1–F5".into(), from: "2026-08-02".into(), to: "2026-08-06".into(), tag_pattern: r"^(F[1-5][: ]|Merge F5|docs\(F5\))".into(), note: "DB temelj → moji materijali → editor u čvoru → privatne slike → produkcija".into() },
                ClosedPhase { name: "Frontend redizajn C0–C7 + KOSTUR·TELEFON·POLICA·SEO".into(), from: "2026-08-07".into(), to: "2026-09-01".into(), tag_pattern: r"^(C\d[ab]?|T\d|K\d|N\d|S\d|ALAT-\d)".into(), note: "Tailwind CLI, tokeni, 3 teme, birač".into() },
                ClosedPhase { name: "MREŽA — sanacija A–E".into(), from: "2026-08-31".into(), to: "2026-09-01".into(), tag_pattern: r"^(faza MREZA|MREZA)".into(), note: "CSP enforce, RLS po retku, brane, CI shardanje".into() },
                ClosedPhase { name: "RAČUN R1 — OAuth + upitnik + dijalog".into(), from: "2026-09-02".into(), to: "2026-09-02".into(), tag_pattern: r"^R1".into(), note: "Google uživo potvrđen; R2/R3 → F2".into() },
            ],
            include_unmerged: false,
            unmerged_warn_days: 5,
            unmerged_alert_days: 10,
            unmerged_alert_count: 3,
            docs_lag_warn_days: 2,
            docs_lag_alert_days: 5,
            docs_weights: DocsWeights::default(),
        }
    }
}

impl Profile {
    /// Od kada io mora dovući git log: najranije od `since` i početaka zatvorenih faza
    /// (zatvorene faze se BROJE iz commita, a njihovi datumi prethode `since`).
    pub fn log_since(&self) -> String {
        self.closed_phases
            .iter()
            .map(|p| p.from.as_str())
            .chain(std::iter::once(self.since.as_str()))
            .min()
            .unwrap_or(self.since.as_str())
            .to_string()
    }

    pub fn is_test_path(&self, path: &str) -> bool {
        self.test_path_prefixes.iter().any(|p| path.starts_with(p.as_str()))
            || self.test_path_contains.iter().any(|c| path.contains(c.as_str()))
            || self.test_path_suffixes.iter().any(|s| path.ends_with(s.as_str()))
    }

    pub fn is_code_path(&self, path: &str) -> bool {
        !(self.code_exclude_prefixes.iter().any(|p| path.starts_with(p.as_str()))
            || self.code_exclude_suffixes.iter().any(|s| path.ends_with(s.as_str())))
    }
}

pub struct Patterns {
    pub diary_heading: Regex,
    pub diary_deploy: Regex,
    pub plan_brick: Regex,
    pub plan_phase_name: Regex,
    pub phase_tag: Regex,
    pub classifier: Vec<(WorkKind, Regex)>,
    pub gate: Regex,
    pub deploy: Regex,
    pub ci_fix: Regex,
    pub paused: Regex,
    pub closed_phases: Vec<(ClosedPhase, Regex)>,
}

impl Patterns {
    pub fn compile(p: &Profile) -> Result<Patterns, regex::Error> {
        Ok(Patterns {
            diary_heading: Regex::new(&p.diary_heading)?,
            diary_deploy: Regex::new(&p.diary_deploy_pattern)?,
            plan_brick: Regex::new(&p.plan_brick)?,
            plan_phase_name: Regex::new(&p.plan_phase_name)?,
            phase_tag: Regex::new(&p.phase_tag)?,
            classifier: p.classifier.iter().map(|r| Regex::new(&r.pattern).map(|re| (r.kind, re))).collect::<Result<_, _>>()?,
            gate: Regex::new(&p.gate_pattern)?,
            deploy: Regex::new(&p.deploy_pattern)?,
            ci_fix: Regex::new(&p.ci_fix_pattern)?,
            paused: Regex::new(&p.paused_marker)?,
            closed_phases: p.closed_phases.iter().map(|c| Regex::new(&c.tag_pattern).map(|re| (c.clone(), re))).collect::<Result<_, _>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_compiles_and_rejects_unknown_field() {
        assert!(Patterns::compile(&Profile::default()).is_ok());
        assert_eq!(Profile::default().log_since(), "2026-08-02");
        let partial: Profile = serde_json::from_str(r#"{"since":"2026-09-01"}"#).unwrap();
        assert_eq!(partial.since, "2026-09-01");
        assert_eq!(partial.default_branch, "main");
        assert!(serde_json::from_str::<Profile>(r#"{"sinc":"x"}"#).is_err());
    }

    #[test]
    fn test_and_code_paths() {
        let p = Profile::default();
        assert!(p.is_test_path("tests/unit/a.test.js"));
        assert!(p.is_test_path("scripts/check-docs.js"));
        assert!(!p.is_test_path("js/auth.js"));
        assert!(p.is_code_path("js/auth.js"));
        assert!(!p.is_code_path("docs/records/PROGRESS.md"));
        assert!(!p.is_code_path("CLAUDE.md"));
    }
}
```

- [ ] **Step 6: stubovi s POTPUNIM potpisima (tijela `todo!("cigla M1/N")`)**

`src/parse/mod.rs`:
```rust
pub mod diary;
pub mod gitlog;
pub mod plan;
pub use diary::parse_diary;
pub use gitlog::{parse_git_log, Parsed};
pub use plan::parse_plan;
```
`src/parse/gitlog.rs`:
```rust
use crate::{Commit, ParseError};
pub struct Parsed { pub commits: Vec<Commit>, pub skipped_lines: usize }
pub fn parse_git_log(text: &str) -> Result<Parsed, ParseError> { let _ = text; todo!("cigla M1/3") }
```
`src/parse/diary.rs`:
```rust
use crate::{Delivery, Patterns};
pub fn parse_diary(text: &str, p: &Patterns, since: &str) -> Vec<Delivery> { let _ = (text, p, since); todo!("cigla M1/4") }
```
`src/parse/plan.rs`:
```rust
use crate::{Patterns, Phase};
pub fn parse_plan(text: &str, p: &Patterns) -> Vec<Phase> { let _ = (text, p); todo!("cigla M1/5") }
```
`src/classify.rs`:
```rust
use crate::{Patterns, SubKind, WorkKind};
pub fn classify_kind(subject: &str, p: &Patterns) -> WorkKind { let _ = (subject, p); todo!("cigla M1/6") }
pub fn classify_sub(subject: &str, p: &Patterns) -> SubKind { let _ = (subject, p); todo!("cigla M1/6") }
pub fn phase_tag(subject: &str, p: &Patterns) -> Option<String> { let _ = (subject, p); todo!("cigla M1/6") }
```
`src/civil.rs`:
```rust
pub fn days_between(from: &str, to: &str) -> Option<i64> { let _ = (from, to); todo!("cigla M1/7") }
```
`src/metrics/mod.rs`:
```rust
pub mod days;
pub mod hours;
pub mod indicators;
pub mod kinds;
pub mod phases;
pub use days::day_stats;
pub use hours::hours_per_day;
pub use indicators::indicators;
pub use kinds::{effective_kind, kind_stats};
pub use phases::{active_phases, closed_phases};
```
`src/metrics/hours.rs`:
```rust
use crate::Commit;
use std::collections::BTreeMap;
pub fn hours_per_day(commits: &[Commit], gap_h: f64, start_h: f64) -> BTreeMap<String, f64> { let _ = (commits, gap_h, start_h); todo!("cigla M1/8") }
```
`src/metrics/days.rs`:
```rust
use crate::{Commit, DayStats, Delivery, Profile};
use std::collections::BTreeMap;
pub fn day_stats(commits: &[Commit], deliveries: &[Delivery], hours: &BTreeMap<String, f64>, p: &Profile) -> Vec<DayStats> { let _ = (commits, deliveries, hours, p); todo!("cigla M1/9") }
```
`src/metrics/kinds.rs`:
```rust
use crate::{Commit, KindStats, Patterns, WorkKind};
use std::collections::HashMap;
pub fn effective_kind(c: &Commit, overrides: &HashMap<String, WorkKind>, p: &Patterns) -> WorkKind { let _ = (c, overrides, p); todo!("cigla M1/10") }
pub fn kind_stats(commits: &[Commit], overrides: &HashMap<String, WorkKind>, p: &Patterns) -> Vec<KindStats> { let _ = (commits, overrides, p); todo!("cigla M1/10") }
```
`src/metrics/phases.rs`:
```rust
use crate::{Commit, Patterns, Phase};
pub fn closed_phases(all_commits: &[Commit], p: &Patterns) -> Vec<Phase> { let _ = (all_commits, p); todo!("cigla M1/11") }
pub fn active_phases(plan_phases: Vec<Phase>, commits: &[Commit], today: &str) -> Vec<Phase> { let _ = (plan_phases, commits, today); todo!("cigla M1/11") }
```
`src/metrics/indicators.rs`:
```rust
use crate::{Commit, DayStats, Delivery, Indicator, Patterns, Phase, Profile, WorkKind};
use std::collections::HashMap;
pub struct IndicatorInput<'a> {
    pub commits: &'a [Commit],
    pub deliveries: &'a [Delivery],
    pub days: &'a [DayStats],
    pub phases: &'a [Phase],
    pub overrides: &'a HashMap<String, WorkKind>,
}
pub fn indicators(input: &IndicatorInput<'_>, profile: &Profile, p: &Patterns) -> Vec<Indicator> { let _ = (input, profile, p); todo!("cigla M1/11") }
```
`src/docs.rs`:
```rust
use crate::{DocFile, DocsHealth, Patterns, Profile};
pub fn docs_health(files: &[DocFile], last_code_commit_time: Option<i64>, profile: &Profile, p: &Patterns) -> Option<DocsHealth> { let _ = (files, last_code_commit_time, profile, p); todo!("cigla M1/12") }
```
`src/rules/mod.rs`:
```rust
pub mod docs_lag;
pub mod unmerged_branches;
use crate::{Context, Signal};
pub trait Rule {
    fn id(&self) -> &'static str;
    fn evaluate(&self, ctx: &Context) -> Vec<Signal>;
}
pub fn default_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(unmerged_branches::UnmergedBranches), Box::new(docs_lag::DocsLag)]
}
pub fn evaluate_all(rules: &[Box<dyn Rule>], ctx: &Context) -> Vec<Signal> {
    rules.iter().flat_map(|r| r.evaluate(ctx)).collect()
}
```
`src/rules/unmerged_branches.rs`:
```rust
use super::Rule;
use crate::{Context, Signal};
pub struct UnmergedBranches;
impl Rule for UnmergedBranches {
    fn id(&self) -> &'static str { "unmerged-branches" }
    fn evaluate(&self, ctx: &Context) -> Vec<Signal> { let _ = ctx; todo!("cigla M1/13") }
}
```
`src/rules/docs_lag.rs`:
```rust
use super::Rule;
use crate::{Context, Signal};
pub struct DocsLag;
impl Rule for DocsLag {
    fn id(&self) -> &'static str { "docs-lag" }
    fn evaluate(&self, ctx: &Context) -> Vec<Signal> { let _ = ctx; todo!("cigla M1/14") }
}
```
`src/report.rs`:
```rust
use crate::{ParseError, Profile, Report, ReportInput};
pub fn build_report(input: &ReportInput, profile: &Profile) -> Result<Report, ParseError> { let _ = (input, profile); todo!("cigla M1/20") }
```

`crates/sokratis-io/src/lib.rs`:
```rust
//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur io)
//! Ovo je JEDINO mjesto koje dira disk i procese. `pub use` izlaže tri stvari: trait `GitSource`
//! (ugovor), `GitCli` (implementacija kroz `git` proces) i `Project` (repo + profil + ručni podaci).
pub mod error;
pub mod git;
pub mod project;
pub use error::IoError;
pub use git::{GitCli, GitSource};
pub use project::Project;
```
`crates/sokratis-io/src/error.rs`:
```rust
use std::path::PathBuf;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum IoError {
    #[error("`git` nije na PATH-u")]
    GitMissing,
    #[error("{0} nije git repozitorij")]
    NotARepo(PathBuf),
    #[error("git {cmd}: {stderr}")]
    Git { cmd: String, stderr: String },
    #[error("profil {path}: {source}")]
    Profile { path: PathBuf, #[source] source: serde_json::Error },
    #[error("ručni podaci {path}: {source}")]
    Manual { path: PathBuf, #[source] source: serde_json::Error },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```
`crates/sokratis-io/src/git.rs`:
```rust
use crate::IoError;
use sokratis_core::BranchInfo;
use std::path::PathBuf;
pub trait GitSource {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError>;
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError>;
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError>;
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError>;
    fn common_dir(&self) -> Result<PathBuf, IoError>;
    fn toplevel(&self) -> Result<PathBuf, IoError>;
    fn branch_exists(&self, name: &str) -> Result<bool, IoError>;
    fn current_branch(&self) -> Result<String, IoError>;
}
pub struct GitCli { pub repo: PathBuf }
impl GitCli {
    pub fn new(repo: impl Into<PathBuf>) -> Self { Self { repo: repo.into() } }
}
impl GitSource for GitCli {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError> { let _ = (branch, since); todo!("cigla M1/15") }
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError> { let _ = default_branch; todo!("cigla M1/16") }
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError> { todo!("cigla M1/16") }
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError> { let _ = path; todo!("cigla M1/16") }
    fn common_dir(&self) -> Result<PathBuf, IoError> { todo!("cigla M1/15") }
    fn toplevel(&self) -> Result<PathBuf, IoError> { todo!("cigla M1/15") }
    fn branch_exists(&self, name: &str) -> Result<bool, IoError> { let _ = name; todo!("cigla M1/15") }
    fn current_branch(&self) -> Result<String, IoError> { todo!("cigla M1/15") }
}
```
`crates/sokratis-io/src/project.rs`:
```rust
use crate::{GitCli, IoError};
use sokratis_core::{DocFile, Profile, ReportInput, Vision, WorkKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
pub struct Project { pub root: PathBuf, pub common_dir: PathBuf, pub profile: Profile, pub git: GitCli }
impl Project {
    pub fn open(path: &Path) -> Result<Project, IoError> { let _ = path; todo!("cigla M1/17") }
    pub fn overrides(&self) -> Result<HashMap<String, WorkKind>, IoError> { todo!("cigla M1/17") }
    pub fn visions(&self) -> Result<Vec<Vision>, IoError> { todo!("cigla M1/17") }
    pub fn docs(&self) -> Result<Vec<DocFile>, IoError> { todo!("cigla M1/17") }
    pub fn input(&self, since: Option<&str>) -> Result<ReportInput, IoError> { let _ = since; todo!("cigla M1/17") }
}
```
`crates/sokratis-cli/src/main.rs`:
```rust
//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur CLI)
//! Binarna je tanka: `clap` parsira argumente, `anyhow` nosi grešku do `main`, a `main` je jedino
//! mjesto koje zove `std::process::exit` — izlazni kod je ugovor prema preflightu (0/1/2/3).
mod table;
fn main() {
    eprintln!("sokratis: kostur (cigla M1/18 puni naredbe)");
    std::process::exit(3);
}
```
`crates/sokratis-cli/src/table.rs`:
```rust
use sokratis_core::Report;
pub fn render(report: &Report) -> String { let _ = report; todo!("cigla M1/19") }
```

- [ ] **Step 7: gates i commit**

Run: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
Expected: sve zeleno; testovi `work_kind_serializes_snake_case`, `default_profile_compiles_and_rejects_unknown_field`, `test_and_code_paths` PASS. (Clippy može tražiti `#[allow(dead_code)]` na stubovima — dodaj `#![allow(dead_code)]` privremeno u `cli/src/table.rs` i ukloni u T19.)

```bash
git add -A
git commit -m "M1/1: kostur workspacea -- ugovor tipova i profila, sve ostalo todo!() po ciglama"
```

- [ ] **Step 8: grane i radna stabla za tokove (orkestrator)**

```powershell
cd C:\Users\leonk\Documents\sokratis
foreach ($t in @("fixtures","core-parse","core-metrics","core-rules","io","cli")) {
  $wt = "..\sokratis." + ($t -replace "core-","")
  git worktree add $wt -b "feat/$t" main
}
git worktree list
```
Expected: šest stabala uz `sokratis`, svako na svojoj grani od `main`.

---
### Task 2: Fixture pariteta iz Sokrat Studyja (tok FIXTURE, bez Rusta — može i prije M0)

**Files:**
- Create: `crates/sokratis-core/tests/fixtures/sokratstudy-2026-09-17.log`, `…-2026-09-17.PROGRESS.md`, `…-2026-09-17.RASPORED.md`, `…-2026-09-17.sha`, `…-2026-09-17.expected.json`, `…-2026-09-17.README.md`
- Create (alat, izvan repoa Sokratisa): `scratch/rad-xlsx-fixture.py`, `scratch/extract_expected.py`

**Interfaces:** Produces `expected.json` oblika `{ "sha", "since", "days": {date: {commits, cumulative, lines, hours_legacy, deliveries, deploys, test_lines}}, "kinds": {kind_id: {commits, share, lines}}, "indicators": {id: value}, "phases": [{name, state, done, total}] }`. Ključevi `kind_id` i `id` su engleski identifikatori iz T10/T11 (mapa dolje).

- [ ] **Step 1: snimi ulaz s `main`-a Sokrat Studyja (sve tri datoteke s ISTOG commita)**

```powershell
$S = "C:\Users\leonk\Documents\sokratstudy.dev"
$F = "C:\Users\leonk\Documents\sokratis.fixtures\crates\sokratis-core\tests\fixtures"
New-Item -ItemType Directory -Force $F | Out-Null
git -C $S rev-parse main | Out-File -Encoding ascii "$F\sokratstudy-2026-09-17.sha"
git -C $S log main --since=2026-08-02 --reverse --date=format:%Y-%m-%d "--format=@@%h|%at|%ct|%ad|%cd|%s" --numstat | Out-File -Encoding utf8 "$F\sokratstudy-2026-09-17.log"
git -C $S show main:docs/records/PROGRESS.md | Out-File -Encoding utf8 "$F\sokratstudy-2026-09-17.PROGRESS.md"
git -C $S show main:docs/plan/RASPORED.md | Out-File -Encoding utf8 "$F\sokratstudy-2026-09-17.RASPORED.md"
```
Expected: `.log` počinje s `@@` retkom i ima ~250 commita (od 2026-08-02); `.sha` = 40 hex znakova. ⚠️ `Out-File -Encoding utf8` na PowerShellu 7 piše bez BOM-a; provjeri `Get-Content -Raw … | Select-Object -First 1` da nema `ï»¿`.

- [ ] **Step 2: generiraj referentnu knjigu iz ISTOG stanja, u scratch (ne dira Leonov `RAD.xlsx`, bez ručnih overridea)**

```powershell
$W = "C:\Users\leonk\AppData\Local\Temp\sokratis-fixture"; New-Item -ItemType Directory -Force $W | Out-Null
Copy-Item "$S\scripts\rad-xlsx.py" "$W\rad-xlsx-fixture.py"
(Get-Content "$W\rad-xlsx-fixture.py" -Raw) `
  -replace "KORIJEN = os\.path\.abspath\(os\.path\.join\(os\.path\.dirname\(__file__\), '\.\.'\)\)", "KORIJEN = r'$S'" `
  -replace "IZLAZ = os\.path\.join\(KORIJEN, 'docs', 'records', 'RAD\.xlsx'\)", "IZLAZ = r'$W\RAD-fixture.xlsx'" `
  | Set-Content "$W\rad-xlsx-fixture.py" -Encoding utf8
git -C $S branch --show-current   # MORA ispisati: main
$env:PYTHONUTF8 = "1"; python "$W\rad-xlsx-fixture.py"
```
Expected: `✅ … commita · … isporuka · 11 faza · 21 vizija · grafova: 8` i datoteka `$W\RAD-fixture.xlsx`. Ako `branch --show-current` nije `main`, STANI — knjiga bi bila s krive grane.

- [ ] **Step 3: izvuci očekivano iz lista Sažetak u `expected.json`**

`$W\extract_expected.py`:
```python
import json, sys, openpyxl
xlsx, out, sha = sys.argv[1], sys.argv[2], open(sys.argv[3], encoding="ascii").read().strip()
ws = openpyxl.load_workbook(xlsx, data_only=True)["Sažetak"]
rows = [list(r) for r in ws.iter_rows(values_only=True)]
def section(title):
    i = next(k for k, r in enumerate(rows) if r and r[0] == title)
    j = i + 2  # naslov, zaglavlje, pa redci do praznog
    body = []
    while j < len(rows) and rows[j] and rows[j][0] is not None:
        body.append(rows[j]); j += 1
    return body
KIND = {"planiranje": "planning", "vođenje dokumentacije": "documentation", "izvođenje procesa": "execution",
        "poliranje koda": "polish", "debugging": "debugging"}
IND = {"radnih dana (dana s commitom)": "working_days", "commita": "commits", "commita po radnom danu": "commits_per_day",
       "isporuka (PROGRESS unosa)": "deliveries", "isporuka po radnom danu": "deliveries_per_day",
       "sati rada (git-hours proxy)": "hours_legacy", "commita po satu (proxy)": "commits_per_hour_legacy",
       "redaka promijenjeno (+/−)": "lines_changed", "redaka u testovima i branama": "test_lines",
       "udio testnih redaka": "test_share", "deploya na produkciju": "deploys", "debugging commita": "debugging_commits",
       "udio debugginga": "debugging_share", "udio dokumentacije": "docs_share", "CI-padova popravljenih": "ci_fixes",
       "isporuka pokrenutih Leonovim nalazom": "owner_driven_deliveries", "zatvorenih faza u razdoblju": "closed_phases_in_range",
       "prosječno trajanje zatvorene faze (dana)": "closed_phase_avg_days"}
STATE = {"zatvoreno": "closed", "u tijeku": "running", "planirano": "planned"}
exp = {"sha": sha, "since": "2026-08-29", "days": {}, "kinds": {}, "indicators": {}, "phases": []}
for d, n, kum, lines, hours, isp, dep, test in section("TEMPO PO DANU"):
    exp["days"][str(d)[:10]] = {"commits": n, "cumulative": kum, "lines": lines, "hours_legacy": hours,
                                "deliveries": isp, "deploys": dep, "test_lines": test}
for v, n, share, lines in section("VRSTE RADA (po commitima; „ručno\" pregazi „auto\")"):
    exp["kinds"][KIND[v]] = {"commits": n, "share": share, "lines": lines}
for name, val, _ in section("KVALITETA I BRZINA (razdoblje od 2026-08-29)"):
    exp["indicators"][IND[name]] = val
for name, done, left, state in section("FAZE — gotovo / preostalo (cigle)"):
    exp["phases"].append({"name": name, "state": STATE[state], "done": done, "total": done + left})
json.dump(exp, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
print("dana:", len(exp["days"]), "vrsta:", len(exp["kinds"]), "pokazatelja:", len(exp["indicators"]), "faza:", len(exp["phases"]))
```
Run: `python "$W\extract_expected.py" "$W\RAD-fixture.xlsx" "$F\sokratstudy-2026-09-17.expected.json" "$F\sokratstudy-2026-09-17.sha"`
Expected: `dana: N vrsta: 5 pokazatelja: 18 faza: 11`. Ako naslov sekcije ne pogodi (navodnici), otvori list i prepiši naslov doslovno.

- [ ] **Step 4: README uz fixture + commit**

`sokratstudy-2026-09-17.README.md`:
```markdown
# Fixture pariteta — Sokrat Study, main @ <sha>, snimljeno 2026-09-17

- `.log` = `git log main --since=2026-08-02 --reverse --date=format:%Y-%m-%d --format=@@%h|%at|%ct|%ad|%cd|%s --numstat`
  (od 2026-08-02 jer se zatvorene faze BROJE iz commita; metrike filtriraju `commit_date >= 2026-08-29`).
- `.PROGRESS.md` / `.RASPORED.md` = `git show main:<put>` s istog commita.
- `.expected.json` = list Sažetak knjige koju je `rad-xlsx.py` generirao nad ISTIM stanjem, bez ručnih overridea
  (svjež IZLAZ → nema `vrsta (ručno)`), izvučeno skriptom `extract_expected.py`.
- `hours_legacy` su sati Python-proxyja S KVAROM (S-007); test pariteta ih uspoređuje samo na danima bez
  cherry-pickova i tvrdi da su Sokratisovi sati ≥ 0.
```
```bash
git add crates/sokratis-core/tests/fixtures/
git commit -m "M1/2: fixture pariteta -- git log, dnevnik, plan i ocekivane brojke sa main-a Sokrat Studyja"
```

---

### Task 3: Parser git loga (tok PARSE)

**Files:**
- Modify: `crates/sokratis-core/src/parse/gitlog.rs`
- Test: inline `#[cfg(test)]`

**Interfaces:**
- Consumes: `Commit`, `FileChange`, `ParseError` (T1)
- Produces: `parse_git_log(text: &str) -> Result<Parsed, ParseError>`; `Parsed { commits, skipped_lines }`. Redak `@@sha|at|ct|ad|cd|subject`, zatim `added\tdeleted\tpath` redci (binarno = `-`), prazan redak razdvaja.

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const LOG: &str = "@@abc1234|1788664600|1788817583|2026-09-06|2026-09-08|fix(check:docs): gitignoriran artefakt nije duh-datoteka -- brana je padala\n3\t1\tscripts/check-docs.js\n-\t-\tassets/logo.png\n\n@@def5678|1788700000|1788700000|2026-09-06|2026-09-06|docs: zapis | s okomitom crtom\n10\t0\tdocs/records/PROGRESS.md\nsmeće bez tabova\n";

    #[test]
    fn parses_commits_files_binary_and_counts_skipped() {
        let p = parse_git_log(LOG).unwrap();
        assert_eq!(p.commits.len(), 2);
        let a = &p.commits[0];
        assert_eq!((a.sha.as_str(), a.author_time, a.commit_time), ("abc1234", 1788664600, 1788817583));
        assert_eq!((a.date.as_str(), a.commit_date.as_str()), ("2026-09-06", "2026-09-08"));
        assert_eq!(a.files.len(), 2);
        assert_eq!((a.files[0].added, a.files[0].deleted, a.files[0].path.as_str()), (3, 1, "scripts/check-docs.js"));
        assert_eq!((a.files[1].added, a.files[1].deleted), (0, 0));
        assert_eq!(p.commits[1].subject, "docs: zapis | s okomitom crtom");
        assert_eq!(p.skipped_lines, 1);
    }

    #[test]
    fn bad_header_is_an_error_with_line_number() {
        let err = parse_git_log("@@abc|notanumber|1|2026-01-01|2026-01-01|x\n").unwrap_err();
        assert!(matches!(err, ParseError::BadNumber { line: 1, .. }), "{err}");
        assert!(matches!(parse_git_log("@@abc|1|2\n").unwrap_err(), ParseError::BadLine { line: 1, .. }));
    }
}
```

- [ ] **Step 2: pokreni, mora pasti**

Run: `cargo test -p sokratis-core gitlog`
Expected: FAIL s `not yet implemented: cigla M1/3`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/3 — parser git loga)
//! `let … else` (Rust 1.65+): raspakiraj ili izađi s greškom u istom retku — bez ugniježđenog
//! `match`. `splitn(6, '|')` čuva `|` unutar opisa commita jer zadnji komad uzima ostatak.
//! `commits.last_mut()` = posudba zadnjeg elementa za upis (`&mut`) — jedan `&mut` u jednom trenu.
use crate::{Commit, FileChange, ParseError};

pub struct Parsed {
    pub commits: Vec<Commit>,
    pub skipped_lines: usize,
}

fn numstat(s: &str) -> Option<u64> {
    if s == "-" { Some(0) } else { s.parse().ok() }
}

pub fn parse_git_log(text: &str) -> Result<Parsed, ParseError> {
    let mut commits: Vec<Commit> = Vec::new();
    let mut skipped_lines = 0usize;
    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        if let Some(rest) = line.strip_prefix("@@") {
            let mut parts = rest.splitn(6, '|');
            let (Some(sha), Some(at), Some(ct), Some(date), Some(commit_date), Some(subject)) =
                (parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next())
            else {
                return Err(ParseError::BadLine { line: n, text: line.to_string() });
            };
            let num = |s: &str| s.parse::<i64>().map_err(|_| ParseError::BadNumber { line: n, text: s.to_string() });
            commits.push(Commit {
                sha: sha.to_string(),
                author_time: num(at)?,
                commit_time: num(ct)?,
                date: date.to_string(),
                commit_date: commit_date.to_string(),
                subject: subject.to_string(),
                files: Vec::new(),
            });
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        match (commits.last_mut(), cols.as_slice()) {
            (Some(cur), [a, d, path]) => match (numstat(a), numstat(d)) {
                (Some(added), Some(deleted)) => cur.files.push(FileChange { path: path.to_string(), added, deleted }),
                _ => skipped_lines += 1,
            },
            _ => skipped_lines += 1,
        }
    }
    Ok(Parsed { commits, skipped_lines })
}
```

- [ ] **Step 4: pokreni, mora proći** — Run: `cargo test -p sokratis-core gitlog` → 2 PASS; `cargo clippy --all-targets -- -D warnings` čist.

- [ ] **Step 5: commit** — `git commit -am "M1/3: parser git loga -- @@ zaglavlje + numstat, preskoceni redci se broje"`

---

### Task 4: Parser dnevnika (tok PARSE)

**Files:** Modify `crates/sokratis-core/src/parse/diary.rs`; Create `crates/sokratis-core/tests/fixtures/diary-sample.md`

**Interfaces:**
- Consumes: `Patterns.diary_heading` (3 grupe: datum, model, naslov), `Patterns.diary_deploy`, `classify_kind` (T6 — u ovom toku; do T6 test koristi naslove koji padaju u `Execution`... NE: T6 se radi PRIJE T4 u ovom toku, redoslijed: T3 → T6 → T4 → T5).
- Produces: `parse_diary(text, p, since) -> Vec<Delivery>` sortirano po datumu uzlazno (stabilno), samo `date >= since`.

- [ ] **Step 1: fixture + test koji pada**

`tests/fixtures/diary-sample.md`:
```markdown
# Progress Log

## 2026-09-13 (FABLE, sesija F2/2) — docs: 🚀 F2/2 slike profila na produkciji (main 61c39dd)

tekst

## 2026-09-12 (FABLE) — Napredak pouzdan: pet kvarova iz vanjske recenzije, svaki test-prvo

## 2026-08-14 (OPUS) — Popravak C2: prebacivanje teme slomilo je PRIJAVLJENE površine

## bez datuma — ne broji se
```
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Patterns, Profile, WorkKind};

    #[test]
    fn headings_become_deliveries_sorted_and_filtered() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let text = include_str!("../../tests/fixtures/diary-sample.md");
        let d = parse_diary(text, &p, "2026-08-29");
        assert_eq!(d.len(), 2, "unos prije `since` i naslov bez datuma otpadaju");
        assert_eq!(d[0].date, "2026-09-12");
        assert_eq!(d[0].model, "FABLE");
        assert!(d[0].title.starts_with("Napredak pouzdan"));
        assert_eq!(d[0].kind, WorkKind::Debugging, "'kvarova' → debugging");
        assert!(!d[0].deploy);
        assert_eq!(d[1].model, "FABLE, SESIJA F2/2");
        assert!(d[1].deploy, "🚀 u naslovu");
        assert_eq!(d[1].kind, WorkKind::Documentation, "naslov počinje s docs:");
    }
}
```

- [ ] **Step 2: pokreni, pada** — Run: `cargo test -p sokratis-core diary` → FAIL `not yet implemented`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/4 — parser dnevnika)
//! `text.lines()` + `captures(line)` po retku umjesto `(?m)` nad cijelim tekstom: isti ishod, a
//! greška u jednom naslovu ne ruši sve. `sort_by(|a, b| a.date.cmp(&b.date))` je STABILAN —
//! redoslijed unutar istog datuma ostaje kakav je u datoteci (kao Python `sorted`).
use crate::classify::classify_kind;
use crate::{Delivery, Patterns};

pub fn parse_diary(text: &str, p: &Patterns, since: &str) -> Vec<Delivery> {
    let mut out: Vec<Delivery> = text
        .lines()
        .filter_map(|line| p.diary_heading.captures(line))
        .filter_map(|c| {
            let date = c.get(1)?.as_str().to_string();
            if date.as_str() < since {
                return None;
            }
            let model = c.get(2).map(|m| m.as_str().trim().to_uppercase()).unwrap_or_default();
            let title = c.get(3)?.as_str().trim().to_string();
            let lower = title.to_lowercase();
            let deploy = title.contains('🚀') || p.diary_deploy.is_match(&lower);
            let kind = classify_kind(&title, p);
            Some(Delivery { date, model, title, kind, deploy })
        })
        .collect();
    out.sort_by(|a, b| a.date.cmp(&b.date));
    out
}
```

- [ ] **Step 4: pokreni, prolazi** — `cargo test -p sokratis-core diary` → PASS; clippy čist.
- [ ] **Step 5: commit** — `git add -A && git commit -m "M1/4: parser dnevnika -- naslov = isporuka, model, deploy-oznaka, filtar od since"`

---

### Task 5: Parser plana (tok PARSE)

**Files:** Modify `crates/sokratis-core/src/parse/plan.rs`; Create `tests/fixtures/plan-sample.md`

**Interfaces:**
- Consumes: `Patterns.plan_phase_name` (grupe: id, ime), `Patterns.plan_brick` (grupe: id, broj, `✅` ili prazno), `Phase`, `PhaseState`.
- Produces: `parse_plan(text, p) -> Vec<Phase>` redoslijedom prvog pojavljivanja; `from/to/days = None`, `commits = 0` (puni T11).

- [ ] **Step 1: fixture + test koji pada**

`tests/fixtures/plan-sample.md`:
```markdown
### F1 · UREĐAJ — izgled i glatkoća
| **F1/1** ✅ | nešto |
| **F1/2** ✅ | nešto |
| **F1/3** | čeka |
### F2 · RAČUN
| **F2/1** ✅ | gotovo |
### F3 · DVOJEZIČNOST
| **F3/1** | planirano |
| **F3/2** | planirano |
### F4 · ČIŠĆENJE
| **F4/1** ✅ | gotovo |
| **F4/2** ✅ | gotovo |
```
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Patterns, PhaseState, Profile};

    #[test]
    fn bricks_and_states_from_plan() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let ph = parse_plan(include_str!("../../tests/fixtures/plan-sample.md"), &p);
        let ids: Vec<&str> = ph.iter().map(|x| x.id.as_str()).collect();
        assert_eq!(ids, ["F1", "F2", "F3", "F4"]);
        assert_eq!(ph[0].name, "F1 · UREĐAJ — izgled i glatkoća");
        assert_eq!((ph[0].total_bricks, ph[0].done_bricks, ph[0].state), (3, 2, PhaseState::Running));
        assert_eq!((ph[1].total_bricks, ph[1].done_bricks, ph[1].state), (1, 1, PhaseState::Closed));
        assert_eq!((ph[2].total_bricks, ph[2].done_bricks, ph[2].state), (2, 0, PhaseState::Planned));
        assert_eq!(ph[3].state, PhaseState::Closed);
        assert!(ph[0].from.is_none() && ph[0].commits == 0);
    }
}
```

- [ ] **Step 2: pokreni, pada** — `cargo test -p sokratis-core plan` → FAIL

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/5 — parser plana)
//! Dva prolaza kroz iste retke: prvi skuplja imena faza u `HashMap`, drugi broji cigle. `Vec` +
//! `iter_mut().find()` čuva REDOSLIJED pojavljivanja (HashMap ga ne bi), a faza ih je najviše
//! desetak pa je linearno traženje jeftinije od dodatne strukture.
use crate::{Patterns, Phase, PhaseState};
use std::collections::HashMap;

pub fn parse_plan(text: &str, p: &Patterns) -> Vec<Phase> {
    let names: HashMap<String, String> = text
        .lines()
        .filter_map(|l| p.plan_phase_name.captures(l))
        .map(|c| (c[1].to_string(), c[2].trim().to_string()))
        .collect();
    let mut phases: Vec<Phase> = Vec::new();
    for c in text.lines().filter_map(|l| p.plan_brick.captures(l)) {
        let id = c[1].to_string();
        let done = !c[3].is_empty();
        let phase = match phases.iter_mut().find(|ph| ph.id == id) {
            Some(ph) => ph,
            None => {
                let name = match names.get(&id) {
                    Some(n) => format!("{id} · {n}"),
                    None => id.clone(),
                };
                phases.push(Phase { id: id.clone(), name, state: PhaseState::Planned, total_bricks: 0, done_bricks: 0, from: None, to: None, days: None, commits: 0 });
                phases.last_mut().expect("upravo dodano")
            }
        };
        phase.total_bricks += 1;
        if done {
            phase.done_bricks += 1;
        }
    }
    for ph in &mut phases {
        ph.state = if ph.total_bricks > 0 && ph.done_bricks == ph.total_bricks {
            PhaseState::Closed
        } else if ph.done_bricks > 0 {
            PhaseState::Running
        } else {
            PhaseState::Planned
        };
    }
    phases
}
```
⚠️ `expect("upravo dodano")` je jedini `expect` izvan testova u jezgri; ako clippy ili recenzent prigovore, zamijeni s `if let Some(ph) = phases.last_mut() { … }`.

- [ ] **Step 4: pokreni, prolazi** · **Step 5: commit** — `git add -A && git commit -m "M1/5: parser plana -- cigle po fazi, stanje planirano/u tijeku/zatvoreno"`

---

### Task 6: Klasifikator vrste i podvrste (tok PARSE — radi se ODMAH nakon T3, prije T4)

**Files:** Modify `crates/sokratis-core/src/classify.rs`

**Interfaces:**
- Consumes: `Patterns.classifier` (uređeno), `gate`, `deploy`, `phase_tag`.
- Produces: `classify_kind(subject, p) -> WorkKind` (prvi pogodak po redu, inače `Execution`; uspoređuje se MALIM slovima); `classify_sub(subject, p) -> SubKind` (deploy > gate > brick > other); `phase_tag(subject, p) -> Option<String>` (grupa 1).

- [ ] **Step 1: test koji pada (tablica stvarnih naslova iz Sokrat Studyja)**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Profile, SubKind::*, WorkKind::*};

    #[test]
    fn kinds_follow_sokrat_study_order() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let cases = [
            ("docs: BUGS.md razdvojen + RASPORED §0 SPAJANJE", Planning),       // 'raspored' prije 'docs'
            ("docs(progress): sesija 07.09. -- model monetizacije zakljucan", Documentation),
            ("fix(check:docs): gitignoriran artefakt nije duh-datoteka", Debugging),
            ("BUG-045: Service Worker nikad nije spremao runtime assete", Debugging),
            ("C5b/1: paleta ostatak", Polish),
            ("F2/2 cigla 3: zid crta profilnu i naslovnu", Execution),
            ("N1: naucena kartica se pamti po IDENTITETU", Execution),
        ];
        for (s, want) in cases {
            assert_eq!(classify_kind(s, &p), want, "{s}");
        }
    }

    #[test]
    fn sub_kinds_and_tags() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        assert_eq!(classify_sub("docs: 🚀 F2/2 slike profila na produkciji", &p), Deploy);
        assert_eq!(classify_sub("MREZA A2: check:node napisan i dokazan", &p), GateOrMeasure);
        assert_eq!(classify_sub("F2/2 cigla 3: zid", &p), Brick);
        assert_eq!(classify_sub("N1: naucena kartica", &p), Other);
        assert_eq!(phase_tag("F2/2 cigla 3: zid", &p).as_deref(), Some("F2/2"));
        assert_eq!(phase_tag("MREZA A1: baza popravljena", &p).as_deref(), Some("MREZA A1"));
        assert_eq!(phase_tag("docs: nešto", &p), None);
    }
}
```

- [ ] **Step 2: pokreni, pada** — `cargo test -p sokratis-core classify`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/6 — klasifikator)
//! `for (kind, re) in &p.classifier` posuđuje vektor parova bez kopiranja; `*kind` kopira `Copy`
//! enum iz reference. Redoslijed pravila je dio ugovora (planiranje > dokumentacija > debugging >
//! poliranje), zato je to `Vec`, ne `HashMap`.
use crate::{Patterns, SubKind, WorkKind};

pub fn classify_kind(subject: &str, p: &Patterns) -> WorkKind {
    let t = subject.to_lowercase();
    for (kind, re) in &p.classifier {
        if re.is_match(&t) {
            return *kind;
        }
    }
    WorkKind::Execution
}

pub fn classify_sub(subject: &str, p: &Patterns) -> SubKind {
    let t = subject.to_lowercase();
    if subject.contains('🚀') || p.deploy.is_match(&t) {
        SubKind::Deploy
    } else if p.gate.is_match(&t) {
        SubKind::GateOrMeasure
    } else if p.phase_tag.is_match(subject) {
        SubKind::Brick
    } else {
        SubKind::Other
    }
}

pub fn phase_tag(subject: &str, p: &Patterns) -> Option<String> {
    p.phase_tag.captures(subject).map(|c| c[1].to_string())
}
```

- [ ] **Step 4: pokreni, prolazi** · **Step 5: commit** — `git commit -am "M1/6: klasifikator vrste i podvrste -- regexi iz rad-xlsx.py 1:1, redoslijed je ugovor"`

---
### Task 7: Civilni datumi bez ovisnosti (tok METRIKE)

**Files:** Modify `crates/sokratis-core/src/civil.rs`

**Interfaces:** Produces `days_between(from: &str, to: &str) -> Option<i64>` = `to − from` u danima za `YYYY-MM-DD` (negativno ako je `to` prije); `None` ako format ne valja. Koriste T11 (trajanje faza) i T14.

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn days_between_handles_month_and_year_edges() {
        assert_eq!(days_between("2026-08-02", "2026-08-06"), Some(4));
        assert_eq!(days_between("2026-08-07", "2026-09-01"), Some(25));
        assert_eq!(days_between("2025-12-31", "2026-01-01"), Some(1));
        assert_eq!(days_between("2024-02-28", "2024-03-01"), Some(2), "prijestupna");
        assert_eq!(days_between("2026-09-02", "2026-09-01"), Some(-1));
        assert_eq!(days_between("2026-9-2", "2026-09-01"), None);
        assert_eq!(days_between("x", "2026-09-01"), None);
    }
}
```

- [ ] **Step 2: pokreni, pada** — `cargo test -p sokratis-core civil`

- [ ] **Step 3: implementacija (Hinnant, days_from_civil)**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/7 — civilni datumi)
//! Nula ovisnosti: `chrono` bi ovdje bio top za muhu. `Option` kroz `?` u pomoćnoj funkciji:
//! prvi neuspjeli `parse` vraća `None` i gotovo. Cijeli brojevi (`i64`) jer je danima mjesto u
//! cijelim brojevima, a prijestupne godine rješava formula, ne tablica.
fn ymd(s: &str) -> Option<(i64, i64, i64)> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    Some((s[0..4].parse().ok()?, s[5..7].parse().ok()?, s[8..10].parse().ok()?))
}

/// Dani od 1970-01-01 (algoritam H. Hinnanta, „days_from_civil").
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

pub fn days_between(from: &str, to: &str) -> Option<i64> {
    let (fy, fm, fd) = ymd(from)?;
    let (ty, tm, td) = ymd(to)?;
    Some(days_from_civil(ty, tm, td) - days_from_civil(fy, fm, fd))
}
```

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/7: civil.rs -- dani izmedju datuma bez ovisnosti"`

---

### Task 8: Sati rada — git-hours proxy, ISPRAVLJEN (tok METRIKE)

**Files:** Modify `crates/sokratis-core/src/metrics/hours.rs`

**Interfaces:** Produces `hours_per_day(commits, gap_h, start_h) -> BTreeMap<String, f64>`; ključ = `Commit.date`; sortira po `author_time` uzlazno; razmak `< gap_h` = rad, inače `+ start_h`; sat ide danu commita koji zatvara razmak; zaokruženo na 1 decimalu.

- [ ] **Step 1: test koji pada — tri stvarna commita iz nalaza (S-007)**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Commit;

    fn c(sha: &str, author_time: i64, date: &str) -> Commit {
        Commit { sha: sha.into(), author_time, commit_time: author_time, date: date.into(), commit_date: date.into(), subject: String::new(), files: vec![] }
    }

    #[test]
    fn cherry_pick_never_yields_negative_hours() {
        // Redoslijed loga: X (07.09. 23:45) pa cherry-pick Y s autorskim datumom 06.09. 05:16.
        // Stari proxy: Y − X = −42,5 h. Novi: sortiraj pa računaj.
        let log_order = vec![
            c("x", 1_788_817_514, "2026-09-07"),
            c("y", 1_788_664_600, "2026-09-06"),
            c("z", 1_788_817_514 + 3_600, "2026-09-07"),
        ];
        let h = hours_per_day(&log_order, 2.0, 0.5);
        assert!(h.values().all(|v| *v >= 0.0), "{h:?}");
        assert_eq!(h["2026-09-06"], 0.5, "prvi u nizu dobiva početak sesije");
        assert_eq!(h["2026-09-07"], 1.5, "x otvara novu sesiju (+0,5), z zatvara razmak od 1 h");
    }

    #[test]
    fn gap_equal_to_threshold_starts_new_session_and_rounds() {
        let v = vec![c("a", 0, "1970-01-01"), c("b", 7_200, "1970-01-01"), c("c", 7_200 + 4_000, "1970-01-01")];
        let h = hours_per_day(&v, 2.0, 0.5);
        assert_eq!(h["1970-01-01"], 0.5 + 0.5 + 1.1, "4000 s = 1,11 h → 1,1");
    }
}
```

- [ ] **Step 2: pokreni, pada** — `cargo test -p sokratis-core hours`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/8 — sati)
//! `Vec<&Commit>` = vektor POSUDBI: sortiramo reference, ne kopiramo commite. `BTreeMap` umjesto
//! `HashMap` jer želimo dane uzlazno bez naknadnog sortiranja. `entry().or_insert()` = „uzmi ili
//! stvori" u jednom potezu. Ovo je ispravak S-007: sortiranje po `author_time` čini negativan
//! razmak nemogućim.
use crate::Commit;
use std::collections::BTreeMap;

pub fn hours_per_day(commits: &[Commit], gap_h: f64, start_h: f64) -> BTreeMap<String, f64> {
    let mut sorted: Vec<&Commit> = commits.iter().collect();
    sorted.sort_by_key(|c| c.author_time);
    let mut hours: BTreeMap<String, f64> = BTreeMap::new();
    let mut prev: Option<i64> = None;
    for c in sorted {
        let gap = prev.map(|p| (c.author_time - p) as f64 / 3600.0);
        let add = match gap {
            Some(g) if g < gap_h => g,
            _ => start_h,
        };
        *hours.entry(c.date.clone()).or_insert(0.0) += add;
        prev = Some(c.author_time);
    }
    hours.into_iter().map(|(d, h)| (d, (h * 10.0).round() / 10.0)).collect()
}
```

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/8: sati rada -- sortiranje po author_time, negativan razmak nemoguc (S-007)"`

---

### Task 9: Tempo po danu (tok METRIKE)

**Files:** Modify `crates/sokratis-core/src/metrics/days.rs`

**Interfaces:** Produces `day_stats(commits, deliveries, hours, profile) -> Vec<DayStats>` uzlazno po datumu; samo dani s commitom; `lines = Σ added+deleted`; `test_lines` = redci datoteka za koje `profile.is_test_path`; `deliveries`/`deploys` po `Delivery.date`; `hours` iz mape (0.0 ako nema).

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Delivery, FileChange, Profile, WorkKind};

    fn c(date: &str, files: Vec<(&str, u64, u64)>) -> Commit {
        Commit { sha: date.into(), author_time: 0, commit_time: 0, date: date.into(), commit_date: date.into(), subject: String::new(),
            files: files.into_iter().map(|(p, a, d)| FileChange { path: p.into(), added: a, deleted: d }).collect() }
    }
    fn d(date: &str, deploy: bool) -> Delivery {
        Delivery { date: date.into(), model: String::new(), title: String::new(), kind: WorkKind::Execution, deploy }
    }

    #[test]
    fn groups_by_day_with_cumulative_tests_and_deliveries() {
        let commits = vec![
            c("2026-09-02", vec![("js/a.js", 10, 2), ("tests/a.test.js", 5, 0)]),
            c("2026-09-01", vec![("scripts/check-x.js", 7, 7)]),
            c("2026-09-02", vec![("docs/x.md", 1, 1)]),
        ];
        let deliveries = vec![d("2026-09-02", true), d("2026-09-02", false), d("2026-09-05", false)];
        let mut hours = BTreeMap::new();
        hours.insert("2026-09-02".to_string(), 1.5);
        let days = day_stats(&commits, &deliveries, &hours, &Profile::default());
        assert_eq!(days.len(), 2, "05.09. nema commit → nema retka");
        assert_eq!((days[0].date.as_str(), days[0].commits, days[0].commits_cumulative, days[0].lines, days[0].test_lines), ("2026-09-01", 1, 1, 14, 14));
        assert_eq!((days[1].commits, days[1].commits_cumulative, days[1].lines, days[1].test_lines), (2, 3, 19, 5));
        assert_eq!((days[1].deliveries, days[1].deploys, days[1].hours), (2, 1, 1.5));
        assert_eq!(days[0].hours, 0.0);
    }
}
```

- [ ] **Step 2: pokreni, pada** — `cargo test -p sokratis-core days`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/9 — tempo po danu)
//! `BTreeMap<String, Acc>` s privatnom `#[derive(Default)]` strukturom akumulatora: `entry().or_default()`
//! stvara prazan akumulator kad dan prvi put naiđe. Zatim jedan prolaz uzlazno gradi kumulativ.
use crate::{Commit, DayStats, Delivery, Profile};
use std::collections::BTreeMap;

#[derive(Default)]
struct Acc { commits: u32, lines: u64, test_lines: u64 }

pub fn day_stats(commits: &[Commit], deliveries: &[Delivery], hours: &BTreeMap<String, f64>, p: &Profile) -> Vec<DayStats> {
    let mut acc: BTreeMap<&str, Acc> = BTreeMap::new();
    for c in commits {
        let a = acc.entry(c.date.as_str()).or_default();
        a.commits += 1;
        for f in &c.files {
            let n = f.added + f.deleted;
            a.lines += n;
            if p.is_test_path(&f.path) {
                a.test_lines += n;
            }
        }
    }
    let mut cumulative = 0u32;
    acc.into_iter()
        .map(|(date, a)| {
            cumulative += a.commits;
            let on_day = deliveries.iter().filter(|d| d.date == date);
            DayStats {
                date: date.to_string(),
                commits: a.commits,
                commits_cumulative: cumulative,
                lines: a.lines,
                hours: hours.get(date).copied().unwrap_or(0.0),
                deliveries: on_day.clone().count() as u32,
                deploys: on_day.filter(|d| d.deploy).count() as u32,
                test_lines: a.test_lines,
            }
        })
        .collect()
}
```

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/9: tempo po danu -- commiti, kumulativ, redci, testni redci, isporuke, deployi"`

---

### Task 10: Vrste rada s ručnim overrideom (tok METRIKE)

**Files:** Modify `crates/sokratis-core/src/metrics/kinds.rs`

**Interfaces:** Produces `effective_kind(c, overrides, p) -> WorkKind` (override po SHA pobjeđuje `classify_kind`); `kind_stats(commits, overrides, p) -> Vec<KindStats>` u redoslijedu `WorkKind::ALL`, sve vrste prisutne i s nulom; `share = round(commits/total, 3)`, total 0 → share 0.

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FileChange, Patterns, Profile};

    fn c(sha: &str, subject: &str, lines: u64) -> Commit {
        Commit { sha: sha.into(), author_time: 0, commit_time: 0, date: "2026-09-01".into(), commit_date: "2026-09-01".into(),
            subject: subject.into(), files: vec![FileChange { path: "x".into(), added: lines, deleted: 0 }] }
    }

    #[test]
    fn override_wins_and_all_kinds_are_listed() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let commits = vec![c("a", "fix: nešto", 10), c("b", "fix: drugo", 20), c("c", "F1/1 cigla", 30)];
        let mut ov = HashMap::new();
        ov.insert("b".to_string(), WorkKind::Polish);
        assert_eq!(effective_kind(&commits[0], &ov, &p), WorkKind::Debugging);
        assert_eq!(effective_kind(&commits[1], &ov, &p), WorkKind::Polish);
        let ks = kind_stats(&commits, &ov, &p);
        let kinds: Vec<WorkKind> = ks.iter().map(|k| k.kind).collect();
        assert_eq!(kinds, WorkKind::ALL.to_vec());
        let get = |k: WorkKind| ks.iter().find(|x| x.kind == k).unwrap();
        assert_eq!((get(WorkKind::Debugging).commits, get(WorkKind::Debugging).lines, get(WorkKind::Debugging).share), (1, 10, 0.333));
        assert_eq!((get(WorkKind::Polish).commits, get(WorkKind::Execution).commits, get(WorkKind::Planning).commits), (1, 1, 0));
    }
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-core kinds`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/10 — vrste rada)
//! `HashMap<String, WorkKind>::get(&c.sha).copied()` vraća `Option<WorkKind>` bez posudbe koja bi
//! nadživjela funkciju; `.unwrap_or_else(|| classify_kind(..))` računa heuristiku SAMO kad
//! overridea nema (lijeno). `WorkKind::ALL` jamči da su sve vrste u izlazu i kad imaju nulu.
use crate::classify::classify_kind;
use crate::{Commit, KindStats, Patterns, WorkKind};
use std::collections::HashMap;

pub fn effective_kind(c: &Commit, overrides: &HashMap<String, WorkKind>, p: &Patterns) -> WorkKind {
    overrides.get(&c.sha).copied().unwrap_or_else(|| classify_kind(&c.subject, p))
}

pub fn kind_stats(commits: &[Commit], overrides: &HashMap<String, WorkKind>, p: &Patterns) -> Vec<KindStats> {
    let total = commits.len() as f64;
    WorkKind::ALL
        .iter()
        .map(|&kind| {
            let mine = commits.iter().filter(|c| effective_kind(c, overrides, p) == kind);
            let n = mine.clone().count() as u32;
            let lines = mine.flat_map(|c| c.files.iter()).map(|f| f.added + f.deleted).sum();
            let share = if total > 0.0 { (n as f64 / total * 1000.0).round() / 1000.0 } else { 0.0 };
            KindStats { kind, commits: n, share, lines }
        })
        .collect()
}
```

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/10: vrste rada -- override po SHA pobjedjuje heuristiku, sve vrste u izlazu"`

---

### Task 11: Faze i 18 pokazatelja (tok METRIKE)

**Files:** Modify `crates/sokratis-core/src/metrics/phases.rs`, `crates/sokratis-core/src/metrics/indicators.rs`

**Interfaces:**
- `closed_phases(all_commits, p) -> Vec<Phase>`: za svaku `Patterns.closed_phases` (ClosedPhase, Regex): `commits` = broj commita s `date` u `[from, to]` čiji `subject` pogađa regex; `total = done = commits`; `days = days_between(from,to)+1`; `state = Closed`; `id` = ime.
- `active_phases(plan_phases, commits, today) -> Vec<Phase>`: za svaku fazu iz plana: `from` = `date` prvog commita (po `author_time`) čiji `subject` počinje s `"{id}/"`; ako postoji: `days = days_between(from, today)+1`, `commits` = broj takvih commita; `to = Some(today)` samo ako `state == Closed`.
- `indicators(input, profile, p) -> Vec<Indicator>` — 18 pokazatelja, id-jevi i formule u tablici dolje, zaokruživanje kao Python.

| id | vrijednost | kind |
|---|---|---|
| `working_days` | broj dana u `days` | measure |
| `commits` | broj commita | measure |
| `commits_per_day` | round(commits / working_days, 1) | measure |
| `deliveries` | broj isporuka | measure |
| `deliveries_per_day` | round(deliveries / working_days, 1) | measure |
| `hours` | round(Σ days.hours, 1) | proxy |
| `commits_per_hour` | round(commits / hours, 1), 0 ako hours = 0 | proxy |
| `lines_changed` | Σ added+deleted | measure |
| `test_lines` | Σ redaka testnih putanja | measure |
| `test_share` | round(test_lines / max(1, lines_changed), 3) | measure |
| `deploys` | isporuke s `deploy` | measure |
| `debugging_commits` | commiti s `effective_kind == Debugging` | measure |
| `debugging_share` | round(debugging / max(1, commits), 3) | measure |
| `docs_share` | round(documentation / max(1, commits), 3) | measure |
| `ci_fixes` | commiti čiji lowercase subject pogađa `ci_fix` | measure |
| `owner_driven_deliveries` | isporuke čiji lowercase naslov sadrži `owner_name` | measure |
| `closed_phases_in_range` | zatvorene faze s `to >= profile.since` | measure |
| `closed_phase_avg_days` | round(mean(days) po zatvorenim fazama, 1); 0 ako nema | measure |

Napomena: `working_days` = 0 → sve „per day" = 0 (Python koristi `or 1`, što daje isti rezultat jer je i brojnik 0).

- [ ] **Step 1: test koji pada**

```rust
// phases.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Patterns, PhaseState, Profile};

    fn c(sha: &str, t: i64, date: &str, subject: &str) -> Commit {
        Commit { sha: sha.into(), author_time: t, commit_time: t, date: date.into(), commit_date: date.into(), subject: subject.into(), files: vec![] }
    }

    #[test]
    fn closed_phases_count_tagged_commits_in_range() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let all = vec![
            c("a", 1, "2026-09-02", "R1: Google prijava"),
            c("b", 2, "2026-09-02", "R1/U5: id_token"),
            c("c", 3, "2026-09-03", "R1 kasni — izvan raspona"),
            c("d", 4, "2026-08-31", "MREZA A1: baza"),
        ];
        let ph = closed_phases(&all, &p);
        assert_eq!(ph.len(), 4);
        let r1 = ph.iter().find(|x| x.name.starts_with("RAČUN R1")).unwrap();
        assert_eq!((r1.commits, r1.done_bricks, r1.total_bricks, r1.days, r1.state), (2, 2, 2, Some(1), PhaseState::Closed));
        let mreza = ph.iter().find(|x| x.name.starts_with("MREŽA")).unwrap();
        assert_eq!((mreza.commits, mreza.days), (1, Some(2)));
    }

    #[test]
    fn active_phase_gets_start_from_first_tagged_commit() {
        let plan = vec![Phase { id: "F1".into(), name: "F1 · UREĐAJ".into(), state: PhaseState::Running, total_bricks: 3, done_bricks: 2, from: None, to: None, days: None, commits: 0 },
                        Phase { id: "F3".into(), name: "F3".into(), state: PhaseState::Planned, total_bricks: 2, done_bricks: 0, from: None, to: None, days: None, commits: 0 }];
        let commits = vec![c("x", 20, "2026-09-05", "F1/2 druga"), c("y", 10, "2026-09-04", "F1/1 prva"), c("z", 30, "2026-09-06", "docs: ne")];
        let ph = active_phases(plan, &commits, "2026-09-06");
        assert_eq!((ph[0].from.as_deref(), ph[0].days, ph[0].commits), (Some("2026-09-04"), Some(3), 2));
        assert_eq!((ph[1].from, ph[1].days, ph[1].commits), (None, None, 0));
    }
}
```
```rust
// indicators.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{day_stats, hours_per_day};
    use crate::{FileChange, PhaseState};

    fn c(sha: &str, t: i64, subject: &str, path: &str, lines: u64) -> Commit {
        Commit { sha: sha.into(), author_time: t, commit_time: t, date: "2026-09-01".into(), commit_date: "2026-09-01".into(), subject: subject.into(),
            files: vec![FileChange { path: path.into(), added: lines, deleted: 0 }] }
    }

    #[test]
    fn eighteen_indicators_with_python_rounding() {
        let profile = Profile::default();
        let p = Patterns::compile(&profile).unwrap();
        let commits = vec![c("a", 0, "fix: kvar", "js/a.js", 90), c("b", 3600, "docs: zapis", "docs/a.md", 5), c("c", 7200, "ci: popravak ci", "tests/a.test.js", 5)];
        let deliveries = vec![Delivery { date: "2026-09-01".into(), model: "".into(), title: "Leonov nalaz 🚀".into(), kind: WorkKind::Execution, deploy: true }];
        let hours = hours_per_day(&commits, 2.0, 0.5);
        let days = day_stats(&commits, &deliveries, &hours, &profile);
        let phases = vec![Phase { id: "x".into(), name: "x".into(), state: PhaseState::Closed, total_bricks: 1, done_bricks: 1, from: Some("2026-08-31".into()), to: Some("2026-09-01".into()), days: Some(2), commits: 1 }];
        let ov = HashMap::new();
        let ind = indicators(&IndicatorInput { commits: &commits, deliveries: &deliveries, days: &days, phases: &phases, overrides: &ov }, &profile, &p);
        assert_eq!(ind.len(), 18);
        let v = |id: &str| ind.iter().find(|i| i.id == id).unwrap_or_else(|| panic!("{id}")).value;
        assert_eq!((v("working_days"), v("commits"), v("commits_per_day")), (1.0, 3.0, 3.0));
        assert_eq!((v("hours"), v("commits_per_hour")), (2.5, 1.2));
        assert_eq!((v("lines_changed"), v("test_lines"), v("test_share")), (100.0, 5.0, 0.05));
        assert_eq!((v("deploys"), v("debugging_commits"), v("debugging_share"), v("docs_share")), (1.0, 2.0, 0.667, 0.333));
        assert_eq!((v("ci_fixes"), v("owner_driven_deliveries")), (1.0, 1.0));
        assert_eq!((v("closed_phases_in_range"), v("closed_phase_avg_days")), (1.0, 2.0));
        assert!(ind.iter().filter(|i| i.kind == IndicatorKind::Proxy).count() == 2);
    }
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-core phases indicators`

- [ ] **Step 3: implementacija**

`phases.rs`:
```rust
//! ZAŠTO RUST OVAKO (cigla M1/11a — faze)
//! Zatvorene faze se BROJE iz commita (mjera), ne prepisuju (procjena). `plan_phases: Vec<Phase>`
//! se uzima u VLASNIŠTVO i mutira u mjestu (`for ph in &mut phases`) — pozivatelju ionako ne
//! treba stara verzija, pa nema kloniranja.
use crate::civil::days_between;
use crate::{Commit, Patterns, Phase, PhaseState};

pub fn closed_phases(all_commits: &[Commit], p: &Patterns) -> Vec<Phase> {
    p.closed_phases
        .iter()
        .map(|(cp, re)| {
            let n = all_commits
                .iter()
                .filter(|c| c.date.as_str() >= cp.from.as_str() && c.date.as_str() <= cp.to.as_str() && re.is_match(&c.subject))
                .count() as u32;
            Phase {
                id: cp.name.clone(),
                name: cp.name.clone(),
                state: PhaseState::Closed,
                total_bricks: n,
                done_bricks: n,
                from: Some(cp.from.clone()),
                to: Some(cp.to.clone()),
                days: days_between(&cp.from, &cp.to).map(|d| d + 1),
                commits: n,
            }
        })
        .collect()
}

pub fn active_phases(mut plan_phases: Vec<Phase>, commits: &[Commit], today: &str) -> Vec<Phase> {
    for ph in &mut plan_phases {
        let prefix = format!("{}/", ph.id);
        let mut tagged: Vec<&Commit> = commits.iter().filter(|c| c.subject.starts_with(&prefix)).collect();
        tagged.sort_by_key(|c| c.author_time);
        if let Some(first) = tagged.first() {
            ph.from = Some(first.date.clone());
            ph.days = days_between(&first.date, today).map(|d| d + 1);
            ph.commits = tagged.len() as u32;
        }
        if ph.state == PhaseState::Closed {
            ph.to = Some(today.to_string());
        }
    }
    plan_phases
}
```

`indicators.rs`:
```rust
//! ZAŠTO RUST OVAKO (cigla M1/11b — pokazatelji)
//! `IndicatorInput<'a>` je PRVI lifetime u jezgri: struktura koja samo POSUĐUJE pet slice-ova
//! dok traje jedan poziv — kopirati ih bilo bi rasipno, a vlasništvo nepotrebno. `'a` kaže
//! „svi žive bar koliko i ovaj ulaz". Mala zatvaranja `r1`/`r3` drže zaokruživanje na jednom mjestu.
use crate::metrics::effective_kind;
use crate::{Commit, DayStats, Delivery, Indicator, IndicatorKind, Patterns, Phase, PhaseState, Profile, WorkKind};
use std::collections::HashMap;

pub struct IndicatorInput<'a> {
    pub commits: &'a [Commit],
    pub deliveries: &'a [Delivery],
    pub days: &'a [DayStats],
    pub phases: &'a [Phase],
    pub overrides: &'a HashMap<String, WorkKind>,
}

pub fn indicators(input: &IndicatorInput<'_>, profile: &Profile, p: &Patterns) -> Vec<Indicator> {
    let r1 = |x: f64| (x * 10.0).round() / 10.0;
    let r3 = |x: f64| (x * 1000.0).round() / 1000.0;
    let days = input.days.len() as f64;
    let commits = input.commits.len() as f64;
    let deliveries = input.deliveries.len() as f64;
    let hours = r1(input.days.iter().map(|d| d.hours).sum());
    let lines: u64 = input.commits.iter().flat_map(|c| &c.files).map(|f| f.added + f.deleted).sum();
    let test_lines: u64 = input.days.iter().map(|d| d.test_lines).sum();
    let kind_count = |k: WorkKind| input.commits.iter().filter(|c| effective_kind(c, input.overrides, p) == k).count() as f64;
    let closed: Vec<&Phase> = input.phases.iter().filter(|ph| ph.state == PhaseState::Closed).collect();
    let per = |a: f64, b: f64| if b > 0.0 { a / b } else { 0.0 };
    let m = IndicatorKind::Measure;
    let x = IndicatorKind::Proxy;
    let mk = |id: &str, value: f64, kind: IndicatorKind, formula: &str| Indicator { id: id.into(), value, kind, formula: formula.into() };
    vec![
        mk("working_days", days, m, "dana s bar jednim commitom"),
        mk("commits", commits, m, "broj commita od since"),
        mk("commits_per_day", r1(per(commits, days)), m, "commits / working_days"),
        mk("deliveries", deliveries, m, "naslova u dnevniku od since"),
        mk("deliveries_per_day", r1(per(deliveries, days)), m, "deliveries / working_days"),
        mk("hours", hours, x, "git-hours: razmak < gap_h + start_h po sesiji, sortirano po author_time"),
        mk("commits_per_hour", r1(per(commits, hours)), x, "commits / hours"),
        mk("lines_changed", lines as f64, m, "Σ added + deleted"),
        mk("test_lines", test_lines as f64, m, "Σ redaka na testnim putanjama"),
        mk("test_share", r3(test_lines as f64 / (lines.max(1)) as f64), m, "test_lines / lines_changed"),
        mk("deploys", input.deliveries.iter().filter(|d| d.deploy).count() as f64, m, "isporuke s 🚀/deploy"),
        mk("debugging_commits", kind_count(WorkKind::Debugging), m, "effective_kind == debugging"),
        mk("debugging_share", r3(kind_count(WorkKind::Debugging) / commits.max(1.0)), m, "debugging_commits / commits"),
        mk("docs_share", r3(kind_count(WorkKind::Documentation) / commits.max(1.0)), m, "documentation / commits"),
        mk("ci_fixes", input.commits.iter().filter(|c| p.ci_fix.is_match(&c.subject.to_lowercase())).count() as f64, m, "subject ~ ci_fix_pattern"),
        mk("owner_driven_deliveries", input.deliveries.iter().filter(|d| d.title.to_lowercase().contains(&profile.owner_name)).count() as f64, m, "naslov sadrži owner_name"),
        mk("closed_phases_in_range", closed.iter().filter(|ph| ph.to.as_deref().is_some_and(|t| t >= profile.since.as_str())).count() as f64, m, "zatvorene faze s to >= since"),
        mk("closed_phase_avg_days", r1(per(closed.iter().filter_map(|ph| ph.days).sum::<i64>() as f64, closed.len() as f64)), m, "mean(days) zatvorenih faza"),
    ]
}
```
Provjera brojki testa: sati = 0,5 (a) + 1,0 (b, razmak 1 h) + 1,0 (c) = 2,5; 3/2,5 = 1,2 ✓; debugging = `fix:` + `ci:` = 2 → 0,667 ✓; docs = 1 → 0,333 ✓.

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git add -A && git commit -m "M1/11: faze i 18 pokazatelja -- zatvorene se broje iz gita, zaokruzivanje kao u tablici"`

---
### Task 12: Čistoća dokumentacije (tok DOCS+PRAVILA)

**Files:** Modify `crates/sokratis-core/src/docs.rs`

**Interfaces:**
- Consumes: `DocFile { path, content, last_change_time }`, `Profile` (docs_dir, docs_index, plan_dir, plan_dir_ignore, product_dir, key_file, key_file_budget_bytes, changelog_path, diary_path, docs_lag_warn_days, docs_weights), `Patterns.paused`.
- Produces: `docs_health(files, last_code_commit_time, profile, p) -> Option<DocsHealth>`; `None` ako nijedna datoteka nije pod `docs_dir/`. `Finding.check` ∈ {`dead-link`, `not-indexed`, `multiple-active-plans`, `no-active-plan`, `diary-in-definition`, `docs-lag`, `key-file-budget`}. `score = 100 − Σ težina`, min 0. `lag_days` = dani između `last_code_commit_time` i max(`last_change_time` dnevnika, changeloga), samo ako je pozitivan.

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Profile;

    fn f(path: &str, content: &str, t: Option<i64>) -> DocFile {
        DocFile { path: path.into(), content: content.into(), last_change_time: t }
    }
    const DAY: i64 = 86_400;

    #[test]
    fn no_docs_dir_means_none_not_zero() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        assert!(docs_health(&[f("README.md", "# x", None)], None, &p, &pat).is_none());
    }

    #[test]
    fn findings_with_locations_and_score() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "[PRD](./product/PRD.md)\n[nema](./plan/NEMA.md)\nplan/A.md plan/B.md plan/ROADMAP.md records/PROGRESS.md records/CHANGELOG.md", None),
            f("docs/product/PRD.md", "2026-01-01 2026-01-02 2026-01-03 2026-01-04", None),
            f("docs/plan/A.md", "**Status:** AKTIVAN", None),
            f("docs/plan/B.md", "**Status:** AKTIVAN", None),
            f("docs/plan/ROADMAP.md", "", None),
            f("docs/records/DUH.md", "", None),
            f("docs/records/PROGRESS.md", "", Some(10 * DAY)),
            f("docs/records/CHANGELOG.md", "", Some(9 * DAY)),
            f("CLAUDE.md", "x", None),
        ];
        let h = docs_health(&files, Some(13 * DAY), &p, &pat).unwrap();
        let checks: Vec<(&str, &str)> = h.findings.iter().map(|x| (x.check.as_str(), x.path.as_str())).collect();
        assert!(checks.contains(&("dead-link", "docs/README.md")));
        assert!(checks.contains(&("not-indexed", "docs/records/DUH.md")));
        assert!(checks.contains(&("multiple-active-plans", "docs/plan")));
        assert!(checks.contains(&("diary-in-definition", "docs/product/PRD.md")));
        assert!(checks.contains(&("docs-lag", "docs/records/PROGRESS.md")));
        assert!(!checks.iter().any(|(c, _)| *c == "key-file-budget"));
        let dead = h.findings.iter().find(|x| x.check == "dead-link").unwrap();
        assert_eq!(dead.line, Some(2));
        assert_eq!(h.lag_days, Some(3));
        assert_eq!(h.score, 100 - 5 - 3 - 15 - 5 - 10);
    }

    #[test]
    fn paused_plan_is_not_active_and_budget_is_checked() {
        let mut p = Profile::default();
        p.key_file_budget_bytes = 3;
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "plan/A.md plan/B.md", None),
            f("docs/plan/A.md", "**Status:** ⏸️ PAUZIRAN", None),
            f("docs/plan/B.md", "aktivan", None),
            f("CLAUDE.md", "abcd\r\n", None),
        ];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        assert!(!h.findings.iter().any(|x| x.check.ends_with("active-plan") || x.check.ends_with("active-plans")));
        let b = h.findings.iter().find(|x| x.check == "key-file-budget").unwrap();
        assert!(b.message.contains("5 B"), "mjeri se bez CR: {}", b.message);
        assert_eq!(h.lag_days, None);
    }
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-core docs`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/12 — čistoća dokumentacije)
//! Svaka provjera je mala privatna funkcija koja PUNI `Vec<Finding>` kroz `&mut` — jedan
//! vlasnik vektora (ova funkcija), više posudbi u nizu, nikad istodobno. `HashSet<&str>` nad
//! putanjama daje O(1) provjeru „postoji li cilj poveznice" bez kopiranja stringova.
use crate::{DocFile, DocsHealth, Finding, Patterns, Profile};
use regex::Regex;
use std::collections::HashSet;

fn finding(check: &str, path: &str, line: Option<usize>, message: String) -> Finding {
    Finding { check: check.into(), path: path.into(), line, message }
}

/// `dir/../x.md` → normalizirano s `/`; vraća None ako izlazi iznad korijena.
fn resolve(from_file: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = from_file.rsplitn(2, '/').nth(1).unwrap_or("").split('/').filter(|s| !s.is_empty()).collect();
    for seg in link.split('/') {
        match seg {
            "" | "." => {}
            ".." => { parts.pop()?; }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

fn dead_links(files: &[DocFile], known: &HashSet<&str>, out: &mut Vec<Finding>) {
    let re = Regex::new(r"\]\(([^)\s]+\.md)(#[^)\s]*)?\)").expect("regex konstanta");
    for f in files {
        let mut in_fence = false; // poveznica unutar ``` bloka je primjer, ne poveznica
        for (i, line) in f.content.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if in_fence {
                continue;
            }
            for c in re.captures_iter(line) {
                let link = &c[1];
                if link.contains("://") { continue; }
                let ok = resolve(&f.path, link).is_some_and(|t| known.contains(t.as_str()));
                if !ok {
                    out.push(finding("dead-link", &f.path, Some(i + 1), format!("poveznica na {link} ne postoji")));
                }
            }
        }
    }
}

fn not_indexed(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    let Some(index) = files.iter().find(|f| f.path == p.docs_index) else { return };
    let prefix = format!("{}/", p.docs_dir);
    for f in files.iter().filter(|f| f.path.starts_with(&prefix) && f.path != p.docs_index) {
        let rel = &f.path[prefix.len()..];
        if !index.content.contains(rel) {
            out.push(finding("not-indexed", &f.path, None, format!("{rel} nije naveden u {}", p.docs_index)));
        }
    }
}

fn active_plans(files: &[DocFile], p: &Profile, pat: &Patterns, out: &mut Vec<Finding>) {
    let prefix = format!("{}/", p.plan_dir);
    let specs: Vec<&DocFile> = files.iter()
        .filter(|f| f.path.starts_with(&prefix) && !p.plan_dir_ignore.iter().any(|i| f.path.ends_with(i.as_str())))
        .collect();
    if specs.is_empty() { return; }
    let active: Vec<&str> = specs.iter().filter(|f| !pat.paused.is_match(&f.content)).map(|f| f.path.as_str()).collect();
    if active.len() > 1 {
        out.push(finding("multiple-active-plans", &p.plan_dir, None, format!("više aktivnih planova: {}", active.join(", "))));
    } else if active.is_empty() {
        out.push(finding("no-active-plan", &p.plan_dir, None, "svi planovi su PAUZIRANI; točno jedan mora nositi prvenstvo".into()));
    }
}

fn diary_in_definition(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    let re = Regex::new(r"\b20\d\d-\d\d-\d\d\b").expect("regex konstanta");
    let prefix = format!("{}/", p.product_dir);
    for f in files.iter().filter(|f| f.path.starts_with(&prefix)) {
        let n = re.find_iter(&f.content).count();
        if n > 3 {
            out.push(finding("diary-in-definition", &f.path, None, format!("{n} datuma u definiciji; kronologija ide u records/")));
        }
    }
}

fn key_file_budget(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    if let Some(f) = files.iter().find(|f| f.path == p.key_file) {
        let bytes = f.content.replace('\r', "").len() as u64;
        if bytes > p.key_file_budget_bytes {
            out.push(finding("key-file-budget", &f.path, None, format!("{bytes} B > budžet {} B", p.key_file_budget_bytes)));
        }
    }
}

fn lag(files: &[DocFile], last_code: Option<i64>, p: &Profile, out: &mut Vec<Finding>) -> Option<i64> {
    let code = last_code?;
    let docs_time = files.iter()
        .filter(|f| f.path == p.diary_path || f.path == p.changelog_path)
        .filter_map(|f| f.last_change_time)
        .max()?;
    let days = (code - docs_time) / 86_400;
    if days <= 0 { return Some(0); }
    if days >= p.docs_lag_warn_days {
        out.push(finding("docs-lag", &p.diary_path, None, format!("dnevnik i changelog kasne {days} dana za zadnjim commitom koda")));
    }
    Some(days)
}

pub fn docs_health(files: &[DocFile], last_code_commit_time: Option<i64>, profile: &Profile, p: &Patterns) -> Option<DocsHealth> {
    let prefix = format!("{}/", profile.docs_dir);
    if !files.iter().any(|f| f.path.starts_with(&prefix)) {
        return None;
    }
    let known: HashSet<&str> = files.iter().map(|f| f.path.as_str()).collect();
    let mut findings = Vec::new();
    dead_links(files, &known, &mut findings);
    not_indexed(files, profile, &mut findings);
    active_plans(files, profile, p, &mut findings);
    diary_in_definition(files, profile, &mut findings);
    key_file_budget(files, profile, &mut findings);
    let lag_days = lag(files, last_code_commit_time, profile, &mut findings);
    let w = &profile.docs_weights;
    let penalty: u32 = findings.iter().map(|f| match f.check.as_str() {
        "dead-link" => w.dead_link, "not-indexed" => w.not_indexed, "multiple-active-plans" => w.multiple_plans,
        "no-active-plan" => w.no_active_plan, "diary-in-definition" => w.diary_in_definition, "docs-lag" => w.lag,
        "key-file-budget" => w.key_file_budget, _ => 0,
    } as u32).sum();
    Some(DocsHealth { score: 100u32.saturating_sub(penalty) as u8, findings, lag_days })
}
```
⚠️ `expect("regex konstanta")` na literalu je dopušten (ne može pasti); ako recenzent traži, prebaci regexe u `Patterns`.

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/12: cistoca dokumentacije -- sest provjera s mjestom nalaza, None bez docs/, ocjena po tezinama"`

---

### Task 13: Pravilo `unmerged-branches` (tok DOCS+PRAVILA)

**Files:** Modify `crates/sokratis-core/src/rules/unmerged_branches.rs`

**Interfaces:** Consumes `Context { profile, now, branches }`, `Rule` trait (T1). Produces: jedan `Signal` ili ništa. Grane = `!merged && name != profile.default_branch`. `age_days = (now − last_commit_time) / 86400`. Warn ako je bar jedna `>= unmerged_warn_days`; Alert ako bar jedna `>= unmerged_alert_days` **ili** broj takvih grana `> unmerged_alert_count`. Bez grana koje prelaze Warn → nema signala. `evidence` = po grani `"{name}: {age} dana, {ahead} commita ispred {default}"`, `since` = min `last_commit_time` među prijavljenima, `title_key = "signal.unmerged_branches"`.

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BranchInfo, Profile, Severity};
    const DAY: i64 = 86_400;

    fn ctx(branches: Vec<BranchInfo>) -> Context {
        Context { profile: Profile::default(), now: 100 * DAY, commits: vec![], branches, docs: vec![], last_code_commit: None }
    }
    fn b(name: &str, age_days: i64, ahead: u32, merged: bool) -> BranchInfo {
        BranchInfo { name: name.into(), last_commit_time: 100 * DAY - age_days * DAY, ahead_of_default: ahead, merged }
    }

    #[test]
    fn young_or_merged_branches_are_silent() {
        assert!(UnmergedBranches.evaluate(&ctx(vec![b("main", 0, 0, true), b("feat/a", 2, 3, false), b("old", 30, 1, true)])).is_empty());
    }

    #[test]
    fn warn_then_alert_with_evidence() {
        let s = UnmergedBranches.evaluate(&ctx(vec![b("feat/a", 6, 3, false)]));
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].severity, Severity::Warn);
        assert_eq!(s[0].evidence, vec!["feat/a: 6 dana, 3 commita ispred main"]);
        assert_eq!(s[0].since, Some(94 * DAY));
        let s = UnmergedBranches.evaluate(&ctx(vec![b("feat/a", 11, 1, false)]));
        assert_eq!(s[0].severity, Severity::Alert);
        let many = UnmergedBranches.evaluate(&ctx(vec![b("a", 6, 1, false), b("b", 6, 1, false), b("c", 6, 1, false), b("d", 6, 1, false)]));
        assert_eq!((many[0].severity, many[0].evidence.len()), (Severity::Alert, 4));
    }
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-core unmerged`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/13 — pravilo nespojenih grana)
//! `impl Rule for UnmergedBranches` je ugovor iz `rules/mod.rs`; pravilo nema stanja (unit struct)
//! pa se konstruira golim imenom. `Severity` derivira `Ord` → `max()` bira težu težinu bez `if`.
use super::Rule;
use crate::{Context, Severity, Signal};

pub struct UnmergedBranches;

impl Rule for UnmergedBranches {
    fn id(&self) -> &'static str { "unmerged-branches" }

    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let p = &ctx.profile;
        let mut offenders: Vec<(&crate::BranchInfo, i64)> = ctx.branches.iter()
            .filter(|b| !b.merged && b.name != p.default_branch)
            .map(|b| (b, (ctx.now - b.last_commit_time) / 86_400))
            .filter(|(_, age)| *age >= p.unmerged_warn_days)
            .collect();
        if offenders.is_empty() {
            return vec![];
        }
        offenders.sort_by_key(|(b, _)| b.last_commit_time);
        let oldest = offenders.iter().map(|(_, age)| *age).max().unwrap_or(0);
        let severity = if oldest >= p.unmerged_alert_days || offenders.len() > p.unmerged_alert_count { Severity::Alert } else { Severity::Warn };
        vec![Signal {
            rule: self.id().into(),
            severity,
            title_key: "signal.unmerged_branches".into(),
            evidence: offenders.iter().map(|(b, age)| format!("{}: {age} dana, {} commita ispred {}", b.name, b.ahead_of_default, p.default_branch)).collect(),
            since: offenders.first().map(|(b, _)| b.last_commit_time),
        }]
    }
}
```

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/13: pravilo unmerged-branches -- Warn/Alert po starosti i broju, dokaz po grani"`

---

### Task 14: Pravilo `docs-lag` (tok DOCS+PRAVILA)

**Files:** Modify `crates/sokratis-core/src/rules/docs_lag.rs`

**Interfaces:** Consumes `Context { profile, docs, last_code_commit }`. Produces `Signal` ako `lag_days >= docs_lag_warn_days` (Alert od `docs_lag_alert_days`); `lag_days = (last_code_commit.author_time − max(last_change_time dnevnika/changeloga)) / 86400`. Bez commita koda ili bez dnevnika → ništa. `evidence = ["zadnji commit koda: {sha} {date}", "zadnja promjena dnevnika: prije {lag} dana"]`, `since = Some(docs_time)`, `title_key = "signal.docs_lag"`.

- [ ] **Step 1: test koji pada**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Commit, DocFile, Profile, Severity};
    const DAY: i64 = 86_400;

    fn ctx(code_day: Option<i64>, diary_day: Option<i64>) -> Context {
        let last_code_commit = code_day.map(|d| Commit { sha: "abc1234".into(), author_time: d * DAY, commit_time: d * DAY, date: "2026-09-13".into(), commit_date: "2026-09-13".into(), subject: "x".into(), files: vec![] });
        let docs = diary_day.map(|d| vec![DocFile { path: "docs/records/PROGRESS.md".into(), content: String::new(), last_change_time: Some(d * DAY) }]).unwrap_or_default();
        Context { profile: Profile::default(), now: 0, commits: vec![], branches: vec![], docs, last_code_commit }
    }

    #[test]
    fn silent_without_code_or_diary_or_when_fresh() {
        assert!(DocsLag.evaluate(&ctx(None, Some(1))).is_empty());
        assert!(DocsLag.evaluate(&ctx(Some(5), None)).is_empty());
        assert!(DocsLag.evaluate(&ctx(Some(5), Some(4))).is_empty(), "1 dan < prag 2");
    }

    #[test]
    fn warn_at_two_days_alert_at_five() {
        let s = DocsLag.evaluate(&ctx(Some(5), Some(3)));
        assert_eq!((s.len(), s[0].severity), (1, Severity::Warn));
        assert_eq!(s[0].evidence, vec!["zadnji commit koda: abc1234 2026-09-13", "zadnja promjena dnevnika: prije 2 dana"]);
        assert_eq!(s[0].since, Some(3 * DAY));
        assert_eq!(DocsLag.evaluate(&ctx(Some(10), Some(5)))[0].severity, Severity::Alert);
    }
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-core docs_lag`

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/14 — pravilo kašnjenja docs-a)
//! Rani `return vec![]` kroz `let … else`: tri preduvjeta (commit koda, dnevnik, prag) čitaju se
//! odozgo prema dolje kao rečenice, bez piramide ugniježđenih `if`-ova.
use super::Rule;
use crate::{Context, Severity, Signal};

pub struct DocsLag;

impl Rule for DocsLag {
    fn id(&self) -> &'static str { "docs-lag" }

    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let p = &ctx.profile;
        let Some(code) = ctx.last_code_commit.as_ref() else { return vec![] };
        let Some(docs_time) = ctx.docs.iter()
            .filter(|f| f.path == p.diary_path || f.path == p.changelog_path)
            .filter_map(|f| f.last_change_time)
            .max()
        else { return vec![] };
        let lag = (code.author_time - docs_time) / 86_400;
        if lag < p.docs_lag_warn_days {
            return vec![];
        }
        let severity = if lag >= p.docs_lag_alert_days { Severity::Alert } else { Severity::Warn };
        vec![Signal {
            rule: self.id().into(),
            severity,
            title_key: "signal.docs_lag".into(),
            evidence: vec![format!("zadnji commit koda: {} {}", code.sha, code.date), format!("zadnja promjena dnevnika: prije {lag} dana")],
            since: Some(docs_time),
        }]
    }
}
```

- [ ] **Step 4: prolazi** · **Step 5: commit** — `git commit -am "M1/14: pravilo docs-lag -- dnevnik iza koda, Warn 2 dana, Alert 5"`

---
### Task 15: `GitCli` — log, toplevel, common_dir, grane (tok IO)

**Files:** Modify `crates/sokratis-io/src/git.rs`; Create `crates/sokratis-io/tests/common/mod.rs`, `crates/sokratis-io/tests/git_cli.rs`

**Interfaces:**
- Produces `GitCli::run(&self, args: &[&str]) -> Result<String, IoError>` (privatno) i implementaciju `GitSource::{log, toplevel, common_dir, branch_exists, current_branch}`.
- `log(branch, since)` vrti: `git log <branch> --since=<since> --reverse --date=format:%Y-%m-%d --format=@@%h|%at|%ct|%ad|%cd|%s --numstat` — **isti format kao T2 fixture**.

- [ ] **Step 1: pomoćnik za privremeni repo (dijele ga svi io-testovi)**

`tests/common/mod.rs`:
```rust
//! Privremeni git-repo s FIKSNIM datumima: `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE` čine test
//! determinističkim, a `tempfile::TempDir` briše mapu kad test završi (RAII — `Drop`).
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

pub struct Repo { pub dir: TempDir }

impl Repo {
    pub fn path(&self) -> &Path { self.dir.path() }

    pub fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git").arg("-C").arg(self.path()).args(args).output().expect("git");
        assert!(out.status.success(), "git {:?}: {}", args, String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    pub fn init() -> Repo {
        let dir = tempfile::tempdir().expect("tempdir");
        let r = Repo { dir };
        r.git(&["init", "-q", "-b", "main"]);
        r.git(&["config", "user.email", "t@t"]);
        r.git(&["config", "user.name", "T"]);
        r.git(&["config", "commit.gpgsign", "false"]);
        r
    }

    /// Commit s datumom autora `a` i commita `c` (ISO 8601 s pomakom), datoteka `path` dobiva `content`.
    pub fn commit(&self, path: &str, content: &str, msg: &str, a: &str, c: &str) -> String {
        let full: PathBuf = self.path().join(path);
        std::fs::create_dir_all(full.parent().expect("parent")).expect("mkdir");
        std::fs::write(&full, content).expect("write");
        self.git(&["add", "-A"]);
        let out = Command::new("git").arg("-C").arg(self.path())
            .env("GIT_AUTHOR_DATE", a).env("GIT_COMMITTER_DATE", c)
            .args(["commit", "-q", "-m", msg]).output().expect("git commit");
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        self.git(&["rev-parse", "--short", "HEAD"])
    }
}
```

- [ ] **Step 2: test koji pada**

`tests/git_cli.rs`:
```rust
mod common;
use common::Repo;
use sokratis_io::{GitCli, GitSource, IoError};

#[test]
fn log_has_fixture_format_and_since_filters_by_commit_date() {
    let r = Repo::init();
    let old = r.commit("a.txt", "1", "prvi", "2026-08-01T10:00:00+02:00", "2026-08-01T10:00:00+02:00");
    let cp = r.commit("b.txt", "2\n3\n", "cherry", "2026-09-06T05:16:40+02:00", "2026-09-08T21:19:43+02:00");
    let g = GitCli::new(r.path());
    let log = g.log("main", "2026-08-29").unwrap();
    assert!(!log.contains(&old), "commit s committer-datumom prije since otpada");
    let head = log.lines().find(|l| l.starts_with("@@")).unwrap();
    assert!(head.starts_with(&format!("@@{cp}|1788664600|1788817183|2026-09-06|2026-09-08|cherry")), "{head}");
    assert!(log.contains("2\t0\tb.txt"));
}

#[test]
fn toplevel_common_dir_and_branches() {
    let r = Repo::init();
    r.commit("a.txt", "1", "prvi", "2026-09-01T10:00:00+02:00", "2026-09-01T10:00:00+02:00");
    let g = GitCli::new(r.path());
    assert_eq!(g.current_branch().unwrap(), "main");
    assert!(g.branch_exists("main").unwrap() && !g.branch_exists("nema").unwrap());
    assert_eq!(g.toplevel().unwrap().canonicalize().unwrap(), r.path().canonicalize().unwrap());
    assert!(g.common_dir().unwrap().ends_with(".git"));
}

#[test]
fn not_a_repo_is_a_typed_error() {
    let dir = tempfile::tempdir().unwrap();
    let err = GitCli::new(dir.path()).toplevel().unwrap_err();
    assert!(matches!(err, IoError::NotARepo(_)), "{err}");
}
```

- [ ] **Step 3: pada** — `cargo test -p sokratis-io git_cli` → FAIL `not yet implemented`

- [ ] **Step 4: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/15 — git kroz proces)
//! `std::process::Command` gradi poziv bez shella (nema quotinga, nema injekcije). `output()`
//! vraća `Output { status, stdout, stderr }`; `String::from_utf8_lossy` toleriše tuđi ne-UTF-8
//! bajt umjesto da sruši cijeli izvještaj. Greška se MAPIRA u `IoError` po uzroku (`map_err`).
use crate::IoError;
use sokratis_core::BranchInfo;
use std::path::PathBuf;
use std::process::Command;

pub trait GitSource {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError>;
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError>;
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError>;
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError>;
    fn common_dir(&self) -> Result<PathBuf, IoError>;
    fn toplevel(&self) -> Result<PathBuf, IoError>;
    fn branch_exists(&self, name: &str) -> Result<bool, IoError>;
    fn current_branch(&self) -> Result<String, IoError>;
}

pub struct GitCli { pub repo: PathBuf }

impl GitCli {
    pub fn new(repo: impl Into<PathBuf>) -> Self { Self { repo: repo.into() } }

    fn run(&self, args: &[&str]) -> Result<String, IoError> {
        let out = Command::new("git").arg("-C").arg(&self.repo).args(args).output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound { IoError::GitMissing } else { IoError::Io(e) }
        })?;
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            if stderr.contains("not a git repository") {
                return Err(IoError::NotARepo(self.repo.clone()));
            }
            return Err(IoError::Git { cmd: args.join(" "), stderr });
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    fn path_of(&self, args: &[&str]) -> Result<PathBuf, IoError> {
        let s = self.run(args)?;
        let rel = PathBuf::from(s.trim());
        Ok(if rel.is_absolute() { rel } else { self.repo.join(rel) })
    }
}

impl GitSource for GitCli {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError> {
        let since_arg = format!("--since={since}");
        self.run(&[
            "log", branch, &since_arg, "--reverse", "--date=format:%Y-%m-%d",
            "--format=@@%h|%at|%ct|%ad|%cd|%s", "--numstat",
        ])
    }
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError> { let _ = default_branch; todo!("cigla M1/16") }
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError> { todo!("cigla M1/16") }
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError> { let _ = path; todo!("cigla M1/16") }
    fn common_dir(&self) -> Result<PathBuf, IoError> { self.path_of(&["rev-parse", "--git-common-dir"]) }
    fn toplevel(&self) -> Result<PathBuf, IoError> { self.path_of(&["rev-parse", "--show-toplevel"]) }
    fn branch_exists(&self, name: &str) -> Result<bool, IoError> {
        let refname = format!("refs/heads/{name}");
        match self.run(&["show-ref", "--verify", "--quiet", &refname]) {
            Ok(_) => Ok(true),
            Err(IoError::Git { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }
    fn current_branch(&self) -> Result<String, IoError> {
        Ok(self.run(&["branch", "--show-current"])?.trim().to_string())
    }
}
```
Provjera unix-vremena u testu: `2026-09-06T05:16:40+02:00` = 1788664600; `2026-09-08T21:19:43+02:00` = 1788817183 (09-08 = 1788825600 − 86400·… → 2026-09-08 00:00Z = 1788825600; 19:19:43Z = 69583; 1788825600 + 69583 − 86400 = 1788808783 ⚠️). **Izračunaj u testu ovako, ne napamet:** `date -u -d "2026-09-08T21:19:43+02:00" +%s` u Git Bashu, i upiši dobiveni broj u test prije prvog pokretanja.

- [ ] **Step 5: prolazi** — `cargo test -p sokratis-io git_cli` → 3 PASS. **Step 6: commit** — `git add -A && git commit -m "M1/15: GitCli -- git kroz proces, log u formatu fixturea, NotARepo/GitMissing tipizirano"`

---

### Task 16: `GitCli` — grane, radna stabla, zadnja promjena putanje (tok IO)

**Files:** Modify `crates/sokratis-io/src/git.rs`; Modify `crates/sokratis-io/tests/git_cli.rs`

**Interfaces:**
- `branches(default)`: `for-each-ref refs/heads --format=%(refname:short)|%(authordate:unix)`; `merged` iz `branch --merged <default> --format=%(refname:short)`; `ahead_of_default` iz `rev-list --count <default>..<name>`; zadana grana ima `merged = true`, `ahead = 0`.
- `worktrees()`: retci `worktree <put>` iz `worktree list --porcelain`.
- `last_change(path)`: `log -1 --format=%at -- <path>` → `None` ako prazno.

- [ ] **Step 1: testovi koji padaju (dodaj u `tests/git_cli.rs`)**

```rust
#[test]
fn branches_report_age_ahead_and_merged() {
    let r = Repo::init();
    r.commit("a.txt", "1", "prvi", "2026-09-01T10:00:00+02:00", "2026-09-01T10:00:00+02:00");
    r.git(&["checkout", "-q", "-b", "feat/x"]);
    r.commit("b.txt", "2", "F1/1 cigla", "2026-09-03T10:00:00+02:00", "2026-09-03T10:00:00+02:00");
    r.commit("c.txt", "3", "F1/2 cigla", "2026-09-04T10:00:00+02:00", "2026-09-04T10:00:00+02:00");
    r.git(&["checkout", "-q", "-b", "merged/y", "main"]);
    r.git(&["checkout", "-q", "main"]);
    let g = GitCli::new(r.path());
    let mut b = g.branches("main").unwrap();
    b.sort_by(|x, y| x.name.cmp(&y.name));
    let names: Vec<&str> = b.iter().map(|x| x.name.as_str()).collect();
    assert_eq!(names, ["feat/x", "main", "merged/y"]);
    let fx = &b[0];
    assert_eq!((fx.ahead_of_default, fx.merged), (2, false));
    assert_eq!(fx.last_commit_time, 1788854400 + 8 * 3600, "2026-09-04T10:00+02:00");
    assert_eq!((b[1].ahead_of_default, b[1].merged), (0, true));
    assert!(b[2].merged, "grana na istom commitu kao main je spojena");
}

#[test]
fn worktrees_share_common_dir() {
    let r = Repo::init();
    r.commit("a.txt", "1", "prvi", "2026-09-01T10:00:00+02:00", "2026-09-01T10:00:00+02:00");
    let wt = tempfile::tempdir().unwrap();
    let wt_path = wt.path().join("stablo");
    r.git(&["worktree", "add", "-q", wt_path.to_str().unwrap(), "-b", "feat/wt"]);
    let g = GitCli::new(r.path());
    let list = g.worktrees().unwrap();
    assert_eq!(list.len(), 2);
    let a = GitCli::new(r.path()).common_dir().unwrap().canonicalize().unwrap();
    let b = GitCli::new(&wt_path).common_dir().unwrap().canonicalize().unwrap();
    assert_eq!(a, b, "isti projekt");
    r.git(&["worktree", "remove", "--force", wt_path.to_str().unwrap()]);
}

#[test]
fn last_change_of_path() {
    let r = Repo::init();
    r.commit("docs/PROGRESS.md", "1", "docs", "2026-09-02T10:00:00+02:00", "2026-09-02T10:00:00+02:00");
    r.commit("js/a.js", "1", "kod", "2026-09-05T10:00:00+02:00", "2026-09-05T10:00:00+02:00");
    let g = GitCli::new(r.path());
    assert_eq!(g.last_change("docs/PROGRESS.md").unwrap(), Some(1788681600 + 8 * 3600));
    assert_eq!(g.last_change("nema.md").unwrap(), None);
}
```
⚠️ Unix-vremena u tvrdnjama izračunaj naredbom (`date -u -d "…" +%s`) i upiši prije pokretanja; brojevi gore su smjernica, ne istina.

- [ ] **Step 2: pada** · **Step 3: implementacija (zamijeni tri `todo!` u `impl GitSource for GitCli`)**

```rust
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError> {
        let merged: std::collections::HashSet<String> = self
            .run(&["branch", "--merged", default_branch, "--format=%(refname:short)"])?
            .lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
        let mut out = Vec::new();
        for line in self.run(&["for-each-ref", "refs/heads", "--format=%(refname:short)|%(authordate:unix)"])?.lines() {
            let Some((name, t)) = line.trim().split_once('|') else { continue };
            let last_commit_time: i64 = t.parse().unwrap_or(0);
            let ahead_of_default = if name == default_branch {
                0
            } else {
                let range = format!("{default_branch}..{name}");
                self.run(&["rev-list", "--count", &range])?.trim().parse().unwrap_or(0)
            };
            out.push(BranchInfo { name: name.to_string(), last_commit_time, ahead_of_default, merged: name == default_branch || merged.contains(name) });
        }
        Ok(out)
    }

    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError> {
        Ok(self.run(&["worktree", "list", "--porcelain"])?
            .lines()
            .filter_map(|l| l.strip_prefix("worktree "))
            .map(|p| PathBuf::from(p.trim()))
            .collect())
    }

    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError> {
        let s = self.run(&["log", "-1", "--format=%at", "--", path])?;
        Ok(s.trim().parse().ok())
    }
```
`unwrap_or(0)` na `parse` je svjesna odluka: git nikad ne ispisuje ne-broj u `%(authordate:unix)`; zapiši to u zaglavlje.

- [ ] **Step 4: prolazi** — `cargo test -p sokratis-io` → 6 PASS. **Step 5: commit** — `git commit -am "M1/16: GitCli -- grane sa starosti i ahead, radna stabla, zadnja promjena putanje"`

---

### Task 17: `Project` — profil, ručni podaci, docs, `ReportInput` (tok IO)

**Files:** Modify `crates/sokratis-io/src/project.rs`; Create `crates/sokratis-io/tests/project.rs`

**Interfaces:**
- Consumes `GitCli` (T15/T16), `Profile`, `ReportInput`, `DocFile`, `Vision`, `WorkKind`.
- Produces `Project::open(path)`: greška `NotARepo` ako nije repo; `root = toplevel`, `common_dir`; profil iz `<root>/.sokratis/profile.json` ako postoji (nepoznato polje → `IoError::Profile`), inače `Profile::default()`.
- `overrides()`: `<root>/.sokratis/overrides.json` = `{ "sha": "debugging", … }`; nema datoteke → prazno.
- `visions()`: `<root>/.sokratis/visions.json` = `[Vision, …]`; nema → prazno.
- `docs()`: svi `*.md` pod `<root>/<docs_dir>/**` + `*.md` u korijenu; putanje relativne s `/`; preskače `node_modules`, `.git`, `target`; `last_change_time` iz gita.
- `input(since)`: `branch` = `profile.default_branch` ako postoji, inače `current_branch()`; `git_log = log(branch, profile.log_since())`; `diary`/`plan` = sadržaj datoteka ako postoje; `branches`; `now` = `SystemTime`; `today` = `chrono::Local::now().format("%Y-%m-%d")`; `since` = argument ili `profile.since`.

- [ ] **Step 1: test koji pada**

`tests/project.rs`:
```rust
mod common;
use common::Repo;
use sokratis_core::WorkKind;
use sokratis_io::{IoError, Project};

fn write(r: &Repo, rel: &str, content: &str) {
    let p = r.path().join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, content).unwrap();
}

#[test]
fn open_reads_profile_manual_data_and_docs() {
    let r = Repo::init();
    r.commit("docs/records/PROGRESS.md", "## 2026-09-01 (X) — prvi unos\n", "docs", "2026-09-01T10:00:00+02:00", "2026-09-01T10:00:00+02:00");
    r.commit("js/a.js", "1", "F1/1 kod", "2026-09-02T10:00:00+02:00", "2026-09-02T10:00:00+02:00");
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01", "owner_name": "test" }"#);
    write(&r, ".sokratis/overrides.json", r#"{ "abc1234": "polish" }"#);
    write(&r, ".sokratis/visions.json", r#"[{"title":"V","source":"s","state":"idea","percent":null,"note":""}]"#);
    write(&r, "README.md", "# r");
    let p = Project::open(r.path()).unwrap();
    assert_eq!((p.profile.since.as_str(), p.profile.owner_name.as_str(), p.profile.default_branch.as_str()), ("2026-09-01", "test", "main"));
    assert_eq!(p.overrides().unwrap().get("abc1234"), Some(&WorkKind::Polish));
    assert_eq!(p.visions().unwrap()[0].title, "V");
    let docs = p.docs().unwrap();
    let mut paths: Vec<&str> = docs.iter().map(|d| d.path.as_str()).collect();
    paths.sort();
    assert_eq!(paths, ["README.md", "docs/records/PROGRESS.md"]);
    let diary = docs.iter().find(|d| d.path == "docs/records/PROGRESS.md").unwrap();
    assert!(diary.last_change_time.is_some() && diary.content.starts_with("## 2026-09-01"));
    assert_eq!(docs.iter().find(|d| d.path == "README.md").unwrap().last_change_time, None, "necommitana datoteka");
    let input = p.input(None).unwrap();
    assert_eq!((input.branch.as_str(), input.since.as_str()), ("main", "2026-09-01"));
    assert!(input.git_log.contains("F1/1 kod"));
    assert!(input.diary.as_deref().unwrap().starts_with("## 2026-09-01"));
    assert!(input.plan.is_none());
    assert_eq!(input.today.len(), 10);
    assert_eq!(input.branches.len(), 1);
}

#[test]
fn unknown_profile_field_is_an_error_and_missing_files_default() {
    let r = Repo::init();
    r.commit("a.txt", "1", "prvi", "2026-09-01T10:00:00+02:00", "2026-09-01T10:00:00+02:00");
    let p = Project::open(r.path()).unwrap();
    assert!(p.overrides().unwrap().is_empty() && p.visions().unwrap().is_empty());
    write(&r, ".sokratis/profile.json", r#"{ "sinc": "x" }"#);
    assert!(matches!(Project::open(r.path()).unwrap_err(), IoError::Profile { .. }));
}
```

- [ ] **Step 2: pada** · **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/17 — projekt)
//! `Project` POSJEDUJE `GitCli` i `Profile`; metode posuđuju `&self`. `fs::read_to_string(..).ok()`
//! pretvara „nema datoteke" u `None` — a JSON koji POSTOJI, a ne valja, je greška s putanjom
//! (`map_err` + `IoError::Manual { path, source }`), jer tiho ignoriranje krivog JSON-a je laž.
//! Rekurzivni `walk` je obična funkcija koja puni `&mut Vec` — bez rekurzivnih zatvaranja.
use crate::{GitCli, GitSource, IoError};
use sokratis_core::{DocFile, Profile, ReportInput, Vision, WorkKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Project {
    pub root: PathBuf,
    pub common_dir: PathBuf,
    pub profile: Profile,
    pub git: GitCli,
}

const SKIP_DIRS: [&str; 3] = ["node_modules", ".git", "target"];

fn read_json_or<T: serde::de::DeserializeOwned>(path: &Path, fallback: T) -> Result<T, IoError> {
    match std::fs::read_to_string(path) {
        Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Manual { path: path.to_path_buf(), source }),
        Err(_) => Ok(fallback),
    }
}

fn walk(dir: &Path, root: &Path, out: &mut Vec<PathBuf>) -> Result<(), IoError> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if !SKIP_DIRS.contains(&name) {
                walk(&path, root, out)?;
            }
        } else if name.ends_with(".md") {
            out.push(path);
        }
    }
    Ok(())
}

impl Project {
    pub fn open(path: &Path) -> Result<Project, IoError> {
        let probe = GitCli::new(path);
        let root = probe.toplevel()?;
        let common_dir = probe.common_dir()?;
        let profile_path = root.join(".sokratis").join("profile.json");
        let profile = match std::fs::read_to_string(&profile_path) {
            Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Profile { path: profile_path.clone(), source })?,
            Err(_) => Profile::default(),
        };
        let git = GitCli::new(&root);
        Ok(Project { root, common_dir, profile, git })
    }

    pub fn overrides(&self) -> Result<HashMap<String, WorkKind>, IoError> {
        read_json_or(&self.root.join(".sokratis").join("overrides.json"), HashMap::new())
    }

    pub fn visions(&self) -> Result<Vec<Vision>, IoError> {
        read_json_or(&self.root.join(".sokratis").join("visions.json"), Vec::new())
    }

    pub fn docs(&self) -> Result<Vec<DocFile>, IoError> {
        let mut paths = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let p = entry?.path();
            if p.is_file() && p.extension().is_some_and(|e| e == "md") {
                paths.push(p);
            }
        }
        let docs_dir = self.root.join(&self.profile.docs_dir);
        if docs_dir.is_dir() {
            walk(&docs_dir, &self.root, &mut paths)?;
        }
        let mut out = Vec::new();
        for p in paths {
            let rel = p.strip_prefix(&self.root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
            out.push(DocFile { content: std::fs::read_to_string(&p)?, last_change_time: self.git.last_change(&rel)?, path: rel });
        }
        Ok(out)
    }

    pub fn input(&self, since: Option<&str>) -> Result<ReportInput, IoError> {
        let branch = if self.git.branch_exists(&self.profile.default_branch)? { self.profile.default_branch.clone() } else { self.git.current_branch()? };
        let read_opt = |rel: &str| std::fs::read_to_string(self.root.join(rel)).ok();
        Ok(ReportInput {
            git_log: self.git.log(&branch, &self.profile.log_since())?,
            diary: read_opt(&self.profile.diary_path),
            plan: read_opt(&self.profile.plan_path),
            docs: self.docs()?,
            branches: self.git.branches(&branch)?,
            overrides: self.overrides()?,
            visions: self.visions()?,
            now: SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0),
            today: chrono::Local::now().format("%Y-%m-%d").to_string(),
            since: since.map(str::to_string).unwrap_or_else(|| self.profile.since.clone()),
            branch,
        })
    }
}
```
Bilješka za RUST.md §2: ovdje pada odluka `chrono` (ne `time`) — jedini razlog je lokalni datum na Windowsu bez feature-gatea.

- [ ] **Step 4: prolazi** — `cargo test -p sokratis-io` → 8 PASS. **Step 5: commit** — `git add -A && git commit -m "M1/17: Project -- profil, overridei, vizije, docs s vremenom promjene, ReportInput"`

---

### Task 18: CLI naredbe i izlazni kodovi (tok CLI)

**Files:** Modify `crates/sokratis-cli/src/main.rs`

**Interfaces:**
- Consumes `Project::{open, input}` (T17), `build_report` (T20 — do tada `todo!()`: zato CLI test u T19 koristi `--json` nad repoom tek NAKON spajanja; u T18 se testira samo parsiranje argumenata i kod 3).
- Produces naredbe `report [path] [--since] [--json|--table]`, `docs [path] [--json]`, `signals [path] [--json]`; kod: `report`/`docs` 0; `signals` = max severity (Info→0, Warn→1, Alert→2); svaka greška → poruka na stderr + 3.

- [ ] **Step 1: test koji pada (`tests/cli.rs`, samo argumenti i kod 3)**

```rust
use std::process::Command;

fn bin() -> Command { Command::new(env!("CARGO_BIN_EXE_sokratis")) }

#[test]
fn no_args_prints_help_and_exits_2_by_clap() {
    let out = bin().output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("report"));
}

#[test]
fn not_a_repo_exits_3_with_message() {
    let dir = tempfile::tempdir().unwrap();
    let out = bin().args(["report", dir.path().to_str().unwrap(), "--json"]).output().unwrap();
    assert_eq!(out.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&out.stderr).contains("nije git repozitorij"));
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-cli` (prvi test pada jer kostur ne parsira argumente)

- [ ] **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI)
//! `clap` derive: struktura JE dokumentacija naredbe (`--help` se generira). `anyhow::Result<i32>`
//! u `run()`: svaka greška (io ili parse) ide `?`-om do `main`, koji je JEDINO mjesto s
//! `process::exit` — izlazni kod je ugovor prema preflightu.
mod table;

use clap::{Parser, Subcommand};
use sokratis_core::{build_report, Report, Severity};
use sokratis_io::Project;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sokratis", version, about = "Statistika rada, čistoća docs-a i signali smjera iz gita")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Cijeli izvještaj (tempo, vrste, pokazatelji, faze, vizije, docs, signali)
    Report { path: Option<PathBuf>, #[arg(long)] since: Option<String>, #[arg(long)] json: bool, #[arg(long)] table: bool },
    /// Samo čistoća dokumentacije
    Docs { path: Option<PathBuf>, #[arg(long)] json: bool },
    /// Samo signali; izlazni kod 0 nema · 1 Warn · 2 Alert
    Signals { path: Option<PathBuf>, #[arg(long)] json: bool },
}

fn report_for(path: Option<PathBuf>, since: Option<&str>) -> anyhow::Result<Report> {
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    let project = Project::open(&path)?;
    let input = project.input(since)?;
    Ok(build_report(&input, &project.profile)?)
}

fn run() -> anyhow::Result<i32> {
    match Cli::parse().cmd {
        Cmd::Report { path, since, json, table } => {
            let r = report_for(path, since.as_deref())?;
            if json || !table { println!("{}", serde_json::to_string_pretty(&r)?); } else { println!("{}", table::render(&r)); }
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
            if json { println!("{}", serde_json::to_string_pretty(&r.signals)?); } else { println!("{}", table::render_signals(&r.signals)); }
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
        Err(e) => { eprintln!("sokratis: {e:#}"); 3 }
    };
    std::process::exit(code);
}
```
`table.rs` privremeno dobiva potpise `render_docs(&DocsHealth) -> String` i `render_signals(&[Signal]) -> String` s `todo!("cigla M1/19")` (i ukloni `#![allow(dead_code)]`).

- [ ] **Step 4: prolazi** — `cargo test -p sokratis-cli` → 2 PASS. **Step 5: commit** — `git commit -am "M1/18: CLI naredbe report/docs/signals -- izlazni kod je ugovor (0/1/2/3)"`

---

### Task 19: Tablični ispis s hrvatskim natpisima (tok CLI)

**Files:** Modify `crates/sokratis-cli/src/table.rs`

**Interfaces:** Produces `render(&Report) -> String`, `render_docs(&DocsHealth) -> String`, `render_signals(&[Signal]) -> String`. Natpisi = hrvatski nazivi iz lista Sažetak (mapa `label(id)`); id bez natpisa se ispisuje kakav jest.

- [ ] **Step 1: test koji pada (inline)**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sokratis_core::*;

    #[test]
    fn labels_are_croatian_and_unknown_ids_pass_through() {
        assert_eq!(label("debugging_share"), "udio debugginga");
        assert_eq!(label("nepoznato"), "nepoznato");
        let r = Report { generated_at: 0, since: "2026-08-29".into(), branch: "main".into(), touched: Touched { commits: 3, lines: 10, files: 2, skipped_lines: 0 },
            days: vec![DayStats { date: "2026-09-01".into(), commits: 3, commits_cumulative: 3, lines: 10, hours: 1.5, deliveries: 1, deploys: 0, test_lines: 2 }],
            kinds: vec![], indicators: vec![Indicator { id: "commits".into(), value: 3.0, kind: IndicatorKind::Measure, formula: "n".into() }],
            phases: vec![], visions: vec![], docs: None, signals: vec![] };
        let s = render(&r);
        assert!(s.contains("TEMPO PO DANU") && s.contains("2026-09-01") && s.contains("commita") && s.contains("dotaknuto: 3 commita"));
        let sig = render_signals(&[Signal { rule: "docs-lag".into(), severity: Severity::Warn, title_key: "signal.docs_lag".into(), evidence: vec!["dokaz".into()], since: None }]);
        assert!(sig.contains("WARN") && sig.contains("docs-lag") && sig.contains("  - dokaz"));
        assert_eq!(render_signals(&[]), "nema signala");
    }
}
```

- [ ] **Step 2: pada** · **Step 3: implementacija**

```rust
//! ZAŠTO RUST OVAKO (cigla M1/19 — tablica)
//! `match` nad `&str` s `_ => id` = tablica prijevoda bez `HashMap` i bez alokacije; natpisi su
//! ovdje, u sučelju, a ne u jezgri (S-008). `writeln!(s, …)` na `String` kroz `std::fmt::Write`
//! gradi izlaz bez međuvektora.
use sokratis_core::{DocsHealth, Report, Severity, Signal};
use std::fmt::Write;

pub fn label(id: &str) -> &str {
    match id {
        "working_days" => "radnih dana (dana s commitom)", "commits" => "commita", "commits_per_day" => "commita po radnom danu",
        "deliveries" => "isporuka (unosa u dnevniku)", "deliveries_per_day" => "isporuka po radnom danu",
        "hours" => "sati rada (git-hours proxy)", "commits_per_hour" => "commita po satu (proxy)",
        "lines_changed" => "redaka promijenjeno (+/−)", "test_lines" => "redaka u testovima i branama", "test_share" => "udio testnih redaka",
        "deploys" => "deploya na produkciju", "debugging_commits" => "debugging commita", "debugging_share" => "udio debugginga",
        "docs_share" => "udio dokumentacije", "ci_fixes" => "CI-padova popravljenih", "owner_driven_deliveries" => "isporuka pokrenutih vlasnikovim nalazom",
        "closed_phases_in_range" => "zatvorenih faza u razdoblju", "closed_phase_avg_days" => "prosječno trajanje zatvorene faze (dana)",
        "planning" => "planiranje", "documentation" => "vođenje dokumentacije", "execution" => "izvođenje procesa", "polish" => "poliranje koda", "debugging" => "debugging",
        _ => id,
    }
}

pub fn render(r: &Report) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "Sokratis — analiza rada · grana {} · od {} · dotaknuto: {} commita, {} redaka, {} datoteka, preskočeno {} redaka\n",
        r.branch, r.since, r.touched.commits, r.touched.lines, r.touched.files, r.touched.skipped_lines);
    let _ = writeln!(s, "TEMPO PO DANU\n{:<12}{:>8}{:>8}{:>10}{:>7}{:>9}{:>8}{:>10}", "dan", "commiti", "kum.", "redaka", "sati", "isporuke", "deploya", "testnih");
    for d in &r.days {
        let _ = writeln!(s, "{:<12}{:>8}{:>8}{:>10}{:>7.1}{:>9}{:>8}{:>10}", d.date, d.commits, d.commits_cumulative, d.lines, d.hours, d.deliveries, d.deploys, d.test_lines);
    }
    let _ = writeln!(s, "\nVRSTE RADA");
    for k in &r.kinds {
        let _ = writeln!(s, "{:<26}{:>6}{:>8.1}%{:>10}", label(&serde_json::to_value(k.kind).map(|v| v.as_str().unwrap_or("").to_string()).unwrap_or_default()), k.commits, k.share * 100.0, k.lines);
    }
    let _ = writeln!(s, "\nKVALITETA I BRZINA");
    for i in &r.indicators {
        let kind = if i.kind == sokratis_core::IndicatorKind::Proxy { " (proxy)" } else { "" };
        let _ = writeln!(s, "{:<44}{:>10}{}", label(&i.id), i.value, kind);
    }
    let _ = writeln!(s, "\nFAZE");
    for p in &r.phases {
        let _ = writeln!(s, "{:<50} {:?} {}/{}", p.name, p.state, p.done_bricks, p.total_bricks);
    }
    if let Some(d) = &r.docs {
        let _ = writeln!(s, "\n{}", render_docs(d));
    }
    let _ = writeln!(s, "\nSIGNALI\n{}", render_signals(&r.signals));
    s
}

pub fn render_docs(d: &DocsHealth) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "DOKUMENTACIJA — ocjena {}/100, nalaza {}, kašnjenje {}", d.score, d.findings.len(),
        d.lag_days.map(|x| format!("{x} dana")).unwrap_or_else(|| "n/a".into()));
    for f in &d.findings {
        let line = f.line.map(|l| format!(":{l}")).unwrap_or_default();
        let _ = writeln!(s, "  {:<22} {}{}  {}", f.check, f.path, line, f.message);
    }
    s.trim_end().to_string()
}

pub fn render_signals(signals: &[Signal]) -> String {
    if signals.is_empty() {
        return "nema signala".into();
    }
    let mut s = String::new();
    for sig in signals {
        let sev = match sig.severity { Severity::Alert => "ALERT", Severity::Warn => "WARN", Severity::Info => "INFO" };
        let _ = writeln!(s, "{sev:<6} {}", sig.rule);
        for e in &sig.evidence {
            let _ = writeln!(s, "  - {e}");
        }
    }
    s.trim_end().to_string()
}
```
Ako `serde_json` nije u `[dependencies]` CLI-ja — jest (T1). Za vrstu rada je jednostavnije dodati u jezgru `impl WorkKind { pub fn id(&self) -> &'static str }`; to je izmjena tuđe datoteke → **javi orkestratoru**, ne radi sam.

- [ ] **Step 4: prolazi** — `cargo test -p sokratis-cli` → 3 PASS; clippy čist. **Step 5: commit** — `git commit -am "M1/19: tablicni ispis -- hrvatski natpisi u sucelju, id-jevi ostaju engleski"`

---
### Task 20: `build_report` — sastavljanje (tok INTEGRACIJA, nakon spajanja svih tokova)

**Files:** Modify `crates/sokratis-core/src/report.rs`; Modify `crates/sokratis-core/src/model.rs` (dodaj `WorkKind::id()`, traženo iz T19)

**Interfaces:** Consumes sve iz T3–T14. Produces `build_report(input, profile) -> Result<Report, ParseError>`:
1. `Patterns::compile`; `parse_git_log(input.git_log)` → `all`; `commits` = `all` s `commit_date >= input.since` (kao git `--since`);
2. `deliveries = parse_diary(diary, p, since)` (prazno ako nema); `plan_phases = parse_plan(plan, p)`;
3. `hours`, `days`, `kinds`, `phases = closed_phases(all) ++ active_phases(plan_phases, commits, today)`, `indicators`;
4. `last_code_commit` = commit s najvećim `author_time` među `commits` koji ima bar jednu `profile.is_code_path` datoteku;
5. `docs = docs_health(input.docs, last_code_commit.author_time, profile, p)`;
6. `Context` (vlasništvo, klonira `commits`/`branches`/`docs`) → `evaluate_all(default_rules())`;
7. `touched`: commits = `commits.len()`, lines = Σ, files = Σ `files.len()`, skipped = `parsed.skipped_lines`;
8. `generated_at = input.now`, `since`, `branch`, `visions` = kopija ulaza.

- [ ] **Step 1: test koji pada (inline u `report.rs`)**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BranchInfo, DocFile, WorkKind};
    use std::collections::HashMap;

    const LOG: &str = "@@a1|1788700000|1788700000|2026-08-28|2026-08-28|F1/1 prije since\n1\t0\tjs/a.js\n\n@@b2|1788854400|1788854400|2026-09-04|2026-09-04|fix: kvar u js\n5\t1\tjs/b.js\n\n@@c3|1788858000|1788858000|2026-09-04|2026-09-04|docs: zapis\n3\t0\tdocs/records/PROGRESS.md\n";

    fn input() -> ReportInput {
        ReportInput {
            git_log: LOG.into(),
            diary: Some("## 2026-09-04 (X) — 🚀 deploy nečega\n".into()),
            plan: Some("| **F1/1** ✅ |\n| **F1/2** |\n".into()),
            docs: vec![DocFile { path: "docs/records/PROGRESS.md".into(), content: String::new(), last_change_time: Some(1788858000) }],
            branches: vec![BranchInfo { name: "feat/stara".into(), last_commit_time: 1788854400 - 20 * 86_400, ahead_of_default: 4, merged: false }],
            overrides: HashMap::from([("b2".to_string(), WorkKind::Polish)]),
            visions: vec![],
            now: 1788854400 + 86_400,
            today: "2026-09-05".into(),
            since: "2026-08-29".into(),
            branch: "main".into(),
        }
    }

    #[test]
    fn assembles_everything_and_filters_by_since() {
        let r = build_report(&input(), &Profile::default()).unwrap();
        assert_eq!((r.touched.commits, r.touched.lines, r.touched.files, r.touched.skipped_lines), (2, 9, 2, 0));
        assert_eq!(r.days.len(), 1);
        assert_eq!((r.days[0].deliveries, r.days[0].deploys), (1, 1));
        let polish = r.kinds.iter().find(|k| k.kind == WorkKind::Polish).unwrap();
        assert_eq!(polish.commits, 1, "override b2 → polish");
        assert_eq!(r.indicators.len(), 18);
        let f1 = r.phases.iter().find(|p| p.id == "F1").unwrap();
        assert_eq!((f1.total_bricks, f1.done_bricks, f1.commits), (2, 1, 0), "F1/1 je prije since pa se ne broji");
        assert_eq!(r.phases.iter().filter(|p| p.state == crate::PhaseState::Closed).count(), 4);
        assert!(r.docs.is_some());
        let rules: Vec<&str> = r.signals.iter().map(|s| s.rule.as_str()).collect();
        assert!(rules.contains(&"unmerged-branches"), "{rules:?}");
        assert!(!rules.contains(&"docs-lag"), "dnevnik je svježiji od koda");
        assert_eq!((r.generated_at, r.branch.as_str()), (input().now, "main"));
    }
}
```

- [ ] **Step 2: pada** — `cargo test -p sokratis-core report`

- [ ] **Step 3: implementacija**

U `model.rs` dodaj:
```rust
impl WorkKind {
    pub fn id(&self) -> &'static str {
        match self { WorkKind::Planning => "planning", WorkKind::Documentation => "documentation", WorkKind::Execution => "execution", WorkKind::Polish => "polish", WorkKind::Debugging => "debugging" }
    }
}
```
(T19 tada zove `label(k.kind.id())` umjesto `serde_json::to_value`.)

`report.rs`:
```rust
//! ZAŠTO RUST OVAKO (cigla M1/20 — sastavljanje izvještaja)
//! Ovo je jedino mjesto koje zna REDOSLIJED koraka; svaki korak je funkcija iz svog modula.
//! `Context` klonira `commits`/`branches`/`docs` u vlasništvo — jedna kopija po izvještaju, a
//! zauzvrat nijedan lifetime u potpisu pravila (S-002, RUST.md §1).
use crate::classify;
use crate::docs::docs_health;
use crate::metrics::indicators::IndicatorInput;
use crate::metrics::{active_phases, closed_phases, day_stats, hours_per_day, indicators, kind_stats};
use crate::parse::{parse_diary, parse_git_log, parse_plan};
use crate::rules::{default_rules, evaluate_all};
use crate::{Commit, Context, ParseError, Patterns, Profile, Report, ReportInput, Touched};

fn last_code_commit(commits: &[Commit], profile: &Profile) -> Option<Commit> {
    commits.iter()
        .filter(|c| c.files.iter().any(|f| profile.is_code_path(&f.path)))
        .max_by_key(|c| c.author_time)
        .cloned()
}

pub fn build_report(input: &ReportInput, profile: &Profile) -> Result<Report, ParseError> {
    let p = Patterns::compile(profile)?;
    let parsed = parse_git_log(&input.git_log)?;
    let all = parsed.commits;
    let commits: Vec<Commit> = all.iter().filter(|c| c.commit_date.as_str() >= input.since.as_str()).cloned().collect();
    let deliveries = input.diary.as_deref().map(|d| parse_diary(d, &p, &input.since)).unwrap_or_default();
    let plan_phases = input.plan.as_deref().map(|t| parse_plan(t, &p)).unwrap_or_default();
    let hours = hours_per_day(&commits, profile.session_gap_hours, profile.session_start_hours);
    let days = day_stats(&commits, &deliveries, &hours, profile);
    let kinds = kind_stats(&commits, &input.overrides, &p);
    let mut phases = closed_phases(&all, &p);
    phases.extend(active_phases(plan_phases, &commits, &input.today));
    let indicators = indicators(&IndicatorInput { commits: &commits, deliveries: &deliveries, days: &days, phases: &phases, overrides: &input.overrides }, profile, &p);
    let last_code = last_code_commit(&commits, profile);
    let docs = docs_health(&input.docs, last_code.as_ref().map(|c| c.author_time), profile, &p);
    let ctx = Context { profile: profile.clone(), now: input.now, commits: commits.clone(), branches: input.branches.clone(), docs: input.docs.clone(), last_code_commit: last_code };
    let signals = evaluate_all(&default_rules(), &ctx);
    let _ = classify::classify_sub; // podvrsta ulazi u izvještaj po commitu u M2 (Dnevnik pogled); ovdje samo potvrda da modul postoji
    Ok(Report {
        generated_at: input.now,
        since: input.since.clone(),
        branch: input.branch.clone(),
        touched: Touched {
            commits: commits.len(),
            lines: commits.iter().flat_map(|c| &c.files).map(|f| f.added + f.deleted).sum(),
            files: commits.iter().map(|c| c.files.len()).sum(),
            skipped_lines: parsed.skipped_lines,
        },
        days, kinds, indicators, phases,
        visions: input.visions.clone(),
        docs, signals,
    })
}
```
Redak `let _ = classify::classify_sub;` obriši ako clippy prigovori; `classify_sub` ostaje javan za M2.

- [ ] **Step 4: prolazi** — `cargo test` (cijeli workspace) zelen. **Step 5: commit** — `git add -A && git commit -m "M1/20: build_report -- redoslijed koraka na jednom mjestu, Context u vlasnistvu"`

---

### Task 21: Test pariteta s `RAD.xlsx` (tok INTEGRACIJA)

**Files:** Create `crates/sokratis-core/tests/parity.rs`

**Interfaces:** Consumes fixture iz T2 i `build_report`. Tvrdi: dani (commiti, kumulativ, redci, isporuke, deployi, testni redci) jednaki; vrste (commita, udio, redci) jednake; 16 pokazatelja jednaki (svi osim `hours` i `commits_per_hour`); faze (done/total/state) jednake; sati ≥ 0 na svakom danu; sati jednaki `hours_legacy` na danima **bez** cherry-picka (svi osim onih koje fixture README navodi).

- [ ] **Step 1: test (pada dok nema fixturea ili dok se brojke ne slažu)**

```rust
//! Paritet s Python generatorom (TESTING.md §3). Fixture je snimka; ako ovaj test padne, prvo
//! pitaj „je li fixture snimljen s istog commita" (README uz fixture), tek onda „je li kod kriv".
use serde_json::Value;
use sokratis_core::{build_report, Profile, ReportInput};
use std::collections::HashMap;

const STAMP: &str = "2026-09-17";
fn fx(suffix: &str) -> String {
    std::fs::read_to_string(format!("{}/tests/fixtures/sokratstudy-{STAMP}.{suffix}", env!("CARGO_MANIFEST_DIR"))).expect(suffix)
}

#[test]
fn matches_rad_xlsx_except_fixed_hours() {
    let expected: Value = serde_json::from_str(&fx("expected.json")).unwrap();
    let input = ReportInput {
        git_log: fx("log"), diary: Some(fx("PROGRESS.md")), plan: Some(fx("RASPORED.md")),
        docs: vec![], branches: vec![], overrides: HashMap::new(), visions: vec![],
        now: 0, today: "2026-09-17".into(), since: "2026-08-29".into(), branch: "main".into(),
    };
    let r = build_report(&input, &Profile::default()).unwrap();
    assert_eq!(r.touched.skipped_lines, 0, "fixture mora biti čist");

    let days = expected["days"].as_object().unwrap();
    assert_eq!(r.days.len(), days.len(), "broj radnih dana");
    let mut mismatches = Vec::new();
    for d in &r.days {
        let e = &days[&d.date];
        for (name, got, want) in [
            ("commits", d.commits as f64, e["commits"].as_f64().unwrap()),
            ("cumulative", d.commits_cumulative as f64, e["cumulative"].as_f64().unwrap()),
            ("lines", d.lines as f64, e["lines"].as_f64().unwrap()),
            ("deliveries", d.deliveries as f64, e["deliveries"].as_f64().unwrap()),
            ("deploys", d.deploys as f64, e["deploys"].as_f64().unwrap()),
            ("test_lines", d.test_lines as f64, e["test_lines"].as_f64().unwrap()),
        ] {
            if got != want { mismatches.push(format!("{} {name}: {got} != {want}", d.date)); }
        }
        assert!(d.hours >= 0.0, "{}: negativni sati {}", d.date, d.hours);
        let legacy = e["hours_legacy"].as_f64().unwrap();
        if legacy >= 0.0 && (d.hours - legacy).abs() > 0.051 {
            mismatches.push(format!("{} hours: {} != legacy {legacy} (dopušteno samo na danima s cherry-pickom)", d.date, d.hours));
        }
    }
    for k in &r.kinds {
        let e = &expected["kinds"][k.kind.id()];
        if (k.commits as f64, k.share, k.lines as f64) != (e["commits"].as_f64().unwrap(), e["share"].as_f64().unwrap(), e["lines"].as_f64().unwrap()) {
            mismatches.push(format!("kind {}: {:?}", k.kind.id(), (k.commits, k.share, k.lines)));
        }
    }
    for i in &r.indicators {
        if i.id == "hours" || i.id == "commits_per_hour" { continue; }
        let want = expected["indicators"][&i.id].as_f64().unwrap_or_else(|| panic!("nema očekivanja za {}", i.id));
        if (i.value - want).abs() > 1e-9 { mismatches.push(format!("indicator {}: {} != {want}", i.id, i.value)); }
    }
    let phases = expected["phases"].as_array().unwrap();
    assert_eq!(r.phases.len(), phases.len(), "broj faza");
    for (got, want) in r.phases.iter().zip(phases) {
        let state = serde_json::to_value(got.state).unwrap();
        if got.done_bricks as f64 != want["done"].as_f64().unwrap() || got.total_bricks as f64 != want["total"].as_f64().unwrap() || state != want["state"] {
            mismatches.push(format!("phase {}: {}/{} {:?} != {}", got.name, got.done_bricks, got.total_bricks, got.state, want));
        }
    }
    assert!(mismatches.is_empty(), "razlike prema RAD.xlsx:\n{}", mismatches.join("\n"));
}
```
Dani s cherry-pickom (iz nalaza 2026-09-17): `2026-09-06` i `2026-09-08` imaju `hours_legacy < 0`, pa ih uvjet `legacy >= 0.0` preskače; ako fixture pokaže još koji negativan dan, on je automatski preskočen — a README uz fixture ih nabraja.

- [ ] **Step 2: pokreni** — `cargo test -p sokratis-core --test parity`. **Očekuj razlike** pri prvom prolazu: svaka je ili (a) greška u jezgri → popravi u toku koji je vlasnik, ili (b) razlika Python/git semantike (npr. `--since` po committer-datumu, `%ad` po autoru) → zapiši u README uz fixture i u `TESTING.md` §3, pa uskladi jezgru s gitom (git je istina, ne Python). Nema (c).

- [ ] **Step 3: commit** — `git add -A && git commit -m "M1/21: test pariteta s RAD.xlsx -- sve jednako osim ispravljenih sati"`

---

### Task 22: Dogfooding, preflight u Sokrat Studyju, dokumentacija (tok INTEGRACIJA)

**Files:** Create `.sokratis/profile.json` (Sokratis sam); Modify `docs/records/PROGRESS.md`, `docs/records/CHANGELOG.md`, `docs/workflow/RUST.md` §4, `docs/plan/ROADMAP.md`, `CLAUDE.md` (stanje); Create `docs/architecture/ARCHITECTURE.md`; Move `docs/plan/ARHITEKTURA_M1.md` → `docs/archive/ARHITEKTURA_M1.md` (s pečatom datuma); Modify `docs/README.md` (indeks)

- [ ] **Step 1: profil Sokratisa (dogfooding) — dnevnik ima isti format, plan nema cigle, faze nema**

`.sokratis/profile.json`:
```json
{
  "since": "2026-09-17",
  "plan_path": "docs/plan/ROADMAP.md",
  "closed_phases": [],
  "phase_tag": "^(M\\d/\\d+)",
  "owner_name": "leon"
}
```
Run: `cargo run -p sokratis-cli -- report . --table` → tablica s današnjim danima; `cargo run -p sokratis-cli -- docs .` → ocjena i 0 nalaza (ili popravi nalaze); `cargo run -p sokratis-cli -- signals .` → kod 0 ili 1 (grane tokova još žive → očekuj `unmerged-branches` dok ih ne obrišeš).

- [ ] **Step 2: nad Sokrat Studyjem (izlazni uvjet M1 §7 speca)**

```powershell
cargo build --release
$S = "C:\Users\leonk\Documents\sokratstudy.dev"
.\target\release\sokratis.exe report $S --table | Select-Object -First 40
.\target\release\sokratis.exe docs $S
.\target\release\sokratis.exe signals $S ; echo "kod: $LASTEXITCODE"
```
Expected: tablica sa 16 pokazatelja jednakih tablici + ispravni sati; docs-nalazi s `datoteka:redak`; signali s dokazom; kod 0/1/2. **Ništa se u Sokrat Studyju ne mijenja** — ulazak `sokratis signals` u njegov preflight je Leonova zasebna odluka i zaseban commit ondje.

- [ ] **Step 3: dokumentacija (čuvar dokumentacije)**

- `docs/architecture/ARCHITECTURE.md`: crateovi, tok podataka, format profila (sva polja s zadanim vrijednostima — jedina tablica, jezgra je izvor), format `.sokratis/*.json`, izlazni kodovi. Bez kronologije.
- `docs/plan/ARHITEKTURA_M1.md` → `docs/archive/ARHITEKTURA_M1.md` s prvim retkom `**Status:** ✅ ISPUNJEN <datum> — referenca, ne izvor istine`.
- `docs/README.md`: nova sekcija `architecture/` i `archive/`; `plan/` pokazuje da sljedeći spec (M2) tek dolazi.
- `docs/workflow/RUST.md` §4: svaki pojam iz zaglavlja cigli T3–T20 dobiva redak s datotekom (`let … else`, `splitn`, `last_mut`, `BTreeMap`, `entry`, lifetime `'a` u `IndicatorInput`, trait objekt, `saturating_sub`, `Command`, `map_err`, RAII `TempDir`, `env!("CARGO_BIN_EXE_…")`).
- `CHANGELOG.md`: `## [0.1.0] — <datum>` s popisom naredbi; `PROGRESS.md`: zapis sesije; `ROADMAP.md`: M1 ✅, M2 sljedeći; `CLAUDE.md` „Stanje": M1 isporučen, M0 gotov, komande žive.
- Provjera: `cargo run -p sokratis-cli -- docs .` = 0 nalaza (Sokratis mjeri vlastite docs-e).

- [ ] **Step 4: commit i zastanak** — `git add -A && git commit -m "M1/22: dogfooding profil, ARCHITECTURE.md, spec u arhivu, 0.1.0"` → **STANI i javi Leonu** (kraj milestonea, RASPORED-pravilo). Grane tokova brišu se tek uz njegov OK: `git branch -d feat/<tok>` + `git worktree remove ../sokratis.<tok>`.

---

## Self-review plana (napravljeno pri pisanju)

**Pokrivenost speca:** §1 kostur → T1 · §1.1 radna stabla i grane → T16/T17 (`common_dir`, `branches`) · §2.1 model → T1 · §2.2 formati → T3–T6 (+ `%cd` dodan formatu, spec §2.2 ažuriran) · §2.3 metrike → T8–T11 · §2.4 docs → T12 · §2.5 pravila i `Context` → T13, T14, T20 · §3 io → T15–T17 · §4 CLI → T18, T19 · §6 testovi → svaki task + T21 · §7 izlazni uvjet → T22. Podvrsta (`classify_sub`) se računa (T6) ali još ne ulazi u `Report` — spec je to predvidio za Dnevnik pogled u M2; zapisano u T20.

**Tipovi:** `Commit.date` (autor) i `Commit.commit_date` dodani u T1 i korišteni dosljedno (dan = `date`, `since` = `commit_date`). `Patterns` polja: `diary_heading`, `diary_deploy`, `plan_brick`, `plan_phase_name`, `phase_tag`, `classifier`, `gate`, `deploy`, `ci_fix`, `paused`, `closed_phases` — sva korištena u T4–T14. `IndicatorInput<'a>` je jedini lifetime; RUST.md §1 to bilježi kao svjesnu iznimku. `WorkKind::id()` uveden u T20, koristi T19 i T21.

**Placeholderi:** nema nedovršenih oznaka; `todo!("cigla M1/N")` u T1 su stubovi koje kasniji taskovi zamjenjuju, svaki imenovan. Provjera mrtvih poveznica preskače ``` blokove (T12) — primjer u testu nije poveznica. Unix-vremena u io-testovima su označena kao „izračunaj naredbom prije pokretanja" — to je uputa, ne rupa.

**Rizici koje orkestrator prati:** (1) prvi `cargo build` s Tauri-jem tek u M2 — M1 ne treba MSVC ništa osim linkera; (2) `serde(default)` + `deny_unknown_fields` na `Profile` — ako serde-verzija prigovori, `default` ide po polju; (3) paritet može otkriti Python/git razlike — T21 kaže: git je istina.
