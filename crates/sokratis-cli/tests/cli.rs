//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI naredbe i izlazni kodovi)
//! Test pokreće pravi binarni proces (`CARGO_BIN_EXE_sokratis`), ne poziva funkcije iz crate-a:
//! ono što se ovdje provjerava JE izlazni kod i tekst na stderr — ugovor prema preflightu koji
//! čita samo proces, ne Rust API. `assert_eq!` na `.status.code()` je zato jedini ispravan alat.
mod common;
use common::Repo;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sokratis"))
}

/// Repozitorij s jednim commitom na `main` i bez ijedne `.md` datoteke: zadani profil (S-005),
/// nema docs-a, nema tuđih grana — dakle nema ni jednog signala.
fn repo_with_one_commit_on_main() -> Repo {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1\n",
        "F1/1 prvi",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    r
}

/// I4 (završna recenzija M1): `clap` kod pogrešne uporabe sam izlazi s **2**, a 2 je kod za
/// ALERT — `sokratis signals --sinse …` u skripti je izgledao kao „projekt je u alarmu" umjesto
/// „naredba je pogrešno napisana". Pogrešna uporaba je greška okoline: izlaz **3**.
#[test]
fn wrong_usage_exits_3_not_2_which_means_alert() {
    let no_args = bin().output().unwrap();
    assert_eq!(no_args.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&no_args.stderr).contains("report"));

    let typo = bin()
        .args(["signals", "--sinse", "2026-09-01"])
        .output()
        .unwrap();
    assert_eq!(typo.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&typo.stderr).contains("--sinse"));
}

/// `--help` i `--version` clap također vraća kao `Err` (s `kind()` `DisplayHelp`/
/// `DisplayVersion`) — oni NISU greška i moraju ostati izlaz 0 na stdout.
#[test]
fn help_and_version_exit_0_on_stdout() {
    for arg in ["--help", "--version"] {
        let out = bin().arg(arg).output().unwrap();
        assert_eq!(out.status.code(), Some(0), "{arg}");
        assert!(!out.stdout.is_empty(), "{arg} mora pisati na stdout");
    }
}

/// I7 (spec §6): izlazni kodovi 0/1/2 su ugovor prema preflightu i jedini potrošač signala, a do
/// sada su ih testovi pokrivali samo za 2 (clap) i 3 (nije repo). Ova dva testa ih mjere nad
/// PRAVIM privremenim repozitorijem, kroz pravi binarni proces.
#[test]
fn signals_exit_0_when_there_is_nothing_to_report() {
    let r = repo_with_one_commit_on_main();
    let out = bin()
        .args(["signals", r.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("nema signala"));
}

#[test]
fn signals_exit_2_on_an_old_unmerged_branch() {
    let r = repo_with_one_commit_on_main();
    r.git(&["checkout", "-q", "-b", "feat/x"]);
    // Datum grane je FIKSAN i u prošlosti, pa starost samo raste kako sat ide naprijed: prag za
    // ALERT je 10 dana (`unmerged_alert_days`), a ova je grana stara više od dva mjeseca.
    r.commit(
        "js/b.js",
        "2\n",
        "F1/2 na staroj grani",
        "2026-07-01T12:00:00+02:00",
        "2026-07-01T12:00:00+02:00",
    );
    r.git(&["checkout", "-q", "main"]);
    let out = bin()
        .args(["signals", r.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("ALERT") && stdout.contains("unmerged-branches"),
        "{stdout}"
    );
    assert!(
        stdout.contains("feat/x"),
        "signal mora nositi dokaz: {stdout}"
    );
}

/// M10: `report --json --table` je tiho ispisao JSON. Dva zahtjeva za dva oblika ispisa su
/// pogrešna uporaba, ne „zadnji pobjeđuje" — repo je PRAVI, pa izlaz 3 dolazi od `clap`-a.
#[test]
fn json_and_table_together_is_a_usage_error() {
    let r = repo_with_one_commit_on_main();
    let out = bin()
        .args(["report", r.path().to_str().unwrap(), "--json", "--table"])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(3),
        "stdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("--table"));
}

/// M2/19: `--until` je gornja granica razdoblja (S-011/S-012) — jednodnevni prozor (`since` ==
/// `until`) hvata točno commit prvog dana, a `d2` (03-09) ostaje izvan. Neispravan oblik datuma
/// jezgra prijavi kao imenovanu grešku (`malformed_until_is_a_named_error`), CLI je samo ispiše
/// (izlaz 3, poruka spominje polje `until`) — bez vlastite validacije datuma u CLI-ju.
#[test]
fn report_until_limits_the_window_and_is_echoed_in_json() {
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
    let out = bin()
        .args([
            "report",
            r.path().to_str().unwrap(),
            "--json",
            "--since",
            "2026-09-01",
            "--until",
            "2026-09-01",
        ])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["until"], "2026-09-01");
    assert_eq!(v["touched"]["commits"], 1);

    let bad = bin()
        .args(["report", r.path().to_str().unwrap(), "--until", "1.9.2026"])
        .output()
        .unwrap();
    assert_eq!(bad.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&bad.stderr).contains("until"));
}

