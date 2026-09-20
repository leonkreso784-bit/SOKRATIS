// ZAŠTO OVAKO (cigla M2/17 — snimke brojki, profil kao kanonski JSON, trend)
// Integracijski test (u `tests/`) vidi samo javni API cratea, isto kao pozivatelj iz `desktop`
// (T29/T30) — dokazuje ugovor iz brifa, ne unutarnje detalje SQL-a.

use sokratis_core::{IndicatorKind, MetricValue, SignalCounts, SnapshotMetrics};
use sokratis_store::{Store, canonical_json};
use std::path::Path;

fn m(commits: f64, docs: Option<u8>) -> SnapshotMetrics {
    SnapshotMetrics {
        indicators: vec![
            MetricValue {
                id: "commits".into(),
                value: commits,
                kind: IndicatorKind::Measure,
            },
            MetricValue {
                id: "hours".into(),
                value: 2.5,
                kind: IndicatorKind::Proxy,
            },
        ],
        docs_score: docs,
        signals: SignalCounts {
            info: 0,
            warn: 1,
            alert: 0,
        },
    }
}

#[test]
fn canonical_json_sorts_keys() {
    #[derive(serde::Serialize)]
    struct X {
        b: u8,
        a: u8,
    }
    assert_eq!(
        canonical_json(&X { b: 2, a: 1 }).unwrap(),
        r#"{"a":1,"b":2}"#
    );
}

