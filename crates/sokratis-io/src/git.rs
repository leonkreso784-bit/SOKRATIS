//! ZAŠTO RUST OVAKO (cigla M1/15 — git kroz proces)
//! `std::process::Command` gradi poziv bez shella (nema quotinga, nema injekcije). `output()`
//! vraća `Output { status, stdout, stderr }`; `String::from_utf8_lossy` toleriše tuđi ne-UTF-8
//! bajt umjesto da sruši cijeli izvještaj. Greška se MAPIRA u `IoError` po uzroku (`map_err`).
//!
//! ZAŠTO RUST OVAKO (cigla M1/16 — grane, radna stabla, zadnja promjena)
//! `HashSet<String>` u `branches` daje O(1) provjeru „je li grana spojena" umjesto linearnog
//! pretraživanja liste. `let-else` (`let Some((name, t)) = ... else { continue }`) preskače
//! redak bez ugniježđenog `if let`. `unwrap_or(0)` na `parse::<i64>()` je svjesna odluka, ne
//! skrivena greška: git format `%(authordate:unix)` uvijek ispisuje broj, pa je alternativa
//! (0) mrtav kod koji se nikad ne izvrši — provjereno testovima, ne pretpostavkom.
use crate::IoError;
use sokratis_core::BranchInfo;
use std::path::PathBuf;
use std::process::Command;
pub trait GitSource {
    /// `since` je goli datum `YYYY-MM-DD`. Implementacija MORA dodati sat `00:00:00`: git-ov
    /// parser datuma bez sata uzima TRENUTNO DOBA DANA (sat kad se naredba pokreće), ne ponoć —
    /// izmjereno nad Sokrat Studyjem (`--since=2026-08-29` u 17:15 → 183 commita,
    /// `--since='2026-08-29 00:00'` → 190). Bez fiksnog sata bi tablica ovisila o tome KADA se
    /// izvještaj generira, ne samo o datumu. Uz to MORA dovući DAN VIŠE (nalaz I2): ta je ponoć
    /// u zoni stroja, a datumi commita u zoni commita, pa granicu mora presuditi jezgrin
    /// `commit_date >= since`, ne git. Rezerva ne mijenja nijednu brojku — jezgra je odbaci.
    fn log(&self, branch: &str, since: &str) -> Result<String, IoError>;
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError>;
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError>;
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError>;
    fn common_dir(&self) -> Result<PathBuf, IoError>;
    fn toplevel(&self) -> Result<PathBuf, IoError>;
    fn branch_exists(&self, name: &str) -> Result<bool, IoError>;
    fn current_branch(&self) -> Result<String, IoError>;
}
#[derive(Debug)]
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
        // ` 00:00:00` fiksira sat na ponoć, a `prev_day` dodaje dan rezerve zbog zone — vidi
        // doc-komentar `GitSource::log` (trait) za oba razloga. `unwrap_or_else` vraća neispravan
        // datum nepromijenjen: njega jezgra prijavi kao `ParseError::BadDate` (C2), ne ovaj sloj.
        let from = sokratis_core::civil::prev_day(since).unwrap_or_else(|| since.to_string());
        let since_arg = format!("--since={from} 00:00:00");
        self.run(&[
            "log",
            branch,
            &since_arg,
            "--reverse",
            "--date=format:%Y-%m-%d",
            "--format=@@%h|%at|%ct|%ad|%cd|%s",
            "--numstat",
            // Završni `--` kaže gitu „dalje nema putanja": bez njega je ime grane dvosmisleno s
            // datotekom istog imena (nalaz M2). Mora biti ZADNJI — sve iza `--` git čita kao
            // putanju, pa bi `--` odmah iza grane pojeo naše opcije.
            "--",
        ])
    }
    fn branches(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError> {
        // `merged` dolazi iz posebnog poziva (git ga računa naspram trenutnog HEAD-a preko
        // `--merged`), a starost i „ahead" iz `for-each-ref`/`rev-list` po grani.
        let merged: std::collections::HashSet<String> = self
            .run(&[
                "branch",
                "--merged",
                default_branch,
                "--format=%(refname:short)",
            ])?
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        let mut out = Vec::new();
        for line in self
            .run(&[
                "for-each-ref",
                "refs/heads",
                "--format=%(refname:short)|%(authordate:unix)",
            ])?
            .lines()
        {
            let Some((name, t)) = line.trim().split_once('|') else {
                continue;
            };
            // `unwrap_or(0)` je svjesna odluka: git nikad ne ispisuje ne-broj u
            // `%(authordate:unix)`, pa je jedini realni ishod parsiranja uspjeh.
            let last_commit_time: i64 = t.parse().unwrap_or(0);
            let ahead_of_default = if name == default_branch {
                0
            } else {
                let range = format!("{default_branch}..{name}");
                self.run(&["rev-list", "--count", &range])?
                    .trim()
                    .parse()
                    .unwrap_or(0)
            };
            out.push(BranchInfo {
                name: name.to_string(),
                last_commit_time,
                ahead_of_default,
                merged: name == default_branch || merged.contains(name),
            });
        }
        Ok(out)
    }
    fn worktrees(&self) -> Result<Vec<PathBuf>, IoError> {
        Ok(self
            .run(&["worktree", "list", "--porcelain"])?
            .lines()
            .filter_map(|l| l.strip_prefix("worktree "))
            .map(|p| PathBuf::from(p.trim()))
            .collect())
    }
    fn last_change(&self, path: &str) -> Result<Option<i64>, IoError> {
        let s = self.run(&["log", "-1", "--format=%at", "--", path])?;
        Ok(s.trim().parse().ok())
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
