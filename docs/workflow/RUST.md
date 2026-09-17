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
| **`unwrap()`/`expect()` samo u testovima** | u produkcijskom kodu pad je bug, ne tok |
| **`?` umjesto `match` na `Result` gdje samo propagiramo** | manje buke, ista sigurnost |
| **jedno pravilo = jedna datoteka** u `core/src/rules/` | novo pravilo ne dira stara; test uz kod |
| **`///` doc-komentar na svakom javnom tipu i funkciji** | `cargo doc` postaje dokumentacija jezgre |
| **`serde` derive na svemu što izlazi iz jezgre** | JSON je ugovor prema CLI-ju i sučelju |
| **bez `async` u M1** | nema mreže; `notify` u M2 je jedini kandidat |
| **lifetime samo gdje štedi kopiju koja bi boljela:** `IndicatorInput<'a>` je jedina iznimka u jezgri; `Context` je u vlasništvu (klonira se jednom po izvještaju) | pravila i parseri ostaju čitljivi bez `'a` |
| **imenovanje:** tipovi `PascalCase`, funkcije/polja `snake_case`, engleski | S-008; clippy to i traži |

## 2 · Dopušteni crateovi (M1) i zašto

| crate | uloga | zašto baš on |
|---|---|---|
| `serde` + `serde_json` | JSON profil, ručni podaci, `Report` | de-facto standard; derive = bez ručnog koda |
| `regex` | parseri dnevnika, plana, klasifikator | regexi se prenose 1:1 iz Python skripte |
| `thiserror` | tipizirane greške u `core`/`io` | kratke definicije enum-grešaka |
| `anyhow` | greške u `cli` | binarnoj je dovoljno „što je pošlo krivo" |
| `clap` (derive) | argumenti CLI-ja | `--json`, `--since` bez ručnog parsiranja |
| `chrono` (samo `io`) | današnji lokalni datum za `ReportInput.today` | odlučeno u planu M1 (T17): lokalni datum na Windowsu bez feature-gatea; jezgra datume računa sama (`civil.rs`), bez ovisnosti |
| `tempfile` (dev) | privremeni repo u io-testovima | čišćenje bez ručnog `rm` |
| `insta` (dev) | snapshot testovi CLI izlaza | snimka JSON-a je čitljiva u PR-u |

Nova ovisnost = namjerna radnja: redak ovdje + obrazloženje u commitu (CLAUDE.md #6).
`Cargo.lock` se commita.

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
| `trait` i trait objekt (`Box<dyn Rule>`) | M1 (pravila) | ugovor koji tip ispunjava; `dyn` = poziv preko ugovora u vrijeme izvođenja |
| `derive` | M1 (model) | kompajler generira implementaciju (`Debug`, `Clone`, `Serialize`) |
| `mod` i `pub` | M1 (workspace) | moduli su datoteke/mape; ništa nije javno dok ne kažeš |
| `#[cfg(test)]` | M1 (prvi test) | kod koji postoji samo pri `cargo test` |
| workspace | M0 | više crateova, jedan `Cargo.lock`, jedan `target/` |
| `[workspace.dependencies]` + `polje.workspace = true` | M1 (T1, korijenski `Cargo.toml`) | verzija ovisnosti piše se jednom za sav workspace; svaki crate je nasljeđuje s `polje.workspace = true` umjesto da je ponavlja |
| `#[serde(rename_all = "snake_case")]` | M1 (T1, `model.rs`) | enum-varijanta se (de)serializira kao `snake_case` string (`Debugging` → `"debugging"`), ne kao Rustov `PascalCase` naziv (S-008) |
| `#[serde(default, deny_unknown_fields)]` | M1 (T1, `profile.rs`) | polje koje nedostaje u JSON-u uzima `Default`; polje koje profil ne poznaje je greška deserializacije, ne tiho ignoriranje |
| `thiserror` derive s `#[source]`/`#[from]` | M1 (T1, `core`/`io` `error.rs`) | enum grešaka postaje pravi `std::error::Error`; `#[from]` daje automatsku `?`-pretvorbu tuđe greške (npr. `regex::Error`) u našu |
| `todo!()` kao stub-konvencija | M1 (T1, svi stubovi) | makro koji panicira porukom pri pozivu; potpis funkcije postoji prije tijela, pa cigla koja nedostaje puca jasnom porukom umjesto da tiho vrati krivu vrijednost |
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
| struct-update `..Default::default()` | M1 (T12, `docs.rs` test) | preostala polja literala preuzimaju zadane vrijednosti; test postavlja samo polje koje testira |
| `for (kind, re) in &p.classifier` + `*kind` na Copy-enumu; `Vec` umjesto `HashMap` kad je redoslijed ugovor | M1 (T6, `classify.rs`) | iteracija po posuđenom vektoru parova čuva ugovoreni redoslijed (planiranje > dokumentacija > …); `HashMap` ga ne bi garantirao |
| `lines()+filter_map(captures)` vs `(?m)`; stabilan `sort_by` | M1 (T4, `diary.rs`) | red-po-red s regexom bez multiline-zastavice je čitljiviji od jednog `(?m)` uzorka; `sort_by` čuva izvorni poredak jednakih ključeva |
| `include_str!` za fixture u testu | M1 (T4, `diary.rs`; kasnije `plan.rs`) | ugrađuje sadržaj datoteke u binarku pri kompajliranju — test ne čita disk u vrijeme izvođenja |
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
| `std::path::Component::ParentDir` | M1 (T17, `tests/project.rs`) | prepoznaje `..` segment putanje kao zaseban tip komponente, ne kao običan string za usporedbu |
| `Vec<&Commit>` | *na grani `feat/core-metrics`* | vektor referenci umjesto vlasništva kad metrika samo čita commite koje već drži pozivatelj |
| `BTreeMap` | *na grani `feat/core-metrics`* | mapa sortirana po ključu — korisna kad se ispisuje po danu uzlazno bez naknadnog sortiranja |
| `entry().or_insert()` / `or_default()` | *na grani `feat/core-metrics`* | dohvati-ili-umetni u jednom potezu, bez dvostrukog pretraživanja mape |
| `Option::is_some_and` | *na grani `feat/core-metrics`* | provjerava predikat nad sadržajem `Option` bez ručnog `match`/`unwrap` |
| `Option::filter` | *na grani `feat/core-metrics`* | zadrži `Some` samo ako sadržaj zadovoljava predikat, inače `None` |
| `BTreeMap<&str, Acc>` s posuđenim ključem | *na grani `feat/core-metrics`* | ključ mape je posudba iz izvornih podataka, ne kopija — akumulator ne smije nadživjeti izvor |
| privatni `#[derive(Default)]` akumulator | *na grani `feat/core-metrics`* | pomoćni struct vidljiv samo unutar modula, s automatskim nula-stanjem za zbrajanje po danu/vrsti |
| `unwrap_or_else` s lijenim closureom | *na grani `feat/core-metrics`* | zadana vrijednost se računa tek ako stvarno treba, ne unaprijed kao kod `unwrap_or` |
| `flat_map().map().sum()` | *na grani `feat/core-metrics`* | spljošti ugniježđene kolekcije, preslikaj pa zbroji u jednom cjevovodu |
| lifetime (`'a`) | *kad se pojavi* | koliko dugo posudba vrijedi; u `core` ih izbjegavamo vlasništvom |

Redak se dodaje **u cigli u kojoj se pojam prvi put pojavi**, s referencom na datoteku.
