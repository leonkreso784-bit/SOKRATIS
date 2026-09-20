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
`cargo run -p sokratis-cli -- report <putanja> [--since YYYY-MM-DD] [--json|--table]` ·
`… docs <putanja>` · `… signals <putanja>` (izlazni kod 0 nema · 1 Warn · 2 Alert · 3 greška **ili
pogrešna uporaba**; `--help`/`--version` = 0) — sve rade nad pravim repozitorijem (potvrđeno nad
Sokrat Studyjem).
Testovi i brane: `docs/workflow/TESTING.md`.

## Stanje — TRENUTNO (2026-09-18 večer — M1 zatvoren; M2 u izvedbi, JEZGRA+PROFIL u `main`-u, IO/STORE/SUČELJE recenzirani dalje u granama; sesija stala jer je predugo trajala, ne jer je M2 gotov)
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
- **Repo je od 2026-09-20 JAVAN na GitHubu** — `origin` = `leonkreso784-bit/SOKRATIS`, Leonov izričit
  OK i njegova odluka da bude javan (S-023: što je time postalo javno, pretraga na tajne, posljedice).
  Pushani su `main` i tri nespojene grane tokova (`feat/io-m2` · `feat/store` · `feat/ui`). **Svaki
  novi fixture ili dokument je od sada javna objava — tajne se traže PRIJE commita.** io-dio ograde
  putanja (I9, cigla T14) je time dug prema javnom kodu i ima prednost; licence još nema (BACKLOG).
