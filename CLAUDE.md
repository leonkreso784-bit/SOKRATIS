# CLAUDE.md — Sokratis (ključni kontekst)

> Ovaj fajl se učitava SVAKU sesiju. **Drži ga sažetim — samo TRENUTNO stanje, pravila i pointeri.**
> Dnevnik: `docs/records/PROGRESS.md` · odluke: `docs/records/DECISIONS.md` · ulaz u docs: `docs/README.md`.

> **👤 Jedan autor: Leon Kreso.** Piše hrvatski. Cilj projekta je i **učenje Rusta** — kod se piše
> tako da ga Leon može pročitati i razumjeti (pravilo #5). Ja pišem, Leon čita i odlučuje.

## Što je projekt
Desktop aplikacija koja se priključi na git-repozitorij projekta, izračuna statistiku rada (ono što
danas radi `docs/records/RAD.xlsx` u Sokrat Studyju), ocijeni čistoću dokumentacije i javi smjer
projekta **s dokazom**. Prati više projekata. Izgled kao Sokrat Study. Repo je javan (S-023); licenca je neodlučena.
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
   kopija). **Grane i stabla koji su potpuno spojeni u pushani `main` i čisti** orkestrator briše sam, lokalno i
   na remoteu (Leon, 2026-09-21) — nakon provjere (`git branch --merged main`, `merge-base --is-ancestor`).
   **I dalje traži Leonov izričit OK:** force-push, brisanje NESPOJENE grane, novi remote,
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

## Stanje — TRENUTNO (2026-09-24 — drugi rez do 1.0.0: izvedbe sesija 1–3 gotove)
- **M0 gotov, M1 zatvoren** (0.1.0); etape 1+2 M2 cjelovite u kodu (svih devet tokova + SUČELJE-2).
  Leon je nakon instalacije tražio veće promjene (`docs/product/NALAZI_LEON_2026-09-23.md`) →
  brainstorming dao **drugi rez do 1.0.0**: S-032…S-037.
- **Aktivan spec `docs/plan/ARHITEKTURA_1_0.md`** (spec M2 arhiviran, `docs/archive/ARHITEKTURA_M2.md`).
  **Aktivan plan `docs/superpowers/plans/2026-09-24-1-0-0-grane-i-ploca.md`**: T44–T63, šest tokova
  (DESKTOP-2 · JEZGRA-2 · IO-2 · GRAFOVI · PLOČA · IZDANJE), pet sesija; tok IZDANJE preuzima T35/T43
  iz plana M2 (ostaje u `superpowers/plans/`, nije arhiviran).
- **Sesije 1–3 gotove:** DESKTOP-2 (T44–T45) · JEZGRA-2 (T46–T48) · GRAFOVI (T53–T55) · IO-2 (T49–T50)
  spojeni, vrh `main`-a `9a0e98f`, verzija u kodu `1.0.0-pre.3` (Leon nije stigao instalirati pre.2).
  Mjerenje po zadanom profilu sad ide preko **svih lokalnih grana** (S-032), dnevnik je unija svih
  radnih stabala (S-033). **Sesija 4 slijedi:** T51 (CLI `--scope`) · T52 (mjerenje procesa/trajanja) ·
  T56–T57 (GRAFOVI kraj) · T58 (PLOČA — tok kreće).
- **Dva stabla:** `sokratis` (`main`) i `sokratis.rel` (`feat/release`, T35 napola, necommitano, ne
  dirati do toka IZDANJE). Tag traži Leonov izričit OK.
- Druga verzija (M3) i kasnije: `docs/records/BACKLOG.md`.
- Brane, brojke i tijek sesija: `PROGRESS.md`; isporuke: `CHANGELOG.md`. Ovaj odjeljak to ne ponavlja.

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
iz loga izmjeren, Tailwind kroz Vite plugin · **S-018** grafovi vlastiti SVG (dio „bez biblioteke" ukinut
S-035: d3-matematika ulazi, izgled ostaje naš) · **S-019** animacija jednom po pokretanju, prozor čeka,
preskočiva · **S-020** tray minimizira, autostart, jedna instanca, obavijest samo na prijelaz u Alert
(dio „X sakriva, tray" ukinut S-036: X pita pa izlazi, tray uklonjen) · **S-021** HR/EN od M2 ·
**S-022** snapshot `Report`-a prva cigla M2 ·
**S-023** repo javan na GitHubu od 2026-09-20 (Leonova odluka, prije kraja M2); tajne se traže prije commita.
**[rez za 1.0.0]** **S-024** izlaz iz M2 = 1.0.0, 0.2.0 se preskače · **S-025** crta reza i tri etape (funkcija →
izgled → izdanje, „1.0.0-pre“ nakon prve) · **S-026** animacije ≤ 250 ms, bez biblioteke, `data-motion="off"` ·
**S-027** kartica s objašnjenjem je statična (`explain.<id>.what|how|read`) · **S-028** tema · jezik · autostart ·
animacije u pogledu Postavke · **S-029** instalater NSIS samo za Leona · **S-030** README engleski, `docs/`
hrvatski · **S-031** dokumentacija: točnost se čuva smanjivanjem, ispunjeno ide u `archive/`.
**[drugi rez do 1.0.0]** **S-032** metrike nad svim lokalnim granama, oznaka grane po commitu ·
**S-033** dnevnik = unija radnih stabala, plan/`docs/` iz vodećeg stabla · **S-034** klik na karticu →
nadzorna ploča projekta · **S-035** grafovi: d3-matematika + naš SVG · **S-036** X = upit → izlaz, tray
uklonjen, autostart otvara prozor · **S-037** sve prije 1.0.0, instalater „1.0.0-pre.N" nakon svake
sesije.

## Agenti — više grana, jedan orkestrator
Uloge i protokol: `docs/workflow/AGENTI.md`. Graditelj radi **jednu ciglu u svom stablu**, recenzent presuđuje
u dva prolaza, čuvar dokumentacije piše zapise; **samo orkestrator spaja u `main`**. Nakon compacta stanje se
čita iz gita (`git log --oneline -15` · `git worktree list`), ne iz sjećanja. **Otvorena su dva stabla** (v. „Stanje"):
`sokratis` (`main`) i `sokratis.rel` (`feat/release`, T35 napola, necommitano) — potpuno spojena se
brišu (pravilo #1), ali `sokratis.rel` NIJE spojeno, pa ga nova sesija ne dira ni ne briše, samo
nastavlja T35 ondje kad tok IZDANJE dođe na red. Popis tokova i vlasništvo datoteka:
`docs/superpowers/plans/2026-09-24-1-0-0-grane-i-ploca.md` (T44–T63); T35/T43 su u starijem
`docs/superpowers/plans/2026-09-18-m2-desktop.md`.

## Dokumentacija — ulaz je SAMO `docs/README.md`
Složena **po ulozi dokumenta** (kao Sokrat Study): `product/` ŠTO · `plan/` ŠTO SADA (najviše **jedan**
aktivni spec + ROADMAP; aktivan je `docs/plan/ARHITEKTURA_1_0.md`) · `architecture/` **ŠTO JE IZGRAĐENO**
(`docs/architecture/ARCHITECTURE.md` — granice crateova, tok podataka, sva polja profila, formati
`.sokratis/*.json`, izlazni kodovi) · `workflow/` KAKO RADIMO · `records/` POVIJEST ·
`archive/` ispunjeni specovi — oboje **nikad izvor istine**. Ne traži fajlove napamet — otvori indeks.
