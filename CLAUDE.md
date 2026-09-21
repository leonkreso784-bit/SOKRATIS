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
  `crates/sokratis-store` (SQLite) i `apps/desktop/src-tauri` (crate `sokratis-desktop`) — **kostur
  cijel u `main`-u** (`M2/1a`+`M2/1b`); desktop crate je u `[workspace] members` i builda se.
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
   i javi se.** **Push na `origin`: Leon je 2026-09-20 dao TRAJNI OK** — orkestrator sam pusha `main`
   nakon svakog spajanja toka (tek kad pune brane prođu) i grane tokova na kraju sesije (sigurnosna
   kopija). **I dalje traži Leonov izričit OK:** force-push, brisanje grane na remoteu, novi remote,
   promjena vidljivosti, izdanje/tag. Repo je JAVAN (S-023) — tajne se traže PRIJE commita.
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
`cargo run -p sokratis-cli -- report <putanja> [--since YYYY-MM-DD] [--until YYYY-MM-DD] [--json|--table]` ·
`… docs <putanja>` · `… signals <putanja>` (izlazni kod 0 nema · 1 Warn · 2 Alert · 3 greška **ili
pogrešna uporaba**; `--help`/`--version` = 0) — sve rade nad pravim repozitorijem (potvrđeno nad
Sokrat Studyjem).
Testovi i brane: `docs/workflow/TESTING.md`.

## Stanje — TRENUTNO (2026-09-21 — M1 zatvoren; M2 u izvedbi, šest od devet tokova u `main`-u, SUČELJE jedino nespojeno; sesija stala zbog usagea, sljedeća je PLANIRANJE)
- **M0 gotov. M1 zatvoren** (verzija 0.1.0, u `main`-u). Cijeli lanac radi nad pravim repozitorijem:
  `sokratis report/docs/signals` čitaju git kroz `io`, jezgra računa dane, sate, vrste rada, faze, 18
  pokazatelja, docs-ocjenu i signale. Paritet s `RAD.xlsx` je test. Brojke i popravci:
  `CHANGELOG.md` (0.1.0); što je izgrađeno i **što još ne radi**: `docs/architecture/ARCHITECTURE.md`
  (§11); što čeka M2: `docs/records/BACKLOG.md`.
- **M2 (desktop):** spec `docs/plan/ARHITEKTURA_M2.md` (Leonov OK 2026-09-18, S-012…S-022), plan cigli
  `docs/superpowers/plans/2026-09-18-m2-desktop.md` (35 cigli, 9 tokova, vlasništvo datoteka i
  ovisnosti među tokovima na jednom mjestu). **Šest tokova su gotovi i spojeni u `main`:** KOSTUR (T1)
  · JEZGRA (T2–T7) · PROFIL (T8–T9) · STORE (T15–T18) · IO (T10–T14 + M2/14b potrošač keša, cigla
  izvan plana; dug I9 zatvoren u cijelosti) · CLI (T19, `report --until`) — svi preduvjeti za DESKTOP
  osim sučelja su ispunjeni. Brane na `main`-u nakon zadnjeg spajanja koda: fmt · clippy `--workspace
  --all-targets` · **136 testova, 1 ignoriran** (mjerni test) · `signals .` 0. Brojke, presude po
  toku i odstupanja od plana: `PROGRESS.md`.
- **SUČELJE (stablo `sokratis.ui`, grana `feat/ui`) je JEDINI nespojeni tok:** M2/20–M2/27 su
  vizualno potvrđene u pregledniku (M2/26 i M2/27 uz po jedan krug popravka); **M2/28 (Dnevnik ·
  Isporuke · Vizije · Dokumentacija) nije započeta**, ništa od sučelja nije u `main`-u. DESKTOP
  (T29–T33, tray-znak iz `apps/desktop/src/assets/intro/graph.webp`, Leonov izbor 2026-09-20) i
  INTEGRACIJA (T34–T35) čekaju sučelje, nisu započeti.
- **Sesija je stala jer je Leonu ponestalo usagea — ne zato što je tok gotov ili je milestone stigao
  do zastanka.** Sedam stabala je na disku (`git worktree list`); spojene grane (jezgra · profil ·
  store · io · cli) i njihova stabla ostaju dok Leon ne da izričit OK za brisanje; LICENCE i taga nema.
