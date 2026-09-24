//! ZAŠTO RUST OVAKO (cigla M2/14b — testovi potrošača keša)
//! `MemoryCache`/`FailingCache` su lokalni pokusni tipovi iza `dyn CommitCache` (io ne zna tko
//! stvarno pamti); `RefCell`/`Cell` daju unutarnju promjenjivost kroz `&self` jer je `CommitCache`
//! posuđen nepromjenjivo, isto kao `GitCli::calls`. `PartialGit` u testu 8 dokazuje da `cached_log`
//! ne vjeruje šutke gitu koji „zaboravi" dostižan commit — svaka metoda koja mu ne treba puca
//! s `unimplemented!()`, dopušteno u `tests/` (pravilo #6).
mod common;
use common::Repo;
use sokratis_core::parse::parse_git_log;
use sokratis_core::{BranchInfo, Commit};
use sokratis_io::{
    CacheError, CommitCache, GitCli, GitSource, IoError, Project, Scope, WorktreeHead, cached_log,
};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// Piše ručnu datoteku (npr. `.sokratis/profile.json`) u privremeni repo — isti obrazac kao u
/// `tests/project.rs`, ali svaki `tests/*.rs` cilj gradi vlastiti modul.
fn write(r: &Repo, rel: &str, content: &str) {
    let p = r.path().join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, content).unwrap();
}

/// Commit koji odjednom dira DVIJE datoteke — `Repo::commit` iz `tests/common` piše samo jednu,
/// a test hladnog keša treba jedan commit s dva `FileChange` retka.
fn commit_two_files(r: &Repo, files: [(&str, &str); 2], msg: &str, a: &str, c: &str) -> String {
    for (path, content) in files {
        let full = r.path().join(path);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(&full, content).unwrap();
    }
    r.git(&["add", "-A"]);
    let out = Command::new("git")
        .arg("-C")
        .arg(r.path())
        .env("GIT_AUTHOR_DATE", a)
        .env("GIT_COMMITTER_DATE", c)
        .args(["commit", "-q", "-m", msg])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    r.git(&["rev-parse", "--short", "HEAD"])
}

