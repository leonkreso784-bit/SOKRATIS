//! ZAŠTO RUST OVAKO (cigla M2/29 — sažetak projekta za Pregled)
//! Jedini nov oblik prema sučelju (spec §5.1). `Option<&Report>` umjesto `Report`: projekt čija
//! mapa više ne postoji ima `error`, ne lažne nule. Sve brojke dolaze iz `core::snapshot` — ovdje je
//! samo preslagivanje polja u strukturu koju Svelte crta, nikakvo novo mjerenje (S-012).
//!
//! Dopunjeno M2/63 — I4: broj stabala više NE dolazi izravno iz registra (koji pamti stanje s dana
//! dodavanja projekta) nego iz `Report.touched.worktrees` kad izvještaj postoji — to je izmjereno
//! pri zadnjem izračunu (S-012); registarski broj ostaje samo padobran dok izvještaja još nema.
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
        // I4 (M2/63): registar pamti stabla s DANA DODAVANJA (`track_project`, ne osvježava se dok
        // se projekt ne izbriše i doda opet); `touched.worktrees` je izmjereno pri ZADNJEM izračunu
        // (`worktree list` u `io`, S-012) — kad izvještaj postoji, on je istinitiji broj.
        worktrees: report.map(|r| r.touched.worktrees).unwrap_or(worktrees),
        last_refresh: report.map(|r| r.generated_at),
        last_commit,
        worst: report.and_then(|r| sokratis_core::worst_severity(&r.signals)),
        signals: report
            .map(|r| SignalCounts::from_signals(&r.signals))
            .unwrap_or_default(),
        error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sokratis_core::{BranchScope, Profile, ReportInput, build_report};
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// I4: registar (`ProjectRecord`) je izmišljen sa `worktrees` kojih tu nema (5 — stanje s dana
    /// dodavanja); izvještaj nosi `touched.worktrees = 7` (izmjereno PRI ZADNJEM izračunu). Kad
    /// izvještaj postoji, sažetak mora pokazati NJEGOVU brojku, ne registarsku.
    fn minimal_report(worktrees: u32) -> Report {
        let input = ReportInput {
            git_log: String::new(),
            diaries: vec![],
            plan: None,
            docs: vec![],
            branches: vec![],
            overrides: HashMap::new(),
            visions: vec![],
            now: 1_767_312_000, // 2026-01-02 00:00:00 UTC
            today: "2026-01-02".into(),
            since: "2026-01-01".into(),
            until: None,
            branch: "main".into(),
            scope: BranchScope::AllBranches,
            commit_branches: HashMap::new(),
            worktrees,
            branch_graph: String::new(),
        };
        build_report(&input, &Profile::default()).expect("minimalni ulaz mora proći kroz jezgru")
    }

    fn record() -> ProjectRecord {
        ProjectRecord {
            id: 1,
            name: "sokratstudy".into(),
            root_path: PathBuf::from("C:/repo"),
            git_common_dir: PathBuf::from("C:/repo/.git"),
            added_at: 1_767_000_000,
            last_seen_at: 1_767_000_000,
        }
    }

    #[test]
    fn worktrees_prefer_the_report_measurement_over_the_registry() {
        let report = minimal_report(7);
        let summary = summarize(&record(), 5, Some(&report), None);
        assert_eq!(summary.worktrees, 7);
    }

    #[test]
    fn worktrees_fall_back_to_the_registry_without_a_report() {
        let summary = summarize(&record(), 5, None, None);
        assert_eq!(summary.worktrees, 5);
    }
}
