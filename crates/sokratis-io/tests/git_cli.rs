mod common;
use common::Repo;
use sokratis_io::{GitCli, GitSource, IoError};

#[test]
fn log_has_fixture_format_and_since_filters_by_commit_date() {
    let r = Repo::init();
    let old = r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-08-01T10:00:00+02:00",
        "2026-08-01T10:00:00+02:00",
    );
    let cp = r.commit(
        "b.txt",
        "2\n3\n",
        "cherry",
        "2026-09-06T05:16:40+02:00",
        "2026-09-08T21:19:43+02:00",
    );
    let g = GitCli::new(r.path());
    let log = g.log("main", "2026-08-29").unwrap();
    assert!(
        !log.contains(&old),
        "commit s committer-datumom prije since otpada"
    );
    let head = log.lines().find(|l| l.starts_with("@@")).unwrap();
    // NAPOMENA: brif navodi %ct = 1788817183, ali `date -u -d "2026-09-08T21:19:43+02:00" +%s`
    // (i python `datetime.fromisoformat(...).timestamp()`) daju 1788895183 — izračunato, ne pogađano.
    assert!(
        head.starts_with(&format!(
            "@@{cp}|1788664600|1788895183|2026-09-06|2026-09-08|cherry"
        )),
        "{head}"
    );
    assert!(log.contains("2\t0\tb.txt"));
}

#[test]
fn toplevel_common_dir_and_branches() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let g = GitCli::new(r.path());
    assert_eq!(g.current_branch().unwrap(), "main");
    assert!(g.branch_exists("main").unwrap() && !g.branch_exists("nema").unwrap());
    assert_eq!(
        g.toplevel().unwrap().canonicalize().unwrap(),
        r.path().canonicalize().unwrap()
    );
    assert!(g.common_dir().unwrap().ends_with(".git"));
}

#[test]
fn not_a_repo_is_a_typed_error() {
    let dir = tempfile::tempdir().unwrap();
    let err = GitCli::new(dir.path()).toplevel().unwrap_err();
    assert!(matches!(err, IoError::NotARepo(_)), "{err}");
}
