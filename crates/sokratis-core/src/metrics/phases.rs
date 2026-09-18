//! ZAŠTO RUST OVAKO (cigla M1/11a — faze)
//! Zatvorene faze se BROJE iz commita (mjera), ne prepisuju (procjena). `plan_phases: Vec<Phase>`
//! se uzima u VLASNIŠTVO i mutira u mjestu (`for ph in &mut plan_phases`) — pozivatelju ionako ne
//! treba stara verzija, pa nema kloniranja.
//!
//! ZAŠTO RUST OVAKO (cigla M2/6 — aktivne faze preko `phase_tag` iz profila)
//! `Option::is_some_and` presuđuje `classify::phase_tag(...)` u jednom izrazu bez ugnježđenog
//! `match`-a: `None` (commit nema oznaku faze) ne prolazi filtar, `Some(tag)` se provjerava protiv
//! `ph.id` ili djeteta `"{id}/"`. Time vezanje commita na fazu prestaje ovisiti o tvrdom
//! `starts_with(prefix)` i umjesto toga čita regex iz profila — polje `phase_tag` i
//! `classify::phase_tag` prestaju biti mrtvi (I3, M14), a tuđi projekt s drukčijom konvencijom
//! (npr. `[M1.3]`) veže se istim putem kao Sokrat Study (`F2/3`).
use crate::civil::days_between;
use crate::{Commit, Patterns, Phase, PhaseState};

/// Raspon `[from, to]` se filtrira po `commit_date` (S-011: dosljedno tomu kako jezgra svugdje
/// filtrira `since`, i Pythonu — `git --since/--until` gleda datum commita, ne autora).
pub fn closed_phases(all_commits: &[Commit], p: &Patterns) -> Vec<Phase> {
    p.closed_phases
        .iter()
        .map(|(cp, re)| {
            let n = all_commits
                .iter()
                .filter(|c| {
                    c.commit_date.as_str() >= cp.from.as_str()
                        && c.commit_date.as_str() <= cp.to.as_str()
                        && re.is_match(&c.subject)
                })
                .count() as u32;
            Phase {
                id: cp.name.clone(),
                name: cp.name.clone(),
                state: PhaseState::Closed,
                total_bricks: n,
                done_bricks: n,
                from: Some(cp.from.clone()),
                to: Some(cp.to.clone()),
                days: days_between(&cp.from, &cp.to).map(|d| d + 1),
                commits: n,
            }
        })
        .collect()
}

