# AGENTI — kako se Sokratis gradi s više agenata na više grana

> Leon (2026-09-17): *„kreirat nekoliko agenata da se sustavno radi na cijelom projektu… više agenata
> na više branča a ti ih kontroliraš i nadzireš."* Ovo je protokol. Definicije agenata su u
> `.claude/agents/*.md`, praćene u repou od 2026-09-18 (prije toga globalno git-ignorirane). Plan koji
> izvode je **aktivni plan u `docs/superpowers/plans/`** — od 2026-09-18 to je
> `2026-09-18-m2-desktop.md` (M1 plan `2026-09-17-m1-jezgra-i-cli.md` je izvršen i ostaje kao zapis).

## 1 · Uloge

| uloga | tko | radi | ne radi |
|---|---|---|---|
| **Orkestrator** | glavna sesija (Claude) | reže plan na cigle, otvara grane i stabla, šalje graditelje, čita recenzije, **jedini spaja u `main`**, vrti brane na `main`-u, javlja Leonu | ne piše cigle tokova sam (osim T1 i integracije) |
| **graditelj** | subagent `.claude/agents/graditelj.md` | jedna cigla, test-prvo, u svom stablu, commit na svoju granu | push · main · tuđe datoteke · nove ovisnosti |
| **recenzent** | subagent `.claude/agents/recenzent.md` | dva prolaza (plan/spec → kvaliteta), pokreće testove sam, presuda SPOJIVO / VRATI | ne mijenja kod |
| **čuvar dokumentacije** | subagent `.claude/agents/cuvar-dokumentacije.md` | PROGRESS · CHANGELOG · pojmovnik · ROADMAP · audit prije compacta | kod, testovi, `.sokratis/` |
| **Leon** | vlasnik | OK za M0 (instalacija toolchaina), OK na kraju milestonea, OK za objavu/push; može prekinuti bilo što | ne mora čitati svaku ciglu — čita presude i PROGRESS |

Ako harness u sesiji ne nudi tipove `graditelj`/`recenzent`/`cuvar-dokumentacije` (definicije se učitavaju
pri startu), orkestrator šalje `general-purpose` agenta i u prompt zalijepi **cijeli sadržaj** odgovarajuće
`.md` definicije. Ponašanje mora biti isto.

## 2 · Tokovi, grane, radna stabla (Leonova konvencija: stabla su braća `sokratis.<tok>`)

| tok | grana | stablo | cigle | vlasništvo |
|---|---|---|---|---|
| KOSTUR | `feat/kostur` | `sokratis.kostur` | T1 | sve (orkestrator) |
| FIXTURE | `feat/fixtures` | `sokratis.fixtures` | T2 | `crates/sokratis-core/tests/fixtures/sokratstudy-*` |
| PARSE | `feat/core-parse` | `sokratis.parse` | T3 · T6 · T4 · T5 | `core/src/parse/**`, `core/src/classify.rs` |
| METRIKE | `feat/core-metrics` | `sokratis.metrics` | T7–T11 | `core/src/metrics/**`, `core/src/civil.rs` |
| DOCS+PRAVILA | `feat/core-rules` | `sokratis.rules` | T12–T14 | `core/src/docs.rs`, `core/src/rules/**` |
| IO | `feat/io` | `sokratis.io` | T15–T17 | `crates/sokratis-io/**` |
| CLI | `feat/cli` | `sokratis.cli` | T18–T19 | `crates/sokratis-cli/**` |
| INTEGRACIJA | `feat/integracija` | `sokratis.integracija` | T20–T22 | `core/src/report.rs`, `core/src/model.rs` (samo `WorkKind::id`), `tests/parity.rs`, `.sokratis/`, `docs/` |

Tokovi se **ne dodiruju po datotekama**, pa merge nema sudara. Jedina zajednička točka je T1 (ugovor tipova):
zato T1 ide prvi i sam, pa se spoji u `main` prije nego se otvore ostala stabla.

**Ova tablica je povijest M1.** Sve grane `feat/*` i stabla `sokratis.<tok>` iz nje su 2026-09-18
obrisana uz Leonov OK (sve su bile spojene) — objašnjava merge-povijest, ne trenutno stanje.

