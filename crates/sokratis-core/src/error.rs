//! ZAŠTO RUST OVAKO (cigla M1/1 — greške jezgre)
//! `thiserror` pretvara enum u pravi `Error` s porukom po varijanti. Pozivatelj dobiva TIP
//! greške (koji redak, koji broj), ne string — i može odlučiti što s njom (`?` je propagira).
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("redak {line}: neispravan zapis commita: {text}")]
    BadLine { line: usize, text: String },
    #[error("redak {line}: nije broj: {text}")]
    BadNumber { line: usize, text: String },
    #[error("neispravan regex u profilu: {0}")]
    Regex(#[from] regex::Error),
    #[error("profil.{field}: regex mora imati {need} capture-grupa, ima {got}")]
    BadPattern {
        field: String,
        need: usize,
        got: usize,
    },
}
