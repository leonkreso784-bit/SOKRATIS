//! ZAŠTO RUST OVAKO (cigla M1/12 — čistoća dokumentacije)
//! Svaka provjera je mala privatna funkcija koja PUNI `Vec<Finding>` kroz `&mut` — jedan
//! vlasnik vektora (ova funkcija), više posudbi u nizu, nikad istodobno. `HashSet<&str>` nad
//! putanjama daje O(1) provjeru „postoji li cilj poveznice" bez kopiranja stringova.
use crate::{DocFile, DocsHealth, Finding, Patterns, Profile};
use regex::Regex;
use std::collections::HashSet;

fn finding(check: &str, path: &str, line: Option<usize>, message: String) -> Finding {
    Finding {
        check: check.into(),
        path: path.into(),
        line,
        message,
    }
}

/// `dir/../x.md` → normalizirano s `/`; vraća None ako izlazi iznad korijena.
fn resolve(from_file: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = from_file
        .rsplit_once('/')
        .map(|(dir, _)| dir)
        .unwrap_or("")
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    for seg in link.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

fn dead_links(files: &[DocFile], known: &HashSet<&str>, out: &mut Vec<Finding>) {
    let re = Regex::new(r"\]\(([^)\s]+\.md)(#[^)\s]*)?\)").expect("regex konstanta");
    for f in files {
        let mut in_fence = false; // poveznica unutar ``` bloka je primjer, ne poveznica
        for (i, line) in f.content.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if in_fence {
                continue;
            }
            for c in re.captures_iter(line) {
                let link = &c[1];
                if link.contains("://") {
                    continue;
                }
                let ok = resolve(&f.path, link).is_some_and(|t| known.contains(t.as_str()));
                if !ok {
                    out.push(finding(
                        "dead-link",
                        &f.path,
                        Some(i + 1),
                        format!("poveznica na {link} ne postoji"),
                    ));
                }
            }
        }
    }
}

fn not_indexed(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    let Some(index) = files.iter().find(|f| f.path == p.docs_index) else {
        return;
    };
    let prefix = format!("{}/", p.docs_dir);
    for f in files
        .iter()
        .filter(|f| f.path.starts_with(&prefix) && f.path != p.docs_index)
    {
        let rel = &f.path[prefix.len()..];
        if !index.content.contains(rel) {
            out.push(finding(
                "not-indexed",
                &f.path,
                None,
                format!("{rel} nije naveden u {}", p.docs_index),
            ));
        }
    }
}

fn active_plans(files: &[DocFile], p: &Profile, pat: &Patterns, out: &mut Vec<Finding>) {
    let prefix = format!("{}/", p.plan_dir);
    let specs: Vec<&DocFile> = files
        .iter()
        .filter(|f| {
            f.path.starts_with(&prefix)
                && !p
                    .plan_dir_ignore
                    .iter()
                    .any(|i| f.path.ends_with(i.as_str()))
        })
        .collect();
    if specs.is_empty() {
        return;
    }
    let active: Vec<&str> = specs
        .iter()
        .filter(|f| !pat.paused.is_match(&f.content))
        .map(|f| f.path.as_str())
        .collect();
    if active.len() > 1 {
        out.push(finding(
            "multiple-active-plans",
            &p.plan_dir,
            None,
            format!("više aktivnih planova: {}", active.join(", ")),
        ));
    } else if active.is_empty() {
        out.push(finding(
            "no-active-plan",
            &p.plan_dir,
            None,
            "svi planovi su PAUZIRANI; točno jedan mora nositi prvenstvo".into(),
        ));
    }
}

fn diary_in_definition(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    let re = Regex::new(r"\b20\d\d-\d\d-\d\d\b").expect("regex konstanta");
    let prefix = format!("{}/", p.product_dir);
    for f in files.iter().filter(|f| f.path.starts_with(&prefix)) {
        let n = re.find_iter(&f.content).count();
        if n > 3 {
            out.push(finding(
                "diary-in-definition",
                &f.path,
                None,
                format!("{n} datuma u definiciji; kronologija ide u records/"),
            ));
        }
    }
}

fn key_file_budget(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    if let Some(f) = files.iter().find(|f| f.path == p.key_file) {
        let bytes = f.content.replace('\r', "").len() as u64;
        if bytes > p.key_file_budget_bytes {
            out.push(finding(
                "key-file-budget",
                &f.path,
                None,
                format!("{bytes} B > budžet {} B", p.key_file_budget_bytes),
            ));
        }
    }
}

fn lag(
    files: &[DocFile],
    last_code: Option<i64>,
    p: &Profile,
    out: &mut Vec<Finding>,
) -> Option<i64> {
    let code = last_code?;
    let docs_time = files
        .iter()
        .filter(|f| f.path == p.diary_path || f.path == p.changelog_path)
        .filter_map(|f| f.last_change_time)
        .max()?;
    let days = (code - docs_time) / 86_400;
    if days <= 0 {
        return Some(0);
    }
    if days >= p.docs_lag_warn_days {
        out.push(finding(
            "docs-lag",
            &p.diary_path,
            None,
            format!("dnevnik i changelog kasne {days} dana za zadnjim commitom koda"),
        ));
    }
    Some(days)
}

