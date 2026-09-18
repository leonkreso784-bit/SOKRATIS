//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur io)
//! Ovo je JEDINO mjesto koje dira disk i procese. `pub use` izlaže tri stvari: trait `GitSource`
//! (ugovor), `GitCli` (implementacija kroz `git` proces) i `Project` (repo + profil + ručni podaci).
//!
//! Cigla M2/13 (watcher, S-016): `pub mod watch` dodaje četvrtu stvar — `Watcher` gleda `.git`,
//! dokumentaciju i `.sokratis` po projektu i javlja promjene kroz `mpsc`, bez ikakve ovisnosti o
//! Tauriju (DESKTOP T30 je spaja s `RefreshQueue` i stvarnim izračunom).
pub mod error;
pub mod git;
pub mod project;
pub mod watch;
pub use error::IoError;
pub use git::{GitCli, GitSource};
pub use project::Project;
pub use watch::{RefreshQueue, WatchEvent, WatchReason, Watcher};
