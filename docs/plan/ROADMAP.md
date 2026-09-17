# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Aktivni spec je
> [ARHITEKTURA_M1.md](./ARHITEKTURA_M1.md); što je isporučeno zna
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-17, večer)

**M0 gotovo.** Plan M1 skoro gotov: u `main`-u su KOSTUR (T1), FIXTURE (T2, T2b, T2c), PARSE (T3–T6),
METRIKE (T7–T11), DOCS+PRAVILA (T12–T14), IO (T15–T17), CLI (T18–T19) i INTEGRACIJA (T20–T21);
`cargo test --workspace` = 47 passed. `sokratis report/docs/signals` rade nad pravim repozitorijem
i daju paritet s `RAD.xlsx` (test `tests/parity.rs`). Dogfooding nad Sokrat Studyjem potvrđen:
14 dana, 190 commita, 105 isporuka, 85 h, docs 100/100, signal `unmerged-branches` ALERT. Detalji i
brojke: `records/CHANGELOG.md`, tijek sesije: `records/PROGRESS.md`. **Preostaje samo T22**
(dogfooding profila, `ARCHITECTURE.md`, spec u `archive/`, `CHANGELOG.md` 0.1.0) → završna
recenzija cijelog M1 → **zastanak na kraju M1** (Leonov OK).

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | 🔨 pred zastankom (T1–T21 u `main`; T22 preostaje) |
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
