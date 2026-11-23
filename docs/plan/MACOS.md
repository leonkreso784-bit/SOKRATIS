# Sokratis na macOS-u — plan cigli

**Status:** 📋 AKTIVAN — Leon je odlučio O1–O7 (S-043). Napisan 2026-11-23 nakon pregleda cijelog projekta na
MacBooku; Faza 1 (T74–T76) i CI (T85) su izvedeni, zakrpa 1.0.2 je izdana (O5), pa slijede Faze 2–5 i
izdanje 1.1.0. Numeracija nastavlja plan 1.0.2 ([`../archive/2026-11-23-zakrpa-1-0-2.md`](../archive/2026-11-23-zakrpa-1-0-2.md),
zadnja cigla T73). Plan nije izvor istine: što je izgrađeno kaže
[`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md).

**Cilj:** Sokratis se razvija, testira, gradi i koristi na Leonovu MacBooku (Apple Silicon, macOS 26),
a da Windows izdanje i dalje radi.

---

## 0 · Što je izmjereno (2026-11-23, MacBook arm64, macOS 26.6)

> **Faza 1 izvedena 2026-11-23 — brane su zelene na Macu** (N1–N3 riješeni u T74–T76). Tablica i nalazi
> ispod opisuju stanje PRIJE Faze 1; brojke nakon nje: [`../records/PROGRESS.md`](../records/PROGRESS.md) 2026-11-23.

**Okruženje je spremno:** Xcode Command Line Tools · rustup s pinanim 1.98.1 `aarch64-apple-darwin`
· Node 24 · git · sva radna stabla Sokrat Studyja u `~/Documents/sokratstudy.*` (`sokratstudy.dev` nije
`core.bare`).

| brana | rezultat na Macu |
|---|---|
| `cargo fmt --check` | ✅ |
| `cargo clippy --all-targets -- -D warnings` | ✅ (cijeli workspace, i `sokratis-desktop`) |
| `cargo test` | ❌ **4 crvena** (dolje) · ostalo zeleno, paritet s `RAD.xlsx` zelen |
| `npm ci` + `check:svelte` · `check:i18n` · `check:contrast` · `test` (162) | ✅ |
| `npm run notices:check` | ❌ (dolje) |
| CLI nad `~/Documents/sokratstudy.f6` (`report`, `signals`) | ✅ radi |

**Posljedica:** pravilo #2 („crveno ne ide u commit") znači da se **s ovog Maca danas ne može commitati
ništa**, pa ni preostale cigle zakrpe 1.0.2 (T71–T73). Faza 1 zato ide prva.

### Nalazi: kvarovi koji su izmjereni

| # | gdje | što | vrsta |
|---|---|---|---|
| N1 | `crates/sokratis-io/tests/project.rs:520`, `tests/write.rs:60` | `tempdir()` daje `/var/folders/…`, git vraća stvarnu putanju `/private/var/folders/…` (`/var` je na macOS-u simbolička poveznica). Testovi uspoređuju bez `canonicalize` (`tests/git_cli.rs:110` to već radi ispravno) | kvar testa, ne koda: identitet projekta (`common_dir`) uvijek dolazi iz gita |
| N2 | `crates/sokratis-core/src/profile.rs:276` `inside_root` | na Unixu `\\server\share` i `C:\Users` nisu apsolutni, pa prolaze kao „unutar repoa". `profile.json` je commitan i dijeli se među sustavima, pa isti profil mora dobiti **isti odgovor na svakom OS-u** | stvarni kvar (2 testa crvena: `profile.rs:449`, `:486`) |
| N3 | `apps/desktop/scripts/notices-build.mjs:26` | (a) `--offline` pada dok crateovi samo za Windows nisu preuzeti, a to rješava jednokratni `cargo fetch`. (b) **Nakon toga** pada `base64@0.21.7 je u cargo tree, a nema ga u cargo metadata`: `cargo tree --target windows` na Mac domaćinu uključuje ovisnosti proc-makroa za **domaćina** (macOS), a `metadata --filter-platform windows` ih izbaci. Skripta pretpostavlja da je domaćin Windows | stvarni kvar skripte |

### Nalazi: kvarovi koji se vide u kodu, a u aplikaciji još nisu provjereni

| # | gdje | što |
|---|---|---|
| N4 | `apps/desktop/src-tauri/src/state.rs:39` `db_path` | `LOCALAPPDATA` na Macu ne postoji, pa baza ide u `std::env::temp_dir()` (`$TMPDIR`), **koju macOS sam čisti**: registar projekata i snimke trenda bi nestajali. Mac mjesto: `~/Library/Application Support/sokratis/`. I `expect("baza u %LOCALAPPDATA%")` u `lib.rs:56` |
| N5 | `apps/desktop/src/views/helpers.ts:165` `joinRepoPath` | svaki `/` pretvara u `\`, pa „Kopiraj putanju" na Macu daje `/Users/leon\docs\records\…`, putanju koja ne postoji. Test `tests/helpers.test.ts:242` tvrdi samo Windows oblik |
| N6 | `apps/desktop/src/views/Overview.svelte:38`, `:49` | `prompt()` (preimenovanje) i `confirm()` (uklanjanje projekta) su ugrađeni dijalozi WebView2-a. Na macOS-u Tauri radi preko WKWebViewa, gdje ti dijalozi ovise o tome što wry implementira, pa `prompt()` može tiho vratiti `null`. **Prvo izmjeriti** (Faza 4), pa tek onda mijenjati |
| N7 | `src/lib/i18n/hr.json:333`, `en.json:333` | „Sokratis se otvara pri prijavi u **Windows**." |
| N8 | `src-tauri/tauri.conf.json:42` | `bundle.targets = ["nsis"]`: na Macu se ne gradi ni `.app` ni `.dmg`. Test `tests/version.test.ts:19` to i tvrdi |
| N9 | `tauri.conf.json:4` `identifier: "dev.sokratis.app"` | Tauri na macOS-u upozorava na identifikator koji završava s `.app`, jer se sudara s nastavkom paketa aplikacije. O promjeni odlučuje O2 |
| N10 | `crates/sokratis-io/src/git.rs` (`git_command`) | aplikacija pokrenuta iz Findera ima `PATH=/usr/bin:/bin:/usr/sbin:/sbin`: Homebrew `git` ne vidi, nego koristi `/usr/bin/git`, koji radi samo uz Command Line Tools (bez njih macOS iskoči s dijalogom za instalaciju). Leon ih ima, a tuđi Mac možda nema |
| N11 | ponašanje WebKita ≠ Chromium | `format.ts` (znak minus, ICU), hrvatsko sortiranje `C < Č < D` (`helpers.ts:29`) i `navigator.clipboard` (`helpers.ts:150`) izmjereni su na WebView2 i u Nodeu, a ne u WebKitu. Vitest je zelen, ali ne vrti WebKit |
| N12 | macOS prečaci | `Cmd+Q` gasi aplikaciju kroz izbornik **bez** upita iz S-036 (`CloseRequested` stiže samo za X i `Cmd+W`). To je uobičajeno ponašanje na Macu, ali razlika mora biti odluka, a ne slučaj |

**Ono što već radi za Mac i ne treba dirati:** `#[cfg(windows)]` oko `CREATE_NO_WINDOW` (`git.rs:87`) se na
Macu ne prevodi · `windows_subsystem` (`main.rs:5`) je na Macu bez učinka · autostart je već
`MacosLauncher::LaunchAgent` (`lib.rs:48`) · `icons/icon.icns` postoji · font `-apple-system` je prvi u
`tokens.css` · `.gitattributes` drži LF · upute agenata u `.claude/agents/` nemaju Windows putanja.

---

## 1 · Odluke (Leon, 2026-11-23 — [S-043](../records/DECISIONS.md))

Odlučeno je svih sedam pitanja i N12, sve po preporuci: O1 uz Windows · O2 identifikator samo na Macu · O3 prvo samo Leonov Mac · O4 samo Apple Silicon · O5 prvo 1.0.2, pa Mac, pa 1.1.0 · O6 zasebni popis licenci po platformi · O7 CI **sada** (ne u M4) · N12 `Cmd+Q` zatvara odmah. Tablica niže je podloga odluke.

| # | pitanje | preporuka | zašto |
|---|---|---|---|
| **O1** | Mac **uz** Windows, ili **samo** Mac? | **uz Windows** | izdanja 1.0.x su Windows; `cfg` grane već postoje; samo-Mac znači micati NSIS, `CREATE_NO_WINDOW` i dio dokumentacije |
| **O2** | Identifikator `dev.sokratis.app` → npr. `dev.sokratis.desktop`? | promijeniti **samo za Mac** (`tauri.macos.conf.json`), Windows ne dirati | nadogradnja NSIS-om na Windowsu ovisi o identifikatoru; baza ne ovisi (N4 je vlastita putanja) |
| **O3** | Za koga je Mac izdanje? | **prvo samo Leonov Mac** (lokalni build, bez potpisa); javni `.dmg` u M4 | lokalni build nema karantenu pa Gatekeeper ne smeta. Javni nepotpisan `.dmg` od macOS-a 15 traži „Postavke → Privatnost i sigurnost → Ipak otvori"; potpis + notarizacija traže Apple Developer Program (99 $ godišnje) — isti razgovor kao potpis za Windows (BACKLOG „Potpisan instalater") |
| **O4** | Arhitektura | **samo `aarch64`** | Leonov Mac je arm64; univerzalni binarni zapis (`+x86_64`) tek ako netko s Intel Macom zatraži |
| **O5** | Redoslijed i verzija | Faza 1 → dovršiti 1.0.2 (T71–T73) s Maca → Faze 2–5 → **1.1.0** | nova platforma je nova mogućnost (minor); Faza 1 je preduvjet za bilo koji commit |
| **O6** | `THIRD-PARTY-NOTICES.txt` po platformi | **jedna datoteka po cilju** (`…-windows.txt`, `…-macos.txt`), ili popravak da izlaz ne ovisi o domaćinu pa jedna Windows datoteka + jedna Mac | Mac binarni zapis sadrži druge crateove (`objc2`, `cocoa`…) nego Windows |
| **O7** | CI na GitHubu (macOS + Windows brane) | **da, u M4**; do tada jedan ručni Windows build prije svakog izdanja | Leon na Macu više ne vidi Windows regresije (`CREATE_NO_WINDOW`, NSIS, `\`); CI je jedina zaštita. Nov workflow u javnom repou je vanjska radnja, pa traži OK |

---

## 2 · Cigle

Pravila vrijede kao i inače: jedna cigla = jedan commit, test prvo, zaglavlje „zašto ovako" i brane prije
svakog commita (`CLAUDE.md`). Tokovi i stabla idu po [`../workflow/AGENTI.md`](../workflow/AGENTI.md).

### Faza 1 — brane zelene na Macu (blokira sve ostalo)

| cigla | predmet | test prvo | gotovo kad |
|---|---|---|---|
| **T74** ✅ `a33359c` | io testovi: očekivane putanje kroz `canonicalize` (N1), isti obrazac kao `git_cli.rs:110`. Kod se ne dira | testovi su već crveni na Macu | `cargo test -p sokratis-io` zelen na Macu |
| **T75** ✅ `a33359c` | `inside_root` jednako na svakom OS-u (N2): uz `is_absolute` izričito odbiti `\` na početku, `\\` i prefiks pogona `X:`. Čisto parsiranje teksta, `core` ostaje bez I/O-a (S-002) | `profile.rs:436` i `:474` već postoje i crveni su | `cargo test -p sokratis-core` zelen na Macu i na Windowsu |
| **T76** ✅ `b814d9e` | `notices-build.mjs` ne ovisi o domaćinu (N3b): cilj kao argument (`--target`) i dosljedan skup crateova bez ovisnosti proc-makroa za domaćina. Oblik ovisi o O6. U `TESTING.md` ide jednokratni `cargo fetch` (N3a) | `tests/notices.test.ts`: ulaz iz `cargo tree` s host-ovisnošću → ne ulazi | `npm run check` zelen na Macu; na Windowsu izlaz bajt-za-bajt isti kao danas |

➜ **Nakon T76** (✅ 2026-11-23): pune brane zelene na Macu → nastavak zakrpe 1.0.2 (T71–T73) s ovog stroja. T76 cilj nije dobio kao argument: cilj ostaje Windows, skripta je samo prestala ovisiti o domaćinu, pa O6 ostaje otvoren.

### Faza 2 — aplikacija ispravna na macOS-u (kod)

| cigla | predmet | test prvo | gotovo kad |
|---|---|---|---|
| **T77** | `db_path` po sustavu (N4): čista funkcija `data_dir(os, env)`. Na Windowsu `%LOCALAPPDATA%\sokratis` (bez promjene), na macOS-u `$HOME/Library/Application Support/sokratis`. Rezerva u privremenu mapu **samo** ako nema ni jednog ni drugog, uz `eprintln!`. Poruka u `lib.rs:56` postaje neutralna. **Bez nove ovisnosti** (bez `dirs`, bez Taurijeva `app_data_dir`, koji ovisi o identifikatoru, O2) | `state.rs` tests: tablica (os, env) → putanja | `npm run tauri dev` na Macu piše `~/Library/Application Support/sokratis/sokratis-dev.db` |
| **T78** | `joinRepoPath` čuva razdjelnik korijena (N5): `\` ako korijen ima `\` ili prefiks pogona, inače `/`. Bez `plugin-os` (nova ovisnost nije potrebna) | `helpers.test.ts`: `/Users/leon/x` + `docs/a.md` → `/Users/leon/x/docs/a.md`; postojeća dva Windows testa ostaju | vitest zelen |
| **T79** | natpis autostarta bez imena OS-a (N7): „…pri prijavi u sustav" / „…when you sign in" | `check:i18n` | zelen |
| **T80** | *(uvjetno, ako Faza 4 izmjeri kvar)* `prompt`/`confirm` u `Overview.svelte` (N6) zamijeniti dijalogom u sučelju, istim obrascem kao dvokoračni gumb u `Visions.svelte` | vitest za čistu logiku dijaloga | preimenovanje i uklanjanje rade na Macu |
| **T81** | *(uvjetno)* poruka kad `git` ne postoji ili traži Command Line Tools (N10): greška u sučelju kaže `xcode-select --install` | `io` test s nepostojećim programom | Leon vidi razumljivu poruku umjesto sirove greške procesa |

### Faza 3 — pakiranje za Mac

| cigla | predmet | test prvo | gotovo kad |
|---|---|---|---|
| **T82** | `src-tauri/tauri.macos.conf.json` (Tauri 2 ga spaja s glavnim samo na macOS-u): `bundle.targets = ["app", "dmg"]`, `minimumSystemVersion`, identifikator po O2. Glavni `tauri.conf.json` i NSIS se **ne diraju** (N8, N9) | `version.test.ts`: glavni = `["nsis"]`, Mac datoteka = `["app","dmg"]`, nijedna nema `version` | `npm run tauri build` na Macu daje `.app` i `.dmg` u `target/release/bundle/` |
| **T87** | `THIRD-PARTY-NOTICES` po platformi (O6): `notices-build.mjs` dobiva cilj kao argument; Windows datoteka ostaje bajt-za-bajt ista, Mac popis ide u zasebnu datoteku koju nosi `.app` (`tauri.macos.conf.json` → `bundle.resources`); `notices:check` provjerava obje | `notices.test.ts` | `npm run check` zelen; `.app` nosi Mac popis |
| **T83** | izmjeriti build: veličina `.dmg`/`.app`, ad-hoc potpis (`codesign -dv`), vrijeme do prvog prozora, pokretanje iz `/Applications` (Finder, ne terminal: tako se vidi N10). Brojke idu u `PROGRESS.md`, ne u kod | — | brojke zapisane |

### Faza 4 — ručna provjera u pravoj aplikaciji (Leon + Claude, `.app` iz T82)

Svaki nalaz dobiva test i vlastitu ciglu (pravilo #4). Popis:

1. **Splash** (`decorations: false`, `alwaysOnTop`): animacija, zaobljeni rubovi, zatvaranje; glavni prozor se pojavi.
2. **Jedna instanca:** drugi dvoklik na ikonu podigne postojeći prozor (single-instance plugin, macOS put).
3. **Dodavanje projekta:** dijalog za mapu (`dialog:allow-open`) nad `~/Documents/sokratstudy.f6`; pet stabala = jedan projekt (S-015).
4. **Brojke = CLI:** ploča projekta i `cargo run -p sokratis-cli -- report …` daju iste brojke.
5. **Watcher (FSEvents umjesto ReadDirectoryChangesW):** commit u terminalu osvježi ploču bez klika, jednom; nema petlje na vlastiti upis u `.sokratis/` (S-016; BACKLOG m5, test pod opterećenjem).
6. **Obavijest:** prvi Alert → macOS traži dopuštenje; obavijest nosi ime „Sokratis", ne „Terminal" (u `tauri dev` ide pod terminalom, pa vrijedi samo izgrađeni `.app`).
7. **Autostart:** prekidač u Postavkama stvori ili makne `~/Library/LaunchAgents/…plist`; nakon odjave i prijave Sokratis se otvori.
8. **Zatvaranje:** X i `Cmd+W` → upit (S-036); `Cmd+Q` zatvara odmah (N12, S-043) — provjeriti da baza i watcher ne ostanu u lošem stanju.
9. **Dijalozi:** preimenovanje i uklanjanje projekta (N6 → T80 ako ne rade).
10. **Međuspremnik:** „Kopiraj putanju" daje putanju koja se otvara u Finderu (`Cmd+Shift+G`) (N5 nakon T78).
11. **WebKit (N11):** znak minus i brojevi u `hr` i `en`; poredak `Cvijet < Čokolada < D…`; datumi grafova; teme (4) i kontrast; `data-motion="off"` i Postavke → Pristupačnost → Smanji kretanje.
12. **Retina, trackpad, tamni način sustava:** oštrina SVG-a, glatko pomicanje, tema ne prati sustav ako to nije odluka.

### Faza 5 — dokumentacija, odluka, izdanje

| cigla | predmet |
|---|---|
| **T84** | (S-043 je zapisan 2026-11-23) · `CLAUDE.md`: stack (target po platformi umjesto „MSVC"), putanja prvog korisnika `~/Documents/sokratstudy.dev` · `README.md`: zahtjevi za macOS (Command Line Tools, build, Gatekeeper), „Windows only" → stvarno stanje · `ARCHITECTURE.md` §1 (putanja baze po OS-u, Mac bundle) · `RUST.md` §1–2 (redak MSVC, NSIS + DMG) · `TESTING.md` (izolirana baza na Macu: preusmjeriti `HOME`, odnosno ono što T77 čita, umjesto `LOCALAPPDATA`; `cargo fetch`) · `ROADMAP.md` (redak M0 za macOS) · `BACKLOG.md` (redak 8 „cross-platform": Mac stroj sada postoji) · `PROGRESS.md` + `CHANGELOG.md` |
| **T85** | *(O7: odmah, prije 1.0.2)* `.github/workflows/gates.yml`: matrica `macos-latest` + `windows-latest` → fmt · clippy · test · `npm run check` |
| **T86** | izdanje **1.1.0**: verzija u `Cargo.toml` (+ zrcalo `package.json`), Mac `.dmg` (+ Windows NSIS s Windows stroja ili CI-ja). **Tag i GitHub izdanje traže Leonov izričit OK** (pravilo #1) |

---

## 3 · Tokovi (kad Leon da OK)

| tok | cigle | vlasništvo datoteka |
|---|---|---|
| MAC-BRANE | T74 → T75 | `crates/sokratis-io/tests/**`, `crates/sokratis-core/src/profile.rs` (`inside_root`) |
| MAC-NOTICES | T76 | `apps/desktop/scripts/notices*`, `apps/desktop/tests/notices.test.ts` |
| MAC-LJUSKA | T77 → T81 → T82 → T87 | `apps/desktop/src-tauri/**` |
| MAC-SUČELJE | T78 → T79 → (T80) | `apps/desktop/src/**`, `apps/desktop/tests/**` (osim notices) |
| orkestrator | T83, Faza 4, T84–T86 | `docs/**`, verzija, `.github/` |

MAC-BRANE i MAC-NOTICES idu paralelno. Ostali tokovi kreću kad su obje u `main`-u, a T71/T72 iz 1.0.2
diraju iste mape kao MAC-LJUSKA i MAC-SUČELJE, pa se 1.0.2 dovršava prije njih (O5).
