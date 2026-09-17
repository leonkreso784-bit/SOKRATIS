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

### Što slijedi
**Compact** (Leonova riječ) → **M0 uz Leonov OK** (instalacija mijenja sustav) → T1 kostur na `main` →
grane i stabla → paralelni tokovi → spajanje → integracija → zastanak na kraju M1.
