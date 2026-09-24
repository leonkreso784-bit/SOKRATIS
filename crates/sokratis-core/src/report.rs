//! ZAŠTO RUST OVAKO (cigla M1/20 — sastavljanje izvještaja)
//! Ovo je jedino mjesto koje zna REDOSLIJED koraka; svaki korak je funkcija iz svog modula.
//! `Context` klonira `commits`/`branches`/`docs` u vlasništvo — jedna kopija po izvještaju, a
//! zauzvrat nijedan lifetime u potpisu pravila (S-002, RUST.md §1).
//!
//! ZAŠTO RUST OVAKO (cigla M2/3 — `until` gornja granica)
//! `Option<&str>::is_none_or` (stabilan u edition 2024) izražava „nema gornje granice ILI je
//! datum unutar nje" u jednom izrazu, bez ugnježđenog `match`-a u `filter`-u. Isti obrazac
//! provjere oblika kao `since` gore, ovdje kroz let-chain jer je `until` `Option`.
//!
//! ZAŠTO RUST OVAKO (cigla M2/4 — zbroj vizija po stanju)
//! Sastavljanje ostaje jedino mjesto koje zna redoslijed: `vision_totals` je čisto mjerenje
//! (`metrics::visions`) pozvano ovdje, isto kao `kind_stats` ili `day_stats` — sučelje samo
//! oblikuje ono što `Report` već nosi (S-012).
//!
//! ZAŠTO RUST OVAKO (cigla M2/5 — redci commita, klasifikacija jednom)
//! `commit_rows` klasificira svaki commit ovdje, JEDNOM; `kind_stats` posuđuje te redke prije nego
//! što se pomaknu (`move`) u `Report` na kraju — posudba završava prije premještanja, pa borrow
//! checker to dopušta bez klona (S-012: mjerenje u jezgri, ne u sučelju).
//!
//! Dopuna (cigla M2/46, S-032): `branch_stats` je pozvan nakon `rows`/`days`, kao svaki drugi korak
//! mjerenja — sastavljanje ostaje jedino mjesto koje zna redoslijed, `branch_stats` sam ne zna ni
//! za `commit_rows` ni za `day_stats`.
//!
//! Dopuna (cigla M2/47, S-033): `input.diary.map(parse_diary)` (jedan `Option`) postaje
//! `parse_diaries(&input.diaries, …)` (poziv koji sam zna raditi s praznim popisom) — jedan poziv
//! manje grana nego prije, jer unija PRIHVAĆA nula tekstova bez posebnog slučaja.
use crate::docs::docs_health;
use crate::metrics::indicators::IndicatorInput;
use crate::metrics::{
    active_phases, branch_stats, closed_phases, commit_rows, day_stats, hours_per_day, indicators,
    kind_stats, vision_totals,
};
use crate::parse::{parse_diaries, parse_git_log, parse_plan};
use crate::rules::{default_rules, evaluate_all};
use crate::{Commit, Context, ParseError, Patterns, Profile, Report, ReportInput, Touched};

/// Zadnji commit (po `author_time`) koji dira bar jednu putanju koju profil smatra kodom —
/// ulaz za docs-lag pravilo (dokumentacija smije kasniti za dnevnikom, ne za kodom).
fn last_code_commit(commits: &[Commit], profile: &Profile) -> Option<Commit> {
    commits
        .iter()
        .filter(|c| c.files.iter().any(|f| profile.is_code_path(&f.path)))
        .max_by_key(|c| c.author_time)
        .cloned()
}