/// Repozitorij s 1 commitom na `main` i 1 na `feat/x`: mjeri razliku između `--scope all`
/// (zadano, S-032) i `--scope default` (paritet s 1.0.0-pre, samo zadana grana).
fn repo_with_feature_branch() -> Repo {
    let r = Repo::init();
    r.commit(
        "js/a.js",
        "1",
        "F1/1 main",
        "2026-09-10T10:00:00+02:00",
        "2026-09-10T10:00:00+02:00",
    );
    r.git(&["checkout", "-q", "-b", "feat/x"]);
    r.commit(
        "js/b.js",
        "2",
        "F1/2 grana",
        "2026-09-11T10:00:00+02:00",
        "2026-09-11T10:00:00+02:00",
    );
    r.git(&["checkout", "-q", "main"]);
    r
}

/// M2/51 (S-032): zadano (bez `--scope`) broji sve lokalne grane — commit na `feat/x` ulazi u
/// `touched.commits` i `branches`, s oznakom grane na retku commita.
#[test]
fn report_counts_all_branches_by_default_and_labels_rows() {
    let r = repo_with_feature_branch();
    let out = bin()
        .args(["report", "--json", "--since", "2026-09-01"])
        .arg(r.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["scope"], "all");
    assert_eq!(v["touched"]["commits"], 2);
    assert_eq!(v["branches"].as_array().unwrap().len(), 2);
    assert!(
        v["commits"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["branch"] == "feat/x"),
        "{v}"
    );
}

/// `--scope default` pregazi zadano iz profila (S-032) i vraća ponašanje 1.0.0-pre: samo zadana
/// grana ulazi u brojke, kao paritet s `RAD.xlsx`.
#[test]
fn scope_default_matches_pre_1_0_behaviour() {
    let r = repo_with_feature_branch();
    let out = bin()
        .args([
            "report",
            "--json",
            "--since",
            "2026-09-01",
            "--scope",
            "default",
        ])
        .arg(r.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["scope"], "default");
    assert_eq!(v["touched"]["commits"], 1, "samo main, kao 1.0.0-pre");
    assert_eq!(v["branches"].as_array().unwrap().len(), 1);
    assert_eq!(v["days"].as_array().unwrap().len(), 1);
}

/// `--scope` prima SAMO `all`/`default` (`value_parser`) — bilo što drugo je pogrešna uporaba
/// (izlaz 3), ne ALERT (2, nalaz I4).
#[test]
fn scope_rejects_unknown_value_as_usage_error() {
    let r = repo_with_feature_branch();
    let out = bin()
        .args(["report", "--scope", "worktrees"])
        .arg(r.path())
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(3),
        "pogrešna uporaba = 3, ne 2 (Alert)"
    );
}

/// Tablica (bez `--json`) mora imenovati opseg i pokazati granu koja nije zadana.
#[test]
fn table_shows_scope_and_branches() {
    let r = repo_with_feature_branch();
    let out = bin()
        .args(["report", "--table", "--since", "2026-09-01"])
        .arg(r.path())
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("opseg"), "{s}");
    assert!(s.contains("feat/x"), "{s}");
}

#[test]
fn not_a_repo_exits_3_with_message() {
    let dir = tempfile::tempdir().unwrap();
    let out = bin()
        .args(["report", dir.path().to_str().unwrap(), "--json"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&out.stderr).contains("nije git repozitorij"));
}
