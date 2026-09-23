# Changelog — Sokratis

Format: [Keep a Changelog](https://keepachangelog.com/) · Verzioniranje: [SemVer](https://semver.org/).
Isporuka = ono što je u `main`-u; sesije su u `PROGRESS.md`.

## [Unreleased] — rad u tijeku

**Milestone 2 (desktop) je u izvedbi** po odobrenom specu
[`../archive/ARHITEKTURA_M2.md`](../archive/ARHITEKTURA_M2.md) (odluke S-012…S-022) i planu od 35 cigli.
**Ovaj odjeljak izlazi kao 1.0.0, ne 0.2.0** (S-024; rez i dopuna speca §13 od 2026-09-21).
**Na disku je `main` verzije `1.0.0-pre.1`** (jedan izvor: `[workspace.package]` u korijenskom
`Cargo.toml`, M2/37) — netagirano, neobjavljeno; tag traži Leonov izričit OK na kraju etape izdanja
(S-025). Status: [`../plan/ROADMAP.md`](../plan/ROADMAP.md) · tijek sesije:
[`PROGRESS.md`](./PROGRESS.md).

- **`M2/1a` — kostur M2 (2026-09-18).** Korisnik CLI-ja **ne vidi ništa novo**: nema nove naredbe ni
  promjene ponašanja, sve tri naredbe rade kao u 0.1.0. Što se ispod promijenilo: `Report` u JSON-u
  dobiva **četiri nova polja** (`until`, `commits`, `deliveries`, `vision_totals`) — zasad prazna
  (`null`/`[]`), pune ih cigle M2/3–M2/5; profil prima polje `test_path_exclude` (zadano `[]`,
  ponašanje M2/9); dodan je **nov crate `sokratis-store`** (SQLite: otvori bazu, primijeni migraciju
  — registra i snimki još nema). Tko čita `Report` kao ugovor, mora znati i ovo: **od cigle M2/2 se
  oblik JSON-a mijenja samo namjerno**, uz snapshot-test i rečenicu u commitu (S-022). Što točno
  stoji u kodu a ne radi: [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11.
  Brane: `cargo test --workspace` **65 testova**, `sokratis docs .` 100/100, `signals .` 0.
- **`M2/1b` — kostur M2 dovršen (2026-09-18).** I dalje **ništa novo za korisnika CLI-ja**:
  `report`/`docs`/`signals` nepromijenjeni. Ispod: `npm install` (80 paketa, verzije pinane, 0
  ranjivosti) i `npm run tauri icon` iz Leonova loga popunili su `apps/desktop/src-tauri/icons/`
  (desktop + Windows set; `android/`/`ios/` nisu commitani — aplikacija je desktop-only); crate
  `sokratis-desktop` je natrag u `[workspace] members` i **builda se** (prvi `cargo build -p
  sokratis-desktop` ~3 min). Odstupanje od plana, s razlogom (pravilo #6): `vite.config.ts` uvozi
  `defineConfig` iz `vitest/config` umjesto `node:url`, jer bi nova ovisnost samo radi
  config-datoteke bila neopravdana. Brane: `cargo test --workspace` **65 testova**, `npm run check`
  zelen (svelte-check 0 grešaka + 1 vitest), `npm run build` daje `dist/` s `index.html` i
  `splash.html`, `sokratis signals .` 0. Time je **T1 (kostur) cijel u `main`-u**; otvorena su
  četiri radna stabla tokova (JEZGRA · PROFIL · IO · SUČELJE) i poslan prvi val graditelja.
- **`M2/2` — snapshot ugovora `Report`-a (2026-09-18).** Korisnik CLI-ja **ne vidi ništa novo**.
  Ispod: `crates/sokratis-core/tests/snapshot.rs` zaključava oblik JSON-a `Report`-a nad fixtureom
  pariteta (`insta::assert_json_snapshot!`, 467 redaka, 15 ključeva na vrhu) — od sada svaka promjena
  oblika (novo polje, preimenovanje, drugi redoslijed) pada dok se snimka namjerno ne ažurira uz
  rečenicu u commitu (S-022). BACKLOG-stavka I7 je time riješena.
- **`M2/3` — `until` kao gornja granica razdoblja (2026-09-18).** Korisnik CLI-ja i dalje ne vidi
  novu zastavicu (`--until` dolazi u T19, birač raspona u sučelju kasnije) — ono što se promijenilo
  je jezgra: `until` u `ReportInput`/`Report` sad stvarno filtrira (cijeli dan uključivo, S-011:
  commiti i isporuke s datumom poslije `until` ispadnu) i validira oblik (`ParseError::BadDate {
  field: "until" }`, isti ugovor kao `since`). `civil::next_day` je zrcalo `prev_day`-a, priprema za
  rezervu zone koju IO T14 treba za `--until` prema gitu.
- **`M2/8` — ograda putanja iz profila (2026-09-18).** Nijedna promjena za korisnika CLI-ja danas
  (ograda još nije spojena na otvaranje projekta, IO T14) — jezgra sad zna reći: `inside_root(rel)`
  i `Profile::validate_paths()` odbiju profil čije bilo koje od osam polja putanja pokazuje izvan
  korijena repoa (`..`, apsolutna putanja, `C:\…`, `\\server\share`); greška imenuje polje
  (`ParseError::PathOutsideRoot`). Jezgreni dio duga I9; io-dio (provjera pri `Project::open`) dolazi
  u T14 — I9 je zato **djelomično**, ne potpuno riješen.
- **`M2/9` — `test_path_exclude` u profilu (2026-09-18).** Zadani profil se ne mijenja (S-005: zadano
  `[]`, paritet netaknut) — polje sad radi: putanja koja sadrži unos s popisa se više ne broji kao
  test, iako je pod testnom putanjom (npr. `tests/fixtures/…`). Sokratisov vlastiti profil dobiva
  `["/fixtures/"]` tek u T35.
  Brane na `main`-u nakon PROFIL (T8–T9) i JEZGRA 1/2 (T2–T3): `cargo fmt --check` OK · `cargo clippy
  --workspace -- -D warnings` OK · **72 testa** · `sokratis signals .` = nema signala.
- **`M2/4` — zbroj vizija po stanju (2026-09-18).** Korisnik CLI-ja i dalje ne vidi ništa novo (tablica
  vizije ne ispisuje) — `Report.vision_totals` više nije uvijek `[]`: broji vizije po `state` u JSON-u
  (dug I6 riješen). Snimka ugovora netaknuta (fixture nema vizija, `vision_totals(&[])` i dalje `[]`).
- **`M2/5` — redci commita i isporuke u `Report`-u (2026-09-18).** Korisnik CLI-ja i dalje ne vidi
  ništa novo u tablici — ispod: `Report.commits` (redak po commitu: `sha` · `date` · `subject` ·
  `kind` · `sub` · `overridden`) i `Report.deliveries` (redak po isporuci: `date` · `model` · `title` ·
  `kind` · `deploy`) više nisu uvijek `[]` — hrane buduće poglede Dnevnik i Isporuke u sučelju.
  Klasifikacija ide preko novog `commit_rows`, koji svaki commit klasificira jednom za ovaj redak;
  `kind_stats` sad broji iz tih redaka umjesto da klasifikaciju ponavlja. **SNAPSHOT NAMJERNO
  PROMIJENJEN** (S-022): `commits` 190 objekata, `deliveries` 105 (= pokazatelj `deliveries`); sva
  ostala polja snimke ostaju duboko jednaka (programski dokazano), raspodjela `commits[].kind` = `kinds[]`.
  Otvoreno (nalaz recenzije, ne popravljeno ovom ciglom): `metrics/indicators.rs` i dalje zove
  `effective_kind` odvojeno za dva pokazatelja — commit se klasificira više od jednom, ne jednom kako
  spec §3.2 traži; odluka na završnoj recenziji M2 (`BACKLOG.md`).
- **`M2/6` — aktivne faze preko `phase_tag` iz profila (2026-09-18).** Nijedna promjena za korisnika
  CLI-ja danas — aktivne faze se sad na commit vežu regexom `phase_tag` iz profila (`ph.id` ili dijete
  `"{id}/"`), ne tvrdim prefiksom `"{id}/"` (dug I3 + M14 riješen); zadani profil (Sokrat Study) daje
  iste brojke kao prije, diff snimke bajtno prazan.
- **`M2/7` — snimka brojki i prijelaz signala (2026-09-18).** Korisnik CLI-ja ne vidi ništa novo —
  `core/src/snapshot.rs` prestaje biti prazan modul: `SnapshotMetrics::from_report` (18 pokazatelja,
  docs-ocjena, broj signala po težini), `diff` (promjene po `id`-u, NaN-svjestan), `worst_severity`,
  `alerts_raised` (javlja SAMO prijelaz u Warn/Info → Alert, S-020) — priprema STORE T17 (snimke) i
  DESKTOP T29/T30 (obavijesti). `lib.rs` dobiva točno jedan redak (`pub use snapshot::{…}`).
  Brane nakon JEZGRA 2/2 (T4–T7, merge): `cargo fmt --check` OK · `cargo clippy --workspace -- -D
  warnings` OK · **82 testa** · `sokratis signals .` = nema signala. Tok JEZGRA je time **gotov**
  (T2–T7 svi u `main`-u).
- **`M2/15` — registar projekata i stabala (2026-09-20).** Korisnik CLI-ja ne vidi ništa novo (CLI ne
  dira `sokratis-store`) — crate dobiva `registry.rs`: `add_project`/`list_projects`/`project`/
  `rename_project`/`remove_project`/`touch_project`/`set_worktrees`/`worktrees`. Identitet projekta je
  `git_common_dir` (S-015): dva radna stabla istog repozitorija dijele jedan zapis, duplikat vraća ime
  već upisanog umjesto da tiho spoji; brisanje projekta kaskadno odnosi njegova radna stabla i
  postavke (`ON DELETE CASCADE`).
- **`M2/16` — postavke, globalne i po projektu (2026-09-20).** `setting`/`set_setting` (tema, jezik,
  autostart, položaj prozora) i `project_setting`/`set_project_setting` (raspon, zadnji pogled) —
  upisuju kroz `INSERT … ON CONFLICT DO UPDATE` u jednom SQL-pozivu; čitanje postavke koja nikad nije
  zapisana vraća `None`, ne grešku; pisanje na uklonjen ili nepostojeći projekt je tipizirana
  `NoSuchProject`, ne sirova greška stranog ključa.
- **`M2/17` — snimke brojki, profil kao kanonski JSON, trend (2026-09-20, S-014).**
  `record_profile`/`save_snapshot`/`trend`/`latest_snapshot`: profil se sprema u kanonskom obliku
  (ključevi sortirani preko `serde_json::Value`/`BTreeMap`) pa dva sadržajno jednaka profila u drugom
  poretku dijele isti `profile_seen_id`; metrika koja je `NaN` ili beskonačna se preskače pri upisu,
  ne pretvara u nulu. **Krug popravka:** druga snimka istog dana je sad **cjelovita zamjena** —
  `DELETE` pa `INSERT` u istoj transakciji — jer je ostavljala jutrošnje retke kad je večernja snimka
  imala manje metrika, pa je `latest_snapshot` vraćao mješavinu dviju snimki.
- **`M2/18` — keš sirovih commita po SHA (2026-09-20, uvjetna cigla, S-014).** Ušla je jer je mjerenje
  M2/11 dalo 1479,71 ms ≥ prag 500 ms: `put_commits`/`cached_commits`/`newest_cached_commit_date`
  čuvaju SIROVE commite (nikad klasifikaciju) po SHA, `INSERT OR IGNORE` u jednoj transakciji. Korisnik
  CLI-ja i dalje ne vidi ništa novo — keš nema potrošača dok IO ne dobije inkrementalni `git log` +
  punjenje/čitanje keša (posebna cigla, nije izgrađena).
  Brane nakon svih četiriju (merge `e9b01c7`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **107 testova, 0 padova** (store crate
  sam **26**: 2 unit + 10 cache + 3 registry + 2 settings + 9 snapshots) · `sokratis signals .` = nema
  signala. Tok STORE je time **gotov** (T15–T18 svi u `main`-u).
- **`M2/10` — pisanje ručnih podataka u glavno stablo, atomarno (2026-09-20, S-015).** Korisnik CLI-ja
  ne vidi ništa novo (CLI ne piše `overrides.json`/`visions.json`) — `Project::write_override`/
  `write_visions` pišu kroz `.tmp` pa `rename` (atomarno: pad usred pisanja ne ostavlja pola JSON-a) u
  GLAVNO stablo (`main_root`, roditelj `common_dir`), ne u radno stablo iz kojeg je pozvano — pisanje
  iz linked worktree-a završava na jednom mjestu koje git prati.
- **`M2/11` — performanse: ~94 → 4 git-procesa po izvještaju (2026-09-20, dug M11).** Korisnik CLI-ja
  dobiva brži `report`: `last_changes` čita zadnju promjenu SVIH dokumenata jednim `git log
  --name-only` umjesto po datoteci, `branches()` čita `ahead-behind` jednim `for-each-ref` (uz rezervu
  `branches_per_ref` za git < 2.41) umjesto po grani. Izmjereno nad Sokrat Studyjem: 92 → 6
  git-procesa, 3318–3822 ms → **1479,71 ms**. Krug popravka: `--diff-merges=combined` (merge-commit
  više ne gubi doprinos putanje) i `-c core.quotepath=false` (ne-ASCII imena datoteka se više ne
  gube) — 0/56 neslaganja nad Sokrat Studyjem nakon popravka. Mjerenje ostaje ≥ prag 500 ms → keš
  sirovih commita (M2/18, već u `main`-u kroz STORE) je time uvjet ispunio.
- **`M2/12` — detached HEAD dobiva istinitu oznaku (2026-09-20).** Repozitorij u detached HEAD stanju
  (grana ne postoji, commit postoji) više ne javlja lažno „repozitorij nema commita" —
  `Report.branch` postaje `HEAD@<sha>` (`GitSource::head_sha`), grane i log se i dalje čitaju.
- **`M2/13` — watcher nad `.git`, docs i `.sokratis` (2026-09-20, S-016).** Ispod CLI-ja (watcher još
  nema potrošača — DESKTOP T30): `Watcher` javlja najviše jedan `WatchEvent` po projektu 600 ms nakon
  zadnje promjene, potiskuje vlastite upise (uklj. `.tmp` privremenu datoteku i mkdir pretka kad
  `.sokratis` tek nastaje) i serijalizira preklapajuće izračune (`RefreshQueue`).
- **`M2/14` — birač raspona i ograda putanja pri otvaranju (2026-09-20, S-011 dopuna 2, dug I9
  ZATVOREN).** `GitSource::log` dobiva `until` (rezerva dva dana prema naprijed); `Project::open` sad
  zove `validate_paths()` ODMAH nakon učitavanja profila — putanja izvan repoa (npr. `docs_dir:
  "../.."`) je greška s imenom polja (`IoError::ProfileInvalid`) već pri otvaranju projekta, ne tek
  kad jezgra pokuša pročitati datoteku izvan njega. Time je **dug I9 zatvoren u cijelosti** (jezgreni
  dio M2/8 + io-dio M2/14). `--until` u CLI-ju samom i dalje ne postoji (dolazi s M2/19). Krug
  popravka: `IoError::ProfileInvalid` drži uzrok samo u `#[source]` lancu (obrazac I5) — CLI je prije
  rečenicu o polju ispisivao dvaput.
  Brane nakon svih pet (merge `3ca068c`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **123 testova, 0 padova** · `sokratis
  signals .` = nema signala. Tok IO je time **gotov** osim M2/14b (potrošač keša, cigla koju plan
  nema, nastavlja na istoj grani).
- **`M2/19` — `sokratis report --until` (2026-09-20).** Korisnik CLI-ja dobiva novu zastavicu:
  `--until YYYY-MM-DD` ograničava izvještaj na gornju granicu razdoblja (zrcali `--since`, S-011/
  S-012) — `docs` i `signals` je i dalje ne primaju. Neispravan datum → izlazni kod 3 s porukom koja
  imenuje polje; `until` prije `since` daje prazan izvještaj, kod 0. Poznato ograničenje: `--table` u
  zaglavlju `until` ne ispisuje (JSON je ugovor i on je točan; odgođeno u `BACKLOG.md`).
  Brane nakon spajanja (merge `11b1708`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **124 testa, 0 padova** · `sokratis
  signals .` = nema signala. Tok CLI je time **gotov** (T19 u `main`-u).
- **`M2/14b` — potrošač keša sirovih commita (2026-09-20, cigla IZVAN plana od 35).** Napisao ju je
  orkestrator, ne brif iz plana: povod je mjerenje M2/11 (izvještaj nad Sokrat Studyjem i dalje ≥ prag
  500 ms) i keš iz M2/18 (dotad bez potrošača). Korisnik CLI-ja ne vidi ništa novo — CLI keš i dalje ne
  koristi. Ispod: `sokratis-io` dobiva `CommitCache` (trait `cached`/`store`) i `cached_log` —
  `GitSource::rev_list` kaže što je dostižno u prozoru SADA, keš vraća poznate commite, `git log
  --no-walk --stdin` dovlači SAMO nedostajuće; `commit --amend`/`reset --hard` time ne ostavljaju stari
  SHA u brojkama iako ostaje u kešu. `sokratis-core` dobiva `format_gitlog` (inverz parsera git loga, s
  testom okruglog puta) jer keš pamti `Commit`-e, ne sirovi tekst. `io` i dalje ne ovisi o `store`
  (S-013) — adapter prema `sokratis-store` dolazi u desktopu (T29). **Izmjereno nad Sokrat Studyjem**
  (release, `SOKRATIS_PERF_REPO`): bez keša ~2,0–2,7 s, topao keš + izvještaj **256–472 ms** — cilj
  < 500 ms postignut. Poznata ograničenja (`BACKLOG.md`): `touched.skipped_lines` na keširanom putu ne
  broji retke iz PRVOG čitanja commita; ključ keša je kratki SHA (`%h`), pa se keš jednom puni iznova
  ako git produlji kraticu (točnost ne strada).
  Brane nakon spajanja (merge `0740685`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **136 testova, 0 padova, 1 ignoriran**
  (mjerni test) · `sokratis signals .` = nema signala. Tok IO je time **gotov u cijelosti** (T10–T14 +
  M2/14b u `main`-u).
- **README na engleskom (2026-09-21, S-030).** Korijenski `README.md` je preveden i osvježen da ga
  može pročitati svatko tko otvori javni repo; `docs/` ostaje na hrvatskom. Koda nije dirano.
- **SUČELJE (T20–T28) u `main`-u (2026-09-21).** Korisnik dobiva prvi put PRIKAZ `Report`-a: Svelte
  5 + Tailwind v4 sučelje s `tokens.css` (četiri teme, zadana „Akademsko plavo", brana kontrasta),
  HR/EN rječnikom (brana pariteta ključeva), vlastitim SVG grafovima i splash animacijom (4,2 s,
  preskočiva) — devet pogleda: Pregled, Tempo, Vrste rada, Pokazatelji, Faze, **Dnevnik** (vrsta
  rada po commitu se uređuje `<select>`-om, oznaka „ručno"), **Isporuke** (najnovije prvo, 🚀 za
  deploy), **Vizije** (pilule po stanju, dodaj/uredi/obriši u mjestu) i **Dokumentacija** (ocjena,
  nalazi, klik na putanju kopira). Mjerenje ostaje u jezgri, sučelje samo prikazuje (S-012). Sučelje
  **radi zasad samo u pregledniku** (`npm run dev` u `apps/desktop`) — jedina izvedba `Api`-ja je
  `MockApi` nad snimkom jezgre; prava Tauri-ljuska i pravi podaci dolaze s DESKTOP-om (T29–T34).
  Brane: `cargo test --workspace` **136 testova, 1 ignoriran** · `npm run check` (svelte-check,
  i18n-parnost, kontrast, 66 vitest testova) · `npm run build` OK · `sokratis docs .` 100/100 ·
  `signals .` 0.
- **DESKTOP (T29–T33 + tri pred-cigle) u `main`-u (2026-09-21).** Korisnik prvi put dobiva PRAVU
  Tauri ljusku umjesto praznog prozora: `apps/desktop/src-tauri` (crate `sokratis-desktop`) sad ima
  `AppState` i jedanaest naredbi kroz koje sučelje zove pravu jezgru — `Report` prolazi nepromijenjen
  (S-012) — registar projekata (`list_projects`/`add_project`/`rename_project`/`remove_project`),
  izvještaj i trend po rasponu (`get_report`/`get_trend`, četiri gotova presjeka ili vlastiti raspon),
  ručni podaci (`set_override`/`save_visions`), postavke (`get_settings`/`set_setting`) i ručno
  osvježavanje (`refresh`). Adapter (`cache.rs`) spaja keš sirovih commita (M2/18) sa storeom; na
  grešku keša ponovi bez keša umjesto da naredba padne. Motor osvježavanja (`engine.rs`): watcher,
  naredbe i tray dijele JEDAN red čekanja po projektu (`request_refresh`, spec §3.3 t. 3) — izračun je
  uvijek nad cijelim rasponom, piše dnevnu snimku pri svakom izračunu i javlja
  `report_updated { project_id }`; `signal_raised { project_id, rule, severity }` + obavijest OS-a
  stiže SAMO na prijelaz u Alert, nikad pri prvom izračunu nakon pokretanja (S-020). Splash prozor
  čeka animaciju I prvi izračun svih projekata, prikazuje glavni prozor točno jednom uz rezervu od
  10 s (S-019). Tray (Otvori · Osvježi sve · Autostart ✓ · Izađi), X sakriva samo glavni prozor,
  autostart kroz plugin, jedna instanca podiže postojeći prozor. Tray-ikona je znak bez lika
  (`graph.webp`, izabrao Leon). Tri pred-cigle izvan toka: `CommitRow.author_time` (namjeran
  snapshot, S-022), `io::today()` + normaliziran `common_dir` (S-015 — dva radna stabla istog
  projekta se više ne dupliciraju u registru), determinističan `alerts_raised`. Sučelje i dalje radi
  SAMO nad `MockApi` (T34) — instalabilne aplikacije još nema.
  Brane nakon spajanja (merge `cc74bb8`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **142 testa, 1 ignoriran, 0 palo** ·
  `npm run check` (i18n 160 hr=en · kontrast 4/4 · vitest 66/66) · `npm run build` OK · `sokratis
  docs .` 100/100 · `signals .` nema signala. Tok DESKTOP je time **gotov** (T29–T33 u `main`-u).
- **INTEGRACIJA (T34, T36, T37) u `main`-u (2026-09-22).** Korisnik dobiva PRAVU aplikaciju umjesto
  demoa: `apps/desktop/src` prestaje raditi SAMO nad `MockApi` — `TauriApi` prevodi svaku metodu
  `Api`-ja u `invoke()` prema `commands.rs`, `report_updated` osvježava popis I trenutačni izvještaj
  kroz jednu globalnu pretplatu, a svaki neuspio poziv sad izlazi na ekran u traci greške umjesto da
  tiho nestane (zatvara nošene nalaze T25/T26). **Repo bez konvencija Sokrat Studyja više ne laže**
  (spec §13.7): repo bez `docs/`, dnevnika, plana i `.sokratis/` dobiva brojke iz gita i prazna stanja
  za sve što traži konvenciju, nikad izmišljenu brojku ni grešku — popravak u jezgri (Ruling R21):
  zatvorena faza bez ijednog pogođenog commita se od sad ne upisuje ni u `Report.phases` (I8 je isto
  već popravio za pokazatelje, ne za tablicu faza). **Instalater postoji**: `npm run tauri build` daje
  NSIS za trenutnog korisnika (`currentUser`, bez administratorskih prava, S-029) — nepotpisan, bez
  auto-ažuriranja, samo za Leona. **Verzija ima jedan izvor** (`[workspace.package]` u korijenskom
  `Cargo.toml`, `1.0.0-pre.1`) — CLI i desktop dijele isti broj. **Razvojna baza je odvojena od
  instalirane**: `npm run tauri dev` sad piše u `sokratis-dev.db`, instalirana aplikacija u
  `sokratis.db` — razvoj više ne dira Leonovu pravu bazu.
  Brane nakon spajanja (merge `b560f7c`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **145 testova, 1 ignoriran, 0 palo** ·
  `npm run check` (svelte-check 212 datoteka/0 · i18n **162** ključa hr=en · kontrast 4/4 ·
  **vitest 82**) · `npm run build` OK · `sokratis docs .` 100/100 · `signals .` nema signala. Time je
  **etapa 1 „funkcija" (S-025) cjelovita u kodu** — preostaje da Leon instalira „1.0.0-pre".
- **SUČELJE-2 (T38–T42) u `main`-u (2026-09-23).** Korisnik dobiva **deseti pogled, Postavke**
  (S-028): tema · jezik · autostart · animacije, u JEDNOM mjestu — gornja traka više nema temu ni
  jezik. **Animacije** (S-026, `data-motion` na `<html>`, gasi ih prekidač ili
  `prefers-reduced-motion`): grafovi se otvaraju pri prvom prikazu (stupci rastu, linije i sparkline
  se iscrtavaju, prsten se otkriva), prijelaz pogleda traje 250 ms, stari sadržaj se prigušuje dok
  novi izvještaj stiže, a kostur stoji pri prvom učitavanju projekta. **Kartica s objašnjenjem**
  (S-027): klik na bilo koji graf ili brojku u bilo kojem od devet pogleda i u traci signala (37
  id-eva) otvara što podatak govori, kako je izračunat i kako ga čitati — „formula na klik" u
  Pokazateljima je time zamijenjena. Pregled je sad vidljiv i bez odabranog projekta (kao Postavke).
  Brane nakon spajanja (merge `6947189`): `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **145 testova, 1 ignoriran, 0 palo**
  (nepromijenjeno — Rust dirnut samo u `commands.rs`) · `npm run check` (svelte-check 235 datoteka/0 ·
  i18n **283** ključa hr=en, bilo 162 · kontrast 4/4 · **vitest 93**, 13 datoteka, bilo 82) ·
  `npm run build` OK (main.js 208,57 kB / gzip 53,66) · `sokratis docs .` 100/100 · `signals .` nema
  signala. Time je **etapa 2 „izgled" (S-025) cjelovita u kodu** — preostaje etapa 3 „izdanje" (S5).

## [0.1.0] — 2026-09-17 — Jezgra i CLI (Milestone 1)

**Prva verzija s kodom.** `sokratis` iz terminala čita git-povijest projekta i vraća statistiku rada,
ocjenu čistoće dokumentacije i signale smjera — svaki s dokazom. Bez ijedne postavke radi po
konvencijama Sokrat Studyja (S-005); `.sokratis/profile.json` ih pregazi. Što je izgrađeno i kako je
složeno: [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md).

### Što korisnik CLI-ja dobiva

- `sokratis report [putanja] [--since YYYY-MM-DD] [--json|--table]` — cijeli izvještaj: tempo po danu
  (commiti, kumulativ, redaka, sati, isporuke, deployi, testni redci), vrste rada, 18 pokazatelja
  kvalitete i brzine, faze, vizije, ocjena dokumentacije, signali. Bez zastavice ispisuje JSON,
  `--table` daje tablicu s hrvatskim natpisima. Izlazni kod 0.
- `sokratis docs [putanja] [--json]` — ocjena 0–100 s popisom nalaza (`datoteka:redak`) i kašnjenjem
  dnevnika za kodom. Projekt bez mape s dokumentacijom dobiva `null`, ne nulu. Izlazni kod 0.
- `sokratis signals [putanja] [--json]` — signali s dokazom; **izlazni kod je ugovor prema
  preflightu: 0 nema signala · 1 Warn · 2 Alert**. Greška okoline (nema `git`-a, putanja nije
  repozitorij, pokvaren profil) → poruka na stderr i **kod 3**.
- Radi iz korijena repoa, iz podmape i iz radnog stabla; ništa ne zapisuje u mjereni projekt.
- Ručni podaci putuju kroz git u `.sokratis/`: `profile.json` (konvencije projekta),
  `overrides.json` (vrsta rada po SHA-i), `visions.json`.

### Izmjereno nad prvim korisnikom (Sokrat Study, release build, 2026-09-17)

Jedino mjesto s brojkama dogfoodinga; dokument koji ih prepiše ostari.

- `report --table`: dotaknuto **190 commita, 70 164 redaka, 1801 izmjena datoteka, 0 preskočenih
  redaka**; 14 radnih dana od 2026-08-29; **105 isporuka** (7,5/dan), 13,6 commita/dan; **85 h**
  (proxy, 2,2 commita/h); 8922 testnih redaka (udio 0,127); 7 deploya; 22 debugging commita
  (udio 0,116); vrste rada: izvođenje 61 · vođenje dokumentacije 55 · planiranje 26 · poliranje 26 ·
  debugging 22 commita.
- `docs`: **100/100, 0 nalaza**, kašnjenje 0 dana → izlazni kod 0.
- `signals`: **ALERT `unmerged-branches`** s dokazom (`fix/kadar-nalicje` 11 dana / 12 commita
  ispred `main`-a, `feat/tinder-kadar` 8 dana / 34 commita) → **izlazni kod 2**, spreman za preflight.
- **Sokratis nad samim sobom** (vlastiti `.sokratis/profile.json`): docs **100/100, 0 nalaza**,
  **0 signala**, izlazni kod 0 — sve grane tokova M1 su spojene u `main`.

### Popravci nakon završne recenzije M1 (2026-09-17, jedan krug, 19 commita)

Recenzija cijelog lanca (3 Critical · 9 Important · 15 Minor) → **22 stavke riješene**, ponovna
recenzija SPOJIVO. Što korisnik CLI-ja time dobiva:

- **Tipfeler u datumu je greška s imenom polja, ne tiha kriva brojka.** `--since 2026-9-17`,
  `17.09.2026` ili `banana` prije su tiho promijenili prozor mjerenja (0 ili svi commiti); sada je to
  `since: datum mora biti oblika YYYY-MM-DD` i **izlaz 3**. Isto vrijedi za `since` i
  `closed_phases[].from/to` u profilu (poruka imenuje polje).
- **Regex iz profila bez capture-grupe više ne obara binarnu.** Valjan JSON s valjanim regexom bez
  grupe (npr. `plan_brick`) davao je paniku i izlaz 101; sada je greška koja kaže koje polje i koliko
  grupa treba → izlaz 3.
- **Pokazatelj „zatvorenih faza u razdoblju" sluša `--since`**, ne datum iz profila; prije je bio
  jedan od 18 pokazatelja koji je kriv kad god se `--since` razlikuje od profila.
- **Zaglavlje i podaci se odnose na isto razdoblje:** `--since` stariji od profila više se ne odrezuje
  (prozor dovlačenja je `min(profil, --since)`), a log se dovlači s rezervom od jednog dana pa brojke
  ne ovise o zoni stroja.
- **Zatvorena faza bez ijednog pogođenog commita se ne broji** ni u pokazateljima ni u tablici — tuđi
  projekt sa zadanim profilom više ne dobiva faze iz zraka.
- **Pogrešna uporaba CLI-ja daje 3, ne 2** (2 je rezerviran za Alert); `--help`/`--version` daju 0.
  `--json` i `--table` su **isključivi** — prije je `--json --table` tiho ispisao JSON.
- **Poruka o pokvarenom profilu se ispisuje jednom** (bila je dvaput, ~1,6 kB s popisom svih polja),
  s normaliziranom putanjom.
- **Repozitorij bez commita dobiva rečenicu** (`repozitorij nema commita`), ne gitov savjet o `--`.
- Sati u praznom rasponu su `0.0`, ne `-0.0`; tablica piše „izmjena datoteka" (ne „datoteka").
- Iznutra: `serde_json` u jezgri je dev-ovisnost, `insta` je **uklonjen** (neiskorišten, pravilo #6),
  formula kašnjenja docs-a živi na jednom mjestu, `docs.rs` je bez `expect()`, test pariteta sam
  tvrdi da pokazatelja ima 18. **`cargo test --workspace` = 64 testa, 0 padova** (bilo 47).
- Brojke nad Sokrat Studyjem (gore) su **nepromijenjene** — nijedan popravak nije pomaknuo mjeru.

### Dodano

- 2026-09-17 — **Dokumentacija projekta** po modelu Sokrat Studyja: `CLAUDE.md`, `README.md`, `docs/`
  (product · plan · workflow · records), spec `ARHITEKTURA_M1.md` (danas u `docs/archive/`), odluke
  S-001…S-010. Bez koda.
- 2026-09-17 — **Plan implementacije M1** (22 cigle, 8 tokova, testovi i kod po koraku) i **agenti**
  (graditelj · recenzent · čuvar dokumentacije) s protokolom nadzora `docs/workflow/AGENTI.md`. Bez koda.
- 2026-09-17 — **M0: Rust toolchain na stroju** — Visual Studio Build Tools (MSVC) + rustup stable
  (`stable-x86_64-pc-windows-msvc`); `cargo build`/`cargo test`/`cargo fmt`/`cargo clippy` sada rade.
- 2026-09-17 — **T1: kostur workspacea** (`main`) — Cargo workspace triju crateova (`sokratis-core` bez
  I/O-a, `sokratis-io`, binarna `sokratis-cli`), ugovor tipova jezgre (`model.rs`, `profile.rs`,
  `error.rs`) i zadani profil (konvencije Sokrat Studyja, S-005) na mjestu; sve preostale cigle su
  potpisane, ali `todo!()`. `cargo test` zeleno (3 smoke-testa).
- 2026-09-17 — **T2: fixture pariteta** — pravi git log, dnevnik i plan sa `main`-a Sokrat Studyja
  (371 commit od 2026-08-02) + očekivane brojke (`expected.json`: 14 dana, 5 vrsta rada, 18 pokazatelja,
  11 faza, 185 commita od 2026-08-29) u `crates/sokratis-core/tests/fixtures/`, uključujući poznati kvar
  S-007 (dva negativna dana u starom `RAD.xlsx`-obračunu) koji Sokratis mora ispraviti, ne prenijeti.
- 2026-09-17 — **PARSE (T3–T6) u `main`-u** — parser git loga (371 commit fixturea, `skipped_lines` 0),
  klasifikator vrste i podvrste rada (redoslijed planiranje > dokumentacija > debugging > poliranje,
  ostalo izvođenje; provjeren nad 12 stvarnih naslova commita), parser dnevnika (paritet 105/105
  isporuka po danu s `PROGRESS.md`) i parser plana (paritet 7/7 faza s `RASPORED.md`).
- 2026-09-17 — **DOCS+PRAVILA (T12–T14) u `main`-u** — `docs_health` sa sedam provjera (mrtva
  poveznica koja preskače ograde kôda, dokument nije u indeksu, više/nijedan aktivan plan, dnevnik
  unutar definicije, kašnjenje dnevnika za kodom, proračun ključnih datoteka), ocjena 100 minus zbroj
  težina; pravila `unmerged-branches` i `docs-lag`, oba s dokazom u signalu.
- 2026-09-17 — **IO (T15–T17) u `main`-u** — `GitCli` čita git kroz proces (log zadane grane, grane s
  udaljenošću od `main`-a, radna stabla, zadnja promjena putanje); `Project` otvara repo iz podmape ili
  radnog stabla, čita profil (nepoznato polje = greška), ručne podatke (`overrides.json`,
  `visions.json`) i docs s vremenom zadnje promjene; provjereno nad Sokrat Studyjem: 56 docs, 31 grana,
  4301 redak loga. `--since` sada šalje puni dan (S-011) — ispravlja kvar `rad-xlsx.py` gdje je
  `git log --since` bez sata ovisio o dobu dana pokretanja.
- 2026-09-17 — **FIXTURE T2b u `main`-u** — `expected.json` regeneriran za S-011 (`--since` s punim
  danom); dodan `README.md` fixturea koji objašnjava razliku prema staroj snimci.
- 2026-09-17 — **CLI (T18–T19) u `main`-u** — naredbe `sokratis report [path] [--since] [--json|
  --table]`, `sokratis docs [path]` i `sokratis signals [path]` sada rade nad pravim repozitorijem;
  izlazni kod je ugovor prema preflightu (0 nema signala, 1 Warn, 2 Alert, 3 greška); tablični ispis
  ima hrvatske natpise, identifikatori u JSON-u ostaju engleski (S-008).
- 2026-09-17 — **METRIKE (T7–T11) u `main`-u** — sati rada po `author_time` (S-007, ispravak
  negativnih sati zbog cherry-pickova), tempo po danu, vrste rada s ručnim overrideom po SHA, faze
  (planirane i zatvorene, dosljedno S-011) i svih 18 pokazatelja iz `RAD.xlsx`.
- 2026-09-17 — **FIXTURE T2c u `main`-u** — `expected.json` dobio `hours_fixed`: sati preračunati
  Python-generatorom sa sortiranjem po autoru daju paritet 14/14 dana s Rustom (85,0 h umjesto
  −139,2 h u staroj tablici).
- 2026-09-17 — **INTEGRACIJA (T20–T21) u `main`-u** — `sokratis report` sada vraća cjelovit
  izvještaj (dani, vrste, pokazatelji, faze, docs-ocjena, signali) sastavljen u jednom koraku;
  test pariteta s `RAD.xlsx` (`crates/sokratis-core/tests/parity.rs`) zelen nad cijelim fixtureom —
  paritet je test, ne tvrdnja.
- 2026-09-17 — **INTEGRACIJA (T22) u `main`-u** — Sokratis mjeri **sam sebe**: vlastiti
  `.sokratis/profile.json` (plan je `ROADMAP.md`, nema zatvorenih faza, oznaka cigle `M1/N`, testovi
  u `crates/*/tests/`). `cargo build --release` i sve tri naredbe pokrenute release binarkom nad
  Sokrat Studyjem, isključivo čitanjem — brojke gore. `cargo test --workspace` tada **47 testova, 0
  padova** (nakon kruga popravaka 64 — vidi gore).
