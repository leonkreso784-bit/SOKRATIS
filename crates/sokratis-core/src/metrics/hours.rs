//! ZAŠTO RUST OVAKO (cigla M1/8 — sati)
//! `Vec<&Commit>` = vektor POSUDBI: sortiramo reference, ne kopiramo commite. `BTreeMap` umjesto
//! `HashMap` jer želimo dane uzlazno bez naknadnog sortiranja. `entry().or_insert()` = „uzmi ili
//! stvori" u jednom potezu. Ovo je ispravak S-007: sortiranje po `author_time` čini negativan
//! razmak nemogućim.
use crate::Commit;
use std::collections::BTreeMap;

/// Procjena sati rada po danu iz razmaka između uzastopnih commita (paritet s `RAD.xlsx`).
/// Commiti se prvo sortiraju po `author_time` (S-007: log zna doći u drugom redoslijedu, npr.
/// zbog cherry-picka) — bez toga bi razmak mogao ispasti negativan. Razmak manji od `gap_h` sati
/// broji se kao stvarni rad; veći razmak (ili prvi commit) znači da je otvorena nova sesija pa se
/// pribraja fiksni `start_h` (pretpostavka koliko je autor radio prije prvog commita u sesiji).
/// Sat se pripisuje danu (`Commit.date`, autorov datum) commita koji zatvara razmak.
pub fn hours_per_day(commits: &[Commit], gap_h: f64, start_h: f64) -> BTreeMap<String, f64> {
    let mut sorted: Vec<&Commit> = commits.iter().collect();
    sorted.sort_by_key(|c| c.author_time);
    let mut hours: BTreeMap<String, f64> = BTreeMap::new();
    let mut prev: Option<i64> = None;
    for c in sorted {
        let gap = prev.map(|p| (c.author_time - p) as f64 / 3600.0);
        // Imenovana odluka (umjesto `match` koji je tiho spajao "nema prethodnika" i "razmak
        // prevelik" u istu granu): `is_some_and` vraća false i kad je `gap` `None`, pa oba slučaja
        // padaju u "nova sesija" bez promjene ponašanja.
        let same_session = gap.is_some_and(|g| g < gap_h);
        let add = if same_session {
            gap.unwrap_or(start_h)
        } else {
            start_h
        };
        *hours.entry(c.date.clone()).or_insert(0.0) += add;
        prev = Some(c.author_time);
    }
    hours
        .into_iter()
        .map(|(d, h)| (d, (h * 10.0).round() / 10.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Commit;

    fn c(sha: &str, author_time: i64, date: &str) -> Commit {
        Commit {
            sha: sha.into(),
            author_time,
            commit_time: author_time,
            date: date.into(),
            commit_date: date.into(),
            subject: String::new(),
            files: vec![],
        }
    }

    #[test]
    fn cherry_pick_never_yields_negative_hours() {
        // Redoslijed loga: X (07.09. 23:45) pa cherry-pick Y s autorskim datumom 06.09. 05:16.
        // Stari proxy: Y − X = −42,5 h. Novi: sortiraj pa računaj.
        let log_order = vec![
            c("x", 1_788_817_514, "2026-09-07"),
            c("y", 1_788_664_600, "2026-09-06"),
            c("z", 1_788_817_514 + 3_600, "2026-09-07"),
        ];
        let h = hours_per_day(&log_order, 2.0, 0.5);
        assert!(h.values().all(|v| *v >= 0.0), "{h:?}");
        assert_eq!(h["2026-09-06"], 0.5, "prvi u nizu dobiva početak sesije");
        assert_eq!(
            h["2026-09-07"], 1.5,
            "x otvara novu sesiju (+0,5), z zatvara razmak od 1 h"
        );
    }

    #[test]
    fn empty_input_gives_empty_map() {
        assert!(hours_per_day(&[], 2.0, 0.5).is_empty());
    }

    #[test]
    fn gap_equal_to_threshold_starts_new_session_and_rounds() {
        let v = vec![
            c("a", 0, "1970-01-01"),
            c("b", 7_200, "1970-01-01"),
            c("c", 7_200 + 4_000, "1970-01-01"),
        ];
        let h = hours_per_day(&v, 2.0, 0.5);
        assert_eq!(h["1970-01-01"], 0.5 + 0.5 + 1.1, "4000 s = 1,11 h → 1,1");
    }
}
