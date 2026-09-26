# Changelog — Sokratis

Format: [Keep a Changelog](https://keepachangelog.com/) · Verzioniranje: [SemVer](https://semver.org/).
Isporuka = ono što je u `main`-u; sesije su u `PROGRESS.md`.

## [Unreleased] — rad u tijeku

**Milestone 2 (desktop) je u izvedbi** po odobrenom specu
[`../archive/ARHITEKTURA_M2.md`](../archive/ARHITEKTURA_M2.md) (odluke S-012…S-022) i planu od 35 cigli;
**drugi rez do 1.0.0** (S-032…S-037) nastavlja po
[`../plan/ARHITEKTURA_1_0.md`](../plan/ARHITEKTURA_1_0.md).
**Ovaj odjeljak izlazi kao 1.0.0, ne 0.2.0** (S-024; rez i dopuna speca §13 od 2026-09-21).
**Na disku je `main` verzije `1.0.0-pre.4`** (jedan izvor: `[workspace.package]` u korijenskom
`Cargo.toml`, M2/37, S-037) — netagirano, neobjavljeno; tag traži Leonov izričit OK na kraju etape
izdanja (S-025). Status: [`../plan/ROADMAP.md`](../plan/ROADMAP.md) · tijek sesije:
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
- **`M2/44`–`M2/45` — konzola bez bljeska, X = upit → izlaz, tray uklonjen (2026-09-24, DESKTOP-2,
  S-036).** Korisnik desktop aplikacije više ne vidi bljesak konzole `git.exe` pri svakom osvježenju
  (kvar 4 iz Leonovih nalaza) — svaki poziv gita sad ide kroz jedino mjesto `git_command()`, koje na
  Windowsu postavlja `CREATE_NO_WINDOW`. X na prozoru više ne zatvara aplikaciju izravno: pokazuje upit
  „Zatvoriti Sokratis? Nadzor projekata i obavijesti staju dok ga ponovno ne pokreneš." s dva izbora
  (fokus na „Odustani", Esc = odustani); potvrda zove novu naredbu `quit`. **Tray-izbornik i
  tray-ikona su uklonjeni** (nema više „Osvježi sve"/kvačice za autostart u trayu) — autostart ostaje
  postavka u pogledu Postavke, funkcija seli u vlastitu datoteku `autostart.rs`. Svako pokretanje
  (uklj. iz autostarta) je otad nov proces, pa se animacija pokretanja prikazuje **svaki put**, ne samo
  prvi. Poznato ograničenje: „osvježenje bez bljeska konzole" se ne da izmjeriti u `tauri dev` (debug
  build dijeli konzolu s roditeljem) — dokaz je test koji čita izvor + Leonova ručna provjera na
  instaliranoj verziji.
- **`M2/46`–`M2/47` — metrike po grani, dnevnik kao unija stabala (2026-09-24, JEZGRA-2, S-032,
  S-033).** `Report` dobiva dva nova polja: `scope` (`"all"`/`"default"`, koje su grane ušle u
  mjerenje) i `branches[]` (redak po grani: commiti, redci, sati po udjelu commita tog dana — proxy,
  zbroj po granama je jednak ukupnom — i je li spojena); `commits[].branch` bilježi na koju je granu
  commit dospio. CLI-tablica ih još ne ispisuje (dolazi s T51). Jezgra sad zna unirati dnevnik iz
  **više** radnih stabala (`ReportInput.diaries`, bilo `diary`): unija po (datum, naslov), prvi viđeni
  (vodeće stablo) pobjeđuje; `touched` dobiva `worktrees`/`diaries`. **`io` danas i dalje šalje samo
  jedan tekst dnevnika i praznu kartu grana** (privremeno, dok IO-2 ne stigne T49/T50) — mjerenje je
  zato u `1.0.0-pre.2` i dalje samo nad zadanom granom, iako je ugovor prema sučelju već spreman.
  **SNAPSHOT NAMJERNO PROMIJENJEN** (S-022): `+scope: "default"`, `+branches` (main 190 commita /
  70164 redaka / 85,0 h / spojena), `+branch: "main"` u svih 190 redaka commita, `+touched.worktrees:
  1`, `+touched.diaries: 1`; sva ostala polja snimke ostaju jednaka.
- **`M2/53` — temelji grafova: d3-matematika, naš SVG (2026-09-24, GRAFOVI, S-035, Leonov OK).**
  Korisnik i dalje ne vidi ništa novo (nitko još ne uvozi ove datoteke, `main.js` nepromijenjen,
  tree-shaken) — priprema: četiri nove, pinane npm-ovisnosti (`d3-scale` · `d3-shape` · `d3-array` ·
  `d3-time-format` + `@types`) i dvije nove datoteke, `lib/charts/scales.ts` (UTC datumski ticksi,
  HR/EN nazivi mjeseci/dana) i `lib/charts/bucket.ts` (grupiranje po danu/tjednu/mjesecu, doba dana) —
  matematika osi dolazi iz d3, izgled i boje ostaju naši (S-018 dopunjena, ne poništena).
- **Bump verzije `1.0.0-pre.2` (2026-09-24, S-037).** Instalater nakon prve izvedbene sesije drugog
  reza (T44–T47, T53); `Cargo.toml` je izvor broja verzije, `package.json`/lockovi ga zrcale (samo
  redak verzije, bez `npm install`).
  Brane nakon sve tri spojene cigle (`d949163` → `9204512` → `cedbf20`): `cargo fmt --check` OK ·
  `cargo clippy --workspace --all-targets -- -D warnings` OK · `cargo test --workspace` **0 padova, 1
  ignoriran** (`sokratis-core`: **63** unit + `tests/branches.rs` **4** + `tests/diary_union.rs`
  **2** + `tests/parity.rs` **1** + `tests/snapshot.rs` **1**; `io`/`store`/`cli`/`desktop` zeleni) ·
  `npm run check` (svelte-check **0** · `check:i18n` **287** ključa hr=en, bilo 283 · kontrast 4/4 ·
  **vitest 109**, **15** datoteka, bilo 93) · `npm run build` `main.js` **215,01 kB** / gzip **54,22
  kB** · `sokratis docs .` 100/100 · `signals .` nema signala.
- **`M2/48` — `branch_scope` u profilu (2026-09-24, JEZGRA-2, S-032, dopuna S-005).** Korisnik CLI-ja
  ne vidi ništa novo (`io` ga još ne čita — dolazi s T49). `Profile` dobiva `branch_scope: "all" |
  "default"`, **zadano `"all"`**: zadano mjerenje projekta više nije samo zadana grana. Nepoznata
  vrijednost pada s porukom koja imenuje `all`/`default`. Profil ulazi u kanonski JSON dnevne snimke
  (`engine.rs::try_snapshot`) — prva snimka nakon nadogradnje dobiva nov `profile_seen_id` (trend
  „profil promijenjen", bez koda koji bi to iscrtao).
- **`M2/54` — temelji grafova s osima (2026-09-24, GRAFOVI, S-035).** Korisnik i dalje ne vidi ništa
  novo (komponente još nitko ne uvozi). Nove čiste funkcije `lib/charts/layout.ts` (okvir i margine,
  grupirani/naslagani stupci nad `scaleBand`, X-ticksi prorijeđeni na ≤ 10, linija s datumskom
  skalom, najbliža točka za tooltip) i pet tankih komponenata `Chart`/`Axis`/`Grid`/`Legend`/`Tooltip`
  (potonja izvan SVG-a, pozicionirana u postocima).
- **`M2/49`–`M2/50` — grane i radna stabla stvarno ulaze u mjerenje (2026-09-24, IO-2, S-032,
  S-033).** Korisnik CLI-ja i dalje ne vidi novu zastavicu (`--scope` dolazi u T51), ali mjerenje se
  sad ravna po profilu: `branch_scope: "all"` (zadano) šalje `git log --branches` i puni kartu
  `commit_sources` (`sha → grana`, `%h|%S` **izvan** zadane grane preko `git log --branches --not
  <default>`), `"default"` vraća staro ponašanje s praznom kartom. `GitSource` dobiva
  `worktree_heads()`/`commit_times()` — **vodeće stablo** (`Project::lead`) je ono s najnovijim
  `author_time` na HEAD-u, ne nužno glavno; dnevnik je sad unija SVIH radnih stabala s diska (redom od
  vodećeg, necommitani unos vidljiv), a plan/`docs`/zadnja promjena datoteke čitaju se iz vodećeg
  stabla. Ručni podaci se i dalje pišu SAMO u glavno stablo (S-015, nepromijenjeno). `touched.
  worktrees`/`touched.diaries` više nisu uvijek 1. **Granica broja git-procesa po izvještaju
  podignuta 5 → 8** (izmjereno 7: 4 osnovna + 1 `commit_sources` + 2 za vodeće stablo).
- **`M2/55` — grafovi Bars/Line na temeljima, `scale.ts` uklonjen (2026-09-24, GRAFOVI, S-035).**
  `Tempo` sad crta stupce i kumulativnu liniju kroz `<Bars>`/`<Line>`: datumi na X-osi, mreža,
  legenda (kad ima više od jednog niza), tooltip na najbližem stupcu/točki — miš ili tipkovnica
  (svaki stupac je fokusabilan). `Ring`/`Sparkline` nepromijenjenih svojstava, sad nad `scales.ts`.
  Stari `lib/charts/scale.ts` i njegovi testovi **obrisani** — `linear`/`finiteMax`/`arcPath`/
  `ringSegments`/`svgA11y` sele u `scales.ts`. **`main.js` 215,01 → 258,67 kB (gzip 54,22 → 70,57)** —
  d3 prvi put stvarno ulazi u snop (tree-shaking gotov, komponente ga sad uvoze). Prekidač
  dan/tjedan/mjesec dolazi u T59.
  Brane nakon sva četiri spajanja (`5e5db15` → `0bc70cf` → `368eec5` → `2fcecf1`): `cargo fmt --check`
  OK · `cargo clippy --workspace --all-targets -- -D warnings` OK · `cargo test --workspace` **0
  padova** (`sokratis-core`: 65 unit + `branches` 4 + `diary_union` 2 + `parity` 1 + `snapshot` 1;
  `sokratis-io`: **52** + 1 ignoriran) · `npm run check` (svelte-check **248** datoteka 0/0 ·
  `check:i18n` **287** ključa hr=en · kontrast 4/4 · **vitest 112**, **15** datoteka — `scale.test.ts`
  nestao, `layout.test.ts` ušao) · `npm run build` `main.js` **258,67 kB** / gzip **70,57 kB** ·
  `sokratis docs .` 100/100 · `signals .` nema signala. Vizualni dimni test Tempa (vite + preglednik,
  MockApi): datumi na osi, mreža, tooltip, fokusabilni stupci — prošao.
- **Bump verzije `1.0.0-pre.3` (2026-09-24, S-037).** Instalater nakon treće izvedbene sesije drugog
  reza (T48–T50, T54–T55); `Cargo.toml` je izvor broja verzije, `package.json`/lockovi ga zrcale
  (samo redak verzije, bez `npm install`). Leon nije stigao instalirati `1.0.0-pre.2` (registar je
  još pokazivao pre.1) — pre.3 zamjenjuje obje.
- **`M2/51` — CLI `--scope`, zaglavlje tablice imenuje mjereni doseg (2026-09-26, IO-2, S-032).**
  Korisnik CLI-ja dobiva `sokratis report --scope all|default`: `all` (zadano) mjeri sve lokalne
  grane, `default` samo `default_branch` iz profila (paritet s `RAD.xlsx`); nepoznata vrijednost je
  pogrešna uporaba (izlazni kod 3). `--table` zaglavlje sad imenuje mjereni doseg —
  `opseg: sve lokalne grane (N grana) · zadana: main` ili `opseg: samo zadana grana (main)` — umjesto
  dosadašnjeg `grana main · N commita`, koje je od T49 tvrdilo krivi doseg (ime zadane grane uz
  brojku SVIH grana; nalaz vanjske analize 2026-09-25). Nova sekcija GRANE (iza VRSTE RADA,
  preskočena bez commita) ispisuje `Report.branches` s hrvatskim paucalom (1 grana · 2–4 grane ·
  5+ grana). `--scope` pregazi `profile.branch_scope` **u memoriji** — datoteka profila se ne dira.
- **`M2/52` — mjerenje procesa/trajanja nad Sokrat Studyjem (2026-09-26, IO-2).** Korisnik CLI-ja ne
  vidi ništa novo — `crates/sokratis-io/tests/perf.rs` dobio drugi test,
  `measure_real_repo_both_scopes` (`#[ignore]`, ručno pokretanje: `SOKRATIS_MEASURE_REPO=<putanja>
  cargo test -p sokratis-io --test perf -- --ignored --nocapture`). **Kanonsko mjesto brojki (S-010).**
  Mjereno nad radnim stablom `sokratstudy.f6`, ne `sokratstudy.dev`: glavna mapa je od 2026-09-26
  03:17 postala `core.bare = true` (Leonova okolina, izvan Sokratisa) — `Project::open` na bare mapu
  pada, `.f6` dijeli isti `git-common-dir`/refove pa je isti projekt (S-015); nalaz i posljedice u
  `records/BACKLOG.md`.

  | mjera | `all` | `default` |
  |---|---|---|
  | git-procesa (`input()`) | 7 (≤ 8) | 6 (≤ 8) |
  | trajanje `input()` | 4,47 s | 1,19 s |
  | trajanje CLI-ja (`report --json`) | 3,03 s | 2,07 s |
  | `touched.commits` | 339 | 190 |
  | `touched.worktrees` / `touched.diaries` | 5 / 5 | 5 / 5 |
  | broj grana | 10 | 1 |
  | isporuke | 128 | 128 |
  | zadnji dan | 2026-09-25 | 2026-09-13 |
  | docs.score | 100 | 100 |

  Deset grana (opseg `all`, sortirano commiti↓): `main` 190 c / 85,3 h · `feat/f6-mcp` 58 / 18,9 h ·
  `feat/tinder-kadar` 29 / 12,0 h · `feat/f3-dvojezicnost` 25 / 14,5 h · `feat/f2-mail` 9 / 3,2 h ·
  `feat/f2-tema-racun` 9 / 3,8 h · `feat/f2-zid` 9 / 3,2 h · `fix/kadar-nalicje` 6 / 1,9 h ·
  `feat/f2-slike` 3 / 1,0 h · `feat/f2-zid-radionica` 1 / 0,4 h — sve osim `main` nespojene.
- **`M2/56` — temelji grafova Heatmap/Histogram (2026-09-26, GRAFOVI, S-035).** Korisnik i dalje ne
  vidi ništa novo (nitko još ne uvozi). `layout.ts` += `heatmapLayout` (ćelija za svaki dan raspona,
  stupac = ISO tjedan, 5 razina) i `histogramLayout` (24 bina, doba dana); `scales.ts` +=
  `formatMonth`/`formatWeekday`; `Chart` dobio opcionalni prop `m` (margine po grafu, R59); nove
  `Heatmap`/`Histogram`. Razine toplinske karte su `.heat-0…4` u `app.css` — jedna boja (`brand-500`)
  razlikovana samo `fill-opacity`-em (0,3–1), jedini niz monoton na sve četiri teme (R57).
- **`M2/57` — temelji grafova Gantt/HBars (2026-09-26, GRAFOVI, S-035).** Korisnik i dalje ne vidi
  ništa novo. `layout.ts` += `ganttLayout` (faze od-do, „danas" kao crta) i `hbarsLayout` (grane,
  spojene prigušene); nove `Gantt`/`HBars` (boje kroz postojeće tokene `--color-ok` /
  `--color-brand-500` / `--color-ink-2`, R60; stanje faze bez datuma kroz postojeći `phaseState`).
  `lib/charts/` sad ima temelje + **8 komponenata** (`Bars`/`Line`/`Ring`/`Sparkline`/`Heatmap`/
  `Histogram`/`Gantt`/`HBars`); nitko izvan `lib/charts` ih još ne uvozi (T59/T60).
- **`M2/58` — nadzorna ploča projekta (2026-09-26, PLOČA, S-034, + popravak 1).** Korisnik dobiva
  novo ponašanje: klik na karticu projekta u Pregledu otvara pogled **Projekt** — osam sekcija
  (Sažetak, Tempo, Grane, Vrste rada, Faze, Isporuke, Pokazatelji, Dokumentacija) na jednoj dugoj
  stranici, s ljepljivim skok-izbornikom iznad sidara. Gornja traka gubi birač projekta (`<select>`);
  u projektu pokazuje „‹ Pregled" + ime projekta. Šest bivših pogleda preseljeno `git mv` u
  `views/project/*Section.svelte` (sadržaj isti, samo uvozi i `<h1>`→`<h2>`); dvije nove sekcije:
  **Sažetak** (šest kartica: commiti, sati, radni dani, isporuke, docs-ocjena, signali — kroz
  postojeće `<Explainable>` id-eve) i **Grane** (tablica `Report.branches`, natpis „Mjerena je samo
  zadana grana." kad je opseg `default`). `selectProject`/`top.project` obrisani (bez pozivatelja
  nakon T58, R58). i18n **287 → 304** ključa. **Popravak 1** (isti dan, vizualni dimni test): sidra
  sekcija dobivaju `scroll-margin-top` da ih ljepljivi izbornik ne prekrije nakon skoka; kartice
  Sažetka razmaknute u šest stupaca tek od `xl` (1280 px, ne `lg`) da najdulji tekst signala stane bez
  vodoravnog klizača.

  Brane nakon sve tri spojene cigle (`6501deb` IO-2 → `e656457` GRAFOVI → `dd5c3d4` PLOČA): `cargo fmt
  --check` OK · `cargo clippy --workspace --all-targets -- -D warnings` OK · `cargo test --workspace`
  **0 padova** (`sokratis-cli`: 4 unit + 11 integracijskih; `sokratis-io`: 52 + **2** ignorirana —
  `perf`/`cache`; `sokratis-core`: 65 unit + 4 + 2 + 1 + 1; `sokratis-store` zelen) · `npm run check`
  (svelte-check **256** datoteka 0/0 · `check:i18n` **304** ključa hr=en, bilo 287 · kontrast 4/4 ·
  **vitest 125**, **15** datoteka, bilo 112) · `npm run build` `main.js` **265,59 kB** / gzip
  **72,01 kB** (bilo 258,67 kB / gzip 70,57 kB) · `sokratis docs .` 100/100 · `signals .` nema
  signala. Vizualni dimni test ploče (vite + Playwright nad `MockApi`, **bez Tauri-ja**): Pregled →
  kartica → ploča s 8 sekcija, skok-izbornik, „‹ Pregled", zasivljene stavke bez projekta — prošao
  nakon popravka 1.
- **Bump verzije `1.0.0-pre.4` (2026-09-26, S-037).** Instalater nakon četvrte izvedbene sesije
  drugog reza (T51–T52, T56–T58); `Cargo.toml` je izvor broja verzije, `package.json`/lockovi ga
  zrcale (samo redak verzije, bez `npm install`).

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
