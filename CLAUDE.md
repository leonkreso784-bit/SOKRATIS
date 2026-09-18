# CLAUDE.md — Sokratis (ključni kontekst)

> Ovaj fajl se učitava SVAKU sesiju. **Drži ga sažetim — samo TRENUTNO stanje, pravila i pointeri.**
> Dnevnik: `docs/records/PROGRESS.md` · odluke: `docs/records/DECISIONS.md` · ulaz u docs: `docs/README.md`.

> **👤 Jedan autor: Leon Kreso.** Piše hrvatski. Cilj projekta je i **učenje Rusta** — kod se piše
> tako da ga Leon može pročitati i razumjeti (pravilo #5). Ja pišem, Leon čita i odlučuje.

## Što je projekt
Desktop aplikacija koja se priključi na git-repozitorij projekta, izračuna statistiku rada (ono što
danas radi `docs/records/RAD.xlsx` u Sokrat Studyju), ocijeni čistoću dokumentacije i javi smjer
projekta **s dokazom**. Prati više projekata. Izgled kao Sokrat Study. Kasnije open-source na GitHubu.
Definicija: `docs/product/PRD.md` · što je izgrađeno: `docs/architecture/ARCHITECTURE.md` ·
milestonei: `docs/plan/ROADMAP.md` · ispunjeni spec M1: `docs/archive/ARHITEKTURA_M1.md`.

**Prvi korisnik je Sokrat Study** (`C:\Users\leonk\Documents\sokratstudy.dev` + radna stabla
`sokratstudy.f21` · `.f22` · `.f25` · `.f3` = **jedan** projekt). Referentna implementacija za paritet:
`sokratstudy.dev/scripts/rad-xlsx.py` (Python, generira `RAD.xlsx`). Njezin poznati kvar — negativni
sati zbog cherry-pickova — se u Sokratisu **ispravlja, ne prenosi** (S-007).

## Stack
- **Rust** (stable, MSVC target, edition 2024) · Cargo workspace: `crates/sokratis-core` (čisti, bez
  I/O-a) · `crates/sokratis-io` (git, datoteke, profil) · `crates/sokratis-cli` (binarna `sokratis`) ·
  [M2 spec] `crates/sokratis-store` (SQLite) · `apps/desktop/src-tauri` (crate `sokratis-desktop`).
- **Desktop [M2]:** Tauri 2 (`apps/desktop`) · sučelje **Svelte 5 + Tailwind v4** + `tokens.css`
  prenesen iz Sokrat Studyja (4 teme, zadana svijetla „Akademsko plavo"). Znak Sokratisa je **nov**.
- **Pohrana:** ručni podaci u `<repo>/.sokratis/` (profil · overridei · vizije) · [M2] SQLite u
  `%LOCALAPPDATA%\sokratis\`. **Bez oblaka, bez računa.**

## Arhitektura (najvažnije)
- **`core` ne otvara datoteke i ne zove procese** — prima tekst i strukture, vraća `Report` (S-002).
  Sve što treba git ili disk živi u `io`. Zato se core testira bez gita i isti je u CLI-ju i Tauriju.
- **git kroz proces** (`std::process::Command`) iza traita `GitSource` (S-003); `gix` je kandidat kasnije.
- **Profil projekta** `.sokratis/profile.json` deklarira konvencije (dnevnik · plan · regexi · testne
  putanje · pragovi). **Zadano = konvencije Sokrat Studyja** (S-005) → prvi korisnik radi bez konfiguracije.
- **Metrike se računaju nad zadanom granom** (paritet s tablicom); grane izvan nje ulaze **samo u signale**.
- **Identifikatori su engleski i stabilni** (`work_kind = "debugging"`); natpisi HR/EN daje sučelje (S-008).
- **Signali** = `trait Rule`; jedno pravilo = jedna datoteka + jedan test; svaki signal nosi **dokaz**.
- **Mjerač kaže koliko je dotaknuo:** svaki `Report` nosi `touched` (commita · redaka · datoteka · preskočeno).

## ⚠️ KRITIČNA PRAVILA
1. **Jedna cigla = jedan commit.** Zahvat koji raste preko cigle se reže. Na kraju milestonea **STANI
   i javi se.** Objava na GitHub i svaki push na `main` (kad remote bude postojao) = **Leonov izričit OK**;
   nijedno ranije odobrenje se ne proteže na sljedeće.
2. **Prije commita:** `cargo fmt --check` · `cargo clippy --all-targets -- -D warnings` · `cargo test`.
   Crveno ne ide u commit.
3. **Uvijek ažuriraj `docs/`** nakon izmjene: `PROGRESS.md` (sesija) + `CHANGELOG.md` (isporuka) +
   tematski dokument. **Jedna činjenica, jedno mjesto** (S-010): duplikat se briše, ne sinkronizira;
   brojka prepisana u prozu ostari — pokaži na izvor.
4. **Mjeri prije nego popravljaš.** Rub koji prepoznaš odmah dobiva test; zapisan a nepokriven rizik
   je uredno dokumentiran propust.
5. **„Zašto Rust ovako":** svaka cigla u zaglavlju datoteke nosi 2–5 redaka o Rust-konstruktu koji
   uvodi i zašto baš njega; pojmovnik raste u `docs/workflow/RUST.md`. **Kod koji Leon ne može
   pročitati nije gotov.**
6. **Ovisnosti se pinaju** — i toolchain (`rust-toolchain.toml`). `Cargo.lock` se commita; nova
   ovisnost je namjerna radnja s obrazloženjem u commitu, nikad nuspojava. Popis dopuštenih crateova
   i zašto: `docs/workflow/RUST.md`.
7. **Test-prvo** za parsere, metrike i pravila: fixture (tekst) → očekivano → implementacija.
   Paritet s `RAD.xlsx` je **test**, ne tvrdnja (`docs/workflow/TESTING.md`).
8. **PRIJE SVAKOG COMPACTA:** proći sve `.md` (root + `docs/**`) i ispraviti zastarjelo.

## Komande
`cargo build` · `cargo test` · `cargo clippy --all-targets -- -D warnings` · `cargo fmt` ·
`cargo run -p sokratis-cli -- report <putanja> [--since YYYY-MM-DD] [--json|--table]` ·
`… docs <putanja>` · `… signals <putanja>` (izlazni kod 0 nema · 1 Warn · 2 Alert · 3 greška **ili
pogrešna uporaba**; `--help`/`--version` = 0) — sve rade nad pravim repozitorijem (potvrđeno nad
Sokrat Studyjem).
Testovi i brane: `docs/workflow/TESTING.md`.

## Stanje — TRENUTNO (2026-09-18 — M1 zatvoren, spec M2 odobren, plan M2 napisan, izvedba počinje)
- **M0 gotov. M1 zatvoren.** Kod isporučen, recenziran i popravljen (T1–T22 + krug popravaka), verzija
  0.1.0, u `main`-u. Cijeli lanac radi nad pravim repozitorijem: `sokratis report/docs/signals` čitaju
  git kroz `io`, jezgra računa dane, sate, vrste rada, faze, 18 pokazatelja, docs-ocjenu i signale, CLI
  ih ispisuje kao JSON ili tablicu s hrvatskim natpisima. Test pariteta s `RAD.xlsx`
  (`crates/sokratis-core/tests/parity.rs`) zelen. Sokratis mjeri i sam sebe (`.sokratis/profile.json`).
  Brojke, popravci i broj testova: `CHANGELOG.md` (0.1.0); tijek sesije: `PROGRESS.md`; što je
  izgrađeno i **što još ne radi**: `docs/architecture/ARCHITECTURE.md` (§11); što čeka M2:
  `docs/records/BACKLOG.md`.
- **Leon je 2026-09-18 dao izričit OK: svih 8 grana `feat/*` i njihovih 8 radnih stabala
  `sokratis.<tok>` je obrisano** (sve su bile spojene). `main` je sada **jedina grana i jedino
  stablo** (`git worktree list` · `git branch -a`). Time je M1 zatvoren.
- **Na Leona i dalje čeka** (ne radi se bez njegova izričitog OK-a): push/remote/objava na GitHub
  (privatni remote prvo; javna objava tek nakon ograde putanja iz profila, BACKLOG I9). Toolchain je
  pinan (`rust-toolchain.toml`, 1.98.1) i definicije agenata su u repou — obje odluke Leon je odobrio
  2026-09-18.
- **Spec M2 je napisan (2026-09-18): `docs/plan/ARHITEKTURA_M2.md`** — iz brainstorminga s Leonom,
  trinaest odluka zapisano kao **S-012…S-022** u `DECISIONS.md`; preuzima 7 od 9 stavki duga M1
  (BACKLOG drži još dvije i pointere). Leonove datoteke za znak: `C:\Users\leonk\Downloads\sokratis
  logo .png` (ikona) · `download.png` (zaključak `S◍KRATIS`) · `sokratis-intro-clean-graph.html`
  (animacija 4,2 s, canvas, dva WebP-a) — spec §5.2–5.4 ih preuzima doslovno.
  **Leon je spec odobrio 2026-09-18** („Imaš moj OK"). **Plan cigli je napisan:**
  `docs/superpowers/plans/2026-09-18-m2-desktop.md` — 35 cigli u 9 tokova (KOSTUR T1 · JEZGRA T2–T7 ·
  PROFIL T8–T9 · IO T10–T14 · STORE T15–T18 · CLI T19 · SUČELJE T20–T28 · DESKTOP T29–T33 ·
  INTEGRACIJA T34–T35), vlasništvo datoteka i ovisnosti među tokovima u planu (jedno mjesto). Ledger:
  `.superpowers/sdd/2026-09-18-m2-desktop/progress.md`. Izvedba agentima po `AGENTI.md`.
- **M0 za M2 izmjeren** (spec §10): Node 24 · npm 11 · WebView2 · MSVC · Rust 1.98.1 ✅; `cargo tauri`
  nije potreban globalno (`@tauri-apps/cli` je dev-ovisnost). **Jedina instalacija: `npm install` u
  `apps/desktop` — čeka Leonov OK**, kao svaka instalacija u M0.
- **Prva radnja nove sesije:** `git log --oneline -15` · `git worktree list` · ledger
  `.superpowers/sdd/2026-09-17-m1-jezgra-i-cli/progress.md`, odjeljak „STANJE ZA NOVU SESIJU" na dnu
  (operativna uputa, ne izvor činjenica o projektu), pa `docs/README.md`.
- **Agenti definirani:** `.claude/agents/{graditelj,recenzent,cuvar-dokumentacije}.md`, praćene u
  repou od 2026-09-18; protokol nadzora `docs/workflow/AGENTI.md` (orkestrator = ova sesija, jedini
  spaja u `main`). Radna stabla tokova trenutno ne postoje — otvaraju se opet kad plan M2 odredi svoje
  tokove.
- Što je isporučeno i kada zna `CHANGELOG.md`; tijek sesija `PROGRESS.md`. Ovaj odjeljak to ne ponavlja.

## Ključne odluke — samo žive
Puni tekst: `docs/records/DECISIONS.md`. **S-001** Rust · **S-002** core bez I/O-a · **S-003** git kroz
proces iza traita · **S-004** ručni podaci u `.sokratis/` u repou · **S-005** zadano = Sokrat Study, profil
pregazi · **S-006** sučelje web (Svelte 5), ne Rust GUI · **S-007** sati po `author_time`, sortirano ·
**S-008** engleski identifikatori u jezgri · **S-009** SQLite tek u M2 · **S-010** jedna činjenica, jedno
mjesto · **S-011** `--since` računa cijeli dan, ne goli datum · **[M2 spec]** **S-012** Tauri ugovor =
`Report` nepromijenjen, mjerenje u `core`, oblikovanje u Svelteu · **S-013** nov crate `sokratis-store`,
`desktop` bez logike · **S-014** SQLite = istina koju git ne zna + pogodnost (snimke brojki, keš sirovih
commita tek nakon mjerenja) · **S-015** projekt = `git-common-dir`, ručni podaci u glavno stablo, Sokratis
ne commita · **S-016** watcher u `io`, odgoda 600 ms, bez petlje · **S-017** `tokens.css` cijel, `brand-*`
iz loga izmjeren, Tailwind kroz Vite plugin · **S-018** grafovi vlastiti SVG · **S-019** animacija jednom
po pokretanju, prozor čeka, preskočiva · **S-020** tray minimizira, autostart, jedna instanca, obavijest
samo na prijelaz u Alert · **S-021** HR/EN od M2 · **S-022** snapshot `Report`-a prva cigla M2.

## Agenti — više grana, jedan orkestrator
Uloge i protokol: `docs/workflow/AGENTI.md`. Graditelj radi **jednu ciglu u svom stablu**, recenzent presuđuje
u dva prolaza, čuvar dokumentacije piše zapise; **samo orkestrator spaja u `main`**. Nakon compacta stanje se
čita iz gita (`git log --oneline -15` · `git worktree list`), ne iz sjećanja. **Grane i stabla tokova M1 su
obrisane 2026-09-18** (uz Leonov OK, sve spojene); trenutno postoji samo `main`. Nove grane/stabla se otvaraju
kad plan M2 odredi svoje tokove.

## Dokumentacija — ulaz je SAMO `docs/README.md`
Složena **po ulozi dokumenta** (kao Sokrat Study): `product/` ŠTO · `plan/` ŠTO SADA (najviše **jedan**
aktivni spec + ROADMAP; spec za M2 tek dolazi) · `architecture/` **ŠTO JE IZGRAĐENO**
(`docs/architecture/ARCHITECTURE.md` — granice crateova, tok podataka, sva polja profila, formati
`.sokratis/*.json`, izlazni kodovi) · `workflow/` KAKO RADIMO · `records/` POVIJEST ·
`archive/` ispunjeni specovi — oboje **nikad izvor istine**. Ne traži fajlove napamet — otvori indeks.
