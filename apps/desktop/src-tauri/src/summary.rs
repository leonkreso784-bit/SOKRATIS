//! ZAŠTO RUST OVAKO (cigla M2/29 — sažetak projekta za Pregled)
//! Jedini nov oblik prema sučelju (spec §5.1). `Option<&Report>` umjesto `Report`: projekt čija
//! mapa više ne postoji ima `error`, ne lažne nule. Sve brojke dolaze iz `core::snapshot` — ovdje je
//! samo preslagivanje polja u strukturu koju Svelte crta, nikakvo novo mjerenje (S-012).
use serde::Serialize;
use sokratis_core::{Report, Severity, SignalCounts};
use sokratis_store::ProjectRecord;

#[derive(Debug, Clone, Serialize)]
pub struct LastCommit {
    pub sha: String,
    pub time: i64,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectSummary {
    pub id: i64,
    pub name: String,
    pub root_path: String,
    pub worktrees: u32,
    pub last_refresh: Option<i64>,
    pub last_commit: Option<LastCommit>,
    pub worst: Option<Severity>,
    pub signals: SignalCounts,
    pub error: Option<String>,
}

/// Slaže sažetak za Pregled iz registarskog retka i (ako postoji) zadnjeg izračunatog izvještaja.
/// Zadnji commit je NAJNOVIJI po `author_time`, ne zadnji u nizu — `Report.commits` ne jamči
/// poredak (CommitRow nosi `author_time` od M2/29a).
pub fn summarize(
    p: &ProjectRecord,
    worktrees: u32,
    report: Option<&Report>,
    error: Option<String>,
) -> ProjectSummary {
    let last_commit = report
        .and_then(|r| r.commits.iter().max_by_key(|c| c.author_time))
        .map(|c| LastCommit {
            sha: c.sha.clone(),
            time: c.author_time,
            subject: c.subject.clone(),
        });
    ProjectSummary {
        id: p.id,
        name: p.name.clone(),
        root_path: p.root_path.to_string_lossy().to_string(),
        worktrees,
        last_refresh: report.map(|r| r.generated_at),
        last_commit,
        worst: report.and_then(|r| sokratis_core::worst_severity(&r.signals)),
        signals: report
            .map(|r| SignalCounts::from_signals(&r.signals))
            .unwrap_or_default(),
        error,
    }
}
