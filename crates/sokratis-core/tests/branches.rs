//! ZAŠTO RUST OVAKO (cigla M2/46 — grane u jezgri, S-032)
//! Integracijski test: jezgra vidi SAMO tekst loga + kartu `sha → grana` (`ReportInput`), nikad git.
//! Fixture je inline: četiri commita, dvije grane izvan zadane, jedna s `|` u imenu — redak
//! `%h|%S` iz `io` se dijeli na PRVOM `|`, pa ime s `|` mora preživjeti (Review Focus #3).
use sokratis_core::{BranchInfo, BranchScope, Profile, ReportInput, build_report};
use std::collections::HashMap;

// Dan 1 (2026-09-10): a1 na main u 08:00, b2 na feat/x u 09:30 (razmak 1,5 h < 2 h → +1,5)
// Dan 2 (2026-09-11): c3 na feat/x u 08:00 (nova sesija +0,5), d4 na "y|z" u 09:30 (+1,5)
// → sati dana: 2,0 i 2,0; po udjelu: main 1,0 · feat/x 2,0 · y|z 1,0 (zbroj = 4,0 = ukupno).
const LOG: &str = "\
@@a1|1789020000|1789020000|2026-09-10|2026-09-10|F1/1 kod na mainu\n1\t0\tjs/a.js\n\n\
@@b2|1789025400|1789025400|2026-09-10|2026-09-10|F1/2 kod u grani\n2\t0\tjs/b.js\n\n\
@@c3|1789106400|1789106400|2026-09-11|2026-09-11|fix: kvar u grani\n3\t1\tjs/c.js\n\n\
@@d4|1789111800|1789111800|2026-09-11|2026-09-11|docs: zapis u drugoj grani\n4\t0\tdocs/records/PROGRESS.md\n";

fn input(scope: BranchScope, map: &[(&str, &str)]) -> ReportInput {
    ReportInput {
        git_log: LOG.into(),
        diary: None,
        plan: None,
        docs: vec![],
        branches: vec![
            BranchInfo {
                name: "feat/x".into(),
                last_commit_time: 1789106400,
                ahead_of_default: 2,
                merged: false,
            },
            BranchInfo {
                name: "y|z".into(),
                last_commit_time: 1789111800,
                ahead_of_default: 1,
                merged: false,
            },
        ],
        overrides: HashMap::new(),
        visions: vec![],
        now: 1789300000,
        today: "2026-09-12".into(),
        since: "2026-09-01".into(),
        until: None,
        branch: "main".into(),
        scope,
        commit_branches: map
            .iter()
            .map(|(s, b)| (s.to_string(), b.to_string()))
            .collect(),
    }
}

#[test]
fn rows_carry_branch_and_stats_split_hours_by_commit_share() {
    let r = build_report(
        &input(
            BranchScope::AllBranches,
            &[("b2", "feat/x"), ("c3", "feat/x"), ("d4", "y|z")],
        ),
        &Profile::default(),
    )
    .unwrap();
    assert_eq!(r.scope, BranchScope::AllBranches);
    let by_sha: HashMap<&str, &str> = r
        .commits
        .iter()
        .map(|c| (c.sha.as_str(), c.branch.as_str()))
        .collect();
    assert_eq!(by_sha["a1"], "main", "commit izvan karte je zadana grana");
    assert_eq!(by_sha["b2"], "feat/x");
    assert_eq!(by_sha["d4"], "y|z", "ime grane s | preživi");
    let names: Vec<&str> = r.branches.iter().map(|b| b.name.as_str()).collect();
    assert_eq!(
        names,
        ["main", "feat/x", "y|z"],
        "zadana prva, ostale po commitima silazno pa po imenu"
    );
    let feat = &r.branches[1];
    assert_eq!((feat.commits, feat.lines, feat.merged), (2, 6, false));
    assert_eq!(feat.hours, 2.0);
    assert_eq!(r.branches[0].hours, 1.0);
    assert_eq!(r.branches[2].hours, 1.0);
    assert!(r.branches[0].merged, "zadana grana je uvijek 'spojena'");
    let total: f64 = r.days.iter().map(|d| d.hours).sum();
    let split: f64 = r.branches.iter().map(|b| b.hours).sum();
    assert!(
        (total - split).abs() < 1e-9,
        "zbroj po granama = ukupni sati"
    );
}

#[test]
fn single_branch_gives_one_row() {
    let r = build_report(&input(BranchScope::DefaultBranch, &[]), &Profile::default()).unwrap();
    assert_eq!(r.scope, BranchScope::DefaultBranch);
    assert!(r.commits.iter().all(|c| c.branch == "main"));
    assert_eq!(r.branches.len(), 1);
    assert_eq!(r.branches[0].commits, 4);
}

#[test]
fn empty_commits_give_no_rows() {
    let mut i = input(BranchScope::AllBranches, &[]);
    i.since = "2026-09-20".into();
    let r = build_report(&i, &Profile::default()).unwrap();
    assert!(r.commits.is_empty());
    assert!(r.branches.is_empty());
}

#[test]
fn scope_and_branches_are_in_json() {
    let r = build_report(
        &input(BranchScope::AllBranches, &[("b2", "feat/x")]),
        &Profile::default(),
    )
    .unwrap();
    let v: serde_json::Value = serde_json::to_value(&r).unwrap();
    assert_eq!(v["scope"], "all");
    assert_eq!(v["branches"][0]["name"], "main");
    assert_eq!(v["commits"][1]["branch"], "feat/x");
}