pub fn build_report(input: &ReportInput, profile: &Profile) -> Result<Report, ParseError> {
    // C2: `since` je granica mjerenja i usporedba je leksikografska, pa oblik MORA biti
    // `YYYY-MM-DD` — inače tipfeler tiho promijeni prozor umjesto da se javi. Provjera je u
    // jezgri (ne u `clap`-u) da isti ugovor vrijedi i za Tauri put u M2.
    if !crate::civil::is_ymd(&input.since) {
        return Err(ParseError::BadDate {
            field: "since".into(),
            text: input.since.clone(),
        });
    }
    // M2/3: `until` je gornja granica — isti ugovor kao `since` (oblik provjeren u jezgri, ne
    // u `clap`-u/Tauriju), da isto pravilo vrijedi za CLI `--until` i birač raspona u sučelju.
    if let Some(u) = &input.until
        && !crate::civil::is_ymd(u)
    {
        return Err(ParseError::BadDate {
            field: "until".into(),
            text: u.clone(),
        });
    }
    profile.validate_dates()?;
    profile.validate_paths()?;
    let p = Patterns::compile(profile)?;
    let parsed = parse_git_log(&input.git_log)?;
    let all = parsed.commits;
    // S-011: filtar `since`/`until` u jezgri je po `commit_date` (kao git `--since`/`--until`),
    // leksikografski usporediv jer je oblika `YYYY-MM-DD`. `until` je uključiv (cijeli dan ulazi).
    let commits: Vec<Commit> = all
        .iter()
        .filter(|c| c.commit_date.as_str() >= input.since.as_str())
        .filter(|c| {
            input
                .until
                .as_deref()
                .is_none_or(|u| c.commit_date.as_str() <= u)
        })
        .cloned()
        .collect();
    let deliveries: Vec<_> = parse_diaries(&input.diaries, &p, &input.since)
        .into_iter()
        .filter(|d| input.until.as_deref().is_none_or(|u| d.date.as_str() <= u))
        .collect();
    let plan_phases = input
        .plan
        .as_deref()
        .map(|t| parse_plan(t, &p))
        .unwrap_or_default();
    let hours = hours_per_day(
        &commits,
        profile.session_gap_hours,
        profile.session_start_hours,
    );
    let days = day_stats(&commits, &deliveries, &hours, profile);
    let rows = commit_rows(
        &commits,
        &input.overrides,
        &p,
        &input.commit_branches,
        &input.branch,
    );
    let kinds = kind_stats(&rows, &commits);
    let branches = branch_stats(&rows, &commits, &days, &input.branches, &input.branch);
    // Zatvorene faze se broje iz SVIH commita loga (mogu prethoditi `since`); aktivne samo iz
    // filtriranih, jer prate napredak od danas unatrag.
    let mut phases = closed_phases(&all, &p);
    phases.extend(active_phases(plan_phases, &commits, &input.today, &p));
    let indicators = indicators(
        &IndicatorInput {
            commits: &commits,
            deliveries: &deliveries,
            days: &days,
            phases: &phases,
            overrides: &input.overrides,
            since: &input.since,
        },
        profile,
        &p,
    );
    let last_code = last_code_commit(&commits, profile);
    let docs = docs_health(
        &input.docs,
        last_code.as_ref().map(|c| c.author_time),
        profile,
        &p,
    );
    let ctx = Context {
        profile: profile.clone(),
        now: input.now,
        commits: commits.clone(),
        branches: input.branches.clone(),
        docs: input.docs.clone(),
        last_code_commit: last_code,
    };
    let signals = evaluate_all(&default_rules(), &ctx);
    Ok(Report {
        generated_at: input.now,
        since: input.since.clone(),
        until: input.until.clone(),
        branch: input.branch.clone(),
        scope: input.scope,
        touched: Touched {
            commits: commits.len(),
            lines: commits
                .iter()
                .flat_map(|c| &c.files)
                .map(|f| f.added + f.deleted)
                .sum(),
            files: commits.iter().map(|c| c.files.len()).sum(),
            skipped_lines: parsed.skipped_lines,
            worktrees: input.worktrees,
            diaries: input.diaries.len() as u32,
        },
        days,
        kinds,
        branches,
        commits: rows,
        deliveries,
        indicators,
        phases,
        vision_totals: vision_totals(&input.visions),
        visions: input.visions.clone(),
        docs,
        signals,
    })
}