- **Na Leona i dalje čeka** (ne radi se bez njegova izričitog OK-a): brisanje grana i radnih stabala
  tokova M2 na kraju milestonea (lokalno i na remoteu). Pushevi više NE čekaju — trajni OK, pravilo #1. **Tray-znak za T33 je riješen:** Leon je 2026-09-20 odabrao
  da se radi iz `apps/desktop/src/assets/intro/graph.webp` (znak bez lika iz njegove animacije) — ne
  čeka se nikakav novi PNG. Toolchain je pinan (`rust-toolchain.toml`, 1.98.1) i definicije agenata
  su u repou — obje odluke Leon je odobrio 2026-09-18.
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
- **T1 (kostur) je cijel u `main`-u — `M2/1a` + `M2/1b`:** ugovor tipova, `sokratis-store` kostur,
  `apps/desktop` datoteke i ovisnosti pinane (`M2/1a`), pa `npm install` uz Leonov OK (80 paketa, 0
  ranjivosti), ikone iz Leonova loga (`npm run tauri icon`; `android/`·`ios/` nisu commitani —
  desktop-only) i desktop crate natrag u `[workspace] members` (`M2/1b`). Odstupanje od plana,
  zapisano u commitu: `apps/desktop/vite.config.ts` uvozi `defineConfig` iz `vitest/config` umjesto
  `node:url` (nova ovisnost samo radi config-datoteke nije opravdana, pravilo #6). Brane: fmt ·
  clippy `--workspace` · **65 testova** · `npm run check` zelen · `npm run build` daje `dist/` s oba
  HTML-a · `signals .` 0.
- **Pet radnih stabala tokova otvoreno:** `sokratis.jezgra` (`feat/core-m2`) · `sokratis.profil`
  (`feat/core-profile`) · `sokratis.io` (`feat/io-m2`) · `sokratis.store` (`feat/store`) ·
  `sokratis.ui` (`feat/ui`). **Tok PROFIL je gotov** (T8–T9 spojeno, merge `a2e9265`): ograda putanja
  iz profila (`inside_root`/`Profile::validate_paths`, jezgreni dio duga I9 — io-dio čeka IO T14) i
  `test_path_exclude`. **Tok JEZGRA je gotov** (T2–T7, merge `7d23c76` pa `7711a67`): snapshot ugovora
  `Report`-a (dug I7, riješen), `until` kao gornja granica razdoblja (S-011 — `io`/CLI mu još ne šalju
  stvarnu vrijednost, T14/T19), zbroj vizija po stanju (dug I6, riješen), redci commita i isporuke u
  `Report`-u (`commits`/`deliveries` više nisu uvijek `[]`, hrane buduće poglede Dnevnik/Isporuke),
  aktivne faze preko `phase_tag` iz profila (dug I3+M14, riješen), `SnapshotMetrics`/`diff`/
  `worst_severity`/`alerts_raised` u `snapshot.rs`. **Nalaz recenzije, namjerno neriješen ovim
  tokom:** `metrics/indicators.rs` i dalje klasificira dio commita odvojeno od `commit_rows` — spec
  §3.2 „jednom" nije dovršeno; otvoreno do završne recenzije M2 (`docs/records/BACKLOG.md`). Brane na
  `main`-u nakon svih spajanja: fmt · clippy `--workspace` · **82 testa** · `signals .` 0.
  **Ostala tri toka su otišla dalje u svojim granama, ali NIŠTA od ovoga nije spojeno u `main`**
  (brojke i presude: `PROGRESS.md`, dopuna „Nastavak iste večeri"): IO (`feat/io-m2`) — M2/10–M2/13
  (atomarno pisanje, performanse gita — mjerenje ≥ prag 500 ms → T18 po planu zato **ulazi** —,
  detached HEAD, watcher S-016) sve recenzirano SPOJIVO; M2/14 (io-dio ograde I9, `--until` prema
  gitu) nije započeta. STORE (`feat/store`) — M2/15–M2/17 (registar, postavke, snimke uz jedan krug
  popravka: snimka dana je cjelovita zamjena) sve recenzirano SPOJIVO; M2/18 (keš, uvjetna cigla) nije
  započeta. SUČELJE (`feat/ui`) — M2/20–M2/24 (tokeni, HR/EN, `format.ts`, SVG grafovi, splash — svaka
  uz najviše jedan krug popravka) sve recenzirano SPOJIVO; M2/25–M2/28 (okvir s `api.ts`, pogledi)
  nisu započete; otvorena je još orkestratorova ručna provjera puta greške splasha u pregledniku
  (bez commita). Stabla CLI/DESKTOP/INTEGRACIJA otvaraju se kasnije po ovisnostima iz plana; tray-znak
  za T33 radi se iz Leonova `graph.webp` (gore). **Rust-brane u stablima tokova rade bez desktop cratea**
  (`--workspace --exclude sokratis-desktop`); pune brane s desktopom vrti orkestrator na `main`-u
  nakon svakog spajanja — [`docs/workflow/AGENTI.md`](docs/workflow/AGENTI.md) §5.
- **Sesija je večeras stala jer je predugo trajala (Leonova odluka), ne zato što je M2 gotov ili što
  je došao zastanak na kraju milestonea.** Nastavak nije nova analiza nego spajanje gornja tri toka u
  `main`, redom kako im ovisnosti dopuštaju. **Prva radnja nove sesije:** `git log --oneline -15` ·
  `git worktree list`, pa ledger `.superpowers/sdd/2026-09-18-m2-desktop/progress.md`, odjeljak
  „STANJE ZA NOVU SESIJU" **na dnu datoteke** (operativno stanje po cigli, dispatch-napomene i svi
  odgođeni Minor nalazi žive **samo ondje**, S-010 — ovaj dokument ih ne ponavlja), pa `docs/README.md`.
- **M0 za M2 izmjeren** (spec §10): Node 24 · npm 11 · WebView2 · MSVC · Rust 1.98.1 ✅; `cargo tauri`
  nije potreban globalno (`@tauri-apps/cli` je dev-ovisnost). Jedina instalacija (`npm install` u
  `apps/desktop`) je odrađena 2026-09-18 uz Leonov OK.
- **Agenti definirani:** `.claude/agents/{graditelj,recenzent,cuvar-dokumentacije}.md`, praćene u
  repou od 2026-09-18; protokol nadzora `docs/workflow/AGENTI.md` (orkestrator = ova sesija, jedini
  spaja u `main`).
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
obrisane 2026-09-18** (uz Leonov OK, sve spojene). Sad je otvoreno **pet od devet stabala tokova M2**
(JEZGRA · PROFIL · IO · STORE · SUČELJE); **JEZGRA i PROFIL su gotovi** (T2–T7 i T8–T9 spojeni u
`main`), **IO/STORE/SUČELJE su recenzirani do svoje zadnje cigle (SPOJIVO), ali nespojeni** —
spajanje je prva radnja nove sesije. Ostala (CLI · DESKTOP · INTEGRACIJA) otvaraju se kasnije po
ovisnostima iz plana — popis i vlasništvo datoteka su u planu M2, ne ovdje.

## Dokumentacija — ulaz je SAMO `docs/README.md`
Složena **po ulozi dokumenta** (kao Sokrat Study): `product/` ŠTO · `plan/` ŠTO SADA (najviše **jedan**
aktivni spec + ROADMAP; aktivan je `docs/plan/ARHITEKTURA_M2.md`) · `architecture/` **ŠTO JE IZGRAĐENO**
(`docs/architecture/ARCHITECTURE.md` — granice crateova, tok podataka, sva polja profila, formati
`.sokratis/*.json`, izlazni kodovi) · `workflow/` KAKO RADIMO · `records/` POVIJEST ·
`archive/` ispunjeni specovi — oboje **nikad izvor istine**. Ne traži fajlove napamet — otvori indeks.
