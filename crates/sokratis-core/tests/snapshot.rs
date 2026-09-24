//! ZAŠTO RUST OVAKO (cigla M2/2 — snapshot ugovora Report)
//! `insta::assert_json_snapshot!` serijalizira `Report` u JSON i uspoređuje ga s datotekom u
//! `tests/snapshots/`. Prvi put je zapiše (uz `INSTA_UPDATE=always`), svaki idući put je razlika PAD.
//! Tako promjena OBLIKA (novo polje, preimenovanje, drugi redoslijed) ne prolazi neopaženo, a
//! namjerna promjena je diff `.snap` datoteke u commitu s obrazloženjem (S-022). Isti fixture kao
//! paritet: paritet čuva BROJKE, snapshot čuva OBLIK — dva testa, dvije tvrdnje.
use sokratis_core::{BranchScope, Profile, ReportInput, build_report};
use std::collections::HashMap;

const STAMP: &str = "2026-09-17";

fn fx(suffix: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/sokratstudy-{STAMP}.{suffix}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("fixture {path}: {e}"))
}

#[test]
fn report_json_shape_is_locked() {
    let input = ReportInput {
        git_log: fx("log"),
        diary: Some(fx("PROGRESS.md")),
        plan: Some(fx("RASPORED.md")),
        docs: vec![],
        branches: vec![],
        overrides: HashMap::new(),
        visions: vec![],
        now: 1_789_660_685,
        today: "2026-09-17".into(),
        since: "2026-08-29".into(),
        until: None,
        branch: "main".into(),
        scope: BranchScope::DefaultBranch,
        commit_branches: HashMap::new(),
    };
    let r = build_report(&input, &Profile::default()).expect("fixture je čist ulaz");
    insta::assert_json_snapshot!("report-sokratstudy-2026-09-17", r);
}
