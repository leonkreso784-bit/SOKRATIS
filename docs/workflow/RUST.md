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
| `serde` + `serde_json` | JSON profil, ručni podaci, `Report` | de-facto standard; derive = bez ručnog koda. U `core` je `serde_json` **dev-ovisnost** — jezgra ga koristi samo u testovima, čitanje i pisanje JSON-a rade `io` i `cli` |
| `regex` | parseri dnevnika, plana, klasifikator | regexi se prenose 1:1 iz Python skripte |
| `thiserror` | tipizirane greške u `core`/`io` | kratke definicije enum-grešaka |
| `anyhow` | greške u `cli` | binarnoj je dovoljno „što je pošlo krivo" |
| `clap` (derive) | argumenti CLI-ja | `--json`, `--since` bez ručnog parsiranja |
| `chrono` (samo `io`) | današnji lokalni datum za `ReportInput.today` | odlučeno u planu M1 (T17): lokalni datum na Windowsu bez feature-gatea; jezgra datume računa sama (`civil.rs`), bez ovisnosti |
| `tempfile` (dev) | privremeni repo u io- i cli-testovima | čišćenje bez ručnog `rm` |

Nova ovisnost = namjerna radnja: redak ovdje + obrazloženje u commitu (CLAUDE.md #6).
`Cargo.lock` se commita. **`insta` je izašao** u krugu popravaka M1: stajao je u dva manifesta bez
ijednog poziva, a neiskorištena ovisnost je pravilo #6 naopako; snapshot JSON-a čeka M2, kad se oblik
`Report`-a zaključa za Tauri (`workflow/TESTING.md` §1).

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
| `trait` i trait objekt (`Box<dyn Rule>`) | M1 (T13, `rules/mod.rs`) | ugovor koji tip ispunjava; `dyn` = poziv preko ugovora u vrijeme izvođenja, pa jedan `Vec` drži raznorodna pravila |
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
| struct-update `..Default::default()` | M1 (T12, `docs.rs` test) | preostala polja literala preuzimaju zadane vrijednosti; test postavlja samo polje koje testira |
| `saturating_sub` | M1 (T12, `docs.rs`) | oduzimanje koje se zaustavlja na granici tipa umjesto da se prelije — ocjena docs-a zato nikad ne padne ispod 0 |
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
| `Vec<&Commit>` | M1 (T8, `metrics/hours.rs`) | vektor referenci umjesto vlasništva kad metrika samo čita commite koje već drži pozivatelj |
| `BTreeMap` | M1 (T8, `metrics/hours.rs`) | mapa sortirana po ključu — korisna kad se ispisuje po danu uzlazno bez naknadnog sortiranja |
| `entry().or_insert()` / `or_default()` | M1 (T8, `metrics/hours.rs`; kasnije `metrics/days.rs`) | dohvati-ili-umetni u jednom potezu, bez dvostrukog pretraživanja mape |
| `Option::is_some_and` | M1 (T11b, `metrics/indicators.rs`; kasnije popravak C1, `parse/plan.rs`) | provjerava predikat nad sadržajem `Option` bez ručnog `match`/`unwrap` — nad `Captures::get` to je „grupa postoji **i** nije prazna" u jednom izrazu |
| `Option::filter` | M1 (T8, `metrics/hours.rs`) | zadrži `Some` samo ako sadržaj zadovoljava predikat, inače `None` |
| `BTreeMap<&str, Acc>` s posuđenim ključem | M1 (T9, `metrics/days.rs`) | ključ mape je posudba iz izvornih podataka, ne kopija — akumulator ne smije nadživjeti izvor |
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

Redak se dodaje **u cigli u kojoj se pojam prvi put pojavi**, s referencom na datoteku.
