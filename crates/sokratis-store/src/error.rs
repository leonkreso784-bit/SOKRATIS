//! ZAŠTO RUST OVAKO (cigla M2/1 — greške pohrane; M2/17 dodaje `BadIndicatorKind`)
//! `#[from] rusqlite::Error` daje `?` na svakom SQL-pozivu; varijante s poljima nose IME projekta
//! ili PUTANJU jer poruku čita Leon u dijalogu, ne stroj. `BadIndicatorKind` je za stupac koji
//! Sokratis sam piše ("measure"/"proxy") — ako je drukčiji, baza je oštećena izvana.
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("baza {0}")]
    Open(PathBuf),
    #[error("ovo je stablo projekta koji već pratiš: {name}")]
    AlreadyTracked { name: String },
    #[error("projekt {0} ne postoji u registru")]
    NoSuchProject(i64),
    #[error("nepoznata vrsta pokazatelja '{0}' u snimci (očekivano measure ili proxy)")]
    BadIndicatorKind(String),
    #[error(transparent)]
    Sql(#[from] rusqlite::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
