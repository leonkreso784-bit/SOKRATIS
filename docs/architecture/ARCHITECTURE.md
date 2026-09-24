# ARCHITECTURE — što je izgrađeno

**Status:** ✅ opisuje kod koji je u `main`-u — Milestone 1 (verzija 0.1.0, uključujući krug popravaka
nakon završne recenzije) **plus svih devet tokova M2, svi gotovi i spojeni**: KOSTUR (`M2/1a`+
`M2/1b`: ugovor tipova, crate `sokratis-store`, `apps/desktop` s ikonama i `npm install`, desktop crate
u workspaceu) · JEZGRA (M2/2…M2/7: snapshot ugovora `Report`-a, `until` kao gornja granica, zbroj
vizija, redci commita/isporuke, aktivne faze preko `phase_tag`, `SnapshotMetrics`/`diff`/
`alerts_raised`) · PROFIL (M2/8, M2/9: `validate_paths`, `test_path_exclude`) · STORE (M2/15…M2/18:
registar, postavke, snimke, keš sirovih commita) · IO (M2/10…M2/14 + M2/14b izvan plana: atomarno
pisanje, manje git-procesa, detached HEAD, watcher, ograda putanja pri otvaranju — **dug I9 zatvoren u
cijelosti** — i potrošač keša) · CLI (M2/19: `report --until`) · SUČELJE (T20–M2/28: Svelte 5 +
Tailwind v4 nad `Report`-om, devet pogleda) · DESKTOP (M2/29a–c + T29–T33: `AppState` i jedanaest
naredbi, adapter keša, motor osvježavanja, splash, tray, autostart, jedna instanca) ·
INTEGRACIJA (T34, T36, T37, spojeno 2026-09-22, merge `b560f7c`): sučelje sad zove PRAVE naredbe
(`TauriApi`, ne `MockApi`), repo bez konvencija Sokrat Studyja daje brojke iz gita umjesto lažnih faza
(Ruling R21), instalater NSIS postoji, verzija ima jedan izvor (`1.0.0-pre.1`), razvojna baza je
odvojena od instalirane · **SUČELJE-2** (T38–T42, spojeno 2026-09-23, merge `6947189`): **deseti
pogled Postavke** (tema · jezik · autostart · animacije, S-028), **animacije** (S-026: `data-motion`
gasi ulaz grafova, prijelaz pogleda i kostur učitavanja preko jedne varijable), **kartica s
objašnjenjem** (S-027: 37 objašnjivih `id`-eva u svih devet pogleda i traci signala) — §1, §11.
**Time je etapa 2 „izgled" (S-025) cjelovita u kodu**; preostaje etapa 3 „izdanje".
**Drugi rez do 1.0.0** (S-032…S-037, aktivan spec [`../plan/ARHITEKTURA_1_0.md`](../plan/ARHITEKTURA_1_0.md))
je u izvedbi — sesija 1 (2026-09-24) spojila je tri toka: **DESKTOP-2** (T44–T45: `git_command()`
jedino mjesto `Command::new("git")` s `CREATE_NO_WINDOW` na Windowsu, kvar 4; X → upit → `quit`, tray
uklonjen, S-036), **JEZGRA-2** (T46–T47: `BranchScope`, `CommitRow.branch`, `Report.scope`/`branches`
sa satima po udjelu commita, S-032; `ReportInput.diaries` unija po datum+naslov, S-033) i **GRAFOVI**
(T53: četiri d3-ovisnosti pinane, `lib/charts/scales.ts`+`bucket.ts`, S-035). Sesija 3 (2026-09-24) je
zatim spojila **JEZGRA-2 T48** (`Profile.branch_scope`, zadano `"all"`), **GRAFOVI T54–T55** (temelji
`lib/charts/layout.ts` + `Chart`/`Axis`/`Grid`/`Legend`/`Tooltip`; `Bars`/`Line` prepisani na temelje,
`scale.ts` obrisan, d3 prvi put stvarno u snopu) i **IO-2 T49–T50** (`Scope::AllBranches`,
`commit_sources`, `cached_log(scope)` puni `scope`/kartu grana iz profila — `GitSource` sad ima **15**
metoda; `worktree_heads`/`commit_times`, vodeće stablo `Project::lead`, dnevnik kao unija SVIH radnih
stabala s diska, plan/`docs`/zadnja promjena datoteke iz vodećeg stabla) — §1, §3, §4, §11. Verzija u
kodu `1.0.0-pre.3`; sesija 4 nastavlja T51 (CLI `--scope`), T52 (mjerenje procesa/trajanja), T56–T57
(GRAFOVI kraj) i T58 (PLOČA — tok kreće).
Što od koda još stoji bez pokrića ili s poznatim rubom je u §11 ·
**Zadnja provjera:** 2026-09-24

> **Što ovaj dokument JEST:** opis sustava kakav stoji u `crates/` i `apps/` — granice između crateova, tok
> podataka, formati koje čita i ugovori prema korisniku CLI-ja. **Što NIJE:** kronologija (to su
> [records/CHANGELOG.md](../records/CHANGELOG.md) i [records/PROGRESS.md](../records/PROGRESS.md)),
> plan ([plan/ROADMAP.md](../plan/ROADMAP.md)) ni dom odluka
> ([records/DECISIONS.md](../records/DECISIONS.md), S-001…S-037). Specovi po kojima je M1 i M2 građen
> su arhivirani: [archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md) i
> [archive/ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md); aktivan spec (drugi rez do 1.0.0) je
> [plan/ARHITEKTURA_1_0.md](../plan/ARHITEKTURA_1_0.md).
>
> **Izvor istine je kod.** Gdje se dokument i kod razilaze, kod je u pravu, a dokument je propust.
> Zato uz svaku tvrdnju stoji datoteka u kojoj se provjerava.

---

## 1 · Četiri cratea i granica među njima

```
Cargo.toml                  # workspace; verzije ovisnosti na jednom mjestu
crates/
  sokratis-core/            # čisti Rust: model · parseri · metrike · docs-ocjena · pravila
  sokratis-io/              # git kroz proces · datoteke · profil · ručni podaci
  sokratis-store/           # SQLite (rusqlite) — od M2/1a: otvaranje baze i migracije, ništa više
  sokratis-cli/             # binarna `sokratis`: report · docs · signals
apps/desktop/               # ljuska M2: Svelte/Vite datoteke + crate `sokratis-desktop` (src-tauri)
```

| crate | smije | ne smije | ulaz u kod |
|---|---|---|---|
| `sokratis-core` | računati nad tekstom i strukturama | otvoriti datoteku, pokrenuti proces, pitati koliko je sati | `src/lib.rs` |
| `sokratis-io` | `std::process::Command` za `git`, `std::fs`, `chrono::Local` za današnji datum | računati metrike | `src/git.rs`, `src/project.rs` |
| `sokratis-store` | otvoriti SQLite bazu i primijeniti migracije; ovisi o `core` (tipovi), **ne o `io`** (S-013) | računati metrike, dirati git | `src/lib.rs`, `src/store.rs` |
| `sokratis-cli` | ispisati JSON ili tablicu i vratiti izlazni kod | računati bilo što | `src/main.rs`, `src/table.rs` |
| `sokratis-desktop` (`apps/desktop/src-tauri`) | pokrenuti Tauri ljusku: prozori, plugini, naredbe koje posuđuju jezgru/`io`/`store` | držati logiku koja bi se htjela testirati (S-013) | `src/lib.rs`, `src/commands.rs`, `src/engine.rs` |

**Što `store` danas radi:** `Store::open(path)` / `Store::open_in_memory()` otvore vezu i primijene
migraciju `src/migrations/0001_init.sql` (sedam tablica: `project` · `project_worktree` · `setting` ·
`project_setting` · `profile_seen` · `snapshot` · `commit_cache`). Iznad toga rade četiri „posla", svaki
u vlastitoj datoteci (S-010: jedna cjelina, jedno mjesto):

| datoteka | što drži | ključni obrazac |
|---|---|---|
| `registry.rs` | registar projekata i radnih stabala; identitet projekta je `git_common_dir`, ne putanja (S-015) — dva radna stabla istog repozitorija dijele jedan zapis | `add_project`/`list_projects`/`rename_project`/`remove_project`/`touch_project`/`set_worktrees` |
| `settings.rs` | globalne postavke i postavke po projektu (tema, jezik, autostart, animacije, raspon, zadnji pogled) | SQL upsert `INSERT … ON CONFLICT … DO UPDATE` |
| `snapshots.rs` | dnevna snimka 18 pokazatelja + docs-ocjene + broja signala, profil kao kanonski JSON (S-014), trend | `save_snapshot` je CJELOVITA ZAMJENA dana (`DELETE` pa `INSERT` u jednoj transakciji) |
| `cache.rs` | sirovi commiti po SHA — **nikad klasifikacija** (S-014) | `INSERT OR IGNORE` u jednoj transakciji, `newest_cached_commit_date` za inkrementalno dovlačenje |

