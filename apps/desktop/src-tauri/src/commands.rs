//! ZAŠTO RUST OVAKO (cigla M2/29 — jedanaest Tauri naredbi kao tanki pozivatelji)
//! `#[tauri::command]` pretvara običnu funkciju u RPC koji sučelje zove kroz `invoke(ime, args)`;
//! `Result<T, String>` je ugovor Tauri IPC-a (obrazložen uz `state::text`). Zajednička „računica"
//! (izračun izvještaja) živi u `compute`/`compute_input` niže — naredbe SAMO posuđuju stanje i
//! pozivaju je. (M2/30: `set_override`/`save_visions`/`refresh` sad zovu `engine::request_refresh`
//! umjesto da same diraju `state.reports` — detalj uz svaku naredbu niže.)
//! (M2/32: `refresh(None)` petlju sad radi `engine::refresh_all` — i tray je zove, iz zasebne niti.)
//! (M2/38: `Settings` dobiva četvrti ključ `motion: bool`, `SETTING_KEYS` postaje `[&str; 4]` —
//! `set_setting` ga upisuje kroz istu opću granu kao `theme`/`lang`, bez novog `if`.)
use crate::cache::StoreCache;
use crate::state::{AppState, text};
use crate::summary::{ProjectSummary, summarize};
use sokratis_core::{ReportInput, Vision, WorkKind, build_report};
use sokratis_io::{GitSource, IoError, Project};
use sokratis_store::TrendPoint;
use std::path::{Path, PathBuf};
use tauri_plugin_dialog::DialogExt;

// ── raspon (spec §6.1) — jedina „računica" koju desktop smije imati (S-013) ──────────────────────

/// Birač raspona, ugovor prema TS `Range` iz T25: četiri gotova presjeka ili vlastiti `since`/
/// `until`. `#[serde(tag = "preset")]` daje isti JSON oblik kao sučelje
/// (`{"preset":"7d"}`, `{"preset":"custom","since":…,"until":…}`) — provjereno testom niže.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "preset", rename_all = "snake_case")]
pub enum Range {
    All,
    #[serde(rename = "7d")]
    SevenDays,
    #[serde(rename = "30d")]
    ThirtyDays,
    Month,
    Custom {
        since: String,
        until: String,
    },
}

impl Range {
    /// `(since, until)` po ZADANOM „danas" — `compute` ga zove sa `sokratis_io::today()`, test
    /// niže s fiksnim danom. `None` = bez donje/gornje granice (isto što i „do danas"/„od početka").
    pub fn to_dates(&self, today: &str) -> (Option<String>, Option<String>) {
        match self {
            Range::All => (None, None),
            Range::SevenDays => (back(today, 6), None),
            Range::ThirtyDays => (back(today, 29), None),
            Range::Month => (Some(format!("{}-01", &today[..7])), None),
            Range::Custom { since, until } => (Some(since.clone()), Some(until.clone())),
        }
    }
}

/// `date` umanjen za `days` dana, kroz `civil::prev_day` u petlji — bez `chrono` u desktopu
/// (S-013, jezgra i `io` već imaju civilne datume). `None` bi značio neispravan `today`, što se u
/// praksi ne događa (dolazi iz `sokratis_io::today()` ili fiksnog testa).
fn back(date: &str, days: u32) -> Option<String> {
    let mut d = date.to_string();
    for _ in 0..days {
        d = sokratis_core::civil::prev_day(&d)?;
    }
    Some(d)
}

// ── postavke (spec §6.2) ──────────────────────────────────────────────────────────────────────────

/// Postavke: u bazi je tekst po ključu (`"academic"` · `"hr"` · `"off"`/`"on"`), ovdje SAMO
/// preslagane u jedan oblik za sučelje — isti razlog kao `ProjectSummary` (S-012).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub theme: String,
    pub lang: String,
    pub autostart: bool,
    pub motion: bool,
}

const SETTING_KEYS: [&str; 4] = ["theme", "lang", "autostart", "motion"];

/// Jedan ključ postavke, sa zadanom vrijednošću ako nikad nije zapisan — `motion` (M2/38) je upravo
/// takav četvrti ključ, JEDAN redak u `get_settings` niže. `pub(crate)`: motor (`engine.rs`) je zove
/// za `lang` prije obavijesti OS-a.
pub(crate) fn setting_or(
    store: &sokratis_store::Store,
    key: &str,
    default: &str,
) -> Result<String, String> {
    Ok(store
        .setting(key)
        .map_err(text)?
        .unwrap_or_else(|| default.to_string()))
}

