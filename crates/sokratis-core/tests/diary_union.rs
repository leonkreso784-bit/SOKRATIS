//! ZAŠTO RUST OVAKO (cigla M2/47 — unija dnevnika, S-033)
//! Dva teksta dnevnika (vodeće stablo prvo, pa `main`) s jednim preklopljenim naslovom: unija po
//! (datum, naslov), PRVI viđeni pobjeđuje — pa `model` iz vodećeg stabla ostaje. Test ide kroz
//! `build_report` (javni ugovor), ne kroz `parse_diaries` izravno, jer je `Touched.diaries` dio
//! izlaza i mora se vidjeti izvana.
use sokratis_core::{BranchScope, Profile, ReportInput, build_report};
use std::collections::HashMap;

const LEAD: &str = "\
## 2026-09-20 (FABLE) — F6/1 MCP most\n\n\
## 2026-09-21 (FABLE) — F6/2 čitanje polica 🚀\n\n\
## 2026-09-22 (SONNET) — docs: zapis F6\n";
const MAIN: &str = "\
## 2026-09-13 (FABLE) — F2/9 zid gotov\n\n\
## 2026-09-20 (OPUS) — F6/1 MCP most\n";

fn input(diaries: Vec<String>) -> ReportInput {
    ReportInput {
        git_log: "@@a1|1789020000|1789020000|2026-09-10|2026-09-10|F1/1 kod\n1\t0\tjs/a.js\n"
            .into(),
        diaries,
        plan: None,
        docs: vec![],
        branches: vec![],
        overrides: HashMap::new(),
        visions: vec![],
        now: 1789300000,
        today: "2026-09-23".into(),
        since: "2026-09-01".into(),
        until: None,
        branch: "main".into(),
        scope: BranchScope::AllBranches,
        commit_branches: HashMap::new(),
        worktrees: 2,
    }
}

#[test]
fn deliveries_are_united_by_date_and_title_first_wins() {
    let r = build_report(&input(vec![LEAD.into(), MAIN.into()]), &Profile::default()).unwrap();
    let titles: Vec<(&str, &str)> = r
        .deliveries
        .iter()
        .map(|d| (d.date.as_str(), d.title.as_str()))
        .collect();
    assert_eq!(
        titles,
        [
            ("2026-09-13", "F2/9 zid gotov"),
            ("2026-09-20", "F6/1 MCP most"),
            ("2026-09-21", "F6/2 čitanje polica 🚀"),
            ("2026-09-22", "docs: zapis F6"),
        ],
        "4 isporuke: preklop (20. 9.) jednom, sortirano po datumu"
    );
    let overlap = r
        .deliveries
        .iter()
        .find(|d| d.date == "2026-09-20")
        .unwrap();
    assert_eq!(
        overlap.model, "FABLE",
        "prvi tekst (vodeće stablo) pobjeđuje"
    );
    assert_eq!((r.touched.worktrees, r.touched.diaries), (2, 2));
}

#[test]
fn no_diaries_means_no_deliveries_and_zero_touched() {
    let r = build_report(&input(vec![]), &Profile::default()).unwrap();
    assert!(r.deliveries.is_empty());
    assert_eq!(r.touched.diaries, 0);
}