Svaki posao ima jedinične testove nad `:memory:` bazom. **Od DESKTOP-a (T29, 2026-09-21) `sokratis-store`
ima pravog pozivatelja** — `AppState.store` (`Mutex<Store>`), otvorena na `state::db_path()`
(§1 niže). **Od T37 (2026-09-22) putanja baze ovisi o gradnji** — `state::db_file_name(cfg!(debug_assertions))`
vraća `sokratis-dev.db` za `npm run tauri dev` (debug) i `sokratis.db` za instaliranu aplikaciju
(release), oboje u `%LOCALAPPDATA%\sokratis\`; razvoj time ne dira Leonovu pravu bazu (S-029). **CLI
i dalje ne ovisi o `sokratis-store`** i bazu ne stvara.

**`sokratis-desktop` je u `[workspace] members` i builda se** (`M2/1b`, 2026-09-18): `npm install`
uz Leonov OK (80 paketa, 0 ranjivosti) i `npm run tauri icon` iz Leonova loga popunili su
`src-tauri/icons/`, pa `tauri-build` ima što tražiti.

**Od DESKTOP-a (M2/29a–c + T29–T33, spojeno 2026-09-21, merge `cc74bb8`) ljuska ima pravu Rust
stranu** — S-013 i dalje vrijedi (crate ne drži logiku koju bi vrijedilo testirati, samo posuđuje
jezgru/`io`/`store`):

| datoteka | uloga |
|---|---|
| `state.rs` | `AppState` (`store`, `reports` — zadnji izračun po projektu, `watcher`, red čekanja `queue`, primatelj događaja `rx`; svih pet iza `Mutex`, jer svaki `invoke` iz sučelja stiže na svojoj niti); `db_path()` (`%LOCALAPPDATA%\sokratis\` + `db_file_name(cfg!(debug_assertions))`, rezerva u privremenu mapu ako profil ne postoji); `db_file_name` (T37, M2/37) — `sokratis-dev.db` u debug gradnji (`tauri dev`), `sokratis.db` u release (instalirana); `text()` — jedino mjesto koje bilo koju grešku (`IoError`, `StoreError`, otrovan `Mutex`) pretvara u tekst za IPC |
| `cache.rs` | `StoreCache` — JEDINA implementacija `sokratis_io::CommitCache` nad `sokratis-store` (S-013: `io` i dalje ne zna za `store`); zaključava `Mutex<Store>` kratko, po pozivu — pozivatelj (`compute`) NE SMIJE držati bravu preko `Project::input_cached`, std `Mutex` nije reentrantan |
| `summary.rs` | `ProjectSummary`/`LastCommit` — sažetak jednog projekta za Pregled (S-012: preslagivanje `Report`-a, nikakvo novo mjerenje); zadnji commit je najnoviji po `author_time`, ne zadnji u nizu (`Report.commits` ne jamči poredak) |
| `commands.rs` | **dvanaest** `#[tauri::command]` (tablica niže, +`quit` od M2/45), `Range` (birač raspona), `Settings`, zajednički `compute`/`compute_input` (izračun kroz keš; na `IoError::Cache`/`CacheIncomplete` javi razlog na stderr i ponovi BEZ keša — keš je pogodnost, ne istina, S-014) |
| `engine.rs` | motor osvježavanja (opis niže) |
| `splash.rs` | `SplashState` — tri `AtomicBool` (animacija gotova, prvi izračun gotov, već prikazano), rezerva od 10 s ako `splash:done` ne stigne |
| `autostart.rs` (M2/45, S-036) | `set_autostart` — plugin PA baza, tim redom (baza se mijenja SAMO ako plugin uspije); preseljen iz `tray.rs` (uklonjen), jedini pozivatelj je `commands::set_setting` |
| `lib.rs` | `tauri::Builder` — redoslijed plugina (`single-instance` prvi), `manage(AppState)`, `generate_handler!`; **od M2/45 (S-036) X NE zatvara ni skriva prozor sam** — `on_window_event` zove `api.prevent_close()` pa `window.emit("close_requested", ())` (trait `tauri::Emitter`), sučelje pokaže upit i na potvrdu zove naredbu `quit`; `setup` (splash naoružan → motor kreće, **tray-a više nema**) |

