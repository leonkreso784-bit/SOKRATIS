# RUST — konvencije i pojmovnik koji raste

> Sokratis je Leonov **prvi Rust-projekt**. Ovaj dokument ima dva posla: (1) pravila koja drže kod
> čitljivim i (2) pojmovnik koji raste s ciglama — svaki novi konstrukt dobiva redak kad se prvi put
> pojavi. Zaglavlje cigle kaže *zašto ovdje*; ovdje piše *što to jest*.

## 1 · Konvencije

| pravilo | zašto |
|---|---|
| **edition 2024**, stable toolchain, MSVC target | zadano za nov projekt; MSVC jer Tauri to očekuje na Windowsu |
| **`core` bez I/O-a** (S-002) | testira se bez gita; ownership bez lifetimeova prema vanjskom svijetu |
| **greške:** `thiserror` u `core`/`io` (tipizirane), `anyhow` samo u `cli` | biblioteka mora reći *koja* greška; binarna smije samo ispisati |
| **`unwrap()`/`expect()` samo u testovima**, uz iznimku koju **plan izričito imenuje i obrazloži** — danas jedina: `expect` na `tauri::Builder::run` (`apps/desktop/src-tauri/src/lib.rs`) | u produkcijskom kodu pad je bug, ne tok; iznimka vrijedi tamo gdje nastavak nema smisla (ljuska koja se nije digla) i mora stajati u cigli, ne u glavi |
| **`?` umjesto `match` na `Result` gdje samo propagiramo** | manje buke, ista sigurnost |
| **jedno pravilo = jedna datoteka** u `core/src/rules/` | novo pravilo ne dira stara; test uz kod |
| **`///` doc-komentar na svakom javnom tipu i funkciji** | `cargo doc` postaje dokumentacija jezgre |
| **`serde` derive na svemu što izlazi iz jezgre** | JSON je ugovor prema CLI-ju i sučelju |
| **bez `async` u M1** | nema mreže; `notify` u M2 je jedini kandidat |
| **lifetime samo gdje štedi kopiju koja bi boljela:** `IndicatorInput<'a>` je jedina iznimka u jezgri; `Context` je u vlasništvu (klonira se jednom po izvještaju) | pravila i parseri ostaju čitljivi bez `'a` |
| **imenovanje:** tipovi `PascalCase`, funkcije/polja `snake_case`, engleski | S-008; clippy to i traži |
| **svaki `std::process::Command` u `io`/desktopu ide kroz `git_command`** | M2/44, kvar 4 iz Leonovih nalaza — instalirana GUI aplikacija bljeskala konzolu `git.exe` pri svakom osvježenju; `git.rs::git_command(repo)` je jedino mjesto koje smije zvati `Command::new("git")` i na Windowsu postavlja `CREATE_NO_WINDOW` |

## 2 · Dopušteni crateovi i zašto

| crate | uloga | zašto baš on |
|---|---|---|
| `serde` + `serde_json` | JSON profil, ručni podaci, `Report` | de-facto standard; derive = bez ručnog koda. U `core` je `serde_json` **dev-ovisnost** — jezgra ga koristi samo u testovima, čitanje i pisanje JSON-a rade `io` i `cli` |
| `regex` | parseri dnevnika, plana, klasifikator | regexi se prenose 1:1 iz Python skripte |
| `thiserror` | tipizirane greške u `core`/`io` | kratke definicije enum-grešaka |
| `anyhow` | greške u `cli` | binarnoj je dovoljno „što je pošlo krivo" |
| `clap` (derive) | argumenti CLI-ja | `--json`, `--since` bez ručnog parsiranja |
| `chrono` (samo `io`) | današnji lokalni datum za `ReportInput.today` | odlučeno u planu M1 (T17): lokalni datum na Windowsu bez feature-gatea; jezgra datume računa sama (`civil.rs`), bez ovisnosti |
| `tempfile` (dev) | privremeni repo u io- i cli-testovima | čišćenje bez ručnog `rm` |

**Od M2** (pinane u `M2/1a`, `[workspace.dependencies]`; verzije su `max_stable_version` s crates.io
na 2026-09-18):

| crate | uloga | zašto baš on |
|---|---|---|
| `rusqlite` (`bundled`, samo `store`) | SQLite: registar projekata, postavke, snimke brojki (S-014) | zrelo vezivanje na SQLite bez ORM-a; **`bundled`** kompilira samu knjižnicu u binarnu — tuđi stroj nema `sqlite3.dll`, a instalacija ne smije tražiti ništa izvana |
| `notify` (samo `io`) | watcher nad `.git`, docs i `.sokratis` (S-016) | jedini održavan prenosiv watcher; koristi ReadDirectoryChangesW na Windowsu. Ovisnost je u manifestu od `M2/1a`; kod (`crates/sokratis-io/src/watch.rs`) je stigao ciglom M2/13 i od 2026-09-20 (tok IO) je u `main`-u; od DESKTOP-a (T30, `engine.rs`, 2026-09-21) ima pravog pozivatelja — nit motora čita njegov `WatchEvent` |
| `insta` (dev, `core`) | snapshot cijelog `Report`-a nad fixtureom pariteta | **vratio se u `M2/1a` s razlogom** (S-022): M1 ga je izbacio jer bi zamrznuo oblik koji se još mijenja, M2 gradi sučelje nad tim oblikom — pa svaka promjena JSON-a mora biti vidljiva u diffu snimke. Prvi snapshot-test je M2/2 (2026-09-18): `crates/sokratis-core/tests/snapshot.rs`, 467 redaka, 15 ključeva — namjerna promjena oblika: [`TESTING.md`](./TESTING.md) §1 |
| `tauri` 2.11.5 (+ `tauri-build` 2.6.3) | ljuska: prozori, ugovor prema sučelju (S-012, S-013) | **2.x, ne 3 alpha** — pinamo stabilno; jedina uključena mogućnost je `image-png` (**`tray-icon` maknut M2/45, S-036** — tray ukinut, `Cargo.lock` nepromijenjen jer je lock feature-agnostičan). Od DESKTOP-a (T29–T33, 2026-09-21) ljuska ima jedanaest stvarnih naredbi i dva događaja (`report_updated`, `signal_raised`) — prije toga je pokretala samo prazne prozore; **od M2/45 dvanaest naredbi** (+`quit`) |
| `tauri-plugin-dialog` 2.7.3 · `-notification` 2.4.0 · `-autostart` 2.5.1 · `-single-instance` 2.4.4 | odabir mape, obavijest na Alert, pokretanje sa sustavom, jedna instanca (S-020) | službeni plugini istog izdanja; svaki pokriva točno jednu Leonovu odluku, nijedan ne nosi logiku. Kod koji ih stvarno zove stiže DESKTOP-om (T29–T33): `add_project` (dialog), obavijest na prijelaz u Alert (`notification`), `set_setting("autostart")` (`autostart`, plugin PA baza — **od M2/45 prekidač u Postavkama, ne tray-kvačica**), drugi proces podiže postojeći prozor umjesto da se otvori (`single-instance`) |

