//! ZAŠTO RUST OVAKO (cigla M2/45 — autostart bez traya, S-036)
//! `set_autostart` je do sada živio u `tray.rs` jer ga je zvala tray-kvačica; tray je ukinut
//! (X = upit → izlaz), a autostart ostaje postavka u Postavkama. Funkcija seli NEPROMIJENJENA u
//! vlastitu datoteku: plugin pa baza, tim redom — greška plugina se ne upisuje u bazu (stanje u
//! bazi ne smije lagati o OS-u). Jedini pozivatelj je `commands::set_setting`.
use crate::state::{AppState, text};
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

pub(crate) fn set_autostart(app: &AppHandle, on: bool) -> Result<(), String> {
    let result = if on {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };
    result.map_err(text)?;
    app.state::<AppState>()
        .store
        .lock()
        .map_err(text)?
        .set_setting("autostart", if on { "on" } else { "off" })
        .map_err(text)
}
