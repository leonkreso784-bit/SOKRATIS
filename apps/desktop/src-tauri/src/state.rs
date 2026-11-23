//! ZAŠTO RUST OVAKO (cigla M2/29 — stanje ljuske)
//! Tauri drži jedan `AppState` i posuđuje ga naredbama kao `State<'_, AppState>`; `Mutex` oko svakog
//! dijela jer naredbe dolaze s više niti (svaki `invoke` iz sučelja je poziv na drugoj niti). `Store`
//! nije `Sync` (SQLite veza), pa je `Mutex<Store>` jedini ispravan način da ga naredbe dijele — a to
//! je i jedino mjesto gdje se `lock().map_err` pretvara u tekst kroz `text` niže.
//!
//! ZAŠTO RUST OVAKO (cigla mac/T77 — baza na pravom mjestu po sustavu)
//! `data_dir` prima sustav kao `&str` i čitač okoline kao `impl Fn(&str) -> Option<OsString>`:
//! u radu to su `std::env::consts::OS` (ime sustava upisano pri kompilaciji) i `std::env::var_os`,
//! a u testu lažna okolina — pa se pravila za Windows, macOS i Linux provjeravaju na bilo kojem
//! stroju. Razlog: na Macu nema `LOCALAPPDATA`, baza je tiho padala u `$TMPDIR`, koji macOS čisti.
use sokratis_core::Report;
use sokratis_io::{RefreshQueue, WatchEvent, Watcher};
use sokratis_store::Store;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::{Mutex, mpsc::Receiver};

/// Dijeljeno stanje aplikacije: registar (`store`), zadnji izračunati izvještaj po projektu
/// (`reports`, puni ga motor — `engine::refresh_project`, vidi `engine.rs`), watcher datoteka i
/// red čekanja (`watcher`, `queue`) te primatelj događaja koji nit motora vadi iz `rx` (`take()`)
/// prije blokirajuće petlje `recv()` (M2/30).
pub struct AppState {
    pub store: Mutex<Store>,
    pub reports: Mutex<HashMap<i64, Report>>,
    pub watcher: Mutex<Option<Watcher>>,
    pub queue: Mutex<RefreshQueue>,
    pub rx: Mutex<Option<Receiver<WatchEvent>>>,
}

/// Ime datoteke baze — `cfg!(debug_assertions)` je makro koje se PRI KOMPILACIJI svede na `true`/`false`
/// (za razliku od atributa `#[cfg(...)]`, koji nepotreban kod IZBACUJE iz binarnog zapisa), pa se obje
/// grane daju testirati u istom buildu (M2/37). Razlog za dvije datoteke: instalirana „1.0.0-pre" i
/// `npm run tauri dev` inače pišu u ISTU bazu, a Leon instaliranu koristi dok se razvojna gradi.
pub fn db_file_name(debug: bool) -> &'static str {
    if debug {
        "sokratis-dev.db"
    } else {
        "sokratis.db"
    }
}

/// Mapa podataka Sokratisa po sustavu (mac/T77). `os` je ime kao u `std::env::consts::OS`, a `env`
/// čita varijablu okoline — u radu `std::env::var_os`, u testu lažna tablica. Prazna varijabla vrijedi
/// kao da je nema. `None` znači da se iz okoline ne da složiti ništa; o rezervi odlučuje pozivatelj.
///
/// - `"windows"`: `%LOCALAPPDATA%\sokratis` (isto kao 1.0.x — postojeće instalacije nađu svoju bazu)
/// - `"macos"`: `$HOME/Library/Application Support/sokratis`
/// - ostalo: `$XDG_DATA_HOME/sokratis`, inače `$HOME/.local/share/sokratis`
pub fn data_dir(os: &str, env: impl Fn(&str) -> Option<OsString>) -> Option<PathBuf> {
    // Pomoćnik: varijabla kao putanja, ali samo ako postoji I nije prazna.
    let var = |name: &str| -> Option<PathBuf> {
        env(name)
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    };
    let base = match os {
        "windows" => var("LOCALAPPDATA")?,
        "macos" => var("HOME")?.join("Library").join("Application Support"),
        _ => match var("XDG_DATA_HOME") {
            Some(xdg) => xdg,
            None => var("HOME")?.join(".local").join("share"),
        },
    };
    Some(base.join("sokratis"))
}

/// Putanja baze — JEDINO mjesto koje je sastavlja. Mapu bira `data_dir` po sustavu (Windows
/// `%LOCALAPPDATA%`, macOS `~/Library/Application Support`). Ako je okolina ne da složiti (proces
/// bez korisničkog profila), privremena mapa drži razvoj živim umjesto da `run()` padne — ali to se
/// glasno kaže, jer je baš tiha rezerva na Macu sakrila da baza živi u `$TMPDIR`, koji sustav čisti.
pub fn db_path() -> PathBuf {
    // `var_os` je generička funkcija (`K: AsRef<OsStr>`): predana izravno, prevoditelj je veže na
    // JEDAN životni vijek `&str`-a, a `impl Fn(&str)` traži da radi za svaki. Zatvaranje
    // `|name| std::env::var_os(name)` je napisano baš za `&str` bilo kojeg vijeka, pa prolazi.
    let dir = match data_dir(std::env::consts::OS, |name| std::env::var_os(name)) {
        Some(dir) => dir,
        None => {
            let fallback = std::env::temp_dir().join("sokratis");
            eprintln!(
                "sokratis: mapa podataka za sustav {} se ne da složiti iz okoline \
                 (nema LOCALAPPDATA, HOME ili XDG_DATA_HOME); baza ide u privremenu mapu {}, \
                 koju sustav smije obrisati",
                std::env::consts::OS,
                fallback.display()
            );
            fallback
        }
    };
    dir.join(db_file_name(cfg!(debug_assertions)))
}