Nijedan nov crate za nešto što projekt već ima: `serde`/`serde_json` u `store` nisu dev-ovisnost jer
snimka piše profil kao **kanonski JSON** (S-014) — `DefaultHasher` nije stabilan među verzijama, a
zaseban crate samo za hash bio bi pravilo #6 naopako.

**NSIS (instalater, T37, S-029) nije crate ni ovisnost u smislu pravila #6** — Taurijev bundler ga
sam preuzme u `%LOCALAPPDATA%\tauri\` pri prvoj gradnji `npm run tauri build` (alat izvan repoa, ne
ulazi u `Cargo.lock` ni `package-lock.json`, Ruling R18).

Nova ovisnost = namjerna radnja: redak ovdje + obrazloženje u commitu (CLAUDE.md #6).
`Cargo.lock` se commita. Ovisnost smije čekati svoju ciglu (danas: `notify` i `insta`), ali samo ako
je pinana u cigli koja je uvela i ovdje objašnjena — stanje koda je u
[`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11.

**npm-ovisnosti sučelja** (`apps/desktop/package.json`, verzije pinane bez `^`) nisu crateovi i ne
ulaze u ovu tablicu; što je zašto odabrano stoji u specu
[`../archive/ARHITEKTURA_M2.md`](../archive/ARHITEKTURA_M2.md) §7. **Od T53 (2026-09-24, S-035,
Leonov OK) += `d3-scale` 4.0.2 · `d3-shape` 3.2.0 · `d3-array` 3.2.4 · `d3-time-format` 4.1.0 + dev
`@types/*` za sve četiri** — matematika osi grafova (ticksovi, ljestvice, interpolacija), bez DOM-a
(rade i u vitestu); sve četiri su **ISC licenca**, isti d3-monorepo, izgled i boje ostaju naši
SVG-ovi (S-018 dopunjena, ne poništena).

**`rust-toolchain.toml`** (korijen repoa, od 2026-09-18) pina i sam kompajler, ne samo crateove:
`channel = "1.98.1"` + `rustfmt`/`clippy` kao komponente. Rustup ga čita sam kad se pokrene bilo koja
`cargo`/`rustup` naredba u repou i po potrebi preuzme točno tu verziju — tuđi stroj dobiva isti
toolchain bez ručnog koraka. Podizanje verzije je namjerna radnja kao i nova ovisnost: promjena
brojke + commit s obrazloženjem, ne tiha nuspojava.

## 3 · Pravilo „zašto Rust ovako" (CLAUDE.md #5)

Svaka cigla u zaglavlju datoteke (ili modula) nosi 2–5 redaka:

```rust
//! ZAŠTO RUST OVAKO (cigla M1/3 — parser git loga)
//! `Commit` posjeduje svoje `String`-ove (ne `&str`), pa parser vraća vlasništvo pozivatelju i
//! nema lifetimeova. Cijena je kopija teksta; za nekoliko tisuća commita to je nemjerljivo, a kod
//! ostaje čitljiv. `?` na svakom `parse::<i64>()` propagira grešku u `ParseError::BadTime`.
```

Test za pravilo: **Leon može pročitati datoteku i reći što radi.** Ako ne može, cigla nije gotova.

## 4 · Pojmovnik — raste s ciglama

