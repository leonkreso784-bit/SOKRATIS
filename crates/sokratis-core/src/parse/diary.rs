//! ZAŠTO RUST OVAKO (cigla M1/4 — parser dnevnika)
//! `text.lines()` + `captures(line)` po retku umjesto `(?m)` nad cijelim tekstom: isti ishod, a
//! greška u jednom naslovu ne ruši sve. `sort_by(|a, b| a.date.cmp(&b.date))` je STABILAN —
//! redoslijed unutar istog datuma ostaje kakav je u datoteci (kao Python `sorted`).
//!
//! Dopuna (cigla M2/47 — unija dnevnika, S-033): `HashSet<(String, String)>` kao „već viđeno" je
//! najjednostavniji način da `filter` odbaci duplikat po (datum, naslov) bez ugniježđene petlje;
//! `flat_map` čuva redoslijed teksta (vodeće stablo prvo) pa `filter`-ov `insert` vrati `false`
//! (već postoji) baš za KASNIJI (stariji) tekst — prvi viđeni pobjeđuje bez dodatnog `match`-a.
use crate::classify::classify_kind;
use crate::{Delivery, Patterns};
use std::collections::HashSet;

pub fn parse_diary(text: &str, p: &Patterns, since: &str) -> Vec<Delivery> {
    let mut out: Vec<Delivery> = text
        .lines()
        .filter_map(|line| p.diary_heading.captures(line))
        .filter_map(|c| {
            let date = c.get(1)?.as_str().to_string();
            if date.as_str() < since {
                return None;
            }
            let model = c
                .get(2)
                .map(|m| m.as_str().trim().to_uppercase())
                .unwrap_or_default();
            let title = c.get(3)?.as_str().trim().to_string();
            let lower = title.to_lowercase();
            let deploy = title.contains('🚀') || p.diary_deploy.is_match(&lower);
            let kind = classify_kind(&title, p);
            Some(Delivery {
                date,
                model,
                title,
                kind,
                deploy,
            })
        })
        .collect();
    out.sort_by(|a, b| a.date.cmp(&b.date));
    out
}

/// Unija isporuka iz više dnevnika (S-033): svaki tekst prolazi `parse_diary`, zatim se redci
/// spajaju po ključu (datum, naslov) — PRVI viđeni pobjeđuje (pozivatelj šalje vodeće stablo prvo),
/// pa `model`/`deploy` uređeni u grani u kojoj se radi nadjačaju `main`. Ishod je sortiran po
/// datumu, stabilno (kao `parse_diary`).
pub fn parse_diaries(texts: &[String], p: &Patterns, since: &str) -> Vec<Delivery> {
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut out: Vec<Delivery> = texts
        .iter()
        .flat_map(|t| parse_diary(t, p, since))
        .filter(|d| seen.insert((d.date.clone(), d.title.clone())))
        .collect();
    out.sort_by(|a, b| a.date.cmp(&b.date));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Patterns, Profile, WorkKind};

    #[test]
    fn headings_become_deliveries_sorted_and_filtered() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let text = include_str!("../../tests/fixtures/diary-sample.md");
        let d = parse_diary(text, &p, "2026-08-29");
        assert_eq!(
            d.len(),
            2,
            "unos prije `since` i naslov bez datuma otpadaju"
        );
        assert_eq!(d[0].date, "2026-09-12");
        assert_eq!(d[0].model, "FABLE");
        assert!(d[0].title.starts_with("Napredak pouzdan"));
        assert_eq!(d[0].kind, WorkKind::Debugging, "'kvarova' → debugging");
        assert!(!d[0].deploy);
        assert_eq!(d[1].model, "FABLE, SESIJA F2/2");
        assert!(d[1].deploy, "🚀 u naslovu");
        assert_eq!(d[1].kind, WorkKind::Documentation, "naslov počinje s docs:");
    }

    /// Paritet sa Sokrat Studyjevim `rad-xlsx.py`: isti dnevnik, isti `since`, isti broj
    /// isporuka — i po danu, ne samo ukupno (105 se lako slaže slučajno, raspored po danu ne).
    #[test]
    fn matches_sokrat_study_delivery_count_per_day() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let text = include_str!("../../tests/fixtures/sokratstudy-2026-09-17.PROGRESS.md");
        let expected_json =
            include_str!("../../tests/fixtures/sokratstudy-2026-09-17.expected.json");
        let expected: serde_json::Value = serde_json::from_str(expected_json).unwrap();

        let deliveries = parse_diary(text, &p, "2026-08-29");
        assert_eq!(
            deliveries.len(),
            expected["indicators"]["deliveries"].as_u64().unwrap() as usize,
            "ukupan broj isporuka mora odgovarati Pythonu"
        );

        let mut by_day: std::collections::BTreeMap<&str, u32> = std::collections::BTreeMap::new();
        for d in &deliveries {
            *by_day.entry(d.date.as_str()).or_insert(0) += 1;
        }
        let expected_days = expected["days"].as_object().unwrap();
        for (date, expected_day) in expected_days {
            let want = expected_day["deliveries"].as_u64().unwrap() as u32;
            let got = by_day.get(date.as_str()).copied().unwrap_or(0);
            assert_eq!(got, want, "dan {date}: Rust {got} vs Python {want}");
        }
    }

    /// Dva teksta s jednim preklopljenim (datum, naslov): unija ih broji JEDNOM, prvi tekst
    /// pobjeđuje. Dodatni rub (dopuna orkestratora, spec §2): ISTI naslov na DVA RAZLIČITA datuma
    /// ostaje DVAPUT — ključ je par (datum, naslov), ne sam naslov.
    #[test]
    fn union_drops_duplicate_date_and_title_but_keeps_same_title_on_other_dates() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let lead = "## 2026-09-20 (FABLE) — F6/1 MCP most\n".to_string();
        let main = "\
## 2026-09-13 (FABLE) — F2/9 zid gotov\n\
## 2026-09-20 (OPUS) — F6/1 MCP most\n\
## 2026-09-27 (OPUS) — F6/1 MCP most\n"
            .to_string();
        let d = parse_diaries(&[lead, main], &p, "2026-09-01");
        assert_eq!(d.len(), 3, "preklop (20. 9.) se broji jednom");
        let titles_at: Vec<&str> = d
            .iter()
            .filter(|x| x.title == "F6/1 MCP most")
            .map(|x| x.date.as_str())
            .collect();
        assert_eq!(
            titles_at,
            ["2026-09-20", "2026-09-27"],
            "isti naslov na dva različita datuma ostaje dvaput"
        );
        let overlap = d.iter().find(|x| x.date == "2026-09-20").unwrap();
        assert_eq!(overlap.model, "FABLE", "vodeći tekst (prvi) pobjeđuje");
    }
}
