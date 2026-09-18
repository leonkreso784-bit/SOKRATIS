# Changelog — Sokratis

Format: [Keep a Changelog](https://keepachangelog.com/) · Verzioniranje: [SemVer](https://semver.org/).
Isporuka = ono što je u `main`-u; sesije su u `PROGRESS.md`.

## [Unreleased] — rad u tijeku

**Milestone 2 (desktop) je u izvedbi** po odobrenom specu
[`../plan/ARHITEKTURA_M2.md`](../plan/ARHITEKTURA_M2.md) (odluke S-012…S-022) i planu od 35 cigli.
Status: [`../plan/ROADMAP.md`](../plan/ROADMAP.md) · tijek sesije:
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