pub fn docs_health(
    files: &[DocFile],
    last_code_commit_time: Option<i64>,
    profile: &Profile,
    p: &Patterns,
) -> Option<DocsHealth> {
    let prefix = format!("{}/", profile.docs_dir);
    if !files.iter().any(|f| f.path.starts_with(&prefix)) {
        return None;
    }
    let known: HashSet<&str> = files.iter().map(|f| f.path.as_str()).collect();
    let mut findings = Vec::new();
    dead_links(files, &known, &mut findings);
    not_indexed(files, profile, &mut findings);
    active_plans(files, profile, p, &mut findings);
    diary_in_definition(files, profile, &mut findings);
    key_file_budget(files, profile, &mut findings);
    let lag_days = lag(files, last_code_commit_time, profile, &mut findings);
    let w = &profile.docs_weights;
    let penalty: u32 = findings
        .iter()
        .map(|f| match f.check.as_str() {
            "dead-link" => w.dead_link,
            "not-indexed" => w.not_indexed,
            "multiple-active-plans" => w.multiple_plans,
            "no-active-plan" => w.no_active_plan,
            "diary-in-definition" => w.diary_in_definition,
            "docs-lag" => w.lag,
            "key-file-budget" => w.key_file_budget,
            _ => 0,
        } as u32)
        .sum();
    Some(DocsHealth {
        score: 100u32.saturating_sub(penalty) as u8,
        findings,
        lag_days,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Profile;

    fn f(path: &str, content: &str, t: Option<i64>) -> DocFile {
        DocFile {
            path: path.into(),
            content: content.into(),
            last_change_time: t,
        }
    }
    const DAY: i64 = 86_400;

    #[test]
    fn no_docs_dir_means_none_not_zero() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        assert!(docs_health(&[f("README.md", "# x", None)], None, &p, &pat).is_none());
    }

    #[test]
    fn findings_with_locations_and_score() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f(
                "docs/README.md",
                "[PRD](./product/PRD.md)\n[nema](./plan/NEMA.md)\nplan/A.md plan/B.md plan/ROADMAP.md records/PROGRESS.md records/CHANGELOG.md",
                None,
            ),
            f(
                "docs/product/PRD.md",
                "2026-01-01 2026-01-02 2026-01-03 2026-01-04",
                None,
            ),
            f("docs/plan/A.md", "**Status:** AKTIVAN", None),
            f("docs/plan/B.md", "**Status:** AKTIVAN", None),
            f("docs/plan/ROADMAP.md", "", None),
            f("docs/records/DUH.md", "", None),
            f("docs/records/PROGRESS.md", "", Some(10 * DAY)),
            f("docs/records/CHANGELOG.md", "", Some(9 * DAY)),
            f("CLAUDE.md", "x", None),
        ];
        let h = docs_health(&files, Some(13 * DAY), &p, &pat).unwrap();
        let checks: Vec<(&str, &str)> = h
            .findings
            .iter()
            .map(|x| (x.check.as_str(), x.path.as_str()))
            .collect();
        assert!(checks.contains(&("dead-link", "docs/README.md")));
        assert!(checks.contains(&("not-indexed", "docs/records/DUH.md")));
        assert!(checks.contains(&("multiple-active-plans", "docs/plan")));
        assert!(checks.contains(&("diary-in-definition", "docs/product/PRD.md")));
        assert!(checks.contains(&("docs-lag", "docs/records/PROGRESS.md")));
        assert!(!checks.iter().any(|(c, _)| *c == "key-file-budget"));
        let dead = h.findings.iter().find(|x| x.check == "dead-link").unwrap();
        assert_eq!(dead.line, Some(2));
        assert_eq!(h.lag_days, Some(3));
        assert_eq!(h.score, 100 - 5 - 3 - 15 - 5 - 10);
    }

    #[test]
    fn paused_plan_is_not_active_and_budget_is_checked() {
        let p = Profile {
            key_file_budget_bytes: 3,
            ..Profile::default()
        };
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "plan/A.md plan/B.md", None),
            f("docs/plan/A.md", "**Status:** ⏸️ PAUZIRAN", None),
            f("docs/plan/B.md", "aktivan", None),
            f("CLAUDE.md", "abcd\r\n", None),
        ];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        assert!(
            !h.findings
                .iter()
                .any(|x| x.check.ends_with("active-plan") || x.check.ends_with("active-plans"))
        );
        let b = h
            .findings
            .iter()
            .find(|x| x.check == "key-file-budget")
            .unwrap();
        assert!(b.message.contains("5 B"), "mjeri se bez CR: {}", b.message);
        assert_eq!(h.lag_days, None);
    }
}
