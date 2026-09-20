//! ZAŠTO RUST OVAKO (cigla M1/15 — greške io-sloja · popravak I5)
//! `thiserror` daje `Display` po varijanti, a `#[source]` LANAC uzroka. Poruka varijante zato
//! NE smije ponavljati uzrok: `main` ispisuje `{e:#}`, što lanac ionako doda — inače se ista
//! serde-poruka (popis svih 38 polja profila) ispiše dvaput. `#[error(transparent)]` je obrnut
//! slučaj: varijanta bez vlastite rečenice prepušta cijeli `Display` uzroku.
//!
//! Cigla M2/10 (pisanje ručnih podataka): `write_override`/`write_visions` serijaliziraju kroz
//! `serde_json::to_string_pretty`, čiji `Result` ide kroz `?` — `#[from]` na novoj varijanti
//! `Encode` pretvara `serde_json::Error` u `IoError` bez ručnog mapiranja na svakom pozivu.
//!
//! Cigla M2/13 (watcher): `Watch(String)` nosi već oblikovanu poruku (npr. s putanjom koja je
//! pukla) umjesto `notify::Error`-a kroz `#[from]` — `watch_project` javlja PO PUTANJI kod pada,
//! pa je jedna varijanta s gotovim tekstom jednostavnija od posebnog tipa za svaki izvor greške.
//!
//! Cigla M2/14 (ograda putanja pri otvaranju, dug I9): `ProfileInvalid` slijedi ISTI obrazac kao
//! `Profile`/`Manual` (I5) — uzrok ostaje SAMO u `#[source]` lancu, poruka ga ne ponavlja. Krug
//! popravka 1 (recenzija): prva verzija je poruku ugradila kroz `{source}` da bi test mogao čitati
//! ime polja izravno iz `IoError::to_string()`, ali to je bio brifov propust, ne pravilo — kad
//! `IoError::ProfileInvalid` prođe kroz `anyhow` na CLI-ju (`main` ispisuje `{e:#}`), rečenica
//! `ParseError::PathOutsideRoot`-a (ime polja + vrijednost) bi se ispisala DVAPUT, isto što je I5
//! već jednom popravio za `Profile`/`Manual`. Ime polja i dalje stiže do korisnika — samo kroz
//! LANAC (`std::error::Error::source`), ne kroz top-poruku.
//!
//! Cigla M2/14b (potrošač keša): tri nove varijante slijede ISTI obrazac I5. `Cache` nosi
//! `crate::cache::CacheError` (`Box<dyn Error + Send + Sync>`) — thiserror i njemu daje `#[source]`
//! jer std implementira `Error` za taj boksani tip. `CacheIncomplete` NEMA `#[source]`: rub koji
//! opisuje (git tvrdi da je commit dostižan, pa ga ne vrati) nema uzrok izvan sebe.
use std::path::PathBuf;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum IoError {
    #[error("`git` nije na PATH-u")]
    GitMissing,
    #[error("{0} nije git repozitorij")]
    NotARepo(PathBuf),
    #[error("{0}: repozitorij nema commita")]
    NoCommits(PathBuf),
    #[error("git {cmd}: {stderr}")]
    Git { cmd: String, stderr: String },
    #[error("profil {path}")]
    Profile {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("ručni podaci {path}")]
    Manual {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("profil {path}")]
    ProfileInvalid {
        path: PathBuf,
        #[source]
        source: sokratis_core::ParseError,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("zapis ručnih podataka")]
    Encode(#[from] serde_json::Error),
    #[error("watcher: {0}")]
    Watch(String),
    #[error("keš commita")]
    Cache(#[source] crate::cache::CacheError),
    #[error("keš commita: git nije vratio commit {sha} koji je sam naveo kao dostižan")]
    CacheIncomplete { sha: String },
    #[error("git log se ne da pročitati")]
    LogParse(#[source] sokratis_core::ParseError),
}
