# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Ispunjeni specovi i planovi
> cigli su u `archive/` ([ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md) ·
> [ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md) · [ARHITEKTURA_1_0.md](../archive/ARHITEKTURA_1_0.md));
> što je izgrađeno opisuje [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md), što je isporučeno
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-29)

**M2 je izgrađen kao verzija 1.0.0** (S-024): oba reza — prvi (T1–T43, spec M2) i drugi nakon Leonovih
nalaza nad instaliranom „1.0.0-pre" (T44–T64, S-032…S-038) — te krug popravaka završne recenzije
(`M2/63`) su u `main`-u, a verzija u kodu je `1.0.0`. **Izdano 2026-09-29:** tag `v1.0.0`, izdanje na
GitHubu s nepotpisanim instalaterom (S-040), licenca S-039. Nema aktivnog speca; **M3 nema spec** — piše se kad 1.0.0 bude u
uporabi, iz kandidata u [records/BACKLOG.md](../records/BACKLOG.md). Zapis namjere iz kojeg je prvi rez
nastao: [archive/PLAN_DESIGNE.md](../archive/PLAN_DESIGNE.md).

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ **zatvoren 2026-09-18** (0.1.0, T1–T22 + krug popravaka u `main`; grane i stabla tokova obrisani uz Leonov OK) |
| **M2** | **Desktop — verzija 1.0.0** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja (4 teme, `brand-*` iz loga) · `sokratis-store` (SQLite: registar · snimke · keš) · watcher · autostart · obavijesti · 8 pogleda s uređivanjem + Postavke · HR/EN · znak i animacija pokretanja · **dopuna rezom 2026-09-21:** animacije grafova i pogleda · kartica s objašnjenjem · instalater za Leona · repo bez konvencija · dokumentacija točna i manja — sve u [ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md) (§11 + §13, ✅ ispunjen, arhiviran); **drugi rez:** mjerenje svih lokalnih grana, pogledi → ploča Projekt s 8 sekcija i grafovima (S-034), tray uklonjen (S-036) | …instalirati Sokratis, vidjeti animaciju, Pregled sa Sokrat Studyjem (pet stabala = jedan projekt) i Sokratisom, sve poglede s brojkama istim kao CLI, osvježenje bez klika, obavijest OS-a na Alert, objašnjenje svake brojke na klik — i prestati otvarati `RAD.xlsx` | ✅ **kod gotov 2026-09-28 kao 1.0.0** (T1–T64 + krug popravaka `M2/63`; drugi rez S-032…S-038 u [ARHITEKTURA_1_0.md](../archive/ARHITEKTURA_1_0.md), ✅ ispunjen) — tag i izdanje čekaju Leonov OK |
| **M3** | **Druga verzija** | iz stvarne uporabe 1.0.0: ocjena projekta F−…A+ · omjer popravaka i nova statistika · Postavke s vlastitim dodacima · kartica s dokazom · punjenje trenda iz povijesti · izvoz · izgled u iOS stilu — popis u [BACKLOG.md](../records/BACKLOG.md), spec se piše kad 1.0.0 bude u uporabi | …(određuje spec M3) | 📋 planirano |
| **M4** | **Objava** | ostala pravila · profil za tuđe projekte · potpisan instalater · GitHub Actions; kasnije zasebnim planom: timovi · ne samo git | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Push `main`-a: trajni OK** (2026-09-20) | orkestrator pusha nakon spajanja kad pune brane prođu; potpuno spojene grane smije brisati (2026-09-21); force-push, tag/izdanje, vidljivost, novi remote i brisanje nespojene grane traže izričit OK — `CLAUDE.md` pravilo #1 |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
