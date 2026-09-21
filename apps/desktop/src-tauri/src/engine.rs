//! ZAŠTO RUST OVAKO (cigla M2/30 — motor osvježavanja)
//! `AppHandle` je klon-jeftin ključ do stanja i događaja iz bilo koje niti; `app.emit` šalje JSON
//! svim prozorima; nit watchera živi koliko i proces jer `Sender` ostaje u `Watcher`-u (u
//! `state.watcher`), pa `rx.recv()` nikad ne vrati `Disconnected`.
use crate::commands::{Range, compute, now_unix, setting_or};
use crate::splash;
use crate::state::{AppState, text};
use serde::Serialize;
use sokratis_core::{Profile, Report, Severity, Signal, SnapshotMetrics, alerts_raised};
use sokratis_io::{GitSource, Project, WatchEvent, Watcher};
use sokratis_store::canonical_json;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;

/// Teret `report_updated` (R4, spec §5.1) — TS strana čita `event.payload.project_id`.
#[derive(Serialize, Clone)]
struct ReportUpdated {
    project_id: i64,
}

/// Teret `signal_raised` (R4) — polja moraju odgovarati `onSignalRaised` u `apps/desktop/src/lib/api.ts`.
#[derive(Serialize, Clone)]
struct SignalRaised {
    project_id: i64,
    rule: String,
    severity: Severity,
}

/// Pokreće nadzor datoteka i motor osvježavanja (S-016): registrira sve projekte iz registra
/// SINKRONO (u `setup`, brzo — samo `notify::watch`), pa u NOVOJ niti prvo izračuna svaki projekt
/// jednom (ne blokira `setup`), zatim čeka `WatchEvent` dok proces živi. Greška pri pokretanju
/// watchera se zapiše i motor ostaje bez automatskog osvježavanja — ručni `refresh` i dalje radi.
pub fn start(app: &AppHandle) {
    let (tx, rx) = mpsc::channel::<WatchEvent>();
    let watcher = match Watcher::new(tx, Duration::from_millis(600)) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("motor: nadzor datoteka se nije pokrenuo, osvježavanje ostaje ručno: {e}");
            return;
        }
    };

    let state = app.state::<AppState>();
    let ids: Vec<i64> = match state
        .store
        .lock()
        .map_err(text)
        .and_then(|s| s.list_projects().map_err(text))
    {
        Ok(list) => list.iter().map(|p| p.id).collect(),
        Err(e) => {
            eprintln!("motor: popis projekata nije dostupan: {e}");
            Vec::new()
        }
    };
    match state.watcher.lock() {
        Ok(mut guard) => *guard = Some(watcher),
        Err(e) => eprintln!("motor: brava watchera: {e}"),
    }
    match state.rx.lock() {
        Ok(mut guard) => *guard = Some(rx),
        Err(e) => eprintln!("motor: brava reda događaja: {e}"),
    }

    for id in &ids {
        watch(app, *id);
    }

    let app = app.clone();
    std::thread::spawn(move || {
        // Prvi izračun svih projekata ide kroz `request_refresh`, isti red kao naredbe i watcher
        // (Ruling R10, spec §3.3 t. 3) — naredba koja stigne DOK se aplikacija tek diže tako ne
        // pokrene drugi usporedni izračun istog projekta, nego samo produži red.
        for id in &ids {
            request_refresh(&app, *id);
        }
        // Drugi uvjet splasha (S-019, M2/31): prvi izračun SVIH projekata je gotov bez obzira na
        // to je li koji od njih vratio grešku — "gotovo" znači da je petlja iznad prošla do kraja.
        splash::loaded(&app);

        // `rx` se VADI iz `AppState` (`take()`) PRIJE blokirajućeg `recv()` — brava se ne smije
        // držati preko čekanja, inače bi svaka druga nit koja treba `AppState` čekala zauvijek.
        let rx = match app.state::<AppState>().rx.lock() {
            Ok(mut guard) => guard.take(),
            Err(e) => {
                eprintln!("motor: brava reda događaja: {e}");
                None
            }
        };
        let Some(rx) = rx else {
            return;
        };

        while let Ok(event) = rx.recv() {
            // N2: `event.reason` je razlog ZADNJEG sirovog događaja u prozoru — motor ga ne koristi
            // za odluku, izračun je uvijek pun (`Range::All` u `refresh_project`).
            request_refresh(&app, event.project_id);
        }
    });
}

