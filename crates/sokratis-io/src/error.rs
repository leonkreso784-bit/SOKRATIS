//! ZAŠTO RUST OVAKO (cigla M1/15 — greške io-sloja · popravak I5)
//! `thiserror` daje `Display` po varijanti, a `#[source]` LANAC uzroka. Poruka varijante zato
//! NE smije ponavljati uzrok: `main` ispisuje `{e:#}`, što lanac ionako doda — inače se ista
//! serde-poruka (popis svih 38 polja profila) ispiše dvaput. `#[error(transparent)]` je obrnut
//! slučaj: varijanta bez vlastite rečenice prepušta cijeli `Display` uzroku.
use std::path::PathBuf;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum IoError {
    #[error("`git` nije na PATH-u")]
    GitMissing,
    #[error("{0} nije git repozitorij")]
    NotARepo(PathBuf),
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
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