// ── zajednički izračun (dopuna orkestratora, T29 #2) ──────────────────────────────────────────────

/// Izračun jednog izvještaja: registar → otvoren projekt → raspon → keširani log → jezgra;
/// `Report` izlazi iz `build_report` NEPROMIJENJEN (S-012), ova funkcija ništa ne preslaguje. NE
/// piše u `state.reports` — to radi SAMO motor (`engine::refresh_project`, pozvan kroz
/// `request_refresh`, koji `compute` poziva iznutra s `Range::All`), jer uzima `reports[id]` kao
/// „prošli" izvještaj za `alerts_raised` — a izračun nad kraćim rasponom (`get_report`) tu mapu ne
/// smije prljati (S-020).
pub(crate) fn compute(
    state: &AppState,
    id: i64,
    range: &Range,
) -> Result<sokratis_core::Report, String> {
    let rec = state
        .store
        .lock()
        .map_err(text)?
        .project(id)
        .map_err(text)?;
    let project = Project::open(&rec.root_path).map_err(text)?;
    let (since, until) = range.to_dates(&sokratis_io::today());
    let cache = StoreCache {
        store: &state.store,
        project_id: id,
    };
    let input = compute_input(&project, since.as_deref(), until.as_deref(), &cache)?;
    build_report(&input, &project.profile).map_err(text)
}

/// Prozor kroz keš; na `IoError::Cache`/`CacheIncomplete` (Ruling orkestratora — „politika greške")
/// javlja razlog na `stderr` i ponovi BEZ keša, umjesto da naredba padne zbog kvara POGODNOSTI —
/// keš je pogodnost, ne istina (S-014), git izravno je uvijek ispravan pad-nazad.
fn compute_input(
    project: &Project,
    since: Option<&str>,
    until: Option<&str>,
    cache: &StoreCache<'_>,
) -> Result<ReportInput, String> {
    match project.input_cached(since, until, cache) {
        Ok(input) => Ok(input),
        Err(IoError::Cache(e)) => {
            eprintln!("keš commita nedostupan, čitam git izravno: {e}");
            project.input_between(since, until).map_err(text)
        }
        Err(IoError::CacheIncomplete { sha }) => {
            eprintln!("keš commita nepotpun (nedostaje {sha}), čitam git izravno");
            project.input_between(since, until).map_err(text)
        }
        Err(e) => Err(text(e)),
    }
}

// ── registar ──────────────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn list_projects(state: tauri::State<'_, AppState>) -> Result<Vec<ProjectSummary>, String> {
    let store = state.store.lock().map_err(text)?;
    let reports = state.reports.lock().map_err(text)?;
    let mut out = Vec::new();
    for rec in store.list_projects().map_err(text)? {
        let worktrees = store.worktrees(rec.id).map_err(text)?.len() as u32;
        let error = (!rec.root_path.is_dir())
            .then(|| format!("mapa {} ne postoji", rec.root_path.display()));
        out.push(summarize(&rec, worktrees, reports.get(&rec.id), error));
    }
    Ok(out)
}

/// Otvara `dir` kao projekt, upisuje ga u registar (duplikat → `StoreError::AlreadyTracked`) i
/// bilježi radna stabla; naredba `add_project` niže je samo tanki pozivatelj oko ovoga.
fn track_project(state: &AppState, dir: &Path) -> Result<ProjectSummary, String> {
    let project = Project::open(dir).map_err(text)?;
    // `root_path` u registru je GLAVNO stablo (`main_root`), ne stablo koje je dijalog otvorio
    // (`project.root`, može biti sporedno radno stablo) — identitet projekta je `common_dir`
    // (S-015), ali `list_projects`/`compute` otvaraju `Project` PO `root_path`, pa ta putanja mora
    // preživjeti brisanje bilo kojeg sporednog stabla (nalaz recenzije, krug 1).
    let main_root = project.main_root();
    let name = main_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| main_root.to_string_lossy().to_string());
    let now = now_unix();
    let store = state.store.lock().map_err(text)?;
    let rec = store
        .add_project(&name, &main_root, &project.common_dir, now)
        .map_err(text)?;
    // `io` daje samo putanje stabala, ne granu (Ruling orkestratora, T29 #5) — sučelje crta samo
    // BROJ stabala, pa je grana ovdje prazan tekst.
    let paths = project.git.worktrees().map_err(text)?;
    let worktrees: Vec<(PathBuf, String)> = paths.into_iter().map(|p| (p, String::new())).collect();
    store.set_worktrees(rec.id, &worktrees, now).map_err(text)?;
    Ok(summarize(&rec, worktrees.len() as u32, None, None))
}