pub fn active_phases(
    mut plan_phases: Vec<Phase>,
    commits: &[Commit],
    today: &str,
    p: &Patterns,
) -> Vec<Phase> {
    for ph in &mut plan_phases {
        let child = format!("{}/", ph.id);
        let mut tagged: Vec<&Commit> = commits
            .iter()
            .filter(|c| {
                crate::classify::phase_tag(&c.subject, p)
                    .is_some_and(|tag| tag == ph.id || tag.starts_with(&child))
            })
            .collect();
        tagged.sort_by_key(|c| c.author_time);
        if let Some(first) = tagged.first() {
            ph.from = Some(first.date.clone());
            ph.days = days_between(&first.date, today).map(|d| d + 1);
            ph.commits = tagged.len() as u32;
        }
        if ph.state == PhaseState::Closed {
            ph.to = Some(today.to_string());
        }
    }
    plan_phases
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Patterns, PhaseState, Profile};

    fn c(sha: &str, t: i64, date: &str, subject: &str) -> Commit {
        Commit {
            sha: sha.into(),
            author_time: t,
            commit_time: t,
            date: date.into(),
            commit_date: date.into(),
            subject: subject.into(),
            files: vec![],
        }
    }

    #[test]
    fn closed_phases_count_tagged_commits_in_range() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let all = vec![
            c("a", 1, "2026-09-02", "R1: Google prijava"),
            c("b", 2, "2026-09-02", "R1/U5: id_token"),
            c("c", 3, "2026-09-03", "R1 kasni — izvan raspona"),
            c("d", 4, "2026-08-31", "MREZA A1: baza"),
        ];
        let ph = closed_phases(&all, &p);
        assert_eq!(ph.len(), 4);
        let r1 = ph.iter().find(|x| x.name.starts_with("RAČUN R1")).unwrap();
        assert_eq!(
            (
                r1.commits,
                r1.done_bricks,
                r1.total_bricks,
                r1.days,
                r1.state
            ),
            (2, 2, 2, Some(1), PhaseState::Closed)
        );
        let mreza = ph.iter().find(|x| x.name.starts_with("MREŽA")).unwrap();
        assert_eq!((mreza.commits, mreza.days), (1, Some(2)));
    }

    #[test]
    fn closed_phases_filters_by_commit_date_not_author_date() {
        // Cherry-pick obrazac: autorov datum (`date`) upada u raspon R1, ali `commit_date` (kad je
        // commit stvarno stigao u granu) je izvan njega. S-011 broji po `commit_date`, pa se ovaj
        // commit NE smije ubrojiti u R1 (za razliku od "a"/"b" koji ostaju unutra).
        let p = Patterns::compile(&Profile::default()).unwrap();
        let all = vec![
            c("a", 1, "2026-09-02", "R1: Google prijava"),
            c("b", 2, "2026-09-02", "R1/U5: id_token"),
            Commit {
                sha: "e".into(),
                author_time: 5,
                commit_time: 5,
                date: "2026-09-02".into(),
                commit_date: "2026-09-05".into(),
                subject: "R1/X9: cherry-pick s docs grane".into(),
                files: vec![],
            },
        ];
        let ph = closed_phases(&all, &p);
        let r1 = ph.iter().find(|x| x.name.starts_with("RAČUN R1")).unwrap();
        assert_eq!(
            (r1.commits, r1.done_bricks, r1.total_bricks),
            (2, 2, 2),
            "commit 'e' ima commit_date izvan raspona pa se ne broji, iako mu je author date unutra"
        );
    }

    #[test]
    fn active_phase_gets_start_from_first_tagged_commit() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let plan = vec![
            Phase {
                id: "F1".into(),
                name: "F1 · UREĐAJ".into(),
                state: PhaseState::Running,
                total_bricks: 3,
                done_bricks: 2,
                from: None,
                to: None,
                days: None,
                commits: 0,
            },
            Phase {
                id: "F3".into(),
                name: "F3".into(),
                state: PhaseState::Planned,
                total_bricks: 2,
                done_bricks: 0,
                from: None,
                to: None,
                days: None,
                commits: 0,
            },
        ];
        let commits = vec![
            c("x", 20, "2026-09-05", "F1/2 druga"),
            c("y", 10, "2026-09-04", "F1/1 prva"),
            c("z", 30, "2026-09-06", "docs: ne"),
        ];
        let ph = active_phases(plan, &commits, "2026-09-06", &p);
        assert_eq!(
            (ph[0].from.as_deref(), ph[0].days, ph[0].commits),
            (Some("2026-09-04"), Some(3), 2)
        );
        // Brif ovdje piše `(ph[1].from, ph[1].days, ph[1].commits)` — ne prevodi se: `from` je
        // `Option<String>`, a `v[i].polje` je posudba kroz `Index`, ne vlasništvo (ista zamka kao
        // gore, samo bez `.clone()` na `String`). `.as_deref()` posuđuje `&str` umjesto pomicanja.
        assert_eq!(
            (ph[1].from.as_deref(), ph[1].days, ph[1].commits),
            (None, None, 0)
        );
    }

    #[test]
    fn active_phase_binds_commits_through_profile_phase_tag_not_hard_prefix() {
        // Struct-update sintaksa umjesto `let mut prof = Profile::default(); prof.polje = …`:
        // potonje puni SVA polja pa ih odmah prepisuje jedno, što clippy prijavljuje kao
        // `field_reassign_with_default` (M2/9).
        let prof = Profile {
            phase_tag: r"^\[(M\d)\.\d+\]".into(),
            ..Profile::default()
        };
        let p = Patterns::compile(&prof).unwrap();
        let plan = vec![Phase {
            id: "M1".into(),
            name: "Jezgra".into(),
            state: PhaseState::Running,
            total_bricks: 3,
            done_bricks: 1,
            from: None,
            to: None,
            days: None,
            commits: 0,
        }];
        let commits = vec![
            c("a", 1, "2026-09-01", "[M1.1] kostur"),
            c("b", 2, "2026-09-02", "[M1.2] parser"),
            c("x", 3, "2026-09-03", "M1/3 tudji oblik"),
        ];
        let out = active_phases(plan, &commits, "2026-09-05", &p);
        assert_eq!(
            (out[0].commits, out[0].from.as_deref(), out[0].days),
            (2, Some("2026-09-01"), Some(5))
        );
    }

    #[test]
    fn sokrat_study_default_still_binds_f2_slash_3_to_phase_f2() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let plan = vec![Phase {
            id: "F2".into(),
            name: "R".into(),
            state: PhaseState::Running,
            total_bricks: 1,
            done_bricks: 0,
            from: None,
            to: None,
            days: None,
            commits: 0,
        }];
        let out = active_phases(
            plan,
            &[
                c("a", 1, "2026-09-01", "F2/3 nesto"),
                c("b", 2, "2026-09-01", "F3/1 drugo"),
            ],
            "2026-09-01",
            &p,
        );
        assert_eq!(out[0].commits, 1);
    }
}