/// Izračun je SERIJSKI po projektu (spec §3.3, t. 3): dok jedan traje, novi zahtjev ZAMJENJUJE
/// čekanje umjesto da uđe u red (`RefreshQueue`, `crates/sokratis-io/src/watch.rs`). Jedan red za
/// SVE pozivatelje (S-010) — nit watchera, prvi izračun pri pokretanju, naredbe `refresh`/
/// `set_override`/`save_visions`, uskoro i tray (T32+) — nijedan ne smije mimoići red i računati
/// isti projekt usporedno s nekim drugim (recenzija, krug 1).
pub fn request_refresh(app: &AppHandle, id: i64) {
    let should_run = match app.state::<AppState>().queue.lock() {
        Ok(mut q) => q.on_event(id),
        Err(e) => {
            eprintln!("motor: brava reda čekanja: {e}");
            return;
        }
    };
    if !should_run {
        return;
    }
    let mut current = id;
    loop {
        if let Err(e) = refresh_project(app, current) {
            eprintln!("motor: osvježavanje projekta {current} nije uspjelo: {e}");
        }
        // `queue.on_done` se zove BEZ OBZIRA na ishod gornjeg poziva — inače bi projekt čiji
        // izračun padne zauvijek ostao "u tijeku" i nikad se više ne bi osvježio.
        let again = match app.state::<AppState>().queue.lock() {
            Ok(mut q) => q.on_done(current),
            Err(e) => {
                eprintln!("motor: brava reda čekanja: {e}");
                None
            }
        };
        match again {
            Some(next) => current = next,
            None => break,
        }
    }
}

/// Jedan prolaz motora za projekt `id`, uvijek nad CIJELIM projektom (dopuna T30 #1 — dnevna
/// snimka mora biti isto mjerilo iz dana u dan). Redoslijed: `compute` (T29, jedini izračun) →
/// dnevna snimka (best-effort) → `reports.insert` (vraća STARI izvještaj kao `prev`) →
/// `report_updated` → nove obavijesti SAMO ako je `prev` postojao. Jedina greška koja se vraća
/// pozivatelju (`request_refresh`, JEDINI pozivatelj — izvan `engine.rs` se ovo ne zove, S-010) je
/// greška SAMOG izračuna; snimka, slanje događaja i obavijest OS-a su best-effort (zapišu se na
/// `stderr`, ne prekidaju osvježavanje).
fn refresh_project(app: &AppHandle, id: i64) -> Result<(), String> {
    let state = app.state::<AppState>();
    let report = compute(&state, id, &Range::All)?;

    snapshot_today(&state, id, &report);

    let prev = match state.reports.lock() {
        Ok(mut reports) => reports.insert(id, report.clone()),
        Err(e) => {
            eprintln!("motor: brava izvještaja za projekt {id}: {e}");
            None
        }
    };

    if let Err(e) = app.emit("report_updated", ReportUpdated { project_id: id }) {
        eprintln!("motor: slanje report_updated za projekt {id} nije uspjelo: {e}");
    }

    // Bez prijašnjeg izvještaja nema prijelaza (dopuna T30 #3): prvi izračun nakon pokretanja nikad
    // ne javlja, jer `alerts_raised(&[], cur)` bi lažno prijavio SVAKI postojeći Alert kao nov.
    if let Some(prev) = prev {
        notify_new_alerts(app, &state, id, &prev.signals, &report.signals);
    }
    Ok(())
}

