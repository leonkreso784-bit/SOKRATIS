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
    #[error("profil {path}: {source}")]
    Profile {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("ručni podaci {path}: {source}")]
    Manual {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