#[test]
fn one_snapshot_per_day_latest_value_wins_and_trend_marks_profile_change() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let prof1 = s.record_profile(p.id, r#"{"a":1}"#, 1).unwrap();
    assert_eq!(
        s.record_profile(p.id, r#"{"a":1}"#, 2).unwrap(),
        prof1,
        "isti profil = isti id"
    );
    let prof2 = s.record_profile(p.id, r#"{"a":2}"#, 3).unwrap();
    assert!(!s.has_snapshot(p.id, "2026-09-16").unwrap());
    s.save_snapshot(p.id, "2026-09-16", prof1, &m(10.0, Some(90)))
        .unwrap();
    s.save_snapshot(p.id, "2026-09-16", prof1, &m(12.0, Some(90)))
        .unwrap(); // isti dan → pregazi
    s.save_snapshot(p.id, "2026-09-17", prof1, &m(15.0, None))
        .unwrap();
    s.save_snapshot(p.id, "2026-09-18", prof2, &m(20.0, Some(100)))
        .unwrap();
    assert!(s.has_snapshot(p.id, "2026-09-16").unwrap());
    let t = s
        .trend(p.id, "commits", "2026-09-01", "2026-09-30")
        .unwrap();
    let got: Vec<(&str, f64, bool)> = t
        .iter()
        .map(|x| (x.taken_on.as_str(), x.value, x.profile_changed))
        .collect();
    assert_eq!(
        got,
        vec![
            ("2026-09-16", 12.0, false),
            ("2026-09-17", 15.0, false),
            ("2026-09-18", 20.0, true),
        ]
    );
    let docs = s
        .trend(p.id, "docs_score", "2026-09-01", "2026-09-30")
        .unwrap();
    assert_eq!(docs.len(), 2, "dan bez docs-a nema red, ne nulu");
    let (day, latest) = s.latest_snapshot(p.id).unwrap().unwrap();
    assert_eq!(day, "2026-09-18");
    assert_eq!(latest, m(20.0, Some(100)));
}

/// Brif traži da profil bude KANONSKI JSON (naziv cigle): dva teksta s istim sadržajem, drukčijim
/// poretkom ključeva, moraju dati isti `profile_seen_id`; promjena vrijednosti mora dati drugi.
#[test]
fn record_profile_treats_key_order_as_irrelevant() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let same_order = s.record_profile(p.id, r#"{"a":1,"b":2}"#, 1).unwrap();
    let swapped_order = s.record_profile(p.id, r#"{"b":2,"a":1}"#, 2).unwrap();
    assert_eq!(
        same_order, swapped_order,
        "isti sadržaj, drugi poredak ključeva -> isti profile_seen_id"
    );
    let changed_value = s.record_profile(p.id, r#"{"a":1,"b":3}"#, 3).unwrap();
    assert_ne!(
        same_order, changed_value,
        "razlika u jednoj vrijednosti -> drugi profile_seen_id"
    );
}

/// SQLite pretvara `NaN` u NULL pri vezanju (`sqlite3VdbeMemSetDouble`), a stupac `value` je
/// `NOT NULL` — bez zaštite bi cijela snimka pukla na jednoj pokvarenoj metrici. Zato se takva
/// metrika PRESKAČE (nema retka), ostale u istoj snimci ostaju.
#[test]
fn non_finite_metric_is_skipped_not_stored_as_zero() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let prof = s.record_profile(p.id, r#"{"a":1}"#, 1).unwrap();
    let metrics = SnapshotMetrics {
        indicators: vec![
            MetricValue {
                id: "commits".into(),
                value: f64::NAN,
                kind: IndicatorKind::Measure,
            },
            MetricValue {
                id: "hours".into(),
                value: f64::INFINITY,
                kind: IndicatorKind::Proxy,
            },
            MetricValue {
                id: "lines".into(),
                value: 3.0,
                kind: IndicatorKind::Measure,
            },
        ],
        docs_score: None,
        signals: SignalCounts::default(),
    };
    s.save_snapshot(p.id, "2026-09-16", prof, &metrics).unwrap();
    assert!(
        s.trend(p.id, "commits", "2026-09-01", "2026-09-30")
            .unwrap()
            .is_empty(),
        "NaN metrika se ne sprema, ne postaje 0.0"
    );
    assert!(
        s.trend(p.id, "hours", "2026-09-01", "2026-09-30")
            .unwrap()
            .is_empty(),
        "beskonačnost se ne sprema"
    );
    let lines = s.trend(p.id, "lines", "2026-09-01", "2026-09-30").unwrap();
    assert_eq!(lines.len(), 1, "konačna metrika u istoj snimci ostaje");
    assert_eq!(lines[0].value, 3.0);
}

#[test]
fn save_snapshot_for_unknown_project_is_no_such_project() {
    let s = Store::open_in_memory().unwrap();
    let err = s
        .save_snapshot(999, "2026-09-16", 1, &m(1.0, None))
        .unwrap_err();
    assert!(matches!(
        err,
        sokratis_store::StoreError::NoSuchProject(999)
    ));
}

/// Recenzija M2/17, krug 1 (Important): upsert „po (project, dan, metrika)" iz brifa NE briše
/// retke tog dana kojih u novom pozivu nema, pa bi `latest_snapshot`/`trend` vraćali MJEŠAVINU
/// jutrošnje i večernje snimke. `save_snapshot` mora dan zamijeniti CIJELOG: manje pokazatelja
/// navečer (npr. `docs_score` ode iz `Some` u `None`) ne smije ostaviti jutrošnji redak.
#[test]
fn second_snapshot_of_the_day_replaces_the_whole_day_not_row_by_row() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let prof = s.record_profile(p.id, r#"{"a":1}"#, 1).unwrap();

    let morning = SnapshotMetrics {
        indicators: vec![
            MetricValue {
                id: "commits".into(),
                value: 10.0,
                kind: IndicatorKind::Measure,
            },
            MetricValue {
                id: "hours".into(),
                value: 2.0,
                kind: IndicatorKind::Proxy,
            },
            MetricValue {
                id: "lines".into(),
                value: 100.0,
                kind: IndicatorKind::Measure,
            },
        ],
        docs_score: Some(90),
        signals: SignalCounts {
            info: 1,
            warn: 0,
            alert: 0,
        },
    };
    // navečer: "lines" izostaje, docs_score prelazi u None
    let evening = SnapshotMetrics {
        indicators: vec![
            MetricValue {
                id: "commits".into(),
                value: 12.0,
                kind: IndicatorKind::Measure,
            },
            MetricValue {
                id: "hours".into(),
                value: 2.5,
                kind: IndicatorKind::Proxy,
            },
        ],
        docs_score: None,
        signals: SignalCounts {
            info: 0,
            warn: 0,
            alert: 0,
        },
    };

    s.save_snapshot(p.id, "2026-09-16", prof, &morning).unwrap();
    s.save_snapshot(p.id, "2026-09-16", prof, &evening).unwrap();

    let (day, latest) = s.latest_snapshot(p.id).unwrap().unwrap();
    assert_eq!(day, "2026-09-16");
    assert_eq!(
        latest, evening,
        "večernja snimka je CIJELA zamjena, ne mješavina s jutarnjom"
    );
    assert!(
        s.trend(p.id, "lines", "2026-09-01", "2026-09-30")
            .unwrap()
            .is_empty(),
        "pokazatelj koji navečer izostaje ne smije preživjeti iz jutra"
    );
    assert!(
        s.trend(p.id, "docs_score", "2026-09-01", "2026-09-30")
            .unwrap()
            .is_empty(),
        "docs_score koji navečer postane None ne smije preživjeti iz jutra"
    );
}

/// Isti nalaz kao gore, ali izostanak je posljedica `NaN`-a (metrika koja je jutro bila konačna,
/// navečer nije) — jutrošnji konačan redak ne smije preživjeti samo zato što je večernji preskočen.
#[test]
fn second_snapshot_of_the_day_with_nan_metric_does_not_resurrect_the_morning_row() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let prof = s.record_profile(p.id, r#"{"a":1}"#, 1).unwrap();

    let morning = SnapshotMetrics {
        indicators: vec![
            MetricValue {
                id: "commits".into(),
                value: 10.0,
                kind: IndicatorKind::Measure,
            },
            MetricValue {
                id: "lines".into(),
                value: 100.0,
                kind: IndicatorKind::Measure,
            },
        ],
        docs_score: Some(80),
        signals: SignalCounts::default(),
    };
    let evening = SnapshotMetrics {
        indicators: vec![
            MetricValue {
                id: "commits".into(),
                value: 11.0,
                kind: IndicatorKind::Measure,
            },
            MetricValue {
                id: "lines".into(),
                value: f64::NAN,
                kind: IndicatorKind::Measure,
            },
        ],
        docs_score: Some(80),
        signals: SignalCounts::default(),
    };

    s.save_snapshot(p.id, "2026-09-16", prof, &morning).unwrap();
    s.save_snapshot(p.id, "2026-09-16", prof, &evening).unwrap();

    assert!(
        s.trend(p.id, "lines", "2026-09-01", "2026-09-30")
            .unwrap()
            .is_empty(),
        "NaN navečer ne smije ostaviti jutrošnju konačnu vrijednost"
    );
    let (_, latest) = s.latest_snapshot(p.id).unwrap().unwrap();
    assert_eq!(
        latest.indicators.len(),
        1,
        "samo commits ostaje; lines je preskočen zbog NaN, ne pregažen jutrošnjim brojem"
    );
}

/// Profil se vrati na stari sadržaj (A → B → A): treći dan mora dobiti ISTI `profile_seen_id` kao
/// prvi (kanonski JSON je isti), a `trend` mora označiti promjenu na OBA prijelaza (16.→17., 17.→18.).
#[test]
fn profile_seen_id_is_reused_when_profile_reverts_and_trend_marks_each_change() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let a = s.record_profile(p.id, r#"{"a":1}"#, 1).unwrap();
    let b = s.record_profile(p.id, r#"{"a":2}"#, 2).unwrap();
    let a_again = s.record_profile(p.id, r#"{"a":1}"#, 3).unwrap();
    assert_eq!(
        a, a_again,
        "povratak na stari sadržaj vraća STARI profile_seen_id"
    );
    assert_ne!(a, b);

    s.save_snapshot(p.id, "2026-09-16", a, &m(1.0, None))
        .unwrap();
    s.save_snapshot(p.id, "2026-09-17", b, &m(2.0, None))
        .unwrap();
    s.save_snapshot(p.id, "2026-09-18", a_again, &m(3.0, None))
        .unwrap();

    let t = s
        .trend(p.id, "commits", "2026-09-01", "2026-09-30")
        .unwrap();
    let changed: Vec<bool> = t.iter().map(|x| x.profile_changed).collect();
    assert_eq!(
        changed,
        vec![false, true, true],
        "16. je prva točka (nikad promjena), 17. i 18. su oba prijelaza"
    );
}

/// `ON DELETE CASCADE` u shemi (M2/1) mora odnijeti i snimke i viđene profile — provjereno kroz
/// javni API (čitanje nakon brisanja), ne izravnim upitom nad `conn` (`pub(crate)`, nedostupno tests/).
/// Drugi projekt ostaje netaknut (cascade gađa samo obrisani `project_id`).
#[test]
fn removing_project_cascades_its_profiles_and_snapshots() {
    let s = Store::open_in_memory().unwrap();
    let p = s
        .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
        .unwrap();
    let q = s
        .add_project("SS2", Path::new(r"C:\ss2"), Path::new(r"C:\ss2\.git"), 1)
        .unwrap();
    let prof = s.record_profile(p.id, r#"{"a":1}"#, 1).unwrap();
    let prof_q = s.record_profile(q.id, r#"{"a":1}"#, 1).unwrap();
    s.save_snapshot(p.id, "2026-09-16", prof, &m(10.0, Some(90)))
        .unwrap();
    s.save_snapshot(q.id, "2026-09-16", prof_q, &m(5.0, Some(50)))
        .unwrap();

    s.remove_project(p.id).unwrap();

    assert!(!s.has_snapshot(p.id, "2026-09-16").unwrap());
    assert!(s.latest_snapshot(p.id).unwrap().is_none());
    assert!(
        s.trend(p.id, "commits", "2026-09-01", "2026-09-30")
            .unwrap()
            .is_empty()
    );
    // drugi projekt nije diran
    assert!(s.has_snapshot(q.id, "2026-09-16").unwrap());
    let (day, latest) = s.latest_snapshot(q.id).unwrap().unwrap();
    assert_eq!(day, "2026-09-16");
    assert_eq!(latest, m(5.0, Some(50)));
}
