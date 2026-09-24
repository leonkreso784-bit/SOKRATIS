//! ZAŠTO RUST OVAKO (cigla M1/17 — projekt)
//! `Project` POSJEDUJE `GitCli` i `Profile`; metode posuđuju `&self`. Za ručne JSON-datoteke je
//! `Err(e) if e.kind() == NotFound` JEDINO opravdanje za pad na zadano — svaka druga greška
//! čitanja (putanja je direktorij, nema dozvole) mora biti vidljiva, jer tiho gutanje je laž.
//! Za obični tekst `fs::read_to_string(..).ok()` je dovoljan; rekurzivni `walk` puni `&mut Vec`.
//!
//! Cigla M2/10 (pisanje ručnih podataka, S-015): `write_atomic` piše `.tmp` pored ciljne datoteke
//! pa zove `rename` — `rename` je atoman na razini datotečnog sustava, pa pad usred pisanja nikad
//! ne ostavi pola JSON-a na mjestu datoteke koju git prati. `BTreeMap` u `write_override` (umjesto
//! `HashMap` kojim se čita) daje deterministički redoslijed ključeva — stabilan tekst, `git diff`
//! od jednog retka.
//!
//! Cigla M2/11 (performanse, dug M11): `docs()` zove `last_changes` JEDNOM za sve datoteke
//! (`HashMap<String, i64>`) umjesto `last_change` u petlji — isti podatak, 60 puta manje procesa
//! na 60 dokumenata.
//!
//! Cigla M2/12 (detached HEAD): `input` sad razlikuje `ref_name` (što git čita) od `label` (što
//! izvještaj pokazuje) — u detached stanju grane nema, ali commit postoji, pa `git.head_sha()`
//! (poziva se SAMO na tom rubu) daje `HEAD@<sha>` umjesto lažne poruke „nema commita".
//!
//! Cigla M2/14 (birač raspona, ograda putanja, dug I9): `open` zove `profile.validate_paths()`
//! ODMAH nakon učitavanja profila — prije ove cigle je ogradu putanja provjeravala SAMO jezgra
//! (`build_report`), pa je alat mogao pročitati datoteke izvan repoa (npr. `docs_dir: "../.."`)
//! prije nego što bi itko prijavio grešku. `input_between` postaje jedino mjesto koje sastavlja
//! `ReportInput`; `input` je tanka omotnica (`until: None`) da postojeći pozivatelj (CLI) ostane
//! nepromijenjen.
//!
//! Cigla M2/14b (potrošač keša): `input_between`/`input_cached` su sad tanke omotnice oko
//! privatne `input_with`, koja uzima `Option<&dyn CommitCache>` — `None` čita git izravno,
//! `Some(cache)` ide kroz `cached_log`. Jedna razlika u tijelu, dva javna imena.
//!
//! Cigla M2/29b (`common_dir` kao tekst, javni `today()`): `common_dir` prolazi kroz `normalized`
//! ISTO kao `profile_path` (I5) — usporedba TEKSTA (ne `PathBuf ==`) inače vidi glavno i sporedno
//! radno stablo kao dva projekta. `today()` postaje slobodna funkcija jer je desktopu treba bez
//! vlastite ovisnosti o `chrono`.
//!
//! Cigla M2/49 (S-032): `input_with` prevodi `Profile.branch_scope` (T48) u `Scope` (T49) i, SAMO
//! kad je opseg sve grane, plaća jedan dodatan proces (`commit_sources`) da napuni kartu `sha →
//! grana` — Leonov rad izvan zadane grane time ulazi u brojke, ali repo koji ostaje pri paritetu
//! (`branch_scope = default`) ne plaća ništa novo.
use crate::{CommitCache, GitCli, GitSource, IoError, Scope, cached_log};
use sokratis_core::{BranchScope, DocFile, Profile, ReportInput, Vision, WorkKind};
use std::collections::{BTreeMap, HashMap};
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

