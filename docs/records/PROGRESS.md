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
`plan/ARHITEKTURA_M1.md` (tada aktivni spec, danas u `archive/`) · `plan/ROADMAP.md` ·
`workflow/TESTING.md` · `workflow/RUST.md` ·
`records/PROGRESS.md` · `records/CHANGELOG.md` · `records/DECISIONS.md` · `records/BACKLOG.md`.
Tada namjerno još nisu postojali: `architecture/`, `BUGS.md`, `HISTORY.md`, `archive/`, `ideas/`,
`LICENSE` — nastaju kad imaju sadržaj (`architecture/` i `archive/` nastali su na kraju M1, šesti dio sesije).

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
  (tok FIXTURE, **T2b spojen u `main` merge-om 5e79d91**, uz `README.md` fixturea koji objašnjava
  razliku). Kvar se ispravlja, ne prenosi (kao S-007). U `BACKLOG.md`: stavka da Leon razmotri isti
  dodatak u `rad-xlsx.py` (tuđi repo, Leon odlučuje).
- **Testovi na `main`-u:** `cargo test --workspace` = 30 passed.
- **Praksa koja se pokazala:** brifovi nisu bili savršeni (kriva unix-vremena, generička
  `read_json_or` zamijenjena dvjema konkretnim funkcijama jer `serde` nije izravna ovisnost `io`-a),
  ali protokol graditelj → recenzent → orkestrator to hvata bez Leona.

### Isporučeno (peti dio sesije — CLI, METRIKE, FIXTURE T2c i INTEGRACIJA spojeni u `main`)
- **CLI (T18–T19)** spojeno (merge 708e373): naredbe `report [path] [--since] [--json|--table]`,
  `docs`, `signals`; izlazni kodovi 0/1/2/3 (0 nema signala, 1 Warn, 2 Alert, 3 greška — poruka iz
  `IoError` na stderr); tablični ispis s hrvatskim natpisima (`table::label()`), identifikatori
  ostaju engleski (S-008). Recenzije: SPOJIVO.
- **METRIKE (T7–T11)** spojeno (merge 4f2f7d9): `days_between` bez ovisnosti (Hinnant, `civil.rs`),
  sati po `author_time` (S-007), tempo po danu, vrste rada s overrideom, faze (zatvorene po
  `commit_date`, dosljedno S-011) i 18 pokazatelja. Recenzije: T7/T8/T10/T11 SPOJIVO; T9 i T10
  vraćeni jednom (zaglavlje nije odgovaralo kodu, obrazac s nepotrebnim `clone()`) i ispravljeni.
  Paritet 16/18 pokazatelja neovisno preračunat od recenzenta.
- **FIXTURE T2c** spojeno (merge 6e9e676): `expected.json` dobio `hours_fixed` (Python-kopija
  generatora s dodanim sortiranjem po autoru) — Rustovi sati po danu jednaki `hours_fixed` na
  14/14 dana, 85,0 h (stara tablica: −139,2 h zbog cherry-pickova, S-007).
- **INTEGRACIJA (T20–T21)** spojeno (merge cbc7d71): `build_report` (recept od 12 koraka, `Context`
  u vlasništvu — S-002/RUST.md §1) i `tests/parity.rs` (integracijski test): svi dani, vrste,
  pokazatelji i faze jednaki referenci, sati jednaki `hours_fixed`; test dokazano pada kad se
  brojka pomakne. Dogfooding nad Sokrat Studyjem (recenzent ponovio neovisno): 14 dana, 190
  commita, 105 isporuka, 85 h, docs 100/100 (lag 0 dana), 1 signal ALERT `unmerged-branches`
  (`fix/kadar-nalicje` 11 dana/12 commita, `feat/tinder-kadar` 8 dana/34 commita) — potvrđeno
  izravno gitom; `sokratis signals` nad repoom vraća izlazni kod 2.
- **Testovi na `main`-u:** `cargo test --workspace` = 47 passed.
- **Na kraju petog dijela preostajalo je za M1:** T22 (`.sokratis/profile.json` dogfooding nad Sokrat
  Studyjem, `cargo build --release` + izvještaj, `ARCHITECTURE.md`, aktivni spec seli u `archive/`,
  `CHANGELOG.md` dobiva 0.1.0), pa završna recenzija cijelog M1, pa zastanak i Leonov OK. Tada M1
  još nije bio isporučen i `CHANGELOG.md` je stajao pod `[Unreleased]` — riješeno u šestom dijelu.

### Isporučeno (šesti dio sesije, večer — T22 spojen, M1 kod isporučen)
Orkestrator: Fable 5.1; graditelj i recenzent: sonnet.
- **INTEGRACIJA (T22)** spojena (commit 7e07c9e, merge 231bee8): `.sokratis/profile.json` kojim
  Sokratis mjeri sam sebe, pa release build i sve tri naredbe nad Sokrat Studyjem (isključivo
  čitanje; `git status` provjeren prije i poslije — ništa se u tuđem repou nije promijenilo).
  Recenzija: SPOJIVO. **Time su sve cigle M1 (T1–T22) u `main`-u.** Brane zelene (`cargo fmt
  --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace` bez padova);
  brojke i ispisi: `CHANGELOG.md`.
- **Odstupanja od najave (git je istinitiji od naloga):**
  1. `signals .` nad samim Sokratisom daje **kod 0, ne 1** — nalog je očekivao `unmerged-branches`,
     ali su sve grane tokova već spojene u `main` (provjereno `git branch --merged main`). Pravilo je
     zasebno dokazano nad Sokrat Studyjem, gdje hvata dvije stvarno nespojene grane (kod 2).
  2. `docs` nad Sokrat Studyjem daje **0 nalaza**, a nalog je očekivao nalaze s `datoteka:redak`;
     njegova dokumentacija je zatečeno čista prema Sokratisovim pravilima. Ništa nije „popravljano"
     da bi ispis izgledao kao u nalogu.
  3. **`debugging` = 0 % u Sokratisovu vlastitom izvještaju** jer njegove poruke commita (`M1/N: …`)
     ne pogađaju zadani klasifikacijski regex (pisan za vokabular Sokrat Studyja). Nije kvar jezgre
     nego posljedica zadanog profila; Sokratis dobiva vlastiti `classifier` u M2.
  4. Profil je dobio `test_path_contains: ["/tests/"]` izvan brifa (uputa orkestratora) jer zadani
     prefiks `tests/` ne hvata `crates/*/tests/`. Poznato ograničenje ostaje zapisano: inline
     `#[cfg(test)]` moduli **ne** ulaze u „redaka u testovima" jer se testni redak prepoznaje samo
     po putanji (opisano u `architecture/ARCHITECTURE.md` §10).
- **Dokumentacijsko zatvaranje M1** (ovaj commit): nastao `docs/architecture/ARCHITECTURE.md` (što JE
  izgrađeno — granice crateova, tok podataka, sva polja profila, formati `.sokratis/*.json`, izlazni
  kodovi); spec `ARHITEKTURA_M1.md` preseljen u `docs/archive/` s pečatom „ISPUNJEN 2026-09-17 —
  referenca, ne izvor istine"; `docs/README.md` dobio sekcije `architecture/` i `archive/`;
  `CHANGELOG.md` dobio **0.1.0**; `ROADMAP.md`, `CLAUDE.md` i pojmovnik `RUST.md` §4 usklađeni.

### Isporučeno (sedmi dio sesije, večer — završna recenzija M1 i jedini krug popravaka)
Orkestrator: Fable 5.1; završna recenzija, popravljač i čuvar: opus; ponovna recenzija: sonnet.
- **Završna recenzija cijelog M1** (lanca, ne cigle; raspon `8816d1a..7f1697a`): **3 Critical, 9
  Important, 15 Minor**, ocjena „uz popravke", brane zelene. Tri Critical nalaza su bila „kriva
  brojka ili pad", ne stil: regex iz profila **bez capture-grupe** obarao je proces s kodom 101
  (valjan JSON, valjan regex — jedino mjesto gdje korisnikova ispravna konfiguracija ruši binarnu);
  **`--since` se nije provjeravao**, pa je tipfeler (`2026-9-17`, `17.09.2026`) tiho mijenjao prozor
  mjerenja; **pokazatelj zatvorenih faza** gledao je `profile.since` umjesto `--since`, pa je jedan
  od 18 pokazatelja bio kriv kad god se ta dva razlikuju. Recenzent je uz to presudio osam odgođenih
  sitnica i preispitao odluke orkestratora (S-011 potvrđen, ali kao nepotpun — vidi dopunu S-011).
