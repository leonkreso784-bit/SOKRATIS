# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Spec M1 je ispunjen i
> arhiviran ([archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md)), **spec M2 je aktivan**
> ([ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md)); što je izgrađeno opisuje
> [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md), što je isporučeno
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-20)

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

**Time su svi tokovi osim SUČELJA gotovi i u `main`-u** (KOSTUR · PROFIL · JEZGRA · STORE · IO · CLI) —
ispunjeni su svi preduvjeti za DESKTOP (T29–T33) osim sučelja. **Preostali rad je izvan `main`-a:**
SUČELJE (`feat/ui`) — M2/20–M2/25 recenzirano SPOJIVO, **M2/26** sad recenziran SPOJIVO, **M2/27**
(SVG-grafovi u pogledu) u krugu popravka (vizualni nalaz: prsten/Ring prevelik, bez legende), **M2/28**
slijedi. **Sedam stabala otvoreno** (`git worktree list`): `main` · `sokratis.jezgra` · `.profil` ·
`.io` · `.store` · `.ui` · `.cli`. **Opseg preostatka ove sesije, Leonova odluka:** u
`main` ulazi sve osim DESKTOP-a (T29–T33) i INTEGRACIJE (T34–T35); oni i završna recenzija cijelog M2
su treća sesija. Nastavak: ledger `.superpowers/sdd/2026-09-18-m2-desktop/progress.md`, odjeljak
„STANJE ZA NOVU SESIJU" na dnu.

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ **zatvoren 2026-09-18** (0.1.0, T1–T22 + krug popravaka u `main`; grane i stabla tokova obrisani uz Leonov OK) |
| **M2** | **Desktop** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja (sve 4 teme, `brand-*` iz loga) · `sokratis-store` (SQLite: registar · snimke · keš) · watcher · tray · autostart · obavijesti · svih 8 pogleda s uređivanjem · HR/EN · **znak Sokratisa** = ikona aplikacije + animacija pri pokretanju u kojoj logo stoji na mjestu slova „o" (Leonove datoteke od 2026-09-18, spec §5.2–5.4) · 7 od 9 stavki duga M1 — sve u [ARHITEKTURA_M2.md](./ARHITEKTURA_M2.md) | …spec §11: pokrenuti Sokratis, vidjeti animaciju, Pregled sa Sokrat Studyjem kao jednim projektom od pet stabala, svih 8 pogleda s brojkama istim kao CLI, osvježenje bez klika nakon commita, obavijest na Alert iz traya | 🟨 **u izvedbi, druga sesija (2026-09-20)** — kostur T1 cijel u `main`-u (`M2/1a`+`M2/1b`), PROFIL (T8–T9), JEZGRA (T2–T7), STORE (T15–T18), IO (T10–T14 + M2/14b potrošač keša) i CLI (T19, `report --until`) gotovi i spojeni — **dug I9 zatvoren u cijelosti**, tok IO gotov u cijelosti; svi preduvjeti za DESKTOP osim SUČELJA su ispunjeni. SUČELJE (T20–T26 SPOJIVO, M2/27 u krugu popravka) još nespojeno. Opseg ove sesije (Leonova odluka): sve osim DESKTOP-a i INTEGRACIJE; oni i završna recenzija M2 su treća sesija — nastavak: ledger `.superpowers/sdd/2026-09-18-m2-desktop/progress.md` |
| **M3** | **Objava** | ostala pravila · profil za tuđe projekte · HR/EN · instalater · znak · README EN · licenca · GitHub | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Objava i push na `main` = izričit OK** | vrijedi kad remote bude postojao; nijedno odobrenje se ne proteže |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
