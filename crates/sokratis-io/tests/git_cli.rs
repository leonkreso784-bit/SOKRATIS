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

#[test]
fn branches_report_age_ahead_and_merged() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.git(&["checkout", "-q", "-b", "feat/x"]);
    r.commit(
        "b.txt",
        "2",
        "F1/1 cigla",
        "2026-09-03T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    r.commit(
        "c.txt",
        "3",
        "F1/2 cigla",
        "2026-09-04T10:00:00+02:00",
        "2026-09-04T10:00:00+02:00",
    );
    r.git(&["checkout", "-q", "-b", "merged/y", "main"]);
    r.git(&["checkout", "-q", "main"]);
    let g = GitCli::new(r.path());
    let mut b = g.branches("main").unwrap();
    b.sort_by(|x, y| x.name.cmp(&y.name));
    let names: Vec<&str> = b.iter().map(|x| x.name.as_str()).collect();
    assert_eq!(names, ["feat/x", "main", "merged/y"]);
    let fx = &b[0];
    assert_eq!((fx.ahead_of_default, fx.merged), (2, false));
    // NAPOMENA: brif navodi 1788854400 + 8*3600, ali `date -u -d "2026-09-04T10:00:00+02:00" +%s`
    // (i python `datetime.fromisoformat(...).timestamp()`) daju 1788508800 — izračunato, ne pogađano.
    assert_eq!(fx.last_commit_time, 1788508800, "2026-09-04T10:00+02:00");
    assert_eq!((b[1].ahead_of_default, b[1].merged), (0, true));
    assert!(b[2].merged, "grana na istom commitu kao main je spojena");
}

#[test]
fn worktrees_share_common_dir() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let wt = tempfile::tempdir().unwrap();
    let wt_path = wt.path().join("stablo");
    r.git(&[
        "worktree",
        "add",
        "-q",
        wt_path.to_str().unwrap(),
        "-b",
        "feat/wt",
    ]);
    let g = GitCli::new(r.path());
    let list = g.worktrees().unwrap();
    assert_eq!(list.len(), 2);
    let a = GitCli::new(r.path())
        .common_dir()
        .unwrap()
        .canonicalize()
        .unwrap();
    let b = GitCli::new(&wt_path)
        .common_dir()
        .unwrap()
        .canonicalize()
        .unwrap();
    assert_eq!(a, b, "isti projekt");
    r.git(&["worktree", "remove", "--force", wt_path.to_str().unwrap()]);
}

#[test]
fn last_change_of_path() {
    let r = Repo::init();
    r.commit(
        "docs/PROGRESS.md",
        "1",
        "docs",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    r.commit(
        "js/a.js",
        "1",
        "kod",
        "2026-09-05T10:00:00+02:00",
        "2026-09-05T10:00:00+02:00",
    );
    let g = GitCli::new(r.path());
    // NAPOMENA: brif navodi 1788681600 + 8*3600, ali izračunato (`date -u -d …`) je 1788336000.
    assert_eq!(g.last_change("docs/PROGRESS.md").unwrap(), Some(1788336000));
    assert_eq!(g.last_change("nema.md").unwrap(), None);
}
