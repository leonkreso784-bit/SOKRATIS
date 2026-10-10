# Zakrpa 1.0.1 — plan cigli

**Status:** 📋 AKTIVAN — napisan 2026-10-10 iz Leonove odluke iste sesije (`PROGRESS.md`, unos
2026-10-10). Spec nema: zakrpa ne mijenja ugovor `Report` (S-012) ni profil, samo popravlja tri stavke
iz [`../records/BACKLOG.md`](../records/BACKLOG.md). Izvodi se agentima po
[`../workflow/AGENTI.md`](../workflow/AGENTI.md); numeracija nastavlja plan 1.0.0 (zadnja cigla T64).
Plan nije izvor istine — što je izgrađeno kaže [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md).

**Cilj:** korisnik instalatera 1.0.1 vidi koju verziju ima, dobiva obavijesti o licencama ugrađenih
komponenti (obveza MIT · Apache-2.0 · BSD · MPL-2.0 licenci pri binarnoj distribuciji), a kartica
Pregleda piše „1 stablo", ne „1 stabala".

## Ograničenja (za sve cigle)

- **Nijedna nova ovisnost** — ni crate, ni npm paket, ni alat (`cargo-about`, `license-checker` …).
  Sve ide postojećim: Node bez paketa, `cargo metadata`, `@tauri-apps/api` (već u `package.json`).
- `Report`, profil, `.sokratis/*.json`, `sokratis-core` i paritet se **ne diraju**.
- Identifikatori i i18n-ključevi su engleski (S-008); svaki novi ključ ima par u `hr.json` i `en.json`
  (`npm run check:i18n`).
- Zaglavlje „zašto ovako" u svakoj novoj datoteci (pravilo #5) — konstrukti koji su SADA u datoteci,
  ne dnevnik cigli (nalaz m12 završne recenzije 1.0.0).