/// Sada, unix sekunde — desktop smije `std::time` izravno (isti obrazac kao `Project::input_with`).
/// `pub(crate)`: motor (`engine.rs`) je zove za `record_profile` (dopuna T30 #2).
pub(crate) fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[tauri::command]
pub async fn add_project(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<Option<ProjectSummary>, String> {
    let Some(picked) = app.dialog().file().blocking_pick_folder() else {
        return Ok(None);
    };
    let dir = picked.into_path().map_err(text)?;
    let summary = track_project(&state, &dir)?;
    // N5: nov projekt još nema nadzor — `engine::watch` ga prvi put registrira.
    crate::engine::watch(&app, summary.id);
    Ok(Some(summary))
}

#[tauri::command]
pub fn rename_project(
    state: tauri::State<'_, AppState>,
    id: i64,
    name: String,
) -> Result<(), String> {
    state
        .store
        .lock()
        .map_err(text)?
        .rename_project(id, &name)
        .map_err(text)
}

#[tauri::command]
pub fn remove_project(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<(), String> {
    state
        .store
        .lock()
        .map_err(text)?
        .remove_project(id)
        .map_err(text)?;
    state.reports.lock().map_err(text)?.remove(&id);
    crate::engine::unwatch(&app, id);
    Ok(())
}

// ── izvještaj i trend ────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_report(
    state: tauri::State<'_, AppState>,
    id: i64,
    range: Range,
) -> Result<sokratis_core::Report, String> {
    compute(&state, id, &range)
}

/// `store.trend` traži tekstualne granice, a `Range::to_dates` bez presjeka vraća `None` — ova
/// dva imenovana ruba pokrivaju „od uvijek" / „do zauvijek" (Ruling orkestratora, T29 #4).
const TREND_SINCE_FALLBACK: &str = "0000-01-01";
const TREND_UNTIL_FALLBACK: &str = "9999-12-31";

#[tauri::command]
pub fn get_trend(
    state: tauri::State<'_, AppState>,
    id: i64,
    metric: String,
    range: Range,
) -> Result<Vec<TrendPoint>, String> {
    let (since, until) = range.to_dates(&sokratis_io::today());
    let since = since.unwrap_or_else(|| TREND_SINCE_FALLBACK.to_string());
    let until = until.unwrap_or_else(|| TREND_UNTIL_FALLBACK.to_string());
    state
        .store
        .lock()
        .map_err(text)?
        .trend(id, &metric, &since, &until)
        .map_err(text)
}

// ── ručni podaci ─────────────────────────────────────────────────────────────────────────────────

/// Ručni upis (`write_override`/`write_visions`) potiskuje vlastiti odjek u watcheru PRIJE pisanja
/// (S-016), ponovno registrira nadzor (N1 — `.sokratis` je možda BAŠ SADA nastao) i odmah traži
/// osvježavanje kroz `request_refresh` — isti red kao watcher, pa se ne računa usporedno s njim
/// (Ruling R10, spec §3.3 t. 3). Greška SAMOG izračuna ide na `stderr`, ne ovamo (sučelje je svejedno
/// vidi na svom sljedećem `get_report`); greška UPISA i dalje ide van kao tekst.
#[tauri::command]
pub fn set_override(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: i64,
    sha: String,
    kind: Option<WorkKind>,
) -> Result<(), String> {
    let rec = state
        .store
        .lock()
        .map_err(text)?
        .project(id)
        .map_err(text)?;
    let project = Project::open(&rec.root_path).map_err(text)?;
    let path = project.main_root().join(".sokratis").join("overrides.json");
    crate::engine::suppress(&app, &path);
    project.write_override(&sha, kind).map_err(text)?;
    crate::engine::watch(&app, id);
    crate::engine::request_refresh(&app, id);
    Ok(())
}

/// Isti obrazac kao `set_override` iznad — potisni, upiši, ponovno nadziri, zatraži osvježavanje.
#[tauri::command]
pub fn save_visions(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: i64,
    visions: Vec<Vision>,
) -> Result<(), String> {
    let rec = state
        .store
        .lock()
        .map_err(text)?
        .project(id)
        .map_err(text)?;
    let project = Project::open(&rec.root_path).map_err(text)?;
    let path = project.main_root().join(".sokratis").join("visions.json");
    crate::engine::suppress(&app, &path);
    project.write_visions(&visions).map_err(text)?;
    crate::engine::watch(&app, id);
    crate::engine::request_refresh(&app, id);
    Ok(())
}

// ── osvježavanje ─────────────────────────────────────────────────────────────────────────────────

/// `request_refresh` (ne `engine::refresh_project` izravno) — isti red kao watcher (S-010), pa gumb
/// „Osvježi" i vanjska promjena datoteke nikad ne računaju isti projekt istodobno (Ruling R10).
/// Petlja „svi projekti" je `engine::refresh_all` (dopuna T32 #5) — tray je zove iz zasebne niti,
/// pa ta petlja ne smije postojati na dva mjesta (S-010).
#[tauri::command]
pub fn refresh(app: tauri::AppHandle, id: Option<i64>) -> Result<(), String> {
    match id {
        Some(id) => crate::engine::request_refresh(&app, id),
        None => crate::engine::refresh_all(&app),
    }
    Ok(())
}

// ── postavke ─────────────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
    let store = state.store.lock().map_err(text)?;
    Ok(Settings {
        theme: setting_or(&store, "theme", "academic")?,
        lang: setting_or(&store, "lang", "hr")?,
        autostart: setting_or(&store, "autostart", "off")? == "on",
        motion: setting_or(&store, "motion", "on")? != "off",
    })
}