/// Zapisuje dnevnu snimku (profil pa metrike, S-014) pri SVAKOM izračunu (dopuna T30 #2 —
/// `save_snapshot` je cjelovita zamjena dana). Greška se NE širi dalje: izvještaj je važniji od
/// trenda, `report_updated` ide svejedno.
fn snapshot_today(state: &AppState, id: i64, report: &Report) {
    if let Err(e) = try_snapshot(state, id, report) {
        eprintln!("motor: dnevna snimka za projekt {id} nije uspjela: {e}");
    }
}

fn try_snapshot(state: &AppState, id: i64, report: &Report) -> Result<(), String> {
    let rec = state
        .store
        .lock()
        .map_err(text)?
        .project(id)
        .map_err(text)?;
    let project = Project::open(&rec.root_path).map_err(text)?;
    let profile_json = canonical_json(&project.profile).map_err(text)?;
    let store = state.store.lock().map_err(text)?;
    let profile_seen_id = store
        .record_profile(id, &profile_json, now_unix())
        .map_err(text)?;
    store
        .save_snapshot(
            id,
            &sokratis_io::today(),
            profile_seen_id,
            &SnapshotMetrics::from_report(report),
        )
        .map_err(text)
}

/// Nove obavijesti (prijelaz u Alert, S-020): jedan `signal_raised` + jedna obavijest OS-a po
/// signalu koji je TEK postao Alert. `alerts_raised` (M2/29c) je već deterministički — motor ne
/// radi vlastiti dedup.
fn notify_new_alerts(app: &AppHandle, state: &AppState, id: i64, prev: &[Signal], cur: &[Signal]) {
    let raised = alerts_raised(prev, cur);
    if raised.is_empty() {
        return;
    }
    let (name, lang) = match state.store.lock().map_err(text) {
        Ok(store) => {
            let name = store.project(id).map(|r| r.name).unwrap_or_else(|e| {
                eprintln!("motor: ime projekta {id}: {e}");
                id.to_string()
            });
            let lang = setting_or(&store, "lang", "hr").unwrap_or_else(|e| {
                eprintln!("motor: postavka jezika: {e}");
                "hr".to_string()
            });
            (name, lang)
        }
        Err(e) => {
            eprintln!("motor: obavijest za projekt {id}: {e}");
            return;
        }
    };
    for signal in &raised {
        if let Err(e) = app.emit(
            "signal_raised",
            SignalRaised {
                project_id: id,
                rule: signal.rule.clone(),
                severity: signal.severity,
            },
        ) {
            eprintln!("motor: slanje signal_raised za projekt {id} nije uspjelo: {e}");
        }
        notify_os(app, &name, signal, &lang);
    }
}

/// Obavijest OS-a (S-020): naslov = ime projekta, tijelo = natpis pravila + prvi redak dokaza (bez
/// dokaza: samo natpis). Klik na obavijest NE otvara projekt — plugin na desktopu nema povratni
/// poziv za to (poznato ograničenje, dopuna T30 #5, spec §5.3).
fn notify_os(app: &AppHandle, project_name: &str, signal: &Signal, lang: &str) {
    let title = rule_title(lang, &signal.rule);
    let body = match signal.evidence.first() {
        Some(first) => format!("{title} — {first}"),
        None => title,
    };
    if let Err(e) = app
        .notification()
        .builder()
        .title(project_name)
        .body(body)
        .show()
    {
        eprintln!("motor: obavijest OS-a nije uspjela: {e}");
    }
}

const HR_DICT: &str = include_str!("../../src/lib/i18n/hr.json");
const EN_DICT: &str = include_str!("../../src/lib/i18n/en.json");

