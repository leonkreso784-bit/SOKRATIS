//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI naredbe i izlazni kodovi)
//! Test pokreće pravi binarni proces (`CARGO_BIN_EXE_sokratis`), ne poziva funkcije iz crate-a:
//! ono što se ovdje provjerava JE izlazni kod i tekst na stderr — ugovor prema preflightu koji
//! čita samo proces, ne Rust API. `assert_eq!` na `.status.code()` je zato jedini ispravan alat.
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sokratis"))
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
