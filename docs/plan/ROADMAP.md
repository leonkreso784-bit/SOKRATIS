# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Spec M1 je ispunjen i
> arhiviran ([archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md)), **spec M2 je aktivan**
> ([ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md)); što je izgrađeno opisuje
> [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md), što je isporučeno
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-21)

**Rez za 1.0.0 je odlučen (Leonov OK 2026-09-21, S-024…S-031).** Izlaz iz M2 je verzija **1.0.0**,
0.2.0 se preskače. U 1.0.0 ulazi ostatak M2 (T28–T35) i osam novih cigli iz Leonova zapisa namjere —
Postavke · animacije · kartica s objašnjenjem · instalater · repo bez konvencija · dokumentacija;
opseg i „gotovo kad“: [ARHITEKTURA_M2.md §13](./ARHITEKTURA_M2.md). Gradi se u tri etape —
**funkcija → izgled → izdanje**; na kraju prve Leon instalira „1.0.0-pre“ i njome nadgleda ostatak
gradnje. Što je ostalo za drugu verziju i kasnije: [records/BACKLOG.md](../records/BACKLOG.md).
**Sljedeća sesija je izvedba: T28 → spajanje SUČELJA u `main`.**

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
(`.claude/agents/*.md`) — obje odluke Leon je odobrio isti dan. **Repozitorij je od 2026-09-20 javan
na GitHubu** (`origin`, Leonov izričit OK; odluka i što je time postalo javno: `DECISIONS.md` S-023).

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
`SnapshotMetrics`/`diff`/`worst_severity`/`alerts_raised` u `snapshot.rs`. Nalaz recenzije M2/5,
**namjerno neriješen** ovim tokom: `metrics/indicators.rs` i dalje klasificira dio commita odvojeno od
`commit_rows` (spec §3.2 „jednom" nije dovršeno) — otvoreno do završne recenzije M2,
`records/BACKLOG.md`.

**Tok STORE je gotov** (T15–T18, merge `e9b01c7`, druga sesija izvedbe M2, 2026-09-20): registar
projekata i stabala s migracijama (M2/15, identitet = `git_common_dir`, S-015), postavke globalne i po
projektu (M2/16, upsert `ON CONFLICT … DO UPDATE`), snimke brojki s trendom i profilom kao kanonskim
JSON-om (M2/17, S-014 — uz jedan krug popravka: snimka dana je cjelovita zamjena, `DELETE`+`INSERT` u
istoj transakciji), keš sirovih commita po SHA (M2/18 — uvjetna cigla koja je UŠLA jer je mjerenje
M2/11 dalo 1479,71 ms ≥ prag 500 ms). Brane na `main`-u nakon svih spajanja (JEZGRA + PROFIL + STORE):
`cargo fmt --check` OK · `cargo clippy --workspace --all-targets -- -D warnings` OK · **107 testova**
· `sokratis signals .` 0. Što je time ušlo u kod: `records/CHANGELOG.md` (Unreleased) i
`architecture/ARCHITECTURE.md` §11; stanje duga: `records/BACKLOG.md`.

**Tok IO je gotov u cijelosti** (T10–T14 + M2/14b, merge `3ca068c` pa `0740685`, 2026-09-20): pisanje
ručnih podataka u glavno stablo atomarno (M2/10, S-015) · performanse gita — ~94 → 4 git-procesa po
izvještaju, 3318–3822 ms → 1479,71 ms nad Sokrat Studyjem (M2/11, dug M11; krug popravka:
`--diff-merges=combined` i `-c core.quotepath=false`, 0/56 neslaganja) · detached HEAD daje `HEAD@<sha>`
umjesto lažnog „nema commita" (M2/12) · watcher nad `.git`/docs/`.sokratis` s odgodom 600 ms (M2/13,
S-016) · birač raspona (`GitSource::log` dobiva `until`) i io-dio ograde putanja pri `Project::open`
(M2/14) — **dug I9 je time zatvoren u cijelosti** (jezgreni dio M2/8 + io-dio M2/14) · **M2/14b —
potrošač keša (cigla izvan plana od 35, orkestratorova, povod mjerenje M2/11 ≥ prag 500 ms):**
`GitSource::rev_list` kaže što je dostižno, keš (`CommitCache`, trait u `io`) vraća poznate commite,
`git log --no-walk --stdin` dovlači SAMO nedostajuće; `format_gitlog` u jezgri je inverz parsera; izmjereno
nad Sokrat Studyjem 256–472 ms topao (cilj < 500 ms postignut); `io` i dalje ne ovisi o `store`
(S-013), adapter dolazi u desktopu (T29). Brane na `main`-u nakon svih spajanja (IO + STORE + JEZGRA +
PROFIL): `cargo fmt --check` OK · `cargo clippy --workspace --all-targets -- -D warnings` OK ·
**136 testova, 1 ignoriran** (mjerni test) · `sokratis signals .` 0. `main` pushan na `origin`.

