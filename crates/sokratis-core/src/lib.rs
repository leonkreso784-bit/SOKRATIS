//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur jezgre)
//! Jezgra nema I/O: ni `std::fs`, ni `std::process`. Sve što treba dolazi kao `&str` ili
//! struktura kroz `ReportInput`. To je granica S-002 i razlog zašto se testira bez gita.
//! `pub mod` = modul je datoteka; `pub use` = kraći put do tipova za pozivatelja.
pub mod civil;
pub mod classify;
pub mod docs;
pub mod error;
pub mod metrics;
pub mod model;
pub mod parse;
pub mod profile;
pub mod report;
pub mod rules;
pub mod snapshot;

pub use error::ParseError;
pub use model::*;
pub use profile::{Patterns, Profile};
pub use report::build_report;
