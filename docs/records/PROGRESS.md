# Progress Log — Sokratis

Dnevnik rada. Najnoviji unos na vrhu. Svaka sesija: što je napravljeno, što je provjereno, što
slijedi. **Format naslova je ugovor:** `## YYYY-MM-DD (MODEL) — naslov` — Sokratis ga sam parsira
(dogfooding), pa se ne mijenja bez promjene zadanog profila.

---

## 2026-09-17 (FABLE) — Analiza Sokrat Studyja, odluka o arhitekturi (Rust + Tauri 2), dokumentacija projekta

**Folder `sokratis` bio je prazan. Ništa od koda; sve je dizajn i dokumentacija. Git repo inicijaliziran.**

### Što je analizirano
- `sokratstudy.dev` (main) + četiri radna stabla `.f21` · `.f22` · `.f25` · `.f3`; `docs/records/RAD.xlsx`
  i njezin generator `scripts/rad-xlsx.py`; dnevni zadatak `scripts/rad-dnevno.ps1` (Task Scheduler, 23:45);
  `check-docs.js`; `css/tokens.css`; formati `PROGRESS.md`, `RASPORED.md`, `HISTORY.md`, `CHANGELOG.md`, `BUGS.md`, `DECISIONS.md`.

### Nalazi (izmjereno, ne procijenjeno)
- **Tablica je aplikacija bez sučelja:** generator čita git log, naslove dnevnika i redove plana;
  ručni su samo stupac „vrsta (ručno)" po SHA i list Vizije.
- **Negativni sati su kvar proxyja:** razmak se računa po datumu autora, a redoslijed dolazi po datumu
  commita; cherry-pickovi `5233a0a`, `5da7119`, `1e2d157` daju −42 h, −159 h, −22 h → Sažetak −144,1 h.
  Provjereno u gitu (`%ad` vs `%cd`).
- **Dnevni zapis na `main`-u tiho stari:** skripta na `main`-u ne commita (dnevnik: „commit PRESKOČEN"
  od 14.09.), jedan pad s Tracebackom 16.09. 04:31.
- **Dokumenti već imaju „API"** (naslov dnevnika, redak cigle, oznaka faze u commitu) → postaju profil projekta.
- **Docs-čistoća već ima definiciju** u `check:docs`/`check:state` → generalizira se + kašnjenje docs-a za kodom.
- **Tokeni su prenosivi** (4 teme, izmjeren kontrast, sistemski grotesk, indigo marka, žuti marker).
- Na stroju: Node 24.11, Python 3.11, git 2.52, WebView2 153; **nema Rusta, nema .NET-a, nema `gh`**.

### Odluke (Leon)
Rust (želja za novim jezikom; C++ odbijen zbog ekosustava, UTF-8 i Tauri-ja) · Tauri 2 · Svelte 5 s
tokenima Sokrat Studyja · ručni podaci u `.sokratis/` u repou · prvi signali nespojene grane +
kašnjenje docs-a · novi znak. Zapisano kao S-001…S-010 u `DECISIONS.md`.

### Isporučeno (samo dokumentacija)
`CLAUDE.md` · `README.md` · `.gitignore` · `docs/README.md` · `product/PRD.md` ·
`plan/ARHITEKTURA_M1.md` (aktivni spec) · `plan/ROADMAP.md` · `workflow/TESTING.md` · `workflow/RUST.md` ·
`records/PROGRESS.md` · `records/CHANGELOG.md` · `records/DECISIONS.md` · `records/BACKLOG.md`.
Namjerno **ne** postoje još: `architecture/`, `BUGS.md`, `HISTORY.md`, `archive/`, `ideas/`, `LICENSE` — nastaju kad imaju sadržaj.

### Isporučeno (drugi dio sesije, nakon Leonova OK-a na spec)
- **Plan M1** `docs/superpowers/plans/2026-09-17-m1-jezgra-i-cli.md`: 22 cigle, svaka s testom, kodom i
  commit-porukom; 8 tokova s vlasništvom datoteka bez preklapanja (KOSTUR · FIXTURE · PARSE · METRIKE ·
  DOCS+PRAVILA · IO · CLI · INTEGRACIJA). Ugovor tipova i cijeli zadani profil (Sokrat Study) su u T1.