**Tok CLI je gotov** (T19, merge `11b1708`, 2026-09-20, nastavak iste sesije): `sokratis report`
dobiva `--until YYYY-MM-DD` (M2/19) — zrcali `--since`, poziva `Project::input_between`; `docs`/
`signals` i dalje ne primaju `until`. Neispravan `--until` → kod 3 s porukom iz jezgre; `until <
since` → prazan izvještaj, kod 0. Poznato ograničenje: `--table` u zaglavlju ne pokazuje `until`
(JSON je ugovor i on je točan; odgođeno u `records/BACKLOG.md`).

**Time su svi tokovi osim SUČELJA gotovi i u `main`-u** (KOSTUR · PROFIL · JEZGRA · STORE · IO · CLI).
**SUČELJE (`feat/ui`, stablo `sokratis.ui`) je jedini nespojeni tok:** M2/20–M2/27 su vizualno
potvrđene u pregledniku, **M2/28 (Dnevnik · Isporuke · Vizije · Dokumentacija) nije započeta**. Grane i
stabla pet spojenih tokova obrisani su 2026-09-21 (Leonovo dopuštenje za potpuno spojene grane);
na disku su `main` i `sokratis.ui`. Tijek sesija: `records/PROGRESS.md`; nastavak gradnje: ledger
`.superpowers/sdd/2026-09-18-m2-desktop/progress.md` (izvan gita) i `NOVA-SESIJA-PROMPT.md` pored njega.

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ **zatvoren 2026-09-18** (0.1.0, T1–T22 + krug popravaka u `main`; grane i stabla tokova obrisani uz Leonov OK) |
| **M2** | **Desktop — verzija 1.0.0** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja (4 teme, `brand-*` iz loga) · `sokratis-store` (SQLite: registar · snimke · keš) · watcher · tray · autostart · obavijesti · 8 pogleda s uređivanjem + Postavke · HR/EN · znak i animacija pokretanja · **dopuna rezom 2026-09-21:** animacije grafova i pogleda · kartica s objašnjenjem · instalater za Leona · repo bez konvencija · dokumentacija točna i manja — sve u [ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md) (§11 + §13) | …instalirati Sokratis, vidjeti animaciju, Pregled sa Sokrat Studyjem (pet stabala = jedan projekt) i Sokratisom, sve poglede s brojkama istim kao CLI, osvježenje bez klika, obavijest na Alert iz traya, objašnjenje svake brojke na klik — i prestati otvarati `RAD.xlsx` | 🟨 **u izvedbi** — u `main`-u: KOSTUR · PROFIL · JEZGRA · STORE · IO (+ M2/14b) · CLI; nespojeno: SUČELJE (M2/20–27 potvrđeno, M2/28 nije započeta); nije započeto: DESKTOP · INTEGRACIJA · osam cigli iz §13. Etape: funkcija → izgled → izdanje (S-025) |
| **M3** | **Druga verzija** | iz stvarne uporabe 1.0.0: ocjena projekta F−…A+ · omjer popravaka i nova statistika · Postavke s vlastitim dodacima · kartica s dokazom · punjenje trenda iz povijesti · izvoz · izgled u iOS stilu — popis u [BACKLOG.md](../records/BACKLOG.md), spec se piše kad 1.0.0 bude u uporabi | …(određuje spec M3) | 📋 planirano |
| **M4** | **Objava** | ostala pravila · profil za tuđe projekte · potpisan instalater · licenca · GitHub Actions; kasnije zasebnim planom: timovi · ne samo git | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Push `main`-a: trajni OK** (2026-09-20) | orkestrator pusha nakon spajanja kad pune brane prođu; potpuno spojene grane smije brisati (2026-09-21); force-push, tag/izdanje, vidljivost, novi remote i brisanje nespojene grane traže izričit OK — `CLAUDE.md` pravilo #1 |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
