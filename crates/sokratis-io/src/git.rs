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
//!
//! Krug popravka 1 (recenzija): `last_changes` je gubio doprinos merge-commita (zadano
//! `--name-only` ne ispisuje datoteke za merge) i tiho gutao ne-ASCII imena (zadano
//! `core.quotepath` ih escapea) — oba dokazana pokusom nad Sokrat Studyjem i pokrivena testom
//! `tests/last_changes.rs`. Vidi komentar UZ POZIV u `last_changes` za detalje dviju opcija.
//!
//! Cigla M2/12 (detached HEAD): `head_sha` je nova metoda traita — jedan poziv koji ima smisla
//! SAMO na rubu (grane nema, commit ima), pa ne diramo brojač procesa na uobičajenom putu
//! (`tests/perf.rs` broji iste 4 procesa kao prije). Poziva je iz `project.rs`, ne odavde.
//!
//! Cigla M2/14 (birač raspona, S-011 dopuna 2): `log` dobiva `until: Option<&str>` — `Option`
//! umjesto praznog stringa jer „nema gornje granice" i „granica je prazan datum" NISU isto stanje,
//! a `match`/`if let` nad `Option` to prisiljava na svakom pozivu (ne pušta ni jedan slučaj da
//! prođe tiho). Rezerva prema naprijed je DVA dana (`next_day` dvaput), ne jedan kao za `since`:
//! smjer je suprotan, pa je i račun zone suprotan (vidi doc-komentar `window_args`).
//!
//! Cigla M2/14b (potrošač keša): `window_args` izdvaja `since_arg`/`until_arg` iz `log` u JEDNU
//! funkciju koju sad zove i `rev_list` — dva puta se ne mogu razići jer postoji samo jedan. Nova
//! `run_with_stdin` otvara `Stdio::piped()` na stdin/stdout/stderr i `take()`-a `child.stdin`: pipe
//! se MORA zatvoriti (drop) prije `wait_with_output()`, inače `git --stdin` čeka EOF zauvijek.
//! Mapiranje ishoda procesa u `IoError` je izdvojeno u `output_to_result`/`spawn_error` — isto za
//! `run` i `run_with_stdin`, da se ne kopira.
//!
//! Cigla M2/44 (konzolni prozor, kvar 4 iz Leonovih nalaza): `git_command` je JEDINO mjesto koje
//! gradi `Command::new("git")` i na Windowsu postavlja `CREATE_NO_WINDOW` kroz `CommandExt`
//! (`#[cfg(windows)]` — kod za drugu platformu se ne kompilira, ne samo ne izvršava). Zastavica se
//! ne može pročitati natrag, pa test `command_new_lives_only_inside_git_command` čita IZVOR.
//!
//! Cigla M2/49 (S-032): `Scope<'a>` je enum s LIFETIMEOM umjesto `String` — posuđuje ime grane od
//! pozivatelja (`&self.profile.default_branch`) umjesto da ga klonira pri svakom pozivu `log`.
//! `commit_sources` zove `git log --branches --not <default_ref>`: `--not` negira SVE reference
//! iza sebe do kraja retka, pa `--branches` MORA doći PRIJE njega, inače bi negirao i sve grane.
//! `parse_commit_sources` je čista funkcija (tekst → karta) izdvojena iz metode zbog testa: SHA
//! nikad ne sadrži `|`, pa `split_once('|')` dijeli na PRVOM — ostatak retka, i s `|` u imenu,
//! pripada grani (Windows ne dopušta `|` u imenu grane, ali linux/macOS repozitorij može).
use crate::IoError;
use sokratis_core::BranchInfo;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// Argumenti OBLIKA ispisa po commitu — dijele ih `log` (šetnja cijelom poviješću) i `log_commits`
/// (samo navedeni SHA-ovi): isti tekstualni ugovor, jedan izvor istine.
const LOG_FORMAT_ARGS: [&str; 3] = [
    "--date=format:%Y-%m-%d",
    "--format=@@%h|%at|%ct|%ad|%cd|%s",
    "--numstat",
];

