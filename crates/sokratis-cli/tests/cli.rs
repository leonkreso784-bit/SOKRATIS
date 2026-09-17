//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI naredbe i izlazni kodovi)
//! Test pokreće pravi binarni proces (`CARGO_BIN_EXE_sokratis`), ne poziva funkcije iz crate-a:
//! ono što se ovdje provjerava JE izlazni kod i tekst na stderr — ugovor prema preflightu koji
//! čita samo proces, ne Rust API. `assert_eq!` na `.status.code()` je zato jedini ispravan alat.
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sokratis"))
}

#[test]
fn no_args_prints_help_and_exits_2_by_clap() {
    let out = bin().output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("report"));
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