- Brane toka sučelja: `npm run check` u `apps/desktop`. Brane toka s Rustom/izdanjem: uz to i
  `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
- Svako stablo ima **svoj** `target/` — `CARGO_TARGET_DIR` se ne dijeli među stablima.

## Tokovi

| tok | grana | stablo | cigle | vlasništvo datoteka |
|---|---|---|---|---|
| SUČELJE-3 | `feat/zakrpa-ui` | `../sokratis.zakrpa-ui` | T65 → T66 | `apps/desktop/src/**`, `apps/desktop/tests/**` (osim `version.test.ts`) |
| OBAVIJESTI | `feat/notices` | `../sokratis.notices` | T67 | `apps/desktop/scripts/notices*.mjs`, `apps/desktop/tests/notices.test.ts`, `THIRD-PARTY-NOTICES.txt`, `apps/desktop/src-tauri/tauri.conf.json` (samo `bundle.resources`), `apps/desktop/package.json` (samo `scripts`), `apps/desktop/tests/version.test.ts` |
| IZDANJE-2 | `main` | `sokratis` | T68 | verzija, `CHANGELOG.md`, instalater — tek kad su oba toka u `main`-u |

Tokovi se ne dodiruju po datotekama. Jedina dodirna točka je tekst u Postavkama (T66) koji imenuje
datoteku iz T67 (`THIRD-PARTY-NOTICES.txt`) — ime je fiksirano ovim planom.

## T65 — Množina „{n} stabala" (SUČELJE-3)

BACKLOG m4 · `ARCHITECTURE.md` §11 „Sučelje".
- Ključ `overview.worktrees` postaje tri ključa `overview.worktrees.one|few|many` (obrazac
  `signals.count.<sev>.<kategorija>`): HR „{n} stablo" · „{n} stabla" · „{n} stabala"; EN „{n} worktree" ·
  „{n} worktrees" · „{n} worktrees".
- Kategoriju bira postojeći `pluralCategory` (`src/views/helpers.ts`) — nova funkcija u `helpers.ts`
  (npr. `worktreesLabel(dict, n)`) kao `signalSummary`; `Overview.svelte` je zove. Id objašnjenja
  `overview.worktrees` (`lib/explain/ids.ts`) i ključevi `explain.overview.worktrees.*` ostaju.
- Test-prvo u `tests/helpers.test.ts`: 1 → „1 stablo", 2/3/4 → „stabla", 5/11/12/14/0 → „stabala",
  21 → „21 stablo", 22 → „22 stabla"; EN 1 → „1 worktree", 2 → „2 worktrees".

## T66 — „O aplikaciji" u Postavkama (SUČELJE-3)

BACKLOG „Verzija se nigdje ne vidi u aplikaciji".
- `Api` dobiva `appVersion(): Promise<string>`. `TauriApi` zove `getVersion()` iz `@tauri-apps/api/app`
  (Tauri čita verziju iz `Cargo.toml`, jedini izvor — `tests/version.test.ts`); `MockApi` vraća
  `package.json` `version`. Provjeriti da `core:default` u `src-tauri/capabilities/default.json`
  pokriva `core:app:allow-version`; ako ne pokriva, cigla to javlja orkestratoru, ne dodaje dozvolu sama.
- Postavke dobivaju odjeljak **„O aplikaciji"** (ključevi `settings.about.*`): verzija · licenca
  („Besplatno za korištenje, nije open source — Sokratis Freeware License 1.0") · rečenica da su
  obavijesti o licencama ugrađenih komponenti u datoteci `THIRD-PARTY-NOTICES.txt` u mapi instalacije.
  Bez poveznica i otvaranja datoteka (to traži `tauri-plugin-opener`, novu ovisnost).
- Test-prvo: `tests/api.test.ts` (MockApi vraća verziju iz `package.json`) i `tests/tauri-api.test.ts`
  (TauriApi prosljeđuje `getVersion`, obrazac postojećih testova u toj datoteci).

## T67 — `THIRD-PARTY-NOTICES.txt` uz instalater (OBAVIJESTI)

BACKLOG „Obavijesti o licencama ugrađenih komponenti".
- **Mjeri prije nego gradiš:** koliko crateova ulazi u `sokratis-desktop` za
  `x86_64-pc-windows-msvc` po normalnim bridovima (orkestrator je izmjerio ≈ 259 jedinstvenih kroz
  `cargo tree -e normal`), koliko npm paketa nije `dev` u `package-lock.json` (14), i koliko od njih
  **nema** datoteku licence u svom paketu. Brojke idu u izvještaj.
- `apps/desktop/scripts/notices.mjs` (čiste funkcije, bez I/O-a — obrazac `contrast.mjs`) +
  `apps/desktop/scripts/notices-build.mjs` (I/O: `cargo metadata --format-version 1 --filter-platform
  x86_64-pc-windows-msvc`, graf od `sokratis-desktop` po normalnim ovisnostima — ne `dev`, ne `build`;
  `package-lock.json` paketi bez `dev: true` i njihovi `node_modules/<ime>/package.json`).
  Datoteke licence u paketu: `LICENSE*`, `LICENCE*`, `COPYING*`, `NOTICE*`, `UNLICENSE`.
- Izlaz `THIRD-PARTY-NOTICES.txt` u korijenu repoa (UTF-8, LF): uvod (engleski — datoteka putuje uz
  instalater kao README), popis komponenti `ime verzija — SPDX izraz`, pa **tekstovi licenci bez
  duplikata** (isti tekst jednom, s popisom komponenti koje ga nose). Komponenta bez vlastite datoteke
  licence je popisana i upućena na tekst iste SPDX licence iz druge komponente; ako takvog nema,
  skripta pada s popisom (ne izmišlja tekst). Vlastiti crateovi `sokratis-*` se preskaču.
  Izlaz je determinističan (sortirano), da `--check` ima smisla.
- `package.json` skripte: `notices` (piše datoteku) · `notices:check` (pada ako commitana datoteka nije
  ono što bi skripta napisala) — `notices:check` ulazi u `npm run check`.
- `tauri.conf.json` `bundle.resources` stavlja `THIRD-PARTY-NOTICES.txt` i `LICENSE` (kao `LICENSE.txt`)
  u mapu instalacije; `tests/version.test.ts` dobiva tvrdnju da su obje u `bundle.resources`.
  Provjera da putanje stoje: `npm run build` pa `cargo build -p sokratis-desktop` (tauri-build pada na
  resursu koji ne postoji).
- Test-prvo u `tests/notices.test.ts` nad čistim funkcijama: deduplikacija istog teksta · sortiranje ·
  komponenta bez datoteke dobiva uputu na istu SPDX licencu · komponenta bez ičega ruši izgradnju s
  njenim imenom · `sokratis-*` preskočen.

## T68 — Izdanje 1.0.1 (IZDANJE-2, orkestrator)

- Verzija `1.0.1` u `[workspace.package]` (`Cargo.toml`), zrcalo u `apps/desktop/package.json` i
  lockovima; `npm run notices` ponovno (verzije vlastitih crateova se preskaču, ali lock se može
  pomaknuti); pune brane; CHANGELOG `[1.0.1]`.
- Instalater `npm run tauri build`; veličina i sha256 u PROGRESS; provjeriti da instalacija stavlja
  `THIRD-PARTY-NOTICES.txt` i `LICENSE.txt` u mapu instalacije i da Postavke pokazuju `1.0.1`.
- **Tag `v1.0.1` i izdanje na GitHubu traže Leonov izričit OK** (pravilo #1) — sesija staje prije toga.
