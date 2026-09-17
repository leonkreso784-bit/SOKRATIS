//! ZAŠTO RUST OVAKO (cigla M1/11a — faze)
//! Zatvorene faze se BROJE iz commita (mjera), ne prepisuju (procjena). `plan_phases: Vec<Phase>`
//! se uzima u VLASNIŠTVO i mutira u mjestu (`for ph in &mut phases`) — pozivatelju ionako ne
//! treba stara verzija, pa nema kloniranja.
use crate::civil::days_between;
use crate::{Commit, Patterns, Phase, PhaseState};

pub fn closed_phases(all_commits: &[Commit], p: &Patterns) -> Vec<Phase> {
    p.closed_phases
        .iter()
        .map(|(cp, re)| {
            let n = all_commits
                .iter()
                .filter(|c| {
                    c.date.as_str() >= cp.from.as_str()
                        && c.date.as_str() <= cp.to.as_str()
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

pub fn active_phases(mut plan_phases: Vec<Phase>, commits: &[Commit], today: &str) -> Vec<Phase> {
    for ph in &mut plan_phases {
        let prefix = format!("{}/", ph.id);
        let mut tagged: Vec<&Commit> = commits
            .iter()
            .filter(|c| c.subject.starts_with(&prefix))
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
    fn active_phase_gets_start_from_first_tagged_commit() {
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
        let ph = active_phases(plan, &commits, "2026-09-06");
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
}