/// JEDINO mjesto koje gradi `git` proces (kvar 4, spec 1.0.0 §4.2). Instalirana aplikacija je
/// `windows_subsystem = "windows"` (`main.rs`), pa bi svaki dijete-proces bez zastavice na Windowsu
/// nakratko otvorio konzolni prozor — pri svakom osvježenju, watcheru, promjeni raspona.
/// `CREATE_NO_WINDOW` (0x08000000, `winbase.h`) to gasi; na drugim sustavima ništa se ne mijenja.
/// `#[cfg(windows)]` bira kod PRI KOMPILACIJI, pa `CommandExt` ne postoji na Linuxu ni u `cargo check`
/// za druge mete — nema `if cfg!` s mrtvom granom.
fn git_command(repo: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(repo);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

/// Greška spawn-a procesa (`git` nije na PATH-u ili druga IO greška) — dijeli je `run` i
/// `run_with_stdin`.
fn spawn_error(e: std::io::Error) -> IoError {
    if e.kind() == std::io::ErrorKind::NotFound {
        IoError::GitMissing
    } else {
        IoError::Io(e)
    }
}

/// Mapira izlaz gotovog procesa (status + stdout/stderr) u `IoError` — dijele je `run` i
/// `run_with_stdin`, da mapiranje postoji na jednom mjestu.
fn output_to_result(out: Output, repo: &Path, args: &[&str]) -> Result<String, IoError> {
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if stderr.contains("not a git repository") {
            return Err(IoError::NotARepo(repo.to_path_buf()));
        }
        return Err(IoError::Git {
            cmd: args.join(" "),
            stderr,
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// `since_arg`/`until_arg` za `log` I `rev_list` — jedna funkcija da se prozor dovlačenja ne može
/// razići između dvije naredbe (cigla M2/14b — prije refaktora je svaka naredba gradila svoj).
///
/// `since_arg`: ` 00:00:00` fiksira sat na ponoć — git-ov parser datuma bez sata uzima TRENUTNO
/// DOBA DANA (sat kad se naredba pokreće), ne ponoć, izmjereno nad Sokrat Studyjem
/// (`--since=2026-08-29` u 17:15 → 183 commita, `--since='2026-08-29 00:00'` → 190 commita). Bez
/// fiksnog sata bi tablica ovisila o TOME KADA se izvještaj generira, ne samo o datumu. `prev_day`
/// dodaje JEDAN dan rezerve unatrag (nalaz I2): ta je ponoć u zoni STROJA, a datumi commita
/// (`%ad`/`%cd`) su u zoni COMMITA, pa bi git zapadno od pohranjenog pomaka odbacio commit koji
/// jezgra (`commit_date >= since`) zadržava.
///
/// `until_arg`: rezerva prema naprijed je DVA dana (`next_day` dvaput), ne jedan kao za `since` —
/// smjer je suprotan, pa je i račun zone suprotan. Primjer: commit datiran `until` u zoni −12:00
/// pada na `until+1 12:00 UTC`, a stroj u zoni −12:00 ima ponoć `until+2` tek u `until+2 12:00 UTC`
/// — jedan dan rezerve (kao za `since`) ne bi bio dovoljan u OVOM smjeru, jer se granica pomiče
/// prema BUDUĆNOSTI, ne prošlosti.
///
/// U OBA slučaja jezgra presuđuje `since <= commit_date <= until` string-usporedbom (S-011), pa
/// rezerva ovdje NE mijenja nijednu brojku — samo osigurava da git ne odbaci commit prije nego što
/// jezgra stigne odlučiti. `unwrap_or_else` vraća neispravan datum nepromijenjen u oba slučaja:
/// njega jezgra prijavi kao `ParseError::BadDate` (C2), ne ovaj sloj.
fn window_args(since: &str, until: Option<&str>) -> (String, Option<String>) {
    let from = sokratis_core::civil::prev_day(since).unwrap_or_else(|| since.to_string());
    let since_arg = format!("--since={from} 00:00:00");
    let until_arg = until.map(|u| {
        let plus2 = sokratis_core::civil::next_day(u)
            .and_then(|d| sokratis_core::civil::next_day(&d))
            .unwrap_or_else(|| u.to_string());
        format!("--until={plus2} 00:00:00")
    });
    (since_arg, until_arg)
}

/// Koje reference `log`/`rev_list` obilaze (S-032). `Branch(ime)` je današnje ponašanje (paritet,
/// `--scope default`); `AllBranches` je `--branches`: sve lokalne grane, svaki commit JEDNOM (git ga
/// dedupira po SHA-i), bez remote-tracking referenci i tagova. Lifetime `'a` posuđuje ime grane od
/// pozivatelja (`&self.profile.default_branch`) umjesto da ga klonira za svaki poziv.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope<'a> {
    Branch(&'a str),
    AllBranches,
}

impl<'a> Scope<'a> {
    /// Argument revizije koji git razumije: ime grane ili `--branches`.
    fn rev_arg(self) -> &'a str {
        match self {
            Scope::Branch(name) => name,
            Scope::AllBranches => "--branches",
        }
    }
}

/// Tekst `git log --format=%h|%S` → karta `sha → grana`. Dijeli na PRVOM `|` (`split_once`): SHA
/// nikad ne sadrži `|`, pa ostatak retka — i s `|` u imenu — pripada grani. Prazni retci preskaču.
fn parse_commit_sources(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|l| l.split_once('|'))
        .map(|(sha, branch)| (sha.trim().to_string(), branch.trim().to_string()))
        .collect()
}

pub trait GitSource {
    /// `since` je goli datum `YYYY-MM-DD`. Implementacija MORA dodati sat `00:00:00`: git-ov
    /// parser datuma bez sata uzima TRENUTNO DOBA DANA (sat kad se naredba pokreće), ne ponoć —
    /// izmjereno nad Sokrat Studyjem (`--since=2026-08-29` u 17:15 → 183 commita,
    /// `--since='2026-08-29 00:00'` → 190). Bez fiksnog sata bi tablica ovisila o tome KADA se
    /// izvještaj generira, ne samo o datumu. Uz to MORA dovući DAN VIŠE (nalaz I2): ta je ponoć
    /// u zoni stroja, a datumi commita u zoni commita, pa granicu mora presuditi jezgrin
    /// `commit_date >= since`, ne git. Rezerva ne mijenja nijednu brojku — jezgra je odbaci.
    ///
    /// `until` je gornja granica (cigla M2/14, S-011 dopuna 2): `None` znači „bez gornje granice"
    /// (do kraja loga). Kad je zadan, implementacija MORA dodati DVA dana rezerve prema naprijed —
    /// vidi doc-komentar `window_args` za izračun. Jezgra presuđuje
    /// `commit_date <= until`, pa rezerva ovdje ne mijenja nijednu brojku, samo osigurava da git
    /// ne odbaci commit prije nego što jezgra stigne odlučiti.
    fn log(&self, scope: Scope<'_>, since: &str, until: Option<&str>) -> Result<String, IoError>;
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
    /// Kratki SHA trenutnog `HEAD` (`rev-parse --short HEAD`). Jedini poziv koji ima smisla u
    /// DETACHED stanju (grane nema, ali commit postoji) — vidi `project.rs::input`.
    fn head_sha(&self) -> Result<String, IoError>;
    /// Kratki SHA-ovi (`%h`) commita u ISTOM prozoru i ISTIM redom kojim ih `log` ispisuje (cigla
    /// M2/14b, potrošač keša) — brz poziv bez `--numstat` da presudi ŠTO je dostižno.
    fn rev_list(
        &self,
        scope: Scope<'_>,
        since: &str,
        until: Option<&str>,
    ) -> Result<Vec<String>, IoError>;
    /// `git log` SAMO za navedene SHA-ove (bez šetnje poviješću), u istom obliku kao `log`.
    /// Prazan popis → prazan tekst BEZ pokretanja procesa (keš koji već zna sve ne smije platiti
    /// cijenu procesa za ništa).
    fn log_commits(&self, shas: &[String]) -> Result<String, IoError>;
    /// `sha → grana` SAMO za commite izvan `default_ref` (S-032): `git log --branches --not
    /// <default_ref> --format=%h|%S`. `%S` = kratko ime reference kojom je git commit DOSEGAO; commit
    /// dostižan iz dviju nespojenih grana dobiva jednu od njih (gitov obilazak presuđuje). Commit
    /// kojeg nema u karti je na zadanoj grani. Isti prozor (`window_args`) kao `log`/`rev_list`.
    fn commit_sources(
        &self,
        default_ref: &str,
        since: &str,
        until: Option<&str>,
    ) -> Result<HashMap<String, String>, IoError>;
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
        let out = git_command(&self.repo)
            .args(args)
            .output()
            .map_err(spawn_error)?;
        output_to_result(out, &self.repo, args)
    }

    /// Pokreće `git` s ULAZOM na stdin (cigla M2/14b — `log_commits`): `Stdio::piped()` otvara sve
    /// tri cijevi, `child.stdin.take()` izvadi stdin iz `Child`-a da ga možemo POSUDITI za pisanje
    /// i onda ZATVORITI (drop) — bez zatvaranja `git --stdin` čeka EOF koji nikad ne stiže. Ako
    /// pisanje padne (proces je već izašao), svejedno se čeka ishod: proces koji je pukao svojom
    /// greškom je vjerodostojniji uzrok nego naš `write_all`.
    ///
    /// Cijeli stdin se upiše PRIJE čitanja stdouta (`wait_with_output` čita oba tek nakon petlje
    /// upisa) — sigurno SAMO za naredbe koje iscrpe stdin prije prvog bajta izlaza (`git log
    /// --stdin` sve revizije pročita u `setup_revisions` prije ijednog retka ispisa). Naredba koja
    /// bi ispis slala USPOREDO s čitanjem stdina bi se mogla zaglaviti kad stdout napuni cijev.
    fn run_with_stdin(&self, args: &[&str], input: &str) -> Result<String, IoError> {
        self.calls.set(self.calls.get() + 1);
        let mut child = git_command(&self.repo)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(spawn_error)?;
        let write_err = match child.stdin.take() {
            Some(mut stdin) => {
                let result = stdin.write_all(input.as_bytes());
                drop(stdin); // zatvara pipe (EOF) PRIJE cekanja na proces
                result.err()
            }
            None => None,
        };
        let out = child.wait_with_output().map_err(IoError::Io)?;
        match output_to_result(out, &self.repo, args) {
            Ok(text) => match write_err {
                Some(e) => Err(IoError::Io(e)),
                None => Ok(text),
            },
            Err(e) => Err(e),
        }
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
    fn log(&self, scope: Scope<'_>, since: &str, until: Option<&str>) -> Result<String, IoError> {
        // `window_args` fiksira sat na ponoć i dodaje rezervu zone (dan unatrag za `since`, dva
        // dana unaprijed za `until`) — vidi doc-komentar `GitSource::log` (trait) i `window_args`
        // (gore) za oba razloga. Jezgra presuđuje `since <= commit_date <= until` string-usporedbom,
        // pa rezerva ovdje ne mijenja nijednu brojku, samo osigurava da git ne odbaci commit prije
        // nego što jezgra stigne odlučiti.
        let (since_arg, until_arg) = window_args(since, until);
        let mut args = vec!["log", scope.rev_arg(), &since_arg];
        if let Some(a) = &until_arg {
            args.push(a);
        }
        args.push("--reverse");
        args.extend(LOG_FORMAT_ARGS);
        // Završni `--` kaže gitu „dalje nema putanja": bez njega je ime grane dvosmisleno s
        // datotekom istog imena (nalaz M2). Mora biti ZADNJI — sve iza `--` git čita kao
        // putanju, pa bi `--` odmah iza grane pojeo naše opcije.
        args.push("--");
        self.run(&args)
    }
    fn rev_list(
        &self,
        scope: Scope<'_>,
        since: &str,
        until: Option<&str>,
    ) -> Result<Vec<String>, IoError> {
        // ISTI `window_args` kao `log` — prozor dovlačenja i prozor „što je dostižno" ne mogu se
        // razići jer dolaze iz iste funkcije. `--abbrev-commit` daje `%h`-duljinu SHA-ova, isto što
        // `log`-ov `--format=@@%h|…` ispisuje u zaglavlju.
        let (since_arg, until_arg) = window_args(since, until);
        let mut args = vec!["rev-list", scope.rev_arg(), &since_arg];
        if let Some(a) = &until_arg {
            args.push(a);
        }
        args.extend(["--reverse", "--abbrev-commit", "--"]);
        Ok(self
            .run(&args)?
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }
    fn commit_sources(
        &self,
        default_ref: &str,
        since: &str,
        until: Option<&str>,
    ) -> Result<HashMap<String, String>, IoError> {
        let (since_arg, until_arg) = window_args(since, until);
        // `--not` negira SVE reference iza sebe do kraja: `--branches` mora doći PRIJE njega.
        let mut args = vec!["log", "--branches", "--not", default_ref, &since_arg];
        if let Some(a) = &until_arg {
            args.push(a);
        }
        args.extend(["--format=%h|%S", "--"]);
        Ok(parse_commit_sources(&self.run(&args)?))
    }
    fn log_commits(&self, shas: &[String]) -> Result<String, IoError> {
        // Prazan popis → prazan tekst BEZ pokretanja procesa: keš koji već zna sve ne smije
        // platiti cijenu procesa za ništa (brojač `calls` ostaje netaknut).
        if shas.is_empty() {
            return Ok(String::new());
        }
        // `--no-walk=unsorted` čita revizije SAMO s popisa (bez šetnje roditeljima) i ne sortira ih
        // — redoslijed ionako presuđuje `cached_log` (iz `rev_list`), ne ovaj poziv.
        let mut args = vec!["log", "--no-walk=unsorted", "--stdin"];
        args.extend(LOG_FORMAT_ARGS);
        let input = format!("{}\n", shas.join("\n"));
        self.run_with_stdin(&args, &input)
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
        //
        // Krug popravka 1 (recenzija, dokazano pokusom nad Sokrat Studyjem — docs/README.md):
        // - `-c core.quotepath=false` je GIT-OVA GLOBALNA opcija, MORA doći PRIJE `log`: bez nje
        //   git ne-ASCII znak u imenu datoteke ispisuje escapean u navodnicima (npr.
        //   `"docs/\304\215...md"`), pa ključ nikad ne pogodi mapu koju čita `project.rs`. NE
        //   MIJEŠATI s DRUGIM `-c` niže (`--diff-merges=combined`, kratica `-c`, ali opcija
        //   PODNAREDBE `log` — pišemo je puno ime baš da se ne zamijeni s ovom).
        // - Zadano `git log --name-only` NE ispisuje popis datoteka za merge-commit (dvosmisleno
        //   prema kojem roditelju uspoređivati), pa datoteka čija je STVARNA zadnja promjena bila
        //   UNUTAR merge-commita (razrješenje sudara) tiho dobiva stariji datum s neke od grana.
        //   `--diff-merges=combined` ispisuje datoteke koje se razlikuju od SVIH roditelja
        //   odjednom — isto mjerilo kojim git već presuđuje ULAZI li merge uopće u log kad je
        //   putanja zadana (zato stari `last_change`, bez ijedne od ovih opcija, ostaje ispravan:
        //   ta simplifikacija povijesti radi i bez `--name-only`). `tests/last_changes.rs` dokaz.
        let mut args = vec![
            "-c",
            "core.quotepath=false",
            "log",
            "--format=@@%at",
            "--diff-merges=combined",
            "--name-only",
            "--",
        ];
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
    fn head_sha(&self) -> Result<String, IoError> {
        Ok(self
            .run(&["rev-parse", "--short", "HEAD"])?
            .trim()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    /// Zastavica `CREATE_NO_WINDOW` se iz `Command`-a ne može pročitati natrag (std nema getter),
    /// pa test čita IZVOR: `Command::new` smije postojati SAMO unutar `git_command`. Igla se slaže
    /// u runtimeu (`concat!`) i broje se samo redci koji NISU komentari — inače bi test brojao
    /// vlastiti tekst i zaglavlje datoteke (Ruling R38). Pošteno ograničenje, zapisano u zaglavlju.
    #[test]
    fn command_new_lives_only_inside_git_command() {
        let src = include_str!("git.rs");
        let needle = concat!("Command::", "new(");
        let occurrences = src
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .filter(|line| line.contains(needle))
            .count();
        assert_eq!(
            occurrences, 1,
            "Command::new smije postojati samo u git_command()"
        );
        let helper_start = src.find("fn git_command(").expect("git_command postoji");
        let helper_end =
            src[helper_start..].find("\n}\n").expect("kraj git_command") + helper_start;
        let helper = &src[helper_start..helper_end];
        assert!(
            helper.contains(needle),
            "Command::new je unutar git_command"
        );
        assert!(
            helper.contains("creation_flags("),
            "git_command postavlja creation_flags"
        );
        assert!(
            helper.contains("0x0800_0000"),
            "CREATE_NO_WINDOW = 0x08000000"
        );
    }

    /// Review Focus #3 (Ruling R49): grana `y|z` se na Windowsu ne može stvoriti, pa rub `|` u
    /// imenu čuva parser, ne repo. SHA nema `|`, ostatak retka je grana.
    #[test]
    fn parse_commit_sources_keeps_pipe_inside_branch_name() {
        let map = super::parse_commit_sources("abc123|y|z\ndef456|feat/x\n\n");
        assert_eq!(map.get("abc123").map(String::as_str), Some("y|z"));
        assert_eq!(map.get("def456").map(String::as_str), Some("feat/x"));
        assert_eq!(map.len(), 2);
    }
}
