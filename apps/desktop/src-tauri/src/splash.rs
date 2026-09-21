//! ZAŠTO RUST OVAKO (cigla M2/31 — splash prozor)
//! `AtomicBool` umjesto `Mutex<bool>`: tri zastavice se SAMO postavljaju/čitaju, nikad zajedno kao
//! jedna transakcija, pa atomska operacija bez brave sigurno štiti "shown" kad ga TRI niti
//! (događaj, motor, rezerva) pokušaju postaviti gotovo istovremeno.
//! Rezerva od 10 s: pozadinski WebView zna izgubiti `requestAnimationFrame`, pa `splash:done`
//! možda nikad ne stigne — glavni prozor se ipak mora pojaviti.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::{AppHandle, Listener, Manager};

/// Tri neovisna uvjeta prikaza glavnog prozora (dopuna T31 #4, #6): animacija na splashu
/// (`intro_done`), prvi izračun svih projekata (`load_done`) i "već prikazano" (`shown`) — potonji
/// sprječava dvostruki `main.show()`/`splash.close()` kad se uvjeti stignu gotovo istovremeno.
#[derive(Default)]
pub struct SplashState {
    intro_done: AtomicBool,
    load_done: AtomicBool,
    shown: AtomicBool,
}

/// Naoružava splash: upisuje `SplashState`, sluša `splash:done` (sučelje ga šalje TOČNO jednom) i
/// pokreće rezervu od 10 s. Zove ga `lib.rs` u `setup`, PRIJE `engine::start` — motor u svojoj niti
/// poziva `loaded`, a `app.state::<SplashState>()` bi panicirao da state još nije managed.
pub fn arm(app: &AppHandle) {
    app.manage(SplashState::default());

    let for_listener = app.clone();
    app.listen("splash:done", move |_event| {
        for_listener
            .state::<SplashState>()
            .intro_done
            .store(true, Ordering::SeqCst);
        maybe_show(&for_listener);
    });

    let for_reserve = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(10));
        // Rezerva pokazuje glavni BEZ OBZIRA na oba uvjeta (dopuna T31 #5) — izravno na
        // `show_once`, ne preko `maybe_show`.
        show_once(&for_reserve);
    });
}

/// Javlja da je prvi izračun SVIH projekata gotov (dopuna T31 #3) — zove ga `engine.rs` nakon
/// petlje po registru; bez projekata u registru petlja je prazna pa se ovo zove odmah, a greška
/// izračuna pojedinog projekta ne zadržava prozor (izračun je "gotov" i kad vrati grešku).
pub fn loaded(app: &AppHandle) {
    app.state::<SplashState>()
        .load_done
        .store(true, Ordering::SeqCst);
    maybe_show(app);
}

/// Pokazuje glavni prozor ako su OBA uvjeta (`intro_done`, `load_done`) ispunjena.
fn maybe_show(app: &AppHandle) {
    let state = app.state::<SplashState>();
    if state.intro_done.load(Ordering::SeqCst) && state.load_done.load(Ordering::SeqCst) {
        show_once(app);
    }
}

/// Prikaz TOČNO JEDNOM (dopuna T31 #4): `swap` vraća PRIJAŠNJU vrijednost, pa druga i treća
/// utrkujuća nit vide `true` i odustanu. Ako prozora `splash` više nema (korisnik ga zatvorio), to
/// nije greška — `close()` se jednostavno ne poziva.
fn show_once(app: &AppHandle) {
    let state = app.state::<SplashState>();
    if state.shown.swap(true, Ordering::SeqCst) {
        return;
    }
    show_main(app);
    if let Some(splash) = app.get_webview_window("splash")
        && let Err(e) = splash.close()
    {
        eprintln!("splash: zatvaranje prozora nije uspjelo: {e}");
    }
}

/// Pokazuje i fokusira glavni prozor (dopuna T31 #7) — dijeli ga i T32 (tray, single-instance) kad
/// vrati već postojeći prozor u prvi plan, bez ikakve veze sa splashom.
pub(crate) fn show_main(app: &AppHandle) {
    let Some(main) = app.get_webview_window("main") else {
        return;
    };
    if let Err(e) = main.show() {
        eprintln!("glavni prozor: show() nije uspio: {e}");
    }
    if let Err(e) = main.set_focus() {
        eprintln!("glavni prozor: set_focus() nije uspio: {e}");
    }
}
