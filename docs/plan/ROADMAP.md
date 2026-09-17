# ROADMAP — milestonei i status

> **Što ovo jest:** redoslijed milestonea i gdje smo. **Što nije:** spec. Spec M1 je ispunjen i
> arhiviran ([archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md)), spec M2 tek dolazi; što je
> izgrađeno opisuje [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md), što je isporučeno
> [records/CHANGELOG.md](../records/CHANGELOG.md), a tijek sesija [records/PROGRESS.md](../records/PROGRESS.md).

## Gdje smo (2026-09-17, večer)

**M0 gotov. Kod M1 je isporučen, recenziran i popravljen — verzija 0.1.0.** Sve cigle T1–T22 su u
`main`-u: KOSTUR (T1), FIXTURE (T2, T2b, T2c), PARSE (T3–T6), METRIKE (T7–T11), DOCS+PRAVILA
(T12–T14), IO (T15–T17), CLI (T18–T19), INTEGRACIJA (T20–T22), a nad njima je prošla **završna
recenzija cijelog lanca** s jednim krugom popravaka (spojen u `main`; ponovna recenzija SPOJIVO).
`sokratis report/docs/signals` rade nad pravim repozitorijem, paritet s `RAD.xlsx` je test
(`crates/sokratis-core/tests/parity.rs`), a Sokratis mjeri i sam sebe (vlastiti
`.sokratis/profile.json`). Brojke, popravci i broj testova: `records/CHANGELOG.md` (0.1.0); tijek
sesije: `records/PROGRESS.md`; što je izgrađeno i što još ne radi:
`architecture/ARCHITECTURE.md` (§11).

**Milestone 1 se zatvara tek Leonovim OK-om.** Na njega čekaju: brisanje 8 grana tokova i njihovih
radnih stabala, push i objava (remote još ne postoji), odluka o `rust-toolchain.toml`. Nakon toga se
piše spec za M2. Nalazi recenzije koji nisu popravljeni u M1 stoje u `records/BACKLOG.md`, nijedan
nije skriven.

## Milestonei

| # | naziv | sadržaj | gotovo kad Leon može… | status |
|---|---|---|---|---|
| **M0** | **Toolchain** | Visual Studio Build Tools (workload „Desktop development with C++") · rustup s MSVC targetom · `cargo --version` · workspace koji se builda | …pokrenuti `cargo test` u ovom folderu i dobiti zeleno na praznom testu | ✅ gotovo (2026-09-17) |
| **M1** | **Jezgra + CLI** | `core` · `io` (git-proces, profil, ručni podaci) · `cli` · paritet s `RAD.xlsx` · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | …nad Sokrat Studyjem iz terminala dobiti iste brojke kao u tablici, ispravne sate, ocjenu docs-a i dva signala s dokazom; staviti `sokratis signals` u preflight | ✅ kod isporučen i recenziran 2026-09-17 (0.1.0, T1–T22 + krug popravaka u `main`); **zatvaranje čeka Leonov OK** |
| **M2** | **Desktop** | Tauri 2 · Svelte 5 · tokeni Sokrat Studyja · SQLite snimke · watcher · tray · obavijesti · **znak Sokratisa u sučelju** (Leon, 2026-09-17: logo `C:\Users\leonk\Downloads\sokratis logo .png` **umjesto teksta „Sokratis"** na mjestu naziva aplikacije + splash s animacijom logotipa pri učitavanju; znak je nov, Sokratov se ne dira) | …otvoriti Sokratis iz traya, vidjeti sve projekte i dobiti obavijest kad signal padne na Alert | 📋 **sljedeći** — spec se piše nakon zastanka |
| **M3** | **Objava** | ostala pravila · profil za tuđe projekte · HR/EN · instalater · znak · README EN · licenca · GitHub | …instalirati Sokratis s GitHuba na čist stroj i priključiti tuđi repo | 📋 planirano |

## Pravila vožnje (Leonova, ne mijenjaju se između milestonea)

| pravilo | u praksi |
|---|---|
| **Jedna cigla = jedan commit** | zahvat koji dira više od predmeta cigle se reže u seriju |
| **Zastanak na kraju milestonea** | unutar milestonea cigla za ciglom, brana na svakoj; na kraju **stani i javi se** |
| **Objava i push na `main` = izričit OK** | vrijedi kad remote bude postojao; nijedno odobrenje se ne proteže |
| **Mjeri prije nego popravljaš** | fixture i test prije implementacije; paritet je test |
| **„Zašto Rust ovako"** | svaka cigla uči jedan konstrukt; pojmovnik u `workflow/RUST.md` |
