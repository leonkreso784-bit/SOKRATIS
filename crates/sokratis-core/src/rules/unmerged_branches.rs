//! ZAŠTO RUST OVAKO (cigla M1/13 — pravilo nespojenih grana)
//! `impl Rule for UnmergedBranches` je ugovor iz `rules/mod.rs`; bez stanja pa se konstruira golim
//! imenom. Prekršitelji se skupljaju jednim `filter`/`map` lancem nad granama, najstarija starost
//! `.max()` nad `i64`, a `Severity` bira jedan `if/else` nad booleovim uvjetom (starost ILI broj).
//!
//! Dopunjeno M2/64 (S-038): prekršitelji su sad SAMO vrhovi lanca (`contained_in.is_none()`) —
//! `ctx.branches` dolazi iz `report::build_report` VEĆ sa sadržanošću upisanom (`chains::
//! assign_containment`), pa ovo pravilo samo FILTRIRA po tom polju, ne računa graf. Sadržane grane
//! se ne broje ni u prag ni u prosjek starosti — vrh nabraja SVOJE sadržane u dokazu (`contained_of`).
use super::Rule;
use crate::{Context, Severity, Signal};

/// Hrvatski paucal za broj SADRŽANIH grana u dokazu vrha (dopuna R62): 1 → „grana", 2–4 (osim
/// 12–14) → „grane", inače → „grana" — ista aritmetika kao `branch_word` u `cli/src/table.rs`
/// (T51), NAMJERNO samostalna kopija jer jezgra ne smije ovisiti o CLI-ju (S-002).
fn contained_word(n: usize) -> &'static str {
    let last_one = n % 10;
    let last_two = n % 100;
    if (2..=4).contains(&last_one) && !(12..=14).contains(&last_two) {
        "grane"
    } else {
        "grana"
    }
}

/// Imena grana čiji je `contained_in == Some(top_name)`, abecedno — dio dokaza vrha (S-038).
fn contained_of<'a>(branches: &'a [crate::BranchInfo], top_name: &str) -> Vec<&'a str> {
    let mut names: Vec<&str> = branches
        .iter()
        .filter(|b| b.contained_in.as_deref() == Some(top_name))
        .map(|b| b.name.as_str())
        .collect();
    names.sort_unstable();
    names
}

pub struct UnmergedBranches;

impl Rule for UnmergedBranches {
    fn id(&self) -> &'static str {
        "unmerged-branches"
    }

    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let profile = &ctx.profile;
        // S-038: prekršitelj je SAMO vrh lanca (`contained_in.is_none()`) — grana koju nitko ne
        // sadrži. Grana s `contained_in: Some(_)` je duboko u tuđem lancu, radu na njoj svjedoči
        // njezin vrh (dokaz niže), pa se ovdje ne broji ni u prag ni u starost. Vrh mlađi od praga
        // time utišava CIJELI lanac (rad je živ) — automatski, jer se samo vrh provjerava.
        let mut offenders: Vec<(&crate::BranchInfo, i64)> = ctx
            .branches
            .iter()
            .filter(|branch| {
                !branch.merged
                    && branch.name != profile.default_branch
                    && branch.contained_in.is_none()
            })
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
            .map(|(top, age_days)| {
                let base = format!(
                    "{}: {age_days} dana, {} commita ispred {}",
                    top.name, top.ahead_of_default, profile.default_branch
                );
                let contained = contained_of(&ctx.branches, &top.name);
                if contained.is_empty() {
                    base
                } else {
                    format!(
                        "{base} (+{} {} unutar: {})",
                        contained.len(),
                        contained_word(contained.len()),
                        contained.join(", ")
                    )
                }
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
        ctx_with(Profile::default(), branches)
    }

    /// Kao `ctx`, ali s ODABRANIM profilom — Step 1 (S-038) treba `unmerged_alert_days` veći od
    /// starosti fixturea (16 dana), da test tvrdi ono što hoće (dva vrha ≤ prag broja → Warn), a ne
    /// nešto slučajno zbog zadanog `unmerged_alert_days = 10` (dopuna, ispravak 1).
    fn ctx_with(profile: Profile, branches: Vec<BranchInfo>) -> Context {
        Context {
            profile,
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
            tip: name.into(),
            contained_in: None,
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

    /// S-038: lanac a ⊂ b ⊂ c ⊂ d (svaka sesija od prethodne) + odvojena e → DVA vrha, Warn (ne
    /// Alert iako je pet grana starih), dokaz vrha `d` nabraja sadržane. Danas: pet prekršitelja →
    /// lažni Alert (rulno je popravljeno tek u Step 3).
    #[test]
    fn chain_counts_only_its_top_and_lists_contained_branches_as_evidence() {
        let mut a = b("a", 10, 3, false);
        a.contained_in = Some("d".into());
        let mut bb = b("b", 10, 12, false);
        bb.contained_in = Some("d".into());
        let mut c = b("c", 9, 21, false);
        c.contained_in = Some("d".into());
        let d = b("d", 7, 50, false);
        let e = b("e", 16, 34, false);
        // Dopuna, ispravak 1: `unmerged_alert_days` veći od 16 (starost `e`), inače bi test padao iz
        // KRIVOG razloga (Alert po starosti, ne po broju vrhova).
        let profile = Profile {
            unmerged_alert_days: 30,
            ..Profile::default()
        };
        let s = UnmergedBranches.evaluate(&ctx_with(profile, vec![a, bb, c, d, e]));
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].severity, Severity::Warn, "dva vrha ≤ prag 3");
        assert_eq!(s[0].evidence.len(), 2, "samo vrhovi: {:?}", s[0].evidence);
        assert!(
            s[0].evidence
                .iter()
                .any(|l| l.starts_with("d: ") && l.contains("(+3 grane unutar: a, b, c)")),
            "{:?}",
            s[0].evidence
        );
        assert!(
            s[0].evidence
                .iter()
                .any(|l| l.starts_with("e: ") && !l.contains("unutar")),
            "{:?}",
            s[0].evidence
        );
    }

    /// Dopuna R62: hrvatski paucal broja sadržanih grana u dokazu — isti obrazac kao `branch_word`
    /// u `cli/src/table.rs` (T51), samostalna kopija jer jezgra ne smije ovisiti o CLI-ju (S-002 duh:
    /// slojevi se ne miješaju čak ni kad je pravilo trivijalno).
    #[test]
    fn contained_word_follows_croatian_paucal() {
        assert_eq!(contained_word(1), "grana");
        assert_eq!(contained_word(2), "grane");
        assert_eq!(contained_word(4), "grane");
        assert_eq!(contained_word(5), "grana");
        assert_eq!(contained_word(12), "grana");
        assert_eq!(contained_word(21), "grana");
        assert_eq!(contained_word(22), "grane");
    }
}
