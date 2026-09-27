//! ZAŠTO RUST OVAKO (cigla M2/64 — lanci nespojenih grana)
//! Graf roditelja je `HashMap<String, Vec<String>>` (commit → njegovi roditelji) — najjednostavniji
//! prikaz grafa nad TEKSTOM koji `io` preda (S-002: jezgra ne zna za git proces). Obilazak predaka
//! ide ITERATIVNO sa stogom `Vec` (`push`/`pop`), ne rekurzijom — repo može imati tisuće commita, a
//! rekurzija bi za dugu granu mogla prepuniti stog poziva. `HashSet` posjećenih sprječava ponovni
//! obilazak istog commita (dvije grane mogu dijeliti dio povijesti) i beskonačnu petlju.
use crate::BranchInfo;
use std::collections::{HashMap, HashSet};

/// Tekst `git log --branches --not <zadana> --format=%H|%P` → karta `commit → roditelji`. Redak bez
/// `|` (prazan ili neispravan) `split_once` vrati `None`, `filter_map` ga tiho preskoči — jedan
/// neispravan redak ne smije srušiti cijeli graf.
pub fn parse_parents(text: &str) -> HashMap<String, Vec<String>> {
    text.lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(sha, parents)| {
            let parents = parents.split_whitespace().map(str::to_string).collect();
            (sha.trim().to_string(), parents)
        })
        .collect()
}

/// Svi preci commita `start` UNUTAR `parents` — stog `Vec` umjesto rekurzije, `HashSet` pamti
/// posjećene. Roditelj kojeg `parents` ne poznaje je NA ZADANOJ grani (izvan nespojenog skupa koji
/// je `io` poslao) — obilazak na tom rubu jednostavno stane, ne puca.
fn ancestors_within(start: &str, parents: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = parents.get(start).cloned().unwrap_or_default();
    while let Some(sha) = stack.pop() {
        if seen.insert(sha.clone())
            && let Some(grandparents) = parents.get(&sha)
        {
            stack.extend(grandparents.iter().cloned());
        }
    }
    seen
}

/// Je li grana `candidate` sadržana u grani `other` (S-038): `tip(candidate) ∈ preci(other) ∪
/// {tip(other)}` i `candidate ≠ other`. Kad su vrhovi ISTI commit (dvije reference na isto mjesto),
/// obična jednakost bi bila simetrična (obje bi „sadržavale" jedna drugu) — zato jednakost broji
/// SAMO u smjeru veće imena prema manjem: manje ime je vrh (ugovor iz `model.rs`).
fn is_contained(
    candidate: &BranchInfo,
    other: &BranchInfo,
    ancestors_of_other: &HashSet<String>,
) -> bool {
    if candidate.tip == other.tip {
        candidate.name > other.name
    } else {
        ancestors_of_other.contains(&candidate.tip)
    }
}