- **Leonov zapis namjere za izgled, dodatke i put do 1.0.0 (2026-09-21): `docs/product/PLAN_DESIGNE.md`**
  — NIJE spec. **Sljedeća sesija je PLANIRANJE** (rez za 1.0.0, odluke u `DECISIONS.md`, tek onda
  spec/plan), NE nastavak gradnje T28. Prva radnja nove sesije: `git log --oneline -15` ·
  `git worktree list`, pa ledger `.superpowers/sdd/2026-09-18-m2-desktop/progress.md` odjeljak
  „▶▶▶ STANJE ZA NOVU SESIJU" na dnu i `NOVA-SESIJA-PROMPT.md` pored njega (prepisan 2026-09-21 za
  planiranje), pa `docs/README.md`.
- **Repo je od 2026-09-20 JAVAN na GitHubu** (`origin` = `leonkreso784-bit/SOKRATIS`, Leonov OK,
  S-023) uz **TRAJNI OK za pusheve** (pravilo #1): orkestrator pusha `main` nakon spajanja i grane na
  kraju sesije bez pitanja; force-push, brisanje grane na remoteu, tag i vidljivost i dalje traže
  izričit OK. Svaki novi fixture ili dokument je javna objava — tajne se traže prije commita.
- Toolchain je pinan (`rust-toolchain.toml`, 1.98.1). Agenti definirani
  (`.claude/agents/{graditelj,recenzent,cuvar-dokumentacije}.md`), protokol `docs/workflow/AGENTI.md`
  (orkestrator = glavna sesija, jedini spaja u `main`).
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
samo na prijelaz u Alert · **S-021** HR/EN od M2 · **S-022** snapshot `Report`-a prva cigla M2 ·
**S-023** repo javan na GitHubu od 2026-09-20 (Leonova odluka, prije kraja M2); tajne se traže prije commita.

## Agenti — više grana, jedan orkestrator
Uloge i protokol: `docs/workflow/AGENTI.md`. Graditelj radi **jednu ciglu u svom stablu**, recenzent presuđuje
u dva prolaza, čuvar dokumentacije piše zapise; **samo orkestrator spaja u `main`**. Nakon compacta stanje se
čita iz gita (`git log --oneline -15` · `git worktree list`), ne iz sjećanja. **Grane i stabla tokova M1 su
obrisane 2026-09-18** (uz Leonov OK, sve spojene). Otvoreno je **sedam stabala tokova M2**
(JEZGRA · PROFIL · IO · STORE · CLI · SUČELJE); **JEZGRA, PROFIL, STORE, IO i CLI su gotovi u cijelosti**
(T2–T7, T8–T9, T15–T18, T10–T14+M2/14b i T19 spojeni u `main`; IO time zatvorio dug I9 u cijelosti i
dobio keš potrošača), **SUČELJE je jedino nespojeno** — M2/20–M2/27 su vizualno potvrđene u pregledniku
(M2/26 i M2/27 uz po jedan krug popravka), M2/28 nije započeta, spajanje slijedi nakon nje. Ostala
(DESKTOP · INTEGRACIJA) otvaraju se kasnije po ovisnostima iz plana — popis i vlasništvo datoteka su u
planu M2, ne ovdje.

## Dokumentacija — ulaz je SAMO `docs/README.md`
Složena **po ulozi dokumenta** (kao Sokrat Study): `product/` ŠTO · `plan/` ŠTO SADA (najviše **jedan**
aktivni spec + ROADMAP; aktivan je `docs/plan/ARHITEKTURA_M2.md`) · `architecture/` **ŠTO JE IZGRAĐENO**
(`docs/architecture/ARCHITECTURE.md` — granice crateova, tok podataka, sva polja profila, formati
`.sokratis/*.json`, izlazni kodovi) · `workflow/` KAKO RADIMO · `records/` POVIJEST ·
`archive/` ispunjeni specovi — oboje **nikad izvor istine**. Ne traži fajlove napamet — otvori indeks.
