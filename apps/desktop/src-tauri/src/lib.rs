//! ZAŠTO RUST OVAKO (cigla M2/1 — Tauri Builder)
//! `Builder::default()` je gradilište: plugini, stanje i naredbe se lančano dodaju, a `run` uzima
//! vlasništvo i vrti petlju događaja do izlaza. `expect` je ovdje JEDINO dopušteno mjesto izvan
//! testova (RUST.md §1): ako se ljuska ne može pokrenuti, nema ničega što bi se moglo raditi.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("Tauri se nije pokrenuo");
}
