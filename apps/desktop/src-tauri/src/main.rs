//! ZAŠTO RUST OVAKO (cigla M2/1 — ulaz desktop procesa)
//! `windows_subsystem = "windows"` u release buildu gasi konzolni prozor; u debugu ostaje da se
//! vide `eprintln!`-i. Sav rad je u `lib.rs` (`run`) — tako Tauri može ciljati i mobilne platforme
//! istim kodom, iako ih Sokratis ne cilja.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    sokratis_desktop_lib::run()
}
