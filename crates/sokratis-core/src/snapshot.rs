//! ZAŠTO RUST OVAKO (cigla M2/7 — snimka brojki i prijelaz signala)
//! Ovo je snimka BROJKI (za STORE T17) — nije isto što i `tests/snapshot.rs` (insta test OBLIKA
//! `Report`-a, S-022); isto ime, različita svrha, oboje će Leon sresti u istom stablu.
//! `impl` na tipu iz `model.rs` drži ponašanje uz podatak bez traita; `HashMap` po ključu
//! (pravilo, naslov) daje O(1) usporedbu, a `Severity: Ord` čini `max()` i „najteži" i „pogoršano".
use crate::{MetricDelta, MetricValue, Report, Severity, Signal, SignalCounts, SnapshotMetrics};
use std::collections::{HashMap, HashSet};

impl SnapshotMetrics {
    /// Što snimka drži (S-014): 18 pokazatelja s vrstom, docs-ocjena (ako postoji), broj signala
    /// po težini — NE cijeli `Report` (dnevnik, isporuke, faze su prevelike i mijenjaju se prečesto).
    pub fn from_report(r: &Report) -> SnapshotMetrics {
        SnapshotMetrics {
            indicators: r
                .indicators
                .iter()
                .map(|i| MetricValue {
                    id: i.id.clone(),
                    value: i.value,
                    kind: i.kind,
                })
                .collect(),
            docs_score: r.docs.as_ref().map(|d| d.score),
            signals: SignalCounts::from_signals(&r.signals),
        }
    }
}

/// `NaN == NaN` je po IEEE 754 uvijek `false` — bez ove provjere bi pokazatelj koji je `NaN` u
/// obje snimke zauvijek izgledao „promijenjen". Izvan brifa (task-7), dodano jer bi šutnja ovdje
/// jednog dana tiho preplavila snimku lažnim razlikama (nijedan od 18 pokazatelja danas ne
/// proizvodi `NaN`, ali `diff` ne smije o tome pretpostavljati ništa).
fn value_changed(before: f64, after: f64) -> bool {
    !(before == after || (before.is_nan() && after.is_nan()))
}

/// Samo pokazatelji čija se vrijednost promijenila, u redoslijedu kao u `cur`. Pokazatelj koji
/// postoji u `cur` a ne u `prev` (npr. nov nakon nadogradnje) se preskače — brif ne propisuje
/// obrazac za „lažni prije", pa ga ne izmišljamo.
pub fn diff(prev: &SnapshotMetrics, cur: &SnapshotMetrics) -> Vec<MetricDelta> {
    let before: HashMap<&str, f64> = prev
        .indicators
        .iter()
        .map(|m| (m.id.as_str(), m.value))
        .collect();
    cur.indicators
        .iter()
        .filter_map(|m| {
            let b = *before.get(m.id.as_str())?;
            value_changed(b, m.value).then(|| MetricDelta {
                id: m.id.clone(),
                before: b,
                after: m.value,
            })
        })
        .collect()
}

impl SignalCounts {
    pub fn from_signals(signals: &[Signal]) -> SignalCounts {
        let mut c = SignalCounts::default();
        for s in signals {
            match s.severity {
                Severity::Info => c.info += 1,
                Severity::Warn => c.warn += 1,
                Severity::Alert => c.alert += 1,
            }
        }
        c
    }
}

/// Najteža ozbiljnost među signalima, ili `None` bez signala. `Severity` nosi `Ord` (S-008,
/// `model.rs`) u redoslijedu Info < Warn < Alert, pa `max()` po definiciji vrati „najgore".
pub fn worst_severity(signals: &[Signal]) -> Option<Severity> {
    signals.iter().map(|s| s.severity).max()
}

