# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Aktivni spec je
> [ARHITEKTURA_M1.md](./ARHITEKTURA_M1.md); što je isporučeno zna
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-17)

**M0 gotovo.** Plan M1 u izvedbi: u `main`-u su KOSTUR (T1), FIXTURE (T2, T2b), PARSE (T3–T6),
DOCS+PRAVILA (T12–T14) i IO (T15–T17); `cargo test --workspace` = 30 passed. METRIKE (T7–T10) su gotove
na grani `feat/core-metrics`, T11 slijedi; CLI (T18–T19) čeka slobodno mjesto. Usput izmjeren i
ispravljen drugi kvar tablice: `git log --since` bez sata (S-011) — fixture T2b je već regeneriran s
punim danom. Detalji i brojke: `records/CHANGELOG.md`, tijek sesije: `records/PROGRESS.md`. Zatim:
METRIKE → CLI → spajanje → integracija (T20–T22) → zastanak na kraju M1.

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | 🔨 u tijeku (T1–T6, T12–T17 u `main`; T7–T11 na grani; T18–T22 preostaju) |
| **M2** | **Desktop** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja · SQLite snimke · watcher · tray · obavijesti | …otvoriti Sokratis iz traya, vidjeti sve projekte i dobiti obavijest kad signal padne na Alert | 📋 planirano |
| **M3** | **Objava** | ostala pravila · profil za tuđe projekte · HR/EN · instalater · znak · README EN · licenca · GitHub | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Objava i push na `main` = izričit OK** | vrijedi kad remote bude postojao; nijedno odobrenje se ne proteže |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