#[cfg(test)]
// M2/7: `pub(crate)` na modulu (ne samo funkciji) — privatan `mod` je vidljiv samo iz svog
// roditelja i potomaka, a `snapshot::tests` (brat, ne potomak) treba `report::tests::input()`.
pub(crate) mod tests {
    use super::*;
    use crate::{BranchInfo, BranchScope, DocFile, WorkKind};
    use std::collections::HashMap;

    const LOG: &str = "@@a1|1788700000|1788700000|2026-08-28|2026-08-28|F1/1 prije since\n1\t0\tjs/a.js\n\n@@b2|1788854400|1788854400|2026-09-04|2026-09-04|fix: kvar u js\n5\t1\tjs/b.js\n\n@@c3|1788858000|1788858000|2026-09-04|2026-09-04|docs: zapis\n3\t0\tdocs/records/PROGRESS.md\n";

    // M2/7: `pub(crate)` da `snapshot::tests` posudi isti fixture umjesto da ga duplicira —
    // brif (task-7) to izričito dopušta jer je JEZGRA vlasnik i ove i te datoteke.
    pub(crate) fn input() -> ReportInput {
        ReportInput {
            git_log: LOG.into(),
            diaries: vec!["## 2026-09-04 (X) — 🚀 deploy nečega\n".into()],
            plan: Some("| **F1/1** ✅ |\n| **F1/2** |\n".into()),
            docs: vec![DocFile {
                path: "docs/records/PROGRESS.md".into(),
                content: String::new(),
                last_change_time: Some(1788858000),
            }],
            branches: vec![BranchInfo {
                name: "feat/stara".into(),
                last_commit_time: 1788854400 - 20 * 86_400,
                ahead_of_default: 4,
                merged: false,
            }],
            overrides: HashMap::from([("b2".to_string(), WorkKind::Polish)]),
            visions: vec![],
            now: 1788854400 + 86_400,
            today: "2026-09-05".into(),
            since: "2026-08-29".into(),
            until: None,
            branch: "main".into(),
            scope: BranchScope::DefaultBranch,
            commit_branches: HashMap::new(),
            worktrees: 1,
        }
    }

    #[test]
    fn assembles_everything_and_filters_by_since() {
        let r = build_report(&input(), &Profile::default()).unwrap();
        assert_eq!(
            (
                r.touched.commits,
                r.touched.lines,
                r.touched.files,
                r.touched.skipped_lines
            ),
            (2, 9, 2, 0)
        );
        assert_eq!(r.days.len(), 1);
        assert_eq!((r.days[0].deliveries, r.days[0].deploys), (1, 1));
        let polish = r.kinds.iter().find(|k| k.kind == WorkKind::Polish).unwrap();
        assert_eq!(polish.commits, 1, "override b2 → polish");
        assert_eq!(r.indicators.len(), 18);
        let f1 = r.phases.iter().find(|p| p.id == "F1").unwrap();
        assert_eq!(
            (f1.total_bricks, f1.done_bricks, f1.commits),
            (2, 1, 0),
            "F1/1 je prije since pa se ne broji"
        );
        // M2/36: nijedan od dva commita ovog fixturea ne pogađa raspon/uzorak ijedne od četiriju
        // zadanih zatvorenih faza (Sokrat Studyjeva povijest, S-005) — `closed_phases` ih više ne
        // vraća bez pogotka (spec §13.7: profil ne smije tvrditi tuđu povijest kao ovog repoa).
        assert_eq!(
            r.phases
                .iter()
                .filter(|p| p.state == crate::PhaseState::Closed)
                .count(),
            0
        );
        assert!(r.docs.is_some());
        let rules: Vec<&str> = r.signals.iter().map(|s| s.rule.as_str()).collect();
        assert!(rules.contains(&"unmerged-branches"), "{rules:?}");
        assert!(!rules.contains(&"docs-lag"), "dnevnik je svježiji od koda");
        assert_eq!((r.generated_at, r.branch.as_str()), (input().now, "main"));
    }

