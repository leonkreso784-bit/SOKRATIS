//! ZAŠTO RUST OVAKO (cigla M1/10 — vrste rada)
//! `HashMap<String, WorkKind>::get(&c.sha).copied()` vraća `Option<WorkKind>` bez posudbe koja bi
//! nadživjela funkciju; `.unwrap_or_else(|| classify_kind(..))` računa heuristiku SAMO kad
//! overridea nema (lijeno). `WorkKind::ALL` jamči da su sve vrste u izlazu i kad imaju nulu.
//!
//! ZAŠTO RUST OVAKO (cigla M2/5 — redci commita, klasifikacija jednom)
//! `commit_rows` klasificira svaki commit TOČNO JEDNOM (vrsta uz override, podvrsta); `kind_stats`
//! te redke samo broji, umjesto da opet zove `effective_kind` po vrsti kao prije. `rows.iter().zip(
//! commits)` spaja dva paralelna niza bez indeksa; nema `[i]` koji bi mogao pasti izvan granica.
use crate::classify::{classify_kind, classify_sub};
use crate::{Commit, CommitRow, KindStats, Patterns, WorkKind};
use std::collections::HashMap;

pub fn effective_kind(c: &Commit, overrides: &HashMap<String, WorkKind>, p: &Patterns) -> WorkKind {
    overrides
        .get(&c.sha)
        .copied()
        .unwrap_or_else(|| classify_kind(&c.subject, p))
}

/// Klasificira SVAKI commit točno jednom po izvještaju (spec §3.2) — vrsta (uz override), podvrsta,
/// oznaka je li vrsta ručna. Redoslijed = redoslijed `commits`.
pub fn commit_rows(
    commits: &[Commit],
    overrides: &HashMap<String, WorkKind>,
    p: &Patterns,
) -> Vec<CommitRow> {
    commits
        .iter()
        .map(|c| CommitRow {
            sha: c.sha.clone(),
            date: c.date.clone(),
            subject: c.subject.clone(),
            kind: effective_kind(c, overrides, p),
            sub: classify_sub(&c.subject, p),
            overridden: overrides.contains_key(&c.sha),
        })
        .collect()
}

/// Broji redke već klasificirane u `commit_rows` — `rows` i `commits` su paralelni nizovi
/// (isti redoslijed, ista dužina); `lines` se čita iz `commits[i].files`, vrsta iz `rows[i].kind`.
pub fn kind_stats(rows: &[CommitRow], commits: &[Commit]) -> Vec<KindStats> {
    let total = rows.len() as f64;
    WorkKind::ALL
        .iter()
        .map(|&kind| {
            let n = rows.iter().filter(|r| r.kind == kind).count() as u32;
            let lines = rows
                .iter()
                .zip(commits)
                .filter(|(r, _)| r.kind == kind)
                .flat_map(|(_, c)| c.files.iter())
                .map(|f| f.added + f.deleted)
                .sum();
            let share = if total > 0.0 {
                (n as f64 / total * 1000.0).round() / 1000.0
            } else {
                0.0
            };
            KindStats {
                kind,
                commits: n,
                share,
                lines,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FileChange, Patterns, Profile, SubKind};

    fn c(sha: &str, subject: &str, lines: u64) -> Commit {
        Commit {
            sha: sha.into(),
            author_time: 0,
            commit_time: 0,
            date: "2026-09-01".into(),
            commit_date: "2026-09-01".into(),
            subject: subject.into(),
            files: vec![FileChange {
                path: "x".into(),
                added: lines,
                deleted: 0,
            }],
        }
    }

    #[test]
    fn override_wins_and_all_kinds_are_listed() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let commits = vec![
            c("a", "fix: nešto", 10),
            c("b", "fix: drugo", 20),
            c("c", "F1/1 cigla", 30),
        ];
        let mut ov = HashMap::new();
        ov.insert("b".to_string(), WorkKind::Polish);
        assert_eq!(effective_kind(&commits[0], &ov, &p), WorkKind::Debugging);
        assert_eq!(effective_kind(&commits[1], &ov, &p), WorkKind::Polish);
        let rows = commit_rows(&commits, &ov, &p);
        let ks = kind_stats(&rows, &commits);
        let kinds: Vec<WorkKind> = ks.iter().map(|k| k.kind).collect();
        assert_eq!(kinds, WorkKind::ALL.to_vec());
        let get = |k: WorkKind| ks.iter().find(|x| x.kind == k).unwrap();
        assert_eq!(
            (
                get(WorkKind::Debugging).commits,
                get(WorkKind::Debugging).lines,
                get(WorkKind::Debugging).share
            ),
            (1, 10, 0.333)
        );
        assert_eq!(
            (
                get(WorkKind::Polish).commits,
                get(WorkKind::Execution).commits,
                get(WorkKind::Planning).commits
            ),
            (1, 1, 0)
        );
    }

    /// M2/5: pomoćnik za redke commita — subject nosi klasifikaciju, put datoteke ne utječe na
    /// `commit_rows`/`kind_stats` (za razliku od `c` iznad, gdje `lines` mjeri `KindStats.lines`).
    fn row_commit(sha: &str, subject: &str) -> Commit {
        Commit {
            sha: sha.into(),
            author_time: 0,
            commit_time: 0,
            date: "2026-09-01".into(),
            commit_date: "2026-09-01".into(),
            subject: subject.into(),
            files: vec![FileChange {
                path: "js/a.js".into(),
                added: 1,
                deleted: 0,
            }],
        }
    }

    #[test]
    fn commit_rows_carry_kind_sub_and_override_flag() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let commits = vec![
            row_commit("a1", "F1/1 cigla"),
            row_commit("b2", "fix: kvar"),
            row_commit("c3", "docs: zapis 🚀 na produkciji"),
        ];
        let mut ov = HashMap::new();
        ov.insert("b2".to_string(), WorkKind::Polish);
        let rows = commit_rows(&commits, &ov, &p);
        let got: Vec<(&str, WorkKind, SubKind, bool)> = rows
            .iter()
            .map(|r| (r.sha.as_str(), r.kind, r.sub, r.overridden))
            .collect();
        assert_eq!(
            got,
            vec![
                ("a1", WorkKind::Execution, SubKind::Brick, false),
                ("b2", WorkKind::Polish, SubKind::Other, true),
                ("c3", WorkKind::Documentation, SubKind::Deploy, false),
            ]
        );
    }

    #[test]
    fn kind_stats_over_rows_matches_previous_semantics() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let commits = vec![
            row_commit("a1", "F1/1 cigla"),
            row_commit("b2", "fix: kvar"),
        ];
        let rows = commit_rows(&commits, &HashMap::new(), &p);
        let ks = kind_stats(&rows, &commits);
        let exec = ks.iter().find(|k| k.kind == WorkKind::Execution).unwrap();
        assert_eq!((exec.commits, exec.share), (1, 0.5));
    }
}
