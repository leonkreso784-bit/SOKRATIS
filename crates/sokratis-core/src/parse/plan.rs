//! ZAŠTO RUST OVAKO (cigla M1/5 — parser plana)
//! Dva prolaza kroz retke: prvi u `HashMap` skuplja imena faza (redoslijed ovdje nije bitan),
//! drugi broji cigle. Redoslijed PRVOG POJAVLJIVANJA faza čuva `Vec` uz `position()` +
//! indeksiranje (`&mut phases[idx]`) — posudba traje samo dok se brojevi upisuju, za razliku od
//! `iter_mut().find()` koja bi držala `&mut` kroz cijeli `match` i tražila `expect()` na "upravo
//! dodanom" elementu. Bez `expect()` je čitljivije i nema izlaza iz pravila #6 (bez `expect` u
//! jezgri).
use crate::{Patterns, Phase, PhaseState};
use std::collections::HashMap;

pub fn parse_plan(text: &str, p: &Patterns) -> Vec<Phase> {
    let names: HashMap<String, String> = text
        .lines()
        .filter_map(|l| p.plan_phase_name.captures(l))
        .map(|c| (c[1].to_string(), c[2].trim().to_string()))
        .collect();

    let mut phases: Vec<Phase> = Vec::new();
    for c in text.lines().filter_map(|l| p.plan_brick.captures(l)) {
        let id = c[1].to_string();
        let done = !c[3].is_empty();
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
