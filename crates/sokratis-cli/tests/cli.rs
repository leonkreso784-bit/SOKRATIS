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
