//! ZAŠTO RUST OVAKO (cigla M1/1 — model)
//! Sve su strukture `pub` s `pub` poljima i `derive(Serialize, Deserialize)`: to je ugovor
//! prema CLI-ju i sučelju (JSON). Enumi s `rename_all = "snake_case"` daju `"debugging"`, ne
//! `"Debugging"` (S-008). `Clone` je namjeran — jezgra radi s podacima koje posjeduje.
//!
//! Dopuna (cigla M2/29a): `CommitRow.author_time` nosi `i64` iz `Commit`, ne novi tip — Pregled
//! računa „prije X" iz istog broja koji tablica već zna, pa nema drugog izvora vremena (S-010).
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
pub enum WorkKind {
    Planning,
    Documentation,
    Execution,
    Polish,
    Debugging,
}

impl WorkKind {
    pub const ALL: [WorkKind; 5] = [
        WorkKind::Planning,
        WorkKind::Documentation,
        WorkKind::Execution,
        WorkKind::Polish,
        WorkKind::Debugging,
    ];
}

// Dodatak ugovoru (orkestrator, M1/1): plan ovo inače uvodi tek u T20, ali T19 (CLI tablica) i
// T21 (indikatori) trebaju stabilan string-identifikator ranije, pa je dio ugovora od početka.
impl WorkKind {
    pub fn id(&self) -> &'static str {
        match self {
            WorkKind::Planning => "planning",
            WorkKind::Documentation => "documentation",
            WorkKind::Execution => "execution",
            WorkKind::Polish => "polish",
            WorkKind::Debugging => "debugging",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubKind {
    Brick,
    GateOrMeasure,
    Deploy,
    Other,
}

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
pub enum PhaseState {
    Planned,
    Running,
    Closed,
}

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

/// Zbroj vizija po stanju — mjerenje, ne prikaz (S-012); puni ga `metrics::visions` (M2/4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisionTotal {
    pub state: String,
    pub count: u32,
}

/// Jedan redak Dnevnika: commit s vrstom i podvrstom, klasificiran JEDNOM po izvještaju (M2/5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitRow {
    pub sha: String,
    /// `YYYY-MM-DD` autora (kao tablica)
    pub date: String,
    /// unix sekunde autora (S-007) — Pregled iz njega crta „prije 2 h"
    pub author_time: i64,
    pub subject: String,
    pub kind: WorkKind,
    pub sub: SubKind,
    /// vrsta dolazi iz `.sokratis/overrides.json`, ne iz klasifikatora
    pub overridden: bool,
}

/// Broj signala po težini — ono što Pregled i snimka trebaju umjesto cijelog popisa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SignalCounts {
    pub info: u32,
    pub warn: u32,
    pub alert: u32,
}

/// Jedna brojka snimke: `id` pokazatelja, vrijednost i je li mjera ili proxy (stupac `kind` u bazi).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    pub id: String,
    pub value: f64,
    pub kind: IndicatorKind,
}

/// Što snimka drži (S-014): 18 pokazatelja, docs-ocjena, broj signala — NE cijeli `Report`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotMetrics {
    pub indicators: Vec<MetricValue>,
    pub docs_score: Option<u8>,
    pub signals: SignalCounts,
}

/// Razlika jedne brojke između dviju snimki (M2/7 `snapshot::diff`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDelta {
    pub id: String,
    pub before: f64,
    pub after: f64,
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
pub enum IndicatorKind {
    Measure,
    Proxy,
}

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
pub enum Severity {
    Info,
    Warn,
    Alert,
}

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
    /// gornja granica razdoblja (`YYYY-MM-DD`, cijeli dan), `None` = do danas (M2/3)
    pub until: Option<String>,
    pub branch: String,
    pub touched: Touched,
    pub days: Vec<DayStats>,
    pub kinds: Vec<KindStats>,
    /// commiti razdoblja s vrstom i podvrstom — ulaz za Dnevnik (M2/5)
    pub commits: Vec<CommitRow>,
    /// isporuke iz dnevnika u razdoblju — ulaz za pogled Isporuke (M2/5); M1 ih je samo zbrajao po danu
    pub deliveries: Vec<Delivery>,
    pub indicators: Vec<Indicator>,
    pub phases: Vec<Phase>,
    pub visions: Vec<Vision>,
    pub vision_totals: Vec<VisionTotal>,
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
    /// gornja granica, `YYYY-MM-DD`, uključivo cijeli dan; `None` = bez gornje granice
    pub until: Option<String>,
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
        assert_eq!(
            serde_json::to_string(&WorkKind::Debugging).unwrap(),
            "\"debugging\""
        );
        assert_eq!(
            serde_json::to_string(&SubKind::GateOrMeasure).unwrap(),
            "\"gate_or_measure\""
        );
        let s: Severity = serde_json::from_str("\"alert\"").unwrap();
        assert_eq!(s, Severity::Alert);
        assert!(Severity::Alert > Severity::Warn && Severity::Warn > Severity::Info);
        assert_eq!(WorkKind::Debugging.id(), "debugging");
    }
}