**Motor osvježavanja** (`engine.rs`, S-016 + S-020) — **dva** izvora istog zahtjeva (tray uklonjen
M2/45, S-036, bio je treći): watcher nad `.git`/docs/`.sokratis` (600 ms odgoda) · naredba
(`refresh`/`set_override`/`save_visions`) **oba idu kroz `request_refresh(app, id)`** — JEDAN red čekanja po projektu
(`RefreshQueue`, `sokratis-io`): dok jedan izračun projekta traje, novi zahtjev ZAMJENJUJE čekanje
umjesto da uđe u red (spec §3.3 t. 3 — naredba i watcher tako nikad ne računaju isti projekt
istodobno). `refresh_project` zatim: `compute` (UVIJEK `Range::All`, nikad kraći raspon — kraći bi
zaprljao `reports[id]`, koji služi kao „prošli" izvještaj za usporedbu signala) → dnevna snimka
(best-effort, piše se PRI SVAKOM izračunu, ne samo prvom — S-014: snimka dana je stanje ZADNJEG
izračuna) → `reports.insert` (vraća STARI izvještaj) → događaj `report_updated { project_id }` → bez
prijašnjeg izvještaja (prvo pokretanje) nema provjere prijelaza; inače `alerts_raised(&prev.signals,
&cur.signals)` (jezgra, M2/29c, deterministična) — po svakom NOVOM Alertu: `signal_raised {
project_id, rule, severity }` + obavijest OS-a (naslov = ime projekta, tijelo = natpis pravila +
prvi redak dokaza).

**Ugovor prema sučelju** (spec §5, `Report` nepromijenjen — S-012):

| naredba | vraća | napomena |
|---|---|---|
| `list_projects` | `Vec<ProjectSummary>` | |
| `add_project` | `Option<ProjectSummary>` | `None` ako korisnik odustane od dijaloga; upisuje GLAVNO stablo (`main_root`), ne sporedno iz kojeg je dijalog otvoren (S-015) |
| `rename_project` / `remove_project` | `()` | |
| `get_report(id, range: Range)` | `Report` | NE piše u `reports` — to radi SAMO motor |
| `get_trend(id, metric, range: Range)` | `Vec<TrendPoint>` | `None` granice → `"0000-01-01"`/`"9999-12-31"` |
| `set_override` / `save_visions` | `()` | potisni watcher PRIJE upisa → piši → ponovno nadziri → `request_refresh` |
| `refresh(id: Option<i64>)` | `()` | `Some` → jedan projekt kroz `request_refresh`; `None` → `refresh_all` (isti red čekanja, jedna petlja — Topbar je zove bez `id` kad nema odabranog projekta; tray koji je nekoć zvao istu petlju je uklonjen, M2/45) |
| `get_settings` / `set_setting(key, value: String)` | `Settings` / `()` | `autostart` ide kroz `autostart::set_autostart` (plugin PA baza, M2/45); ostale postavke ravno u bazu |
| `quit` (M2/45, S-036) | `()` | `app.exit(0)`; jedini pozivatelj je sučelje, na potvrdu upita „Zatvoriti Sokratis?" nakon događaja `close_requested` (X na prozoru) |

`Range` (`#[serde(tag = "preset", rename_all = "snake_case")]`): `{"preset":"all"}` ·
`{"preset":"7d"}` · `{"preset":"30d"}` · `{"preset":"month"}` ·
`{"preset":"custom","since":…,"until":…}` — `to_dates(today)` vraća `(Option<String>,
Option<String>)` bez `chrono` u desktopu (S-013, kroz `sokratis_core::civil::prev_day`).
`Settings { theme, lang, autostart: bool, motion: bool }` (četvrto polje `motion`, M2/38, S-028) —
baza drži i autostart i pokret kao tekst (`"on"`/`"off"`), `SETTING_KEYS: [&str; 4]`; naredba
`set_setting` i dalje prima tekst (TS strana `TauriApi.setSetting` prevodi SVAKI `boolean` u
`"on"`/`"off"` po TIPU vrijednosti, ne po imenu ključa, T34+M2/38).

`cargo test --workspace` dotiče crate s 2 testa (`Range::to_dates`, jedini test koji S-013 dopušta).
Dokaz da se ljuska stvarno pokreće je **dimni test** (`npm run tauri dev`, ručna provjera opisana u
[`TESTING.md`](../workflow/TESTING.md) §5), ne `cargo test`.

**`apps/desktop/src` (SUČELJE, T20–M2/28 + T34 + SUČELJE-2 T38–T42, spojeno 2026-09-21/23) je Svelte 5
+ Tailwind v4 nad `Report`-om — ne računa ništa, samo prikazuje (S-012):**

| dio | što radi |
|---|---|
| `styles/tokens.css` | četiri teme (zadana „Akademsko plavo"), `brand-*` izmjeren iz Leonova loga (S-017); `scripts/check-contrast.mjs` brani kontrast na sve četiri |
| `styles/motion.css` | **jedino mjesto pokreta sučelja** (S-026): `--motion-dur: 250ms`, `:root[data-motion="off"]` svodi svaku `animation-`/`transition-duration` na `0s !important`; `lib/motion.ts::motionOff` je čista odluka bez DOM-a, `state.svelte.ts::syncMotion` je upisuje kao `data-motion` na `<html>` — isti obrazac kao `data-theme` |
| `lib/i18n/` (`hr.json`, `en.json`, `t.ts`, `index.svelte.ts`) | HR/EN rječnik (S-021); `scripts/check-i18n.mjs` brani da oba jezika imaju isti skup ključeva |
| `lib/format.ts` | jedino mjesto oblikovanja brojki, datuma i postotaka za sučelje |
| `lib/charts/` (`layout.ts`, `scales.ts`, `bucket.ts`, `Chart`/`Axis`/`Grid`/`Legend`/`Tooltip`/`Bars`/`Line`/`Ring`/`Sparkline`) | vlastiti SVG grafovi (S-018) s d3-matematikom za osi (S-035 dopunjena, Leonov OK: `d3-scale`/`d3-shape`/`d3-array`/`d3-time-format`, UTC datumski ticksi, grupiranje po danu/tjednu/mjesecu); izgled i boje ostaju naši; ulaze animirano (M2/39, §11 niže). **T54** (2026-09-24) donio temelje — `layout.ts` (okvir/margine, grupirani/naslagani stupci nad `scaleBand`, X-ticksi prorijeđeni na ≤ 10, linija s datumskom skalom, najbliža točka za tooltip) + pet tankih komponenata `Chart`/`Axis`/`Grid`/`Legend`/`Tooltip`. **T55** prepisao `Bars`/`Line` na te temelje (datumi na X-osi, mreža, legenda, tooltip miš+tipkovnica) i `Ring`/`Sparkline` na `scales.ts`; stari `scale.ts` **obrisan** — `linear`/`finiteMax`/`arcPath`/`ringSegments`/`svgA11y` sele u `scales.ts`. `main.js` sad stvarno uvozi d3 (258,67 kB, `CHANGELOG.md`). Prekidač dan/tjedan/mjesec dolazi u T59 |
| `lib/explain/` (`ids.ts`, `explain.svelte.ts`, `Explainable.svelte`, `ExplainCard.svelte`) | **kartica s objašnjenjem** (S-027, M2/41–42): `EXPLAIN_IDS` (37) + `explainKeys(id)` → ključevi `explain.<id>.what\|how\|read`; `Explainable` je okidač (`<button aria-haspopup="dialog">`), `ExplainCard` (`role="dialog" aria-modal="false"`) prikazuje tekst; test pokrivenosti veže popis na OBA rječnika u oba smjera i na 18 id-eva pokazatelja iz prave snimke jezgre |
| `splash/` (`Splash.svelte`, `intro.ts`) | animacija pokretanja, 4,2 s, preskočiva (S-019); zaseban Vite-ulaz `splash.html` |
| `lib/api.ts` | sučelje `Api`; `MockApi` čita insta snapshot jezgre kroz Viteov `?raw` uvoz — dev-prikaz i vitest vide TOČNO brojke koje bi jezgra izračunala, ne ručno prepisanu kopiju (S-010); `TauriApi` (T34) svaku metodu prevodi u `invoke('<naredba>', {…})` prema `commands.rs`, `onReportUpdated`/`onSignalRaised` idu preko `listen()`; `createApi()` bira izvedbu po `'__TAURI_INTERNALS__' in window`; **od T45 (M2/45, S-036) += `quit()`/`onCloseRequested(cb)`** — `TauriApi` zove naredbu `quit` i sluša događaj `close_requested`, `MockApi` oboje nema-op (preglednik nema proces za ugasiti) |
| `lib/shell/ConfirmQuit.svelte` (T45, S-036) | upit „Zatvoriti Sokratis?" nakon `close_requested`; `role="dialog" aria-modal="true"`, fokus na „Odustani", Esc = odustani; NE testira se vitestom (nema jsdom-a, isti razlog kao T39/R37) — ponašanje dokazuje dimni test |
| `lib/types.ts` | TS zrcalo `Report`-a; `tests/types.test.ts` ga veže na isti insta snapshot da se oblik ne razmine s Rustom |
| `lib/state.svelte.ts` | Svelte 5 rune (`$state`) drže odabrani projekt, raspon, postavke, izvještaj, `epoch` (0 → 1 jednom kad glavni prozor prvi put postane vidljiv, M2/39) |
| `views/` (**deset** pogleda) + `views/helpers.ts` | Pregled, Tempo, Vrste rada, Pokazatelji, Faze, Dnevnik, Isporuke, Vizije, Dokumentacija — svih devet traže odabran projekt — i **Postavke** (M2/38, S-028): tema · jezik · autostart · animacije, jedini pogled koji NE čita `app.report`, radi i bez projekta; `helpers.ts` drži čiste funkcije testirane odvojeno od komponenata (sortiranje, boja po vrsti/težini, i18n-množina, kopiranje i spajanje putanje, zamjena vizije po indeksu) |

Dnevnik uređuje vrstu rada po commitu (`setOverride`, oznaka „ručno"); Vizije se dodaju, uređuju i
brišu u mjestu istim obrascem (Svelte 5 `{#snippet}`, `saveVisions` uvijek šalje cijeli popis, ne
samo izmijenjeni redak). **Od T34 (2026-09-22) `createApi()` bira `TauriApi` u pravom Tauri prozoru**
(`MockApi` ostaje za preglednik, `npm run dev`, i za vitest) — sučelje sad zove prave naredbe iz
DESKTOP-a (gore). Pretplata na `report_updated` je JEDNA globalna, u `App.svelte`: osvježi popis
projekata i, ako je odabrani projekt taj koji se promijenio, i trenutačni izvještaj. Svaki poziv
`api.*` hvata grešku i piše je u `app.error` (traka greške, `role="alert"`); `loadReport` čuva
`reportRequestToken`, pa zastarjeli odgovor (uspjeh ili greška) ne dira stanje.
**Od SUČELJE-2 (T38–T42, 2026-09-23) prva ulazna animacija čeka da korisnik prozor stvarno vidi:**
`{#key \`${app.epoch}:${app.view}\`}` oko lanca pogleda tjera Svelte da ih demontira/remontira kad
`epoch` poraste 0→1; okidač je `getCurrentWindow().onFocusChanged` (fokus stiže iz `splash.rs`), NE
`document.visibilitychange` — WebView2 javlja prozor vidljivim i dok je iza splasha skriven, pa taj
događaj nikad ne okine (Ruling R25, izmjereno u `tauri dev`). Brane su `npm run
check` (`svelte-check` + `check:i18n` + `check:contrast` + `vitest`) i `npm run build` u
`apps/desktop`, ne `cargo` ([`TESTING.md`](../workflow/TESTING.md) §1).

**Što `io` danas radi** (`crates/sokratis-io/src/`): `Project::open` čita `.sokratis/profile.json`
(ili `Profile::default()`) i odmah zove `validate_paths()` — putanja izvan repoa je
`IoError::ProfileInvalid` prije nego se ijedna putanja pročita s diska (M2/14, dug I9 zatvoren u
cijelosti). `write_override`/`write_visions` pišu kroz `.tmp` pa `rename` (atomarno) u GLAVNO stablo
(`main_root`, ne radno stablo iz kojeg je pozvano — S-015). `input()`/`input_between()` traže manje
git-procesa po izvještaju nego prije M2/11 (dug M11; brojke: `CHANGELOG.md`, ne ovdje — S-010).
Detached HEAD (grana ne postoji, commit postoji) daje `Report.branch = "HEAD@<sha>"` umjesto lažnog
„nema commita" (M2/12). `common_dir` prolazi kroz `normalized()` (M2/29b, isti obrazac kao
`profile_path`, I5) — dva radna stabla istog repozitorija davala su tekstualno različit `common_dir`
(veliko/malo slovo diska, završni `\`), pa je registar (`sokratis-store`, S-015) prije vidio dva
projekta umjesto jednog. `today()` (M2/29b) je jedino mjesto koje računa današnji lokalni datum
(`chrono::Local`) izvan `Project::input`; DESKTOP ga zove za zadani raspon birača i ključ dnevne
snimke, bez vlastite ovisnosti o `chrono` (S-013). `Watcher` (`watch.rs`) gleda `.git`, `docs_dir` i
`.sokratis`, javlja najviše jedan `WatchEvent` po projektu 600 ms nakon zadnje promjene, potiskuje
vlastite upise i serijalizira preklapajuće izračune (`RefreshQueue`, S-016) — **od DESKTOP-a (T30,
`engine.rs`) ima pravog pozivatelja**: nit motora čita `WatchEvent` iz `rx` i zove `request_refresh`.
`window_args` (`git.rs`) gradi prozor s rezervom zone (dan unatrag za `since`, dva dana unaprijed za
`until`) i šalje ga i `log`-u i `rev_list`-u istom funkcijom, da se dva prozora ne mogu razići.
**Potrošač keša** (M2/14b): `Project::input_cached` poziva `cached_log` (`cache.rs`) —
`GitSource::rev_list` kaže što je DOSTIŽNO SADA, trait `CommitCache` (ugovor bez ovisnosti o
`sokratis-store`, S-013) vraća poznate commite, `git log --no-walk --stdin` (`log_commits`) dovlači
SAMO nedostajuće; `commit --amend`/`reset --hard` time ne ostavljaju stari SHA u brojkama iako ostaju
u kešu. Greška keša je vidljiva (`IoError::Cache`/`CacheIncomplete`), nema tihog povratka na puni log.
**`input_cached` sad ima pozivatelja** — adapter `StoreCache` (`desktop/src-tauri/src/cache.rs`, T29)
ga ožiči nad `sokratis-store`, `commands.rs::compute_input` ga zove za svaki izračun u desktopu. CLI
(`report_for`) i dalje zove `input_between`, BEZ keša (§11). **Od IO-2 (T49, S-032) `cached_log` prima
`scope: Scope<'_>`** — isti enum kao `log`/`rev_list` (`Scope::Branch(ime)` ili `Scope::AllBranches`,
lifetime posuđuje ime grane) — pa keš vrijedi i za mjerenje preko svih grana, ne samo zadane.

**Grane i radna stabla ulaze u mjerenje** (T49–T50, S-032, S-033): `input_with` čita
`profile.branch_scope` i bira `Scope` — `"all"` (zadano) šalje `git log --branches` i puni kartu
`GitSource::commit_sources(default_ref, since, until)` (`sha → grana` SAMO za commite izvan zadane
grane, preko `git log --branches --not <default_ref> --format=%h|%S`; `"default"` vraća staro
ponašanje i praznu kartu). `GitSource::worktree_heads()` (puni SHA po radnom stablu) i
`commit_times(shas)` daju `Project::lead()` — **vodeće stablo** je ono s najnovijim `author_time` na
HEAD-u, ne nužno glavno; `worktrees_by_recency()` sortira sva stabla silazno po tome
(`sort_by_key` + `Reverse`, stabilno). Dnevnik je unija tekstova SA SVAKOG stabla, čitanih izravno s
DISKA (necommitani unos vidljiv), redom od vodećeg prema starijima; plan, `docs()` (`docs_in(&lead.
root, &lead.head)`) i `GitSource::last_changes(rev, pathspecs)` čitaju SAMO iz vodećeg stabla (`rev`
je `"HEAD"` za glavno stablo ili puni SHA vodećeg radnog stabla — detached stablo tako ipak ima
povijest). Ručni podaci se i dalje pišu SAMO u glavno stablo (S-015, nepromijenjeno).

**Granica S-002:** jezgra ne zna odakle su podaci došli. Sve što joj treba dolazi u jednoj strukturi
(`ReportInput`: `git_log` kao tekst, `diaries: Vec<String>` — **od M2/47 (S-033) više od jednog
teksta**, jedan po radnom stablu od vodećeg prema starijima, bilo `diary: Option<String>` — i plan kao
tekst, docs, grane, ručni podaci, `now`, `today`, `since`, `until`, `branch`, `scope: BranchScope` i
`commit_branches: HashMap<String, String>` — **od M2/46 (S-032)**, karta `sha → grana` SAMO za
commite izvan zadane grane, prazna kad je `scope == DefaultBranch` — te `worktrees: u32`) i vraća se
jedna struktura (`Report`). Zato se jezgra testira bez gita, i zato će je Tauri u M2 koristiti bez
ijedne izmjene (S-012).

**Granica S-003:** git se čita **kroz proces**, ne kroz biblioteku. Poziv se gradi bez shella, kroz
`git_command(repo)` (`git.rs`, M2/44, kvar 4) — **jedino mjesto** koje smije zvati `Command::new("git")`
(test `command_new_lives_only_inside_git_command` to čuva čitanjem izvora); na Windowsu ista funkcija
postavlja `CREATE_NO_WINDOW`, jer bi instalirana aplikacija (`windows_subsystem = "windows"`) inače
bljesnula konzolu `git.exe` pri svakom osvježenju. Iza traita `GitSource` (**15** metoda — T49 dodao
`commit_sources`, T50 `worktree_heads`/`commit_times` i novi parametar `rev` na `last_changes`; jedna
implementacija `GitCli`) stoji mjesto na koje kasnije može ući `gix` bez dizanja jezgre.

## 2 · Tok podataka

```
putanja repoa
  → io:  Project::open        .sokratis/profile.json ili Profile::default()   (S-005)
         Project::input        git log · dnevnik · plan · docs · grane · overridei · vizije
  → core: build_report         parse → metrike → docs-ocjena → pravila → Report
  → cli:  JSON | tablica | izlazni kod
```

Korak po korak, redoslijed je u `core/src/report.rs` (`build_report`) i nigdje drugdje:

1. **provjera ulaza:** `Patterns::compile` kompilira regexe iz profila jednom po izvještaju i traži
   da imaju grupe koje parser čita (`diary_heading` 3 · `plan_brick` 3 · `plan_phase_name` 2 ·
   `phase_tag` 1) → `ParseError::BadPattern`; `Profile::validate_dates` i provjera `input.since`
   traže oblik `YYYY-MM-DD` → `ParseError::BadDate { field, text }`. Valjan JSON s regexom bez grupe
   ili s tipfelerom u datumu je **greška s imenom polja** (izlaz 3), ne panika i ne tiha kriva brojka.
   Uz njih se od `M2/1a` zove i `Profile::validate_paths` (ograda putanja iz profila na korijen
   repoa): `inside_root` razlaže putanju kroz `std::path::Component` i s `matches!` odbija sve osim
   `Normal`/`CurDir` (dakle `..`, apsolutnu putanju, `C:\…`, `\\server\share`), bez ijednog diranja
   diska (S-002) → `ParseError::PathOutsideRoot { field, value }` imenuje prvo pogođeno od osam
   polja (M2/8, jezgreni dio duga I9). Io-dio ograde — `io::Project::open` zove `validate_paths()`
   odmah nakon učitavanja profila, prije nego se ijedna putanja pročita s diska (M2/14) — **dug I9 je
   time zatvoren u cijelosti** (§1).
2. `parse_git_log` — tekst loga → `Vec<Commit>` + broj preskočenih redaka.
3. filtar `since`: ostaju commiti s `commit_date >= since` (S-011 — isti kriterij kao `git log --since`).
4. `parse_diary` — naslovi dnevnika → `Vec<Delivery>`; `parse_plan` — redovi plana → faze.
5. `hours_per_day` — sati po danu (sortirano po `author_time`, S-007).
6. `commit_rows` — klasificira svaki commit (vrsta uz override, podvrsta) u redak za `Report.commits`
   (M2/5); `day_stats` — redak po danu s commitom; `kind_stats` broji **iz tih redaka** (posuđenih
   prije nego se pomaknu u `Report`), umjesto da klasifikaciju ponovi.
7. faze: zatvorene se **broje** iz cijelog loga; aktivne se na filtrirane commite vežu regexom
   `phase_tag` iz profila (M2/6, dug I3+M14 riješen), ne više tvrdim prefiksom `"{id}/"`. Zatvorena
   faza **bez ijednog pogođenog commita** ne ulazi ni u pokazatelje ni u tablicu — zatvorene faze
   dolaze iz profila, pa bi tuđi projekt sa zadanim profilom (S-005) dobio faze iz zraka.
8. `indicators` — 18 pokazatelja, svaki s `kind` (`measure` ili `proxy`) i formulom. `IndicatorInput`
   nosi i `since` (iz `ReportInput`, dakle `--since`): pokazatelj o zatvorenim fazama mjeri isto
   razdoblje kao ostatak izvještaja, ne `profile.since`. **Nedovršeno (§11):** nekoliko pokazatelja
   ovdje zove `effective_kind` odvojeno od koraka 6, pa se commit klasificira više od jednom po
   izvještaju — spec §3.2 traži jednom.
9. `docs_health` — ocjena i nalazi; `evaluate_all(default_rules())` — signali.
10. `Report` se sastavi i serializira (`serde`); `vision_totals` (zbroj vizija po stanju, M2/4) se
    računa u istom koraku iz `input.visions`.

**Grana:** metrike se računaju nad `profile.default_branch` ako ta grana postoji, inače nad trenutnom
granom (`io/src/project.rs::input`). Ostale grane ulaze **samo u signale**. Ako ni jedna ni druga ne
postoji kao referenca (repo nakon `git init`, bez commita), `io` vraća `IoError::NoCommits` →
`<putanja>: repozitorij nema commita` i izlaz 3; prije je korisnik dobivao gitov savjet o `--`.

## 3 · Što `Report` nosi

`core/src/model.rs`. Sedamnaest polja (**M2/46** dodala `scope`, `branches`): `generated_at` ·
`since` · `until` · `branch` · `scope` · `touched` · `days` · `kinds` · `branches` · `commits` ·
`deliveries` · `indicators` · `phases` · `visions` · `vision_totals` · `docs` (`null` kad projekt
nema mapu s dokumentacijom — nula bi bila laž) · `signals`.

**Grane (M2/46, S-032, spec 1.0.0):** `scope: BranchScope` (`"all"`/`"default"`, `#[serde(rename)]`
na varijanti; zadano `AllBranches` kroz `#[default]`) kaže koje je grane `io` obišao pri mjerenju;
`branches: Vec<BranchStats>` je sekcija Grane — jedan redak po grani (`name`, `commits`, `lines`,
`hours`, `merged`), zadana grana prva, sortirano po broju commita silazno pa po imenu; `hours` su
sati DANA (`DayStats.hours`) podijeljeni po udjelu commita te grane tog dana — **proxy**, zbroj po
granama je jednak ukupnim satima (`metrics::branch_stats`). `CommitRow.branch` (zadnje polje u
strukturi, dodatak na kraju — S-022, snapshot-diff ostaje malen) bilježi granu na koju je commit
dospio; commit koji nije u `ReportInput.commit_branches` dobiva zadanu granu. **Od IO-2 (T49, S-032)
`io` čita `profile.branch_scope`** i stvarno šalje `scope: AllBranches` (zadano) s popunjenom kartom
`commit_branches`, ili `scope: DefaultBranch` s praznom kartom ako profil to traži — mjerenje po
zadanom profilu je od `1.0.0-pre.3` preko SVIH lokalnih grana, ne samo zadane. **CLI-tablica
(`table.rs`) `scope`/`branches` i dalje ne ispisuje** (dolazi s T51); sučelje ih isto još ne crta
(§11).

**Tri polja koja je `M2/1a` deklarirala prazna sad jezgra puni** (M2/4, M2/5) — deklarirana su prije
potrošača da sučelje i snapshot ugovora (S-022) ne mijenjaju oblik svakom ciglom:

| polje | što nosi | puni ga |
|---|---|---|
| `commits` | redak po commitu (`CommitRow`: `sha` · `date` · `author_time` · `subject` · `kind` · `sub` · `overridden`) — ulaz za pogled Dnevnik i za Pregled (`author_time`, M2/29a, namjerna snapshot-izmjena: Pregled treba pravi poredak „najnoviji prvo", koji `Report.commits` inače ne jamči) | M2/5, dopuna M2/29a |
| `deliveries` | redak po isporuci (`Delivery`: `date` · `model` · `title` · `kind` · `deploy`); M1 je isporuke iz dnevnika samo zbrajao po danu, sad postoji i popis — ulaz za budući pogled Isporuke | M2/5 |
| `vision_totals` | zbroj vizija po stanju (`VisionTotal { state, count }`, dug I6) | M2/4 |

**CLI-tablica (`cli/src/table.rs`) ova tri polja još ne ispisuje** — korisnik CLI-ja zato ne vidi
ništa novo; podaci postoje u JSON-u i čekaju sučelje M2 (§11).

**`until` (M2/3) je gornja granica razdoblja, istog oblika kao `since` — i jezgra i njezini pozivatelji
je danas šalju.** `ReportInput.until: Option<String>` prolazi istu provjeru oblika kao `since`
(`ParseError::BadDate { field: "until", .. }`, let-chain u `build_report`) i filtrira commite i
isporuke tako da ostane samo `commit_date`/`date <= until` (gornja granica uključuje cijeli dan,
S-011). `civil::next_day` je rezerva zone, zrcalo `prev_day`-a. `io` šalje stvarnu vrijednost od M2/14
(`Project::input_between`, `GitSource::log`/`rev_list` dobivaju `until`), CLI od M2/19 (`sokratis
report --until YYYY-MM-DD`, §9); `docs` i `signals` i dalje šalju `None`. Snapshot ugovora
(`snapshot.rs`) i dalje ima `until: null` jer fixture pariteta ne prosljeđuje `--until` — svojstvo
fixturea, ne jezgre.

**`core/src/snapshot.rs` više nije prazan modul** (M2/7): `SnapshotMetrics::from_report`, `diff`,
`SignalCounts::from_signals`, `worst_severity`, `alerts_raised` postoje i imaju testove — pune se
tipovi `SignalCounts`, `MetricValue { id, value, kind }`, `SnapshotMetrics`, `MetricDelta` iz
`model.rs`. `sokratis-store::save_snapshot`/`latest_snapshot`/`trend` (M2/17) znaju spremiti i čitati
ove tipove i imaju teste, ali ih ništa u `main`-u ne zove (§11); obavijest na prijelaz u Alert
(DESKTOP T29/T30) također nema pozivatelja.

**`touched` je mjerač mjerača** — koliko je izvještaj stvarno dotaknuo (`core/src/report.rs`):

| polje | što je | pažnja |
|---|---|---|
| `commits` | broj commita nakon filtra `since` | |
| `lines` | Σ `added + deleted` po svim izmjenama datoteka | |
| `files` | **broj izmjena datoteka kroz commite**, ne broj različitih datoteka | datoteka dirnuta u 10 commita doda 10 |
| `skipped_lines` | redaka `numstat`-a koje parser nije razumio | preskočeno se broji, nikad tiho ne ispari |
| `worktrees` (M2/47, S-033) | broj radnih stabala koje je `io` obišao | od T50 = stvaran broj radnih stabala (`worktree_heads`), ne uvijek `1` |
| `diaries` (M2/47, S-033) | broj tekstova dnevnika koje je `parse_diaries` unirao (dužina `ReportInput.diaries`) | od T50 = broj radnih stabala s diska, isti razlog |

## 4 · Profil projekta — sva polja i zadane vrijednosti

**Izvor je `crates/sokratis-core/src/profile.rs`** (`impl Default for Profile`); ova tablica prati
njega i ima jednako redaka koliko struktura ima polja (**40** — `M2/1a` je imala 39, T48 dodao
`branch_scope`). Zadane vrijednosti **jesu**
konvencije Sokrat Studyja (S-005): prvi korisnik radi bez ijedne postavke. Profil je
`#[serde(default, deny_unknown_fields)]` — polje koje nedostaje uzima zadano, polje s tipfelerom je
greška, ne tiho ignoriranje (i to jedna poruka s putanjom, ne dvije). Ovo je jedina tablica profila
u dokumentaciji.

| polje | zadano | čemu služi |
|---|---|---|
| `default_branch` | `"main"` | grana nad kojom se računaju metrike kad je `branch_scope: "default"`; i dalje jedina referenca za dnevnik/plan/detached-provjeru |
| `branch_scope` (T48, S-032) | `"all"` | `"all"` = sve lokalne grane ulaze u mjerenje (`Report.scope`/`branches`, `CommitRow.branch`); `"default"` = samo `default_branch`, paritet sa starim ponašanjem; nepoznata vrijednost je greška koja imenuje `all`/`default` |
| `since` | `"2026-08-29"` | od kada se mjeri kad `--since` nije dan |
| `diary_path` | `"docs/records/PROGRESS.md"` | dnevnik iz kojeg se čitaju isporuke |
| `changelog_path` | `"docs/records/CHANGELOG.md"` | drugi dokument koji smije „pokriti" kašnjenje docs-a |
| `plan_path` | `"docs/plan/RASPORED.md"` | plan iz kojeg se čitaju cigle i faze |
| `docs_dir` | `"docs"` | mapa koja se rekurzivno čita (`node_modules`, `.git`, `target` se preskaču) |
| `docs_index` | `"docs/README.md"` | indeks; dokument koji u njemu nije spomenut je nalaz |
| `plan_dir` | `"docs/plan"` | mapa u kojoj se traži točno jedan aktivan spec |
| `plan_dir_ignore` | `["ROADMAP.md"]` | datoteke u `plan_dir` koje nisu spec |
| `paused_marker` | regex (blok §5) | oznaka da spec ne sudjeluje u „koliko je planova aktivno" |
| `product_dir` | `"docs/product"` | mapa definicije proizvoda (ne smije biti dnevnik) |
| `key_file` | `"CLAUDE.md"` | datoteka kojoj se mjeri veličina |
| `key_file_budget_bytes` | `40000` | prag; mjeri se bez `\r` |
| `diary_heading` | regex (blok §5) | naslov unosa u dnevniku → datum, model, naslov |
| `diary_deploy_pattern` | regex (blok §5) | unos u dnevniku koji znači deploy |
| `plan_brick` | regex (blok §5) | redak cigle u planu; `✅` znači gotova |
| `plan_phase_name` | regex (blok §5) | naslov faze u planu |
| `phase_tag` | regex (blok §5) | oznaka aktivne faze u opisu commita; commit se veže na fazu (`ph.id` ili dijete `"{id}/"`) ako mu poklapa (dug I3+M14 riješen, M2/6) |
| `classifier` | 4 pravila (blok §5) | uređena lista `(vrsta, regex)`; **redoslijed je ugovor** |
| `gate_pattern` | regex (blok §5) | podvrsta „brana i mjerenje" |
| `deploy_pattern` | regex (blok §5) | podvrsta „deploy" |
| `ci_fix_pattern` | regex (blok §5) | pokazatelj „CI-padova popravljenih" |
| `owner_name` | `"leon"` | isporuka se smatra vlasnikovom ako mu je ime u naslovu |
| `test_path_prefixes` | `["tests/"]` | testna putanja: počinje s… |
| `test_path_contains` | `["/check-"]` | …ili sadrži… |
| `test_path_suffixes` | `[".test.js", ".spec.js"]` | …ili se završava na |
| `test_path_exclude` | `[]` | podputanje koje se **ne** broje kao test iako su pod testnom putanjom (npr. `fixtures/`); `is_test_path` ga provjerava prvo, kao stražarsku klauzulu (M2/9) — zadano `[]` čuva paritet, Sokratisov vlastiti profil ga postavlja tek u T35 |
| `code_exclude_prefixes` | `["docs/"]` | putanja koja se NE smatra kodom (počinje s) |
| `code_exclude_suffixes` | `[".md"]` | putanja koja se NE smatra kodom (završava na) |
| `session_gap_hours` | `2.0` | razmak manji od toga = neprekinut rad |
| `session_start_hours` | `0.5` | fiksni dodatak za prvi commit nove sesije |
| `closed_phases` | 4 faze Sokrat Studyja | `{name, from, to, tag_pattern, note}`; broje se iz commita |
| `include_unmerged` | `false` | **rezervirano — jezgra ga u 0.1.0 ne čita** (vidi §11) |
| `unmerged_warn_days` | `5` | grana starija od toga → Warn |
| `unmerged_alert_days` | `10` | …starija od toga → Alert |
| `unmerged_alert_count` | `3` | …ili više od toliko takvih grana → Alert |
| `docs_lag_warn_days` | `2` | dnevnik toliko dana iza koda → Warn |
| `docs_lag_alert_days` | `5` | …toliko → Alert |
| `docs_weights` | `dead_link 5` · `not_indexed 3` · `multiple_plans 15` · `no_active_plan 10` · `diary_in_definition 5` · `lag 10` · `key_file_budget 5` | koliko koji nalaz odbija od 100 |

`Profile::log_since()` uzima **najraniji** datum od `since` i početaka zatvorenih faza — git log mora
dovući i commite starije od `since` da bi se zatvorene faze mogle prebrojati. Prozor dovlačenja je
zatim `min(log_since(), --since)`: bez tog `min`-a je `--since` stariji od profila tiho dobivao
kraći log nego što `Report.since` tvrdi (zaglavlje i podaci moraju se odnositi na isto razdoblje).

## 5 · Zadani regexi i klasifikator — doslovno

Iz `profile.rs`; ovdje su izvan tablice jer sadrže znak `|`.

```jsonc
"paused_marker":        "^\\s*\\*\\*Status:\\*\\*\\s*⏸️\\s*PAUZIRAN\\b"
"diary_heading":        "^## (\\d{4}-\\d{2}-\\d{2})(?:\\s*\\(([^)]*)\\))?\\s*[—-]+\\s*(.+)$"
"diary_deploy_pattern": "deploy|main na `"
"plan_brick":           "^\\| \\*\\*(F\\d)/(\\d+)\\*\\*\\s*(✅?)"
"plan_phase_name":      "^### (F\\d) · (.+)$"
"phase_tag":            "^(F\\d/\\d|C\\d[ab]?(?:/\\d\\w*)?|MREZA[- ]?[A-E]\\d?(?: \\(\\d/\\d\\))?|R\\d(?:/[A-Z0-9+]+)?|T\\d|ALAT-\\d|BUG-\\d+|U\\d)"
"gate_pattern":         "check:|brana|gate|^alat|^test|sonda|probe|mjera"
"deploy_pattern":       "na produkciji|deploy"
"ci_fix_pattern":       "^ci:|popravak ci|job je otkazan"

// classifier — redoslijed je ugovor: prvi regex koji pogodi pobjeđuje,
// a commit koji ne pogodi nijedan je "execution" (izvođenje procesa).
1. "planning"      → "raspored|dobiva svoj spec|tracnice|…|zadatak za sljedecu"
2. "documentation" → "^docs|compact|revizija pred|^r\\d/docs|ishod --|deploy-zapis|zapis uz"
3. "debugging"     → "^fix|bug-\\d|^ci:|popravak ci|kvar|obara|^test:|lagati|…|više ne postoji"
4. "polish"        → "^c[4-7]|paleta|mrtv|selektorski|\\bvan\\b|audit|…|dug plaćen"
```

Regexi su preneseni 1:1 iz `rad-xlsx.py` Sokrat Studyja, zato su hrvatski i zato su takvi kakvi su.
Puni tekst svakog je u `profile.rs` — ovdje su skraćeni tri točkice tamo gdje je lista dugačka.

**Format git loga** koji `io` traži i jezgra razumije (`io/src/git.rs`, `core/src/parse/gitlog.rs`):
`--format=@@%h|%at|%ct|%ad|%cd|%s` + `--numstat` + `--date=format:%Y-%m-%d` + `--reverse` + završni
`--` (bez njega je ime grane dvosmisleno s datotekom istog imena). Unix-vremena (`%at`, `%ct`) služe
za razmake, lokalni datumi za dan u tablici (`%ad`, autorov — paritet s `RAD.xlsx`) i za filtar
`since` (`%cd`, commitov — kao git). `--since` se **uvijek** šalje sa satom `00:00:00` (S-011): bez
sata git uzima trenutno doba dana, pa bi isti datum davao različit broj commita ovisno o tome kad se
izvještaj pokreće.

**Rezerva od jednog dana:** `io` traži log od **dana prije** granice (`civil::prev_day`). Razlog je
zona: `--since … 00:00:00` je ponoć u zoni **stroja**, a datumi u logu su u zoni **commita**, pa bi
stroj zapadnije od pohranjenog pomaka odbacio commite koje jezgrin filtar zadržava (najosjetljiviji
je prvi dan zatvorene faze). Mjerodavan je i ostaje jezgrin filtar `commit_date >= since` nad
tekstom; rezerva zato ne mijenja ni jednu brojku, samo čini izvještaj neovisnim o zoni stroja
(dopuna S-011).

## 6 · `.sokratis/` — ručni podaci u repou (S-004)

Tri datoteke u korijenu repoa, sve tri neobavezne, sve tri putuju kroz git s projektom.
Čita ih `io/src/project.rs`. „Datoteka ne postoji" je jedina greška koja tiho pada na zadano; svaka
druga (mapa umjesto datoteke, nema dozvole, pokvaren JSON) se javlja s putanjom.

**`.sokratis/profile.json`** — bilo koji podskup polja iz §4. Primjer je profil kojim Sokratis mjeri
sam sebe (dogfooding): vlastiti plan nema cigle ni faze, a testovi mu žive u `crates/*/tests/`.
`phase_tag` je u njemu upisan i od M2/6 stvarno radi: aktivne faze se na commite vežu ovim regexom,
ne više tvrdo kodiranim prefiksom `"{id}/"`.

```json
{
  "since": "2026-09-17",
  "plan_path": "docs/plan/ROADMAP.md",
  "closed_phases": [],
  "phase_tag": "^(M\\d/\\d+)",
  "owner_name": "leon",
  "test_path_contains": ["/tests/"]
}
```

**`.sokratis/overrides.json`** — ručna vrsta rada po SHA-i commita; pregazi klasifikator. Ključ je
kratka SHA kakvu daje `%h`, vrijednost jedan od pet engleskih identifikatora (S-008): `planning` ·
`documentation` · `execution` · `polish` · `debugging`.

```json
{ "8de5866": "polish", "687db7a": "documentation" }
```

**`.sokratis/visions.json`** — vizije prolaze kroz izvještaj nepromijenjene (jezgra ih ne računa).

```json
[
  { "title": "Sokratis mjeri sam sebe", "source": "Leon", "state": "u tijeku",
    "percent": 80, "note": "dogfooding profil od T22" }
]
```

## 7 · Čistoća dokumentacije (`DocsHealth`)

`core/src/docs.rs`. Ulaz su svi `*.md` iz korijena repoa i rekurzivno iz `docs_dir`, sa sadržajem i
vremenom zadnje promjene iz gita. Projekt bez `docs_dir` dobiva `docs: null`.

| provjera | nalaz kad | dokaz |
|---|---|---|
| `dead-link` | relativna `.md` poveznica ne pokazuje na postojeću datoteku | `datoteka:redak` |
| `not-indexed` | `.md` pod `docs_dir` nije spomenut u `docs_index` | datoteka |
| `multiple-active-plans` | više od jednog nepauziranog spec-a u `plan_dir` | popis putanja |
| `no-active-plan` | postoje specovi, ali su svi pauzirani | mapa plana |
| `diary-in-definition` | više od tri datuma u datoteci pod `product_dir` | broj datuma |
| `docs-lag` | dnevnik/changelog kasni `docs_lag_warn_days`+ dana za zadnjim commitom koda | broj dana |
| `key-file-budget` | `key_file` je veći od budžeta | bajtovi i prag |

Ocjena je `100 − Σ težina nalaza` (`saturating_sub`, nikad ispod nule). **Brojka se nikad ne
prikazuje bez popisa nalaza.** Poveznica unutar ograde kôda (```` ``` ````) je primjer, ne poveznica —
provjera je namjerno preskače.

## 8 · Signali smjera

`core/src/rules/`. Jedno pravilo = jedna datoteka + jedan test; `trait Rule` je ugovor, a
`default_rules()` vraća `Vec<Box<dyn Rule>>` pa novo pravilo ne dira postojeća. **Signal bez dokaza
(`evidence`) ne prolazi test.**

| pravilo | Warn | Alert | dokaz |
|---|---|---|---|
| `unmerged-branches` | nespojena grana izvan zadane starija od `unmerged_warn_days` | najstarija starija od `unmerged_alert_days` **ili** više od `unmerged_alert_count` takvih grana | ime grane · dana od zadnjeg commita · commita ispred zadane |
| `docs-lag` | dnevnik/changelog `docs_lag_warn_days`+ dana iza zadnjeg commita koda | `docs_lag_alert_days`+ dana | SHA i datum zadnjeg commita koda · koliko dana kasni dnevnik |

**Commit koda** = commit koji dira bar jednu putanju koju profil ne isključuje
(`code_exclude_prefixes`/`code_exclude_suffixes`) — zadano: sve osim `docs/` i `*.md`.

## 9 · CLI — naredbe i izlazni kodovi

`cli/src/main.rs`; hrvatske natpise za tablicu daje `cli/src/table.rs`, identifikatori u JSON-u
ostaju engleski (S-008). Putanja je neobavezna; bez nje je to trenutna mapa. Radi i iz podmape i iz
radnog stabla — korijen se dobiva iz `git rev-parse --show-toplevel`.

| naredba | ispis | izlazni kod |
|---|---|---|
| `sokratis report [putanja] [--since YYYY-MM-DD] [--until YYYY-MM-DD] [--json\|--table]` | cijeli `Report`; **bez zastavice je JSON**, `--table` daje tablicu s hrvatskim natpisima (zaglavlje pokazuje samo `since` — `until` se u tablici ne vidi, `docs/records/BACKLOG.md`); zastavice su **isključive** (`--json --table` je pogrešna uporaba, ne „zadnja pobjeđuje") | 0 (`until < since` → prazan izvještaj, i dalje 0) |
| `sokratis docs [putanja] [--json]` | ocjena, broj nalaza, kašnjenje; bez `docs_dir` poruka `docs: nema mape s dokumentacijom (n/a)` | 0 |
| `sokratis signals [putanja] [--json]` | signali s dokazom, ili `nema signala` | **0** nema · **1** Warn · **2** Alert |

Svaka greška okoline (nema `git`-a na PATH-u, putanja nije repozitorij, repozitorij bez commita,
pokvaren profil, tipfeler u datumu ili regex bez grupe) ispisuje se na stderr kao
`sokratis: <poruka>` i daje **izlazni kod 3**. Isti kod dobiva i **pogrešna uporaba CLI-ja**
(nepoznata zastavica, `--json --table`): `clap` bi sam izašao s **2**, a 2 je rezerviran za Alert, pa
`main` koristi `try_parse` i razliku presuđuje po tome piše li `clap` na stderr — `--help` i
`--version` idu na stdout i daju **0**. `process::exit` je u `main` i nigdje drugdje: izlazni kod je
ugovor prema preflightu, a ne nuspojava.

`--since` i `--until` (M2/19) postoje samo na `report`; `docs` i `signals` uzimaju `since` iz profila
i `until` ne primaju (uvijek `None`).

## 10 · Dvije zamke koje je otkrio dogfooding

1. **„Testni redak" se prepoznaje SAMO po putanji** (`test_path_prefixes` · `test_path_contains` ·
   `test_path_suffixes`, `Profile::is_test_path`). Posljedice koje treba znati prije čitanja brojke
   „udio testnih redaka": inline `#[cfg(test)] mod tests` unutar `src/*.rs` se **ne broji** kao test
   (živi u istoj datoteci kao produkcijski kod), a **sve** pod testnom putanjom se broji — i fixture
   datoteke pod `tests/`, koje nisu kod testa. Sokratis zato u svom profilu ima
   `test_path_contains: ["/tests/"]`: njegovi testovi žive u `crates/*/tests/`, što zadani prefiks
   `tests/` ne hvata. Posljedica koju treba znati pri čitanju **Sokratisova vlastitog** udjela
   testnih redaka: pod tom putanjom leže i fixture datoteke (snimka `PROGRESS.md` Sokrat Studyja ima
   767 kB), pa je većina njegovih „testnih redaka" fixture, ne kod testa. Polje koje to rješava
   (`test_path_exclude`) sad radi (M2/9), ali Sokratisov vlastiti profil ga još ne postavlja —
   `["/fixtures/"]` dolazi tek u T35, do tada fixture i dalje napuhuje njegov udio testnih redaka.
2. **`touched.files` je broj izmjena datoteka, ne broj različitih datoteka** (§3). Ista datoteka
   dirnuta u deset commita doda deset. Brojka odgovara na „koliko je izmjena pročitano", ne na
   „koliko datoteka projekt ima".

## 11 · Što stoji u kodu, a još ne izlazi ili ne radi (0.1.x)

Uredno zapisani propusti, ne skrivene rupe (CLAUDE.md #4). Ovo je **stanje koda**; što se od toga
planira uzeti i kada je u [`../records/BACKLOG.md`](../records/BACKLOG.md), a za stavke koje je
preuzeo M2 u [`../archive/ARHITEKTURA_M2.md`](../archive/ARHITEKTURA_M2.md) §8 i planu cigli
[`../superpowers/plans/2026-09-18-m2-desktop.md`](../superpowers/plans/2026-09-18-m2-desktop.md).

**Rezervirano polje profila** (deklarirano, jezgra ga ne čita):

- **`include_unmerged`** — metrike su uvijek samo nad zadanom granom, a nespojene grane ulaze
  isključivo u signale.

(`phase_tag` je do M2/6 bilo ovdje — sad radi, §4 i §6.)

**Izračunato, ali ne izlazi u CLI-tablicu** (JSON ga od M2/4–M2/5 nosi, §3):

- **`GitSource::worktrees`** je implementiran i testiran, ali izvještaj ga ne koristi: identitet
  projekta preko više radnih stabala je posao M2.
- **`Report.commits`/`deliveries`/`vision_totals`** postoje u JSON-u, ali `cli/src/table.rs` ih ne
  ispisuje — redak po commitu (uz `classify_sub`, sad `CommitRow.sub`), redak po isporuci i zbroj
  vizija po stanju čekaju pogled Dnevnik/Isporuke/Vizije u sučelju M2.
- **`Report.scope`/`branches` (M2/46, S-032) postoje u JSON-u i `io` ih od T49/T50 stvarno puni** iz
  profila (§1, §3), ali NITKO ih još ne prikazuje: CLI-tablica nema ispis (`--scope` na CLI-ju dolazi
  s T51), a sučelje (SUČELJE-2, §1 tablica) ih još ne crta ni jednim pogledom — dolazi s PLOČOM
  (T58+).

**Rubovi koje kod danas ne pokriva:**

- **Commit se klasificira više od jednom po izvještaju**, iako spec §3.2 traži jednom.
  `metrics/kinds.rs::commit_rows` (M2/5) klasificira svaki commit točno jednom za `Report.commits`, a
  `kind_stats` broji iz tih redaka — ali `metrics/indicators.rs:31` (`kind_count`, pokazatelji
  `debugging_commits` i `docs_share`) i dalje zove `effective_kind` odvojeno. Nalaz recenzije M2/5
  (2026-09-18), otvoren do završne recenzije M2: [`../records/BACKLOG.md`](../records/BACKLOG.md).
- **Tablični ispis (`cli/src/table.rs`) nije dovršen kao sučelje:** udjeli su goli razlomci
  (`0.589`) dok VRSTE RADA imaju procente, stanje faze ide kroz `{:?}` pa u hrvatskoj tablici stoji
  `Closed`/`Running`/`Planned` bez prijevoda (S-008: natpise daje sučelje), prazan naslov „FAZE"
  ostaje bez ijednog retka, ime faze dulje od 50 znakova prelije stupac, a zaglavlje ne pokazuje
  `until` (§9). JSON je ugovor i on je točan; tablica je pomoć za terminal.
- **Cijena su procesi, ne parsiranje — keš sad ima pozivatelja, ali samo u desktopu.** M2/11 je
  smanjio broj git-procesa po `input()` (dug M11; brojke, ne ovdje — `CHANGELOG.md`, S-010) i taj
  dobitak vrijedi za svaki poziv CLI-ja i desktopa. Ostatak cijene (git log koji svaki put šeta cijelu
  dostižnu povijest) rješava keš sirovih commita (STORE M2/18) kroz `Project::input_cached`/
  `cached_log` (IO M2/14b) — adapter `StoreCache` (`desktop/src-tauri/src/cache.rs`, T29) ga ožiči nad
  `sokratis-store`, `commands.rs::compute` ga zove za svaki izračun. **CLI i dalje ga ne zove**
  (`report_for` poziva `input_between`, bez keša) — izmjereni dobitak keširanog puta (topao ulaz +
  izvještaj ispod 500 ms, `SOKRATIS_PERF_REPO`, `#[ignore]`) korisnik CLI-ja i dalje ne osjeća, korisnik
  desktopa hoće.
- **SQLite (registar, postavke, snimke, keš) i watcher imaju pravog pozivatelja od DESKTOP-a**
  (T29–T30, §1) — `AppState.store`, `engine.rs`. **CLI i dalje ne dira nijedno od toga**: sve računa na
  zahtjev i ne piše ništa osim onoga što korisnik sam stavi u `.sokratis/` — S-009 vrijedi za desktop,
  ne za CLI.
- **`Signal.title_key` (`core/src/model.rs`, npr. `"signal.unmerged_branches"`) nema par u
  i18n-rječniku.** Prijevod natpisa pojedinog signala umjesto njega čita drugi, plosnatiji ključ
  izveden iz `Signal.rule` (`rule.<rule>`, npr. `rule.unmerged-branches` — crtica kao u polju `rule`,
  ne podvlaka kao u `title_key`): dodan T27 za `SignalBar.svelte`, isti ključ DESKTOP (T30, `engine.rs`)
  čita za obavijest OS-a preko `include_str!`. Dva polja istog signala (`rule` i `title_key`) tako vode
  na dva RAZLIČITA imena ključa — `title_key` je danas mrtvo polje s prijevodne strane.
- **Zatvorena faza s nula pogodaka nestaje iz izvještaja, ne prikazuje se kao 0/0** (M2/36, Ruling
  R21, §4/§6 niže). Cijena popravka (spec §13.7 — profil ne smije tvrditi tuđu povijest): ako BILO
  KOJI projekt (uklj. Sokrat Study) stvarno deklarira zatvorenu fazu u planu koja slučajno nema
  ijedan commit s pripadnom oznakom (npr. tipfeler u `tag_pattern`), razlika prema „ova faza nikad
  nije ni postojala u profilu" se gubi — obje izgledaju kao da faze nema. `active_phases` čita cijeli
  `input.plan` tog repoa i ovim filtrom nije pogođen (aktivne faze se ne filtriraju).
- **`closed_phases_in_range` (`metrics/indicators.rs:80-83`) gleda samo `since`, ne `until`** —
  pokazatelj „zatvorenih faza u razdoblju" broji i faze zatvorene NAKON `until` ako je raspon
  ograničen gornjom granicom. Nalaz recenzije M2/42 (2026-09-23, kartica s objašnjenjem): tekst
  kartice to danas istinito opisuje („od početka razdoblja nadalje"); popravak jezgre čeka krug
  popravaka S5 (snimka ima `until: null`, pa se namjerno ne mijenja dok test ne dokaže popravak).

**Svih devet tokova M2 su gotovi i spojeni** (KOSTUR, JEZGRA, PROFIL, STORE, IO uklj. M2/14b, CLI,
SUČELJE, DESKTOP, INTEGRACIJA — popis u zaglavlju ovog dokumenta). Ono što je DESKTOP-om (T29–T33,
2026-09-21) dobilo pravog pozivatelja u aplikaciji (gornje točke to objašnjavaju detaljnije, §1 daje kod):

- `sokratis-store` u cijelosti — registar, postavke, snimke, keš — zove ih `commands.rs`/`engine.rs`
  preko `AppState.store` (§1);
- `io::CommitCache`/`cached_log`/`Project::input_cached` (M2/14b) — zove ih adapter `StoreCache`
  (`cache.rs`, T29); poznata ograničenja i dalje vrijede: `touched.skipped_lines` ne broji retke
  preskočene pri PRVOM čitanju commita (keš pamti `Commit`-e, ne sirovi tekst gita); ključ keša je
  kratki SHA (`%h`) — ako git jednog dana produlji zadanu duljinu kratice, keš se jednom puni iznova
  (`docs/records/BACKLOG.md`);
- `io::Watcher` (M2/13) — nit motora (`engine.rs`) čita njegov `WatchEvent` i zove `request_refresh`;
- `core/src/snapshot.rs` tipovi i `sokratis-store::save_snapshot`/`latest_snapshot`/`trend` (M2/17) —
  motor piše snimku pri svakom izračunu, `get_trend` ih čita; ništa u `apps/desktop/src` (Svelte) ih
  još ne prikazuje (§3).

**CLI ostaje izvan svega gore** — `report`/`docs`/`signals` i dalje ne diraju `sokratis-store`, ne
koriste keš i ne pokreću watcher; §1 (`io` paragraf) i prethodne dvije točke to imenuju eksplicitno.

**Sučelje — poznati rubovi (neovisni o T34):**

- **`Vision.state` je slobodan tekst** (`model.rs`: `state: String`, ne enum) — obrazac u `Visions.svelte`
  ga uzima izravno iz korisničkog unosa, sučelje ga ne prevodi ni ograničava kroz i18n.
- **Potvrda brisanja vizije viri 13 px preko ruba tablice** na najužem prozoru (960 px) — kozmetički
  nalaz vizualne provjere M2/28, odgođen.
- **Stupac „model" u pogledu Isporuke ponekad nosi ostatak zaglavlja dnevnika**, ne samo ime modela —
  uzrok je parser isporuka u jezgri (`core/src/parse/diary.rs`, tok PARSE), ne sučelje; nalaz otkriven
  gradnjom pogleda Isporuke (M2/28), popravak čeka ciglu u `core`.

**DESKTOP i INTEGRACIJA su u `main`-u (T29–T37, 2026-09-21/22) — sučelje zove prave naredbe, ali ovo
i dalje ne radi ili nije provjereno:**

- **Klik na obavijest OS-a ne otvara projekt** (spec §5.3) — `tauri-plugin-notification` na Windowsu
  nema povratni poziv za klik; obavijest samo javlja, ne vodi nikamo (Ruling R7, dopuna T30).
- **Rub autostarta (Ruling R12, T32; `tray.rs` uklonjen M2/45, prekidač je od tad u pogledu
  Postavke):** ako `tauri-plugin-autostart` uspije registrirati OS, a upis u bazu padne (otrovana
  brava, SQLite), prekidač se vrati na staro dok baza kaže suprotno od OS-a — OS je stvarno
  promijenjen, baza i prekidač privremeno lažu dok korisnik ponovno ne klikne (ponovni klik je
  idempotentan i popravlja stanje). Lijek (`is_enabled()` iz plugina kao izvor istine za prekidač i
  `get_settings`) čeka krug popravaka S5.
- **Druga instanca otvori bazu prije nego što je `single-instance` stigne odbiti** — `Store::open` je
  u `.manage(...)`, koji se zove PRIJE `.run()`; bezopasno (SQLite `open` + idempotentne migracije),
  ali znači da i odbijeni proces načas dirne datoteku baze.
- **`project_worktree.branch` se puni praznim tekstom** — `io::Project::git::worktrees()` daje samo
  putanje radnih stabala, ne granu svakog; sučelje danas crta samo BROJ stabala, pa je prazan tekst
  bez posljedice (Ruling R6).
- **Ručna lista za Leona** ([`TESTING.md`](../workflow/TESTING.md) §5, popis se ne ponavlja ovdje —
  S-010): autostart ↔ `HKCU\…\Run`, obavijest na prijelaz u Alert, preskok splasha,
  `prefers-reduced-motion`, osvježenje bez klika sa štopericom, četiri teme.
