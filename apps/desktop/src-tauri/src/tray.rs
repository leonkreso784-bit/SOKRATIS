//! ZAŠTO RUST OVAKO (cigla M2/32 — tray, zatvaranje u tray, autostart, jedna instanca)
//! Povratnu vrijednost `TrayIconBuilder::build` se ne drži u varijabli — Tauri je već sprema u
//! vlastitu tablicu resursa, pa ikona živi dok proces živi bez ijednog dodatnog `AppState` polja.
//! `on_menu_event`/`on_tray_icon_event` moraju biti `Fn` (ne `FnMut`): Tauri ih zove s glavne niti
//! kad god korisnik klikne, pa zatvorena kvačica (`autostart_item`) čita/piše SVOJE stanje preko
//! `is_checked`/`set_checked`, bez `&mut` na dijeljenu varijablu.

use crate::commands::setting_or;
use crate::engine;
use crate::splash;
use crate::state::{AppState, text};
use tauri::image::Image;
use tauri::menu::{CheckMenuItem, Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

/// Natpisi tray-izbornika, hrvatski/engleski (dopuna T32 #4) — JEDINA iznimka od rječnika sučelja
/// (`i18n/*.json`): Tauri izbornik nije WebView, sučelje ga ne može oblikovati. Grade se JEDNOM, pri
/// pokretanju; promjena jezika za vrijeme rada ne prepisuje izbornik (poznato ograničenje).
struct Labels {
    open: &'static str,
    refresh_all: &'static str,
    autostart: &'static str,
    quit: &'static str,
}

const HR: Labels = Labels {
    open: "Otvori",
    refresh_all: "Osvježi sve",
    autostart: "Autostart",
    quit: "Izađi",
};

const EN: Labels = Labels {
    open: "Open",
    refresh_all: "Refresh all",
    autostart: "Autostart",
    quit: "Exit",
};

fn labels(lang: &str) -> &'static Labels {
    if lang == "en" { &EN } else { &HR }
}

/// Jezik i stanje autostart postavke iz baze, čitani JEDNOM pri gradnji izbornika; brava/čitanje
/// koje ne uspije pada natrag na zadano umjesto da sruši pokretanje (isti obrazac kao
/// `notify_new_alerts` u `engine.rs`).
fn lang_and_autostart(app: &AppHandle) -> (String, bool) {
    match app.state::<AppState>().store.lock() {
        Ok(store) => {
            let lang = setting_or(&store, "lang", "hr").unwrap_or_else(|e| {
                eprintln!("tray: postavka jezika: {e}");
                "hr".to_string()
            });
            let autostart = setting_or(&store, "autostart", "off").unwrap_or_else(|e| {
                eprintln!("tray: postavka autostarta: {e}");
                "off".to_string()
            });
            (lang, autostart == "on")
        }
        Err(e) => {
            eprintln!("tray: brava baze: {e}");
            ("hr".to_string(), false)
        }
    }
}

/// Gradi tray-ikonu i izbornik (S-020): `Otvori` i lijevi klik podižu glavni prozor
/// (`splash::show_main`, isti pomoćnik kao splash i `single-instance` u `lib.rs`); `Osvježi sve` ide
/// u ZASEBNOJ niti kroz `engine::refresh_all` (dopuna T32 #5 — rukovatelj izbornika radi na glavnoj
/// niti, izračun bi je zamrznuo); `Autostart` je kvačica čije stanje čuva `set_autostart` niže;
/// `Izađi` gasi proces (`app.exit(0)`). Ikona je do T33 privremena kopija `icons/32x32.png`
/// (dopuna T32 #8).
pub fn build(app: &AppHandle) -> tauri::Result<()> {
    let (lang, autostart_on) = lang_and_autostart(app);
    let l = labels(&lang);

    let open_item = MenuItem::with_id(app, "open", l.open, true, None::<&str>)?;
    let refresh_item = MenuItem::with_id(app, "refresh_all", l.refresh_all, true, None::<&str>)?;
    let autostart_item = CheckMenuItem::with_id(
        app,
        "autostart",
        l.autostart,
        true,
        autostart_on,
        None::<&str>,
    )?;
    let quit_item = MenuItem::with_id(app, "quit", l.quit, true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&open_item, &refresh_item, &autostart_item, &quit_item],
    )?;

    let icon = Image::from_bytes(include_bytes!("../icons/tray.png"))?;

    TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        // Lijevi klik NE otvara izbornik (Tauriju je to zadano) — podiže prozor, isto kao "Otvori".
        // Desni klik uvijek otvara izbornik (Windows/macOS ga sami iscrtaju, ne isključuje se).
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            let id = event.id();
            if id == "open" {
                splash::show_main(app);
            } else if id == "refresh_all" {
                let app = app.clone();
                std::thread::spawn(move || engine::refresh_all(&app));
            } else if id == "autostart" {
                // Windows (muda) preokrene kvačicu PRIJE nego pošalje događaj — `is_checked` ovdje
                // već čita NOVO stanje; na grešci plugina vraćamo je natrag (baza se nije promijenila).
                let on = autostart_item.is_checked().unwrap_or_else(|e| {
                    eprintln!("tray: stanje kvačice autostarta: {e}");
                    false
                });
                if let Err(e) = set_autostart(app, on) {
                    eprintln!("tray: autostart nije uspio: {e}");
                    if let Err(e) = autostart_item.set_checked(!on) {
                        eprintln!("tray: vraćanje kvačice autostarta nije uspjelo: {e}");
                    }
                }
            } else if id == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                splash::show_main(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

/// Jedno mjesto za autostart (dopuna T32 #2): tray-kvačica i naredba `set_setting("autostart", …)`
/// (`commands.rs`) zovu OVU funkciju — plugin pa baza, tim redom. Greška plugina se NE upisuje u
/// bazu (stanje u bazi ne smije lagati o OS-u); baza se mijenja SAMO ako je plugin uspio.
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