/// Pretvara BILO KOJU grešku (`IoError`, `StoreError`, otrovan `Mutex`…) u tekst — Tauri naredbe
/// vraćaju `Result<T, String>` jer greška putuje sučelju preko granice procesa kao JSON tekst, ne
/// kao Rustov `Error`-trait. Jedina funkcija u cijeloj ljusci koja zna za taj ugovor.
///
/// Dopunjeno M2/63 — I3: `{e:#}` (uz `anyhow`) ispisuje CIJELI lanac uzroka, ali `thiserror`-tipovi
/// (`IoError`, `StoreError`…) `#` ne mijenja ništa (poruka je SAMO varijanta, uzrok stoji odvojeno u
/// `#[source]`, obrazac I5 u `crates/sokratis-io/src/error.rs`). Zato `text` sad zahtijeva
/// `std::error::Error` i sam šeta `source()` lancem, spajajući svaku poruku s „: ".
pub fn text<E: std::error::Error>(e: E) -> String {
    let mut out = e.to_string();
    let mut cause: Option<&dyn std::error::Error> = e.source();
    while let Some(err) = cause {
        out.push_str(": ");
        out.push_str(&err.to_string());
        cause = err.source();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_build_uses_its_own_database_file() {
        assert_eq!(db_file_name(false), "sokratis.db");
        assert_eq!(db_file_name(true), "sokratis-dev.db");
    }

    /// Lažna okolina: popis parova (ime varijable, vrijednost). Imenovani tip skraćuje potpise niže.
    type FakeEnv = &'static [(&'static str, &'static str)];

    /// Lažna okolina za `data_dir`: vraća samo zadane parove, NIKAD ne čita stvarnu okolinu
    /// procesa — zato testovi daju isti ishod na Windowsu, Macu i u CI-ju.
    fn fake_env(pairs: FakeEnv) -> impl Fn(&str) -> Option<OsString> {
        move |name| {
            pairs
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| OsString::from(value))
        }
    }

    /// mac/T77 (N4): tablica (sustav, okolina) → mapa podataka. Očekivane putanje se slažu kroz
    /// `join`, pa usporedba ne ovisi o razdjelniku (`\` ili `/`) sustava na kojem test radi.
    #[test]
    fn data_dir_follows_the_operating_system() {
        let windows = PathBuf::from("C:/Users/leon/AppData/Local");
        let home = PathBuf::from("/Users/leon");
        let xdg = PathBuf::from("/home/leon/.data");
        let linux_home = PathBuf::from("/home/leon");

        let cases: Vec<(&str, FakeEnv, Option<PathBuf>)> = vec![
            // Windows: bez promjene ponašanja 1.0.x — baza ostaje gdje je bila.
            (
                "windows",
                &[("LOCALAPPDATA", "C:/Users/leon/AppData/Local")],
                Some(windows.join("sokratis")),
            ),
            // macOS: `Application Support`, ne `$TMPDIR` koji sustav sam čisti.
            (
                "macos",
                &[("HOME", "/Users/leon")],
                Some(
                    home.join("Library")
                        .join("Application Support")
                        .join("sokratis"),
                ),
            ),
            // Linux: XDG ima prednost pred HOME-om.
            (
                "linux",
                &[
                    ("XDG_DATA_HOME", "/home/leon/.data"),
                    ("HOME", "/home/leon"),
                ],
                Some(xdg.join("sokratis")),
            ),
            (
                "linux",
                &[("HOME", "/home/leon")],
                Some(linux_home.join(".local").join("share").join("sokratis")),
            ),
            // Ništa za složiti → `None` (pozivatelj odlučuje o rezervi i kaže zašto).
            ("macos", &[], None),
            ("windows", &[("HOME", "/Users/leon")], None),
            // Prazna varijabla = kao da je nema.
            ("macos", &[("HOME", "")], None),
            (
                "linux",
                &[("XDG_DATA_HOME", ""), ("HOME", "/home/leon")],
                Some(linux_home.join(".local").join("share").join("sokratis")),
            ),
        ];

        for (os, pairs, expected) in cases {
            assert_eq!(
                data_dir(os, fake_env(pairs)),
                expected,
                "sustav {os}, okolina {pairs:?}"
            );
        }
    }

    /// I3: prije popravka `text` je pokazivao SAMO „profil {path}" — ime polja iz `ParseError`
    /// ostajalo je zaključano u `#[source]` lancu koji `{e:#}` (varijanta bez `anyhow`) ne otvara.
    #[test]
    fn text_walks_the_source_chain_into_the_field_name() {
        let err = sokratis_io::IoError::ProfileInvalid {
            path: PathBuf::from("C:/repo/.sokratis/profile.json"),
            source: sokratis_core::ParseError::PathOutsideRoot {
                field: "docs_dir".into(),
                value: "../x".into(),
            },
        };
        let msg = text(err);
        assert!(msg.contains("profil"), "poruka bez top-razine: {msg}");
        assert!(msg.contains("docs_dir"), "poruka bez imena polja: {msg}");
    }
}
