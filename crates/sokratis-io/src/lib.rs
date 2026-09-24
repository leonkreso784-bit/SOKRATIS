//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur io)
//! Ovo je JEDINO mjesto koje dira disk i procese. `pub use` izlaže tri stvari: trait `GitSource`
//! (ugovor), `GitCli` (implementacija kroz `git` proces) i `Project` (repo + profil + ručni podaci).
//!
//! Cigla M2/13 (watcher, S-016): `pub mod watch` dodaje četvrtu stvar — `Watcher` gleda `.git`,
//! dokumentaciju i `.sokratis` po projektu i javlja promjene kroz `mpsc`, bez ikakve ovisnosti o
//! Tauriju (DESKTOP T30 je spaja s `RefreshQueue` i stvarnim izračunom).
//!
//! Cigla M2/14b (potrošač keša): `pub mod cache` dodaje petu stvar — trait `CommitCache` (ugovor
//! prema pamćenju commita, bez ovisnosti o `sokratis-store`, S-013) i `cached_log` (jedina funkcija
//! koja ga koristi). `Project::input_cached` je novi pozivatelj u `project.rs`.
//!
//! Cigla M2/29b: `today` se pridružuje izvozu — desktop treba današnji datum bez vlastite
//! ovisnosti o `chrono` (S-013).
//!
//! Cigla M2/49 (S-032): `Scope` se pridružuje izvozu — pozivatelji `log`/`rev_list`/`cached_log`
//! izvan `io` (testovi) ga trebaju da uopće mogu sastaviti poziv.
pub mod cache;
pub mod error;
pub mod git;
pub mod project;
pub mod watch;
pub use cache::{CacheError, CommitCache, cached_log};
pub use error::IoError;
pub use git::{GitCli, GitSource, Scope};
pub use project::{Project, today};
pub use watch::{RefreshQueue, WatchEvent, WatchReason, Watcher};
