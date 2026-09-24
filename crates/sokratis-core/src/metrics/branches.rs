//! ZAŠTO RUST OVAKO (cigla M2/46 — grane, S-032)
//! `rows.iter().zip(commits)` spaja dva paralelna niza (isti redoslijed, ista dužina — `commit_rows`
//! to jamči) bez indeksa. `BTreeMap<&str, _>` s POSUĐENIM ključem drži grane sortirane po imenu bez
//! klona dok traje zbrajanje; tek `BranchStats` posjeduje `String`. Sati po grani su proxy: sati DANA
//! (`DayStats.hours`, iz sesija S-007) × udio commita grane tog dana — zbroj po granama je uvijek
//! jednak ukupnom, za razliku od sesija po grani (dvije grane istodobno = sati dvaput).
use crate::{BranchInfo, BranchStats, Commit, CommitRow, DayStats};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Default)]
struct Acc {
    commits: u32,
    lines: u64,
}

pub fn branch_stats(
    rows: &[CommitRow],
    commits: &[Commit],
    days: &[DayStats],
    branches: &[BranchInfo],
    default_branch: &str,
) -> Vec<BranchStats> {
    let mut acc: BTreeMap<&str, Acc> = BTreeMap::new();
    let mut per_day: HashMap<(&str, &str), u32> = HashMap::new();
    for (r, c) in rows.iter().zip(commits) {
        let a = acc.entry(r.branch.as_str()).or_default();
        a.commits += 1;
        a.lines += c.files.iter().map(|f| f.added + f.deleted).sum::<u64>();
        *per_day
            .entry((r.branch.as_str(), r.date.as_str()))
            .or_default() += 1;
    }
    let merged: HashSet<&str> = branches
        .iter()
        .filter(|b| b.merged)
        .map(|b| b.name.as_str())
        .collect();
    let mut out: Vec<BranchStats> = acc
        .into_iter()
        .map(|(name, a)| {
            let hours: f64 = days
                .iter()
                .map(|d| {
                    let on_branch = per_day.get(&(name, d.date.as_str())).copied().unwrap_or(0);
                    if d.commits == 0 {
                        0.0
                    } else {
                        d.hours * f64::from(on_branch) / f64::from(d.commits)
                    }
                })
                .sum();
            BranchStats {
                name: name.to_string(),
                commits: a.commits,
                lines: a.lines,
                hours: (hours * 10.0).round() / 10.0,
                merged: name == default_branch || merged.contains(name),
            }
        })
        .collect();
    // Po commitima silazno, pa po imenu (stabilno); zadana grana ide na vrh ako ima commita.
    out.sort_by(|a, b| b.commits.cmp(&a.commits).then_with(|| a.name.cmp(&b.name)));
    if let Some(pos) = out.iter().position(|b| b.name == default_branch)
        && pos != 0
    {
        let d = out.remove(pos);
        out.insert(0, d);
    }
    out
}
