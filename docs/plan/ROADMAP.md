# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Spec M1 je ispunjen i
> arhiviran ([archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md)), spec M2 je ispunjen i
> arhiviran ([archive/ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md)); **aktivan spec je
> ARHITEKTURA_1_0** ([ARHITEKTURA_1_0.md](./ARHITEKTURA_1_0.md), drugi rez do 1.0.0); što je izgrađeno
> opisuje [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md), što je isporučeno
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-24)

**Drugi rez do 1.0.0 je odlučen.** Leon je pregledao instaliranu „1.0.0-pre" i tražio veće promjene
(`docs/product/NALAZI_LEON_2026-09-23.md`); brainstorming (pitanje po pitanje) dao je šest odluka
**S-032…S-037** (`records/DECISIONS.md`): metrike nad **svim lokalnim granama** · dnevnik kao **unija
radnih stabala** · klik na karticu → **nadzorna ploča projekta** · grafovi na **d3-matematici + naš
SVG** · **X = upit → izlaz**, tray uklonjen · **sve prije 1.0.0**. Aktivan spec je
[ARHITEKTURA_1_0.md](./ARHITEKTURA_1_0.md); spec M2 je ispunjen i arhiviran
([archive/ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md)) — etape 1 „funkcija" i 2 „izgled" su
cjelovite u kodu (svih devet tokova M2 + SUČELJE-2), a etapa 3 „izdanje" (T35, završna recenzija,
čuvar izdanja) seli na kraj novog plana.

**Aktivan plan je [2026-09-24-1-0-0-grane-i-ploca.md](../superpowers/plans/2026-09-24-1-0-0-grane-i-ploca.md):**
cigle **T44–T63** u šest tokova — DESKTOP-2 (konzola, X) · JEZGRA-2 (model grana, sati po udjelu) ·
IO-2 (`--branches`, stabla, dnevnik po stablima, CLI) · GRAFOVI (d3-temelji + 8 komponenata) · PLOČA
(tok Pregled→Projekt + 8 sekcija) · IZDANJE (preuzima T35/T43 iz plana M2, koji ostaje u
`superpowers/plans/` jer te cigle još vrijede) — pet sesija, instalater „1.0.0-pre.N" nakon svake
(izlazni uvjet §8 speca).

**Izvedbe sesija 1–3 su gotove (2026-09-24):** DESKTOP-2 (T44–T45) · JEZGRA-2 (T46–T48) · GRAFOVI
(T53–T55) · IO-2 (T49–T50) spojeni u `main`, vrh `9a0e98f`; verzija u kodu je bumpana na
`1.0.0-pre.3`, instalater izgrađen (Leon nije stigao instalirati `pre.2`, `pre.3` zamjenjuje obje).
Brojke i sadržaj svake cigle: `records/CHANGELOG.md`; tijek i rulinzi sesija: `records/PROGRESS.md`.
**Sljedeća je sesija 4:** T51 (CLI `--scope`) · T52 (mjerenje procesa/trajanja nad Sokrat Studyjem) ·
T56–T57 (GRAFOVI kraj) · T58 (PLOČA — tok kreće), pa instalater „1.0.0-pre.4".

**Na disku dva stabla:** `sokratis` (`main`, vrh `9a0e98f`) i `sokratis.rel`
(`feat/release`, T35 napola — pet datoteka necommitano, bez izvještaja) — parkirano do toka IZDANJE,
nova sesija ga ne dira ni ne briše. Sva stabla tokova sesija 1–3 (DESKTOP-2, JEZGRA-2, GRAFOVI, IO-2)
su obrisana nakon provjere da su potpuno spojena u pushani `main`.

**M0 gotov. M1 je zatvoren** (0.1.0, T1–T22, paritet s `RAD.xlsx` je test); repozitorij je od
2026-09-20 javan na GitHubu (`records/DECISIONS.md` S-023). **Spec M2**
([archive/ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md)) je iz brainstorminga 2026-09-18
(S-012…S-022) i rez za 1.0.0 od 2026-09-21 (S-024…S-031); svih devet tokova (KOSTUR · JEZGRA · PROFIL
· STORE · IO · CLI · SUČELJE · DESKTOP · INTEGRACIJA) + SUČELJE-2 (T38–T42: Postavke, animacije,
kartica s objašnjenjem) su spojena u `main`; drugi rez do 1.0.0 (S-032…S-037) dodao je sesije 1–3
(DESKTOP-2, JEZGRA-2, GRAFOVI, IO-2) — mjerenje po zadanom profilu je sad preko **svih lokalnih
grana** (S-032), dnevnik unija svih radnih stabala (S-033), verzija u kodu je sad `1.0.0-pre.3`. Puna
povijest svake spojene sesije (merge-hashevi, brojke testova, nalazi recenzije) je u
`records/PROGRESS.md`; što je izgrađeno danas opisuje `architecture/ARCHITECTURE.md` §11.

Druga verzija (M3) i kasnije: [records/BACKLOG.md](../records/BACKLOG.md). Zapis namjere iz kojeg je
prvi rez nastao: [archive/PLAN_DESIGNE.md](../archive/PLAN_DESIGNE.md) (ispunjen, nije izvor istine).

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ **zatvoren 2026-09-18** (0.1.0, T1–T22 + krug popravaka u `main`; grane i stabla tokova obrisani uz Leonov OK) |
| **M2** | **Desktop — verzija 1.0.0** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja (4 teme, `brand-*` iz loga) · `sokratis-store` (SQLite: registar · snimke · keš) · watcher · autostart · obavijesti · 8 pogleda s uređivanjem + Postavke · HR/EN · znak i animacija pokretanja · **dopuna rezom 2026-09-21:** animacije grafova i pogleda · kartica s objašnjenjem · instalater za Leona · repo bez konvencija · dokumentacija točna i manja — sve u [ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md) (§11 + §13, ✅ ispunjen, arhiviran; **tray iz tog speca uklonjen drugim rezom, S-036**) | …instalirati Sokratis, vidjeti animaciju, Pregled sa Sokrat Studyjem (pet stabala = jedan projekt) i Sokratisom, sve poglede s brojkama istim kao CLI, osvježenje bez klika, obavijest OS-a na Alert, objašnjenje svake brojke na klik — i prestati otvarati `RAD.xlsx` | 🟨 **u izvedbi (drugi rez)** — etape 1 „funkcija" i 2 „izgled" gotove u kodu: u `main`-u svih devet tokova M2 + SUČELJE-2 (T38–T42). Leonovi nalazi nad instaliranom 1.0.0-pre pokrenuli drugi rez (S-032…S-037, [ARHITEKTURA_1_0.md](./ARHITEKTURA_1_0.md)): stara etapa 3 „izdanje" (T35, T43) seli na kraj novog plana [T44–T63](../superpowers/plans/2026-09-24-1-0-0-grane-i-ploca.md), pet sesija. **Sesija 1 gotova** (T44–T45, T46–T47, T53 u `main`-u, verzija `1.0.0-pre.2`); sesija 2 u tijeku (T48–T50, T54); **1.0.0 izlazi tek nakon T44–T63** |
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
