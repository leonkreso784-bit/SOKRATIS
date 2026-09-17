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
| **imenovanje:** tipovi `PascalCase`, funkcije/polja `snake_case`, engleski | S-008; clippy to i traži |

## 2 · Dopušteni crateovi (M1) i zašto

| crate | uloga | zašto baš on |
|---|---|---|
| `serde` + `serde_json` | JSON profil, ručni podaci, `Report` | de-facto standard; derive = bez ručnog koda |
| `regex` | parseri dnevnika, plana, klasifikator | regexi se prenose 1:1 iz Python skripte |
| `thiserror` | tipizirane greške u `core`/`io` | kratke definicije enum-grešaka |
| `anyhow` | greške u `cli` | binarnoj je dovoljno „što je pošlo krivo" |
| `clap` (derive) | argumenti CLI-ja | `--json`, `--since` bez ručnog parsiranja |
| `time` ili `chrono` | datumi iz unix-vremena | **odluka pri prvoj cigli koja ih treba**; jedan, ne oba |
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
| lifetime (`'a`) | *kad se pojavi* | koliko dugo posudba vrijedi; u `core` ih izbjegavamo vlasništvom |

Redak se dodaje **u cigli u kojoj se pojam prvi put pojavi**, s referencom na datoteku.
