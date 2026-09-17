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
