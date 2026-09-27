//! ZAŠTO RUST OVAKO (cigla M2/11 — performanse kao mjerenje)
//! Pravilo #4: prvo brojka, onda popravak. `Cell<u32>` u `GitCli` broji procese kroz `&self` —
//! unutarnja promjenjivost bez `mut`, jer `GitSource` metode posuđuju nepromjenjivo. Test tvrdi
//! GORNJU GRANICU poziva, ne vrijeme: vrijeme ovisi o stroju, broj procesa ne.
//!
//! Cigla M2/52 (mjerenje nad PRAVIM repoom — Sokrat Study): `measure_real_repo_both_scopes` je
//! `#[ignore]` jer ovisi o stroju (repozitorij mora postojati na disku); pokreće se ručno preko
//! `SOKRATIS_MEASURE_REPO` (v. doc-komentar testa niže).
//!
//! Dopunjeno M2/64 (S-038): `branch_graph` (pa i `commit_parents`) se pita SAMO kad postoji bar
//! jedna nespojena grana OSIM zadane — fixture ovog testa ima 30 grana na ISTOM commitu kao `main`
//! (sve `merged`), pa proces se NE pokreće i broj procesa ostaje 7 (granica i dalje 8, rezerva 1).
mod common;
use common::Repo;
use sokratis_io::Project;
use std::time::Instant;

#[test]
fn input_over_sixty_docs_and_thirty_branches_spawns_at_most_eight_git_processes() {
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
        spawned <= 8,
        "input() je pokrenuo {spawned} git procesa, dopušteno 8 — 4 osnovna \
         (log, branches, docs last_changes, HEAD/branch_exists rub) + 1 commit_sources (M2/49) \
         + 2 vodeće stablo (M2/50: worktree list + commit_times) + 1 lanci (M2/64: commit_parents, \
         uvjetno — 30 grana ovog fixturea su SVE na commitu main-a, pa se ovdje NE pokreće)"
    );
}

/// `SOKRATIS_MEASURE_REPO=C:/Users/leonk/Documents/sokratstudy.dev cargo test -p sokratis-io --test perf -- --ignored --nocapture`
#[test]
#[ignore]
fn measure_real_repo_both_scopes() {
    let Some(path) = std::env::var_os("SOKRATIS_MEASURE_REPO") else {
        eprintln!("SOKRATIS_MEASURE_REPO nije zadan — preskačem");
        return;
    };
    for scope in ["all", "default"] {
        let mut p = Project::open(std::path::Path::new(&path)).unwrap();
        p.profile.branch_scope = if scope == "all" {
            sokratis_core::BranchScope::AllBranches
        } else {
            sokratis_core::BranchScope::DefaultBranch
        };
        let before = p.git.calls();
        let t = Instant::now();
        let input = p.input(None).unwrap();
        let elapsed = t.elapsed();
        let r = sokratis_core::build_report(&input, &p.profile).unwrap();
        eprintln!(
            "scope={scope}: {} git procesa, {:?}, commita {}, grana {}, isporuka {}, stabala {}, dnevnika {}",
            p.git.calls() - before,
            elapsed,
            r.touched.commits,
            r.branches.len(),
            r.deliveries.len(),
            r.touched.worktrees,
            r.touched.diaries
        );
        assert!(p.git.calls() - before <= 8);
    }
}
