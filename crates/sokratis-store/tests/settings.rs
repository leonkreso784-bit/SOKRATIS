//! ZAŠTO RUST OVAKO (cigla M2/16 — postavke)
//! Isti obrazac kao `tests/registry.rs`: svaki test svoju `:memory:` bazu, bez čišćenja. Drugi test
//! pokriva slučaj o kojem brif šuti (upis postavke na projekt koji ne postoji) — orkestratorova
//! odluka je da to bude tipizirana `NoSuchProject`, ne sirova SQLite greška stranog ključa.
use sokratis_store::{Store, StoreError};
use std::path::Path;

#[test]
fn settings_upsert_and_read_back() {
    let s = Store::open_in_memory().unwrap();
    assert_eq!(s.setting("theme").unwrap(), None);
    s.set_setting("theme", "chalk").unwrap();
    s.set_setting("theme", "mint").unwrap();
    assert_eq!(s.setting("theme").unwrap().as_deref(), Some("mint"));
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    s.set_project_setting(p.id, "range", "30d").unwrap();
    assert_eq!(
        s.project_setting(p.id, "range").unwrap().as_deref(),
        Some("30d")
    );
    assert_eq!(s.project_setting(p.id, "last_view").unwrap(), None);
    s.remove_project(p.id).unwrap();
    assert_eq!(s.project_setting(p.id, "range").unwrap(), None, "cascade");
}

#[test]
fn set_project_setting_on_missing_project_is_typed_error() {
    let s = Store::open_in_memory().unwrap();
    match s.set_project_setting(999, "range", "30d") {
        Err(StoreError::NoSuchProject(999)) => {}
        other => panic!("{other:?}"),
    }
}
