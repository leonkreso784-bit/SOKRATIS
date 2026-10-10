# Zakrpa 1.0.2 — plan cigli

**Status:** 📋 AKTIVAN — napisan 2026-10-10 iz Leonove odluke iste sesije, nakon vanjske analize ocjene
dokumentacije (druga sesija, nad Leonovim drugim projektom; ime se ne piše u javni repo). Nalazi su
izmjereni prije plana: [`../records/PROGRESS.md`](../records/PROGRESS.md), unos 2026-10-10. Spec nema:
zakrpa popravlja kvarove pravila i **dodaje** jedno polje ugovoru `Report` (ništa postojeće se ne mijenja).
Izvodi se agentima po [`../workflow/AGENTI.md`](../workflow/AGENTI.md); numeracija nastavlja plan 1.0.1
(zadnja cigla T68). Plan nije izvor istine — što je izgrađeno kaže
[`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md).

**Cilj:** ocjena dokumentacije ne broji kao kvar ono što nije kvar (pauzirani plan s naslovom, primjer
poveznice u kodu), i uvijek kaže **iz kojeg stabla, grane i commita** je mjerena.

## Ograničenja (za sve cigle)

- **Nijedna nova ovisnost.** `sokratis-core` ostaje bez I/O-a (S-002).
- Težine kazni (`DocsWeights`) i način zbrajanja se **ne mijenjaju** — to je M3 (BACKLOG).
- Paritet s `RAD.xlsx` se ne dira (ocjena dokumentacije nije dio pariteta).
- Snapshot `crates/sokratis-core/tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap` mijenja
  **samo T71**, i to namjerno (S-022): novo polje, ostatak snimke ostaje bajt-za-bajt isti.
- Test-prvo (pravilo #7): fixture → očekivano → implementacija. Zaglavlje „zašto ovako" o konstruktu,
  2–5 redaka, ne dnevnik cigli (pravilo #5).
- Brane Rust-tokova u stablu: `cargo fmt --check && cargo clippy --workspace --exclude sokratis-desktop
  --all-targets -- -D warnings && cargo test --workspace --exclude sokratis-desktop` (`AGENTI.md` §5);
  tok sučelja: `npm run check`. Svako stablo ima svoj `target/`.
- **Mjeri prije i poslije** nad pravim repoom: `cargo run -p sokratis-cli -- docs <putanja>` nad ovim
  repoom i nad radnim stablom Sokrat Studyja (`sokratstudy.f6` ili drugo povezano stablo — glavna mapa
  `sokratstudy.dev` je ponekad `core.bare`, BACKLOG). Brojke idu u izvještaj.

## Tokovi

| tok | grana | stablo | cigle | vlasništvo datoteka |
|---|---|---|---|---|
| JEZGRA-3 | `feat/docs-rules` | `../sokratis.docs-rules` | T69 → T70 | `crates/sokratis-core/src/docs.rs`, `crates/sokratis-core/src/profile.rs` (samo `paused_marker`) |
| IZVOR | `feat/docs-source` | `../sokratis.docs-source` | T71 | `crates/sokratis-core/src/model.rs`, `crates/sokratis-core/src/report.rs`, `crates/sokratis-core/tests/**` (snapshot), `crates/sokratis-io/src/**`, `crates/sokratis-io/tests/**`, `crates/sokratis-cli/**` |
| SUČELJE-4 | `feat/docs-source-ui` | `../sokratis.docs-source-ui` | T72 | `apps/desktop/src/**`, `apps/desktop/tests/**` — **kreće tek kad je T71 u `main`-u** |
| IZDANJE-3 | `main` | `sokratis` | T73 | verzija, CHANGELOG, instalater (orkestrator) |

JEZGRA-3 i IZVOR se ne dodiruju po datotekama: T71 **ne dira** `docs.rs` — izvor mjerenja upisuje u
`report.rs` nakon što `docs_health` vrati rezultat.

## T69 — Pauzirani plan se prepoznaje u zaglavlju (JEZGRA-3)

Izmjereno 2026-10-10: plan oblika „`# Naslov`, prazan redak, `**Status:** ⏸️ PAUZIRAN`" broji se kao
**aktivan** (`^` bez `(?m)` je početak datoteke); isti status u prvom retku se prepozna.
- Status se traži u **zaglavlju plana**: tekst do prvog retka koji počinje s `## ` (ili cijela datoteka
  ako takvog retka nema). Zadani `paused_marker` dobiva `(?m)` i dopušta `⏸` sa ili bez `U+FE0F`.
  Spominjanje „PAUZIRAN" ispod prvog `## ` NE pauzira plan.
- Korisnički `paused_marker` iz profila primjenjuje se na isto zaglavlje (opis polja u
  `ARCHITECTURE.md` §4 dopunjuje čuvar).
- Testovi (test-prvo, `docs.rs`): naslov → prazan redak → status → pauziran · status u prvom retku →
  pauziran (postojeći test ostaje) · „PAUZIRAN" samo u tijelu ispod `## ` → aktivan · `⏸` bez `FE0F` →
  pauziran.

## T70 — Primjeri poveznica u kodu nisu poveznice (JEZGRA-3)

Izmjereno 2026-10-10: Markdown-poveznica na nepostojeći `nema.md` napisana u inline kodu, i ista
unutar `~~~` bloka, daju `dead-link`. (Primjer se ovdje ne piše doslovno: dok T70 nije izgrađen, ovaj bi
plan sam sebi dao dva nalaza — izmjereno pri pisanju plana.)
- `dead_links` preskače **ograđene blokove** po CommonMark pravilu: ograda je ≥ 3 ista znaka `` ` `` ili
  `~` (do 3 razmaka uvlake); blok zatvara ograda **istog znaka i barem iste duljine**; nezatvoren blok
  traje do kraja datoteke.
- Preskače i **inline kod**: raspon između niza od N backtickova i sljedećeg niza od točno N backtickova
  u istom retku. Poveznica izvan raspona u istom retku se i dalje provjerava.
- Testovi: `~~~` blok · ```` ```` ```` blok koji unutra ima ```` ``` ```` · inline `` `…` `` i
  ``` `` … `` ``` · prava mrtva poveznica u istom retku kao inline kod se i dalje javlja · broj retka
  nalaza ostaje točan.

## T71 — Izvještaj kaže što je mjerio (IZVOR)

Izmjereno 2026-10-10: dokumentacija se čita iz vodećeg stabla (S-033, `io/src/project.rs`, `Lead`), a
ni `Report` ni CLI ne kažu iz kojeg.
- `DocsHealth` dobiva polje `source: DocsSource` sa `worktree` (putanja stabla, kako je `io` već
  normalizira), `branch: Option<String>` (grana tog stabla; `None` za detached HEAD) i `commit` (puni
  SHA HEAD-a tog stabla). Ime tipa i polja mogu se prilagoditi kodu, ali značenje ostaje. `io` predaje
  izvor kroz `ReportInput`, a jezgra ga samo prenosi u `report.rs` (S-002). `docs.rs` se ne dira.
- **Stari zapisi:** ako se igdje deserijalizira spremljeni `Report`/`DocsHealth` (snimke u
  `sokratis-store`, keš), novo polje mora biti `#[serde(default)]` ili `Option`. Graditelj mjeri
  postoji li takvo mjesto i javlja.
- CLI `docs` (i `report --table` gdje ispisuje dokumentaciju) dodaje redak
  `mjereno: <grana ili "detached"> @ <7 znakova SHA> · <stablo>`. JSON nosi puna polja.
- Testovi: `io` integracijski test s dva radna stabla (pomoćnici `tests/common/mod.rs`): izvor je
  stablo s novijim commitom, grana i SHA su njegovi · detached stablo daje `branch = None` · CLI test
  za novi redak · snapshot jezgre namjerno osvježen (`INSTA_UPDATE`), diff snimke = samo novo polje.

## T72 — Ploča pokazuje što je mjereno (SUČELJE-4, nakon T71)

- `types.ts` zrcali novo polje; `MockApi` snapshot ga nosi (dolazi iz osvježene snimke T71).
- Sekcija Dokumentacija na ploči projekta (`views/project/DocsSection.svelte`) uz ocjenu pokazuje
  „Mjereno: grana X @ abc1234" (i18n HR/EN), puni SHA i putanja stabla u `title`. Kartica s
  objašnjenjem za ocjenu dokumentacije (`explain.*`) dobiva rečenicu da se mjeri vodeće stablo.
- Testovi: helper koji slaže tekst izvora (grana · detached · kratki SHA) u `tests/helpers.test.ts`
  ili uz komponentu; `npm run check`.

## T73 — Izdanje 1.0.2 (IZDANJE-3, orkestrator)

Kao T68 zakrpe 1.0.1 (`../archive/2026-10-10-zakrpa-1-0-1.md`): verzija 1.0.2 (`Cargo.toml` izvor,
zrcala), `cargo update -w --offline`, `npm run notices`, pune brane, CHANGELOG `[1.0.2]`, instalater iz
commita koji nosi tag. **Tag `v1.0.2` i izdanje traže Leonov izričit OK.**
