//! ZAŠTO RUST OVAKO (cigla M1/5 — parser plana · popravak C1)
//! Dva prolaza kroz retke: prvi u `HashMap` skuplja imena faza, drugi broji cigle. Redoslijed
//! PRVOG POJAVLJIVANJA čuva `Vec` uz `position()` + indeksiranje (`&mut phases[idx]`), bez
//! `expect()` na „upravo dodanom" elementu (pravilo #6). Grupe iz `Captures` čitamo `get(n)` uz
//! `Option`, ne `c[n]`: indeksiranje PANICIRA kad regex iz profila tu grupu nema (nalaz C1).
use crate::{Patterns, Phase, PhaseState};
use std::collections::HashMap;

pub fn parse_plan(text: &str, p: &Patterns) -> Vec<Phase> {
    let names: HashMap<String, String> = text
        .lines()
        .filter_map(|l| p.plan_phase_name.captures(l))
        .filter_map(|c| {
            let id = c.get(1)?.as_str().to_string();
            let name = c.get(2)?.as_str().trim().to_string();
            Some((id, name))
        })
        .collect();

    let mut phases: Vec<Phase> = Vec::new();
    for c in text.lines().filter_map(|l| p.plan_brick.captures(l)) {
        let Some(id) = c.get(1).map(|m| m.as_str().to_string()) else {
            continue;
        };
        let done = c.get(3).is_some_and(|m| !m.as_str().is_empty());
        let idx = match phases.iter().position(|ph| ph.id == id) {
            Some(i) => i,
            None => {
                let name = match names.get(&id) {
                    Some(n) => format!("{id} · {n}"),
                    None => id.clone(),
                };
                phases.push(Phase {
                    id: id.clone(),
                    name,
                    state: PhaseState::Planned,
                    total_bricks: 0,
                    done_bricks: 0,
                    from: None,
                    to: None,
                    days: None,
                    commits: 0,
                });
                phases.len() - 1
            }
        };
        let phase = &mut phases[idx];
        phase.total_bricks += 1;
        if done {
            phase.done_bricks += 1;
        }
    }

    for ph in &mut phases {
        ph.state = if ph.total_bricks > 0 && ph.done_bricks == ph.total_bricks {
            PhaseState::Closed
        } else if ph.done_bricks > 0 {
            PhaseState::Running
        } else {
            PhaseState::Planned
        };
    }
    phases
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Patterns, PhaseState, Profile};

    #[test]
    fn bricks_and_states_from_plan() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let ph = parse_plan(include_str!("../../tests/fixtures/plan-sample.md"), &p);
        let ids: Vec<&str> = ph.iter().map(|x| x.id.as_str()).collect();
        assert_eq!(ids, ["F1", "F2", "F3", "F4"]);
        assert_eq!(ph[0].name, "F1 · UREĐAJ — izgled i glatkoća");
        assert_eq!(
            (ph[0].total_bricks, ph[0].done_bricks, ph[0].state),
            (3, 2, PhaseState::Running)
        );
        assert_eq!(
            (ph[1].total_bricks, ph[1].done_bricks, ph[1].state),
            (1, 1, PhaseState::Closed)
        );
        assert_eq!(
            (ph[2].total_bricks, ph[2].done_bricks, ph[2].state),
            (2, 0, PhaseState::Planned)
        );
        assert_eq!(ph[3].state, PhaseState::Closed);
        assert!(ph[0].from.is_none() && ph[0].commits == 0);
    }

    /// C1 (završna recenzija M1): profil smije sadržavati VALJAN regex bez capture-grupa.
    /// Stari kod je na `c[1]` paniciralo („no group at index '1'", izlaz 101); danas takav regex
    /// odbije `Patterns::compile` s imenom polja, pa CLI vrati izlaz 3.
    #[test]
    fn plan_regex_without_capture_groups_is_an_error_not_a_panic() {
        let profile = Profile {
            plan_brick: r"^\| \*\*M".into(),
            ..Profile::default()
        };
        let message = match Patterns::compile(&profile) {
            Err(e) => e.to_string(),
            Ok(p) => {
                // Druga brana: ako provjera grupa jednom ispadne iz `compile`, `parse_plan`
                // svejedno ne smije paničariti — `Captures::get` vraća `Option`, ne panika.
                assert!(parse_plan("| **M1/1** ✅ |\n", &p).is_empty());
                return;
            }
        };
        assert!(
            message.contains("plan_brick"),
            "greška mora imenovati polje profila: {message}"
        );
    }

    /// Paritet sa Sokrat Studyjevim `rad-xlsx.py`: isti `RASPORED.md`, iste cigle i stanja.
    /// `expected.json["phases"]` ima 11 unosa — prva 4 su POVIJESNE zatvorene faze iz
    /// `Profile.closed_phases` (Sokrat Studyjev predm1-obrazac; broje se iz git tagova, ne iz
    /// teksta plana) i ne spadaju u `parse_plan` (ugovor T5: samo `plan_phase_name` i
    /// `plan_brick`). Uspoređujemo zadnjih 7 (F1..F7) — one STVARNO nastaju iz `RASPORED.md`.
    #[test]
    fn matches_sokrat_study_plan_phases() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let text = include_str!("../../tests/fixtures/sokratstudy-2026-09-17.RASPORED.md");
        let expected_json =
            include_str!("../../tests/fixtures/sokratstudy-2026-09-17.expected.json");
        let expected: serde_json::Value = serde_json::from_str(expected_json).unwrap();

        let phases = parse_plan(text, &p);
        let all_expected = expected["phases"].as_array().unwrap();
        let expected_plan_phases = &all_expected[4..];
        assert_eq!(
            phases.len(),
            expected_plan_phases.len(),
            "broj faza iz plana (F1..F7)"
        );
        for (ph, exp) in phases.iter().zip(expected_plan_phases) {
            assert_eq!(ph.name, exp["name"].as_str().unwrap(), "ime faze {}", ph.id);
            let want_state = match exp["state"].as_str().unwrap() {
                "closed" => PhaseState::Closed,
                "running" => PhaseState::Running,
                "planned" => PhaseState::Planned,
                other => panic!("nepoznato stanje u expected.json: {other}"),
            };
            assert_eq!(ph.state, want_state, "stanje faze {}", ph.id);
            assert_eq!(
                ph.done_bricks as u64,
                exp["done"].as_u64().unwrap(),
                "gotove cigle {}",
                ph.id
            );
            assert_eq!(
                ph.total_bricks as u64,
                exp["total"].as_u64().unwrap(),
                "ukupno cigli {}",
                ph.id
            );
        }
    }
}
