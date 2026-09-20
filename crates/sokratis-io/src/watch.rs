//! ZAŠTO RUST OVAKO (cigla M2/13 — watcher)
//! `notify` zove callback iz SVOJE niti; callback samo gura putanje u `mpsc` kanal, a NAŠA nit
//! (`spawn`) ih sažima (`recv_timeout` kao otkucaj) i tek onda šalje `WatchEvent`. `Arc<Mutex<_>>`
//! dijeli popis ruta i potisnutih putanja između niti — `Arc` je brojač vlasnika, `Mutex` red za
//! pisanje. `HashSet<i64>` u `RefreshQueue` čini „jedno čekanje po projektu" nemogućim za pokvariti.
use crate::IoError;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher as _};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

/// Razlog osvježenja: koji dio projekta se promijenio (S-016). `Manual` pokriva `.sokratis/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchReason {
    Git,
    Docs,
    Manual,
}

/// Događaj koji `Watcher` javlja kroz `mpsc`; DESKTOP (T30) ga prevodi u Tauri događaj.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatchEvent {
    pub project_id: i64,
    pub reason: WatchReason,
}

/// Nadzirana putanja i njezino značenje; `prefix` se uspoređuje sa svakim sirovim događajem preko
/// `starts_with` — kod preklapanja (npr. `.git` unutar `main_root`) pobjeđuje najduži prefiks.
#[derive(Debug, Clone)]
struct Route {
    project_id: i64,
    prefix: PathBuf,
    reason: WatchReason,
}

/// Dijeljeno stanje između `notify`-jeve niti (samo upisuje sirove putanje u `raw` kanal), naše
/// niti sažimanja (`debounce_loop`, čita rute i potiskivanja) i pozivatelja (`suppress` dodaje).
#[derive(Default)]
struct Shared {
    routes: Vec<Route>,
    suppressed: Vec<(PathBuf, Instant)>,
}