/// Signali koji su u `cur` na `Alert`, a u `prev` ih ili nije bilo ili su bili blaži — DESKTOP
/// (T29/T30) ovo zove da obavijest pošalje SAMO na prijelaz u Alert (S-020), ne na svaki Alert.
/// Identitet signala je par `(rule, title_key)`: dva signala istog pravila s različitim naslovom
/// (npr. dvije grane u `unmerged-branches`) su različiti signali, svaki prati svoj prijelaz.
/// M2/29c: rezultat ne smije ovisiti o redoslijedu ni o duplikatima — ponovljen ključ u `prev`
/// vrijedi po NAJTEŽOJ težini (ne po zadnjem u nizu), a ponovljen ključ u `cur` izlazi NAJVIŠE
/// jednom (prva pojava), inače bi isti signal poslao dvije obavijesti OS-a.
pub fn alerts_raised(prev: &[Signal], cur: &[Signal]) -> Vec<Signal> {
    let mut was: HashMap<(&str, &str), Severity> = HashMap::new();
    for s in prev {
        let key = (s.rule.as_str(), s.title_key.as_str());
        was.entry(key)
            .and_modify(|worst| *worst = (*worst).max(s.severity))
            .or_insert(s.severity);
    }
    let mut seen: HashSet<(&str, &str)> = HashSet::new();
    cur.iter()
        .filter(|s| s.severity == Severity::Alert)
        .filter(|s| {
            was.get(&(s.rule.as_str(), s.title_key.as_str()))
                .is_none_or(|old| *old < Severity::Alert)
        })
        .filter(|s| seen.insert((s.rule.as_str(), s.title_key.as_str())))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        IndicatorKind, MetricValue, Profile, Severity, Signal, SignalCounts, SnapshotMetrics,
        alerts_raised, diff, worst_severity,
    };

    fn sig(rule: &str, key: &str, sev: Severity) -> Signal {
        Signal {
            rule: rule.into(),
            severity: sev,
            title_key: key.into(),
            evidence: vec!["dokaz".into()],
            since: None,
        }
    }

    #[test]
    fn counts_and_worst() {
        let s = vec![
            sig("a", "x", Severity::Warn),
            sig("b", "y", Severity::Alert),
            sig("a", "z", Severity::Info),
        ];
        assert_eq!(
            SignalCounts::from_signals(&s),
            SignalCounts {
                info: 1,
                warn: 1,
                alert: 1
            }
        );
        assert_eq!(worst_severity(&s), Some(Severity::Alert));
        assert_eq!(worst_severity(&[]), None);
    }

    #[test]
    fn alerts_raised_only_on_transition_to_alert() {
        let prev = vec![
            sig("unmerged-branches", "feat/x", Severity::Warn),
            sig("docs-lag", "d", Severity::Alert),
        ];
        let cur = vec![
            sig("unmerged-branches", "feat/x", Severity::Alert), // Warn → Alert: javlja se
            sig("docs-lag", "d", Severity::Alert),               // već bio Alert: tišina
            sig("unmerged-branches", "feat/y", Severity::Alert), // nov: javlja se
            sig("unmerged-branches", "feat/z", Severity::Warn),  // nije Alert: tišina
        ];
        let raised = alerts_raised(&prev, &cur);
        let got: Vec<&str> = raised.iter().map(|s| s.title_key.as_str()).collect();
        assert_eq!(got, vec!["feat/x", "feat/y"]);
    }

    /// M2/29c (nošeni Minor M4 iz T7): isti `(rule, title_key)` dvaput u `cur` je do sada davao
    /// DVIJE obavijesti za isti signal — `alerts_raised` mora svaki ključ vratiti NAJVIŠE jednom.
    #[test]
    fn alerts_raised_dedupes_the_same_key_appearing_twice_in_cur() {
        let cur = vec![
            sig("unmerged-branches", "feat/x", Severity::Alert),
            sig("unmerged-branches", "feat/x", Severity::Alert),
        ];
        let raised = alerts_raised(&[], &cur);
        assert_eq!(
            raised.len(),
            1,
            "isti ključ dvaput u cur -> jedna obavijest"
        );
    }

    /// M2/29c (nošeni Minor M5 iz T7): isti `(rule, title_key)` dvaput u `prev` s različitom
    /// težinom je do sada ovisio o REDOSLIJEDU (`HashMap::collect` zadrži zadnji) — vrijedi
    /// NAJTEŽA težina bez obzira na redoslijed, pa oba poretka daju isti (prazan) rezultat.
    #[test]
    fn alerts_raised_prev_duplicate_key_uses_worst_severity_regardless_of_order() {
        let cur = vec![sig("unmerged-branches", "feat/x", Severity::Alert)];
        let alert_then_warn = vec![
            sig("unmerged-branches", "feat/x", Severity::Alert),
            sig("unmerged-branches", "feat/x", Severity::Warn),
        ];
        assert!(
            alerts_raised(&alert_then_warn, &cur).is_empty(),
            "već je bio Alert -> nema prijelaza"
        );
        let warn_then_alert = vec![
            sig("unmerged-branches", "feat/x", Severity::Warn),
            sig("unmerged-branches", "feat/x", Severity::Alert),
        ];
        assert!(
            alerts_raised(&warn_then_alert, &cur).is_empty(),
            "isti podatak, obrnut redoslijed -> isti (prazan) ishod"
        );
    }

    #[test]
    fn from_report_takes_indicators_docs_and_counts_then_diff_lists_changes() {
        let r = crate::report::build_report(&crate::report::tests::input(), &Profile::default())
            .unwrap();
        let m = SnapshotMetrics::from_report(&r);
        assert_eq!(m.indicators.len(), 18);
        // ODSTUPANJE OD BRIFA: brif traži `assert_eq!(m.docs_score, None)`, ali fixture
        // `report::tests::input()` ima `docs/records/PROGRESS.md` (poklapa se sa zadanim
        // `diary_path`) pa `docs_health` vrati `Some(DocsHealth { score: 100, .. })` — provjereno
        // ispisom (`eprintln!` u testu, uklonjeno) prije pisanja ove tvrdnje. `m.docs_score` je
        // stoga `Some(100)`, ne `None`; `r.docs.is_some()` to već tvrdi u `report::tests`.
        assert_eq!(m.docs_score, Some(100));
        let mut later = m.clone();
        later.indicators[0].value += 1.0;
        let d = diff(&m, &later);
        assert_eq!(d.len(), 1);
        assert_eq!(
            (d[0].id.as_str(), d[0].after - d[0].before),
            (m.indicators[0].id.as_str(), 1.0)
        );
        assert!(diff(&m, &m).is_empty());
    }

    /// Izvan brifa, dodano (brif šuti o `NaN`): `f64::NAN != f64::NAN` bi svaki `NaN` pokazatelj
    /// prijavljivao kao promjenu zauvijek, iako se ništa nije promijenilo. Dvije snimke s istim
    /// `NaN`-om na istom pokazatelju se smatraju NEPROMIJENJENIMA; `NaN` nasuprot broju JEST promjena.
    #[test]
    fn diff_treats_nan_to_nan_as_unchanged_but_nan_to_number_as_changed() {
        let mv = |v: f64| MetricValue {
            id: "x".into(),
            value: v,
            kind: IndicatorKind::Measure,
        };
        let snap = |v: f64| SnapshotMetrics {
            indicators: vec![mv(v)],
            docs_score: None,
            signals: SignalCounts::default(),
        };
        assert!(diff(&snap(f64::NAN), &snap(f64::NAN)).is_empty());
        assert_eq!(diff(&snap(1.0), &snap(f64::NAN)).len(), 1);
        assert_eq!(diff(&snap(f64::NAN), &snap(1.0)).len(), 1);
    }
}
