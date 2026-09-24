//! ZAŠTO RUST OVAKO (cigla M2/11 — performanse kao mjerenje)
//! Pravilo #4: prvo brojka, onda popravak. `Cell<u32>` u `GitCli` broji procese kroz `&self` —
//! unutarnja promjenjivost bez `mut`, jer `GitSource` metode posuđuju nepromjenjivo. Test tvrdi
//! GORNJU GRANICU poziva, ne vrijeme: vrijeme ovisi o stroju, broj procesa ne.
mod common;
use common::Repo;
use sokratis_io::Project;
use std::time::Instant;

#[test]
fn input_over_sixty_docs_and_thirty_branches_spawns_at_most_six_git_processes() {
    let r = Repo::init();
    for i in 0..60 {
        std::fs::create_dir_all(r.path().join("docs")).unwrap();
        std::fs::write(
            r.path().join(format!("docs/d{i:02}.md")),
            format!("# {i}\n"),
        )
        .unwrap();
    }
    r.git(&["add", "-A"]);
    r.commit(
        "js/a.js",
        "1\n",
        "F1/1 docs i kod",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    for i in 0..30 {
        r.git(&["branch", &format!("b{i}")]);
    }
    let p = Project::open(r.path()).unwrap();
    let before = p.git.calls();
    let t = Instant::now();
    let input = p.input(None).unwrap();
    let spawned = p.git.calls() - before;
    eprintln!("perf: {spawned} git procesa, {:?}", t.elapsed());
    assert_eq!(input.docs.len(), 60);
    assert_eq!(input.branches.len(), 31);
    assert!(
        spawned <= 6,
        "input() je pokrenuo {spawned} git procesa, dopušteno 6 — +1 `commit_sources` (M2/49)"
    );
}
