//! ZAŠTO RUST OVAKO (cigla M2/15 — registar)
//! Testovi rade nad `:memory:` bazom: nema datoteka, nema čišćenja, svaki test svoju bazu. Jedini
//! test s diskom (`open_persists_and_migrations_are_idempotent`) dokazuje da `open` dvaput ne
//! ponavlja migracije — `tempfile::TempDir` briše mapu na kraju (RAII).
use sokratis_store::{Store, StoreError};
use std::path::{Path, PathBuf};

#[test]
fn add_list_rename_remove_and_dedupe_by_common_dir() {
    let s = Store::open_in_memory().unwrap();
    let a = s
        .add_project(
            "Sokrat Study",
            Path::new(r"C:\ss"),
            Path::new(r"C:\ss\.git"),
            100,
        )
        .unwrap();
    let b = s
        .add_project(
            "Sokratis",
            Path::new(r"C:\sk"),
            Path::new(r"C:\sk\.git"),
            101,
        )
        .unwrap();
    assert_ne!(a.id, b.id);
    let names: Vec<String> = s
        .list_projects()
        .unwrap()
        .into_iter()
        .map(|p| p.name)
        .collect();
    assert_eq!(names, vec!["Sokrat Study", "Sokratis"]);
    match s.add_project(
        "kopija",
        Path::new(r"C:\ss.f21"),
        Path::new(r"C:\ss\.git"),
        102,
    ) {
        Err(StoreError::AlreadyTracked { name }) => assert_eq!(name, "Sokrat Study"),
        other => panic!("{other:?}"),
    }
    s.rename_project(a.id, "SS").unwrap();
    assert_eq!(s.project(a.id).unwrap().name, "SS");
    s.remove_project(a.id).unwrap();
    assert!(matches!(s.project(a.id), Err(StoreError::NoSuchProject(_))));
    assert_eq!(s.list_projects().unwrap().len(), 1);
}

#[test]
fn worktrees_are_replaced_as_a_set_and_cascade_on_remove() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    s.set_worktrees(
        p.id,
        &[
            (PathBuf::from(r"C:\ss"), "main".into()),
            (PathBuf::from(r"C:\ss.f21"), "f21".into()),
        ],
        2,
    )
    .unwrap();
    assert_eq!(s.worktrees(p.id).unwrap().len(), 2);
    s.set_worktrees(p.id, &[(PathBuf::from(r"C:\ss"), "main".into())], 3)
        .unwrap();
    let w = s.worktrees(p.id).unwrap();
    assert_eq!(
        (w.len(), w[0].branch.as_str(), w[0].seen_at),
        (1, "main", 3)
    );
    s.remove_project(p.id).unwrap();
    assert!(s.worktrees(p.id).unwrap().is_empty(), "ON DELETE CASCADE");
}

#[test]
fn open_persists_and_migrations_are_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("sub").join("sokratis.db");
    {
        let s = Store::open(&db).unwrap();
        s.add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
            .unwrap();
    }
    let s = Store::open(&db).unwrap();
    assert_eq!(s.schema_version().unwrap(), 1);
    assert_eq!(s.list_projects().unwrap().len(), 1);
}
