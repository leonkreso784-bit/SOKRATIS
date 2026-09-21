//! ZAŠTO RUST OVAKO (cigla M2/1 — Tauri Builder)
//! `Builder::default()` je gradilište: plugini, stanje i naredbe se lančano dodaju, a `run` uzima
//! vlasništvo i vrti petlju događaja do izlaza. `expect` je ovdje JEDINO dopušteno mjesto izvan
//! testova (RUST.md §1): ako se ljuska ne može pokrenuti, nema ničega što bi se moglo raditi.
//!
//! ZAŠTO RUST OVAKO (cigla M2/29 — stanje, registar i izvještaj kao naredbe)
//! `manage(AppState { .. })` predaje dijeljeno stanje Tauriju PRIJE `run()`; svaka naredba ga
//! poslije dohvati kao `State<'_, AppState>` (posudba, ne vlasništvo). Detalj uz oba poziva niže.
//! `setup` niže pokreće motor (`engine::start`) TEK NAKON `manage`, jer motor odmah čita `AppState`
//! (M2/30). Splash (`splash::arm`, M2/31) kreće PRIJE motora iz istog razloga — motorova nit zove
//! `splash::loaded`, koji bi panicirao da `SplashState` još nije managed.
//!
//! ZAŠTO RUST OVAKO (cigla M2/32 — tray, zatvaranje, jedna instanca)
//! `single-instance` je PRVI plugin (Tauri to zahtijeva); tray sam (`tray::build`) i X→hide
//! (`on_window_event` niže) su odvojeni od ovog gradilišta jer nijedan drugi modul ih ne treba.
mod cache;
mod commands;
mod engine;
mod splash;
mod state;
mod summary;
mod tray;

use commands::{
    add_project, get_report, get_settings, get_trend, list_projects, refresh, remove_project,
    rename_project, save_visions, set_override, set_setting,
};
use state::AppState;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri_plugin_autostart::MacosLauncher;

pub fn run() {
    tauri::Builder::default()
        // `single-instance` MORA biti prvi plugin (dopuna T32 #7) — drugo pokretanje ne otvara nov
        // proces nego samo podigne postojeći glavni prozor (isti pomoćnik kao tray i splash).
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            splash::show_main(app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
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
        // X sakriva SAMO glavni prozor (S-020) — watcher i tray rade dalje, izlaz je izričit kroz
        // "Izađi" u tray-izborniku (`tray.rs`, `app.exit(0)`). `splash` se zatvara normalno, jer
        // uvjet niže gleda samo prozor s labelom "main".
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event
                && window.label() == "main"
            {
                api.prevent_close();
                if let Err(e) = window.hide() {
                    eprintln!("glavni prozor: hide() nije uspio: {e}");
                }
            }
        })
        // Splash se naoružava PRIJE motora (M2/31) — `engine::start` odmah otvara nit koja na
        // kraju zove `splash::loaded`, pa `SplashState` mora biti managed prije toga. Motor
        // (nadzor datoteka + prvi izračun) kreće nakon `manage` — `engine::start` čita `AppState`
        // čim se pozove (M2/30). Tray (M2/32) se gradi zadnji — treba `AppState` za trenutnu
        // postavku jezika/autostarta pri gradnji izbornika.
        .setup(|app| {
            splash::arm(app.handle());
            engine::start(app.handle());
            tray::build(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri se nije pokrenuo");
}