/// Današnji lokalni datum, `YYYY-MM-DD` — jedino mjesto koje ga računa (S-010); `input_with` ga
/// zove umjesto vlastitog izraza, a DESKTOP (bez izravne ovisnosti o `chrono`) ga zove odavde za
/// zadani raspon i ključ dnevne snimke (cigla M2/29b).
pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Piše `text` u `path` atomarno: prvo `.tmp` pored, pa `rename` (na Windowsu zamjenjuje
/// postojeću). Pad usred pisanja tako nikad ne ostavi pola JSON-a na mjestu datoteke koju git
/// prati.
fn write_atomic(path: &Path, text: &str) -> Result<(), IoError> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
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
        // M2/29b: u glavnom stablu `--git-common-dir` vraća RELATIVNO `.git` (pa `path_of` spoji
        // `root` s `/` i `join`-ov `\` — razdjelnici se pomiješaju), a u sporednom radnom stablu
        // vraća APSOLUTNO, čisto `/`. `PathBuf ==` to ne vidi (uspoređuje komponente), ali
        // `sokratis-store` uspoređuje TEKST — isti `normalized(...)` kao za `profile_path` (I5)
        // ovdje daje isti tekst iz oba stabla, pa isti projekt ostaje JEDAN (S-015).
        let common_dir = normalized(git.common_dir()?);
        let profile_path = normalized(root.join(".sokratis").join("profile.json"));
        let profile = match std::fs::read_to_string(&profile_path) {
            Ok(s) => serde_json::from_str(&s).map_err(|source| IoError::Profile {
                path: profile_path.clone(),
                source,
            })?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Profile::default(),
            Err(e) => return Err(IoError::Io(e)),
        };
        // Dug I9 (repo je JAVAN): ograda putanja se provjerava OVDJE, pri otvaranju, ne tek kad
        // netko zatraži izvještaj — inače `docs()`/`input_between()` mogu pročitati datoteke izvan
        // repoa prije nego što ijedan poziv jezgre stigne prijaviti profil neispravnim.
        profile
            .validate_paths()
            .map_err(|source| IoError::ProfileInvalid {
                path: profile_path.clone(),
                source,
            })?;
        Ok(Project {
            root,
            common_dir,
            profile,
            git,
        })
    }

    /// Glavno stablo repoa: roditelj zajedničkog `.git` direktorija. Sva radna stabla ga dijele,
    /// pa je to JEDINO mjesto za ručne podatke (S-015) — Sokratis piše samo ovdje, nikad u
    /// sporedno radno stablo iz kojeg je možda otvoren, i pritom ništa ne commita.
    pub fn main_root(&self) -> PathBuf {
        self.common_dir
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| self.root.clone())
    }

    /// Putanja ručne datoteke (`overrides.json`, `visions.json`) u `.sokratis/` GLAVNOG stabla —
    /// jedno mjesto koje i čita i piše (za razliku od `profile.json`, koji ostaje uz `root`, jer
    /// deklaraciju konvencija ne mijenja sučelje).
    fn manual_path(&self, name: &str) -> PathBuf {
        normalized(self.main_root().join(".sokratis").join(name))
    }

    /// Ručni overridi klasifikacije po SHA-i commita (`.sokratis/overrides.json`).
    pub fn overrides(&self) -> Result<HashMap<String, WorkKind>, IoError> {
        read_overrides_or_empty(&self.manual_path("overrides.json"))
    }

    /// Ručno upisane vizije (`.sokratis/visions.json`).
    pub fn visions(&self) -> Result<Vec<Vision>, IoError> {
        read_visions_or_empty(&self.manual_path("visions.json"))
    }

    /// Upisuje/uklanja override klasifikacije za commit `sha` (`None` uklanja) i vraća putanju
    /// upisane datoteke — DESKTOP je potiskuje u watcheru (S-016), da vlastiti zapis ne pročita
    /// kao vanjsku promjenu.
    pub fn write_override(&self, sha: &str, kind: Option<WorkKind>) -> Result<PathBuf, IoError> {
        let mut all: BTreeMap<String, WorkKind> = self.overrides()?.into_iter().collect();
        match kind {
            Some(k) => {
                all.insert(sha.to_string(), k);
            }
            None => {
                all.remove(sha);
            }
        }
        let path = self.manual_path("overrides.json");
        write_atomic(&path, &serde_json::to_string_pretty(&all)?)?;
        Ok(path)
    }

    /// Upisuje cijeli popis vizija (zamjena, ne spajanje) i vraća putanju upisane datoteke.
    pub fn write_visions(&self, visions: &[Vision]) -> Result<PathBuf, IoError> {
        let path = self.manual_path("visions.json");
        write_atomic(&path, &serde_json::to_string_pretty(visions)?)?;
        Ok(path)
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
        // Cigla M2/11 (dug M11): jedan `git log --name-only` za SVE dokumente umjesto poziva po
        // datoteci — s 60 dokumenata je stari put trošio 60 git-procesa na isto pitanje.
        // `docs_dir` pokriva podstablo rekurzivno; `:(glob)*.md` dodaje `.md` iz KORIJENA (glob bez
        // `**` ne silazi u podmape), jer korijenske datoteke inače ne bi bile pokrivene niti jednim
        // pathspecom kad `docs_dir` nije korijen.
        let changes = self
            .git
            .last_changes(&[self.profile.docs_dir.as_str(), ":(glob)*.md"])?;
        let mut out = Vec::new();
        for p in paths {
            let rel = p
                .strip_prefix(&self.root)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            out.push(DocFile {
                content: std::fs::read_to_string(&p)?,
                last_change_time: changes.get(&rel).copied(),
                path: rel,
            });
        }
        Ok(out)
    }

    /// Sastavlja `ReportInput` za jezgru: grana, log od `profile.log_since()` do `until` (ako je
    /// zadan; direktno iz gita ili kroz keš — vidi `cache` niže), dnevnik/plan ako postoje,
    /// dokumentacija, grane, ručni podaci i „sada"/„danas" (`chrono` — jedino mjesto sata u
    /// sustavu). `until` je gornja granica razdoblja (S-011); bez nje (`None`) log ide do kraja.
    fn input_with(
        &self,
        since: Option<&str>,
        until: Option<&str>,
        cache: Option<&dyn CommitCache>,
    ) -> Result<ReportInput, IoError> {
        // (ref_name, label): git čita `ref_name`, izvještaj pokazuje `label`. Razlikuju se samo u
        // detached stanju, gdje grane nema, a commita ima — poruka „nema commita" bi lagala.
        let (ref_name, label) = if self.git.branch_exists(&self.profile.default_branch)? {
            (
                self.profile.default_branch.clone(),
                self.profile.default_branch.clone(),
            )
        } else {
            // M2: `git branch --show-current` ispiše ime grane i u repou BEZ ijednog commita
            // (HEAD je „unborn"), pa ime nije dokaz da grana postoji — provjerava se referenca.
            // Bez toga bi git odgovorio svojim savjetom o `--`, a ne rečenicom o repozitoriju.
            let current = self.git.current_branch()?;
            if !current.is_empty() && self.git.branch_exists(&current)? {
                (current.clone(), current)
            } else {
                // M2/12: ni zadana grana ni `current_branch()` ne postoje — repo je ili prazan
                // (bez ijednog commita, pa `rev-parse HEAD` puca s `IoError::Git`) ili je HEAD
                // DETACHED (grane nema, commit ima, pa `rev-parse` uspije). Razlika je jedini
                // dokaz je li poruka „nema commita" istinita ili laž.
                match self.git.head_sha() {
                    Ok(sha) => ("HEAD".to_string(), format!("HEAD@{sha}")),
                    Err(IoError::Git { .. }) => return Err(IoError::NoCommits(self.root.clone())),
                    Err(e) => return Err(e),
                }
            }
        };
        let read_opt = |rel: &str| std::fs::read_to_string(self.root.join(rel)).ok();
        let since = since
            .map(str::to_string)
            .unwrap_or_else(|| self.profile.since.clone());
        // I1: prozor dovlačenja je NAJRANIJE od dvoje — profilskog (`log_since()`, koji uključuje
        // i početke zatvorenih faza) i korisnikova `--since`. Bez tog `min`-a je `--since`
        // stariji od profila tiho dobivao kraći log nego što `Report.since` tvrdi.
        let fetch_since = self.profile.log_since().min(since.clone());
        // S-032: profil bira opseg; `ref_name` je ista referenca kao do sada (zadana → trenutna →
        // HEAD), pa detached HEAD i repo bez zadane grane rade kao prije.
        let scope = match self.profile.branch_scope {
            BranchScope::AllBranches => Scope::AllBranches,
            BranchScope::DefaultBranch => Scope::Branch(&ref_name),
        };
        // Jedina razlika između dva puta (cigla M2/14b): `Some(cache)` ide kroz `cached_log`
        // (dovlači SAMO nedostajuće commite), `None` čita git izravno kao prije.
        let git_log = match cache {
            Some(cache) => cached_log(&self.git, cache, scope, &fetch_since, until)?,
            None => self.git.log(scope, &fetch_since, until)?,
        };
        // Karta `sha → grana` postoji SAMO kad se mjere sve grane — jedan proces više (spec §1.1).
        let commit_branches = match self.profile.branch_scope {
            BranchScope::AllBranches => self.git.commit_sources(&ref_name, &fetch_since, until)?,
            BranchScope::DefaultBranch => HashMap::new(),
        };
        Ok(ReportInput {
            git_log,
            // M2/47: privremeno jedno stablo; IO-2 (T50) čita sva.
            diaries: read_opt(&self.profile.diary_path).into_iter().collect(),
            plan: read_opt(&self.profile.plan_path),
            docs: self.docs()?,
            branches: self.git.branches(&ref_name)?,
            overrides: self.overrides()?,
            visions: self.visions()?,
            now: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            today: today(),
            since,
            until: until.map(str::to_string),
            branch: label,
            scope: self.profile.branch_scope,
            commit_branches,
            // M2/47: privremeno jedno stablo; IO-2 (T50) čita sva.
            worktrees: 1,
        })
    }

    /// Prozor BEZ keša: git se čita izravno. CLI (T19) i pozivatelji koji ne biraju gornju
    /// granicu i dalje zovu `input`, tanku omotnicu oko ovoga.
    pub fn input_between(
        &self,
        since: Option<&str>,
        until: Option<&str>,
    ) -> Result<ReportInput, IoError> {
        self.input_with(since, until, None)
    }

    /// Prozor KROZ keš (cigla M2/14b, S-014): `cached_log` pita `rev_list` ŠTO je dostižno SADA i
    /// dovlači SAMO commite kojih `cache` još nema — amend/rebase/reset nikad ne ostave stari SHA
    /// u brojkama, iako ostaje u kešu kao smeće.
    pub fn input_cached(
        &self,
        since: Option<&str>,
        until: Option<&str>,
        cache: &dyn CommitCache,
    ) -> Result<ReportInput, IoError> {
        self.input_with(since, until, Some(cache))
    }

    /// `input(since)` = `input_between(since, None)` — CLI (T19) i pozivatelji koji ne biraju
    /// gornju granicu i dalje zovu ovo, dvoargumentno ime.
    pub fn input(&self, since: Option<&str>) -> Result<ReportInput, IoError> {
        self.input_between(since, None)
    }
}