- **Jedini krug popravaka** (19 commita, spojeni u `main`): **22 stavke riješene** — C1 C2 C3, I1 I2
  I4 I5 I7 I8, M1 M2 M4 M5 M6 M7 M8 M9 M10 M12 i pet odgođenih sitnica. Testova **47 → 64**; brojke
  nad Sokrat Studyjem **nepromijenjene** (190 commita / 70 164 redaka / 1801 izmjena), paritet zelen.
  Što korisnik CLI-ja time dobiva: `CHANGELOG.md` (0.1.0, „Popravci nakon završne recenzije M1").
- **Ponovna recenzija kruga: SPOJIVO** — svih 22 stavki potvrđeno s dokazom `datoteka:redak` i
  testom, bez novih kvarova; prihvaćena su tri graditeljeva odstupanja (popravak C1 primijenjen i na
  `classify.rs` i na `diary_heading`; regexi `docs.rs` premješteni u `Patterns` umjesto `LazyLock`;
  testna brojka za C3 se kroz I8 promijenila iz 3 u 2).
- **Što je svjesno otišlo u M2, ne skriveno** (svaka stavka u `BACKLOG.md`, stanje koda u
  `ARCHITECTURE.md` §11): mrtvo polje `phase_tag` s tvrdim prefiksom `"{id}/"` za aktivne faze;
  vizije bez zbroja po stanju; putanje iz profila neograđene na korijen repoa (prije objave);
  jedinice i natpisi tablice; performanse (≈90 `git` poziva po izvještaju, 3,2 s nad Sokrat
  Studyjem); poruka u detached-HEAD stanju; snapshot JSON-a kad se oblik `Report`-a zaključa za
  Tauri. `rust-toolchain.toml` je ostavljen **Leonu** jer pin tjera `rustup` da skine još jednu
  kopiju toolchaina na njegov stroj.
- **Leon usred sesije (20:18):** logo Sokratisa postoji (`C:\Users\leonk\Downloads\sokratis logo .png`)
  i treba biti **ikona aplikacije** (prozor, tray, instalacija); pri otvaranju aplikacije ide
  **animacija** s natpisom „Sokratis" u kojem logo stoji **na mjestu slova „o"** (logo predstavlja to
  slovo, ne zamjenjuje cijelu riječ). Prvi zapis ove poruke (logo umjesto cijelog teksta + splash) bio
  je pogrešan; Leon ga je kasnije iste večeri ispravio. Zahtjev za M2 stoji u `plan/ROADMAP.md`
  (red M2), nigdje drugdje.
- **Dokumentacijsko zatvaranje** (ovaj commit): `ARCHITECTURE.md` usklađen s kodom nakon popravaka
  (provjera ulaza, prozor dovlačenja s rezervom, izlazni kodovi, §11 dopunjen), `CHANGELOG.md` dobio
  popravke i broj testova, `DECISIONS.md` dopunu S-011, `TESTING.md`/`RUST.md` bez `insta` i
  snapshot-testova, `BACKLOG.md` nalaze za M2, `ROADMAP.md` i `CLAUDE.md` stanje.
- **Audit svih `.md` pred zastankom** (CLAUDE.md #8): plan M1 dobio pečat „IZVRŠEN" (bio je bez
  statusa, a nosi `insta` iz vremena pisanja), `README.md` govori 0.1.0 umjesto „pred zastankom",
  `AGENTI.md` ispravio tok KOSTUR (T1 je nastao na `feat/kostur`, ne na `main`-u) i dogfooding-odjeljak
  (CLI radi; `signals .` je danas 0 jer su sve grane tokova spojene). `sokratis docs .` = **100/100,
  0 nalaza**, `git branch --no-merged main` prazno.

### Što slijedi (na kraju sesije 2026-09-17)
**Zastanak — Leonov OK.** Na njega čekaju: brisanje 8 grana tokova i njihovih radnih stabala, push i
objava (remote još ne postoji), odluka o `rust-toolchain.toml`. Napomena: `.claude/agents/*.md` su
globalno git-ignorirani, pa definicije agenata **nisu** u repou — i to je Leonova odluka. Nakon OK-a:
spec za M2 (desktop).

---

## 2026-09-18 (FABLE) — Leonov OK, grane i stabla obrisani, M1 zatvoren

**Odmah iza ponoći, nastavak zastanka s kraja M1.**

- **Leon je dao izričit OK** za brisanje: svih **8 radnih stabala** `sokratis.<tok>` (sva čista, 0
  nepohranjenih promjena) i svih **8 grana** `feat/*` (sve spojene u `main`, `git branch -d`). `git
  worktree list` sad pokazuje samo `sokratis` na `main`; `git branch -a` samo `main`.
- **Provjera nakon brisanja:** `cargo test --workspace` = 64 testa zeleno; `sokratis signals .` = 0
  (kod 0); `sokratis docs .` = 100/100, 0 nalaza. Brojke nepromijenjene u odnosu na kraj M1.
- **M1 je time zatvoren** (main = 0e24ffe). Od stavki koje su čekale Leona ostaju samo: push/objava na
  GitHub (remote još ne postoji), odluka o `rust-toolchain.toml`, uvođenje `.claude/agents/*.md` u
  repo (danas globalno git-ignorirani).
- **Čuvar dokumentacije (Način A):** `CLAUDE.md` „Stanje — TRENUTNO" i „Agenti" odjeljak, `ROADMAP.md`
  (M1 red i „Gdje smo"), `AGENTI.md` §2/§7 i `README.md` usklađeni s ovim stanjem — nijedna živa
  tvrdnja da grane/stabla tokova još postoje ili čekaju brisanje.
- **Leon je (~00:20) dao izričit OK za još dva koraka.** Korijenski `rust-toolchain.toml` pina
  kompajler na `1.98.1` (`components = ["rustfmt", "clippy"]`); rustup ga sam preuzeo, brane (`fmt`,
  `clippy`, `cargo test --workspace` = 64) zelene pod pinom (`de107b8`).
- `.claude/agents/{graditelj,recenzent,cuvar-dokumentacije}.md` su ušli u repo s `git add -f` — bili
  su globalno git-ignorirani u `~/.config/git/ignore`; jednom praćeni, taj ignore više ne vrijedi
  (`1996cc0`).
- Push/remote/objava na GitHub i dalje čeka Leona: remote ne postoji, `gh` nije instaliran na stroju.

### Što slijedi
**Spec za M2 (desktop).** Brainstorming s Leonom → jedan aktivni spec `docs/plan/ARHITEKTURA_M2.md` →
plan cigla-po-cigla → agenti kao u M1 (nove grane/stabla otvara plan M2, ne postoje danas).

---

## 2026-09-18 (FABLE) — Spec i plan M2 (desktop) + kostur `M2/1a`

**Popodne i večer.** Spec i plan su napisani bez ijednog retka koda; kod je došao tek na kraju, kao
prva cigla M2. (Prva polovica na Opusu 5, nastavak na Fableu 5.1.)

- **Ulaz pročitan redom, ništa napamet:** `CLAUDE.md`, `git log`/`worktree`/`remote` (samo `main`, jedno
  stablo, **remote ne postoji**), ledger M1 „STANJE ZA NOVU SESIJU", `docs/README` → PRD → ROADMAP
  red M2 → BACKLOG „Iz završne recenzije M1" → ARCHITECTURE §2–3 i §11 → arhivirani spec M1 (cijeli,
  kao uzor odjeljaka) → `tokens.css` Sokrat Studyja (samo čitanje).
- **M0 za M2 izmjeren na stroju:** Node 24.11.1 · npm 11.6.2 · WebView2 153 · MSVC 14.44 · Rust 1.98.1
  ✅; `cargo tauri`, pnpm, `gh` ❌. Zaključak u specu §10: `@tauri-apps/cli` kao pinana dev-ovisnost
  → **ništa globalno ne treba**; jedina instalacija je `npm install` u `apps/desktop` i čeka Leonov OK.
- **Brainstorming (superpowers:brainstorming, arhitektonski put), trinaest pitanja, jedno po jedno;**
  Leonovi odgovori: stalno otvoren za proučavanje · prvi ekran Pregled svih projekata sa signalima ·
  svih 8 pogleda s uređivanjem · SQLite registar + snimke + keš · watcher na `.git` + gumb · tray
  minimizira + autostart + obavijest samo na prijelaz u Alert · „Dodaj projekt" odabirom mape, stabla
  se grupiraju sama · grafovi vlastiti SVG · sve četiri teme, `brand-*` iz loga · birač raspona,
  zadano cijeli projekt · HR/EN prekidač već u M2 · dug M1: 7 od 9 · ručni podaci u glavno stablo ·
  animacija jednom po pokretanju, prozor čeka.
- **Leon je usred sesije dostavio znak i animaciju** (`C:\Users\leonk\Downloads\download.png` = zaključak
  `S◍KRATIS`; `sokratis-intro-clean-graph.html` = canvas animacija 4,2 s s dva WebP-a). Pročitan je
  **kod**, ne opis: vremenska crta (stupci · brisanje · prsten · lik · skupljanje i natpis `#00dce8`)
  i `prefers-reduced-motion` grana su u specu §5.2 doslovno. Dvije bilješke koje kod nije znao: neonski
  cijan na bijeloj ima kontrast ≈1,7:1 → `brand-*` se **izvodi i mjeri** (S-017), ne kopira; tray na
  16 px traži pojednostavljen znak bez lika (§5.4).
- **Tri pristupa ljusci** izložena s preporukom (tanak Tauri + ugovor `Report`; debeo Tauri; sve u
  desktop crateu); Leon odobrio prvi uz posudbu iz drugog (mjerenja u `core`). Dizajn predstavljen u
  tri kruga (granice i ugovor · SQLite, tok, watcher · sučelje, testovi, izlazni uvjet), svaki krug
  odobren prije idućeg.
- **Napisano:** `docs/plan/ARHITEKTURA_M2.md` (12 odjeljaka, isti kostur kao spec M1) ·
  `DECISIONS.md` **S-012…S-022** · BACKLOG: sedam stavki duga → pointer na spec §8, HR/EN podijeljen na
  sučelje (M2) i CLI-tablicu (M3), dvije nove ideje (vektorizacija znaka, otvaranje u editoru) ·
  ROADMAP (uvod, „Gdje smo", red M2) · `docs/README` (plan/ indeks) · `CLAUDE.md` (stanje, stack,
  žive odluke) · `CHANGELOG` Unreleased.
- **Jedno odstupanje od Leonova izbora, s razlogom, zapisano u specu §4.4 i S-014:** keš po SHA je
  odabran, ali cigla ulazi **tek ako mjerenje nakon M11 pokaže da treba** — skupi su procesi (56
  `git log -1` + 31 `rev-list`), ne parsiranje; keš to ne rješava, M11 rješava. Pravilo #4.

- **Leon je spec odobrio** („Imaš moj OK") bez izmjena. **Plan cigli napisan** (`superpowers:writing-plans`):
  `docs/superpowers/plans/2026-09-18-m2-desktop.md`, 3639 redaka, **35 cigli u 9 tokova** (KOSTUR ·
  JEZGRA · PROFIL · IO · STORE · CLI · SUČELJE · DESKTOP · INTEGRACIJA), vlasništvo datoteka po toku,
  graf ovisnosti među tokovima, test + kod po koraku. Verzije za pinanje provjerene na crates.io
  (`max_stable_version`, ne Tauri 3 alpha) i npm-u; TypeScript ostaje 5.9.3 jer svelte-check ne prima 7.
  Dvije stvari koje je pisanje plana **vratilo u ugovor** iako ih spec nije imenovao: `Report.deliveries`
  (pogled Isporuke traži popis, M1 ih je samo zbrajao — T1/T5) i `MetricValue.kind` (stupac `kind` u
  tablici `snapshot`). Jedno odstupanje od speca §5.1 zapisano u planu: nema naredbe `copy_path`, sučelje
  kopira kroz `navigator.clipboard` (T28). Keš po SHA (T18) je **uvjetna** cigla po mjerenju T11.
- `AGENTI.md` pokazuje na tokove M2 u planu (jedno mjesto); definicije agenata (`graditelj`, `recenzent`,
  `cuvar-dokumentacije`) više ne nose putanju plana M1 ni `M1/N` — govore „aktivni plan", znaju za tok
  SUČELJE (TS/Svelte, brane `npm run check`, zaglavlje `// ZAŠTO OVAKO`) i za S-012…S-022.

- **T1 (kostur M2), koraci 1–12, na `main`-u kao `M2/1a`:** ugovor tipova u jezgri (`until`,
  `Report.commits/deliveries/vision_totals`, `SnapshotMetrics`, `MetricValue{kind}`, `PathOutsideRoot`,
  `test_path_exclude`, `validate_paths` stub), nov crate `sokratis-store` (migracija 0001 sa svih sedam
  tablica, 1 test), `apps/desktop` kostur (npm datoteke s točnim verzijama, `src-tauri`), `workspace.dependencies`
  pinane. Brane: fmt · clippy · **65 testova** (64 + 1) · docs 100/100 · signals 0. **Odstupanje od plana,
  zapisano:** T1 je podijeljen u dva commita jer korak 13 (`npm install`) čeka Leonov OK, a desktop crate bez
  ikona iz `tauri icon` ruši `cargo test --workspace` — pa je privremeno izvan `members` (komentar u
  `Cargo.toml` kaže zašto). Leon je tražio primopredaju u novu sesiju prije OK-a.
- **Čuvar dokumentacije (način A + B) nakon `M2/1a`, pred novu sesiju:** `ARCHITECTURE` sada opisuje
  **četiri** cratea i ljusku izvan workspacea, `Report` s 15 polja (četiri prazna do M2/3–M2/5),
  profil s 39 polja i §11 popis kostura koji stoji a ne radi; `RUST.md` §2 dobio je `rusqlite`,
  `notify`, `insta` (vraćen, S-022), `tauri` + plugine, §4 pet novih pojmova i nov §5 za TS/Svelte;
  `TESTING.md` više ne tvrdi da `insta` nije ovisnost i dobio je retke za `store` i sučelje;
  `CHANGELOG` Unreleased kaže što `M2/1a` mijenja u JSON-u. Audit svih `.md` prema gitu: ispravljene
  tvrdnje „spec čeka Leonov OK", ledger M1 kao prva radnja nove sesije i Playwright kao brana M2.

### Što slijedi
**Nova sesija dovršava M2 do kraja.** Prvo T1 korak 13 uz Leonov OK (`npm install` → `tauri icon` → desktop
crate natrag u `members` → `cargo build -p sokratis-desktop` → commit `M2/1b`), pa stabla tokova i prvi val
graditelja (JEZGRA T2, PROFIL T8, IO T10, SUČELJE T20). Ledger: `.superpowers/sdd/2026-09-18-m2-desktop/progress.md`
(odjeljak „STANJE ZA NOVU SESIJU" na dnu) + `NOVA-SESIJA-PROMPT.md` pored njega. Push/remote i dalje samo uz
izričit OK.

---

## 2026-09-18 (FABLE) — Kostur M2 dovršen (`M2/1b`), stabla tokova otvorena, prvi val graditelja

**Nova sesija, večer.** Nastavak zastanka s kraja prošle sesije: korak 13 cigle T1 uz Leonov OK.

- **Leon je dao izričit OK za `npm install`** u `apps/desktop` (isti OK pokriva `npm ci` u stablima
  `sokratis.ui`/`sokratis.desktop`). Instalirano 80 paketa, verzije točno pinane, 0 ranjivosti →
  `apps/desktop/package-lock.json`.
- **Ikone:** `npm run tauri icon` iz Leonova loga → `apps/desktop/src-tauri/icons/` (desktop + Windows
  set). Mape `android/` i `ios/` koje alat usput generira **nisu commitane** (aplikacija je
  desktop-only; odluka orkestratora, vraćaju se istom naredbom ako zatrebaju).
- **Odstupanje od teksta plana, s razlogom (pravilo #6):** `apps/desktop/vite.config.ts` uvozi
  `defineConfig` iz `vitest/config` (taj tip poznaje ključ `test`) i koristi putanje relativne prema
  Vite korijenu umjesto `node:url` — `svelte-check` je padao bez `@types/node`, a nova ovisnost samo
  radi config-datoteke nije opravdana.
- **`apps/desktop/src-tauri` (crate `sokratis-desktop`) je natrag u `[workspace] members`**; prvi
  `cargo build -p sokratis-desktop` je trajao 3 min. Brane na `main`-u: `cargo fmt --check` ·
  `cargo clippy --all-targets -- -D warnings` (`--workspace`) · `cargo test --workspace` **65
  testova** · `npm run check` zelen (svelte-check 0 grešaka + 1 vitest) · `npm run build` daje
  `dist/` s `index.html` i `splash.html` · `sokratis signals .` = nema signala. Koraci 13 i 14 cigle
  T1 su u planu označeni `[x]` u istom commitu (`cb963e8`, poruka `M2/1b: kostur M2 dovršen`).
  **Time je T1 (kostur M2) cijel u `main`-u** (`M2/1a` + `M2/1b`).
- **Otvorena su četiri radna stabla tokova**, sva na `cb963e8` (`git worktree list`):
  `sokratis.jezgra` (`feat/core-m2`) · `sokratis.profil` (`feat/core-profile`) · `sokratis.io`
  (`feat/io-m2`) · `sokratis.ui` (`feat/ui`). **Prvi val graditelja je poslan:** JEZGRA T2 · PROFIL
  T8 · IO T10 · SUČELJE T20. Stabla STORE/CLI/DESKTOP/INTEGRACIJA otvaraju se kasnije po ovisnostima
  iz plana.
- **Odluka orkestratora za izvedbu:** brane Rust-tokova u njihovim stablima idu **bez** desktop
  cratea (`--workspace --exclude sokratis-desktop`), jer bi inače svako stablo kompiliralo Tauri i
  tražilo `dist/`, koji ta stabla ne grade; pune brane s desktopom vrti orkestrator na `main`-u
  nakon svakog spajanja. Zapisano u `docs/workflow/AGENTI.md` §5 (jedno mjesto).
- GitHub: remote i dalje **ne postoji** (`git remote -v` prazno). Push/remote/objava i dalje čekaju
  Leonov izričit OK.
- **Čuvar dokumentacije (način A):** `PROGRESS.md` (ovaj unos), `CHANGELOG.md` (`M2/1b` pod
  `[Unreleased]`), `ROADMAP.md` („Gdje smo" i status M2), `CLAUDE.md` („Stanje — TRENUTNO": T1 cijel,
  desktop crate u workspaceu, stabla otvorena, „na Leona čeka" dobio izvor tray-ikone T33 i brisanje
  grana/stabala na kraju), `docs/workflow/AGENTI.md` §2/§5 (stabla postoje, exclude-flag),
  `docs/workflow/TESTING.md` §4 (`npm run check` više ne čeka `node_modules`),
  `docs/architecture/ARCHITECTURE.md` (desktop crate u workspaceu, §11 builda se), `docs/README.md`
  (plan/ uvod). Audit grepom potvrdio da nijedna preostala `.md` tvrdnja ne kaže „izvan `members`"
  ili „čeka Leonov OK za `npm install`".

**Nastavak iste večeri — PROFIL gotov, JEZGRA prva polovica spojena.**

- **PROFIL (T8–T9) spojen, tok gotov** (merge `a2e9265`): `inside_root(rel: &str) -> bool` u
  `crates/sokratis-core/src/profile.rs` razlaže putanju kroz `std::path::Component` i s `matches!`
  odbija sve osim `Normal`/`CurDir` (dakle `..`, apsolutnu, `C:\…`, `\\server\share`) bez ijednog
  diranja diska (S-002); `Profile::validate_paths()` je bio stub iz T1, sad provjerava svih osam
  polja putanja i vraća `ParseError::PathOutsideRoot { field, value }` na prvi pogodak — jezgreni dio
  duga I9 (BACKLOG). Io-dio (ograda pri `Project::open`) dolazi u IO T14, pa I9 ostaje **djelomično**
  riješen, ne zatvoren. `test_path_exclude` dobio ponašanje u `is_test_path` (stražarska klauzula,
  rani `return false` prije uključivih pravila); zadano `[]` čuva paritet (S-005) — Sokratisov
  vlastiti profil dobiva `["/fixtures/"]` tek u T35. Obje cigle recenzirane: SPOJIVO.
- **JEZGRA prva polovica (T2–T3) spojena, tok nastavlja na istoj grani** (merge `7d23c76`): snapshot
  ugovora `Report`-a (`crates/sokratis-core/tests/snapshot.rs`, `insta::assert_json_snapshot!`, 467
  redaka, 15 ključeva na vrhu) zaključava oblik JSON-a kao test od sada — svaka buduća promjena
  (novo polje, preimenovanje, drugi redoslijed) pada dok se snimka namjerno ne ažurira uz obrazloženje
  u commitu (S-022); time je BACKLOG-stavka I7 riješena. `until` postao stvarna gornja granica
  razdoblja (cijeli dan uključivo, S-011): jezgra provjerava oblik (`ParseError::BadDate`, isti
  ugovor kao `since`) i filtrira commite i isporuke tako da ostane samo `commit_date`/`date <= until`;
  `civil::next_day` je zrcalo `prev_day`-a za rezervu zone u `io` (potrošač dolazi u T14). **Odstupanje
  od brifa, potvrđeno na kodu:** umjesto dopisivanja četvrtog commita u dijeljeni modulski `LOG` u
  `report.rs` (što bi promijenilo `touched.commits` i srušilo postojeći test
  `assembles_everything_and_filters_by_since`), graditelj je dodao lokalni `const
  LOG_WITH_LATER_COMMIT` samo unutar novog testa — isti obrazac kao već postojeći test
  `closed_phases_in_range_follows_input_since_not_profile_since`; recenzent je na kodu potvrdio da je
  time duh napomene ispunjen bez diranja zelenih testova.
- **Brane na `main`-u nakon oba spajanja:** `cargo fmt --check` OK · `cargo clippy --workspace -- -D
  warnings` OK · **72 testa** · `sokratis signals .` = nema signala.
- **Ostali tokovi, izvan `main`-a (stanje za drugi val, ne još isporučeno):** IO — M2/10 (atomarno
  pisanje ručnih podataka u glavno stablo) recenzirano SPOJIVO, M2/11 (performanse gita) u izradi;
  STORE — M2/15 (registar projekata) SPOJIVO, M2/16 (postavke) na recenziji, peto stablo
  `sokratis.store` (`feat/store`) otvoreno; SUČELJE — M2/20 (`tokens.css` + brana kontrasta) na
  recenziji; JEZGRA — M2/4 (zbroj vizija) u izradi. Remote i dalje ne postoji; push/objava i dalje
  čekaju Leonov izričit OK.
- **Čuvar dokumentacije (način A), ovaj zapis:** `PROGRESS.md` (ova dopuna), `CHANGELOG.md` (retci
  M2/2, M2/3, M2/8, M2/9 pod `[Unreleased]`), `RUST.md` §4 (`matches!`, `Option::is_none_or`,
  let-chain, stražarska klauzula; dopuna postojećih redaka za `std::path::Component` i
  struct-update) i §2 (`insta` red ažuriran — prvi snapshot-test više nije najava nego činjenica),
  `ARCHITECTURE.md` (§2/§3/§4/§10/§11: `until` filtrira i validira, `validate_paths` i
  `test_path_exclude` više nisu stub/deklaracija), `TESTING.md` (naredba za namjernu promjenu
  snimke), `BACKLOG.md` (I7 riješen, I9 djelomično), `ROADMAP.md` i `CLAUDE.md` (PROFIL gotov, JEZGRA
  napola, peto stablo).

**Nastavak iste večeri — JEZGRA gotova (T4–T7), IO/STORE/SUČELJE nastavljaju.**

- **JEZGRA druga polovica (T4–T7) spojena, tok GOTOV** (merge `7711a67`; T2–T7 svi u `main`-u):
  - **M2/4** (`d126263`): `metrics/visions.rs::vision_totals` — `BTreeMap<&str, u32>` broji i sortira
    vizije po stanju u jednom prolazu (dug I6 riješen); snimka ugovora netaknuta (fixture nema vizija).
  - **M2/5** (`33ec2ac`): `commit_rows` klasificira svaki commit TOČNO JEDNOM za `Report.commits`
    (`sha`·`date`·`subject`·`kind`·`sub`·`overridden`); `kind_stats` sad broji iz tih redaka (`zip`
    nad paralelnim nizovima) umjesto da klasifikaciju ponovi; `Report.deliveries` prestaje biti uvijek
    `[]`. **SNAPSHOT NAMJERNO PROMIJENJEN** (S-022): `commits` 190 objekata, `deliveries` 105 (=
    pokazatelj `deliveries`). Recenzent (opus) je programski dokazao: sva ostala polja snimke izvan
    `commits`/`deliveries` DUBOKO JEDNAKA (diff triju dijelova snimke po ključu najviše razine, prije/
    poslije, sve tri identične), raspodjela `commits[].kind` = `kinds[]` (planning 26 · documentation
    55 · execution 61 · polish 26 · debugging 22), redoslijed = fixture-log, 3 inverzije `date` su
    poznati cherry-pickovi (S-007). **Nalaz recenzije, NIJE popravljen ovom ciglom (namjerno, izvan
    dosega T5):** `metrics/indicators.rs` i dalje zove `effective_kind` odvojeno za dva pokazatelja
    (`debugging_commits`, `docs_share`) — commit se time klasificira više od jednom po izvještaju, iako
    spec §3.2 traži jednom. Zapisano u `BACKLOG.md` i `ARCHITECTURE.md` §11 kao otvoreno do završne
    recenzije M2, **ne** kao riješeno.
  - **M2/6** (`c4a5a71`): aktivne faze se na commit vežu regexom `phase_tag` iz profila
    (`Option::is_some_and`), ne tvrdim prefiksom `"{id}/"` — dug I3 + M14 riješen; zadani profil
    (Sokrat Study) daje iste brojke, diff snimke bajtno prazan (pokriven i rub `F1/10…F1/14`).
  - **M2/7** (`722fb4d`): `src/snapshot.rs` prestaje biti prazan modul — `SnapshotMetrics::from_report`,
    `diff` (promjene po `id`-u, `NaN == NaN` tretiran kao nepromijenjeno), `SignalCounts::from_signals`,
    `worst_severity` (`Severity: Ord` + `max()`), `alerts_raised` (identitet signala `(rule,
    title_key)`, javlja SAMO prijelaz u Alert — S-020). **Odstupanje od brifa, na kodu:** brifov test
    je tražio `assert_eq!(m.docs_score, None)`, ali dijeljeni fixture `report::tests::input()` ima
    `docs/records/PROGRESS.md` (poklapa zadani `diary_path`) pa `docs_health` vrati `Some(100)` —
    graditelj je ispravio tvrdnju na `Some(100)` uz komentar u testu koji to obrazlaže. `mod tests` u
    `report.rs` postao `pub(crate)` da `snapshot::tests` posudi isti fixture umjesto da ga duplicira.
  - Recenzije: T5 i T7 na **opusu** (SPOJIVO, redom 4 i 5 Minor odgođenih), T6 na sonnetu (SPOJIVO bez
    nalaza). Nijedan Minor ne blokira STORE T17 ni DESKTOP T29/T30.
  - **Brane na `main`-u:** `cargo fmt --check` OK · `cargo clippy --workspace -- -D warnings` OK ·
    **82 testa** · `sokratis signals .` = nema signala.
- **IO — mjerenje M2/11 (performanse, dug M11) i krug popravaka.** Prvo mjerenje (release, topli keš,
  Sokrat Study, `Measure-Command`): **prije 3318–3822 ms**, git-procesa 92 (test-repo 94). Poslije
  popravka: **najbolje 1479,71 ms**, git-procesa **6** (test-repo 4) — dominantan trošak bio je jedan
  `git log --numstat` nad cijelom poviješću umjesto po datoteci/grani. **Odluka orkestratora po
  pravilu iz plana:** 1479,71 ms ≥ 500 ms → **T18 (keš sirovih commita u SQLite-u) ULAZI** u plan kao
  isporuka, ne odgađa se. **Recenzija je prvo vratila ciglu s 2 Critical**, oba dokazana pokusom: (1)
  `last_changes` je gubio zadnju promjenu putanje na merge-commitima (`--name-only` bez combined
  diffa vidi samo prvog roditelja; primjer nad Sokrat Studyjem: `docs/README.md` davao stariji unix-
  vremena umjesto stvarno zadnjeg); (2) bez `core.quotepath=false` git ne-ASCII imena `.md` datoteka
  tiho vraća kao `None`. Paritet JSON-a stare/nove binarke bio je identičan usprkos oba kvara — **dokaz
  da sam paritet JSON-a nije dovoljan test** za `last_changes`. Popravak: `--diff-merges=combined`
  (ne `-m`, koji bi merge-commitu krivo pripisao čisto spojenu datoteku) i `-c core.quotepath=false`;
  dva nova testa u `tests/last_changes.rs`, oba pala prije popravka na opisani uzrok. **Neovisno
  provjereno nad Sokrat Studyjem: 0/56 neslaganja** (bilo netočno na barem jednom danu prije popravka).
  Ponovna recenzija: SPOJIVO. M2/10, M2/11, M2/12 su time recenzirani SPOJIVO; M2/13 (watcher nad
  `.git`, S-016) je u izradi.
- **Ostali tokovi, izvan `main`-a (stanje za treći val, ne još isporučeno):** STORE — M2/15, M2/16
  SPOJIVO, M2/17 (snimke, potrošač `SnapshotMetrics`/`diff` iz JEZGRE) u izradi; SUČELJE — M2/20,
  M2/21, M2/22 SPOJIVO, M2/23 (SVG grafovi) na krugu popravka (2 Important: `label?` ostaje opcionalan
  na sve četiri komponente grafa, imenovan graf = `role="img"`+`aria-label`, neimenovan =
  `aria-hidden`). Pet stabala tokova otvoreno (`sokratis.jezgra` · `.profil` · `.io` · `.store` ·
  `.ui`); CLI/DESKTOP/INTEGRACIJA se otvaraju kasnije po ovisnostima iz plana. Remote i dalje ne
  postoji; push/objava i dalje čekaju Leonov izričit OK.
- **Čuvar dokumentacije (način A), ovaj zapis:** `PROGRESS.md` (ova dopuna), `CHANGELOG.md` (retci
  M2/4–M2/7 pod `[Unreleased]`), `RUST.md` §4 (`Iterator::zip`, `pub(crate) mod`, `Ord` derive +
  `max()` nad enumom, `?` unutar `filter_map`-a; dopuna postojećih redaka `entry().or_insert()` i
  `Option::is_some_and`), `ARCHITECTURE.md` (§2 koraci 6–7–10, §3 tri polja više nisu prazna i
  `snapshot.rs` više nije prazan modul, §4/§6 `phase_tag` radi, §11 „izračunato ali ne izlazi u
  tablicu" i nova stavka o dvostrukoj klasifikaciji), `BACKLOG.md` (I6 riješen, I3+M14 riješen, M11
  izmjeren i popravljen ali ne zatvoren jer M2/11 nije u `main`-u, nova stavka „klasifikacija jednom"),
  `ROADMAP.md` i `CLAUDE.md` (JEZGRA gotova, PROFIL gotov, IO/STORE/SUČELJE stanje).

**Nastavak iste večeri — IO, STORE i SUČELJE recenzirani dalje u svojim granama; sesija staje (predugo trajanje).**

- **IO (`sokratis.io`, `feat/io-m2`):** nakon M2/10–M2/12 (sve SPOJIVO) dodan **M2/13** (`e128986`):
  watcher nad `.git`/docs/`.sokratis` s odgodom 600 ms, potiskivanjem vlastitih upisa i
  `RefreshQueue`, javlja kroz `mpsc`, ne zna za Tauri (S-016). Recenzija (opus): SPOJIVO uz 6 Minor
  odgođenih — tri prenesena kao uputa za DESKTOP T30 (watcher se ne obnavlja kad `.sokratis` tek
  nastane; `reason` treba prioritet umjesto djelomičnog izračuna; uža provjera pretka). **M2/10–M2/13
  su sve recenzirane SPOJIVO, ali nijedna nije u `main`-u** — sljedeći korak je `git merge main` u
  `sokratis.io`, pa T14 (io-dio ograde putanja I9, `--until` prema gitu, `log --until`).
- **STORE (`sokratis.store`, `feat/store`):** **M2/16** (postavke globalne i po projektu — tema,
  jezik, autostart, prozor, raspon, zadnji pogled) i **M2/17** (snimke brojki jednom dnevno, upsert,
  profil kao kanonski JSON, trend s oznakom promjene profila, S-014) spojeni na grani; M2/17 je
  vratio jedan krug popravka (recenzija: druga snimka istog dana s manje metrika ostavljala je
  jutrošnje retke, pa je `latest_snapshot` vraćao mješavinu — popravljeno DELETE+INSERT u istoj
  transakciji, `893b3e7`). **M2/15–M2/17 su sve recenzirane SPOJIVO, nespojene u `main`.** M2/18
  (keš sirovih commita, uvjetna cigla koja **ULAZI** po mjerenju M2/11) nije još započeta.
- **SUČELJE (`sokratis.ui`, `feat/ui`):** nakon M2/20–M2/22 (SPOJIVO) dodani **M2/23** (SVG grafovi —
  `Bars`/`Line`/`Ring`/`Sparkline` nad `scale.ts`, S-018; jedan krug popravka: graf bez imena je
  ukras, `niceMax`/`linePath` ne propuštaju NaN, Ring-tooltip ne oblikuje postotak) i **M2/24**
  (splash — Leonova canvas-animacija portirana doslovno, 4,2 s, preskočiva,
  `prefers-reduced-motion` = zadnji kadar, S-019; jedan krug popravka: `splash:done` mora stići
  točno jednom i kad se slika ne učita ili se prozor preskoči prije učitavanja, ne samo na sretnom
  putu — `37d5da4`). **M2/20–M2/24 su sve recenzirane SPOJIVO, nespojene u `main`.** M2/25–M2/28
  (okvir s `api.ts`, pogledi) nisu započete. Otvoreno: orkestratorova ručna provjera puta greške
  splasha u pregledniku (namjerno pokvaren URL slike u dev-poslužitelju) — bez commita, prenosi se
  u novu sesiju.
- **T18 (keš sirovih commita) ostaje odluka „ulazi"** po mjerenju M2/11 (release, topli keš, nad
  Sokrat Studyjem: najbolje 1479,71 ms, i dalje ≥ prag 500 ms) — potvrđena, ne ponovno izmjerena.
- **Leon je sesiju zatvorio jer je postala preduga.** Ovo NIJE zastanak na kraju milestonea — M2
  nastavlja u sljedećoj sesiji bez novog OK-a za nastavak rada, samo za push/remote/objavu i
  brisanje grana na kraju M2 kao i dosad. GitHub: remote i dalje **ne postoji**; Leon je tijekom
  sesije najavio da će napraviti repo, ali nijedan URL nije stigao dovoljno pouzdano da uđe u
  dokumentaciju.
- **Čuvar dokumentacije (Način B, audit pred kraj sesije):** `CLAUDE.md` „Stanje — TRENUTNO" i
  „Agenti" (naslov i sadržaj usklađeni sa stvarnim stanjem grana; „Agenti" je ispravljen jer je
  tvrdio da su IO/STORE/SUČELJE nespojena ali dalje generički „u planu", bez da kaže da su već
  recenzirani SPOJIVO), `ROADMAP.md` „Gdje smo" i status M2 u tablici milestonea usklađeni; `docs/
  README.md`, `AGENTI.md`, `ARCHITECTURE.md`, `BACKLOG.md`, `CHANGELOG.md` provjereni — bez izmjene,
  jer već točno govore samo o `main`-u (ništa iz nespojenih grana im nije bilo upisano). `sokratis
  docs .` i dalje 100/100, 0 nalaza; `sokratis signals .` i dalje nema signala.

### Što slijedi
**Nastavak iste sesijske niti, u novoj sesiji — ne nova analiza, nego spajanje.** Prvo pročitati
`.superpowers/sdd/2026-09-18-m2-desktop/progress.md`, odjeljak „STANJE ZA NOVU SESIJU" (najnoviji, na
dnu) — on nosi redoslijed spajanja, dispatch-napomene i sve odgođene Minor nalaze po ciglama (S-010,
ne prepisuje se ovdje). Ukratko: `git merge main` u `sokratis.io` → T14 → merge IO. STORE → T18
(mjerenje već kaže da ulazi) → merge STORE. SUČELJE → T25–T28 nakon okvira s `api.ts` (i orkestratorova
provjera puta greške splasha). Potom CLI (T19, kad je IO u `main`-u), DESKTOP (T29–T33, i dalje čeka
Leonov PNG tray-znak) i INTEGRACIJA (T34–T35) kad njihove ovisnosti stoje u `main`-u. Push/remote i
objava i dalje čekaju Leonov izričit OK; isto brisanje grana/stabala tokova na kraju M2.

## 2026-09-20 (FABLE) — Repo javan na GitHubu (Leonov OK), tray-znak iz `graph.webp`

**Kratka sesija nakon zatvaranja prve sesije izvedbe M2. Nijedna cigla; jedna objava i dvije odluke.**

- **Leon je dao izričit OK za push i odlučio da repo bude JAVAN** (želi ga dijeliti). Prije pusha mu je
  rečeno što time postaje javno (fixturei pariteta = cijeli dnevnik, plan, README i git-log Sokrat
  Studyja; lokalne putanje u dokumentaciji; e-mail autora u povijesti) — odgovor: nije bitno. Odluka,
  kontekst i posljedice: `DECISIONS.md` **S-023** (jedno mjesto, ne ponavlja se ovdje).
- **Prije pusha sve četiri grane pretražene na tajne** (`git grep` po uzorcima ključeva/tokena/JWT-a/
  privatnih ključeva, pa labaviji prolaz po dodjelama `password=`/`token:` i e-mail adresama): ništa
  nije nađeno; u datotekama stoje samo dvije bezazlene adrese (kontakt projekta, testna).
- **Pushano:** `main` (`9acbeef`) · `feat/io-m2` (`e128986`) · `feat/store` (`9fba0d1`) ·
  `feat/ui` (`37d5da4`). Remote je bio prazan repo (ništa nije pregaženo); `git ls-remote` nakon pusha
  daje iste SHA-ove kao lokalne grane. Tri grane tokova su ujedno prva sigurnosna kopija recenziranog,
  a nespojenog rada (IO T10–T13 · STORE T15–T18 · SUČELJE T20–T24).
- **Tray-znak (T33) riješen bez novog crteža:** Leon nije razumio što se traži; pokazalo se da njegova
  animacija već sadrži znak bez lika (`apps/desktop/src/assets/intro/graph.webp`: prsten + četiri stupca
  + linija s točkama). Leon je odabrao njega. T33 više ne čeka nikakav materijal.
- **T18 (keš) je izgrađen** na kraju prethodne sesije (`9fba0d1`, nakon čuvareva audita) — NIJE
  recenziran; paket za recenziju je spreman u ledgeru.

**Što slijedi:** nepromijenjeno — ledger, odjeljak „STANJE ZA NOVU SESIJU". Novo iz S-023: T14 (io-dio
ograde putanja, I9) ima prednost jer je kod sada javan; licenca ne postoji (BACKLOG). **Leon je isti
dan dao TRAJNI OK za pusheve** — pravilo #1 u `CLAUDE.md` je prepisano (što orkestrator pusha sam, a
što i dalje traži izričit OK).

---

## 2026-09-20 (FABLE) — Druga sesija izvedbe M2: STORE (T15–T18) spojen u `main`

**Nastavak iste sesijske niti nakon zastanka s kraja prve sesije (2026-09-18) i kratke sesije objave
(isti dan, gore). Ovaj naslov nosi cijelu drugu sesiju izvedbe — dopunjuje ga svaki sljedeći čuvar
dokumentacije koji u njoj radi, novim odlomkom „Nastavak iste sesije", ne novim naslovom.**

- **STORE (T15–T18) spojen** (merge `e9b01c7`): registar projekata i stabala s migracijama (M2/15,
  `registry.rs`: `add_project`/`list_projects`/`project`/`rename_project`/`remove_project`/
  `touch_project`/`set_worktrees`/`worktrees`, identitet = `git_common_dir`, S-015) · postavke
  globalne i po projektu (M2/16, `settings.rs`: upsert `ON CONFLICT … DO UPDATE`) · snimke brojki s
  trendom i profilom kao kanonskim JSON-om (M2/17, `snapshots.rs`, S-014 — uz jedan krug popravka:
  druga snimka istog dana je sad cjelovita zamjena, `DELETE` pa `INSERT` u istoj transakciji, jer je
  ostavljala jutrošnje retke, `893b3e7`) · keš sirovih commita po SHA (M2/18, `cache.rs` — uvjetna
  cigla koja je UŠLA jer je mjerenje M2/11 dalo 1479,71 ms ≥ prag 500 ms, S-014; keš drži samo sirove
  commite, nikad klasifikaciju). Popis commita: `git log --oneline 3098d5b..e9b01c7`.
- **Brane na `main`-u nakon spajanja:** `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **107 passed, 0 failed** · `sokratis
  signals .` nema signala. `main` pushan na `origin` (trajni OK, pravilo #1).
- **Ispravak brojke:** `task-18-report.md` (graditeljev izvještaj, negitiran) zbraja testove store
  cratea krivo kao 21; zbroj njegovih vlastitih navedenih dijelova (2 unit + 10 cache + 3 registry +
  2 settings + 9 snapshots) je **26** — u dokumente ide 26.
- **Tok STORE je time GOTOV** (T15–T18 u `main`-u), uz PROFIL i JEZGRA otprije gotove.
- **Ostala dva otvorena toka nepromijenjena ovim spajanjem** (stanje potvrđeno iz gita, ne iz starijih
  zapisa): IO (`sokratis.io`, `feat/io-m2`) — T10–T13 i dalje recenzirani SPOJIVO, **T14** (io-dio
  ograde putanja I9 + `--until` prema gitu) u izvedbi, još nije u `main`-u; iza T14 dolazi cigla koju
  plan nema, **M2/14b — potrošač keša** (tok IO: `rev-list` daje dostižne SHA-ove, keš vraća poznate,
  `git log --no-walk --stdin` samo nedostajuće; trait `CommitCache` u `io`, adapter u desktopu) — još
  nije izgrađena. SUČELJE (`sokratis.ui`, `feat/ui`) — T20–T24 recenzirani SPOJIVO, **T25** u izvedbi,
  još nije u `main`-u.
- **Šest stabala i dalje na disku** (`git worktree list`: `main` + `sokratis.io` · `.jezgra` · `.profil`
  · `.store` · `.ui`); `feat/store` je sad spojena grana, ali grana i njeno stablo ostaju dok Leon ne
  da izričit OK za brisanje (nepromijenjeno pravilo).
- **Opseg preostatka ove sesije, Leonova odluka:** u `main` ulazi sve osim DESKTOP-a (T29–T33) i
  INTEGRACIJE (T34–T35); oni i završna recenzija cijelog M2 su treća sesija.
- **Čuvar dokumentacije (način A), ovaj zapis:** `PROGRESS.md` (ovaj naslov), `CHANGELOG.md` (retci
  M2/15–M2/18 pod `[Unreleased]`), `RUST.md` §4 (šest novih pojmova iz store-cratea: više `impl`
  blokova po datotekama, `params!` makro, `.optional()`, `to_string_lossy()` na putanji,
  `unchecked_transaction`, SQL upsert), `ROADMAP.md` („Gdje smo" i status M2), `CLAUDE.md` („Stanje —
  TRENUTNO": STORE gotov, datum stanja). `ARCHITECTURE.md` namjerno nedirana (radi je čuvar način B na
  kraju sesije). `sokratis docs .` provjeren prije i poslije izmjena.

**Nastavak iste sesije — IO (T10–T14) spojen u `main`, dug I9 zatvoren u cijelosti.**

- **IO (T10–T14) spojen** (merge `3ca068c`): pisanje ručnih podataka u GLAVNO stablo, atomarno `.tmp`+
  `rename` (M2/10, S-015) · performanse — `last_changes` jednim `git log --name-only` za sve
  dokumente, `branches()` jednim `for-each-ref` s `ahead-behind` (rezerva `branches_per_ref` za git <
  2.41); ~94 → 4 git-procesa po `input()`, 3,5 s → 1,48 s nad Sokrat Studyjem (M2/11, dug M11; krug
  popravka: `--diff-merges=combined` i `-c core.quotepath=false` jer je paritet-po-datoteci otkrio da
  je stari batch gubio doprinos merge-commita i ne-ASCII imena — 0/56 neslaganja nakon popravka) ·
  detached HEAD sad daje `HEAD@<sha>`, ne lažno „nema commita" (M2/12) · watcher nad `.git`/docs/
  `.sokratis`, odgoda 600 ms, potiskivanje vlastitih upisa (uklj. `.tmp` i predak), serijski red
  (M2/13, S-016; 7/7 ponovljenih pokretanja zeleno, recenzirao opus) · `input_between(since, until)` s
  rezervom zone od dva dana prema naprijed, `GitSource::log` dobiva `until`, **`Project::open` sad
  zove `validate_paths()` → `IoError::ProfileInvalid` — io-dio ograde putanja je ušao, DUG I9 JE TIME
  ZATVOREN U CIJELOSTI** (jezgreni dio M2/8 + io-dio M2/14; krug popravka: uzrok ostaje samo u lancu
  grešaka, CLI je prije rečenicu o polju ispisivao dvaput). Popis commita: `git log --oneline
  0c99c76..3ca068c -- crates/`.
- **Brane na `main`-u nakon spajanja:** `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **123 passed, 0 failed** · `sokratis
  signals .` nema signala. `main` pushan na `origin` (trajni OK, pravilo #1).
- **Sedam stabala na disku** (`git worktree list`): `main` · `sokratis.jezgra` · `.profil` · `.io` ·
  `.store` · `.ui` · **`.cli`** (novo, `feat/cli-m2`, otvoreno danas). Spojene grane i njihova stabla
  ostaju dok Leon ne kaže.
- **U izvedbi, nije u `main`-u:** tok IO nastavlja s **M2/14b — potrošač keša** (cigla koju plan nema:
  `rev-list` daje dostižne SHA-ove, keš vraća poznate, `git log --no-walk --stdin` samo nedostajuće;
  trait `CommitCache` u `io`, adapter u desktopu), na istoj grani `feat/io-m2`. **Tok CLI je otvoren
  danas** (`sokratis.cli`, `feat/cli-m2`) za **M2/19** (`report --until`) — `--until` u CLI-ju zato
  još NE postoji u `main`-u. SUČELJE (`feat/ui`) — **M2/25** (okvir s `api.ts`) recenziran SPOJIVO,
  **M2/26** u izvedbi.
- **Ručna provjera puta greške splasha** (otvorena stavka iz prošle sesije) je **odrađena danas**:
  obje slike namjerno srušene u pregledniku → `splash:done` točno jednom u oba slučaja; bez commita
  (nije bio potreban popravak koda).
- **Sitnost koja se ne rješava ovim tokom (zapisana u ledgeru za završni krug popravaka):** doc-
  komentar uz `sokratis_core::civil::next_day` kaže da IO gradi `--until=<dan+1>`, a stvarna rezerva
  (po Interfaces retku brifa T14) je dva dana — `crates/sokratis-io/src/git.rs` je dosljedan brifu,
  samo je `civil.rs`-ov komentar zastario; datoteka je izvan vlasništva toka IO.
- **Čuvar dokumentacije (način A), ovaj zapis:** `PROGRESS.md` (ovaj odlomak), `CHANGELOG.md` (retci
  M2/10–M2/14 pod `[Unreleased]`), `RUST.md` §4 (atomarni `rename`, `Cell<u32>`, `mpsc::Sender`/
  `Receiver`, `Arc<Mutex<_>>`, oporavak iz otrovanog mutexa, `thread::spawn`+`recv_timeout` kao
  otkucaj, `std::slice::from_ref`; dopuna let-chain retka za `git.rs::last_changes`) i §2 (`notify`
  redak: watcher sad u `main`-u, ne samo pinana ovisnost), `ROADMAP.md` („Gdje smo", status M2),
  `CLAUDE.md` („Stanje — TRENUTNO" i „Agenti"), `BACKLOG.md` (I9 zatvoren u cijelosti, M11 zatvoren
  jer je M2/11 u `main`-u). `ARCHITECTURE.md` namjerno dirana SAMO u tvrdnji o I9 u §11 (ostatak radi
  čuvar B na kraju sesije). `cargo run -q -p sokratis-cli -- docs .` provjeren prije i poslije izmjena.

**Nastavak iste sesije — CLI (T19) spojen u `main`, tok CLI gotov.**

- **CLI (T19) spojen** (merge `11b1708`): `sokratis report` dobiva `--until YYYY-MM-DD` (M2/19,
  `292fa0d`) — zrcali `--since`; `report_for` sad zove `Project::input_between(since, until)` umjesto
  `input(since)`, poziv u `Cmd::Report` prosljeđuje `until.as_deref()`; `docs` i `signals` i dalje
  šalju `None, None`. Neispravan `--until` → izlazni kod 3, poruku daje jezgra i imenuje polje;
  `until < since` → prazan izvještaj, kod 0. Provjereno nad ovim repoom: `--until 2026-09-18` je dao
  124 commita umjesto 134 u tom trenutku mjerenja (brojku ne prepisuj dalje — S-010).
- **Brane na `main`-u nakon spajanja:** `cargo fmt --check` OK · `cargo clippy --workspace
  --all-targets -- -D warnings` OK · `cargo test --workspace` **124 passed, 0 failed** · `sokratis
  signals .` nema signala. `main` pushan na `origin` (trajni OK, pravilo #1).
- **Poznato ograničenje, odgođeno za završni krug popravaka M2** (recenzentov Minor): `--table` u
  zaglavlju ispisuje samo „od {since}" — `until` se u tabličnom ispisu ne vidi iako je prozor
  ograničen; JSON (ugovor) je točan. Zapisano u `BACKLOG.md`.
- **Tok CLI je time GOTOV** (T19 u `main`-u), uz PROFIL, JEZGRA, STORE i IO otprije gotove.
- **Stanje ostalog, nepromijenjeno ovim spajanjem:** IO nastavlja s M2/14b (potrošač keša) u izvedbi;
  SUČELJE — M2/25 recenziran SPOJIVO, M2/26 u krugu popravka. Sedam stabala i dalje na disku.
- **Čuvar dokumentacije (način A), ovaj zapis:** `PROGRESS.md` (ovaj odlomak), `CHANGELOG.md` (redak
  M2/19 pod `[Unreleased]`), `BACKLOG.md` (novi redak: `--table` ne pokazuje `until`), `ROADMAP.md`
  („Gdje smo", status M2), `CLAUDE.md` („Komande", „Stanje — TRENUTNO", „Agenti"). `ARCHITECTURE.md`
  dirana SAMO u §9 (CLI naredbe) i u tvrdnji §11 da `io`/CLI ne šalju `until` (ostatak radi čuvar B na
  kraju sesije). RUST.md §4 nedirana — cigla ne uvodi nov pojam (`as_deref()` je već u pojmovniku, M1
  T11a). `cargo run -q -p sokratis-cli -- docs .` provjeren prije i poslije izmjena.

**Nastavak iste sesije — IO 2/2 (M2/14b, potrošač keša) spojen u `main`, tok IO potpuno gotov.**

- **M2/14b spojen** (merge `0740685`: `f3fdd11` + krug popravka `9604dbd`, samo komentari). **Cigla
  izvan plana od 35** — napisao ju je orkestrator, ne graditelj po brifu iz plana: povod je mjerenje
  M2/11 (izvještaj nad Sokrat Studyjem i dalje ≥ prag 500 ms) i cigla M2/18 (SQLite keš sirovih
  commita, već u `main`-u kroz STORE, dotad bez potrošača).
  - `sokratis-core`: `parse::gitlog::format_gitlog(&[Commit]) -> String` — INVERZ `parse_git_log`-a, s
    testom okruglog puta (parsiraj → formatiraj → parsiraj daje isti `Vec<Commit>`); `ReportInput.git_log`
    ostaje tekst bez obzira dolazi li od `git log` ili od keša (S-012), ugovor i snapshot netaknuti.
  - `sokratis-io`: trait `CommitCache { cached, store }` + `cached_log(...)` u novom `src/cache.rs`;
    `GitSource` dobiva `rev_list` (isti prozor i poredak kao `log`, izvedeno iz JEDNE funkcije
    `window_args` da se dva puta ne mogu razići) i `log_commits` (`git log --no-walk=unsorted --stdin`,
    stdin kroz `Stdio::piped()` + `take()`); `Project::input_cached(since, until, &dyn CommitCache)`;
    nove greške `IoError::Cache` · `CacheIncomplete { sha }` · `LogParse`.
  - **Ponašanje:** `rev-list` kaže ŠTO je dostižno u prozoru SADA; keš vraća commite koje već zna;
    dovlače se SAMO nedostajući (`git log --no-walk --stdin`); u izvještaj ulaze ISKLJUČIVO dostižni
    commiti (obavezni testovi: `commit --amend` i `reset --hard` ne ostavljaju stari SHA u brojkama,
    iako ostaje u kešu — golo „dovuci sve novije od najnovijeg keširanog" taj rub ne bi pokrilo). Prozor
    uvijek presuđuje `rev-list`, ne keš. Greška keša je VIDLJIVA — nema tihog povratka na puni log
    (politiku „kad se to smije dogoditi" dobiva desktop, T29). `io` i dalje NE ovisi o `store` (S-013)
    — adapter prema `sokratis-store` dolazi u desktopu (T29). CLI keš i dalje ne koristi.
  - **Poznata ograničenja, zapisana u `BACKLOG.md`:** `touched.skipped_lines` na keširanom putu ne broji
    retke preskočene pri PRVOM čitanju commita (keš pamti `Commit`-e, ne sirovi tekst); ključ keša je
    kratki SHA (`%h`) — ako git jednog dana produlji kraticu, keš se jednom puni iznova, točnost ne strada.
  - **Mjerenje nad Sokrat Studyjem** (samo čitanje, `--release`, `#[ignore]` test
    `measure_cached_input_over_a_real_repo` kroz varijablu `SOKRATIS_PERF_REPO`, test ujedno tvrdi
    jednakost commita i JSON-a `build_report`-a između keširanog i običnog puta): graditelj — bez keša
    2696 ms · hladan keš 2853 ms (5 procesa) · topao keš 441 ms (4 procesa) · `build_report` 31 ms ·
    topao ulaz + izvještaj **472 ms**; recenzent (opus), neovisno ponovljeno — 2022 ms · 1663 ms · 237 ms
    · 19 ms · **256 ms**. Cilj < 500 ms postignut u oba mjerenja.
  - **Brane na `main`-u:** `cargo fmt --check` OK · `cargo clippy --workspace --all-targets -- -D
    warnings` OK · `cargo test --workspace` **136 passed, 0 failed, 1 ignored** (ignoriran je upravo
    mjerni test) · paritet (`parity.rs`) i snapshot zeleni bez izmjene · `sokratis signals .` = nema
    signala. `main` pushan na `origin` (trajni OK, pravilo #1).
  - **Tok IO je time GOTOV u cijelosti** (T10–T14 + M2/14b), uz PROFIL, JEZGRA, STORE i CLI otprije
    gotove. Time su ispunjeni svi preduvjeti za DESKTOP (T29–T33) OSIM sučelja.
- **Stanje ostalog, nepromijenjeno ovim spajanjem:** SUČELJE (`sokratis.ui`, `feat/ui`) — M2/20–M2/24 i
  M2/25 (okvir s `api.ts`) SPOJIVO, **M2/26 sad recenziran SPOJIVO**, **M2/27** (SVG-grafovi u pogledu)
  u krugu popravka — vizualni nalaz: prsten (Ring) prevelik, bez legende; **M2/28** slijedi. Ništa od
  SUČELJA još nije u `main`-u. Sedam stabala i dalje na disku.
- **Opseg preostatka ove sesije nepromijenjen (Leonova odluka):** DESKTOP (T29–T33), INTEGRACIJA
  (T34–T35) i završna recenzija cijelog M2 su **treća sesija**.
- **Čuvar dokumentacije (način A), ovaj zapis:** `PROGRESS.md` (ovaj odlomak), `CHANGELOG.md` (redak
  M2/14b pod `[Unreleased]`, izričito označen kao cigla izvan plana), `RUST.md` §4 (novi retci:
  `Box<dyn Error + Send + Sync>`, `Stdio::piped()` + `take()`, inverz parsera/okrugli put; dopuna
  postojećeg retka o trait-objektu za `&dyn CommitCache`/`Option<&dyn CommitCache>` kao posuđeni,
  zamjenjivi trait-objekt), `ROADMAP.md` („Gdje smo", status M2 — tok IO gotov u cijelosti),
  `CLAUDE.md` („Stanje — TRENUTNO": M2/14b gotov, keš ima potrošača u `io`, adapter čeka T29),
  `BACKLOG.md` (dva nova poznata ograničenja). `docs/plan/superpowers/…m2-desktop.md` NIJE dirana (plan
  je zapis; da je M2/14b izvan njega stoji ovdje, ne tamo). `ARCHITECTURE.md` namjerno nedirana (čuvar
  B je piše cijelu za IO/STORE/CLI na kraju sesije). `cargo run -q -p sokratis-cli -- docs .` provjeren
  prije i poslije izmjena.

**Nastavak iste sesije — M2/26 i M2/27 vizualno potvrđeni, sesija stala zbog usagea, PLAN_DESIGNE.md zapisan.**

- **Orkestrator je vizualno provjerio SUČELJE u pregledniku** (`npm run dev`): M2/25 (okvir), **M2/26**
  (Pregled — kartice projekata) i **M2/27** (Tempo · Vrste rada · Pokazatelji · Faze) su potvrđene, uz
  po jedan krug popravka za M2/26 (`54aff7d`: `sortProjects` s fiksnim localeom) i M2/27 (`5da6d9e`:
  prsten vrsta rada ograničen i uz tablicu s legendom — prije je zauzimao cijeli zaslon bez legende).
  **M2/28 (Dnevnik · Isporuke · Vizije · Dokumentacija) NIJE započeta.** Ništa od SUČELJA nije spojeno
  u `main` — spajanje, DESKTOP (T29–T33), INTEGRACIJA (T34–T35) i završna recenzija M2 ostaju otvoreni.
- **Leon je izravnom porukom zatražio da mu se otvori sučelje** (dev-poslužitelj + preglednik); ocjena:
  „odlično, ali na izgledu i dodacima treba još dosta rada", i da se **pripazi na dokumentaciju** jer je
  u kratkom vremenu izgrađeno puno. Iz toga je nastao **`docs/product/PLAN_DESIGNE.md`** (2026-09-21,
  `e570e9e`, preseljen u `product/` ciglom `685def0` jer je u `plan/` rušio branu jednog aktivnog speca)
  — Leonov ZAPIS NAMJERE, ne spec: animirano otvaranje grafova i pogleda, kartica s objašnjenjem na
  klik, tema i jezik u Postavke, više statistike, kasnije timovi i ne-samo-git, i **rez za 1.0.0**.
- **Sesija je stala jer je Leonu ponestalo usagea (~92%) — ne zato što je tok gotov ili je milestone
  stigao do dogovorenog zastanka.** Opseg druge sesije (Leonova odluka: sve osim DESKTOP-a i
  INTEGRACIJE) time NIJE ispunjen — nedostaje T28, spajanje SUČELJA i čuvar B.
- **Ovaj zapis (čuvar B, sonnet — Leonov usage pri kraju, opus-prolaz ARCHITECTURE-a ostaje za izdanje):**
  `PROGRESS.md` (ovaj odlomak), `CLAUDE.md` („Stanje — TRENUTNO", „Agenti" — bitno sažeto, CLAUDE.md
  drži samo što vrijedi SAD), `docs/architecture/ARCHITECTURE.md` (STORE/IO/CLI/M2/14b dobivaju
  opis „što danas radi" i pošten §11 bez zastarjelih tvrdnji — „until je iznimka", „bez potrošača",
  Detached HEAD kao otvoren rub), `docs/README.md` (odjeljak „plan/ — što sada"),
  `docs/workflow/AGENTI.md` §2, `docs/workflow/TESTING.md` (store-red), `docs/plan/ROADMAP.md`
  („Gdje smo", status M2), `README.md` (korijen). **NOVA-SESIJA-PROMPT.md je prepisan 2026-09-21**
  (izvan ovog čuvarova djelokruga, `.superpowers/` nije `docs/`): sljedeća sesija je **PLANIRANJE**
  (rez za 1.0.0 iz PLAN_DESIGNE, odluke u `DECISIONS.md`, tek onda spec/plan), ne nastavak T28.
  `cargo run -q -p sokratis-cli -- docs .` i `signals .` provjereni prije i poslije izmjena.

### Što slijedi
**Sljedeća sesija je PLANIRANJE, ne nastavak gradnje:** rez za 1.0.0 iz `docs/product/PLAN_DESIGNE.md`
(do koje cigle se zatvara verzija 1.0.0), odluke u `DECISIONS.md`, tek onda spec/plan po uobičajenom
putu. Redoslijed gradnje nakon odobrenog reza ostaje onaj iz plana M2: T28 → spajanje SUČELJA u `main`
→ DESKTOP (T29–T33) → INTEGRACIJA (T34–T35) → završna recenzija M2. Šest tokova (KOSTUR · JEZGRA ·
PROFIL · STORE · IO · CLI) su gotovi i u `main`-u; SUČELJE je jedini nespojeni tok. Ledger:
`.superpowers/sdd/2026-09-18-m2-desktop/progress.md`, odjeljak „▶▶▶ STANJE ZA NOVU SESIJU" na dnu, i
`NOVA-SESIJA-PROMPT.md` pored njega.

---

## 2026-09-21 (FABLE) — Planiranje: rez za 1.0.0 (S-024…S-031), dopuna speca M2 §13

**Sesija planiranja, bez ijedne cigle koda.** Brainstorming s Leonom iz njegova zapisa namjere
(`PLAN_DESIGNE.md`): devetnaest pitanja u jednoj poruci (Leon je tražio sva odjednom), prijedlog
crte, Leonov OK („super je sve možeš krenuti“).

- **Rez:** izlaz iz M2 je **1.0.0**, 0.2.0 se preskače (S-024). Orkestratorova preporuka je bila
  najmanja crta (ništa iz zapisa namjere); **Leon je odlučio** da u 1.0.0 ulaze kartica s
  objašnjenjem, tema i jezik u Postavkama te animacije grafova i pogleda. Crta, tri etape
  (funkcija → izgled → izdanje) i „1.0.0-pre“ nakon prve: S-025. Pojedinačne odluke: S-026 animacije
  · S-027 kartica · S-028 Postavke · S-029 instalater · S-030 README engleski · S-031 dokumentacija.
- **Spec M2 dopunjen, ne zamijenjen** (`plan/` smije imati jedan aktivni spec): nov §13 s osam stavki
  i „gotovo kad“ za svaku; §6.1, §6.2, §11 i §12 pokazuju na njega.
- **Druga verzija i kasnije** su u `BACKLOG.md` (dva nova odjeljka): ocjena projekta F−…A+, omjer
  popravaka, vlastiti dodaci, kartica s dokazom, punjenje trenda, izvoz, iOS izgled · timovi, ne samo
  git, prodaja. Zabilježen je sudar „timovi preko vlastitog Supabasea“ s PRD §5 „sve lokalno“.
- **`PLAN_DESIGNE.md` je ispunio svrhu i preseljen u `archive/`** s pečatom (S-031) — nije postao
  treći izvor istine pored speca i odluka.
- **README preveden na engleski** (S-030). Rečenica o engleskom koja je u drugoj sesiji stigla unutar
  rezultata alata time je zatvorena: Leon ju je potvrdio izravnom porukom, i to samo za README.
- **Grane i stabla pet spojenih tokova obrisani** (jezgra · profil · store · io · cli; lokalno i tri
  grane na remoteu), nakon provjere da su potpuno spojeni i čisti. Leon je dao trajno dopuštenje za
  takve grane — `CLAUDE.md` pravilo #1 je dopunjeno. Na disku su `main` i `sokratis.ui`.
- **Točnost dokumentacije:** iz `CLAUDE.md`, `ROADMAP.md`, `docs/README.md` i `AGENTI.md` izbačene
  su tvrdnje koje su ovom sesijom postale netočne („sedam stabala“, „sljedeća sesija je planiranje“,
  „push traži izričit OK“, „kasnije open-source“) i kronologija koju drugi dokument već drži.
  Veliko čišćenje (PROGRESS po milestoneu, ARCHITECTURE tvrdnju po tvrdnju) je cigla §13.8.
- **Plan cigli dopunjen** (`docs/superpowers/plans/2026-09-18-m2-desktop.md`, T36–T43): repo bez konvencija ·
  instalater · Postavke s prekidačem pokreta · ulaz grafova · prijelaz pogleda · kartica ×2 · dokumentacija; T35 sada
  postavlja verziju 1.0.0. Pri pisanju plana prekidač pokreta je preseljen uz Postavke (spec §13.3 usklađen).
- Brane: samo dokumentacija — `sokratis docs .` i `signals .` prije pusha.

### Što slijedi
**Prva sesija izvedbe etape 1: T28 (Dnevnik · Isporuke · Vizije · Dokumentacija) → recenzija i
vizualna provjera → spajanje SUČELJA (T20–T28) u `main` → pune brane → push → STANI.** Zatim, svaka
u svojoj kratkoj sesiji: DESKTOP T29–T33 · T34 + repo bez konvencija + instalater („1.0.0-pre“) ·
izgled (pet cigli) · izdanje. Ledger: `.superpowers/sdd/2026-09-18-m2-desktop/progress.md` i
`NOVA-SESIJA-PROMPT.md` pored njega.

## 2026-09-21 (FABLE) — Izvedba S1: T28 + spajanje SUČELJA (T20–T28) u `main`

**Prva od pet kratkih sesija izvedbe nakon reza za 1.0.0.** Opseg: T28 + spajanje. DESKTOP nije
započet.

- **Cigla M2/28** (tri commita u `feat/ui`): `3e6355d` četiri preostala pogleda — Dnevnik s
  overrideom u mjestu (`<select>` pet vrsta → `setOverride` → `loadReport`, oznaka „ručno" +
  vraćanje klasifikatora), Isporuke (iz `report.deliveries`, najnovije prvo, 🚀 za deploy), Vizije
  (pilule `vision_totals`, dodaj/obriši s potvrdom u retku), Dokumentacija (`docs === null` →
  `docs.na`, ocjena/100, kašnjenje, nalazi, klik na putanju kopira) · `893fef1` uređivanje postojeće
  vizije istim obrascem kao dodavanje (spec §5 traži „dodaj · uredi · promijeni stanje", brif je
  imao samo dodaj/obriši — orkestratorov Ruling) · `f666503` širine stupaca za najuži prozor
  (`minWidth: 960`): naslov commita u Dnevniku imao je 17 px na 960×600, `<select>` je rezao
  „vođenje dokumentacije" — nađeno ORKESTRATOROVIM MJERENJEM u pregledniku, ne recenzijom koda
  (pouka T27 potvrđena drugi put: širine se mjere na najužem dopuštenom prozoru, ne samo na 1280).
- Nove čiste funkcije u `apps/desktop/src/views/helpers.ts`, sve test-prvo: `sortDiary`,
  `sortDeliveries`, `copyText`, `joinRepoPath`, `replaceVisionAt`, `parsePercent`. `App.svelte`
  usmjerava svih devet pogleda.
- Svjesno odstupanje od speca §5.1: nema Rust naredbe `copy_path` — kopiranje ide kroz
  `navigator.clipboard.writeText` (`copyText`), WebView2 je ima uz korisničku gestu (zapisano u
  planu, T28 i T34).
- **Spajanje: `da77f90` `merge: SUCELJE (T20-T28)`.** Pune brane na `main`-u nakon spajanja: `cargo
  fmt --check` OK · `cargo clippy --workspace --all-targets -- -D warnings` OK · **136 Rust testova,
  1 ignoriran** · `npm run check` u `apps/desktop` (svelte-check 207 datoteka/0 grešaka ·
  check:i18n 160 ključeva hr=en · check:contrast 4/4 teme · vitest 66 testova u 9 datoteka) ·
  `npm run build` OK · `sokratis docs .` 100/100 · `signals .` 0. Pushano.
- **Stabla i grane:** `sokratis.ui`/`feat/ui` obrisani (lokalno i na remoteu) čim je merge potvrđen
  potpuno spojen — isto dopuštenje kao za JEZGRU/PROFIL/STORE/IO/CLI 2026-09-21. **Na disku je JEDNO
  stablo (`main`), nespojenih grana nema.**
- Poznata ograničenja koja SUČELJE otkriva, zapisana u `ARCHITECTURE.md` §11 (jedno mjesto): tihe
  greške u sučelju bez stanja greške (rješava T34), `Vision.state` slobodan tekst bez prijevoda,
  potvrda brisanja vizije viri 13 px preko ruba tablice (kozmetika), stupac `model` u Isporukama
  ponekad nosi ostatak zaglavlja dnevnika (parser isporuka u jezgri).

### Što slijedi
**S2 = DESKTOP T29–T33** u novom stablu `sokratis.desktop`, grana `feat/desktop` iz `main`-a. Karta
preostalih sesija je u ledgeru/planu (`CLAUDE.md` „Tri etape").

---

## 2026-09-21 (FABLE) — Izvedba S2: DESKTOP (T29–T33 + tri pred-cigle) spojeno u `main`

**Druga od pet kratkih sesija izvedbe nakon reza za 1.0.0.** Opseg: DESKTOP T29–T33, serijski (jedno
stablo, cigle dijele iste datoteke). Leon je usred sesije poručio „Imaš moj izričit OK za sve" — OK
vrijedi za opseg S2; tag/izdanje/LICENCA/force-push ostaju izvan i dalje se ne diraju.

- **Stablo `sokratis.desktop`, grana `feat/desktop` iz `ce8a6f8`.** Pre-flight scan (parovi cigli koji
  dijele datoteku ili sučelje, provjereni protiv plana i speca prije prve cigle) otkrio je devet
  stvarnih rupa i sukoba — presuđeni rulinzima R1–R12, od kojih su najvažniji: mapa `reports` drži
  SAMO izračun nad cijelim rasponom, `get_report` u nju ne piše (R2, spec §5.1 protiv teksta plana);
  motor ne čita spremljeni raspon iz postavki, uvijek računa `all` (R3); teret oba događaja ide po
  specu, ne po planu (R4); dnevna snimka se piše pri SVAKOM izračunu, ne samo prvom (R9); naredbe i
  tray idu kroz ISTI red čekanja kao watcher, `request_refresh` (R10 — otvoren tek u recenziji T30,
  krug popravka); `tray@2x.png` se ne pravi jer Windows tray sam skalira (R11).
- **Tri pred-cigle IZVAN vlasništva DESKTOP-a** (tokovi JEZGRA/IO su zatvoreni; plan T29 sam imenuje
  prvi lijek): **M2/29a** `178ee34` `CommitRow.author_time` (namjeran snapshot, +190 redaka, SAMO to
  polje, S-022) · **M2/29b** `c218ef6` `sokratis_io::today()` javno + `common_dir` normaliziran
  (S-015 — dva radna stabla istog repozitorija su prije bila dva projekta u registru, tekstualna
  usporedba putanje se razlikovala u velikom/malom slovu diska i završnom `\`) · **M2/29c** `6753f74`
  `alerts_raised` determinističan (duplikat u `cur` se javlja jednom, u `prev` vrijedi najteža
  težina). Recenzija: SPOJIVO, 2 Minor odgođena.
- **T29** (`2363735` → popravak `2f772ad`): `AppState` (pet polja iza `Mutex`), `StoreCache` (adapter
  `CommitCache`), `ProjectSummary`/`summarize`, jedanaest naredbi, `Range`/`Settings`. Recenzija je
  prvo vratila ciglu — `add_project` je upisivao stablo iz kojeg je dijalog otvoren umjesto GLAVNOG
  stabla (`main_root`, S-015: projekt bi ovisio o opstanku sporednog stabla); popravljeno, ponovna
  recenzija SPOJIVO.
- **T30** (`f39e418`, krug popravka `3d42a77`): `engine.rs` — motor osvježavanja. Recenzija je otvorila
  **Ruling R10**: naredbe su zvale `refresh_project` MIMO reda čekanja, pa su watcher i naredba mogle
  računati isti projekt istodobno — spec §3.3 t. 3 je obvezujući; popravak je uveo `request_refresh`
  kao JEDINI ulaz (nit watchera, prvi izračun pri pokretanju, tri naredbe, tray). Ponovna recenzija
  SPOJIVO.
- **T31** (`70501dc`): `splash.rs` — tri `AtomicBool` (animacija, prvi izračun, već prikazano),
  `Ordering::SeqCst` potvrđen NUŽAN (dvije zastavice se čitaju/pišu unakrsno iz dvije niti, Dekkerov
  obrazac), rezerva 10 s. Recenzija: SPOJIVO bez nalaza.
- **T32** (`9dab7e6`): `tray.rs` — izbornik (Otvori · Osvježi sve · Autostart · Izađi), `single-instance`
  prvi plugin, X sakriva glavni prozor, autostart kroz plugin (baza se mijenja SAMO ako plugin uspije).
  Recenzija: SPOJIVO uz **Ruling R12 parked** — ako plugin autostarta uspije a upis u bazu padne,
  kvačica privremeno laže dok se ponovno ne klikne; stvaran rub, ali dokaz bi tražio stvarno
  uključivanje autostarta na ovom stroju (Leonova uputa: ne u S2) — ide u krug popravaka S5.
- **T33** (`f315861`): tray-ikona bez lika, izrezana iz `graph.webp` (Leonov izvor, odabran
  2026-09-20). Orkestrator provjerio gledanjem na stvarnim 16×16 i 32×32 (uvećano 10×): na 32 px
  prsten/stupci/linija razlučivi, na 16 px lik se stapa (očekivano, zato je izbačen).
- **Spajanje: `cc74bb8` `merge: DESKTOP (T29-T33)`** (`--no-ff`). Pretraga tajni/imena/putanja nad
  `main...feat/desktop` (19 datoteka, +1669/−17): ništa nađeno. Pune brane na `main`-u: `cargo fmt
  --check` OK · `cargo clippy --workspace --all-targets -- -D warnings` OK · **142 testa, 1
  ignoriran, 0 palo** (136 + 2 io + 2 jezgra + 2 desktop `Range`) · `npm run check` (i18n 160 hr=en ·
  kontrast 4/4 · vitest 66/66) · `npm run build` OK · `sokratis docs .` 100/100 · `signals .` nema
  signala. Pushano (`ce8a6f8..cc74bb8`), `main` = `origin/main`.
- **Dimni test `npm run tauri dev`, U STABLU PRIJE SPAJANJA** (`LOCALAPPDATA` preusmjeren u
  scratchpad, da razvoj ne dira Leonovu pravu bazu — T37 tek odvaja razvojnu bazu): splash vidljiv dok
  je glavni skriven (t≈1,6 s) → glavni vidljiv t≈7,2 s (≈5,6 s animacija + učitavanje) → tray stvoren
  → druga instanca izlazi sama, kod 0 → X sakriva glavni (proces živi) → ponovno pokretanje `.exe`-a
  podiže skriveni prozor. Baza je nastala samo u scratchpadu; Leonov pravi `%LOCALAPPDATA%\sokratis`
  nije dirnut. **NIJE provjereno u S2** (ide na ručnu listu T34): izgled ikone u pravom trayu na
  100 %/200 %, desni klik → izbornik, kvačica autostarta ↔ `HKCU\…\Run`, obavijest na prijelaz u
  Alert, preskok splasha klikom, `prefers-reduced-motion`.
- **Stablo `sokratis.desktop` i grana `feat/desktop` obrisani** nakon provjere (`--merged main` da,
  `merge-base --is-ancestor` da, stablo bez izmjena; grana nikad nije bila na remoteu). **Na disku je
  JEDNO stablo (`main`), jedina grana `main`.**
- **Poznata ograničenja otvorena ovom sesijom** (u `ARCHITECTURE.md` §11, jedno mjesto): sučelje do
  T34 radi SAMO nad `MockApi` (naredbe postoje, ništa ih još ne zove); klik na obavijest ne otvara
  projekt (plugin na Windowsu nema povratni poziv); autostart R12 (gore); natpisi traya se ne
  mijenjaju s jezikom do ponovnog pokretanja; druga instanca otvori bazu prije nego što je
  `single-instance` odbije (bezopasno, SQLite je idempotentan); `project_worktree.branch` se puni
  praznim tekstom (`io::worktrees()` ne daje granu); do T37 `tauri dev` piše na PRAVU putanju baze.

### Što slijedi
**S3 = INTEGRACIJA T34 → T36 → T37** u novom stablu `sokratis.int`, grana `feat/integration` iz
`main`-a — `TauriApi` (sučelje prestaje raditi nad `MockApi`), repo bez konvencija, instalater NSIS;
nakon T34 Leon instalira „1.0.0-pre" (S-025, prva etapa). Ledger:
`.superpowers/sdd/2026-09-18-m2-desktop/progress.md` i `NOVA-SESIJA-PROMPT.md` pored njega.

## 2026-09-22 (FABLE) — Izvedba S3: INTEGRACIJA (T34, T36, T37) spojena u `main`

**Treća od pet kratkih sesija izvedbe.** Opseg: INTEGRACIJA T34 → T36 → T37, serijski u jednom stablu
(`sokratis.int`, grana `feat/integration` iz `cc74bb8`). Time je **etapa 1 „funkcija" (S-025)
cjelovita u kodu** — preostaje da Leon instalira i mjeri (§13.1).

- **T34 — `TauriApi`** (`029601c`): sučelje prestaje raditi SAMO nad `MockApi`. Svaka metoda `Api`-ja
  je sad `invoke('<naredba>', {…})` s imenima naredbi/argumenata iz `commands.rs`; `createApi()` bira
  izvedbu po `'__TAURI_INTERNALS__' in window`. Događaj `report_updated` seli u JEDNU globalnu
  pretplatu u `App.svelte` (prije: `Overview.svelte` posebno) — promjena u repou sad osvježi pogled u
  kojem korisnik trenutačno stoji, ne samo Pregled. Svaki poziv `api.*` hvata grešku u traku greške
  (`role="alert"`); „zadnji zahtjev pobjeđuje" za `getReport`. Nošeni nalazi T25/T26 (tihe greške)
  time zatvoreni.
- **T36 — repo bez konvencija** (`28f311e`, spec §13.7): dva nova testa u `io` (repo bez `docs/`/
  dnevnika/plana/`.sokratis/`; repo s jednim praznim commitom) otkrila su **Ruling R21** — `closed_phases()`
  je iz ZADANOG profila (S-005) vraćao sve četiri Sokrat Studyjeve zatvorene faze i za repo s nula
  pogodaka; popravak filtrira `commits > 0` prije nego faza uđe u `Report.phases` (isti nalaz kao I8,
  ali za tablicu faza, ne pokazatelje). Snapshot jezgre nepromijenjen. Devet pogleda sučelja već ima
  natpis praznog stanja — nijedan `.svelte` nije trebalo dirati.
- **T37 — instalater** (`b1b137a`, S-029): verzija ima JEDAN izvor (`[workspace.package] version =
  "1.0.0-pre.1"`, `package.json` zrcalo s testom, `tauri.conf.json` bez `version`) — CLI je time isto
  1.0.0-pre.1. `bundle` NSIS `currentUser`, bez potpisa i auto-ažuriranja. `state::db_file_name`
  razdvaja razvojnu bazu (`tauri dev` → `sokratis-dev.db`) od instalirane (`sokratis.db`) preko
  `cfg!(debug_assertions)`. Nova ovisnost: nijedna.
- **Spajanje: `b560f7c` `merge: INTEGRACIJA (T34, T36, T37)`** (`--no-ff` na `main` iz `3819ab8`).
  Pune brane: `cargo fmt --check` OK · `cargo clippy --workspace --all-targets -- -D warnings` OK ·
  **145 Rust-testova + 1 ignoriran** (bilo 142) · `npm run check` (svelte-check 212 datoteka/0 ·
  i18n **162** ključa hr=en, bilo 160 · kontrast 4/4 · **vitest 82**, bilo 66, 11 datoteka) ·
  `npm run build` OK (main.js 143,92 kB / gzip 34,86) · `sokratis docs .` 100/100 · `signals .` nema
  signala. Pushano, `main` = `origin/main`.
- **Dimni test, u stablu prije spajanja** (`tauri dev`, `LOCALAPPDATA` preusmjeren, provjera kroz CDP
  nad vlastitim webviewom umjesto snimke ekrana): `TauriApi` živ, dijalog mape dodaje projekt, brojke
  Tempa = `sokratis report --json` (178 commita · 5 dana · 23,7 h), baza `sokratis-dev.db`, „Osvježi" →
  `report_updated` → popis i izvještaj ponovno učitani. Ostaje na ručnoj listi za Leona na instaliranoj
  „1.0.0-pre": tray na 100 %/200 %, izbornik, „Izađi", autostart ↔ `HKCU\…\Run`, obavijest na Alert,
  preskok splasha, `prefers-reduced-motion`, osvježenje bez klika sa štopericom, četiri teme.
- **Stablo `sokratis.int` i grana `feat/integration` obrisani** nakon provjere (spojeno, čisto). **Na
  disku je jedno stablo (`main`), jedina grana `main`.**

### Što slijedi
**Etapa 2 „izgled" (S-025) — S4 = SUČELJE-2, cigle T38–T42** (Postavke §13.2, animacije §13.3,
kartica s objašnjenjem §13.4) u novom stablu `sokratis.ui2`, grana `feat/ui-2`. Zatim S5: T35
(mjerenja, verzija 1.0.0) → završna recenzija → jedan krug popravaka → §13.8 (čuvar izdanja) → STANI,
tag traži Leonov izričit OK. Ledger: `.superpowers/sdd/2026-09-18-m2-desktop/progress.md` i
`NOVA-SESIJA-PROMPT.md` pored njega.

## 2026-09-23 (FABLE) — Izvedba S4: SUČELJE-2 (T38–T42) spojena u `main`, etapa 2 „izgled" gotova

**Četvrta od pet kratkih sesija izvedbe.** Opseg: SUČELJE-2 T38 → T39 → T40 → T41 → T42, serijski u
jednom stablu (`sokratis.ui2`, grana `feat/ui-2` iz `e5f5e51`). Time je **etapa 2 „izgled" (S-025)
cjelovita u kodu** — preostaje etapa 3 „izdanje" (S5).

- **T38 — pogled Postavke** (`bcf092f` + popravci `808c33d`, `b097acc`): deseti, globalni pogled
  (tema · jezik · autostart · animacije); `Settings.motion: bool` (Rust `SETTING_KEYS` postaje 4);
  `data-motion` na `<html>` gasi sav pokret preko jedne varijable `--motion-dur`. Dva nalaza dimnog
  testa: Sidebar je bez odabranog projekta prikazivao SAMO Postavke (R29 — `overview` sad uvijek
  vidljiv) i test trajanja nikad nije stvarno pogađao CSS-deklaracije (R30, popravljen regex).
- **T39 — ulaz grafova** (`a75089d`): stupci rastu, linije i sparkline se iscrtavaju
  (`stroke-dashoffset`), prsten se otkriva SVG-maskom (`$props.id()` drži `id` maske jedinstvenim po
  instanci). **R25, izmjereno u `tauri dev`:** WebView2 javlja prozor vidljivim i dok je skriven iza
  splasha, pa `visibilitychange` NIKAD ne okida — `onFocusChanged` (fokus stiže iz `splash.rs`) je
  pravi okidač prve animacije; oba čuvara ostaju u kodu, idempotentna.
- **T40 — prijelaz pogleda i učitavanje** (`253cbc3`): ulaz 250 ms po pogledu, stari sadržaj prigušen
  dok novi izvještaj stiže, kostur pri prvom učitavanju projekta (dimni test: pojavio se 8 ms nakon
  klika, nestao na 551 ms).
- **T41 — kartica s objašnjenjem, mehanizam + 18 pokazatelja** (`b2580a9` + popravak `6f2c74c`):
  `Explainable`/`ExplainCard` (R31 — preimenovana s brifova `Explain.svelte`, jer se na Windowsu
  sudara neosjetljivo na veličinu slova s `explain.svelte.ts` u istoj mapi); „formula na klik" postaje
  kartica. Recenzija je vratila pet natpisa koji su parsiranje jezgre pripisivali `io`-u — ispravljeno.
- **T42 — kartica u svim pogledima** (`bcc9ddc` + popravak `ec5e377`): 37 id-eva ukupno (18 + 19),
  svih devet pogleda i traka signala imaju barem jedno objašnjenje; kartica projekta na Pregledu
  razdvojena na naslov-gumb i tri objašnjive sestre (R26 — ugniježđeni gumbi nisu valjan HTML).
  Recenzija je vratila tri natpisa — ispravljeno.
- **Spajanje: `6947189` `merge: SUCELJE-2 (T38-T42)`** (`--no-ff` na `main` iz `e5f5e51`; 39 datoteka,
  +1006/−103, nijedna nova ovisnost, Rust dirnut samo u `commands.rs`). Pune brane: `cargo fmt --check`
  OK · `cargo clippy --workspace --all-targets -- -D warnings` OK · **145 Rust-testova + 1 ignoriran,
  0 palo** (nepromijenjeno) · svelte-check **235** datoteka/0 · i18n **283** ključa hr=en (bilo 162) ·
  kontrast 4/4 · **vitest 93**, 13 datoteka (bilo 82) · `npm run build` OK (main.js 208,57 kB /
  gzip 53,66; main.css 22,80 kB) · `sokratis docs .` 100/100 · `signals .` nema. Pushano, `main` =
  `origin/main`.
- **Dimni test S4** (`tauri dev --config identifier=dev.sokratis.app.dev` UZ instaliranu 1.0.0-pre.1 —
  `single-instance` po identifikatoru, bez sudara; `LOCALAPPDATA` u scratchpad; CDP nad vlastitim
  webviewom): Postavke bez teme/jezika u gornjoj traci, postavke prežive reload, sve četiri vrste
  grafova animiraju, `data-motion=off` svodi trajanja na 0. Kartica s objašnjenjem NIJE vizualno
  provjerena okom (samo testovi + strukturna provjera) — ostaje na Leonovoj ručnoj listi.
- **Nalaz za S5 (jezgra):** `closed_phases_in_range` (`indicators.rs:80-83`) gleda samo `since`, ne
  `until` — tekst kartice to istinito opisuje; popravak jezgre čeka krug popravaka S5.
- **Stablo `sokratis.ui2` i grana `feat/ui-2` obrisani** nakon provjere (spojeno, čisto). **Na disku
  je jedno stablo (`main`), jedina grana `main`.**

### Što slijedi
**Etapa 3 „izdanje" — S5:** T35 (mjerenja, verzija 1.0.0) → završna recenzija (opus, `8edf3df..main`)
→ jedan krug popravaka → §13.8 (čuvar izdanja) → STANI, tag traži Leonov izričit OK. Ledger:
`.superpowers/sdd/2026-09-18-m2-desktop/progress.md` i `NOVA-SESIJA-PROMPT.md` pored njega.
