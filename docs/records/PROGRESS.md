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

## 2026-09-18 (FABLE) — Brainstorming i spec M2 (desktop)

**Popodne. Sesija bez koda — samo spec.** (Prva polovica na Opusu 5, nastavak na Fableu 5.1.)

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

### Što slijedi
**Leon čita spec** (`docs/plan/ARHITEKTURA_M2.md`) i daje OK ili traži izmjene. Nakon OK-a:
`superpowers:writing-plans` → plan cigli u `docs/superpowers/plans/` → nov SDD ledger → agenti kao u
M1 (tokovi i stabla se otvaraju po planu M2). Prije prve cigle koja treba `node_modules`: Leonov OK
za `npm install`. Push/remote i dalje samo uz izričit OK.
