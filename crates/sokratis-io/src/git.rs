//! ZAŠTO RUST OVAKO (cigle M1/15 i M1/16 — git kroz proces, grane i radna stabla)
//! `std::process::Command` gradi poziv bez shella (nema quotinga, nema injekcije), a greška se
//! MAPIRA u `IoError` po uzroku (`map_err`); `String::from_utf8_lossy` toleriše tuđi ne-UTF-8
//! bajt umjesto da sruši izvještaj. `HashSet<String>` daje O(1) „je li grana spojena", `let-else`
//! preskače neispravan redak, a `unwrap_or(0)` na `%(authordate:unix)` je svjesna alternativa.
//!
//! Cigla M2/11 (performanse, dug M11): `Cell<u32>` broji pokrenute procese kroz `&self` bez
//! `mut` (trait `GitSource` posuđuje nepromjenjivo) — mjerač za `tests/perf.rs`. `last_changes`
//! zamjenjuje `last_change` po datoteci jednim `git log --name-only`; `branches` zamjenjuje
//! poziv-po-grani jednim `for-each-ref` s atomom `ahead-behind`, uz stari put kao rezervu
//! (`branches_per_ref`) kad atom ne postoji (git < 2.41). Let-chain (`if let … && let …`) u
//! `last_changes` je stabilan od Rust 1.88 — čita se kao jedna provjera, ne ugniježđeni `if`-ovi.
use crate::IoError;
use sokratis_core::BranchInfo;
use std::collections::HashMap;
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
    /// Zadnja promjena (unix-vrijeme) za SVAKU putanju koju je git ikad dirnuo pod danim
    /// pathspecovima — jednim `git log --name-only` (cigla M2/11, dug M11). Zamjenjuje poziv
    /// `last_change` po datoteci: `docs()` je s 60 dokumenata trošio 60 procesa na isto pitanje.
    fn last_changes(&self, pathspecs: &[&str]) -> Result<HashMap<String, i64>, IoError>;
    fn common_dir(&self) -> Result<PathBuf, IoError>;
    fn toplevel(&self) -> Result<PathBuf, IoError>;
    fn branch_exists(&self, name: &str) -> Result<bool, IoError>;
    fn current_branch(&self) -> Result<String, IoError>;
}
#[derive(Debug)]
pub struct GitCli {
    pub repo: PathBuf,
    /// Broji koliko je `git` procesa ovaj `GitCli` pokrenuo (mjerač; `tests/perf.rs`, cigla M2/11).
    /// `Cell` daje unutarnju promjenjivost kroz `&self` — `GitSource` metode posuđuju nepromjenjivo
    /// (trait to zahtijeva), a brojač ipak mora rasti pri svakom pozivu.
    calls: std::cell::Cell<u32>,
}
impl GitCli {
    pub fn new(repo: impl Into<PathBuf>) -> Self {
        Self {
            repo: repo.into(),
            calls: std::cell::Cell::new(0),
        }
    }

    /// Koliko je `git` procesa pokrenuto kroz ovaj `GitCli` otkad je stvoren (mjerač, ne semantika).
    pub fn calls(&self) -> u32 {
        self.calls.get()
    }

    /// Pokreće `git` u repozitoriju bez shella i mapira ishod u `IoError`.
    fn run(&self, args: &[&str]) -> Result<String, IoError> {
        self.calls.set(self.calls.get() + 1);
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

    /// Rezerva za `branches()` na gitu starijem od 2.41 (nema atom `ahead-behind`): isti rezultat,
    /// ali TRI poziva po grani (`branch --merged` jednom + `rev-list --count` po grani) — M1 put,
    /// izmjereno u cigli M2/11 kao ~94 procesa nad 30 grana. Ostaje u kodu kao dokazana rezerva.
    fn branches_per_ref(&self, default_branch: &str) -> Result<Vec<BranchInfo>, IoError> {
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
}

/// Parsira jedan redak `for-each-ref --format=ime|unix|ahead behind` u `BranchInfo`. `ahead` je
/// broj commitova koje grana ima a zadana nema (isto što je stari kod računao s
/// `rev-list default..name --count`), pa je `ahead == 0` istovjetno onome što je `git branch
/// --merged` govorio — zadana grana provjerava se i imenom, za slučaj da git ikad vrati drukčiji
/// rezultat za samo-usporedbu.
fn parse_ahead_behind(out: &str, default_branch: &str) -> Vec<BranchInfo> {
    let mut result = Vec::new();
    for line in out.lines() {
        let mut parts = line.trim().splitn(3, '|');
        let (Some(name), Some(t), Some(ahead_behind)) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        let last_commit_time: i64 = t.parse().unwrap_or(0);
        let ahead_of_default: u32 = ahead_behind
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        result.push(BranchInfo {
            name: name.to_string(),
            last_commit_time,
            ahead_of_default,
            merged: name == default_branch || ahead_of_default == 0,
        });
    }
    result
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
        // Jedan `for-each-ref` s atomom `%(ahead-behind:<default>)` (git ≥ 2.41) zamjenjuje
        // stara TRI poziva po grani (`branch --merged` jednom + `rev-list --count` po grani) —
        // cigla M2/11, dug M11. Ako git ne zna atom (stariji od 2.41), poruka o grešci sadrži
        // ime atoma; rezerva je stari put, sporiji ali dokazano isti rezultat (test ostaje zelen).
        let fmt = format!(
            "--format=%(refname:short)|%(authordate:unix)|%(ahead-behind:{default_branch})"
        );
        match self.run(&["for-each-ref", "refs/heads", &fmt]) {
            Ok(out) => Ok(parse_ahead_behind(&out, default_branch)),
            Err(IoError::Git { stderr, .. }) if stderr.contains("ahead-behind") => {
                self.branches_per_ref(default_branch)
            }
            Err(e) => Err(e),
        }
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
    fn last_changes(&self, pathspecs: &[&str]) -> Result<HashMap<String, i64>, IoError> {
        // Log je od najnovijeg prema starijem, pa je PRVA pojava putanje njezina zadnja promjena.
        // Jedan proces za SVE putanje odjednom (cigla M2/11) umjesto `last_change` po datoteci.
        let mut args = vec!["log", "--format=@@%at", "--name-only", "--"];
        args.extend_from_slice(pathspecs);
        let out = self.run(&args)?;
        let mut map = HashMap::new();
        let mut current: Option<i64> = None;
        for line in out.lines() {
            if let Some(ts) = line.strip_prefix("@@") {
                current = ts.trim().parse().ok();
            } else if !line.trim().is_empty()
                && let Some(ts) = current
            {
                map.entry(line.trim().to_string()).or_insert(ts);
            }
        }
        Ok(map)
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
