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
    r.commit(
        "docs/records/PROGRESS.md",
        "## 2026-09-01 (X) — prvi unos\n",
        "docs",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.commit(
        "js/a.js",
        "1",
        "F1/1 kod",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    write(
        &r,
        ".sokratis/profile.json",
        r#"{ "since": "2026-09-01", "owner_name": "test" }"#,
    );
    write(&r, ".sokratis/overrides.json", r#"{ "abc1234": "polish" }"#);
    write(
        &r,
        ".sokratis/visions.json",
        r#"[{"title":"V","source":"s","state":"idea","percent":null,"note":""}]"#,
    );
    write(&r, "README.md", "# r");
    let p = Project::open(r.path()).unwrap();
    assert_eq!(
        (
            p.profile.since.as_str(),
            p.profile.owner_name.as_str(),
            p.profile.default_branch.as_str()
        ),
        ("2026-09-01", "test", "main")
    );
    assert_eq!(
        p.overrides().unwrap().get("abc1234"),
        Some(&WorkKind::Polish)
    );
    assert_eq!(p.visions().unwrap()[0].title, "V");
    let docs = p.docs().unwrap();
    let mut paths: Vec<&str> = docs.iter().map(|d| d.path.as_str()).collect();
    paths.sort();
    assert_eq!(paths, ["README.md", "docs/records/PROGRESS.md"]);
    let diary = docs
        .iter()
        .find(|d| d.path == "docs/records/PROGRESS.md")
        .unwrap();
    assert!(diary.last_change_time.is_some() && diary.content.starts_with("## 2026-09-01"));
    assert_eq!(
        docs.iter()
            .find(|d| d.path == "README.md")
            .unwrap()
            .last_change_time,
        None,
        "necommitana datoteka"
    );
    let input = p.input(None).unwrap();
    assert_eq!(
        (input.branch.as_str(), input.since.as_str()),
        ("main", "2026-09-01")
    );
    assert!(input.git_log.contains("F1/1 kod"));
    assert!(input.diary.as_deref().unwrap().starts_with("## 2026-09-01"));
    assert!(input.plan.is_none());
    assert_eq!(input.today.len(), 10);
    assert_eq!(input.branches.len(), 1);
}

#[test]
fn unknown_profile_field_is_an_error_and_missing_files_default() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let p = Project::open(r.path()).unwrap();
    assert!(p.overrides().unwrap().is_empty() && p.visions().unwrap().is_empty());
    write(&r, ".sokratis/profile.json", r#"{ "sinc": "x" }"#);
    assert!(matches!(
        Project::open(r.path()).unwrap_err(),
        IoError::Profile { .. }
    ));
}
