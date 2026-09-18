//! ZAŠTO RUST OVAKO (cigla M2/17 — snimke brojki, profil kao kanonski JSON, trend)
//! `canonical_json` ide preko `serde_json::Value`: bez feature `preserve_order` je `Value::Object`
//! `BTreeMap`, pa ponovna serijalizacija SORTIRA ključeve — dva profila s istim sadržajem u drugom
//! poretku daju isti string pa isti `profile_seen_id` (provjereno: `preserve_order` nije uključen
//! nigdje u `Cargo.lock`). `ON CONFLICT … DO UPDATE` je upsert u jednom SQL-u bez utrke; jedna
//! `unchecked_transaction` drži cijelu snimku (redak po metrici) kao jednu cjelinu — S-014.
use crate::{Store, StoreError};
use rusqlite::{OptionalExtension, params};
use serde::Serialize;
use sokratis_core::{IndicatorKind, MetricValue, SignalCounts, SnapshotMetrics};

/// Jedna točka trenda: dan, vrijednost, i je li se profil promijenio od PRETHODNE točke u nizu
/// (prva točka niza nikad nije „promjena" — nema prethodne).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TrendPoint {
    pub taken_on: String,
    pub value: f64,
    pub profile_changed: bool,
}

/// Kanonski JSON: sortirani ključevi, bez razmaka — dva teksta s istim sadržajem u drugom poretku
/// ključeva daju isti string. Koristi `serde_json::Value` kao posrednika (S-014, T17).
pub fn canonical_json<T: Serialize>(v: &T) -> Result<String, StoreError> {
    let value = serde_json::to_value(v)?;
    Ok(serde_json::to_string(&value)?)
}

/// Naziv stupca `kind` u tablici `snapshot`. Ručni `match` umjesto `serde_json::to_value` jer je
/// `IndicatorKind` dvočlan enum bez podataka — pretvorba ne može propasti, pa joj ne treba `Result`.
fn kind_to_text(kind: IndicatorKind) -> &'static str {
    match kind {
        IndicatorKind::Measure => "measure",
        IndicatorKind::Proxy => "proxy",
    }
}

/// Obrat od `kind_to_text` — čita se iz baze, pa TEKST može biti oštećen (drugi program, ručna
/// izmjena datoteke); zato vraća `Result`, a ne `match` s neizbježnom granom.
fn kind_from_text(text: &str) -> Result<IndicatorKind, StoreError> {
    match text {
        "measure" => Ok(IndicatorKind::Measure),
        "proxy" => Ok(IndicatorKind::Proxy),
        other => Err(StoreError::BadIndicatorKind(other.to_string())),
    }
}

impl Store {
    /// Bilježi profil kao kanonski JSON i vraća njegov `id` (nov ili postojeći — isti sadržaj,
    /// drugi poredak ključeva, isti `id`). Snimka uz njega pamti OVAJ id (S-014): trend tako zna
    /// je li se profil promijenio između dvaju dana bez ijednog commita.
    pub fn record_profile(&self, id: i64, profile_json: &str, now: i64) -> Result<i64, StoreError> {
        self.project(id)?;
        let value: serde_json::Value = serde_json::from_str(profile_json)?;
        let canonical = canonical_json(&value)?;
        self.conn.execute(
            "INSERT OR IGNORE INTO profile_seen (project_id, json, first_seen) VALUES (?1, ?2, ?3)",
            params![id, canonical, now],
        )?;
        Ok(self.conn.query_row(
            "SELECT id FROM profile_seen WHERE project_id = ?1 AND json = ?2",
            params![id, canonical],
            |r| r.get(0),
        )?)
    }

    /// Postoji li već snimka za taj projekt i dan (bez obzira na metriku) — desktop (T30) time
    /// odlučuje treba li uopće mjeriti danas.
    pub fn has_snapshot(&self, id: i64, taken_on: &str) -> Result<bool, StoreError> {
        Ok(self
            .conn
            .query_row(
                "SELECT 1 FROM snapshot WHERE project_id = ?1 AND taken_on = ?2 LIMIT 1",
                params![id, taken_on],
                |_| Ok(()),
            )
            .optional()?
            .is_some())
    }

