use crate::IoError;
use sokratis_core::BranchInfo;
use std::path::PathBuf;
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
}
impl GitSource for GitCli {
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError> {
        let _ = (branch, since);
        todo!("cigla M1/15")
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
        todo!("cigla M1/15")
    }
    fn toplevel(&self) -> Result<PathBuf, IoError> {
        todo!("cigla M1/15")
    }
    fn branch_exists(&self, name: &str) -> Result<bool, IoError> {
        let _ = name;
        todo!("cigla M1/15")
    }
    fn current_branch(&self) -> Result<String, IoError> {
        todo!("cigla M1/15")
    }
}
