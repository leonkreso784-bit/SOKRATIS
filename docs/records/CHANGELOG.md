# Changelog — Sokratis

Format: [Keep a Changelog](https://keepachangelog.com/) · Verzioniranje: [SemVer](https://semver.org/).
Prva verzija s kodom bit će 0.1.0 (M1). Isporuka = ono što je u `main`-u; sesije su u `PROGRESS.md`.

## [Unreleased] — rad u tijeku (cilj: 0.1.0 = M1)

### Dodano
- 2026-09-17 — **Dokumentacija projekta** po modelu Sokrat Studyja: `CLAUDE.md`, `README.md`, `docs/`
  (product · plan · workflow · records), aktivni spec `docs/plan/ARHITEKTURA_M1.md`, odluke S-001…S-010.
  Bez koda.
- 2026-09-17 — **Plan implementacije M1** (22 cigle, 8 tokova, testovi i kod po koraku) i **agenti**
  (graditelj · recenzent · čuvar dokumentacije) s protokolom nadzora `docs/workflow/AGENTI.md`. Bez koda.
- 2026-09-17 — **M0: Rust toolchain na stroju** — Visual Studio Build Tools (MSVC) + rustup stable
  (`stable-x86_64-pc-windows-msvc`); `cargo build`/`cargo test`/`cargo fmt`/`cargo clippy` sada rade.
- 2026-09-17 — **T1: kostur workspacea** (`main`) — Cargo workspace triju crateova (`sokratis-core` bez
  I/O-a, `sokratis-io`, binarna `sokratis-cli`), ugovor tipova jezgre (`model.rs`, `profile.rs`,
  `error.rs`) i zadani profil (konvencije Sokrat Studyja, S-005) na mjestu; sve preostale cigle su
  potpisane, ali `todo!()`. `cargo test` zeleno (3 smoke-testa). CLI naredbe iz `CLAUDE.md` su ugovor,
  još ne rade — dolaze u T18–T19.
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
- 2026-09-17 — **METRIKE (T7–T10) na grani `feat/core-metrics`, još ne u `main`-u** — sati po
  `author_time` (S-007): 0 negativnih dana na fixtureu, 33/37 dana identično `RAD.xlsx`-u, razlika samo
  oko tri poznata cherry-picka.
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
  test pariteta s `RAD.xlsx` zelen nad cijelim fixtureom. Dogfooding nad Sokrat Studyjem: 14 dana,
  190 commita, 105 isporuka, 85 h, docs 100/100, signal `unmerged-branches` ALERT za dvije
  nespojene grane starije od praga.