/// Autostart ima DVA učinka (registracija u OS-u preko plugina + zapis u bazu) na JEDNOM mjestu,
/// `tray::set_autostart` (dopuna T32 #2) — tray-kvačica zove ISTU funkciju, pa baza nikad ne prođe
/// mimo plugina. Ostale postavke idu ravno u bazu, kao i do sada.
#[tauri::command]
pub fn set_setting(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    if !SETTING_KEYS.contains(&key.as_str()) {
        return Err(format!("nepoznata postavka '{key}'"));
    }
    if key == "autostart" {
        return crate::tray::set_autostart(&app, value == "on");
    }
    state
        .store
        .lock()
        .map_err(text)?
        .set_setting(&key, &value)
        .map_err(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_json_round_trip_matches_ts_shape() {
        let cases = [
            (Range::All, r#"{"preset":"all"}"#),
            (Range::SevenDays, r#"{"preset":"7d"}"#),
            (Range::ThirtyDays, r#"{"preset":"30d"}"#),
            (Range::Month, r#"{"preset":"month"}"#),
        ];
        for (value, json) in cases {
            assert_eq!(serde_json::to_string(&value).unwrap(), json);
            let back: Range = serde_json::from_str(json).unwrap();
            assert_eq!(format!("{back:?}"), format!("{value:?}"));
        }
        let custom = Range::Custom {
            since: "2026-01-01".into(),
            until: "2026-01-31".into(),
        };
        let json = serde_json::to_string(&custom).unwrap();
        assert_eq!(
            json,
            r#"{"preset":"custom","since":"2026-01-01","until":"2026-01-31"}"#
        );
        let back: Range = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{back:?}"), format!("{custom:?}"));
    }

    #[test]
    fn to_dates_crosses_month_and_year_for_7d_and_30d() {
        assert_eq!(Range::All.to_dates("2026-09-17"), (None, None));
        assert_eq!(
            Range::SevenDays.to_dates("2026-01-03"),
            (Some("2025-12-28".to_string()), None),
            "7d preko granice godine"
        );
        assert_eq!(
            Range::ThirtyDays.to_dates("2026-03-05"),
            (Some("2026-02-04".to_string()), None),
            "30d preko granice mjeseca"
        );
        assert_eq!(
            Range::Month.to_dates("2026-09-17"),
            (Some("2026-09-01".to_string()), None)
        );
        let custom = Range::Custom {
            since: "2026-01-01".into(),
            until: "2026-01-31".into(),
        };
        assert_eq!(
            custom.to_dates("2026-09-17"),
            (
                Some("2026-01-01".to_string()),
                Some("2026-01-31".to_string())
            )
        );
    }
}
