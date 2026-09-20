//! ZAŠTO RUST OVAKO (cigla M2/10 — pisanje ručnih podataka)
//! Testovi gledaju DISK, ne povratnu vrijednost: atomarnost znači „nema `.tmp` ostatka", a oblik
//! znači „`git diff` pokazuje jedan redak" — pa se uspoređuje doslovan tekst datoteke.
mod common;
use common::Repo;
use sokratis_core::{Vision, WorkKind};
use sokratis_io::Project;

fn repo() -> Repo {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1\n",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r
}

#[test]
fn override_is_written_pretty_sorted_and_removable() {
    let r = repo();
    let p = Project::open(r.path()).unwrap();
    let path = p.write_override("b2", Some(WorkKind::Debugging)).unwrap();
    p.write_override("a1", Some(WorkKind::Polish)).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert_eq!(text, "{\n  \"a1\": \"polish\",\n  \"b2\": \"debugging\"\n}");
    assert!(
        !path.with_extension("json.tmp").exists(),
        "privremena datoteka ne smije ostati"
    );
    p.write_override("b2", None).unwrap();
    assert_eq!(p.overrides().unwrap().len(), 1);
    assert_eq!(p.overrides().unwrap().get("a1"), Some(&WorkKind::Polish));
}

#[test]
fn visions_round_trip() {
    let r = repo();
    let p = Project::open(r.path()).unwrap();
    let v = vec![Vision {
        title: "V".into(),
        source: "s".into(),
        state: "idea".into(),
        percent: Some(10),
        note: "n".into(),
    }];
    p.write_visions(&v).unwrap();
    assert_eq!(p.visions().unwrap(), v);
}

#[test]
fn writes_from_a_linked_worktree_land_in_the_main_worktree() {
    let r = repo();
    let side = tempfile::tempdir().unwrap();
    let wt = side.path().join("wt");
    r.git(&["worktree", "add", wt.to_str().unwrap(), "-b", "f1"]);
    let p = Project::open(&wt).unwrap();
    assert_eq!(
        p.main_root().components().collect::<Vec<_>>(),
        r.path().components().collect::<Vec<_>>()
    );
    let path = p.write_override("a1", Some(WorkKind::Planning)).unwrap();
    assert!(path.starts_with(r.path()), "{path:?} nije u glavnom stablu");
    assert!(!wt.join(".sokratis").exists(), "u sporednom stablu ništa");
    assert_eq!(
        p.overrides().unwrap().get("a1"),
        Some(&WorkKind::Planning),
        "čitanje ide iz istog mjesta"
    );
}
