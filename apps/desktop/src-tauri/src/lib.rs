//! ZAŠTO RUST OVAKO (cigla M2/1 — Tauri Builder)
//! `Builder::default()` je gradilište: plugini, stanje i naredbe se lančano dodaju, a `run` uzima
//! vlasništvo i vrti petlju događaja do izlaza. `expect` je ovdje JEDINO dopušteno mjesto izvan
//! testova (RUST.md §1): ako se ljuska ne može pokrenuti, nema ničega što bi se moglo raditi.
//!
//! ZAŠTO RUST OVAKO (cigla M2/29 — stanje, registar i izvještaj kao naredbe)
//! `manage(AppState { .. })` predaje dijeljeno stanje Tauriju PRIJE `run()`; svaka naredba ga
//! poslije dohvati kao `State<'_, AppState>` (posudba, ne vlasništvo). Detalj uz oba poziva niže.
//! `setup` niže pokreće motor (`engine::start`) TEK NAKON `manage`, jer motor odmah čita `AppState`
//! (M2/30).
mod cache;
mod commands;
mod engine;
mod state;
mod summary;

use commands::{
    add_project, get_report, get_settings, get_trend, list_projects, refresh, remove_project,
    rename_project, save_visions, set_override, set_setting,
};
use state::AppState;
use std::collections::HashMap;
use std::sync::Mutex;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState {
            // Drugi `expect` u ovoj funkciji je namjeran iz istog razloga kao onaj na `run()`
            // niže: bez registra projekata nema što raditi, pa je pad odmah, s jasnom porukom,
            // bolji od tihog rada bez pohrane.
            store: Mutex::new(
                sokratis_store::Store::open(&state::db_path()).expect("baza u %LOCALAPPDATA%"),
            ),
            reports: Mutex::new(HashMap::new()),
            watcher: Mutex::new(None),
            queue: Mutex::new(sokratis_io::RefreshQueue::default()),
            rx: Mutex::new(None),
        })
        // `generate_handler!` je makro koji NA KOMPAJLIRANJU provjeri da svako ime iz popisa
        // postoji i ima ispravan potpis — tipfeler u imenu naredbe je greška prevoditelja, ne
        // runtime iznenađenje kad ga sučelje jednom pozove.
        .invoke_handler(tauri::generate_handler![
            list_projects,
            add_project,
            rename_project,
            remove_project,
            get_report,
            get_trend,
            set_override,
            save_visions,
            refresh,
            get_settings,
            set_setting
        ])
        // Motor (nadzor datoteka + prvi izračun) kreće OVDJE, nakon `manage` — `engine::start` čita
        // `AppState` čim se pozove (M2/30).
        .setup(|app| {
            engine::start(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri se nije pokrenuo");
}