/// Uzima bravu; otrovan mutex (druga nit je pukla dok je brava bila zauzeta) ovdje NIJE fatalan —
/// stanje je samo popis putanja i rokova, nema invarijante koja bi ostala polupisana pucanjem
/// usred izmjene, pa se nastavlja s onim što je zapisano umjesto da cijeli watcher padne s njim.
/// `expect()` bi ovdje bio neopravdan: druga nit koja je pukla ne mora obarati cijeli proces.
fn lock_shared(shared: &Mutex<Shared>) -> MutexGuard<'_, Shared> {
    shared
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// `\\?\C:\…` (verbatim, Windows) i `C:\…` su ista putanja; usporedba za potiskivanje i
/// razvrstavanje ide po komponentama bez tog prefiksa, pa je `\` / `/` svejedno. NE zove se
/// `canonicalize`: cilj je često putanja koja još ne postoji (npr. `.sokratis` prije prvog upisa).
fn plain(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    PathBuf::from(s.strip_prefix(r"\\?\").unwrap_or(&s))
        .components()
        .collect()
}

/// Privremena datoteka koju `write_atomic` (`project.rs`) piše prije `rename` (`<ime>.<ext>.tmp`).
/// Potiskivanje mora pokriti i nju, ne samo konačnu putanju — Windows javi stvaranje `.tmp`
/// datoteke kao zaseban sirovi događaj, pa bi vlastiti upis inače ipak okinuo osvježavanje.
fn tmp_sibling(path: &Path) -> PathBuf {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => path.with_extension(format!("{ext}.tmp")),
        None => path.with_extension("tmp"),
    }
}

/// Je li sirovi `raw` događaj posljedica potisnutog upisa: točan pogodak, potisnuta privremena
/// `.tmp` datoteka, ili je `raw` PREDAK potisnute putanje. Zadnje pokriva rub kad `.sokratis` još
/// ne postoji: `watch_project` tada nadzire `main_root` nerekurzivno, pa je jedini sirovi događaj
/// koji stigne baš stvaranje same mape `.sokratis` (predak `overrides.json`/`visions.json`).
fn is_suppressed(raw: &Path, suppressed: &[(PathBuf, Instant)], now: Instant) -> bool {
    suppressed.iter().any(|(p, until)| {
        *until > now && (raw == p.as_path() || p.starts_with(raw) || raw == tmp_sibling(p))
    })
}

/// Gleda `.git`, dokumentaciju i `.sokratis` po projektu; javlja kroz `mpsc`, ne zna za Tauri.
pub struct Watcher {
    inner: RecommendedWatcher,
    shared: Arc<Mutex<Shared>>,
    watched: Vec<(i64, PathBuf)>,
}

impl Watcher {
    /// `tx` prima najviše po jedan `WatchEvent` po sažetom rafalu; `debounce` je odgoda tišine
    /// (S-016: 600 ms). Pokreće pozadinsku nit (`debounce_loop`) koja živi dok postoji barem jedan
    /// pošiljatelj sirovih putanja — taj se ugasi kad `notify`-jev watcher (`inner`) bude odbačen.
    pub fn new(tx: Sender<WatchEvent>, debounce: Duration) -> Result<Watcher, IoError> {
        let shared = Arc::new(Mutex::new(Shared::default()));
        let (raw_tx, raw_rx) = mpsc::channel::<PathBuf>();
        let inner = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(ev) = res {
                    for p in ev.paths {
                        let _ = raw_tx.send(p);
                    }
                }
            },
            notify::Config::default(),
        )
        .map_err(|e| IoError::Watch(e.to_string()))?;
        let shared_for_thread = Arc::clone(&shared);
        std::thread::spawn(move || debounce_loop(raw_rx, tx, shared_for_thread, debounce));
        Ok(Watcher {
            inner,
            shared,
            watched: Vec::new(),
        })
    }

    /// Registrira `.git` (zajednička mapa + `refs`/`logs`), dokumentaciju po radnom stablu i
    /// `.sokratis`: rekurzivno ako VEĆ postoji, inače `main_root` nerekurzivno kao zamjena dok se
    /// mapa prvi put ne pojavi (razlika se rješava filterom u `debounce_loop`, ne ovdje).
    pub fn watch_project(
        &mut self,
        project_id: i64,
        common_dir: &Path,
        worktrees: &[PathBuf],
        docs_rel: &[&str],
        main_root: &Path,
    ) -> Result<(), IoError> {
        let mut targets: Vec<(PathBuf, RecursiveMode, WatchReason)> = vec![
            (
                common_dir.to_path_buf(),
                RecursiveMode::NonRecursive,
                WatchReason::Git,
            ),
            (
                common_dir.join("refs"),
                RecursiveMode::Recursive,
                WatchReason::Git,
            ),
            (
                common_dir.join("logs"),
                RecursiveMode::Recursive,
                WatchReason::Git,
            ),
        ];
        for wt in worktrees {
            for rel in docs_rel {
                targets.push((wt.join(rel), RecursiveMode::Recursive, WatchReason::Docs));
            }
        }
        let manual = main_root.join(".sokratis");
        if manual.is_dir() {
            targets.push((manual, RecursiveMode::Recursive, WatchReason::Manual));
        } else {
            targets.push((
                main_root.to_path_buf(),
                RecursiveMode::NonRecursive,
                WatchReason::Manual,
            ));
        }
        for (path, mode, reason) in targets {
            if !path.exists() {
                continue;
            }
            self.inner
                .watch(&path, mode)
                .map_err(|e| IoError::Watch(format!("{}: {e}", path.display())))?;
            self.watched.push((project_id, path.clone()));
            lock_shared(&self.shared).routes.push(Route {
                project_id,
                prefix: plain(&path),
                reason,
            });
        }
        Ok(())
    }

    /// Prestaje nadzirati sve putanje registrirane za `project_id` (npr. projekt je uklonjen).
    pub fn unwatch_project(&mut self, project_id: i64) {
        for (_, path) in self.watched.iter().filter(|(id, _)| *id == project_id) {
            let _ = self.inner.unwatch(path);
        }
        self.watched.retain(|(id, _)| *id != project_id);
        lock_shared(&self.shared)
            .routes
            .retain(|r| r.project_id != project_id);
    }

    /// Vlastiti upis se ne smije vratiti kao događaj: `path` (i njegova `.tmp` privremena
    /// inačica, vidi `tmp_sibling`, plus svaki predak koji je tim upisom nastao) potisnuti su
    /// sljedećih `for_`.
    pub fn suppress(&self, path: &Path, for_: Duration) {
        lock_shared(&self.shared)
            .suppressed
            .push((plain(path), Instant::now() + for_));
    }
}

