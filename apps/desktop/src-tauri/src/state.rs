//! ZAŠTO RUST OVAKO (cigla M2/29 — stanje ljuske)
//! Tauri drži jedan `AppState` i posuđuje ga naredbama kao `State<'_, AppState>`; `Mutex` oko svakog
//! dijela jer naredbe dolaze s više niti (svaki `invoke` iz sučelja je poziv na drugoj niti). `Store`
//! nije `Sync` (SQLite veza), pa je `Mutex<Store>` jedini ispravan način da ga naredbe dijele — a to
//! je i jedino mjesto gdje se `lock().map_err` pretvara u tekst kroz `text` niže.
use sokratis_core::Report;
use sokratis_io::{RefreshQueue, WatchEvent, Watcher};
use sokratis_store::Store;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, mpsc::Receiver};

/// Dijeljeno stanje aplikacije: registar (`store`), zadnji izračunati izvještaj po projektu
/// (`reports`, puni ga SAMO `refresh` — vidi `commands.rs`), i tri polja koje T30 (motor, watcher)
/// tek počinje koristiti (`watcher`, `queue`, `rx`) — ovdje su prazna, da `manage(...)` u `lib.rs`
/// ne mora čekati sljedeću ciglu.
pub struct AppState {
    pub store: Mutex<Store>,
    pub reports: Mutex<HashMap<i64, Report>>,
    // Ova cigla (M2/29) ih samo NAPUNI praznima — nijedna naredba iz T29 ih još ne čita. `clippy`
    // bi to bez `allow` prijavio kao mrtav kod; T30 (motor, watcher) ih čita i piše.
    #[allow(dead_code)]
    pub watcher: Mutex<Option<Watcher>>,
    #[allow(dead_code)]
    pub queue: Mutex<RefreshQueue>,
    #[allow(dead_code)]
    pub rx: Mutex<Option<Receiver<WatchEvent>>>,
}

/// Putanja baze — JEDINO mjesto koje je sastavlja (T37 će iz njega kasnije izdvojiti ime datoteke
/// za debug/release build). `%LOCALAPPDATA%` nedostaje samo u okruženju bez korisničkog profila;
/// tada privremena mapa drži razvoj živim umjesto da `run()` padne bez razloga.
pub fn db_path() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("sokratis")
        .join("sokratis.db")
}

/// Pretvara BILO KOJU grešku (`IoError`, `StoreError`, otrovan `Mutex`…) u tekst — Tauri naredbe
/// vraćaju `Result<T, String>` jer greška putuje sučelju preko granice procesa kao JSON tekst, ne
/// kao Rustov `Error`-trait. Jedina funkcija u cijeloj ljusci koja zna za taj ugovor.
pub fn text<E: std::fmt::Display>(e: E) -> String {
    format!("{e:#}")
}