/// Natpis pravila iz SUČELJEVOG rječnika (jedan rječnik, S-010 — Rust ga SAMO čita). Ključevi su
/// plosnati (`rule.<rule>`, npr. `rule.unmerged-branches`, NE `Signal.title_key`); nepoznat `rule`
/// vraća sam sebe.
fn rule_title(lang: &str, rule: &str) -> String {
    let dict = if lang == "en" { EN_DICT } else { HR_DICT };
    let key = format!("rule.{rule}");
    serde_json::from_str::<serde_json::Value>(dict)
        .ok()
        .and_then(|v| v.get(&key).and_then(|s| s.as_str()).map(str::to_string))
        .unwrap_or_else(|| rule.to_string())
}

/// Registrira nadzor za jedan projekt: `watch_project` NIJE idempotentan (N5, `crates/sokratis-io/
/// src/watch.rs`), pa svaki poziv prvo `unwatch_project`, tek onda ponovno gleda svježi popis
/// stabala i mapa dokumentacije. Zove ga `start` (svi projekti), `add_project` (nov projekt) i
/// `set_override`/`save_visions` (N1 — `.sokratis` je možda BAŠ SADA nastao, pa nerekurzivni
/// zamjenski nadzor mora postati rekurzivan).
pub(crate) fn watch(app: &AppHandle, id: i64) {
    let state = app.state::<AppState>();
    let rec = match state
        .store
        .lock()
        .map_err(text)
        .and_then(|s| s.project(id).map_err(text))
    {
        Ok(rec) => rec,
        Err(e) => {
            eprintln!("motor: nadzor za projekt {id}: {e}");
            return;
        }
    };
    let project = match Project::open(&rec.root_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("motor: nadzor za projekt {id}: {e}");
            return;
        }
    };
    let worktrees = match project.git.worktrees() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("motor: nadzor za projekt {id}: {e}");
            return;
        }
    };
    let dirs = docs_dirs(&project.profile);
    let dirs: Vec<&str> = dirs.iter().map(String::as_str).collect();
    let mut guard = match state.watcher.lock() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("motor: brava watchera: {e}");
            return;
        }
    };
    let Some(w) = guard.as_mut() else {
        return;
    };
    w.unwatch_project(id);
    if let Err(e) = w.watch_project(
        id,
        &project.common_dir,
        &worktrees,
        &dirs,
        &project.main_root(),
    ) {
        eprintln!("motor: nadzor za projekt {id} nije uspio: {e}");
    }
}

/// Prestaje nadzirati projekt (uklonjen iz registra) — zove ga `remove_project`.
pub(crate) fn unwatch(app: &AppHandle, id: i64) {
    let state = app.state::<AppState>();
    match state.watcher.lock() {
        Ok(mut guard) => {
            if let Some(w) = guard.as_mut() {
                w.unwatch_project(id);
            }
        }
        Err(e) => eprintln!("motor: brava watchera: {e}"),
    }
}

/// Potiskuje sljedeći upis u `path` na 2 s (S-016) — MORA se pozvati PRIJE pisanja: `write_override`/
/// `write_visions` vrate putanju istom tek KAD upis već završi, prekasno za potiskivanje.
pub(crate) fn suppress(app: &AppHandle, path: &Path) {
    let state = app.state::<AppState>();
    match state.watcher.lock() {
        Ok(guard) => {
            if let Some(w) = guard.as_ref() {
                w.suppress(path, Duration::from_secs(2));
            }
        }
        Err(e) => eprintln!("motor: brava watchera: {e}"),
    }
}

/// Mape koje watcher gleda kao "dokumentacija" (S-016): `docs_dir` iz profila, plus roditeljske
/// mape `diary_path`-a i `plan_path`-a (mogu biti izvan `docs_dir`, npr. `docs/plan`).
fn docs_dirs(profile: &Profile) -> Vec<String> {
    let mut dirs = vec![profile.docs_dir.clone()];
    for path in [&profile.diary_path, &profile.plan_path] {
        if let Some(parent) = Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .filter(|p| !p.is_empty())
        {
            dirs.push(parent.to_string());
        }
    }
    dirs
}
