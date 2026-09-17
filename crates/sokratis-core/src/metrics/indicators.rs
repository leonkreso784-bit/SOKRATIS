//! ZAŠTO RUST OVAKO (cigla M1/11b — pokazatelji)
//! `IndicatorInput<'a>` je PRVI lifetime u jezgri: struktura koja samo POSUĐUJE pet slice-ova
//! dok traje jedan poziv — kopirati ih bilo bi rasipno, a vlasništvo nepotrebno. `'a` kaže
//! „svi žive bar koliko i ovaj ulaz". Mala zatvaranja `r1`/`r3` drže zaokruživanje na jednom mjestu.
use crate::metrics::effective_kind;
use crate::{
    Commit, DayStats, Delivery, Indicator, IndicatorKind, Patterns, Phase, PhaseState, Profile,
    WorkKind,
};
use std::collections::HashMap;

pub struct IndicatorInput<'a> {
    pub commits: &'a [Commit],
    pub deliveries: &'a [Delivery],
    pub days: &'a [DayStats],
    pub phases: &'a [Phase],
    pub overrides: &'a HashMap<String, WorkKind>,
}

pub fn indicators(input: &IndicatorInput<'_>, profile: &Profile, p: &Patterns) -> Vec<Indicator> {
    let r1 = |x: f64| (x * 10.0).round() / 10.0;
    let r3 = |x: f64| (x * 1000.0).round() / 1000.0;
    let per = |a: f64, b: f64| if b > 0.0 { a / b } else { 0.0 };
    let kind_count = |k: WorkKind| {
        input
            .commits
            .iter()
            .filter(|c| effective_kind(c, input.overrides, p) == k)
            .count() as f64
    };
    let closed: Vec<&Phase> = input
        .phases
        .iter()
        .filter(|ph| ph.state == PhaseState::Closed)
        .collect();
    let lines: u64 = input
        .commits
        .iter()
        .flat_map(|c| &c.files)
        .map(|f| f.added + f.deleted)
        .sum();
    let test_lines: u64 = input.days.iter().map(|d| d.test_lines).sum();

    // Svaka vrijednost niže odgovara TOČNO jednom retku brifove tablice 18 pokazatelja; `vec![...]`
    // ispod time postaje čitljiv kao ta tablica, ne kao ugniježđeni izraz po pokazatelju.
    let working_days = input.days.len() as f64;
    let commits = input.commits.len() as f64;
    let commits_per_day = r1(per(commits, working_days));
    let deliveries = input.deliveries.len() as f64;
    let deliveries_per_day = r1(per(deliveries, working_days));
    let hours = r1(input.days.iter().map(|d| d.hours).sum());
    let commits_per_hour = r1(per(commits, hours));
    let lines_changed = lines as f64;
    let test_lines_changed = test_lines as f64;
    let test_share = r3(test_lines_changed / lines_changed.max(1.0));
    let deploys = input.deliveries.iter().filter(|d| d.deploy).count() as f64;
    let debugging_commits = kind_count(WorkKind::Debugging);
    let debugging_share = r3(debugging_commits / commits.max(1.0));
    let docs_share = r3(kind_count(WorkKind::Documentation) / commits.max(1.0));
    let ci_fixes = input
        .commits
        .iter()
        .filter(|c| p.ci_fix.is_match(&c.subject.to_lowercase()))
        .count() as f64;
    let owner_driven = input
        .deliveries
        .iter()
        .filter(|d| d.title.to_lowercase().contains(&profile.owner_name))
        .count() as f64;
    let closed_in_range = closed
        .iter()
        .filter(|ph| {
            ph.to
                .as_deref()
                .is_some_and(|t| t >= profile.since.as_str())
        })
        .count() as f64;
    let avg_days = r1(per(
        closed.iter().filter_map(|ph| ph.days).sum::<i64>() as f64,
        closed.len() as f64,
    ));

    let m = IndicatorKind::Measure;
    let x = IndicatorKind::Proxy;
    let mk = |id: &str, value: f64, kind: IndicatorKind, formula: &str| Indicator {
        id: id.into(),
        value,
        kind,
        formula: formula.into(),
    };
    vec![
        mk(
            "working_days",
            working_days,
            m,
            "dana s bar jednim commitom",
        ),
        mk("commits", commits, m, "broj commita od since"),
        mk(
            "commits_per_day",
            commits_per_day,
            m,
            "commits / working_days",
        ),
        mk("deliveries", deliveries, m, "naslova u dnevniku od since"),
        mk(
            "deliveries_per_day",
            deliveries_per_day,
            m,
            "deliveries / working_days",
        ),
        mk(
            "hours",
            hours,
            x,
            "git-hours: razmak < gap_h + start_h po sesiji, sortirano po author_time",
        ),
        mk("commits_per_hour", commits_per_hour, x, "commits / hours"),
        mk("lines_changed", lines_changed, m, "Σ added + deleted"),
        mk(
            "test_lines",
            test_lines_changed,
            m,
            "Σ redaka na testnim putanjama",
        ),
        mk("test_share", test_share, m, "test_lines / lines_changed"),
        mk("deploys", deploys, m, "isporuke s 🚀/deploy"),
        mk(
            "debugging_commits",
            debugging_commits,
            m,
            "effective_kind == debugging",
        ),
        mk(
            "debugging_share",
            debugging_share,
            m,
            "debugging_commits / commits",
        ),
        mk("docs_share", docs_share, m, "documentation / commits"),
        mk("ci_fixes", ci_fixes, m, "subject ~ ci_fix_pattern"),
        mk(
            "owner_driven_deliveries",
            owner_driven,
            m,
            "naslov sadrži owner_name",
        ),
        mk(
            "closed_phases_in_range",
            closed_in_range,
            m,
            "zatvorene faze s to >= since",
        ),
        mk(
            "closed_phase_avg_days",
            avg_days,
            m,
            "mean(days) zatvorenih faza",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{day_stats, hours_per_day};
    use crate::{FileChange, PhaseState};

    fn c(sha: &str, t: i64, subject: &str, path: &str, lines: u64) -> Commit {
        Commit {
            sha: sha.into(),
            author_time: t,
            commit_time: t,
            date: "2026-09-01".into(),
            commit_date: "2026-09-01".into(),
            subject: subject.into(),
            files: vec![FileChange {
                path: path.into(),
                added: lines,
                deleted: 0,
            }],
        }
    }

    #[test]
    fn eighteen_indicators_with_python_rounding() {
        let profile = Profile::default();
        let p = Patterns::compile(&profile).unwrap();
        let commits = vec![
            c("a", 0, "fix: kvar", "js/a.js", 90),
            c("b", 3600, "docs: zapis", "docs/a.md", 5),
            c("c", 7200, "ci: popravak ci", "tests/a.test.js", 5),
        ];
        let deliveries = vec![Delivery {
            date: "2026-09-01".into(),
            model: "".into(),
            title: "Leonov nalaz 🚀".into(),
            kind: WorkKind::Execution,
            deploy: true,
        }];
        let hours = hours_per_day(&commits, 2.0, 0.5);
        let days = day_stats(&commits, &deliveries, &hours, &profile);
        let phases = vec![Phase {
            id: "x".into(),
            name: "x".into(),
            state: PhaseState::Closed,
            total_bricks: 1,
            done_bricks: 1,
            from: Some("2026-08-31".into()),
            to: Some("2026-09-01".into()),
            days: Some(2),
            commits: 1,
        }];
        let ov = HashMap::new();
        let ind = indicators(
            &IndicatorInput {
                commits: &commits,
                deliveries: &deliveries,
                days: &days,
                phases: &phases,
                overrides: &ov,
            },
            &profile,
            &p,
        );
        assert_eq!(ind.len(), 18);
        let v = |id: &str| {
            ind.iter()
                .find(|i| i.id == id)
                .unwrap_or_else(|| panic!("{id}"))
                .value
        };
        assert_eq!(
            (v("working_days"), v("commits"), v("commits_per_day")),
            (1.0, 3.0, 3.0)
        );
        assert_eq!((v("hours"), v("commits_per_hour")), (2.5, 1.2));
        assert_eq!(
            (v("lines_changed"), v("test_lines"), v("test_share")),
            (100.0, 5.0, 0.05)
        );
        assert_eq!(
            (
                v("deploys"),
                v("debugging_commits"),
                v("debugging_share"),
                v("docs_share")
            ),
            (1.0, 2.0, 0.667, 0.333)
        );
        assert_eq!((v("ci_fixes"), v("owner_driven_deliveries")), (1.0, 1.0));
        assert_eq!(
            (v("closed_phases_in_range"), v("closed_phase_avg_days")),
            (1.0, 2.0)
        );
        assert!(
            ind.iter()
                .filter(|i| i.kind == IndicatorKind::Proxy)
                .count()
                == 2
        );
    }

    /// Paritet sa Sokrat Studyjevim `rad-xlsx.py` nad stvarnim repozitorijem (191 dan povijesti,
    /// 190 commita od `since`). `hours`/`commits_per_hour` NISU u usporedbi: Python broji sate bez
    /// sortiranja po `author_time` pa zna izaći negativan (S-007 — vidi `hours_legacy` u
    /// `expected.json`, -139.2 h); Rust sortira pa je zbroj uvijek >= 0. To je ispravak kvara,
    /// ne odstupanje — pravilo je „ne prenosi se" (S-007), pa se ovdje samo tvrdi predznak.
    #[test]
    fn matches_sokrat_study_18_indicators() {
        use crate::metrics::{active_phases, closed_phases};
        use crate::parse::{parse_diary, parse_git_log, parse_plan};

        let profile = Profile::default();
        let p = Patterns::compile(&profile).unwrap();
        let since = profile.since.as_str();

        let log_text = include_str!("../../tests/fixtures/sokratstudy-2026-09-17.log");
        let diary_text = include_str!("../../tests/fixtures/sokratstudy-2026-09-17.PROGRESS.md");
        let plan_text = include_str!("../../tests/fixtures/sokratstudy-2026-09-17.RASPORED.md");
        let expected_json =
            include_str!("../../tests/fixtures/sokratstudy-2026-09-17.expected.json");
        let expected: serde_json::Value = serde_json::from_str(expected_json).unwrap();

        // Zatvorene faze se broje iz SVIH commita loga (log već počinje 2026-08-02, otkud kreće
        // prva povijesna faza) — `commits` niže je uži skup, od `since`, za sve ostale pokazatelje.
        let all_commits = parse_git_log(log_text).unwrap().commits;
        let commits: Vec<Commit> = all_commits
            .iter()
            .filter(|c| c.commit_date.as_str() >= since)
            .cloned()
            .collect();
        assert_eq!(
            commits.len(),
            190,
            "brif tvrdi 190 commita od since (commit_date)"
        );

        let deliveries = parse_diary(diary_text, &p, since);
        let hours = hours_per_day(
            &commits,
            profile.session_gap_hours,
            profile.session_start_hours,
        );
        let days = day_stats(&commits, &deliveries, &hours, &profile);

        let mut phases = closed_phases(&all_commits, &p);
        phases.extend(active_phases(
            parse_plan(plan_text, &p),
            &commits,
            "2026-09-17",
        ));

        let overrides: HashMap<String, WorkKind> = HashMap::new();
        let ind = indicators(
            &IndicatorInput {
                commits: &commits,
                deliveries: &deliveries,
                days: &days,
                phases: &phases,
                overrides: &overrides,
            },
            &profile,
            &p,
        );
        assert_eq!(ind.len(), 18);

        let want = &expected["indicators"];
        for i in &ind {
            if i.id == "hours" || i.id == "commits_per_hour" {
                continue;
            }
            let expected_v = want
                .get(&i.id)
                .and_then(|v| v.as_f64())
                .unwrap_or_else(|| panic!("expected.json nema indicators.{}", i.id));
            assert!(
                (i.value - expected_v).abs() < 1e-9,
                "{}: rust {} vs python {}",
                i.id,
                i.value,
                expected_v
            );
        }
        let hours_v = v_of(&ind, "hours");
        assert!(
            hours_v >= 0.0,
            "S-007 ispravljen: sati ne smiju biti negativni ({hours_v})"
        );
    }

    fn v_of(ind: &[Indicator], id: &str) -> f64 {
        ind.iter()
            .find(|i| i.id == id)
            .unwrap_or_else(|| panic!("{id}"))
            .value
    }
}
