//! ZAŠTO RUST OVAKO (cigla M1/13 — pravilo nespojenih grana)
//! `impl Rule for UnmergedBranches` je ugovor iz `rules/mod.rs`; bez stanja pa se konstruira golim
//! imenom. Prekršitelji se skupljaju jednim `filter`/`map` lancem nad granama, najstarija starost
//! `.max()` nad `i64`, a `Severity` bira jedan `if/else` nad booleovim uvjetom (starost ILI broj).
use super::Rule;
use crate::{Context, Severity, Signal};

pub struct UnmergedBranches;

impl Rule for UnmergedBranches {
    fn id(&self) -> &'static str {
        "unmerged-branches"
    }

    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let profile = &ctx.profile;
        let mut offenders: Vec<(&crate::BranchInfo, i64)> = ctx
            .branches
            .iter()
            .filter(|branch| !branch.merged && branch.name != profile.default_branch)
            .map(|branch| (branch, (ctx.now - branch.last_commit_time) / 86_400))
            .filter(|(_, age_days)| *age_days >= profile.unmerged_warn_days)
            .collect();
        if offenders.is_empty() {
            return vec![];
        }
        // Najstarija grana prva → `since` (najmanji `last_commit_time`) čita se s prvog mjesta.
        offenders.sort_by_key(|(branch, _)| branch.last_commit_time);
        let oldest_age = offenders
            .iter()
            .map(|(_, age_days)| *age_days)
            .max()
            .unwrap_or(0);
        let severity = if oldest_age >= profile.unmerged_alert_days
            || offenders.len() > profile.unmerged_alert_count
        {
            Severity::Alert
        } else {
            Severity::Warn
        };
        let evidence = offenders
            .iter()
            .map(|(branch, age_days)| {
                format!(
                    "{}: {age_days} dana, {} commita ispred {}",
                    branch.name, branch.ahead_of_default, profile.default_branch
                )
            })
            .collect();
        let since = offenders.first().map(|(branch, _)| branch.last_commit_time);
        vec![Signal {
            rule: self.id().into(),
            severity,
            title_key: "signal.unmerged_branches".into(),
            evidence,
            since,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BranchInfo, Profile, Severity};
    const DAY: i64 = 86_400;

    fn ctx(branches: Vec<BranchInfo>) -> Context {
        Context {
            profile: Profile::default(),
            now: 100 * DAY,
            commits: vec![],
            branches,
            docs: vec![],
            last_code_commit: None,
        }
    }
    fn b(name: &str, age_days: i64, ahead: u32, merged: bool) -> BranchInfo {
        BranchInfo {
            name: name.into(),
            last_commit_time: 100 * DAY - age_days * DAY,
            ahead_of_default: ahead,
            merged,
        }
    }

    #[test]
    fn young_or_merged_branches_are_silent() {
        assert!(
            UnmergedBranches
                .evaluate(&ctx(vec![
                    b("main", 0, 0, true),
                    b("feat/a", 2, 3, false),
                    b("old", 30, 1, true)
                ]))
                .is_empty()
        );
    }

    #[test]
    fn warn_then_alert_with_evidence() {
        let s = UnmergedBranches.evaluate(&ctx(vec![b("feat/a", 6, 3, false)]));
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].severity, Severity::Warn);
        assert_eq!(s[0].evidence, vec!["feat/a: 6 dana, 3 commita ispred main"]);
        assert_eq!(s[0].since, Some(94 * DAY));
        let s = UnmergedBranches.evaluate(&ctx(vec![b("feat/a", 11, 1, false)]));
        assert_eq!(s[0].severity, Severity::Alert);
        let many = UnmergedBranches.evaluate(&ctx(vec![
            b("a", 6, 1, false),
            b("b", 6, 1, false),
            b("c", 6, 1, false),
            b("d", 6, 1, false),
        ]));
        assert_eq!(
            (many[0].severity, many[0].evidence.len()),
            (Severity::Alert, 4)
        );
    }
}
