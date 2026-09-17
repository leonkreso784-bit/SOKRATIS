//! ZAŠTO RUST OVAKO (cigla M1/9 — tempo po danu)
//! `BTreeMap<&str, Acc>` s privatnom `#[derive(Default)]` strukturom akumulatora: ključ je POSUĐEN
//! (`c.date.as_str()`), ne kloniran u `String` — akumulacija traje samo dok `commits` postoji
//! posuđen, pa nema razloga plaćati kopiju datuma za svaki dan. `entry().or_default()` stvara
//! prazan akumulator kad dan prvi put naiđe; tek na izlazu (`date.to_string()`) datum se klonira,
//! jer `DayStats` mora posjedovati svoje podatke. Zatim jedan prolaz uzlazno gradi kumulativ.
use crate::{Commit, DayStats, Delivery, Profile};
use std::collections::BTreeMap;

#[derive(Default)]
struct Acc {
    commits: u32,
    lines: u64,
    test_lines: u64,
}

/// Grupira commite po danu autora (paritet s `RAD.xlsx`): redak po danu koji ima barem jedan
/// commit, uzlazno po datumu (S-007 — `BTreeMap` sortira ključeve umjesto naknadnog sortiranja).
/// `lines` broji added+deleted po danu; `test_lines` isto, ali samo za datoteke koje profil
/// prepoznaje kao testne (`Profile::is_test_path`). `commits_cumulative` raste kroz dane u
/// kronološkom redu. `hours` dolazi iz mape T8 — 0.0 kad dana u njoj nema (npr. commit bez
/// izračunatog razmaka). `deliveries`/`deploys` broje isporuke po `Delivery.date`.
pub fn day_stats(
    commits: &[Commit],
    deliveries: &[Delivery],
    hours: &BTreeMap<String, f64>,
    p: &Profile,
) -> Vec<DayStats> {
    let mut acc: BTreeMap<&str, Acc> = BTreeMap::new();
    for c in commits {
        let a = acc.entry(c.date.as_str()).or_default();
        a.commits += 1;
        for f in &c.files {
            let n = f.added + f.deleted;
            a.lines += n;
            if p.is_test_path(&f.path) {
                a.test_lines += n;
            }
        }
    }
    let mut cumulative = 0u32;
    acc.into_iter()
        .map(|(date, a)| {
            cumulative += a.commits;
            DayStats {
                date: date.to_string(),
                commits: a.commits,
                commits_cumulative: cumulative,
                lines: a.lines,
                hours: hours.get(date).copied().unwrap_or(0.0),
                deliveries: deliveries.iter().filter(|d| d.date == date).count() as u32,
                deploys: deliveries
                    .iter()
                    .filter(|d| d.date == date && d.deploy)
                    .count() as u32,
                test_lines: a.test_lines,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Delivery, FileChange, Profile, WorkKind};

    fn c(date: &str, files: Vec<(&str, u64, u64)>) -> Commit {
        Commit {
            sha: date.into(),
            author_time: 0,
            commit_time: 0,
            date: date.into(),
            commit_date: date.into(),
            subject: String::new(),
            files: files
                .into_iter()
                .map(|(p, a, d)| FileChange {
                    path: p.into(),
                    added: a,
                    deleted: d,
                })
                .collect(),
        }
    }
    fn d(date: &str, deploy: bool) -> Delivery {
        Delivery {
            date: date.into(),
            model: String::new(),
            title: String::new(),
            kind: WorkKind::Execution,
            deploy,
        }
    }

    #[test]
    fn groups_by_day_with_cumulative_tests_and_deliveries() {
        let commits = vec![
            c(
                "2026-09-02",
                vec![("js/a.js", 10, 2), ("tests/a.test.js", 5, 0)],
            ),
            c("2026-09-01", vec![("scripts/check-x.js", 7, 7)]),
            c("2026-09-02", vec![("docs/x.md", 1, 1)]),
        ];
        let deliveries = vec![
            d("2026-09-02", true),
            d("2026-09-02", false),
            d("2026-09-05", false),
        ];
        let mut hours = BTreeMap::new();
        hours.insert("2026-09-02".to_string(), 1.5);
        let days = day_stats(&commits, &deliveries, &hours, &Profile::default());
        assert_eq!(days.len(), 2, "05.09. nema commit → nema retka");
        assert_eq!(
            (
                days[0].date.as_str(),
                days[0].commits,
                days[0].commits_cumulative,
                days[0].lines,
                days[0].test_lines
            ),
            ("2026-09-01", 1, 1, 14, 14)
        );
        assert_eq!(
            (
                days[1].commits,
                days[1].commits_cumulative,
                days[1].lines,
                days[1].test_lines
            ),
            (2, 3, 19, 5)
        );
        assert_eq!(
            (days[1].deliveries, days[1].deploys, days[1].hours),
            (2, 1, 1.5)
        );
        assert_eq!(days[0].hours, 0.0);
    }
}
