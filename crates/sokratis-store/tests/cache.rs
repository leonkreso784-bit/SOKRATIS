// ZAŠTO OVAKO (cigla M2/18 — keš sirovih commita po SHA)
// Integracijski test (u `tests/`) vidi samo javni API cratea — `Commit`/`FileChange` iz
// `sokratis_core`, `Store` iz `sokratis_store` — isto što će vidjeti budući potrošač u `io`.

use sokratis_core::{Commit, FileChange};
use sokratis_store::{Store, StoreError};
use std::path::Path;

fn commit(
    sha: &str,
    author_time: i64,
    commit_time: i64,
    date: &str,
    commit_date: &str,
    files: Vec<FileChange>,
) -> Commit {
    Commit {
        sha: sha.into(),
        author_time,
        commit_time,
        date: date.into(),
        commit_date: commit_date.into(),
        subject: format!("commit {sha}"),
        files,
    }
}

fn fc(path: &str, added: u64, deleted: u64) -> FileChange {
    FileChange {
        path: path.into(),
        added,
        deleted,
    }
}

fn new_project(s: &Store) -> i64 {
    s.add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap()
        .id
}

#[test]
fn put_commits_is_idempotent_by_sha() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    let c = commit(
        "aaa",
        100,
        100,
        "2026-09-01",
        "2026-09-01",
        vec![fc("a.rs", 1, 0)],
    );
    assert_eq!(s.put_commits(p, std::slice::from_ref(&c)).unwrap(), 1);
    assert_eq!(
        s.put_commits(p, &[c]).unwrap(),
        0,
        "isti SHA drugi put ne broji se kao nov"
    );
}

#[test]
fn put_commits_counts_only_new_in_mixed_batch() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    let old = commit("aaa", 100, 100, "2026-09-01", "2026-09-01", vec![]);
    s.put_commits(p, std::slice::from_ref(&old)).unwrap();
    let new1 = commit("bbb", 200, 200, "2026-09-02", "2026-09-02", vec![]);
    let new2 = commit("ccc", 300, 300, "2026-09-03", "2026-09-03", vec![]);
    let n = s.put_commits(p, &[old, new1, new2]).unwrap();
    assert_eq!(n, 2, "samo dva nova u mješavini starog i novog");
}

#[test]
fn same_sha_in_two_projects_are_two_records() {
    let s = Store::open_in_memory().unwrap();
    let p1 = new_project(&s);
    let p2 = s
        .add_project("SS2", Path::new(r"C:\ss2"), Path::new(r"C:\ss2\.git"), 1)
        .unwrap()
        .id;
    let c = commit("aaa", 100, 100, "2026-09-01", "2026-09-01", vec![]);
    assert_eq!(s.put_commits(p1, std::slice::from_ref(&c)).unwrap(), 1);
    assert_eq!(
        s.put_commits(p2, &[c]).unwrap(),
        1,
        "isti SHA u drugom projektu je nov zapis -- ključ je (project_id, sha)"
    );
}

#[test]
fn put_commits_for_unknown_project_is_no_such_project() {
    let s = Store::open_in_memory().unwrap();
    let c = commit("aaa", 100, 100, "2026-09-01", "2026-09-01", vec![]);
    assert!(matches!(
        s.put_commits(999, &[c]),
        Err(StoreError::NoSuchProject(999))
    ));
}

#[test]
fn cached_commits_filters_by_commit_date_and_sorts_by_author_time() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    let c1 = commit("aaa", 300, 100, "2026-09-01", "2026-09-01", vec![]);
    let c2 = commit("bbb", 100, 200, "2026-09-02", "2026-09-02", vec![]);
    // commit_date iza date (moguće kod cherry-picka/rebasea) -- filtar gleda commit_date
    let c3 = commit("ccc", 200, 300, "2026-08-20", "2026-09-05", vec![]);
    s.put_commits(p, std::slice::from_ref(&c1)).unwrap();
    s.put_commits(p, std::slice::from_ref(&c2)).unwrap();
    s.put_commits(p, std::slice::from_ref(&c3)).unwrap();
    let got = s.cached_commits(p, "2026-09-02").unwrap();
    let shas: Vec<&str> = got.iter().map(|c| c.sha.as_str()).collect();
    assert_eq!(
        shas,
        vec!["bbb", "ccc"],
        "samo commit_date >= 2026-09-02, poredano po author_time"
    );
}

#[test]
fn cached_commits_with_empty_since_returns_all_and_ties_are_stable_by_sha() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    // dva commita s ISTIM author_time -- poredak mora biti stabilan (sekundarno po sha)
    let c1 = commit("bbb", 100, 100, "2026-09-01", "2026-09-01", vec![]);
    let c2 = commit("aaa", 100, 100, "2026-09-01", "2026-09-01", vec![]);
    s.put_commits(p, &[c1, c2]).unwrap();
    let got = s.cached_commits(p, "").unwrap();
    let shas: Vec<&str> = got.iter().map(|c| c.sha.as_str()).collect();
    assert_eq!(
        shas,
        vec!["aaa", "bbb"],
        "isti author_time -> sekundarni ključ sha, poredak nije neodređen"
    );
}

#[test]
fn cached_commits_for_unknown_project_is_empty_not_error() {
    let s = Store::open_in_memory().unwrap();
    assert_eq!(s.cached_commits(999, "").unwrap(), Vec::new());
}

#[test]
fn newest_cached_commit_date_is_max_commit_date_or_none() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    assert_eq!(s.newest_cached_commit_date(p).unwrap(), None);
    s.put_commits(
        p,
        &[
            commit("aaa", 100, 100, "2026-09-01", "2026-09-01", vec![]),
            commit("bbb", 200, 200, "2026-09-05", "2026-09-05", vec![]),
            commit("ccc", 300, 300, "2026-09-03", "2026-09-03", vec![]),
        ],
    )
    .unwrap();
    assert_eq!(
        s.newest_cached_commit_date(p).unwrap().as_deref(),
        Some("2026-09-05")
    );
}

#[test]
fn files_round_trip_including_empty_unicode_space_and_zero_counts() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    let files = vec![
        fc("dokumentacija/čitanje sa razmakom.md", 3, 1),
        fc("slika.png", 0, 0), // binarna datoteka: numstat daje "-" -> io ranije pretvara u 0/0
    ];
    let with_files = commit("aaa", 100, 100, "2026-09-01", "2026-09-01", files);
    let merge_without_numstat = commit("bbb", 200, 200, "2026-09-02", "2026-09-02", vec![]);
    s.put_commits(p, &[with_files.clone(), merge_without_numstat.clone()])
        .unwrap();
    let got = s.cached_commits(p, "").unwrap();
    assert_eq!(got, vec![with_files, merge_without_numstat]);
}

#[test]
fn removing_project_cascades_commit_cache() {
    let s = Store::open_in_memory().unwrap();
    let p = new_project(&s);
    s.put_commits(
        p,
        &[commit("aaa", 100, 100, "2026-09-01", "2026-09-01", vec![])],
    )
    .unwrap();
    s.remove_project(p).unwrap();
    assert!(s.cached_commits(p, "").unwrap().is_empty());
    assert_eq!(s.newest_cached_commit_date(p).unwrap(), None);
}