| pojam | prvi put | jedna rečenica |
|---|---|---|
| ownership | M1 (model) | svaka vrijednost ima točno jednog vlasnika; kad vlasnik ode, vrijednost se oslobodi |
| borrow (`&T`, `&mut T`) | M1 (parser) | posudba bez preuzimanja vlasništva; više `&` ili jedan `&mut`, nikad oboje |
| `String` vs `&str` | M1 (parser) | `String` posjeduje tekst, `&str` ga gleda |
| `enum` s podacima | M1 (model) | varijanta može nositi vrijednosti; `WorkKind` i `Severity` su enumi |
| `match` | M1 (klasifikator) | iscrpno grananje po varijantama — kompajler traži da pokriješ sve |
| `Result<T, E>` i `?` | M1 (parser) | uspjeh ili greška kao tip; `?` propagira grešku prema gore |
| `Option<T>` | M1 (docs) | vrijednost ili ništa, bez `null` |
| `Vec<T>`, `HashMap<K, V>` | M1 (metrike) | rastući niz; mapa ključ → vrijednost |
| iteratori (`iter().filter().map()`) | M1 (metrike) | lijeni cjevovod nad kolekcijom; `collect()` ga materijalizira |
| `trait` i trait objekt (`Box<dyn Rule>`) | M1 (T13, `rules/mod.rs`); dopuna M2/14b (`io/src/cache.rs`, `&dyn CommitCache`/`&dyn GitSource`; `io/src/project.rs`, `Option<&dyn CommitCache>`); dopuna T29 (`desktop/src-tauri/src/cache.rs`, `StoreCache` — konkretna implementacija `CommitCache` nad `sokratis-store`, upravo ta „SQLite" posudba koju je M2/14b najavio) | ugovor koji tip ispunjava; `dyn` = poziv preko ugovora u vrijeme izvođenja, pa jedan `Vec` drži raznorodna pravila. Posuđeni trait-objekt (`&dyn`) kao argument funkcije nosi istu ideju bez vlasništva: pozivatelj bira implementaciju (SQLite, memorija u testu) po pozivu, a `io` je nikad ne poznaje po imenu (S-013); `Option<&dyn …>` dodaje „ili nijedna" istim mehanizmom kao `Option` nad bilo kojim tipom |
| `derive` | M1 (model) | kompajler generira implementaciju (`Debug`, `Clone`, `Serialize`) |
| `mod` i `pub` | M1 (workspace) | moduli su datoteke/mape; ništa nije javno dok ne kažeš |
| `#[cfg(test)]` | M1 (prvi test) | kod koji postoji samo pri `cargo test` |
| workspace | M0 | više crateova, jedan `Cargo.lock`, jedan `target/` |
| `[workspace.dependencies]` + `polje.workspace = true` | M1 (T1, korijenski `Cargo.toml`) | verzija ovisnosti piše se jednom za sav workspace; svaki crate je nasljeđuje s `polje.workspace = true` umjesto da je ponavlja |
| `#[serde(rename_all = "snake_case")]` | M1 (T1, `model.rs`) | enum-varijanta se (de)serializira kao `snake_case` string (`Debugging` → `"debugging"`), ne kao Rustov `PascalCase` naziv (S-008) |
| `#[serde(default, deny_unknown_fields)]` | M1 (T1, `profile.rs`) | polje koje nedostaje u JSON-u uzima `Default`; polje koje profil ne poznaje je greška deserializacije, ne tiho ignoriranje |
| `thiserror` derive s `#[source]`/`#[from]` | M1 (T1, `core`/`io` `error.rs`) | enum grešaka postaje pravi `std::error::Error`; `#[from]` daje automatsku `?`-pretvorbu tuđe greške (npr. `regex::Error`) u našu |
| `todo!()` kao stub-konvencija | M1 (T1, svi stubovi; u 0.1.0 ih više nema) | makro koji panicira porukom pri pozivu; potpis funkcije postoji prije tijela, pa cigla koja nedostaje puca jasnom porukom umjesto da tiho vrati krivu vrijednost |
| `impl Into<PathBuf>` | M1 (T1, `io/git.rs`) | argument prima bilo što pretvorivo u `PathBuf` (`&str`, `String`, `PathBuf`); pozivatelj ne mora sam zvati `.into()` |
| `collect::<Result<_, _>>()` | M1 (T1, `profile.rs:214,223`) | iterator stavki `Result<T, E>` se "okreće" u jedan `Result<Vec<T>, E>` — prvi `Err` prekida i vraća se, inače se skupe sve `Ok` vrijednosti |
| `let … else` | M1 (T3, `gitlog.rs`; kasnije `docs.rs`, `git.rs`) | rani izlazak (`return`/`continue`) kad uzorak ne odgovara, bez ugnježđenog `match` za jedan slučaj |
| `splitn(n, pat)` | M1 (T3, `gitlog.rs`) | dijeli string na najviše `n` dijelova; ostatak (npr. poruka commita sa `\|`) ostaje netaknut u zadnjem |
| `Vec::last_mut()` | M1 (T3, `gitlog.rs`) | izmjenjiva posudba zadnjeg elementa (npr. dodavanje sljedećeg `numstat` retka trenutnom commitu) bez ponovnog pretraživanja |
| `?` na `Option<T>` | M1 (T3, `civil.rs`; kasnije `docs.rs`) | isto načelo kao `?` na `Result`, ali funkcija prekida s `None` umjesto propagirane greške |
| byte-indeksiranje `&str` uz provjeru duljine | M1 (T3, `civil.rs`) | rezanje stringa po bajtovima puca na neispravnoj UTF-8 granici osim ako je duljina (ovdje: ASCII datum `YYYY-MM-DD`) provjerena unaprijed |
| Hinnantov `days_from_civil` | M1 (T3, `civil.rs`) | poznati algoritam (Howard Hinnant) za datum → broj dana bez kalendarske ovisnosti (`chrono` ostaje samo u `io`, S-011 kontekst) |
| `HashSet<&str>` / `HashSet<String>` za članstvo | M1 (T12, `docs.rs`; kasnije `git.rs`) | skup za provjeru „je li unutra" u O(1), bez duplikata i bez brige za redoslijed |
| `rsplit_once` | M1 (T12, `docs.rs`) | dijeli string na zadnjem pojavljivanju uzorka, vraća `Option<(prefiks, sufiks)>` |
| struct-update `..Default::default()` | M1 (T12, `docs.rs` test); dopuna M2/9 (`profile.rs` test — zamjena za `let mut p = Profile::default(); p.polje = …` koje clippy odbija kao `field_reassign_with_default`) | preostala polja literala preuzimaju zadane vrijednosti; test postavlja samo polje koje testira |
| `saturating_sub` | M1 (T12, `docs.rs`) | oduzimanje koje se zaustavlja na granici tipa umjesto da se prelije — ocjena docs-a zato nikad ne padne ispod 0 |
| `for (kind, re) in &p.classifier` + `*kind` na Copy-enumu; `Vec` umjesto `HashMap` kad je redoslijed ugovor | M1 (T6, `classify.rs`) | iteracija po posuđenom vektoru parova čuva ugovoreni redoslijed (planiranje > dokumentacija > …); `HashMap` ga ne bi garantirao |
| `lines()+filter_map(captures)` vs `(?m)`; stabilan `sort_by` | M1 (T4, `diary.rs`) | red-po-red s regexom bez multiline-zastavice je čitljiviji od jednog `(?m)` uzorka; `sort_by` čuva izvorni poredak jednakih ključeva |
| `include_str!` za fixture u testu | M1 (T4, `diary.rs`; kasnije `plan.rs`, M2/1a `store/src/store.rs`); dopuna M2/30 (`desktop/src-tauri/src/engine.rs`, `HR_DICT`/`EN_DICT`) | ugrađuje sadržaj datoteke u binarku pri kompajliranju — test ne čita disk u vrijeme izvođenja, a SQL migracija ne može se „izgubiti" uz instalaciju; izvan testa: obavijest OS-a čita SUČELJEV i18n-rječnik iz binarke umjesto duplog izvora natpisa (S-010) |
| `Vec::position()` + `&mut v[idx]` umjesto `iter_mut().find()`+`expect` | M1 (T5, `plan.rs`) | nađi indeks pa uzmi izmjenjivu referencu — izbjegava dvostruku posudbu koju kasnija izmjena susjednog polja komplicira |
| unit struct kao pravilo | M1 (T13, `rules/*.rs`) | struct bez polja koji nosi samo `impl Rule`; identitet pravila je u tipu, ne u podacima |
| `std::process::Command` bez shella | M1 (T15, `git.rs`) | pokreće vanjski proces s argumentima kao vektorom, bez interpretacije shella — nema escapinga ni injekcije |
| `Output { status, stdout, stderr }` | M1 (T15, `git.rs`) | rezultat pokrenutog procesa razdvaja izlazni kod od dva odvojena toka teksta |
| `String::from_utf8_lossy` | M1 (T15, `git.rs`) | pretvara bajtove u tekst; neispravan UTF-8 zamjenjuje znakom umjesto da panicira |
| `map_err` `io::Error` → `IoError` po `ErrorKind` | M1 (T15, `git.rs`) | grana po vrsti sistemske greške (npr. `NotFound`) prije pretvorbe u naš tipizirani error |
| `TempDir` RAII | M1 (T15, `tests/common`) | privremena mapa se obriše kad vrijednost izađe iz opsega — čišćenje bez ručnog `rm`, i kad test panicira |
| `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE` kroz `Command::env` | M1 (T15, `tests/common`) | test postavlja okolišne varijable da `git commit` dobije točan, ponovljiv datum umjesto trenutka izvođenja |
| `strip_prefix` + `filter_map` | M1 (T16, `git.rs`) | `strip_prefix` vrati `Option`; `filter_map` u istom prolazu odbaci retke gdje je rezultat `None` |
| `ErrorKind::NotFound` kao jedini opravdan fallback | M1 (T17, `project.rs`) | jedino „datoteka ne postoji" smije tiho pasti na zadano; svaka druga greška čitanja profila se propagira |
| rekurzivna fn s `&mut Vec` | M1 (T17, `project.rs`) | funkcija poziva samu sebe i puni zajednički izmjenjivi vektor (npr. popis docs-a po podmapama) umjesto da vraća i spaja rezultate |
| `strip_prefix` + `replace('\\', '/')` za prenosive putanje | M1 (T17, `project.rs`) | ukloni apsolutni prefiks pa zamijeni Windows-separator kosom crtom da izlaz bude isti na svim OS-ovima |
| `std::path::Component::ParentDir` | M1 (T17, `tests/project.rs`); dopuna M2/8 (`profile.rs`, `inside_root`: `Normal`/`CurDir` su jedine dopuštene varijante) | prepoznaje segment putanje (`..`, obična komponenta, `.`, korijen, disk) kao zaseban tip, ne kao string za usporedbu |
| `Vec<&Commit>` | M1 (T8, `metrics/hours.rs`) | vektor referenci umjesto vlasništva kad metrika samo čita commite koje već drži pozivatelj |
| `BTreeMap` | M1 (T8, `metrics/hours.rs`) | mapa sortirana po ključu — korisna kad se ispisuje po danu uzlazno bez naknadnog sortiranja |
| `entry().or_insert()` / `or_default()` | M1 (T8, `metrics/hours.rs`; kasnije `metrics/days.rs`); dopuna M2/4 (`metrics/visions.rs`, `BTreeMap<&str, u32>` — brojanje i sortiranje po ključu u jednom prolazu) | dohvati-ili-umetni u jednom potezu, bez dvostrukog pretraživanja mape |
| `Option::is_some_and` | M1 (T11b, `metrics/indicators.rs`; kasnije popravak C1, `parse/plan.rs`); dopuna M2/6 (`metrics/phases.rs`, `active_phases` — `phase_tag(...)` presuđuje pripada li commit fazi u jednom izrazu bez ugnježđenog `match`-a) | provjerava predikat nad sadržajem `Option` bez ručnog `match`/`unwrap` — nad `Captures::get` to je „grupa postoji **i** nije prazna" u jednom izrazu |
| `Option::filter` | M1 (T8, `metrics/hours.rs`) | zadrži `Some` samo ako sadržaj zadovoljava predikat, inače `None` |
| `BTreeMap<&str, Acc>` s posuđenim ključem | M1 (T9, `metrics/days.rs`); dopuna M2/46 (`metrics/branches.rs`, akumulator `commits`/`lines` po grani — sortiran ispis po imenu bez naknadnog sortiranja) | ključ mape je posudba iz izvornih podataka, ne kopija — akumulator ne smije nadživjeti izvor |
| privatni `#[derive(Default)]` akumulator | M1 (T9, `metrics/days.rs`) | pomoćni struct vidljiv samo unutar modula, s automatskim nula-stanjem za zbrajanje po danu/vrsti |
| `unwrap_or_else` s lijenim closureom | M1 (T10, `metrics/kinds.rs`) | zadana vrijednost se računa tek ako stvarno treba, ne unaprijed kao kod `unwrap_or` |
| `Option::copied` | M1 (T10, `metrics/kinds.rs`) | `Option<&T>` (posudba iz mape) postaje `Option<T>` kopijom sadržaja — posudba ne nadživi poziv |
| `flat_map().map().sum()` | M1 (T10, `metrics/kinds.rs`) | spljošti ugniježđene kolekcije, preslikaj pa zbroji u jednom cjevovodu |
| `for ph in &mut plan_phases` | M1 (T11a, `metrics/phases.rs`) | mutabilna iteracija po vlastitom vektoru: svaka faza se popuni (`from`/`to`/`days`/`commits`) u mjestu, bez alokacije novog vektora |
| zamka E0507 i `.as_deref()` | M1 (T11a, `metrics/phases.rs`, test) | `v[i].polje` kroz `Index` je posudba, ne vlasništvo — `Option<String>` se iz nje ne smije pomaknuti (E0507); `.as_deref()` posuđuje `Option<&str>` umjesto toga |
| lifetime (`'a`) | M1 (T11b, `metrics/indicators.rs`, `IndicatorInput<'a>`; već u §1) | koliko dugo posudba vrijedi; u `core` ih izbjegavamo vlasništvom — ovo je jedina iznimka |
| `clap` derive (`Parser`/`Subcommand`/`#[command]`/`#[arg(long)]`) | M1 (T18, `cli/main.rs`) | struktura/enum postaju CLI naredbe i argumenti bez ručnog parsiranja; `--help` se generira iz komentara i atributa |
| `anyhow::Result<i32>` do `main` | M1 (T18, `cli/main.rs`) | `run()` vraća izlazni kod umjesto da ga sam ispisuje; `?` diže bilo koju grešku (io/parse) do jednog mjesta koje odlučuje |
| `process::exit` na jednom mjestu | M1 (T18, `cli/main.rs`) | `main` je jedina funkcija koja stvarno završava proces zadanim kodom; svugdje drugdje kod je samo vrijednost koja putuje kroz `Result` |
| `env!("CARGO_BIN_EXE_sokratis")` u integracijskom testu | M1 (T18, `cli/tests/cli.rs`; kasnije `core/tests/parity.rs`) | staza do binarke koju je Cargo upravo izgradio, poznata u vrijeme kompajliranja — test pokreće pravi proces, ne funkciju |
| integracijski test u `tests/` (crate kao vanjski korisnik) | M1 (T18, `cli/tests/cli.rs`; kasnije `core/tests/parity.rs`) | datoteka u `tests/` vidi samo javni API crate-a, kao vanjski korisnik — ne može posegnuti za privatnim poljima kao `#[cfg(test)]` unutar `src/` |
| `std::fmt::Write` + `writeln!` u `String` | M1 (T19, `cli/table.rs`) | isti `writeln!` makro kao za stdout, ali cilj je `String` koji raste u memoriji — bez međuvektora redaka |
| `match &str` kao tablica prijevoda | M1 (T19, `cli/table.rs`) | grananje po tekstualnoj vrijednosti (ne enumu) prevodi engleski identifikator u hrvatski natpis; `_ => id` je siguran pad na nepoznati slučaj |
| `let _ = ` za namjerno ignoriran `Result` | M1 (T19, `cli/table.rs`) | eksplicitno „znam da ovo vraća `Result` i svjesno ga ne gledam" (`write!` u `String` ne može pasti) — clippy time zna da nije zabuna |
| `serde_json::Value` indeksiranje | M1 (T21, `core/tests/parity.rs`) | čitanje tuđeg JSON-a (Python fixture) bez definiranja Rust-tipa za njega — `value["polje"]` posuđuje po ključu/indeksu |
| `Captures::get(n) -> Option<Match>` | M1 (popravak C1, `parse/plan.rs`, `classify.rs`, `docs.rs`) | dohvat capture-grupe koji vraća `Option`; indeksiranje `c[n]` istu stvar radi **panikom** kad grupe nema, a regex dolazi iz tuđeg profila |
| `Regex::captures_len()` | M1 (popravak C1, `profile.rs`) | broj capture-grupa regexa + grupa 0; time se regex iz profila provjerava **prije** uporabe, pa je krivo napisan uzorak greška s imenom polja |
| enum-varijanta u obliku structa (`BadPattern { field, need, got }`) | M1 (popravak C1/C2/M2, `core/src/error.rs`, `io/src/error.rs`) | varijanta greške nosi **podatke**, ne samo tekst — poruka onda može imenovati krivca (koje polje, koji tekst), a pozivatelj po njoj granati |
| `Display` varijante vs lanac `#[source]` (`{e}` vs `{e:#}`) | M1 (popravak I5, `io/src/error.rs`) | poruka varijante i lanac uzroka su dvije stvari; `{e:#}` (anyhow) ispiše cijeli lanac, pa ga poruka ne smije ponavljati — inače se ista serde-greška vidi dvaput |
| `#[error(transparent)]` | M1 (popravak I5, `io/src/error.rs`) | varijanta bez vlastite rečenice prepušta cijeli `Display` uzroku (npr. `std::io::Error`) |
| `Path::components().collect::<PathBuf>()` | M1 (popravak I5, `io/src/project.rs`) | ponovno sastavljanje putanje iz komponenata normalizira razdjelnike (`/` iz gita + `\` iz `join`) u jedan oblik |
| `clap::Error::use_stderr()` + `try_parse()` | M1 (popravak I4, `cli/src/main.rs`) | `try_parse` vrati grešku umjesto da sam izađe; `use_stderr()` razlikuje pogrešnu uporabu (stderr → izlaz 3) od `--help`/`--version` (stdout → 0) |
| `#[arg(long, conflicts_with = "…")]` | M1 (popravak M10, `cli/src/main.rs`) | isključivost dvije zastavice je **deklaracija** u atributu, ne `if` u kodu; `clap` je sam prijavi kao pogrešnu uporabu |
| `min()` nad `String` (leksikografski `Ord`) | M1 (popravak I1, `io/src/project.rs`) | `YYYY-MM-DD` se kao tekst poredava isto kao kalendarski, pa je „najraniji datum" običan `min` bez parsiranja |
| predznak nule u IEEE 754 (`-0.0`, `is_sign_positive`) | M1 (popravak M1, `metrics/indicators.rs`) | `f64::sum()` praznog iteratora je `-0.0`, a `assert_eq!(-0.0, 0.0)` je istina — mjera zato dobiva `+ 0.0`, a test gleda predznak |
| `[dev-dependencies]` | M1 (popravak M7, `core/Cargo.toml`) | ovisnost koju traže samo testovi ne ulazi u isporučenu biblioteku; ne dijeli se ni između crateova (zato `cli` ima svoj `tests/common`) |
| `mod common;` dijeljen između testnih binarija | M1 (popravak I7, `cli/tests/common/mod.rs`) | svaki `tests/*.rs` je **svoj** crate, pa se pomoćni modul u njega uključuje izvorno (`mod`), a ne linka kao biblioteka |
| `pub(crate)` | M2/1a (`store/src/store.rs`) | vidljivost usko: polje smiju vidjeti moduli **istog** cratea (registar, postavke, snimke), vanjski korisnik ne — `Store` tako posjeduje `Connection` bez da je izlaže |
| `const` tablica + `include_str!` za migracije | M2/1a (`store/src/store.rs`) | `&[(i64, &str)]` u konstanti drži par „verzija sheme → SQL"; petlja primijeni samo ono što je novije od zapisane verzije |
| `Connection::query_row` s closureom nad retkom | M2/1a (`store/src/store.rs`) | rusqlite ne mapira tipove sam: closure `\|r\| r.get(0)` kaže koji stupac i u koji Rust-tip ide, pa je konverzija vidljiva na mjestu upita |
| `#![cfg_attr(…, windows_subsystem = "windows")]` | M2/1a (`desktop/src-tauri/src/main.rs`) | atribut **na razini cratea** (`#!`) koji se primjenjuje uvjetno; ovdje: release build bez konzolnog prozora, debug ga zadržava zbog `eprintln!`-a |
| builder-lanac `tauri::Builder::default()…run()` + `generate_context!` | M2/1a (`desktop/src-tauri/src/lib.rs`) | svaka metoda vraća `Self` pa se plugini nižu lancem; `run` uzima **vlasništvo** i ne vraća se do izlaza aplikacije, a makro ugradi `tauri.conf.json` u binarku pri kompilaciji |
| `Option::is_none_or` | M2/3 (`report.rs`, filtar `until`) | izražava „nema gornje granice ILI je vrijednost unutar nje" u jednom izrazu nad `Option`, bez ugniježđenog `match`-a u `filter`-u |
| let-chain (`if let PATTERN = IZRAZ && UVJET`) | M2/3 (`report.rs`, provjera oblika `until`); dopuna M2/11 (`io/src/git.rs::last_changes`, spajanje retka `@@<unix>` s pripadnim putanjama u jednoj provjeri); dopuna M2/50 (`io/src/git.rs::worktree_heads`, „imam sha I čekam putanju" kao jedna provjera pri parsiranju `git worktree list --porcelain`) | stabilno od edition 2024: `let`-uzorak i dodatni bool-uvjet u istom `if`, bez ugniježđenog `if` unutar `if let` |
| `matches!(izraz, uzorak1 \| uzorak2)` | M2/8 (`profile.rs`, `inside_root`) | provjerava odgovara li vrijednost jednom od navedenih uzoraka i vraća `bool` u jednom izrazu — kraće od `match` koji bi za isti test trebao granu za svaku varijantu |
| stražarska klauzula (guard clause, rani `return`) | M2/9 (`profile.rs`, `is_test_path`) | isključenje (`test_path_exclude`) se provjerava PRIJE svih uključivih pravila; jedan pogodak presiječe ostatak funkcije bez ugniježđenih `if`-ova |
| `Iterator::zip` nad paralelnim nizovima | M2/5 (`metrics/kinds.rs`, `kind_stats`); dopuna M2/46 (`metrics/branches.rs::branch_stats`, `rows.iter().zip(commits)`) | spaja `rows` i `commits` (isti redoslijed, ista dužina) u parove bez ručnog indeksiranja (`v[i]`) — stane na kraću sekvencu ako duljine ikad ne bi bile jednake |
| `pub(crate) mod` (na modulu, ne funkciji) | M2/7 (`report.rs`, `mod tests`) | vidljivost cijelog modula ograničena na crate umjesto na roditelja: sestrinski modul (`snapshot::tests`) smije posuditi `report::tests::input()`, vanjski korisnik cratea ne vidi ništa novo |
| `#[derive(PartialOrd, Ord)]` + `Iterator::max()` nad enumom | M2/7 (`snapshot.rs::worst_severity`; derive na `Severity` je od M1/T1) | redoslijed varijanti u definiciji enuma (`Info < Warn < Alert`) postaje ugovor za usporedbu — `max()` nad nizom enum-vrijednosti vrati „najtežu" bez ijedne grane `match`-a |
| `?` unutar `filter_map`-ove zatvorenja | M2/7 (`snapshot.rs::diff`) | `let b = *before.get(id)?;` unutar zatvorenja koje `filter_map` očekuje: `None` iz `?` znači „preskoči ovaj element", ne prekid cijele funkcije — isto načelo kao `?` na `Option` (§4 gore), ali primijenjeno po elementu unutar cjevovoda |
| više `impl` blokova istog tipa u različitim datotekama | M2/15 (`store/src/registry.rs`; nastavlja se u `settings.rs`, `snapshots.rs`, `cache.rs`) | Rust dopušta razdvajanje `impl Store` po modulima — `store.rs` drži vezu i migracije, a svaki „posao" (registar, postavke, snimke, keš) svoju datoteku |
| `params![...]` makro (rusqlite) | M2/15 (`store/src/registry.rs`) | veže vrijednosti u SQL upit bez ručnog formatiranja teksta — nema injekcije, nema quotinga |
| `.optional()` (`rusqlite::OptionalExtension`) nad `query_row` | M2/15 (`store/src/registry.rs`) | „nema retka" (npr. postavka koja nikad nije zapisana) postaje `Ok(None)` umjesto greške `QueryReturnedNoRows` |
| `Path`/`PathBuf::to_string_lossy()` | M2/15 (`store/src/registry.rs`) | pretvara putanju u tekst za SQLite (koji ne zna za `PathBuf`); neispravan UTF-8 u putanji zamjenjuje znakom umjesto panike — isto načelo kao `String::from_utf8_lossy` (§4 gore), na drugom tipu |
| `Connection::unchecked_transaction()` za skupni upis | M2/15 (`store/src/registry.rs::set_worktrees`; kasnije M2/17 `save_snapshot`, M2/18 `put_commits`) | više redaka u jednoj transakciji umjesto `execute` po retku — jedan fsync umjesto stotina, i pad usred upisa vraća SVE (rollback bez `commit()`) |
| SQL upsert `INSERT … ON CONFLICT … DO UPDATE` | M2/16 (`store/src/settings.rs`) | piše-ili-mijenja u jednom SQL-pozivu bez utrke između čitanja i pisanja — zamjena za ručni „SELECT pa INSERT-ili-UPDATE" |
| atomarni `rename` (piši `.tmp` pored cilja, onda preimenuj) | M2/10 (`io/src/project.rs::write_atomic`) | preimenovanje je atomarno na razini datotečnog sustava — pad usred pisanja nikad ne ostavi pola JSON-a na mjestu datoteke koju git prati |
| `Cell<u32>` | M2/11 (`io/src/git.rs`, brojač pokrenutih git-procesa) | unutarnja promjenjivost kroz `&self` bez `mut`: polje se mijenja iznutra dok trait `GitSource` posuđuje `&self` nepromjenjivo — mjerač za `tests/perf.rs` |
| `mpsc::Sender`/`Receiver` | M2/13 (`io/src/watch.rs`); dopuna M2/30 (`desktop/src-tauri/src/{state,engine}.rs`, `Mutex<Option<Receiver<WatchEvent>>>` + `guard.take()`) | kanal je granica vlasništva između tuđe niti (`notify`) i naše: pošiljatelj gura sirove putanje, primatelj ih preuzima na drugoj strani; `take()` vadi primatelja iz dijeljenog stanja PRIJE blokirajućeg `recv()` u petlji — brava se ne smije držati preko čekanja, inače bi svaka druga nit koja treba `AppState` čekala zauvijek |
| `Arc<Mutex<_>>` za dijeljeno stanje između niti | M2/13 (`io/src/watch.rs`) | `Arc` broji vlasnike, `Mutex` daje red za pisanje — obje niti (notify-jeva i naša pozadinska) vide isti popis ruta i potisnutih putanja |
| oporavak iz otrovanog `Mutex`-a (`PoisonError::into_inner`) | M2/13 (`io/src/watch.rs::lock_shared`) | kad jedna nit panira dok drži bravu, `Mutex` se „otruje"; `unwrap_or_else(\|p\| p.into_inner())` svjesno nastavlja sa stanjem ispod jer ovdje nema invarijante koja bi ostala polupisana — alternativa `expect()`-u (pravilo #6) opravdana u zaglavlju cigle |
| `thread::spawn` s `loop`/`recv_timeout` kao „otkucaj" | M2/13 (`io/src/watch.rs::debounce_loop`); dopuna M2/30 (`desktop/src-tauri/src/engine.rs::start` — nit motora čeka `rx.recv()` u petlji; `tray.rs` — „Osvježi sve" pokreće `engine::refresh_all` u ZASEBNOJ niti da rukovatelj izbornika na glavnoj niti ne blokira) | pozadinska nit se budi periodički umjesto da čeka zauvijek; izlazi sama kad `RecvTimeoutError::Disconnected` javi da su svi pošiljatelji nestali, bez ručnog signala za gašenje |
| `std::slice::from_ref` | M2/13 (`io/tests/watch.rs`) | pretvara `&T` u `&[T]` bez kloniranja — jedan element kao rezanje, gdje `clippy::cloned_ref_to_slice_refs` odbija `&[x.clone()]` |
| `Box<dyn Error + Send + Sync>` | M2/14b (`io/src/cache.rs`, `CacheError`) | tip greške čiji KONKRETAN nositelj `io` ne poznaje (dolazi iz stranog cratea, npr. `rusqlite`, ožičen tek u desktopu T29) — jedini potpis koji prihvaća „bilo koja greška" i smije putovati preko granice dretvi (`Send + Sync`) |
| `Stdio::piped()` + `Child::stdin.take()` | M2/14b (`io/src/git.rs::run_with_stdin`) | otvara pipe na stdin/stdout/stderr djeteta procesa; `take()` uzima vlasništvo nad `Option<ChildStdin>` da se pipe stvarno ZATVORI (drop) prije `wait_with_output()` — bez toga `git --stdin` čeka EOF zauvijek |
| inverz parsera / okrugli put (round-trip) | M2/14b (`core/src/parse/gitlog.rs::format_gitlog`, test `parse → format → parse`) | funkcija koja poništava drugu (`format_gitlog` je inverz `parse_git_log`-a); test okruglog puta tvrdi da su ulaz i rezultat nakon oba prolaza jednaki, ne samo da svaki prolaz zasebno radi |
| `Mutex<T>` (bez `Arc`) kroz Tauri `State` | M2/29 (`desktop/src-tauri/src/state.rs`, `AppState`) | Tauri sam čuva `AppState` iza svoje tablice stanja i posuđuje ga naredbama kao `State<'_, AppState>` na bilo kojoj niti; svako polje ipak treba vlastiti `Mutex` jer svaki `invoke` iz sučelja stiže na svojoj niti — `lock().map_err(text)` pretvara `PoisonError` u tekst na granici IPC-a, umjesto `unwrap()` |
| std `Mutex` nije reentrantan (zamka adaptera) | M2/29 (`desktop/src-tauri/src/cache.rs`, `StoreCache`) | drugi `lock()` iz ISTE niti dok je prvi još otvoren blokira zauvijek (nema „ponovnog ulaska" kao u nekim drugim jezicima) — `compute` zato NE smije držati bravu storea preko `Project::input_cached`; adapter je zaključava kratko, samo unutar `cached`/`store` |
| `AtomicBool` + `Ordering::SeqCst` | M2/31 (`desktop/src-tauri/src/splash.rs`, `SplashState`) | tri zastavice se SAMO postavljaju/čitaju pojedinačno, nikad kao jedna transakcija, pa atomska operacija bez `Mutex`-a dovoljno štiti svaku; **`SeqCst` je ovdje NUŽAN, ne „radi jednostavnosti"** — `intro_done` i `load_done` se čitaju/pišu unakrsno iz dvije niti (Dekkerov obrazac), a slabiji poredak (`Relaxed`/`Acquire`/`Release`) dopušta izgubljeno buđenje (jedna nit ne vidi na vrijeme pisanje druge, glavni prozor se nikad ne pokaže); `shown.swap(true, SeqCst)` daje „točno jednom" jer `swap` atomski čita staro I piše novo u istom koraku |
| `AppHandle`, `State<'_, T>`, `#[tauri::command]`, `generate_handler!` | M2/29 (`desktop/src-tauri/src/{commands,lib}.rs`) | `AppHandle` je jeftin klon-ključ do aplikacije iz bilo koje niti (šalje događaje, dohvaća stanje); `State<'_, T>` je posuđeni pristup dijeljenom stanju unutar jedne naredbe; `#[tauri::command]` pretvara običnu funkciju u RPC koji sučelje zove kroz `invoke(ime, args)`; `generate_handler!` na KOMPAJLIRANJU provjeri da svako navedeno ime postoji i ima ispravan potpis — tipfeler je greška prevoditelja, ne runtime iznenađenje |
| `include_bytes!` | M2/32 (`desktop/src-tauri/src/tray.rs`, tray-ikona) | isto što i `include_str!` (niže), ali za binarni sadržaj (PNG) — ikona putuje UNUTAR `.exe`-a, instalacija ne treba zasebnu datoteku pored njega |
| `#[serde(tag = "preset", rename_all = "snake_case")]` + `#[serde(rename = "7d")]` | M2/29 (`desktop/src-tauri/src/commands.rs`, `Range`) | vanjski tag u JSON-u kao poseban ključ (`"preset"`) uz podatke varijante u istoj razini — `{"preset":"7d"}`, ne `{"SevenDays":{...}}`; `rename` na pojedinoj varijanti prepiše automatsko ime (`SevenDays` → `"7d"`) kad ugovor prema TS-u traži baš taj tekst |
| `cfg!(debug_assertions)` vs `#[cfg(...)]` | M2/37 (`desktop/src-tauri/src/state.rs`, `db_file_name`) | `cfg!(...)` je makro koje se PRI KOMPILACIJI svede na `true`/`false` — obje grane ostaju u binarnoj i obje se daju testirati u istom buildu; atribut `#[cfg(...)]` umjesto toga NEPOTREBAN kod izbacuje iz binarnog zapisa. Ovdje bira ime datoteke baze (`sokratis-dev.db` u debug, `sokratis.db` u release) bez dvije zasebne binarne |
| `#[cfg(windows)]` + `std::os::windows::process::CommandExt::creation_flags` | M2/44 (`io/src/git.rs::git_command`) | `#[cfg(windows)]` bira kod PRI KOMPILACIJI — `CommandExt` ne postoji na drugim metama, pa se ni ne pokuša kompilirati ondje (nema mrtve grane preko `if cfg!`); `creation_flags(0x0800_0000)` (`CREATE_NO_WINDOW`) je zastavica koju Windows dijete-proces ne može pročitati natrag, otud test niže čita izvor |
| `concat!` sastavljena igla u testu koji čita izvor | M2/44 (`io/src/git.rs`, test `command_new_lives_only_inside_git_command`) | `concat!("Command::", "new(")` sastavlja niz u vrijeme kompilacije tek u tijelu testa — da SAM test ne sadrži doslovan tekst `Command::new(`, inače bi `include_str!` na vlastitom izvoru pronašao i iglu i zaglavlje pa test nikad ne bi mogao proći |
| `HashSet::insert` kao predikat u `filter` | M2/47 (`core/src/parse/diary.rs::parse_diaries`) | `insert` vraća `bool` (je li vrijednost NOVA) — korišten izravno kao predikat, `filter(\|d\| seen.insert(...))` odbacuje duplikat po (datum, naslov) bez ugniježđene provjere „je li već unutra pa dodaj" |
| `enum` s lifetimeom umjesto `String` (`Scope<'a>`) | M2/49 (`io/src/git.rs`) | varijanta enuma nosi POSUĐENI `&'a str` (ime grane) umjesto vlasničkog `String` — pozivatelj (`&self.profile.default_branch`) ostaje vlasnik, `log`/`rev_list`/`commit_sources` ga ne kloniraju pri svakom pozivu; `#[derive(Copy)]` je moguć baš zato što enum ne nosi ništa skuplje od reference |
| `--not` u `git log` i redoslijed argumenata | M2/49 (`io/src/git.rs::commit_sources`) | `--not` negira SVE reference navedene IZA sebe do kraja naredbe — `--branches --not <default>` znači „sve grane osim dostižnih iz zadane"; obrnut redoslijed bi negirao i same grane, pa poredak argumenata ovdje NIJE stilska stvar |
| `str::split_once` | M2/49 (`io/src/git.rs::parse_commit_sources`) | dijeli string na PRVOM pojavljivanju uzorka u `(prije, poslije)` — suprotno od već postojećeg `rsplit_once` (§4 gore, dijeli na ZADNJEM); SHA nikad ne sadrži `\|`, pa prvi razdjelnik pouzdano odvaja SHA od imena grane, čak i kad grana sama ima `\|` u imenu |
| `Option::take` u parseru porcelaina | M2/50 (`io/src/git.rs::worktree_heads`) | `path.take()` isprazni `Option` i vrati staru vrijednost u ISTOM koraku — kad par (putanja, HEAD) upari, isti buffer služi za SLJEDEĆI blok bez ručnog `path = None` na kraju petlje |
| `sort_by_key` + `std::cmp::Reverse` | M2/50 (`io/src/project.rs::worktrees_by_recency`) | silazan, STABILAN sort po `author_time` bez pisanja vlastitog komparatora — `Reverse(x)` okrene `Ord` naopako, `sort_by_key` čuva relativni poredak jednakih ključeva (za razliku od `sort_unstable_by_key`) |
| `--no-walk=unsorted` | M2/50 (`io/src/git.rs::commit_times`) | `--no-walk` čita TOČNO navedene SHA-ove bez šetnje njihovim roditeljima; `unsorted` gasi i sortiranje po topologiji — za kartu `sha → vrijeme` redoslijed ispisa ne igra ulogu, pa se njegova cijena ne plaća |
| `#[allow(dead_code)]` u `tests/common` | M2/36 (`io/tests/common/mod.rs::commit_empty`); dopuna M2/50 (`add_worktree`/`commit_at`) | `mod common` se prevodi ZASEBNO za svaki `tests/*.rs` cilj — pomoćna metoda koju koristi samo JEDAN test-cilj bi u svim ostalima bila „nikad korištena" i pala na `cargo clippy -D warnings` bez ove napomene |
| `#[default]` na varijanti enuma + `#[serde(rename)]` | M2/46 (`core/src/model.rs::BranchScope`) | `#[default]` bira KOJA varijanta nastaje iz `Default::default()` (ovdje `AllBranches`) bez ručnog `impl Default`; `#[serde(rename = "…")]` po varijanti daje JSON tekst (`"all"`/`"default"`) koji se ne poklapa s Rustovim `PascalCase` imenom varijante |
| trait `tauri::Emitter` | M2/45 (`desktop/src-tauri/src/lib.rs`) | Tauri 2 dijeli metode `AppHandle`/`Window` po traitovima koje moraš uvesti u opseg da ih vidiš — `Manager` daje pristup dijeljenom stanju (`app.state()`), `Emitter` daje slanje događaja (`window.emit(...)`); `use tauri::Emitter` je zato NUŽAN uvoz, ne stil |
| `app.exit(0)` | M2/45 (`desktop/src-tauri/src/commands.rs::quit`) | prekida `tauri::Builder::run`-ovu petlju događaja i sam proces s danim kodom — poziva ga SAMO naredba `quit`, na potvrdu upita, jer prekida i nit motora usred izračuna (snimke su transakcije, S-014, pa prekid ne ostavlja pola dana u bazi) |

Redak se dodaje **u cigli u kojoj se pojam prvi put pojavi**, s referencom na datoteku.

## 5 · TS/Svelte uz Rust (tok SUČELJE, od M2)

Sučelje se piše u TypeScriptu i Svelteu 5, ali po istim pravilima: zaglavlje `// ZAŠTO OVAKO (cigla
M2/N — naziv)` od 2–5 redaka u svakoj novoj datoteci, hrvatski komentari, engleski identifikatori
(S-008), nijedan natpis izvan i18n rječnika (S-021), nijedna boja izvan tokena (S-017). Brane su
`npm run check` u `apps/desktop`, ne `cargo` ([`TESTING.md`](./TESTING.md) §1). Pojmovi koji se ovdje
uvode idu u ovu tablicu — **ne** u §4, koji je Rust.

| pojam | prvi put | jedna rečenica |
|---|---|---|
| `mount(App, { target })` | M2/1a (`src/main.ts`) | Svelte 5 više ne radi `new App(...)`: komponenta se montira funkcijom na postojeći DOM-čvor |
| runa `$state` | M2/1a (`src/App.svelte`) | vrijednost označena runom je reaktivna — promjena sama osvježi svaki prikaz koji je čita, bez `store`-a i bez `$:` |
| dva ulaza u Viteu (`index.html` + `splash.html`) | M2/1a (`vite.config.ts`) | jedan build daje dvije HTML stranice; Tauri ih otvara kao dva prozora (splash i glavni, S-019) |
| `svelte-check` | M2/1a (`package.json`, `npm run check:svelte`) | tipovi se provjeravaju i **unutar** `.svelte` datoteka, ne samo u `.ts` — to je TS-ekvivalent `cargo clippy` brane |
| `{#snippet}` | M2/28 (`views/Visions.svelte`, `visionForm`) | Svelte 5 blok markupa koji se poziva na više mjesta (ovdje: iznad tablice za dodavanje, unutar retka za uređivanje) bez kopiranja — isti obrazac, jedan izvor |
| `$props.id()` | M2/39 (`lib/charts/Ring.svelte`, maska otkrivanja) | Svelte 5 daje svakoj instanci komponente jedinstven, stabilan ID bez ručnog brojača ili `crypto.randomUUID()` — ovdje čuva `id` SVG-`<mask>`-e jedinstvenim po prstenu, jer bi dva prstena na istom ekranu inače dijelila `<mask id="…">` preko istog imena |
| `{#key IZRAZ}` | M2/40 (`App.svelte`, `` {#key `${app.epoch}:${app.view}`} `` oko lanca pogleda) | kad se vrijednost izraza promijeni, Svelte DEMONTIRA blok i MONTIRA ga iznova umjesto da ga ažurira u mjestu — spojeni ključ (`epoch` + `view`) tjera remontiranje i pri prvoj vidljivoj animaciji (`epoch` 0→1) i pri svakoj promjeni pogleda, jednim mehanizmom |
| `Snippet<[Frame]>` — tipiziran snippet kao prop | M2/54 (`lib/charts/Chart.svelte`, `children: Snippet<[Frame]>`) | uvezeni TS-tip `Snippet` iz `'svelte'` opisuje dijete-blok koji prima TOČNO ONE argumente (ovdje `Frame` iz `layout.ts`) — poziv `{@render children(f)}` je time provjeren u vrijeme kompajliranja, za razliku od golog `{#snippet}` (M2/28) koji ne nosi generički tip |