    /// Upisuje jednu dnevnu snimku: jedan redak po pokazatelju + `docs_score` (ako postoji) +
    /// tri `signals_*` retka, sve u JEDNOJ transakciji (pola snimke je gore nego nijedna).
    /// Nekonačna vrijednost (`NaN`/beskonačnost) se PRESKAČE — SQLite `NaN` pretvara u NULL pri
    /// vezanju parametra, a stupac `value` je `NOT NULL`, pa bi inače pukla cijela transakcija.
    pub fn save_snapshot(
        &self,
        id: i64,
        taken_on: &str,
        profile_seen_id: i64,
        m: &SnapshotMetrics,
    ) -> Result<(), StoreError> {
        self.project(id)?;
        let tx = self.conn.unchecked_transaction()?;
        for indicator in &m.indicators {
            upsert_metric(
                &tx,
                id,
                taken_on,
                profile_seen_id,
                &indicator.id,
                indicator.value,
                indicator.kind,
            )?;
        }
        if let Some(docs) = m.docs_score {
            upsert_metric(
                &tx,
                id,
                taken_on,
                profile_seen_id,
                "docs_score",
                f64::from(docs),
                IndicatorKind::Measure,
            )?;
        }
        upsert_metric(
            &tx,
            id,
            taken_on,
            profile_seen_id,
            "signals_info",
            f64::from(m.signals.info),
            IndicatorKind::Measure,
        )?;
        upsert_metric(
            &tx,
            id,
            taken_on,
            profile_seen_id,
            "signals_warn",
            f64::from(m.signals.warn),
            IndicatorKind::Measure,
        )?;
        upsert_metric(
            &tx,
            id,
            taken_on,
            profile_seen_id,
            "signals_alert",
            f64::from(m.signals.alert),
            IndicatorKind::Measure,
        )?;
        tx.commit()?;
        Ok(())
    }

    /// Niz točaka jedne metrike u razdoblju, po danu; `profile_changed` uspoređuje `profile_seen_id`
    /// s PRETHODNOM točkom istog niza (ne s cijelom poviješću) — prva točka nikad nije promjena.
    pub fn trend(
        &self,
        id: i64,
        metric: &str,
        since: &str,
        until: &str,
    ) -> Result<Vec<TrendPoint>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT taken_on, value, profile_seen_id FROM snapshot
             WHERE project_id = ?1 AND metric = ?2 AND taken_on BETWEEN ?3 AND ?4
             ORDER BY taken_on",
        )?;
        let rows = stmt.query_map(params![id, metric, since, until], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, f64>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })?;
        let mut points = Vec::new();
        let mut previous_profile: Option<i64> = None;
        for row in rows {
            let (taken_on, value, profile_seen_id) = row?;
            let profile_changed = previous_profile.is_some_and(|p| p != profile_seen_id);
            previous_profile = Some(profile_seen_id);
            points.push(TrendPoint {
                taken_on,
                value,
                profile_changed,
            });
        }
        Ok(points)
    }

    /// Zadnji dan koji ima snimku i njegove brojke, ponovno sastavljene u `SnapshotMetrics`.
    /// Redoslijed `indicators` prati redoslijed UMETANJA (stupac `rowid`, koji `ON CONFLICT DO
    /// UPDATE` ne mijenja) — brif šuti o poretku, a jednakost `SnapshotMetrics` u testu ovisi o njemu.
    pub fn latest_snapshot(
        &self,
        id: i64,
    ) -> Result<Option<(String, SnapshotMetrics)>, StoreError> {
        let latest_day: Option<String> = self.conn.query_row(
            "SELECT MAX(taken_on) FROM snapshot WHERE project_id = ?1",
            [id],
            |r| r.get(0),
        )?;
        let Some(day) = latest_day else {
            return Ok(None);
        };
        let mut stmt = self.conn.prepare(
            "SELECT metric, value, kind FROM snapshot
             WHERE project_id = ?1 AND taken_on = ?2 ORDER BY rowid",
        )?;
        let rows = stmt.query_map(params![id, day], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, f64>(1)?,
                r.get::<_, String>(2)?,
            ))
        })?;
        let mut indicators = Vec::new();
        let mut docs_score = None;
        let mut signals = SignalCounts::default();
        for row in rows {
            let (metric, value, kind_text) = row?;
            match metric.as_str() {
                "docs_score" => docs_score = Some(value as u8),
                "signals_info" => signals.info = value as u32,
                "signals_warn" => signals.warn = value as u32,
                "signals_alert" => signals.alert = value as u32,
                _ => indicators.push(MetricValue {
                    id: metric,
                    value,
                    kind: kind_from_text(&kind_text)?,
                }),
            }
        }
        Ok(Some((
            day,
            SnapshotMetrics {
                indicators,
                docs_score,
                signals,
            },
        )))
    }
}

/// Upisuje/mijenja jedan redak snimke; preskače nekonačne vrijednosti (vidi `save_snapshot`).
fn upsert_metric(
    tx: &rusqlite::Transaction<'_>,
    project_id: i64,
    taken_on: &str,
    profile_seen_id: i64,
    metric: &str,
    value: f64,
    kind: IndicatorKind,
) -> Result<(), StoreError> {
    if !value.is_finite() {
        return Ok(());
    }
    tx.execute(
        "INSERT INTO snapshot (project_id, taken_on, profile_seen_id, metric, value, kind)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(project_id, taken_on, metric) DO UPDATE SET
             value = excluded.value,
             profile_seen_id = excluded.profile_seen_id,
             kind = excluded.kind",
        params![
            project_id,
            taken_on,
            profile_seen_id,
            metric,
            value,
            kind_to_text(kind)
        ],
    )?;
    Ok(())
}
