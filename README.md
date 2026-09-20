# Sokratis

**Prati kako radiš, ne samo što si napravio.** Desktop aplikacija koja se priključi na
git-repozitorij, izračuna statistiku rada, ocijeni čistoću dokumentacije i javi smjer projekta
s dokazom. Za više projekata odjednom.

> 🚧 **U gradnji (rujan 2026).** Milestone 1 je zatvoren (2026-09-18) — verzija 0.1.0:
> `core`/`io`/`cli` rade nad pravom git-povijesti (paritet s `RAD.xlsx` je test, ne tvrdnja).
> **Milestone 2 (desktop) je u izvedbi** po odobrenom specu
> [docs/plan/ARHITEKTURA_M2.md](./docs/plan/ARHITEKTURA_M2.md); aplikacije još nema (kostur i jezgrina
> nadogradnja su u `main`-u, ostali tokovi rade u svojim granama). Objava na GitHub i push čekaju
> autorov izričit OK. Ulaz u dokumentaciju:
> [docs/README.md](./docs/README.md) · milestonei: [docs/plan/ROADMAP.md](./docs/plan/ROADMAP.md) ·
> isporučeno: [docs/records/CHANGELOG.md](./docs/records/CHANGELOG.md).

## Zašto postoji

Autor je statistiku rada na projektu [Sokrat Study](https://www.sokratstudy.com) vodio u Excel
tablici koju je Python skripta svaki dan generirala iz gita, dnevnika sesija i plana. Tablica je
radila, ali je imala granice: gubila je grafove pri svakoj dopuni, nije znala za više projekata,
tiho je zastarijevala na glavnoj grani i imala je mjerljiv kvar u satima rada. Sokratis tu logiku
seli u pravu aplikaciju: **sve izvodi iz gita na zahtjev, čuva samo ručne podatke, i umjesto tablice
daje signale s dokazom.**

## Što će raditi

- **Statistika rada:** tempo po danu, vrste rada, kvaliteta i brzina, faze i vizije — sve što je bilo u tablici.
- **Čistoća dokumentacije:** mrtve poveznice, dokumenti izvan indeksa, više od jednog aktivnog plana,
  kašnjenje dnevnika za kodom.
- **Signali smjera:** nespojene grane, kašnjenje docs-a i dalje — svaki s dokazom, nikad gola brojka.
- **Više projekata:** svaki opisuje svoje konvencije u `.sokratis/profile.json`; bez profila vrijede zadane.

## Stack

Rust (`core` · `io` · `store` · `cli`) · SQLite kroz `rusqlite` (registar i snimke, M2) · Tauri 2 ·
Svelte 5 · dizajn-tokeni Sokrat Studyja.

## Pokretanje

Radi iz izvornog koda (0.1.0): `cargo run -p sokratis-cli -- report <putanja-do-repoa>` (i `docs`,
`signals`). Toolchain je pinan u `rust-toolchain.toml` (1.98.1) — rustup ga sam preuzme pri prvom
pokretanju `cargo`, bez ručnog koraka. Izlazni kod `signals` je ugovor prema preflightu (0 nema ·
1 Warn · 2 Alert · 3 greška). Status i preostalo: [ROADMAP](./docs/plan/ROADMAP.md).

## Licenca

Otvorena licenca dolazi s objavom (M3). Do tada sva prava pridržana. Autor: Leon Kreso.
