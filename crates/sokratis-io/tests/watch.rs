//! ZAŠTO RUST OVAKO (cigla M2/13 — watcher)
//! `std::sync::mpsc` je kanal vlasništva: `Watcher` drži `Sender`, test `Receiver`; `recv_timeout`
//! pretvara „nije stiglo" u vrijednost (`Err(Timeout)`) umjesto vječnog čekanja. Rafal se dokazuje
//! NEGATIVNO — drugi `recv_timeout` mora isteći.
mod common;
use common::Repo;
use sokratis_io::{Project, WatchReason, Watcher};
use std::sync::mpsc;
use std::time::Duration;

const DEBOUNCE: Duration = Duration::from_millis(600);

/// Repo s jednim commitom u `docs/`, `Project` otvoren nad njim i `Watcher` koji ga već nadzire
/// (`.git`, `docs/`, `.sokratis` kao zamjenski nerekurzivni `main_root` jer mapa još ne postoji).
fn watched() -> (
    Repo,
    Project,
    Watcher,
    mpsc::Receiver<sokratis_io::WatchEvent>,
) {
    let r = Repo::init();
    r.commit(
        "docs/a.md",
        "# a\n",
        "docs",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    let p = Project::open(r.path()).expect("project::open");
    let (tx, rx) = mpsc::channel();
    let mut w = Watcher::new(tx, DEBOUNCE).expect("watcher::new");
    w.watch_project(
        1,
        &p.common_dir,
        std::slice::from_ref(&p.root),
        &["docs"],
        &p.main_root(),
    )
    .expect("watch_project");
    settle(&rx);
    (r, p, w, rx)
}

/// Nadzor se ovdje registrira ODMAH nakon commita iz postave; stroj zna javiti zakašnjeli sirovi
/// događaj vezan uz TU postavu (izmjereno: Windows Defender skenira svježe `.git` datoteke i
/// zna dotaknuti atribut nakon što je nadzor već aktivan), ne uz radnju koju test stvarno
/// ispituje. Pričekaj dulje od odgode i ispuhni sve što je dotad stiglo, da test kreće s čistim
/// prijamnikom.
fn settle(rx: &mpsc::Receiver<sokratis_io::WatchEvent>) {
    std::thread::sleep(DEBOUNCE + Duration::from_millis(400));
    while rx.try_recv().is_ok() {}
}

#[test]
fn a_commit_yields_exactly_one_git_event_within_two_seconds() {
    let (r, _p, _w, rx) = watched();
    r.commit(
        "js/b.js",
        "1",
        "F1/2 x",
        "2026-09-02T10:00:00+02:00",
        "2026-09-02T10:00:00+02:00",
    );
    let e = rx
        .recv_timeout(Duration::from_secs(2))
        .expect("događaj nakon commita");
    assert_eq!((e.project_id, e.reason), (1, WatchReason::Git));
    assert!(
        rx.recv_timeout(Duration::from_millis(1500)).is_err(),
        "rafal HEAD/refs/logs = jedan događaj, ne tri"
    );
}

#[test]
fn a_docs_change_yields_a_docs_event() {
    let (r, _p, _w, rx) = watched();
    std::fs::write(r.path().join("docs/a.md"), "# promjena\n").expect("write docs/a.md");
    let e = rx.recv_timeout(Duration::from_secs(2)).expect("događaj");
    assert_eq!(e.reason, WatchReason::Docs);
}

/// `.sokratis/` nastaje TEK ovim upisom, pa `watch_project` (pozvan u `watched()`, prije nego što
/// mapa postoji) nadzire `main_root` nerekurzivno kao zamjenu — jedini sirovi događaj koji stigne
/// je stvaranje same mape `.sokratis` (predak potisnute putanje). Vidi `is_suppressed` u `watch.rs`.
#[test]
fn own_write_is_suppressed_so_editing_a_kind_does_not_loop() {
    let (_r, p, w, rx) = watched();
    let target = p.main_root().join(".sokratis").join("overrides.json");
    w.suppress(&target, Duration::from_secs(2));
    p.write_override("abc", Some(sokratis_core::WorkKind::Polish))
        .expect("write_override");
    assert!(
        rx.recv_timeout(Duration::from_millis(1500)).is_err(),
        "vlastiti upis ne smije proizvesti događaj"
    );
}

/// Kad `.sokratis` VEĆ postoji (za razliku od testa iznad), `watch_project` ga nadzire
/// REKURZIVNO — pa Windows za jedan `write_override` javi DVA sirova događaja: stvaranje
/// `overrides.json.tmp` i preimenovanje u `overrides.json`. `suppress` mora pokriti oba, inače
/// vlastiti upis ipak okine osvježavanje kroz privremenu datoteku (kontekst orkestratora, S-016).
#[test]
fn own_write_is_suppressed_including_the_temporary_file_when_sokratis_already_exists() {
    let r = Repo::init();
    r.commit(
        "docs/a.md",
        "# a\n",
        "docs",
        "2026-09-01T10:00:00+02:00",
        "2026-09-01T10:00:00+02:00",
    );
    std::fs::create_dir_all(r.path().join(".sokratis")).expect("mkdir .sokratis unaprijed");
    let p = Project::open(r.path()).expect("project::open");
    let (tx, rx) = mpsc::channel();
    let mut w = Watcher::new(tx, DEBOUNCE).expect("watcher::new");
    w.watch_project(
        1,
        &p.common_dir,
        std::slice::from_ref(&p.root),
        &["docs"],
        &p.main_root(),
    )
    .expect("watch_project (.sokratis već postoji => rekurzivno)");
    settle(&rx);
    let target = p.main_root().join(".sokratis").join("overrides.json");
    w.suppress(&target, Duration::from_secs(2));
    p.write_override("abc", Some(sokratis_core::WorkKind::Polish))
        .expect("write_override");
    assert!(
        rx.recv_timeout(Duration::from_millis(1500)).is_err(),
        "i privremena .tmp datoteka i konačan upis moraju biti potisnuti"
    );
}

/// `debounce_loop` mora završiti kad se `Watcher` odbaci: `notify`-jev unutarnji watcher se
/// zatvara u svom `Drop`-u, njegov callback (koji drži `raw_tx`) se time diže, `raw_rx.recv_timeout`
/// vrati `Disconnected`, petlja izađe i ispusti `tx` — inače svaki test ostavi živu nit (kontekst
/// orkestratora: bez ovog izlaza, `Watcher::drop` ne bi nikad gasio pozadinsku nit).
#[test]
fn dropping_watcher_ends_the_debounce_thread_so_receiver_disconnects() {
    let (tx, rx) = mpsc::channel::<sokratis_io::WatchEvent>();
    let w = Watcher::new(tx, DEBOUNCE).expect("watcher::new");
    drop(w);
    match rx.recv_timeout(Duration::from_secs(3)) {
        Err(mpsc::RecvTimeoutError::Disconnected) => {}
        other => panic!("očekivan Disconnected nakon drop(Watcher), dobiveno {other:?}"),
    }
}
