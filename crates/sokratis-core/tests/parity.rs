//! ZAŠTO RUST OVAKO (cigla M1/21 — paritet s RAD.xlsx)
//! Integracijski test u `tests/` (ne `#[cfg(test)]` u `src/`): vidi jezgru kao vanjski korisnik —
//! poziva samo javni `build_report`, kao `sokratis-cli`. `serde_json::Value` čita referencu bez
//! posebne strukture: fixture je tuđi izlaz (Python). Ako test padne, prvo pitaj „je li fixture
//! snimljen s istog commita" (README uz fixture), tek onda „je li kod kriv".
use serde_json::Value;
use sokratis_core::{Profile, ReportInput, build_report};
use std::collections::HashMap;

const STAMP: &str = "2026-09-17";
/// Dopuštena razlika u satima: referenca (Python, zaokruženo na 1 decimalu u knjizi) i Sokratis
/// (puna preciznost) se smiju razlikovati za pola zaokruživanja plus malu rezervu.
const HOURS_TOLERANCE: f64 = 0.051;

fn fx(suffix: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/sokratstudy-{STAMP}.{suffix}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("fixture {path}: {e}"))
}

#[test]
fn matches_rad_xlsx_except_fixed_hours() {
    let expected: Value =
        serde_json::from_str(&fx("expected.json")).expect("expected.json je valjani JSON");
    let input = ReportInput {
        git_log: fx("log"),
        diary: Some(fx("PROGRESS.md")),
        plan: Some(fx("RASPORED.md")),
        docs: vec![],
        branches: vec![],
        overrides: HashMap::new(),
        visions: vec![],
        now: 1_789_660_685, // 2026-09-17 (provjereno: date -d @1789660685)
        today: "2026-09-17".into(),
        since: "2026-08-29".into(),
        branch: "main".into(),
    };
    let r =
        build_report(&input, &Profile::default()).expect("fixture je čist ulaz za build_report");
    assert_eq!(
        r.touched.skipped_lines, 0,
        "fixture mora biti čist (nula preskočenih redaka)"
    );

    let mut mismatches = Vec::new();

    // Dani: commiti, kumulativ, redci, isporuke, deployi, testni redci na svih 14 dana; sati se
    // uspoređuju s ISPRAVLJENOM referencom (`hours_fixed`, S-007), ne sa zastarjelim proxyjem.
    let days = expected["days"]
        .as_object()
        .expect("expected.days je objekt");
    assert_eq!(r.days.len(), days.len(), "broj radnih dana");
    for d in &r.days {
        let e = days
            .get(&d.date)
            .unwrap_or_else(|| panic!("dan {} nije u referenci", d.date));
        for (name, got, want) in [
            ("commits", d.commits as f64, e["commits"].as_f64().unwrap()),
            (
                "cumulative",
                d.commits_cumulative as f64,
                e["cumulative"].as_f64().unwrap(),
            ),
            ("lines", d.lines as f64, e["lines"].as_f64().unwrap()),
            (
                "deliveries",
                d.deliveries as f64,
                e["deliveries"].as_f64().unwrap(),
            ),
            ("deploys", d.deploys as f64, e["deploys"].as_f64().unwrap()),
            (
                "test_lines",
                d.test_lines as f64,
                e["test_lines"].as_f64().unwrap(),
            ),
        ] {
            if got != want {
                mismatches.push(format!(
                    "{} {name}: Sokratis {got} != RAD.xlsx {want}",
                    d.date
                ));
            }
        }
        assert!(d.hours >= 0.0, "{}: negativni sati {}", d.date, d.hours);
        let hours_fixed = e["hours_fixed"].as_f64().unwrap();
        if (d.hours - hours_fixed).abs() > HOURS_TOLERANCE {
            mismatches.push(format!(
                "{} hours: Sokratis {} != RAD.xlsx(hours_fixed) {hours_fixed}",
                d.date, d.hours
            ));
        }
    }

    // Vrste rada: commiti, udio, redci za svih pet vrsta (`WorkKind::id()`, S-008).
    for k in &r.kinds {
        let e = &expected["kinds"][k.kind.id()];
        let want = (
            e["commits"].as_f64().unwrap(),
            e["share"].as_f64().unwrap(),
            e["lines"].as_f64().unwrap(),
        );
        let got = (k.commits as f64, k.share, k.lines as f64);
        if got.0 != want.0 || (got.1 - want.1).abs() > 0.001 || got.2 != want.2 {
            mismatches.push(format!(
                "kind {}: Sokratis {:?} != RAD.xlsx {:?}",
                k.kind.id(),
                got,
                want
            ));
        }
    }

    // Pokazatelji: 16 od 18 jednaki referenci na bit; `hours` i `commits_per_hour` se uspoređuju
    // s ispravljenom referencom (`hours_fixed`/`commits_per_hour_fixed`), ne sa zastarjelim
    // proxyjem koji tablica danas prikazuje (`hours_legacy`/`commits_per_hour_legacy`).
    // M9: petlja ide po RUSTOVIM pokazateljima, pa bi bez ovog `assert_eq!` paritet ostao zelen
    // i kad bi jedan pokazatelj ispao iz `indicators()` — usporedba bi se samo prestala raditi.
    assert_eq!(
        r.indicators.len(),
        18,
        "brif tvrdi 18 pokazatelja; paritet mora pasti i kad jedan NEDOSTAJE"
    );
    for i in &r.indicators {
        let (want, tol) = match i.id.as_str() {
            "hours" => (
                expected["indicators"]["hours_fixed"].as_f64().unwrap(),
                HOURS_TOLERANCE,
            ),
            "commits_per_hour" => (
                expected["indicators"]["commits_per_hour_fixed"]
                    .as_f64()
                    .unwrap(),
                HOURS_TOLERANCE,
            ),
            id => (
                expected["indicators"][id]
                    .as_f64()
                    .unwrap_or_else(|| panic!("nema očekivanja za {id}")),
                1e-9,
            ),
        };
        if (i.value - want).abs() > tol {
            mismatches.push(format!(
                "indicator {}: Sokratis {} != RAD.xlsx {want}",
                i.id, i.value
            ));
        }
    }

    // Dokumentacijska tvrdnja (nije o Sokratisu): fixture doista čuva Python-kvar koji Sokratis
    // ispravlja (S-007) — zbroj sati po starom proxyju je negativan, ne procjena „približno nula".
    let hours_legacy_total = expected["indicators"]["hours_legacy"].as_f64().unwrap();
    assert!(
        hours_legacy_total < 0.0,
        "fixture mora čuvati kvar (hours_legacy = {hours_legacy_total} nije negativan)"
    );

    // Faze: usporedba po redoslijedu — i referenca i jezgra čitaju isti plan odozgo prema dolje,
    // pa redoslijed nosi identitet faze bez potrebe za usklađivanjem po imenu.
    let phases = expected["phases"]
        .as_array()
        .expect("expected.phases je niz");
    assert_eq!(r.phases.len(), phases.len(), "broj faza");
    for (got, want) in r.phases.iter().zip(phases) {
        let state = serde_json::to_value(got.state).expect("PhaseState se serijalizira u JSON");
        if got.done_bricks as f64 != want["done"].as_f64().unwrap()
            || got.total_bricks as f64 != want["total"].as_f64().unwrap()
            || state != want["state"]
        {
            mismatches.push(format!(
                "phase {} ({}): Sokratis {}/{} {:?} != RAD.xlsx {}",
                got.name, got.id, got.done_bricks, got.total_bricks, got.state, want
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "razlike prema RAD.xlsx:\n{}",
        mismatches.join("\n")
    );
}