- **Odluke u planu koje spec dopunjuju:** git format dobiva `%ad` i `%cd` (dan = autor kao tablica, `since` =
  commit kao git); `Context` u vlasništvu, jedini lifetime je `IndicatorInput<'a>`; `chrono` samo u `io`;
  `WorkKind::id()`; podvrsta se računa, u izvještaj ulazi u M2.
- **Agenti:** `.claude/agents/graditelj.md` · `recenzent.md` · `cuvar-dokumentacije.md` + protokol
  `docs/workflow/AGENTI.md` (Leon: „više agenata na više branča a ti ih kontroliraš i nadzireš").
- Spec §2.1/§2.2, TESTING §3, RUST §1/§2, docs/README, CLAUDE.md usklađeni s planom.

### Isporučeno (treći dio sesije, nakon compacta — M0, T1, T2)
- **M0 toolchain gotov**, uz Leonov OK: `winget install Microsoft.VisualStudio.2022.BuildTools` s
  workloadom VCTools (MSVC 14.44.35207, Windows SDK 10.0.26100) + `winget install Rustlang.Rustup` →
  `rustup default stable` = rustc/cargo 1.98.1 stable-x86_64-pc-windows-msvc, rustfmt 1.9.0, clippy 0.1.98.
  Prazan crate `cargo test` zelen. `%USERPROFILE%\.cargo\bin` na PATH (nove ljuske ga vide bez ručnog exporta).
- **T1 KOSTUR** spojen (commit 8de5866, merge 4f91227): Cargo workspace (`sokratis-core` · `sokratis-io` ·
  `sokratis-cli`), ugovor tipova `model.rs`/`profile.rs`/`error.rs`, stubovi svih preostalih cigli
  (`todo!("cigla M1/N")`), 3 smoke-testa. Brane na `main` zelene. Recenzent (opus): SPOJIVO bez nalaza.
- **T2 FIXTURE** spojen (commit 687db7a, merge 9cb6fcf): fixture pariteta sa `main`-a Sokrat Studyja
  @ 090bd0c u `crates/sokratis-core/tests/fixtures/sokratstudy-2026-09-17.*`. Recenzent: SPOJIVO bez nalaza.
  Chore b90152c: `.gitignore` iznimka `!crates/sokratis-core/tests/fixtures/*.log` (fixture-log ne smije
  biti sakriven pravilom koje ignorira `*.log`). Brojke fixture-a: `CHANGELOG.md`.
- **Odluke orkestratora:** `WorkKind::id()` uveden već u T1 (plan ga imao u T20) jer ga T19 i T21 trebaju
  prije njega; PARSE (`sokratis.parse`) se spaja djelomično nakon T6 jer METRIKE (T10) treba pravi
  `classify_kind`, ne stub.
- **Tokovi otvoreni i u tijeku u radnim stablima:** `sokratis.parse` (`feat/core-parse`, T3→T6→T4→T5) ·
  `sokratis.metrics` (`feat/core-metrics`, T7→T11) · `sokratis.rules` (`feat/core-rules`, T12→T14) ·
  `sokratis.io` (`feat/io`, T15→T17); `sokratis.cli` (`feat/cli`, T18→T19) čeka slobodno mjesto (limit
  4 graditelja istodobno).

### Isporučeno (četvrti dio sesije — PARSE, DOCS+PRAVILA i IO spojeni u `main`, nalaz S-011)
- **PARSE (T3, T6) pa (T4, T5)** spojeno (merge 1e33c3f, pa 7daaba8): parser git loga (371 commit
  fixturea, `skipped_lines` 0), klasifikator vrste/podvrste (redoslijed planiranje > dokumentacija >
  debugging > poliranje, ostalo izvođenje; provjeren nad 12 stvarnih naslova = paritet s Pythonom),
  parser dnevnika (paritet 105/105 isporuka po danu), parser plana (paritet 7/7 faza). Recenzije: sve
  SPOJIVO bez nalaza.
- **DOCS+PRAVILA (T12–T14)** spojeno (merge 04398b3): `docs_health` sa sedam provjera (dead-link
  preskače ograde kôda, not-indexed, multiple/no-active-plan, diary-in-definition, docs-lag,
  key-file-budget; ocjena 100 minus zbroj težina, `saturating_sub`), pravila `unmerged-branches` i
  `docs-lag` s dokazom u signalu. Recenzije: T12 SPOJIVO; T13 vraćen jednom (zaglavlje opisivalo
  Ord/max na `Severity` koji u datoteci ne postoji → ispravljeno); T14 SPOJIVO. Napomena recenzenta za
  T20: `docs_lag.rs` i `docs.rs::lag()` računaju isti razmak neovisno jedno o drugom — pogledati kod
  integracije.
- **IO (T15–T17)** spojeno (merge 060924e): `GitCli` (log/toplevel/common_dir/branch_exists/
  current_branch, grane s brojem commita ispred i starošću, radna stabla, zadnja promjena putanje),
  `Project` (otvaranje iz podmape i radnog stabla, profil s `deny_unknown_fields`, overridei/vizije,
  docs s vremenom zadnje promjene, `ReportInput`); provjereno nad Sokrat Studyjem: 56 docs, 31 grana,
  4301 redak loga. Recenzije: T15/T16 SPOJIVO; T17 SPOJIVO uz 2 prijedloga pretvorena u nalaze i
  ispravljena (`common_dir` se računa iz korijena repoa, ne iz korisnikove putanje; greška čitanja
  profila koja nije „datoteka ne postoji" se propagira, ne guta). Brifovi T15/T16 imali kriva
  unix-vremena; plan traži da ih graditelj sam izračuna naredbom, što se i dogodilo.
- **METRIKE (T7–T10)** gotovi i recenzirani na grani `feat/core-metrics` (T11 slijedi, još ne u
  `main`-u): sati po `author_time` (S-007) — 0 negativnih dana na fixtureu, 33/37 dana identično
  `RAD.xlsx`-u, razlika samo oko tri poznata cherry-picka.
- **Novi nalaz, mjeren (drugi kvar tablice): `git log --since` bez sata uzima trenutno doba dana**
  (approxidate). `git log main --since=2026-08-29` u 17:15 → 183 commita; `--since='2026-08-29 00:00'`
  → 190. `rad-xlsx.py` (retci 154, 178) šalje goli datum → `RAD.xlsx` ovisi o satu pokretanja skripte;
  dnevni zadatak u 23:45 zna izgubiti gotovo cijeli tekući dan. Isti kvar pogađa fiksne raspone
  zatvorenih faza (MREŽA 20→25 commita, RAČUN R1 0→6 commita kad se doda puni dan). Zapisano kao
  **S-011** u `DECISIONS.md`: `io` šalje `--since=<datum> 00:00:00`, jezgra filtrira po
  `commit_date >= since`; referentni `expected.json` regeneriran Python-kopijom s dodanim `' 00:00'`
  (tok FIXTURE, T2b, na grani `feat/fixtures`, spaja se uskoro). Kvar se ispravlja, ne prenosi (kao
  S-007). U `BACKLOG.md`: stavka da Leon razmotri isti dodatak u `rad-xlsx.py` (tuđi repo, Leon
  odlučuje).
- **Testovi na `main`-u:** `cargo test --workspace` = 30 passed.
- **Praksa koja se pokazala:** brifovi nisu bili savršeni (kriva unix-vremena, generička
  `read_json_or` zamijenjena dvjema konkretnim funkcijama jer `serde` nije izravna ovisnost `io`-a),
  ali protokol graditelj → recenzent → orkestrator to hvata bez Leona.

### Što slijedi
METRIKE T11 (na grani `feat/core-metrics`) → CLI T18–T19 (`feat/cli`, čeka slobodno mjesto) → FIXTURE
T2b (`feat/fixtures`, regeneracija `expected.json` za S-011) → spajanje svakog toka u `main` čim
recenzent kaže SPOJIVO → INTEGRACIJA (T20–T22) u svom stablu → zastanak na kraju M1 (Leonov OK).
