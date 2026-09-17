//! ZAŠTO RUST OVAKO (cigla M1/17 — projekt)
//! `Project` POSJEDUJE `GitCli` i `Profile`; metode posuđuju `&self`. `fs::read_to_string(..).ok()`
//! pretvara „nema datoteke" u `None` — a JSON koji POSTOJI, a ne valja, je greška s putanjom
//! (`map_err` + `IoError::Manual { path, source }`), jer tiho ignoriranje krivog JSON-a je laž.
//! Rekurzivni `walk` je obična funkcija koja puni `&mut Vec` — bez rekurzivnih zatvaranja.
use crate::{GitCli, GitSource, IoError};
use sokratis_core::{DocFile, Profile, ReportInput, Vision, WorkKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub common_dir: PathBuf,
    pub profile: Profile,
    pub git: GitCli,
}

const SKIP_DIRS: [&str; 3] = ["node_modules", ".git", "target"];

/// Učitava `overrides.json` s korijena ako postoji; nema datoteke → prazna mapa.
/// Napomena o odstupanju od brifa: umjesto GENERIČKE funkcije preko `serde::de::DeserializeOwned`
/// (brif) ova je nezavisna za svaki konkretni tip, jer `sokratis-io` NE ovisi izravno o crateu
/// `serde` (samo o `serde_json`) — a generička granica bi to zahtijevala. Ne diramo `Cargo.toml`.
fn read_overrides_or_empty(path: &Path) -> Result<HashMap<String, WorkKind>, IoError> {
    match std::fs::read_to_string(path) {
        Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Manual {
            path: path.to_path_buf(),
            source,
        }),
        Err(_) => Ok(HashMap::new()),
    }
}

/// Učitava `visions.json` s korijena ako postoji; nema datoteke → prazan popis.
fn read_visions_or_empty(path: &Path) -> Result<Vec<Vision>, IoError> {
    match std::fs::read_to_string(path) {
        Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Manual {
            path: path.to_path_buf(),
            source,
        }),
        Err(_) => Ok(Vec::new()),
    }
}

/// Rekurzivno skuplja `*.md` pod `dir` u `out`, preskačući `SKIP_DIRS`.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), IoError> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if !SKIP_DIRS.contains(&name) {
                walk(&path, out)?;
            }
        } else if name.ends_with(".md") {
            out.push(path);
        }
    }
    Ok(())
}

impl Project {
    /// Otvara projekt iz bilo koje putanje unutar repozitorija (podmapa, radno stablo):
    /// `root` = `git rev-parse --show-toplevel`, `common_dir` dijele sva radna stabla istog projekta.
    /// Profil dolazi iz `<root>/.sokratis/profile.json` ako postoji, inače je `Profile::default()`
    /// (S-005 — zadano su konvencije Sokrat Studyja).
    pub fn open(path: &Path) -> Result<Project, IoError> {
        let probe = GitCli::new(path);
        let root = probe.toplevel()?;
        let common_dir = probe.common_dir()?;
        let profile_path = root.join(".sokratis").join("profile.json");
        let profile = match std::fs::read_to_string(&profile_path) {
            Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Profile {
                path: profile_path.clone(),
                source,
            })?,
            Err(_) => Profile::default(),
        };
        let git = GitCli::new(&root);
        Ok(Project {
            root,
            common_dir,
            profile,
            git,
        })
    }

    /// Ručni overridi klasifikacije po SHA-i commita (`.sokratis/overrides.json`).
    pub fn overrides(&self) -> Result<HashMap<String, WorkKind>, IoError> {
        read_overrides_or_empty(&self.root.join(".sokratis").join("overrides.json"))
    }

    /// Ručno upisane vizije (`.sokratis/visions.json`).
    pub fn visions(&self) -> Result<Vec<Vision>, IoError> {
        read_visions_or_empty(&self.root.join(".sokratis").join("visions.json"))
    }

    /// Svi `*.md` u korijenu i pod `profile.docs_dir`, sa sadržajem i zadnjom promjenom iz gita.
    pub fn docs(&self) -> Result<Vec<DocFile>, IoError> {
        let mut paths = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let p = entry?.path();
            if p.is_file() && p.extension().is_some_and(|e| e == "md") {
                paths.push(p);
            }
        }
        let docs_dir = self.root.join(&self.profile.docs_dir);
        if docs_dir.is_dir() {
            walk(&docs_dir, &mut paths)?;
        }
        let mut out = Vec::new();
        for p in paths {
            let rel = p
                .strip_prefix(&self.root)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            out.push(DocFile {
                content: std::fs::read_to_string(&p)?,
                last_change_time: self.git.last_change(&rel)?,
                path: rel,
            });
        }
        Ok(out)
    }

    /// Sastavlja `ReportInput` za jezgru: grana, log od `profile.log_since()`, dnevnik/plan ako
    /// postoje, dokumentacija, grane, ručni podaci i „sada"/„danas" (`chrono` — jedino mjesto sata
    /// u sustavu).
    pub fn input(&self, since: Option<&str>) -> Result<ReportInput, IoError> {
        let branch = if self.git.branch_exists(&self.profile.default_branch)? {
            self.profile.default_branch.clone()
        } else {
            self.git.current_branch()?
        };
        let read_opt = |rel: &str| std::fs::read_to_string(self.root.join(rel)).ok();
        Ok(ReportInput {
            git_log: self.git.log(&branch, &self.profile.log_since())?,
            diary: read_opt(&self.profile.diary_path),
            plan: read_opt(&self.profile.plan_path),
            docs: self.docs()?,
            branches: self.git.branches(&branch)?,
            overrides: self.overrides()?,
            visions: self.visions()?,
            now: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            today: chrono::Local::now().format("%Y-%m-%d").to_string(),
            since: since
                .map(str::to_string)
                .unwrap_or_else(|| self.profile.since.clone()),
            branch,
        })
    }
}