/// Prima sirove putanje iz `notify`-jeve niti, razvrstava ih po ruti u projekt+razlog, potiskuje
/// vlastite upise i šalje NAJVIŠE jedan `WatchEvent` po projektu nakon `debounce` tišine (svaki
/// novi sirovi događaj za isti projekt pomiče rok, pa rafal ne fura više poziva). Izlazi čim
/// `raw_rx` javi `Disconnected` — svi pošiljatelji su ugašeni jer je `Watcher` odbačen (S-016:
/// bez toga bi svaki test i svako uklanjanje projekta ostavili živu nit).
fn debounce_loop(
    raw_rx: Receiver<PathBuf>,
    tx: Sender<WatchEvent>,
    shared: Arc<Mutex<Shared>>,
    debounce: Duration,
) {
    let mut pending: HashMap<i64, (WatchReason, Instant)> = HashMap::new();
    loop {
        match raw_rx.recv_timeout(Duration::from_millis(50)) {
            Ok(raw) => {
                let raw = plain(&raw);
                let mut s = lock_shared(&shared);
                let now = Instant::now();
                s.suppressed.retain(|(_, until)| *until > now);
                let suppressed = is_suppressed(&raw, &s.suppressed, now);
                let route = s
                    .routes
                    .iter()
                    .filter(|r| raw.starts_with(&r.prefix))
                    .max_by_key(|r| r.prefix.components().count())
                    .cloned();
                drop(s);
                if suppressed {
                    continue;
                }
                if let Some(r) = route {
                    // `.sokratis/` pod glavnim stablom nadziremo i kroz nerekurzivan roditelj
                    // (kad mapa još ne postoji) — filtriraj SVE ostale promjene tog roditelja
                    // (npr. nova podmapa u repou) koje s `.sokratis` nemaju nikakve veze.
                    if r.reason == WatchReason::Manual
                        && !raw.components().any(|c| c.as_os_str() == ".sokratis")
                    {
                        continue;
                    }
                    pending.insert(r.project_id, (r.reason, now));
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
        }
        let now = Instant::now();
        let ready: Vec<i64> = pending
            .iter()
            .filter(|(_, (_, t))| now.duration_since(*t) >= debounce)
            .map(|(id, _)| *id)
            .collect();
        for id in ready {
            if let Some((reason, _)) = pending.remove(&id)
                && tx
                    .send(WatchEvent {
                        project_id: id,
                        reason,
                    })
                    .is_err()
            {
                return;
            }
        }
    }
}

/// Jamči „najviše jedan izračun po projektu odjednom": dok izračun teče, novi događaji se sažimaju
/// u JEDNO čekanje (ne u red više njih), pa se nakon završetka izračun ponovi točno jednom.
#[derive(Default, Debug)]
pub struct RefreshQueue {
    running: HashSet<i64>,
    pending: HashSet<i64>,
}

impl RefreshQueue {
    /// `true` = pokreni izračun sada; `false` = već teče, zapamćeno jedno čekanje.
    pub fn on_event(&mut self, project_id: i64) -> bool {
        if self.running.contains(&project_id) {
            self.pending.insert(project_id);
            false
        } else {
            self.running.insert(project_id);
            true
        }
    }

    /// Izračun za `project_id` je završio: `Some(id)` = čekanje je stiglo, ponovi odmah (projekt
    /// ostaje „running"); `None` = nije bilo čekanja, projekt više ne teče.
    pub fn on_done(&mut self, project_id: i64) -> Option<i64> {
        if self.pending.remove(&project_id) {
            Some(project_id)
        } else {
            self.running.remove(&project_id);
            None
        }
    }

    pub fn is_running(&self, project_id: i64) -> bool {
        self.running.contains(&project_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_runs_first_event_coalesces_the_rest_and_reruns_once() {
        let mut q = RefreshQueue::default();
        assert!(q.on_event(7), "prvi događaj pokreće");
        for _ in 0..10 {
            assert!(
                !q.on_event(7),
                "dok teče, događaji se sažimaju u jedno čekanje"
            );
        }
        assert!(q.is_running(7));
        assert_eq!(
            q.on_done(7),
            Some(7),
            "jedno čekanje = jedan ponovni izračun"
        );
        assert!(q.is_running(7), "ponovni izračun opet teče");
        assert_eq!(q.on_done(7), None);
        assert!(!q.is_running(7));
        assert!(q.on_event(8), "drugi projekt je neovisan");
    }
}