**Tokovi M2** (9: KOSTUR · JEZGRA · PROFIL · IO · STORE · CLI · SUČELJE · DESKTOP · INTEGRACIJA), njihove
grane, stabla, cigle, vlasništvo datoteka i **ovisnosti među tokovima** (koji se spaja prije kojeg) stoje
na **jednom mjestu**: `docs/superpowers/plans/2026-09-18-m2-desktop.md`, odjeljak „Struktura datoteka i
vlasništvo po tokovima" (S-010 — ovdje se ne prepisuju). Novo u M2: tok SUČELJE gradi TypeScript/Svelte,
pa su mu brane `npm run check` umjesto `cargo`; tokovi SUČELJE i DESKTOP nakon `git worktree add` pokreću
`npm ci` u svom stablu (`node_modules` je git-ignoriran). **T1 je cijel u `main`-u:** koraci 1–12 kao
`M2/1a`, korak 13 (`npm install` uz Leonov OK → ikone iz Leonova loga → desktop crate natrag u `members`)
kao `M2/1b`. **Koja su stabla otvorena kaže `git worktree list`, a koji je tok dokle stigao
`plan/ROADMAP.md`** — ovdje se to ne prepisuje (S-010). Stabla i grane tokova koji su **potpuno spojeni u
pushani `main`** orkestrator briše sam (Leonovo dopuštenje 2026-09-21, `CLAUDE.md` pravilo #1); tako su
2026-09-21 obrisana stabla JEZGRE, PROFILA, STOREA, IO-a, CLI-ja i (nakon merga T20–T28) SUČELJA — na
disku ostaje jedno stablo, `main`. Cigle iz dopune reza za 1.0.0 (spec M2 §13) idu istim protokolom;
stanje po cigli je u ledgeru `.superpowers/sdd/2026-09-18-m2-desktop/progress.md` (izvan gita).

## 3 · Protokol po cigli

```
orkestrator → graditelj(stablo, grana, tekst cigle, vlasništvo)
           ← izvještaj (commit, testovi, brane, odstupanja, novi pojmovi)
orkestrator → recenzent(stablo, cigla, izvještaj)
           ← presuda
   VRATI GRADITELJU → graditelj(isti kontekst + točan popis nalaza) → recenzent …  (najviše 2 kruga; treći = orkestrator sam)
   SPOJIVO → orkestrator: cigla je spremna; tok nastavlja na sljedeću ciglu
```

Graditelj dobiva **samo svoju ciglu**, ne cijeli plan — tako ne „popravlja" tuđe. Recenzent dobiva ciglu +
izvještaj, ne graditeljev razgovor — tako sudi kod, ne priču.

## 4 · Spajanje u `main` (samo orkestrator)

1. Tok završi sve svoje cigle sa SPOJIVO.
2. U `sokratis` (main): `git merge --no-ff feat/<tok> -m "merge: <tok> (T..–T..) -- <što donosi>"`.
3. Brane na `main`-u: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`. Crveno → merge se vraća (`git reset --hard ORIG_HEAD`) i tok dobiva nalaz.
4. **čuvar dokumentacije, način A** (PROGRESS, CHANGELOG, pojmovnik, ROADMAP).
5. Grana i stablo ostaju dok Leon ne kaže; brisanje = `git worktree remove ../sokratis.<tok>` + `git branch -d feat/<tok>`.

Redoslijed spajanja: T1 → tokovi kako završe (FIXTURE i PARSE obično prvi) → tek kad su SVI u `main`-u:
INTEGRACIJA (T20–T22) u svom stablu od svježeg `main`-a.

## 5 · Paralelnost i tempo

- Istodobno najviše **4 graditelja** (jedan po stablu) — Playwright suite Sokrat Studyja i Rust build dijele isti stroj.
- Recenzent ide odmah nakon svakog izvještaja; ne čeka se kraj toka.
- Orkestrator ne piše kod tokova; ako graditelj tri puta zapne na istoj cigli, orkestrator je preuzima sam i **zapiše zašto** u PROGRESS.
- **Leon se ne pita usred cigle** — osim kad cigla traži **instalaciju** ili **njegov materijal**: u M2 su
  to `npm install` (T1, korak 13 — odrađeno 2026-09-18 kao `M2/1b`) i izvor tray-ikone (T33 — riješeno 2026-09-20: Leon je odabrao `graph.webp` iz svoje animacije).
  Inače se pita: prije M0, na kraju milestonea (zastanak), prije pusha/objave.
- **Brane u stablima tokova rade bez desktop cratea** (`cargo ... --workspace --exclude sokratis-desktop`)
  — svako stablo Rust-tokova inače kompilira cijeli Tauri i traži `dist/`, koji ta stabla ne grade; pune
  brane s desktopom (`cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`, bez
  isključivanja) vrti orkestrator na `main`-u nakon svakog spajanja.

## 6 · Compact

Prije svakog compacta: **čuvar dokumentacije, način B** + orkestrator osvježi memoriju (stanje tokova: koja
je cigla gdje, što čeka recenziju, što je spojeno). Nakon compacta prva radnja je `git -C sokratis log --oneline -15`
i `git worktree list` — stanje se čita iz gita, ne iz sjećanja.

## 7 · Što se mjeri (dogfooding, radi od T22)

`sokratis signals .` nad ovim repoom prijavljuje `unmerged-branches` za tokove koji predugo žive izvan
`main`-a — orkestrator ga vrti nakon svakog spajanja. Na kraju M1 daje **0 signala** jer su sve grane
tokova bile spojene; pravilo je dokazano nad Sokrat Studyjem, gdje hvata dvije stvarno nespojene grane.
Grane i stabla tokova su 2026-09-18 obrisane uz Leonov OK — `main` je jedina grana i jedino stablo.
Sokratis nadzire vlastitu gradnju.
