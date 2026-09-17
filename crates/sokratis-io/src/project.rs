use crate::{GitCli, IoError};
use sokratis_core::{DocFile, Profile, ReportInput, Vision, WorkKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
pub struct Project {
    pub root: PathBuf,
    pub common_dir: PathBuf,
    pub profile: Profile,
    pub git: GitCli,
}
impl Project {
    pub fn open(path: &Path) -> Result<Project, IoError> {
        let _ = path;
        todo!("cigla M1/17")
    }
    pub fn overrides(&self) -> Result<HashMap<String, WorkKind>, IoError> {
        todo!("cigla M1/17")
    }
    pub fn visions(&self) -> Result<Vec<Vision>, IoError> {
        todo!("cigla M1/17")
    }
    pub fn docs(&self) -> Result<Vec<DocFile>, IoError> {
        todo!("cigla M1/17")
    }
    pub fn input(&self, since: Option<&str>) -> Result<ReportInput, IoError> {
        let _ = since;
        todo!("cigla M1/17")
    }
}
