# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Spec M1 je ispunjen i
> arhiviran ([archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md)), **spec M2 je aktivan**
> ([ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md)); što je izgrađeno opisuje
> [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md), što je isporučeno
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-18)

**M0 gotov. M1 je zatvoren — verzija 0.1.0.** Sve cigle T1–T22 su u `main`-u: KOSTUR (T1), FIXTURE
(T2, T2b, T2c), PARSE (T3–T6), METRIKE (T7–T11), DOCS+PRAVILA (T12–T14), IO (T15–T17), CLI (T18–T19),
INTEGRACIJA (T20–T22), a nad njima je prošla **završna recenzija cijelog lanca** s jednim krugom
popravaka (spojen u `main`; ponovna recenzija SPOJIVO). `sokratis report/docs/signals` rade nad
pravim repozitorijem, paritet s `RAD.xlsx` je test (`crates/sokratis-core/tests/parity.rs`), a
Sokratis mjeri i sam sebe (vlastiti `.sokratis/profile.json`). Brojke, popravci i broj testova:
`records/CHANGELOG.md` (0.1.0); tijek sesije: `records/PROGRESS.md`; što je izgrađeno i što još ne
radi: `architecture/ARCHITECTURE.md` (§11).

**Leon je dao izričit OK 2026-09-18:** svih 8 grana tokova i njihovih radnih stabala je obrisano
(sve su bile spojene) — `main` je sada jedina grana i jedino stablo. Time je Milestone 1 zatvoren.
Toolchain je pinan (`rust-toolchain.toml`, 1.98.1) i definicije agenata su u repou
(`.claude/agents/*.md`) — obje odluke Leon je odobrio isti dan. Na Leona i dalje čeka (ne radi se
bez njegova OK-a): push i objava na GitHub (remote još ne postoji).

**Spec M2 je napisan 2026-09-18** ([ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md)) iz brainstorminga s Leonom
(trinaest odluka, pitanje po pitanje; zapisane kao S-012…S-022 u `records/DECISIONS.md`). Preuzima
**sedam od devet** stavki duga iz recenzije M1 (`records/BACKLOG.md` sad drži samo dvije i pointere).
**Leon je spec odobrio isti dan**, pa je napisan i plan cigli
([superpowers/plans/2026-09-18-m2-desktop.md](../superpowers/plans/2026-09-18-m2-desktop.md): 35 cigli u
9 tokova, vlasništvo datoteka i ovisnosti među tokovima). Izvedba ide agentima kao u M1. M0 za M2 je
izmjeren (spec §10): jedina instalacija je `npm install` u `apps/desktop` (cigla T1), i ona čeka OK.

**Izvedba M2 je počela: T1 (kostur) je napola u `main`-u kao `M2/1a`** — koraci 1–12 (ugovor tipova,
crate `sokratis-store`, datoteke `apps/desktop`, pinane ovisnosti); **korak 13 čeka Leonov OK za
`npm install`** i radi se u novoj sesiji, pa je desktop crate dotad izvan workspacea. Što je time
ušlo u kod: `records/CHANGELOG.md` (Unreleased) i `architecture/ARCHITECTURE.md` §11. Stabla tokova
još ne postoje — otvaraju se nakon `M2/1b`.

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ **zatvoren 2026-09-18** (0.1.0, T1–T22 + krug popravaka u `main`; grane i stabla tokova obrisani uz Leonov OK) |
| **M2** | **Desktop** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja (sve 4 teme, `brand-*` iz loga) · `sokratis-store` (SQLite: registar · snimke · keš) · watcher · tray · autostart · obavijesti · svih 8 pogleda s uređivanjem · HR/EN · **znak Sokratisa** = ikona aplikacije + animacija pri pokretanju u kojoj logo stoji na mjestu slova „o" (Leonove datoteke od 2026-09-18, spec §5.2–5.4) · 7 od 9 stavki duga M1 — sve u [ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md) | …spec §11: pokrenuti Sokratis, vidjeti animaciju, Pregled sa Sokrat Studyjem kao jednim projektom od pet stabala, svih 8 pogleda s brojkama istim kao CLI, osvježenje bez klika nakon commita, obavijest na Alert iz traya | 🟨 **u izvedbi od 2026-09-18** — spec odobren, plan napisan (35 cigli), kostur T1 napola u `main`-u (`M2/1a`) |
| **M3** | **Objava** | ostala pravila · profil za tuđe projekte · HR/EN · instalater · znak · README EN · licenca · GitHub | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Objava i push na `main` = izričit OK** | vrijedi kad remote bude postojao; nijedno odobrenje se ne proteže |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
