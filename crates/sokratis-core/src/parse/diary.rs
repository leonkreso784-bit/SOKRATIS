//! ZAŠTO RUST OVAKO (cigla M1/4 — parser dnevnika)
//! `text.lines()` + `captures(line)` po retku umjesto `(?m)` nad cijelim tekstom: isti ishod, a
//! greška u jednom naslovu ne ruši sve. `sort_by(|a, b| a.date.cmp(&b.date))` je STABILAN —
//! redoslijed unutar istog datuma ostaje kakav je u datoteci (kao Python `sorted`).
use crate::classify::classify_kind;
use crate::{Delivery, Patterns};

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
}