    /// I8 (završna recenzija M1) + M2/36 (spec §13.7): zatvorena faza bez ijednog pogođenog
    /// commita je TUĐA povijest, ne naša. Nad praznim repoom sa ZADANIM profilom (S-005) je pisalo
    /// „zatvorenih faza u razdoblju 3" i „prosječno trajanje 8.5" — dva od 18 pokazatelja izmišljena
    /// za svaki projekt osim Sokrat Studyja; I8 je to popravio SAMO za pokazatelje. `closed_phases`
    /// (M2/36) ide korak dalje: takva faza više se i ne ispisuje u `r.phases` (bila je 0/0 do ove
    /// cigle) — provjera ispod ostaje vrijediti i nad praznim popisom (`all()` na `[]` je `true`).
    #[test]
    fn closed_phase_without_commits_is_not_counted() {
        let mut empty = input();
        empty.git_log = String::new();
        empty.plan = None;
        empty.diaries = vec![];
        let r = build_report(&empty, &Profile::default()).expect("prazan log je valjan ulaz");
        let value = |id: &str| {
            r.indicators
                .iter()
                .find(|i| i.id == id)
                .unwrap_or_else(|| panic!("{id}"))
                .value
        };
        assert_eq!(value("closed_phases_in_range"), 0.0);
        assert_eq!(value("closed_phase_avg_days"), 0.0);
        assert!(
            r.phases.iter().all(|p| p.commits == 0),
            "prazan log ne može pogoditi ni jednu fazu"
        );
    }

    /// C3 (završna recenzija M1): pokazatelj `closed_phases_in_range` je uspoređivao s
    /// `profile.since`, dok cijeli ostatak izvještaja filtrira po `input.since` — pa je jedan od
    /// 18 pokazatelja bio kriv (u JSON-u i u tablici) kad god se `--since` razlikuje od profila.
    #[test]
    fn closed_phases_in_range_follows_input_since_not_profile_since() {
        // Dva commita koja upadaju u DVIJE povijesne zatvorene faze zadanog profila:
        // „MREŽA" (2026-08-31..2026-09-01) i „RAČUN R1" (2026-09-02..2026-09-02).
        const LOG: &str = "@@m1|1787000000|1787000000|2026-08-31|2026-08-31|MREZA A1: baza\n1\t0\tjs/a.js\n\n@@r1|1788000000|1788000000|2026-09-02|2026-09-02|R1: Google prijava\n1\t0\tjs/b.js\n";
        let closed_in_range = |since: &str| {
            let mut i = input();
            i.git_log = LOG.into();
            i.since = since.into();
            let r = build_report(&i, &Profile::default()).expect("valjan ulaz");
            r.indicators
                .iter()
                .find(|x| x.id == "closed_phases_in_range")
                .expect("pokazatelj postoji")
                .value
        };
        assert_eq!(
            closed_in_range("2026-08-29"),
            2.0,
            "MREŽA (to 2026-09-01) i R1 (to 2026-09-02); faza bez commita ne ulazi (I8)"
        );
        assert_eq!(
            closed_in_range("2026-09-02"),
            1.0,
            "samo R1 završava 2026-09-02 ili poslije"
        );
    }

    /// C2 (završna recenzija M1): neprovjeren `since` je tiho mijenjao prozor mjerenja —
    /// `2026-9-17` je leksikografski VEĆI od `2026-09-17` pa je davao 0 commita i izlaz 0, a
    /// `17.09.2026` je manji od svakog `2026-…` pa je propuštao sve. Tipfeler mora biti greška.
    #[test]
    fn malformed_since_is_an_error_not_a_silent_window() {
        for bad in ["2026-9-17", "17.09.2026", "banana", "2026-02-30", ""] {
            let mut bad_input = input();
            bad_input.since = bad.into();
            match build_report(&bad_input, &Profile::default()) {
                Ok(r) => panic!("since `{bad}` je prošao: {} commita", r.touched.commits),
                Err(e) => {
                    let text = e.to_string();
                    assert!(text.contains("since"), "greška ne imenuje polje: {text}");
                }
            }
        }
        assert!(
            build_report(&input(), &Profile::default()).is_ok(),
            "ispravan `YYYY-MM-DD` mora proći"
        );
    }

