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

**Izvedba M2 je krenula: T1 (kostur) je cijel u `main`-u** (`M2/1a`+`M2/1b`) — ugovor tipova, crate
`sokratis-store`, `apps/desktop` s ikonama iz Leonova loga, `npm install` uz Leonov OK i desktop
crate natrag u `[workspace] members` (builda se, prvi put ~3 min). **Otvorena su pet radnih stabala
tokova** (`sokratis.jezgra` · `.profil` · `.io` · `.store` · `.ui`). **Tok PROFIL je gotov** (T8–T9,
merge `a2e9265`): ograda putanja iz profila (`inside_root`/`Profile::validate_paths`, jezgreni dio
duga I9) i `test_path_exclude`. **Tok JEZGRA je gotov** (T2–T7, merge `7d23c76` pa `7711a67`):
snapshot ugovora `Report`-a (dug I7, riješen), `until` kao gornja granica razdoblja (S-011), zbroj
vizija po stanju (dug I6, riješen), redci commita i isporuke u `Report`-u (`Report.commits`/
`deliveries` više nisu uvijek `[]`), aktivne faze preko `phase_tag` iz profila (dug I3+M14, riješen),
`SnapshotMetrics`/`diff`/`worst_severity`/`alerts_raised` u `snapshot.rs`. Brane na `main`-u nakon
svih spajanja: `cargo fmt --check` OK · `cargo clippy --workspace -- -D warnings` OK · **82 testa** ·
`sokratis signals .` 0. Nalaz recenzije M2/5, **namjerno neriješen** ovim tokom: `metrics/
indicators.rs` i dalje klasificira dio commita odvojeno od `commit_rows` (spec §3.2 „jednom" nije
dovršeno) — otvoreno do završne recenzije M2, `records/BACKLOG.md`. Što je time ušlo u kod:
`records/CHANGELOG.md` (Unreleased) i `architecture/ARCHITECTURE.md` §11; stanje duga:
`records/BACKLOG.md`.

**Ostala tri toka su otišla dalje u svojim granama i sve su njihove dosadašnje cigle recenzirane
SPOJIVO, ali NIŠTA od toga nije spojeno u `main`** (detalji: `records/PROGRESS.md`, dopuna „Nastavak
iste večeri"): IO (`feat/io-m2`) — M2/10–M2/13 (atomarno pisanje, performanse gita — mjerenje i dalje
≥ prag 500 ms → T18 po pravilu iz plana **ulazi** —, detached HEAD, watcher S-016); M2/14 (io-dio
ograde I9, `--until` prema gitu) nije započeta. STORE (`feat/store`) — M2/15–M2/17 (registar,
postavke, snimke uz jedan krug popravka); M2/18 (keš, uvjetna cigla) nije započeta. SUČELJE
(`feat/ui`) — M2/20–M2/24 (tokeni, HR/EN, `format.ts`, SVG grafovi, splash); M2/25–M2/28 (okvir s
`api.ts`, pogledi) nisu započete. Stabla CLI/DESKTOP/INTEGRACIJA otvaraju se kasnije po ovisnostima
iz plana. **Sesija je večeras stala jer je predugo trajala (Leonova odluka), ne jer je M2 gotov** —
nastavak je spajanje gornja tri toka, ne nova analiza; prva radnja nove sesije čita
`.superpowers/sdd/2026-09-18-m2-desktop/progress.md`, odjeljak „STANJE ZA NOVU SESIJU" na dnu.

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ **zatvoren 2026-09-18** (0.1.0, T1–T22 + krug popravaka u `main`; grane i stabla tokova obrisani uz Leonov OK) |
| **M2** | **Desktop** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja (sve 4 teme, `brand-*` iz loga) · `sokratis-store` (SQLite: registar · snimke · keš) · watcher · tray · autostart · obavijesti · svih 8 pogleda s uređivanjem · HR/EN · **znak Sokratisa** = ikona aplikacije + animacija pri pokretanju u kojoj logo stoji na mjestu slova „o" (Leonove datoteke od 2026-09-18, spec §5.2–5.4) · 7 od 9 stavki duga M1 — sve u [ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md) | …spec §11: pokrenuti Sokratis, vidjeti animaciju, Pregled sa Sokrat Studyjem kao jednim projektom od pet stabala, svih 8 pogleda s brojkama istim kao CLI, osvježenje bez klika nakon commita, obavijest na Alert iz traya | 🟨 **u izvedbi, sesija stala 2026-09-18 (predugo trajanje, ne kraj milestonea)** — kostur T1 cijel u `main`-u (`M2/1a`+`M2/1b`), PROFIL (T8–T9) i JEZGRA (T2–T7) gotovi i spojeni; IO/STORE/SUČELJE recenzirani do svoje zadnje cigle (SPOJIVO) u svojim granama, još nespojeni — nastavak: ledger `.superpowers/sdd/2026-09-18-m2-desktop/progress.md` |
| **M3** | **Objava** | ostala pravila · profil za tuđe projekte · HR/EN · instalater · znak · README EN · licenca · GitHub | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Objava i push na `main` = izričit OK** | vrijedi kad remote bude postojao; nijedno odobrenje se ne proteže |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
