//! ZAŠTO RUST OVAKO (cigla M1/15 — git kroz proces)
//! `std::process::Command` gradi poziv bez shella (nema quotinga, nema injekcije). `output()`
//! vraća `Output { status, stdout, stderr }`; `String::from_utf8_lossy` toleriše tuđi ne-UTF-8
//! bajt umjesto da sruši cijeli izvještaj. Greška se MAPIRA u `IoError` po uzroku (`map_err`).
use crate::IoError;
use sokratis_core::BranchInfo;
use std::path::PathBuf;
use std::process::Command;
pub trait GitSource {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError>;
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError>;
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError>;
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError>;
    fn common_dir(&self) -> Result<PathBuf, IoError>;
    fn toplevel(&self) -> Result<PathBuf, IoError>;
    fn branch_exists(&self, name: &str) -> Result<bool, IoError>;
    fn current_branch(&self) -> Result<String, IoError>;
}
pub struct GitCli {
    pub repo: PathBuf,
}
impl GitCli {
    pub fn new(repo: impl Into<PathBuf>) -> Self {
        Self { repo: repo.into() }
    }

    /// Pokreće `git` u repozitoriju bez shella i mapira ishod u `IoError`.
    fn run(&self, args: &[&str]) -> Result<String, IoError> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo)
            .args(args)
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    IoError::GitMissing
                } else {
                    IoError::Io(e)
                }
            })?;
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            if stderr.contains("not a git repository") {
                return Err(IoError::NotARepo(self.repo.clone()));
            }
            return Err(IoError::Git {
                cmd: args.join(" "),
                stderr,
            });
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    /// Pretvara relativan izlaz git-naredbe (npr. `rev-parse --show-toplevel`) u apsolutnu putanju.
    fn path_of(&self, args: &[&str]) -> Result<PathBuf, IoError> {
        let s = self.run(args)?;
        let rel = PathBuf::from(s.trim());
        Ok(if rel.is_absolute() {
            rel
        } else {
            self.repo.join(rel)
        })
    }
}
impl GitSource for GitCli {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError> {
        let since_arg = format!("--since={since}");
        self.run(&[
            "log",
            branch,
            &since_arg,
            "--reverse",
            "--date=format:%Y-%m-%d",
            "--format=@@%h|%at|%ct|%ad|%cd|%s",
            "--numstat",
        ])
    }
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError> {
        let _ = default_branch;
        todo!("cigla M1/16")
    }
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError> {
        todo!("cigla M1/16")
    }
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError> {
        let _ = path;
        todo!("cigla M1/16")
    }
    fn common_dir(&self) -> Result<PathBuf, IoError> {
        self.path_of(&["rev-parse", "--git-common-dir"])
    }
    fn toplevel(&self) -> Result<PathBuf, IoError> {
        self.path_of(&["rev-parse", "--show-toplevel"])
    }
    fn branch_exists(&self, name: &str) -> Result<bool, IoError> {
        let refname = format!("refs/heads/{name}");
        match self.run(&["show-ref", "--verify", "--quiet", &refname]) {
            Ok(_) => Ok(true),
            Err(IoError::Git { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }
    fn current_branch(&self) -> Result<String, IoError> {
        Ok(self.run(&["branch", "--show-current"])?.trim().to_string())
    }
}
