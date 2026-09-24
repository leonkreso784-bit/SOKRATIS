//! ZAŠTO RUST OVAKO (cigla M1/1 — profil · popravak C1)
//! `#[serde(default)]`: polje koje u JSON-u nedostaje uzima vrijednost iz `impl Default` — a taj
//! Default JE Sokrat Study (S-005). `deny_unknown_fields`: tipfeler u profilu je greška, ne tiho
//! ignoriranje. `Patterns` drži kompilirane regexe (`Regex` nije `Serialize`), a tamo
//! `Regex::captures_len()` prebroji grupe: regex bez grupe koju parser čita je greška, ne panika.
//!
//! ZAŠTO RUST OVAKO (cigla M2/8 — `inside_root`)
//! `Component` je enum kojim `std::path` razlaže putanju; `matches!` na njemu je ograda bez ijednog
//! string-uspoređivanja. `std::path` samo parsira tekst — ne dira disk — pa smije u jezgru bez I/O-a
//! (S-002); `canonicalize` (koji disk dira) ovdje ne smije nikad.
//!
//! ZAŠTO RUST OVAKO (cigla M2/9 — `test_path_exclude`)
//! Rani `return false` u `is_test_path` je stražarska klauzula (guard clause): isključenje se
//! provjerava PRIJE svih uključivih pravila, pa jedan pogodak u `test_path_exclude` presiječe
//! ostatak funkcije bez ugniježđenih `if`. Zadano `[]` znači da `.any()` nad praznim vektorom vrati
//! `false` i stara staza ostane netaknuta — paritet je zaštićen samim tipom, ne posebnim testom.
//!
//! ZAŠTO RUST OVAKO (cigla M2/48 — `branch_scope`)
//! `BranchScope` (definiran u `model.rs`, dijeljen s `Report.scope`) je `enum` s `#[serde(rename)]`
//! na svakoj varijanti: nepoznata vrijednost u JSON-u ("worktrees") sama padne s porukom koja
//! nabraja dopuštene nazive ("all", "default") — bez ručne `validate_*` funkcije, jer to serde već
//! radi bolje nego string-usporedba koju bismo inače pisali.
use crate::ParseError;
use crate::model::{BranchScope, WorkKind};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassifierRule {
    pub kind: WorkKind,
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedPhase {
    pub name: String,
    pub from: String,
    pub to: String,
    pub tag_pattern: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DocsWeights {
    pub dead_link: u8,
    pub not_indexed: u8,
    pub multiple_plans: u8,
    pub no_active_plan: u8,
    pub diary_in_definition: u8,
    pub lag: u8,
    pub key_file_budget: u8,
}

impl Default for DocsWeights {
    fn default() -> Self {
        Self {
            dead_link: 5,
            not_indexed: 3,
            multiple_plans: 15,
            no_active_plan: 10,
            diary_in_definition: 5,
            lag: 10,
            key_file_budget: 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Profile {
    pub default_branch: String,
    /// Koje grane ulaze u metrike (S-032): `"all"` = sve lokalne grane (zadano — prvi korisnik radi
    /// u granama), `"default"` = samo `default_branch` (paritet s `RAD.xlsx`, ponašanje do 1.0.0-pre).
    pub branch_scope: BranchScope,
    pub since: String,
    pub diary_path: String,
    pub changelog_path: String,
    pub plan_path: String,
    pub docs_dir: String,
    pub docs_index: String,
    pub plan_dir: String,
    pub plan_dir_ignore: Vec<String>,
    pub paused_marker: String,
    pub product_dir: String,
    pub key_file: String,
    pub key_file_budget_bytes: u64,
    pub diary_heading: String,
    pub diary_deploy_pattern: String,
    pub plan_brick: String,
    pub plan_phase_name: String,
    pub phase_tag: String,
    pub classifier: Vec<ClassifierRule>,
    pub gate_pattern: String,
    pub deploy_pattern: String,
    pub ci_fix_pattern: String,
    pub owner_name: String,
    pub test_path_prefixes: Vec<String>,
    pub test_path_contains: Vec<String>,
    pub test_path_suffixes: Vec<String>,
    /// Podputanje koje se NE broje kao test iako su pod testnom putanjom (npr. `fixtures/`).
    /// Zadano prazno: tablica broji sve pod `tests/` (S-005). Ponašanje: M2/9.
    pub test_path_exclude: Vec<String>,
    pub code_exclude_prefixes: Vec<String>,
    pub code_exclude_suffixes: Vec<String>,
    pub session_gap_hours: f64,
    pub session_start_hours: f64,
    pub closed_phases: Vec<ClosedPhase>,
    pub include_unmerged: bool,
    pub unmerged_warn_days: i64,
    pub unmerged_alert_days: i64,
    pub unmerged_alert_count: usize,
    pub docs_lag_warn_days: i64,
    pub docs_lag_alert_days: i64,
    pub docs_weights: DocsWeights,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            default_branch: "main".into(),
            branch_scope: BranchScope::AllBranches,
            since: "2026-08-29".into(),
            diary_path: "docs/records/PROGRESS.md".into(),
            changelog_path: "docs/records/CHANGELOG.md".into(),
            plan_path: "docs/plan/RASPORED.md".into(),
            docs_dir: "docs".into(),
            docs_index: "docs/README.md".into(),
            plan_dir: "docs/plan".into(),
            plan_dir_ignore: vec!["ROADMAP.md".into()],
            paused_marker: r"^\s*\*\*Status:\*\*\s*⏸️\s*PAUZIRAN\b".into(),
            product_dir: "docs/product".into(),
            key_file: "CLAUDE.md".into(),
            key_file_budget_bytes: 40_000,
            diary_heading: r"^## (\d{4}-\d{2}-\d{2})(?:\s*\(([^)]*)\))?\s*[—-]+\s*(.+)$".into(),
            diary_deploy_pattern: "deploy|main na `".into(),
            plan_brick: r"^\| \*\*(F\d)/(\d+)\*\*\s*(✅?)".into(),
            plan_phase_name: r"^### (F\d) · (.+)$".into(),
            phase_tag: r"^(F\d/\d|C\d[ab]?(?:/\d\w*)?|MREZA[- ]?[A-E]\d?(?: \(\d/\d\))?|R\d(?:/[A-Z0-9+]+)?|T\d|ALAT-\d|BUG-\d+|U\d)".into(),
            classifier: vec![
                ClassifierRule { kind: WorkKind::Planning, pattern: "raspored|dobiva svoj spec|tracnice|tračnice|mjera za c|adr-0|odluk|presud|revizija je dala|sto sada|što sada|zadatka za sljedecu|zadatak za sljedecu".into() },
                ClassifierRule { kind: WorkKind::Documentation, pattern: r"^docs|compact|revizija pred|^r\d/docs|ishod --|deploy-zapis|zapis uz".into() },
                ClassifierRule { kind: WorkKind::Debugging, pattern: r"^fix|bug-\d|^ci:|popravak ci|kvar|obara|^test:|lagati|laze|laže|rupe|bomba|vise ne postoji|više ne postoji".into() },
                ClassifierRule { kind: WorkKind::Polish, pattern: r"^c[4-7]|paleta|mrtv|selektorski|\bvan\b|audit|node 24|migrira|utility|ljestv|raspusten|raspušten|skuplj|refactor|^alat|ugasen|ugašen|dug placen|dug plaćen".into() },
            ],
            gate_pattern: r"check:|brana|gate|^alat|^test|sonda|probe|mjera".into(),
            deploy_pattern: "na produkciji|deploy".into(),
            ci_fix_pattern: r"^ci:|popravak ci|job je otkazan".into(),
            owner_name: "leon".into(),
            test_path_prefixes: vec!["tests/".into()],
            test_path_contains: vec!["/check-".into()],
            test_path_suffixes: vec![".test.js".into(), ".spec.js".into()],
            test_path_exclude: vec![],
            code_exclude_prefixes: vec!["docs/".into()],
            code_exclude_suffixes: vec![".md".into()],
            session_gap_hours: 2.0,
            session_start_hours: 0.5,
            closed_phases: vec![
                ClosedPhase { name: "Osobni UGC-graditelj F1–F5".into(), from: "2026-08-02".into(), to: "2026-08-06".into(), tag_pattern: r"^(F[1-5][: ]|Merge F5|docs\(F5\))".into(), note: "DB temelj → moji materijali → editor u čvoru → privatne slike → produkcija".into() },
                ClosedPhase { name: "Frontend redizajn C0–C7 + KOSTUR·TELEFON·POLICA·SEO".into(), from: "2026-08-07".into(), to: "2026-09-01".into(), tag_pattern: r"^(C\d[ab]?|T\d|K\d|N\d|S\d|ALAT-\d)".into(), note: "Tailwind CLI, tokeni, 3 teme, birač".into() },
                ClosedPhase { name: "MREŽA — sanacija A–E".into(), from: "2026-08-31".into(), to: "2026-09-01".into(), tag_pattern: r"^(faza MREZA|MREZA)".into(), note: "CSP enforce, RLS po retku, brane, CI shardanje".into() },
                ClosedPhase { name: "RAČUN R1 — OAuth + upitnik + dijalog".into(), from: "2026-09-02".into(), to: "2026-09-02".into(), tag_pattern: r"^R1".into(), note: "Google uživo potvrđen; R2/R3 → F2".into() },
            ],
            include_unmerged: false,
            unmerged_warn_days: 5,
            unmerged_alert_days: 10,
            unmerged_alert_count: 3,
            docs_lag_warn_days: 2,
            docs_lag_alert_days: 5,
            docs_weights: DocsWeights::default(),
        }
    }
}

impl Profile {
    /// Od kada io mora dovući git log: najranije od `since` i početaka zatvorenih faza
    /// (zatvorene faze se BROJE iz commita, a njihovi datumi prethode `since`).
    pub fn log_since(&self) -> String {
        self.closed_phases
            .iter()
            .map(|p| p.from.as_str())
            .chain(std::iter::once(self.since.as_str()))
            .min()
            .unwrap_or(self.since.as_str())
            .to_string()
    }

    /// Svi datumi iz profila moraju biti `YYYY-MM-DD`: `since` i `closed_phases[].from/to` su
    /// granice mjerenja, a tipfeler u njima tiho pomakne prozor (nalaz C2). Poruka nosi IME
    /// polja jer profil piše čovjek.
    pub fn validate_dates(&self) -> Result<(), ParseError> {
        let bad = |field: String, text: &str| ParseError::BadDate {
            field,
            text: text.to_string(),
        };
        if !crate::civil::is_ymd(&self.since) {
            return Err(bad("profil.since".into(), &self.since));
        }
        for (i, phase) in self.closed_phases.iter().enumerate() {
            if !crate::civil::is_ymd(&phase.from) {
                return Err(bad(format!("profil.closed_phases[{i}].from"), &phase.from));
            }
            if !crate::civil::is_ymd(&phase.to) {
                return Err(bad(format!("profil.closed_phases[{i}].to"), &phase.to));
            }
        }
        Ok(())
    }

    /// Sve putanje iz profila moraju ostati unutar korijena repoa (nalaz I9): alat koji mjeri
    /// repo se ne smije dati navesti da čita izvan njega (npr. `../../secrets`). Provjerava
    /// osam polja koja profil deklarira kao putanje; poruka imenuje TOČNO polje jer profil piše
    /// čovjek.
    pub fn validate_paths(&self) -> Result<(), ParseError> {
        let fields = [
            ("diary_path", &self.diary_path),
            ("changelog_path", &self.changelog_path),
            ("plan_path", &self.plan_path),
            ("docs_dir", &self.docs_dir),
            ("docs_index", &self.docs_index),
            ("plan_dir", &self.plan_dir),
            ("product_dir", &self.product_dir),
            ("key_file", &self.key_file),
        ];
        for (name, value) in fields {
            if !inside_root(value) {
                return Err(ParseError::PathOutsideRoot {
                    field: format!("profil.{name}"),
                    value: value.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn is_test_path(&self, path: &str) -> bool {
        if self
            .test_path_exclude
            .iter()
            .any(|x| path.contains(x.as_str()))
        {
            return false;
        }
        self.test_path_prefixes
            .iter()
            .any(|p| path.starts_with(p.as_str()))
            || self
                .test_path_contains
                .iter()
                .any(|c| path.contains(c.as_str()))
            || self
                .test_path_suffixes
                .iter()
                .any(|s| path.ends_with(s.as_str()))
    }

    pub fn is_code_path(&self, path: &str) -> bool {
        !(self
            .code_exclude_prefixes
            .iter()
            .any(|p| path.starts_with(p.as_str()))
            || self
                .code_exclude_suffixes
                .iter()
                .any(|s| path.ends_with(s.as_str())))
    }
}

/// Ostaje li relativna putanja unutar korijena? Odbija `..` komponentu, apsolutnu putanju, korijen
/// (`/x`, `\x`) i Windows prefiks (`C:`, `\\server`). Čisto parsiranje — `std::path` ne dira disk,
/// pa smije u jezgru (S-002).
pub fn inside_root(rel: &str) -> bool {
    let p = Path::new(rel);
    if p.is_absolute() {
        return false;
    }
    p.components()
        .all(|c| matches!(c, Component::Normal(_) | Component::CurDir))
}

pub struct Patterns {
    /// Poveznica na `.md` u markdownu: `](put/do.md#odjeljak)`. NIJE iz profila — konstanta je,
    /// ali joj je dom ovdje da `docs.rs` ne mora `expect()` u produkcijskom kodu: `Regex::new`
    /// se ovdje propagira `?`-om kao svaki drugi regex (nalaz M6).
    pub md_link: Regex,
    /// Datum oblika `20xx-xx-xx` u tekstu dokumenta; isto konstanta, isti razlog.
    pub iso_date: Regex,
    pub diary_heading: Regex,
    pub diary_deploy: Regex,
    pub plan_brick: Regex,
    pub plan_phase_name: Regex,
    pub phase_tag: Regex,
    pub classifier: Vec<(WorkKind, Regex)>,
    pub gate: Regex,
    pub deploy: Regex,
    pub ci_fix: Regex,
    pub paused: Regex,
    pub closed_phases: Vec<(ClosedPhase, Regex)>,
}

/// Kompilira regex iz profila i TRAŽI bar `need` capture-grupa, jer ih parser čita po broju.
/// `captures_len()` broji i grupu 0 (cijeli pogodak), pa se od nje odbija jedinica.
fn with_groups(field: &str, pattern: &str, need: usize) -> Result<Regex, ParseError> {
    let re = Regex::new(pattern)?;
    let got = re.captures_len() - 1;
    if got < need {
        return Err(ParseError::BadPattern {
            field: field.into(),
            need,
            got,
        });
    }
    Ok(re)
}

impl Patterns {
    pub fn compile(p: &Profile) -> Result<Patterns, ParseError> {
        Ok(Patterns {
            md_link: Regex::new(r"\]\(([^)\s]+\.md)(#[^)\s]*)?\)")?,
            iso_date: Regex::new(r"\b20\d\d-\d\d-\d\d\b")?,
            diary_heading: with_groups("diary_heading", &p.diary_heading, 3)?,
            diary_deploy: Regex::new(&p.diary_deploy_pattern)?,
            plan_brick: with_groups("plan_brick", &p.plan_brick, 3)?,
            plan_phase_name: with_groups("plan_phase_name", &p.plan_phase_name, 2)?,
            phase_tag: with_groups("phase_tag", &p.phase_tag, 1)?,
            classifier: p
                .classifier
                .iter()
                .map(|r| Regex::new(&r.pattern).map(|re| (r.kind, re)))
                .collect::<Result<_, _>>()?,
            gate: Regex::new(&p.gate_pattern)?,
            deploy: Regex::new(&p.deploy_pattern)?,
            ci_fix: Regex::new(&p.ci_fix_pattern)?,
            paused: Regex::new(&p.paused_marker)?,
            closed_phases: p
                .closed_phases
                .iter()
                .map(|c| Regex::new(&c.tag_pattern).map(|re| (c.clone(), re)))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_compiles_and_rejects_unknown_field() {
        assert!(Patterns::compile(&Profile::default()).is_ok());
        assert_eq!(Profile::default().log_since(), "2026-08-02");
        let partial: Profile = serde_json::from_str(r#"{"since":"2026-09-01"}"#).unwrap();
        assert_eq!(partial.since, "2026-09-01");
        assert_eq!(partial.default_branch, "main");
        assert!(serde_json::from_str::<Profile>(r#"{"sinc":"x"}"#).is_err());
    }

    /// C1: svako polje koje parser indeksira po grupi mora biti odbijeno ako grupe nema —
    /// i to s IMENOM polja, da korisnik zna što u `profile.json` popraviti.
    #[test]
    fn patterns_without_required_capture_groups_name_the_field() {
        let cases = [
            (
                "plan_brick",
                Profile {
                    plan_brick: r"^\| \*\*M".into(),
                    ..Profile::default()
                },
            ),
            (
                "plan_phase_name",
                Profile {
                    plan_phase_name: r"^### F\d".into(),
                    ..Profile::default()
                },
            ),
            (
                "phase_tag",
                Profile {
                    phase_tag: r"^F\d".into(),
                    ..Profile::default()
                },
            ),
        ];
        for (field, profile) in cases {
            let message = match Patterns::compile(&profile) {
                Ok(_) => String::new(),
                Err(e) => e.to_string(),
            };
            assert!(
                message.contains(field),
                "polje {field}: compile nije prijavio grešku ({message})"
            );
        }
    }

    #[test]
    fn test_and_code_paths() {
        let p = Profile::default();
        assert!(p.is_test_path("tests/unit/a.test.js"));
        assert!(p.is_test_path("scripts/check-docs.js"));
        assert!(!p.is_test_path("js/auth.js"));
        assert!(p.is_code_path("js/auth.js"));
        assert!(!p.is_code_path("docs/records/PROGRESS.md"));
        assert!(!p.is_code_path("CLAUDE.md"));
    }

    #[test]
    fn test_path_exclude_removes_fixtures_but_default_keeps_everything_under_tests() {
        let d = Profile::default();
        assert!(
            d.is_test_path("tests/fixtures/big.log"),
            "zadano = tablica: sve pod tests/ je test"
        );
        let p = Profile {
            test_path_contains: vec!["/tests/".into()],
            test_path_exclude: vec!["/fixtures/".into()],
            ..Profile::default()
        };
        assert!(p.is_test_path("crates/sokratis-core/tests/parity.rs"));
        assert!(!p.is_test_path("crates/sokratis-core/tests/fixtures/sokratstudy-2026-09-17.log"));
    }

    #[test]
    fn inside_root_rejects_parent_absolute_and_drive_paths() {
        for ok in [
            "docs",
            "docs/records/PROGRESS.md",
            "./docs",
            "CLAUDE.md",
            "a/./b",
        ] {
            assert!(inside_root(ok), "{ok}");
        }
        for bad in [
            "../..",
            "docs/../../x",
            "/etc/passwd",
            "\\\\server\\share",
            "C:\\Users",
            "C:/x",
            "..",
            "docs/..",
        ] {
            assert!(!inside_root(bad), "{bad}");
        }
    }

    #[test]
    fn branch_scope_defaults_to_all_and_parses_both_values() {
        assert_eq!(Profile::default().branch_scope, BranchScope::AllBranches);
        let p: Profile = serde_json::from_str(r#"{ "branch_scope": "default" }"#).unwrap();
        assert_eq!(p.branch_scope, BranchScope::DefaultBranch);
        let p: Profile = serde_json::from_str(r#"{ "branch_scope": "all" }"#).unwrap();
        assert_eq!(p.branch_scope, BranchScope::AllBranches);
    }

    #[test]
    fn branch_scope_rejects_unknown_value_naming_the_allowed_ones() {
        let err =
            serde_json::from_str::<Profile>(r#"{ "branch_scope": "worktrees" }"#).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("`all`") && msg.contains("`default`"), "{msg}");
    }

    #[test]
    fn validate_paths_names_the_offending_field() {
        let mut p = Profile::default();
        assert!(p.validate_paths().is_ok(), "zadani profil je unutar repoa");
        p.docs_dir = "../..".into();
        match p.validate_paths() {
            Err(ParseError::PathOutsideRoot { field, value }) => {
                assert_eq!(
                    (field.as_str(), value.as_str()),
                    ("profil.docs_dir", "../..")
                );
            }
            other => panic!("{other:?}"),
        }
        p.docs_dir = "docs".into();
        p.key_file = "C:\\CLAUDE.md".into();
        assert!(matches!(
            p.validate_paths(),
            Err(ParseError::PathOutsideRoot { field, .. }) if field == "profil.key_file"
        ));
    }
}
