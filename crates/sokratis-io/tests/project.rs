//! ZAŠTO RUST OVAKO (cigla M1/17 — testovi projekta)
//! `mod common;` dijeli privremeni repo s `git_cli.rs` (isti modul, dva testna binarija), a
//! `write()` je lokalni pomoćnik za ručne JSON-datoteke. Testovi zovu samo javni API
//! (`Project::open`, `docs`, `input`); ono što čuvaju je ponašanje na RUBU — nema datoteke,
//! putanja je direktorij, tipfeler u profilu, `--since` izvan profilskog prozora.
//!
//! Cigla M2/14 (birač raspona, ograda putanja pri otvaranju, dug I9): dva nova testa čuvaju da
//! `open` odbije profil čija putanja izlazi iz repoa PRIJE nego što se icim čita, i da
//! `input_between` proslijedi `until` i jezgri (`ReportInput.until`) i gitu (s rezervom zone).
mod common;
use common::Repo;
use sokratis_core::WorkKind;
use sokratis_io::{IoError, Project, today};

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

/// Spec M2 §13.7: repo koji ne zna ni za jednu konvenciju Sokrat Studyja (nema `docs/`, dnevnika,
/// plana ni `.sokratis/`) daje brojke koje dolaze iz gita i PRAZNA stanja za sve ostalo — bez
/// greške i bez izmišljene brojke. Zadani profil (S-005) nad tuđim repoom ne smije lagati.
#[test]
fn repo_without_conventions_reports_git_numbers_and_empty_states() {
    let r = Repo::init();
    r.commit(
        "src/a.txt",
        "1\n",
        "feat: prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.commit(
        "src/a.txt",
        "1\n2\n",
        "fix: drugi",
        "2026-09-01T11:00:00+02:00",
        "2026-09-01T11:00:00+02:00",
    );
    r.commit(
        "src/b.txt",
        "x\n",
        "treci bez prefiksa",
        "2026-09-02T09:00:00+02:00",
        "2026-09-02T09:00:00+02:00",
    );

    let p = Project::open(r.path()).unwrap();
    let input = p.input(Some("2026-09-01")).unwrap();
    assert!(input.diary.is_none(), "nema dnevnika");
    assert!(input.plan.is_none(), "nema plana");
    assert!(input.docs.is_empty(), "nema docs/");

    let report = sokratis_core::build_report(&input, &p.profile).unwrap();
    assert_eq!(report.touched.commits, 3);
    assert_eq!(report.commits.len(), 3);
    assert_eq!(report.days.len(), 2);
    assert!(report.phases.is_empty(), "bez plana nema faza");
    assert!(report.deliveries.is_empty(), "bez dnevnika nema isporuka");
    assert!(report.visions.is_empty(), "bez .sokratis/ nema vizija");
    assert!(report.docs.is_none(), "bez docs/ ocjena je None, ne 0");
    for i in &report.indicators {
        assert!(
            i.value.is_finite(),
            "pokazatelj {} nije konačan broj: {}",
            i.id,
            i.value
        );
    }
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

/// M2/29b: `today()` je jedina slobodna funkcija koju desktop zove za današnji datum (bez vlastite
/// ovisnosti o `chrono`) — provjerava se oblik, ne konkretan dan (test bi inače ovisio o satu).
#[test]
fn today_is_a_valid_calendar_date() {
    assert!(
        sokratis_core::civil::is_ymd(&today()),
        "today() mora vratiti YYYY-MM-DD"
    );
}

/// M2/29b (rub I5 dalje): `git rev-parse --git-common-dir` u glavnom stablu vraća RELATIVNO
/// `.git` (pa `path_of` složi `repo` s `/` + `\.git` iz `join` — mješavina razdjelnika), a u
/// SPOREDNOM radnom stablu vraća APSOLUTNO `…/.git` (čisto `/`, bez `join`-a). `PathBuf ==` to vidi
/// kao isto (uspoređuje komponente — I5 dolje to i dokazuje), ali `sokratis-store` uspoređuje
/// TEKST (`to_string_lossy()`), pa bi isti projekt otvoren iz dva stabla ispao DVA projekta
/// (krši S-015). Test uspoređuje TEKST, jer to je ono što store zapravo vidi.
#[test]
fn common_dir_text_is_identical_from_main_and_from_a_linked_worktree() {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let side = tempfile::tempdir().unwrap();
    let wt = side.path().join("wt");
    r.git(&["worktree", "add", wt.to_str().unwrap(), "-b", "f1"]);
    let main = Project::open(r.path()).unwrap();
    let linked = Project::open(&wt).unwrap();
    assert_eq!(
        main.common_dir.to_string_lossy(),
        linked.common_dir.to_string_lossy(),
        "isti TEKST common_dir bez obzira iz kojeg se stabla otvara (store ga uspoređuje kao tekst)"
    );
    assert_eq!(
        main.main_root().to_string_lossy(),
        linked.main_root().to_string_lossy(),
        "main_root() se izvodi iz common_dir, pa nosi istu grešku ako se ne normalizira"
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

/// Dug I9 (repo JAVAN na GitHubu, cigla M2/14): profil koji navede putanju IZVAN repoa (npr.
/// `docs_dir: "../.."`) se odbija VEĆ pri `open`-u, prije nego što bilo tko pozove `docs()` ili
/// zatraži izvještaj. Ime polja (`profil.docs_dir`) i vrijednost (`../..`) moraju biti dohvatljivi
/// — ali po obrascu I5 (susjedni test iznad, `profile_error_says_the_path_once_and_leaves_the_cause_to_the_chain`)
/// SAMO kroz LANAC uzroka, ne kroz top-poruku: krug popravka 1 (recenzija) — prva verzija je
/// poruku ugradila kroz `{source}`, pa je `anyhow` na CLI-ju (`{e:#}`) ispisivao istu rečenicu
/// dvaput (top-poruka + „Caused by").
#[test]
fn profile_path_outside_repo_is_rejected_at_open_with_field_name() {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1",
        "F1/1 x",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "docs_dir": "../.." }"#);
    let err = Project::open(r.path()).expect_err("mora pasti");
    let top = err.to_string();
    assert!(
        !top.contains("docs_dir"),
        "uzrok se ne smije ugraditi u top-poruku: {top}"
    );
    let cause = std::error::Error::source(&err)
        .map(|c| c.to_string())
        .unwrap_or_default();
    assert!(
        cause.contains("profil.docs_dir") && cause.contains("../.."),
        "{cause}"
    );
}

/// M2/36 dopuna (R17, rub koji Step 3 sam stvara): repo s TOČNO jednim praznim commitom (nula
/// datoteka, nula redaka) — oblik u kojem „Dodaj projekt" prvi put donese repo koji korisnik tek
/// inicijalizirao. Ako igdje postoji dijeljenje s nulom (npr. `lines_changed.max(1.0)` bez
/// `.max`, ili prosjek nad praznim popisom faza), ovdje bi puklo u NaN/Infinity.
#[test]
fn repo_with_single_empty_commit_has_finite_numbers() {
    let r = Repo::init();
    r.commit_empty(
        "prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );

    let p = Project::open(r.path()).unwrap();
    let input = p.input(None).unwrap();
    let report = sokratis_core::build_report(&input, &p.profile).unwrap();

    assert_eq!(report.touched.commits, 1);
    assert!(
        report.phases.is_empty(),
        "bez plana i bez pogotka nema faza"
    );
    assert!(report.deliveries.is_empty(), "bez dnevnika nema isporuka");
    assert!(report.visions.is_empty(), "bez .sokratis/ nema vizija");
    assert!(report.docs.is_none(), "bez docs/ ocjena je None, ne 0");
    for i in &report.indicators {
        assert!(
            i.value.is_finite(),
            "pokazatelj {} nije konačan broj: {}",
            i.id,
            i.value
        );
    }
}

/// `input_between` prosljeđuje `until` i u `ReportInput.until` (jezgra ga presuđuje) i gitu (kao
/// gornju granicu dovlačenja, s dva dana rezerve zbog zone — `GitSource::log`). Tri commita razmaka
/// jedan dan i tri dana od `until` provjeravaju da je rezerva dovoljna za prvi, a NE i za drugi.
#[test]
fn input_between_passes_until_to_the_core_and_fetches_with_reserve() {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r.commit(
        "js/b.js",
        "1",
        "F1/2 drugi",
        "2026-09-03T10:00:00+02:00",
        "2026-09-03T10:00:00+02:00",
    );
    r.commit(
        "js/c.js",
        "1",
        "F1/3 treci",
        "2026-09-06T10:00:00+02:00",
        "2026-09-06T10:00:00+02:00",
    );
    write(&r, ".sokratis/profile.json", r#"{ "since": "2026-09-01" }"#);
    let p = Project::open(r.path()).unwrap();
    let i = p.input_between(None, Some("2026-09-03")).unwrap();
    assert_eq!(i.until.as_deref(), Some("2026-09-03"));
    assert!(i.git_log.contains("F1/2 drugi"), "dan `until` je uključen");
    assert!(
        !i.git_log.contains("F1/3 treci"),
        "tri dana kasnije je izvan rezerve od dva dana"
    );
}