    /// Isti tipfeler u `profile.json` (`since`, `closed_phases[].from/to`) mora reći KOJE polje
    /// je krivo — profil piše čovjek, pa poruka mora pokazati na redak koji se popravlja.
    #[test]
    fn malformed_profile_dates_name_the_field() {
        let bad_since = Profile {
            since: "17.09.2026".into(),
            ..Profile::default()
        };
        let e = build_report(&input(), &bad_since)
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default();
        assert!(e.contains("profil.since"), "{e}");

        let mut bad_phase = Profile::default();
        bad_phase.closed_phases[1].to = "2026-13-01".into();
        let e = build_report(&input(), &bad_phase)
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default();
        assert!(e.contains("closed_phases[1].to"), "{e}");
    }

    /// M2/3: `until` je zrcalo `since`-a — gornja granica, cijeli dan ulazi (S-011). Lokalni
    /// `LOG` s četvrtim commitom POSLIJE granice ne dira dijeljeni `LOG` iznad (taj ostaje
    /// netaknut za `assembles_everything_and_filters_by_since` i ostale testove).
    #[test]
    fn until_cuts_commits_and_deliveries_after_that_day_inclusive() {
        const LOG_WITH_LATER_COMMIT: &str = "@@a1|1788700000|1788700000|2026-08-28|2026-08-28|F1/1 prije since\n1\t0\tjs/a.js\n\n@@b2|1788854400|1788854400|2026-09-04|2026-09-04|fix: kvar u js\n5\t1\tjs/b.js\n\n@@c3|1788858000|1788858000|2026-09-04|2026-09-04|docs: zapis\n3\t0\tdocs/records/PROGRESS.md\n\n@@d4|1789000000|1789000000|2026-09-06|2026-09-06|F1/2 poslije\n1\t0\tjs/d.js\n";
        let mut i = input();
        i.git_log = LOG_WITH_LATER_COMMIT.into();
        i.diaries = vec!["## 2026-09-04 (X) — unutar\n## 2026-09-05 (X) — poslije\n".into()];
        i.until = Some("2026-09-04".into());
        let r = build_report(&i, &Profile::default()).unwrap();
        assert_eq!(r.until.as_deref(), Some("2026-09-04"));
        assert!(r.days.iter().all(|d| d.date.as_str() <= "2026-09-04"));
        assert_eq!(
            r.days.iter().map(|d| d.deliveries).sum::<u32>(),
            1,
            "isporuka 05. ne ulazi"
        );
        assert_eq!(
            r.touched.commits, 2,
            "d4 (2026-09-06) je poslije until, ne ulazi"
        );
    }

    #[test]
    fn malformed_until_is_a_named_error() {
        let mut i = input();
        i.until = Some("4.9.2026".into());
        match build_report(&i, &Profile::default()) {
            Err(ParseError::BadDate { field, text }) => {
                assert_eq!((field.as_str(), text.as_str()), ("until", "4.9.2026"));
            }
            other => panic!("očekivan BadDate, dobiveno {other:?}"),
        }
    }

    #[test]
    fn all_five_work_kinds_have_stable_ids() {
        // Recenzent T1: potvrda da `WorkKind::id()` (S-008) pokriva SVE varijante, ne samo
        // one koje se pojave u fixtureu iznad — inače bi nova vrsta rada mogla proći bez ida.
        assert_eq!(WorkKind::Planning.id(), "planning");
        assert_eq!(WorkKind::Documentation.id(), "documentation");
        assert_eq!(WorkKind::Execution.id(), "execution");
        assert_eq!(WorkKind::Polish.id(), "polish");
        assert_eq!(WorkKind::Debugging.id(), "debugging");
    }
}
