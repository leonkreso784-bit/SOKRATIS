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
//! Cigla M2/14 (ograda putanja pri otvaranju, dug I9): `ProfileInvalid` UGRAĐUJE `{source}` u
//! vlastitu poruku — za razliku od `Profile`/`Manual` (I5: uzrok ostaje SAMO u lancu, da ga
//! `anyhow` na CLI-ju ne ispiše dvaput). Ovdje je razlog obrnut: `ParseError::PathOutsideRoot` je
//! JEDAN kratak redak (ime polja + vrijednost), ne popis 38 polja, a poruka mora „nositi ime
//! polja" i kad se `IoError` čita izravno (bez `anyhow`-omota) — npr. u testu io-sloja.
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
    #[error("profil {path}: {source}")]
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
}