/// `git commit --amend` s FIKSNIM datumima (isti obrazac kao `Repo::commit`), da amend-testovi
/// ostanu deterministički.
fn amend(r: &Repo, msg: &str, a: &str, c: &str) {
    let out = Command::new("git")
        .arg("-C")
        .arg(r.path())
        .env("GIT_AUTHOR_DATE", a)
        .env("GIT_COMMITTER_DATE", c)
        .args(["commit", "--amend", "-q", "-m", msg])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Keš u memoriji: `store` dopisuje samo SHA-ove koje još nema i broji svaki poziv.
struct MemoryCache {
    commits: RefCell<Vec<Commit>>,
    store_calls: Cell<u32>,
}

impl MemoryCache {
    fn new() -> Self {
        Self {
            commits: RefCell::new(Vec::new()),
            store_calls: Cell::new(0),
        }
    }
}

impl CommitCache for MemoryCache {
    fn cached(&self) -> Result<Vec<Commit>, CacheError> {
        Ok(self.commits.borrow().clone())
    }

    fn store(&self, commits: &[Commit]) -> Result<(), CacheError> {
        self.store_calls.set(self.store_calls.get() + 1);
        let mut all = self.commits.borrow_mut();
        let known: std::collections::HashSet<String> = all.iter().map(|c| c.sha.clone()).collect();
        for c in commits {
            if !known.contains(&c.sha) {
                all.push(c.clone());
            }
        }
        Ok(())
    }
}

/// Keš koji uvijek puca — dokazuje da `cached_log` grešku PROPUŠTA, ne guta.
struct FailingCache;

impl CommitCache for FailingCache {
    fn cached(&self) -> Result<Vec<Commit>, CacheError> {
        Err("kes: citanje puklo".into())
    }

    fn store(&self, _commits: &[Commit]) -> Result<(), CacheError> {
        Err("kes: pisanje puklo".into())
    }
}

/// Lažni `GitSource` (test 8): `rev_list` javi DVA dostižna SHA-a, ali `log_commits` vrati tekst
/// SAMO za prvi — dokazuje da `cached_log` manjak prijavi kao grešku, ne kao manji izvještaj.
/// Sve ostale metode traita puknu s `unimplemented!()` jer `cached_log` do njih ne smije doći.
struct PartialGit;

impl GitSource for PartialGit {
    fn log(
        &self,
        _scope: Scope<'_>,
        _since: &str,
        _until: Option<&str>,
    ) -> Result<String, IoError> {
        unimplemented!()
    }
    fn branches(&self, _default_branch: &str) -> Result<Vec<BranchInfo>, IoError> {
        unimplemented!()
    }
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError> {
        unimplemented!()
    }
    fn worktree_heads(&self) -> Result<Vec<WorktreeHead>, IoError> {
        unimplemented!()
    }
    fn commit_times(&self, _shas: &[String]) -> Result<HashMap<String, i64>, IoError> {
        unimplemented!()
    }
    fn last_change(&self, _path: &str) -> Result<Option<i64>, IoError> {
        unimplemented!()
    }
    fn last_changes(
        &self,
        _rev: &str,
        _pathspecs: &[&str],
    ) -> Result<HashMap<String, i64>, IoError> {
        unimplemented!()
    }
    fn common_dir(&self) -> Result<PathBuf, IoError> {
        unimplemented!()
    }
    fn toplevel(&self) -> Result<PathBuf, IoError> {
        unimplemented!()
    }
    fn branch_exists(&self, _name: &str) -> Result<bool, IoError> {
        unimplemented!()
    }
    fn current_branch(&self) -> Result<String, IoError> {
        unimplemented!()
    }
    fn head_sha(&self) -> Result<String, IoError> {
        unimplemented!()
    }
    fn rev_list(
        &self,
        _scope: Scope<'_>,
        _since: &str,
        _until: Option<&str>,
    ) -> Result<Vec<String>, IoError> {
        Ok(vec!["aaa1111".to_string(), "bbb2222".to_string()])
    }
    fn commit_sources(
        &self,
        _default_ref: &str,
        _since: &str,
        _until: Option<&str>,
    ) -> Result<HashMap<String, String>, IoError> {
        unimplemented!("test ga ne zove")
    }
    fn log_commits(&self, _shas: &[String]) -> Result<String, IoError> {
        Ok("@@aaa1111|1|1|2026-09-01|2026-09-01|samo prvi\n".to_string())
    }
}

#[test]
fn cold_cache_gives_the_same_commits_as_plain_log_and_fills_the_cache() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    commit_two_files(
        &r,
        [("b.txt", "1\n2\n"), ("c.txt", "3\n")],
        "F1/2 dvije datoteke",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    r.commit(
        "docs/čćž.md",
        "sadrzaj\n",
        "F1/3 ne-ascii",
        "2026-09-03T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let plain = p.input_between(None, None).unwrap();
    let cache = MemoryCache::new();
    let cached = p.input_cached(None, None, &cache).unwrap();
    assert_eq!(
        parse_git_log(&plain.git_log).unwrap().commits,
        parse_git_log(&cached.git_log).unwrap().commits,
        "isti commiti bez obzira dolazi li log iz gita ili iz keša"
    );
    assert_eq!(cache.commits.borrow().len(), 3, "kes drzi sva tri commita");
}

#[test]
fn warm_cache_fetches_only_what_is_new() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.commit(
        "b.txt",
        "2",
        "F1/2 drugi",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    r.commit(
        "c.txt",
        "3",
        "F1/3 treci",
        "2026-09-03T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let cache = MemoryCache::new();
    p.input_cached(None, None, &cache).unwrap(); // zagrijavanje puni kes -> store_calls == 1
    assert_eq!(cache.commits.borrow().len(), 3);
    let store_calls_after_warmup = cache.store_calls.get();

    let before = p.git.calls();
    p.input_cached(None, None, &cache).unwrap();
    let spawned = p.git.calls() - before;
    assert_eq!(
        cache.store_calls.get(),
        store_calls_after_warmup,
        "nema novih commita: store se NE zove ponovo"
    );
    // M2/49: zadani profil (nema `.sokratis/profile.json` s `branch_scope`) je `all` (T48) — svaki
    // `input_with` plaća JEDAN dodatan proces (`commit_sources`) bez obzira na toplinu keša, jer
    // karta `sha → grana` nije dio keša commita. Granica je zato +1 u odnosu na prijašnjih 4.
    // M2/50 (S-033, odstupanje od brifa): `input_with` sad UVIJEK plaća i DVA procesa za vodeće
    // stablo (`worktree_heads` + `commit_times`), bez obzira na toplinu keša — nije dio keša
    // commita niti opsega grana. Izmjereno: 7 (5+2), ne 5 kako je pisalo prije T50.
    assert!(spawned <= 7, "topao poziv bez novih commita: {spawned}");

    r.commit(
        "d.txt",
        "4",
        "F1/4 cetvrti",
        "2026-09-04T10:00:00+02:00",
        "2026-09-04T10:00:00+02:00",
    );
    let before = p.git.calls();
    p.input_cached(None, None, &cache).unwrap();
    let spawned = p.git.calls() - before;
    assert_eq!(cache.commits.borrow().len(), 4, "novi commit ulazi u kes");
    assert_eq!(
        cache.store_calls.get(),
        store_calls_after_warmup + 1,
        "jedan novi commit: tocno jedan novi poziv store"
    );
    // M2/49: isti +1 kao gore, jer `commit_sources` nije dio keša — mjerodavno je +1 na 5.
    // M2/50 (S-033, odstupanje od brifa): isti +2 kao gore (vodeće stablo) — mjerodavno je 8 (6+2).
    assert!(
        spawned <= 8,
        "topao poziv s jednim novim commitom: {spawned}"
    );
}

/// OBAVEZAN (brif): amend ostavlja stari SHA kao smeće u kešu, ali on NIKAD ne smije ući u
/// izvještaj — `rev_list` ga više ne vidi kao dostižan.
#[test]
fn amended_commit_stays_in_the_cache_but_never_reaches_the_report() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let old_sha = r.commit(
        "b.txt",
        "1",
        "F1/2 stari naslov",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let cache = MemoryCache::new();
    p.input_cached(None, None, &cache).unwrap(); // zagrijavanje
    assert_eq!(cache.commits.borrow().len(), 2);

    amend(
        &r,
        "F1/2 novi naslov",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );

    let input = p.input_cached(None, None, &cache).unwrap();
    assert!(input.git_log.contains("F1/2 novi naslov"));
    assert!(!input.git_log.contains("F1/2 stari naslov"));
    assert!(
        !input.git_log.contains(&old_sha),
        "stari SHA se ne smije pojaviti u izvjestaju"
    );
    let commits = parse_git_log(&input.git_log).unwrap().commits;
    assert_eq!(
        commits.len(),
        2,
        "amend ne mijenja broj commita u izvjestaju"
    );
    assert_eq!(
        cache.commits.borrow().len(),
        3,
        "stari SHA ostaje u kesu kao smece, ne u brojkama"
    );
}

/// OBAVEZAN (brif): `git reset --hard` briše commit iz povijesti — dostupan je i dalje u kešu
/// (smeće), ali izvještaj ga NE smije sadržavati.
#[test]
fn dropped_commit_never_reaches_the_report() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.commit(
        "b.txt",
        "1",
        "F1/2 drugi",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    r.commit(
        "c.txt",
        "1",
        "F1/3 treci",
        "2026-09-03T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let cache = MemoryCache::new();
    p.input_cached(None, None, &cache).unwrap(); // zagrijavanje
    assert_eq!(cache.commits.borrow().len(), 3);

    r.git(&["reset", "--hard", "HEAD~1"]);

    let input = p.input_cached(None, None, &cache).unwrap();
    let parsed = parse_git_log(&input.git_log).unwrap();
    let subjects: Vec<&str> = parsed.commits.iter().map(|c| c.subject.as_str()).collect();
    assert_eq!(subjects, ["F1/1 prvi", "F1/2 drugi"]);
    assert_eq!(
        cache.commits.borrow().len(),
        3,
        "ispusteni commit ostaje u kesu, ali van dohvata reset-a se vise ne racuna"
    );
}

#[test]
fn window_is_decided_by_rev_list_not_by_the_cache() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.commit(
        "b.txt",
        "1",
        "F1/2 drugi",
        "2026-09-03T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    r.commit(
        "c.txt",
        "1",
        "F1/3 treci",
        "2026-09-06T10:00:00+02:00",
        "2026-09-06T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let cache = MemoryCache::new();
    p.input_cached(None, None, &cache).unwrap(); // zagrijavanje PUNIM rasponom
    assert_eq!(cache.commits.borrow().len(), 3);

    let input = p.input_cached(None, Some("2026-09-03"), &cache).unwrap();
    assert!(input.git_log.contains("F1/2 drugi"));
    assert!(
        !input.git_log.contains("F1/3 treci"),
        "treci commit je u kesu, ali izvan trazenog prozora"
    );
}

#[test]
fn rev_list_matches_log_in_shas_and_order() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-05T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.git(&["checkout", "-q", "-b", "feat/x"]);
    r.commit(
        "b.txt",
        "1",
        "F1/2 grana",
        "2026-09-01T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    r.git(&["checkout", "-q", "main"]);
    r.commit(
        "c.txt",
        "1",
        "F1/3 main",
        "2026-09-06T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    r.try_merge("feat/x");
    r.commit_merge(
        "F1/4 merge",
        "2026-09-04T10:00:00+02:00",
        "2026-09-04T10:00:00+02:00",
    );
    let g = GitCli::new(r.path());
    let shas = g
        .rev_list(Scope::Branch("main"), "2026-09-01", None)
        .unwrap();
    let log = g.log(Scope::Branch("main"), "2026-09-01", None).unwrap();
    let log_shas: Vec<&str> = log
        .lines()
        .filter_map(|l| l.strip_prefix("@@"))
        .filter_map(|rest| rest.split('|').next())
        .collect();
    assert_eq!(shas, log_shas);
    assert_eq!(shas.len(), 4, "tri commita + jedan merge");
}

#[test]
fn cache_error_is_visible_not_swallowed() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let err = p.input_cached(None, None, &FailingCache).unwrap_err();
    assert!(matches!(err, IoError::Cache(_)), "{err}");
}

#[test]
fn git_that_omits_a_reachable_commit_is_an_error_not_a_smaller_report() {
    let git = PartialGit;
    let cache = MemoryCache::new();
    let err = cached_log(&git, &cache, Scope::Branch("main"), "2026-09-01", None).unwrap_err();
    match err {
        IoError::CacheIncomplete { sha } => assert_eq!(sha, "bbb2222"),
        other => panic!("ocekivan IoError::CacheIncomplete, dobiven {other}"),
    }
}

#[test]
fn log_commits_of_nothing_spawns_no_process() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let g = GitCli::new(r.path());
    let before = g.calls();
    let text = g.log_commits(&[]).unwrap();
    assert_eq!(text, "");
    assert_eq!(g.calls(), before, "prazan popis ne pokrece nijedan proces");
}

/// Mjerenje (Step 6 brifa), SAMO ČITANJE: putanja dolazi ISKLJUČIVO iz `SOKRATIS_PERF_REPO` —
/// repo je JAVAN, nijedna apsolutna putanja se ne smije naći u kodu.
#[test]
#[ignore]
fn measure_cached_input_over_a_real_repo() {
    let Ok(repo_path) = std::env::var("SOKRATIS_PERF_REPO") else {
        eprintln!("preskoceno: SOKRATIS_PERF_REPO nije postavljena");
        return;
    };
    let p = Project::open(std::path::Path::new(&repo_path)).unwrap();

    // Zagrijavanje: neizmjereno, u SVOJ kes koji se odmah baca.
    p.input_cached(None, None, &MemoryCache::new()).unwrap();

    let t = std::time::Instant::now();
    let plain = p.input_between(None, None).unwrap();
    let a_ms = t.elapsed().as_millis();

    let cache = MemoryCache::new();
    let before = p.git.calls();
    let t = std::time::Instant::now();
    let _cold = p.input_cached(None, None, &cache).unwrap();
    let b_ms = t.elapsed().as_millis();
    let cold_calls = p.git.calls() - before;

    let mut warm_best_ms = u128::MAX;
    let mut warm_calls = 0u32;
    let mut warm_input = plain.clone();
    for _ in 0..3 {
        let before = p.git.calls();
        let t = std::time::Instant::now();
        let input = p.input_cached(None, None, &cache).unwrap();
        let ms = t.elapsed().as_millis();
        let calls = p.git.calls() - before;
        if ms < warm_best_ms {
            warm_best_ms = ms;
            warm_calls = calls;
            warm_input = input;
        }
    }

    let t = std::time::Instant::now();
    let report_warm = sokratis_core::build_report(&warm_input, &p.profile).unwrap();
    let d_ms = t.elapsed().as_millis();
    let _ = &report_warm;

    eprintln!(
        "mjerenje: a={a_ms}ms (bez kesa) b={b_ms}ms/{cold_calls}proc (hladan kes) c={warm_best_ms}ms/{warm_calls}proc (topao kes, najbolji od 3) d={d_ms}ms (build_report) c+d={}ms",
        warm_best_ms + d_ms
    );
    eprintln!(
        "cilj topao kes + build_report < 500ms: {}",
        if warm_best_ms + d_ms < 500 {
            "DA"
        } else {
            "NE"
        }
    );

    let plain_commits = parse_git_log(&plain.git_log).unwrap().commits;
    let warm_commits = parse_git_log(&warm_input.git_log).unwrap().commits;
    assert_eq!(
        plain_commits, warm_commits,
        "isti commiti bez obzira dolazi li log iz gita ili iz keša"
    );

    let mut warm_aligned = warm_input.clone();
    warm_aligned.now = plain.now;
    warm_aligned.today.clone_from(&plain.today);
    let report_plain = sokratis_core::build_report(&plain, &p.profile).unwrap();
    let report_warm_aligned = sokratis_core::build_report(&warm_aligned, &p.profile).unwrap();
    assert_eq!(
        serde_json::to_string(&report_plain).unwrap(),
        serde_json::to_string(&report_warm_aligned).unwrap(),
        "isti JSON izvjestaja iz hladnog i toplog ulaza (uz poravnate now/today)"
    );
}
