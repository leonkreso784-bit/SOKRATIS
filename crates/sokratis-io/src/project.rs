//! ZAŠTO RUST OVAKO (cigla M1/17 — projekt)
//! `Project` POSJEDUJE `GitCli` i `Profile`; metode posuđuju `&self`. Za ručne JSON-datoteke je
//! `Err(e) if e.kind() == NotFound` JEDINO opravdanje za pad na zadano — svaka druga greška
//! čitanja (putanja je direktorij, nema dozvole) mora biti vidljiva, jer tiho gutanje je laž.
//! Za obični tekst `fs::read_to_string(..).ok()` je dovoljan; rekurzivni `walk` puni `&mut Vec`.
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
        // `NotFound` je jedino „legitimno" opravdanje za prazan rezultat — svaka DRUGA greška
        // čitanja (npr. putanja je direktorij, nema dozvole) mora biti vidljiva, ne progutana.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashMap::new()),
        Err(e) => Err(IoError::Io(e)),
    }
}

/// Učitava `visions.json` s korijena ako postoji; nema datoteke → prazan popis.
fn read_visions_or_empty(path: &Path) -> Result<Vec<Vision>, IoError> {
    match std::fs::read_to_string(path) {
        Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Manual {
            path: path.to_path_buf(),
            source,
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(IoError::Io(e)),
    }
}

/// Putanja za poruke o greškama: `root` dolazi iz gita s `/`, a `join` dodaje `\` (Windows), pa
/// je poruka miješala oba razdjelnika (nalaz I5). `components().collect()` sastavi istu putanju
/// natrag s razdjelnikom ovog sustava — sadržaj se ne mijenja, samo zapis.
fn normalized(path: PathBuf) -> PathBuf {
    path.components().collect()
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
        // `common_dir` se čita iz `root`, ne iz `probe` (korisnikova putanja): git bi za
        // podmapu vratio `common_dir` relativan na TU podmapu (npr. `docs/records/../../.git`),
        // pa bi usporedba identiteta projekta (isti `common_dir`) lagala kad se otvori iz podmape.
        let git = GitCli::new(&root);
        let common_dir = git.common_dir()?;
        let profile_path = normalized(root.join(".sokratis").join("profile.json"));
        let profile = match std::fs::read_to_string(&profile_path) {
            Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Profile {
                path: profile_path.clone(),
                source,
            })?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Profile::default(),
            Err(e) => return Err(IoError::Io(e)),
        };
        Ok(Project {
            root,
            common_dir,
            profile,
            git,
        })
    }

    /// Ručni overridi klasifikacije po SHA-i commita (`.sokratis/overrides.json`).
    pub fn overrides(&self) -> Result<HashMap<String, WorkKind>, IoError> {
        read_overrides_or_empty(&normalized(
            self.root.join(".sokratis").join("overrides.json"),
        ))
    }

    /// Ručno upisane vizije (`.sokratis/visions.json`).
    pub fn visions(&self) -> Result<Vec<Vision>, IoError> {
        read_visions_or_empty(&normalized(
            self.root.join(".sokratis").join("visions.json"),
        ))
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
            // M2: `git branch --show-current` ispiše ime grane i u repou BEZ ijednog commita
            // (HEAD je „unborn"), pa ime nije dokaz da grana postoji — provjerava se referenca.
            // Bez toga bi git odgovorio svojim savjetom o `--`, a ne rečenicom o repozitoriju.
            let current = self.git.current_branch()?;
            if current.is_empty() || !self.git.branch_exists(&current)? {
                return Err(IoError::NoCommits(self.root.clone()));
            }
            current
        };
        let read_opt = |rel: &str| std::fs::read_to_string(self.root.join(rel)).ok();
        let since = since
            .map(str::to_string)
            .unwrap_or_else(|| self.profile.since.clone());
        // I1: prozor dovlačenja je NAJRANIJE od dvoje — profilskog (`log_since()`, koji uključuje
        // i početke zatvorenih faza) i korisnikova `--since`. Bez tog `min`-a je `--since`
        // stariji od profila tiho dobivao kraći log nego što `Report.since` tvrdi.
        let fetch_since = self.profile.log_since().min(since.clone());
        Ok(ReportInput {
            git_log: self.git.log(&branch, &fetch_since)?,
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
            since,
            // M2/1 kostur: gornju granicu uvodi M2/14 (`input_between`).
            until: None,
            branch,
        })
    }
}
