//! ZAŠTO RUST OVAKO (cigla M1/10 — vrste rada)
//! `HashMap<String, WorkKind>::get(&c.sha).copied()` vraća `Option<WorkKind>` bez posudbe koja bi
//! nadživjela funkciju; `.unwrap_or_else(|| classify_kind(..))` računa heuristiku SAMO kad
//! overridea nema (lijeno). `WorkKind::ALL` jamči da su sve vrste u izlazu i kad imaju nulu.
use crate::classify::classify_kind;
use crate::{Commit, KindStats, Patterns, WorkKind};
use std::collections::HashMap;

pub fn effective_kind(c: &Commit, overrides: &HashMap<String, WorkKind>, p: &Patterns) -> WorkKind {
    overrides
        .get(&c.sha)
        .copied()
        .unwrap_or_else(|| classify_kind(&c.subject, p))
}

pub fn kind_stats(
    commits: &[Commit],
    overrides: &HashMap<String, WorkKind>,
    p: &Patterns,
) -> Vec<KindStats> {
    let total = commits.len() as f64;
    WorkKind::ALL
        .iter()
        .map(|&kind| {
            let n = commits
                .iter()
                .filter(|c| effective_kind(c, overrides, p) == kind)
                .count() as u32;
            let lines = commits
                .iter()
                .filter(|c| effective_kind(c, overrides, p) == kind)
                .flat_map(|c| c.files.iter())
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
    use crate::{FileChange, Patterns, Profile};

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
        let ks = kind_stats(&commits, &ov, &p);
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
}
