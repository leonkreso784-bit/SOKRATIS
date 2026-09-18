//! ZAŠTO RUST OVAKO (cigla M1/17 — testovi projekta)
//! `mod common;` dijeli privremeni repo s `git_cli.rs` (isti modul, dva testna binarija), a
//! `write()` je lokalni pomoćnik za ručne JSON-datoteke. Testovi zovu samo javni API
//! (`Project::open`, `docs`, `input`); ono što čuvaju je ponašanje na RUBU — nema datoteke,
//! putanja je direktorij, tipfeler u profilu, `--since` izvan profilskog prozora.
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

/// I1 (završna recenzija M1): `--since` stariji od profilskog prozora se tiho odrezao — log je
/// dolazio samo od `profile.log_since()`, a `Report.since` je ipak tvrdio korisnikov datum
/// (nad Sokrat Studyjem: `--since 2026-01-01`, a prvi dan u tablici 2026-08-02).
#[test]
fn cli_since_older_than_the_profile_widens_the_fetch_window() {
    let r = Repo::init();
    let old = r.commit(
        "js/a.js",
        "1",
        "F1/1 staro",
        "2026-07-01T10:00:00+02:00",
        "2026-07-01T10:00:00+02:00",
    );
    r.commit(
        "js/b.js",
        "1",
        "F1/2 novo",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    // Bez `closed_phases` prozor profila je točno `since` (zadani profil bi ga razvukao na
    // 2026-08-02, početak prve povijesne faze Sokrat Studyja).
    write(
        &r,
        ".sokratis/profile.json",
        r#"{ "since": "2026-08-29", "closed_phases": [] }"#,
    );
    let p = Project::open(r.path()).unwrap();
    assert!(
        !p.input(None).unwrap().git_log.contains(&old),
        "bez `--since` prozor ostaje profilski"
    );
    let wider = p.input(Some("2026-07-01")).unwrap();
    assert!(
        wider.git_log.contains(&old),
        "`--since` stariji od profila mora dovući i stariji commit"
    );
    assert_eq!(wider.since, "2026-07-01");
}

#[test]
fn common_dir_is_the_same_from_root_and_from_a_subdirectory() {
    let r = Repo::init();
    r.commit(
        "docs/records/PROGRESS.md",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let from_root = Project::open(r.path()).unwrap().common_dir;
    let from_subdir = Project::open(&r.path().join("docs").join("records"))
        .unwrap()
        .common_dir;
    // BEZ `canonicalize()`: `canonicalize()` sam razrješava `..`, pa bi ova provjera prošla i sa
    // starim (krivim) kodom koji je `common_dir` računao iz korisnikove (pod)putanje umjesto iz
    // `root`-a (nalaz recenzenta, krug popravka 1) — sirova usporedba i odsutnost `..` komponente
    // stvarno čuvaju popravak.
    assert_eq!(
        from_root, from_subdir,
        "isti projekt bez obzira odakle se otvara"
    );
    assert!(
        !from_subdir
            .components()
            .any(|c| c == std::path::Component::ParentDir),
        "common_dir iz podmape ne smije sadrzavati '..': {from_subdir:?}"
    );
}

#[test]
fn profile_that_cannot_be_read_is_an_error_not_a_default() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    // `.sokratis/profile.json` je DIREKTORIJ, ne datoteka — čitanje puca s PermissionDenied
    // (Windows) odn. IsADirectory (Unix), NIKAD NotFound; to NIJE „nema datoteke".
    std::fs::create_dir_all(r.path().join(".sokratis").join("profile.json")).unwrap();
    assert!(
        Project::open(r.path()).is_err(),
        "kriva greška se ne smije pretvoriti u Profile::default()"
    );
}

/// I5 (završna recenzija M1): `#[error("profil {path}: {source}")]` je uzrok već ugradio u svoj
/// `Display`, a `main` ispisuje `{e:#}` (anyhow doda cijeli `#[source]` lanac) — pa je serde-ova
/// poruka s popisom svih 38 polja izlazila DVAPUT (~1,6 kB). Uzrok pripada lancu, ne poruci.
/// Uz to: putanja je miješala `/` (dolazi iz gita) i `\` (dodaje `join`).
#[test]
fn profile_error_says_the_path_once_and_leaves_the_cause_to_the_chain() {
    let r = Repo::init();
    r.commit(
        "a.txt",
        "1",
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "sinc": "x" }"#);
    let err = Project::open(r.path()).unwrap_err();
    let top = err.to_string();
    assert!(top.contains("profil"), "{top}");
    assert!(
        !top.contains("unknown field"),
        "uzrok se ne smije ugraditi u poruku: {top}"
    );
    let cause = std::error::Error::source(&err)
        .map(|c| c.to_string())
        .unwrap_or_default();
    assert!(
        cause.contains("unknown field"),
        "uzrok mora ostati u lancu: {cause}"
    );
    assert!(
        !(top.contains('/') && top.contains('\\')),
        "putanja ne smije mijesati / i \\: {top}"
    );
}

/// M2/12: detached HEAD (grane nema, commita ima) je do sada davao `NoCommits` — laž, jer
/// commiti postoje. Kad zadana grana ne postoji I `current_branch()` je prazan (detached), log
/// se čita s `HEAD`, a oznaka u izvještaju postaje `HEAD@<sha>`.
#[test]
fn detached_head_with_commits_is_reported_as_head_at_sha_not_as_empty_repo() {
    let r = Repo::init();
    let sha = r.commit(
        "js/a.js",
        "1",
        "F1/1 x",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    write(
        &r,
        ".sokratis/profile.json",
        r#"{ "default_branch": "nema", "since": "2026-09-01" }"#,
    );
    r.git(&["checkout", "-q", "--detach"]);
    let p = Project::open(r.path()).unwrap();
    let input = p.input(None).unwrap();
    assert_eq!(input.branch, format!("HEAD@{sha}"));
    assert!(input.git_log.contains(&sha));
}

/// Izvan brifa, dodano: detached HEAD kad ZADANA grana i dalje POSTOJI ne smije preskočiti na
/// `HEAD@<sha>` — metrike su definirane nad zadanom granom (paritet s tablicom), pa se čita ona,
/// bez obzira gdje trenutno pokazuje HEAD.
#[test]
fn detached_head_with_default_branch_present_still_reads_the_default_branch() {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1",
        "F1/1 x",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    r.git(&["checkout", "-q", "--detach"]);
    let p = Project::open(r.path()).unwrap();
    let input = p.input(None).unwrap();
    assert_eq!(
        input.branch, "main",
        "zadana grana postoji, oznaka ostaje njezino ime"
    );
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
