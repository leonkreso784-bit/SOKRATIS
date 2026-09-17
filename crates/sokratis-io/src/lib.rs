//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur io)
//! Ovo je JEDINO mjesto koje dira disk i procese. `pub use` izlaže tri stvari: trait `GitSource`
//! (ugovor), `GitCli` (implementacija kroz `git` proces) i `Project` (repo + profil + ručni podaci).
pub mod error;
pub mod git;
pub mod project;
pub use error::IoError;
pub use git::{GitCli, GitSource};
pub use project::Project;