/// Puni `contained_in` za svaku nespojenu granu (S-038, spec §1.4): vrh (grana koju nitko ne sadrži)
/// dobiva `None`, ostale ime vrha koji ih sadrži — kad je grana sadržana u više vrhova (rijetko: isti
/// commit s dvije reference, ili grananje s dva potomka), pobjeđuje vrh s najviše `ahead_of_default`,
/// pa manje ime. Spojene/zadana grana se ne diraju (već imaju `contained_in: None` iz konstrukcije).
pub fn assign_containment(branches: &mut [BranchInfo], parents: &HashMap<String, Vec<String>>) {
    let unmerged: Vec<usize> = branches
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.merged)
        .map(|(i, _)| i)
        .collect();

    // Preci se računaju JEDNOM po grani (karta ime → skup) — svaka grana se uspoređuje sa SVIM
    // ostalima, pa bi ponovni obilazak istog grafa za svaki par bio višestruko skuplji bez razloga.
    let ancestors: HashMap<String, HashSet<String>> = unmerged
        .iter()
        .map(|&i| {
            (
                branches[i].name.clone(),
                ancestors_within(&branches[i].tip, parents),
            )
        })
        .collect();

    for &i in &unmerged {
        let mut candidates: Vec<(u32, String)> = Vec::new();
        for &j in &unmerged {
            if i == j {
                continue;
            }
            // `ancestors` je izgrađen iz ISTOG popisa `unmerged` gore, pa svaki indeks `j` iz njega
            // ima svoj unos — `let...else` umjesto `expect()` (pravilo #6): rub koji se ne smije
            // dogoditi jednostavno preskoči granu, ne ruši mjerenje.
            let Some(ancestors_of_j) = ancestors.get(&branches[j].name) else {
                continue;
            };
            if is_contained(&branches[i], &branches[j], ancestors_of_j) {
                candidates.push((branches[j].ahead_of_default, branches[j].name.clone()));
            }
        }
        branches[i].contained_in = candidates
            .into_iter()
            .max_by(|(ahead_a, name_a), (ahead_b, name_b)| {
                ahead_a.cmp(ahead_b).then_with(|| name_b.cmp(name_a))
            })
            .map(|(_, name)| name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn branch(name: &str, tip: &str, ahead: u32) -> BranchInfo {
        BranchInfo {
            name: name.into(),
            last_commit_time: 0,
            ahead_of_default: ahead,
            merged: false,
            tip: tip.into(),
            contained_in: None,
        }
    }

    /// Tekst gradi lanac `a:c1 ⊂ b:c2 ⊂ c:c3 ⊂ d:c4` (svaka sesija od prethodne) i odvojenu `e:c5`
    /// — `m9` NIJE u mapi, znači je na zadanoj grani, obilazak ondje stane.
    fn chain_text() -> &'static str {
        "c4|c3\nc3|c2\nc2|c1\nc1|m9\nc5|m9\n"
    }

    #[test]
    fn parse_parents_splits_sha_from_parents_and_skips_blank_lines() {
        let map = parse_parents("c4|c3\n\nc5|m9 other\n");
        assert_eq!(
            map.get("c4").map(Vec::as_slice),
            Some(&["c3".to_string()][..])
        );
        assert_eq!(
            map.get("c5").map(Vec::as_slice),
            Some(&["m9".to_string(), "other".to_string()][..])
        );
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn linear_chain_gives_one_top_and_the_rest_point_to_it() {
        let parents = parse_parents(chain_text());
        let mut branches = vec![
            branch("a", "c1", 3),
            branch("b", "c2", 12),
            branch("c", "c3", 21),
            branch("d", "c4", 50),
            branch("e", "c5", 34),
        ];
        assign_containment(&mut branches, &parents);
        let by_name = |name: &str| {
            branches
                .iter()
                .find(|b| b.name == name)
                .expect("grana postoji")
                .contained_in
                .clone()
        };
        assert_eq!(by_name("a"), Some("d".to_string()));
        assert_eq!(by_name("b"), Some("d".to_string()));
        assert_eq!(by_name("c"), Some("d".to_string()));
        assert_eq!(by_name("d"), None, "d je vrh lanca");
        assert_eq!(by_name("e"), None, "e je odvojena, sama je vrh");
    }

    /// Determinizam (dopuna R63.6): isti rezultat bez obzira na redoslijed grana u ulazu.
    #[test]
    fn linear_chain_result_does_not_depend_on_input_order() {
        let parents = parse_parents(chain_text());
        let mut reversed = vec![
            branch("e", "c5", 34),
            branch("d", "c4", 50),
            branch("c", "c3", 21),
            branch("b", "c2", 12),
            branch("a", "c1", 3),
        ];
        assign_containment(&mut reversed, &parents);
        let by_name = |name: &str| {
            reversed
                .iter()
                .find(|b| b.name == name)
                .expect("grana postoji")
                .contained_in
                .clone()
        };
        assert_eq!(by_name("a"), Some("d".to_string()));
        assert_eq!(by_name("b"), Some("d".to_string()));
        assert_eq!(by_name("c"), Some("d".to_string()));
        assert_eq!(by_name("d"), None);
        assert_eq!(by_name("e"), None);
    }

    /// Dvije grane na ISTOM commitu: manje ime je vrh (ugovor iz `model.rs`).
    #[test]
    fn two_branches_on_the_same_tip_the_smaller_name_is_the_top() {
        let parents = parse_parents(chain_text());
        let mut branches = vec![branch("x", "c5", 34), branch("e", "c5", 34)];
        assign_containment(&mut branches, &parents);
        let by_name = |name: &str| {
            branches
                .iter()
                .find(|b| b.name == name)
                .expect("grana postoji")
                .contained_in
                .clone()
        };
        assert_eq!(by_name("e"), None, "e je manje ime, vrh");
        assert_eq!(by_name("x"), Some("e".to_string()));
    }

    #[test]
    fn merged_and_default_branch_stay_untouched() {
        let parents = parse_parents(chain_text());
        let mut branches = vec![
            BranchInfo {
                name: "main".into(),
                last_commit_time: 0,
                ahead_of_default: 0,
                merged: true,
                tip: "c4".into(),
                contained_in: None,
            },
            branch("d", "c4", 50),
        ];
        assign_containment(&mut branches, &parents);
        assert_eq!(branches[0].contained_in, None, "spojena grana se ne dira");
    }

    #[test]
    fn empty_graph_leaves_every_branch_as_its_own_top() {
        let parents = parse_parents("");
        let mut branches = vec![branch("a", "c1", 1), branch("b", "c2", 2)];
        assign_containment(&mut branches, &parents);
        assert!(branches.iter().all(|b| b.contained_in.is_none()));
    }
}
