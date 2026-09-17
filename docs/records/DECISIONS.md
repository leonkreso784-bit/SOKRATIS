# DECISIONS — odluke i zašto

> Jedna odluka = jedan zapis: **kontekst → odluka → posljedice**. Ostali dokumenti linkaju `S-xxx`,
> ne prepričavaju. Odluka koja prestane vrijediti dobiva pečat ⚰️ s datumom, ne briše se.
> Najnovija na dnu.

---

## S-001 — Rust kao jezik cijele jezgre (2026-09-17)

**Kontekst:** Leon želi nov jezik koji nije koristio (Rust ili C++). Aplikacija parsira tekst,
računa metrike, drži SQLite, gleda disk i pakira se kao desktop program.
**Odluka:** Rust, stable, MSVC target. C++ odbijen: bez standardnog upravljanja paketima, UTF-8 na
Windowsu bolan (Python skripta je već pala na cp1252), desktop ljuska bi bila Qt ili WebView2 kroz COM.
Go + Wails zabilježen kao lakša rezerva, ne odabran.
**Posljedice:** M0 = instalacija toolchaina (nema ga na stroju). Učenje je cilj → pravilo „zašto Rust
ovako" (CLAUDE.md #5) i `workflow/RUST.md`.

## S-002 — `core` bez I/O-a (2026-09-17)

**Kontekst:** ista logika treba CLI-ju, Tauriju i testovima.
**Odluka:** `sokratis-core` prima tekst i strukture, vraća strukture; ne otvara datoteke, ne zove procese.
**Posljedice:** testovi rade nad tekst-fixtureima bez gita; `io` je jedino mjesto s vanjskim svijetom;
ownership bez lifetimeova prema van.

## S-003 — git kroz proces, iza traita `GitSource` (2026-09-17)

**Kontekst:** `git2` (libgit2) slabo podržava radna stabla; `gix` još evoluira; tablica je čitala
izlaz `git` naredbe.
**Odluka:** M1 poziva `git` binarnu (`std::process::Command`) i parsira tekst. Trait ostaje.
**Posljedice:** paritet je doslovan; nema nativnog builda; `git` mora biti na PATH-u (greška, izlaz 3).
`gix` čeka mjerenje koje kaže da je sporo (BACKLOG).

## S-004 — ručni podaci u `.sokratis/` u repou projekta (2026-09-17)

**Kontekst:** tablica je imala dva ručna podatka (override vrste po SHA, vizije) i putovala kroz git.
**Odluka:** `<repo>/.sokratis/overrides.json` i `visions.json`; profil isto ondje. Keš i snimke lokalno
(M2, SQLite), nikad u repou.
**Posljedice:** ručni podaci su verzionirani i prenosivi; drugi korisnik Sokratisa dobiva isto ponašanje
kloniranjem. Mapa je konvencija koju profil ne može preseliti.

## S-005 — zadano = konvencije Sokrat Studyja; profil pregazi (2026-09-17)

**Kontekst:** prvi korisnik je Sokrat Study; drugi projekti imaju drukčije dnevnike i planove.
**Odluka:** svi regexi, putanje i pragovi imaju zadane vrijednosti jednake Sokrat Studyju; `profile.json`
mijenja pojedina polja; nepoznato polje = greška.
**Posljedice:** Sokrat Study radi bez konfiguracije; tuđi projekt piše profil (M3 dokumentira format).

## S-006 — sučelje web (Svelte 5 + Tailwind v4 + tokeni), ne Rust GUI (2026-09-17)

**Kontekst:** cilj je isti izgled kao Sokrat Study; tokeni i četiri teme već postoje kao CSS.
Sokrat Study je framework odbio (ADR-028) zbog Service Workera i nepromjenjivog keša — toga u
desktop aplikaciji nema, a nadzorna ploča ima puno stanja.
**Odluka:** Tauri 2 s web sučeljem u Svelte 5; `tokens.css` se prenosi. Dioxus/Slint/egui odbijeni.
**Posljedice:** jedini dio izvan Rusta je sučelje; M2 spec bira Tailwind kroz Vite plugin ili CLI.

## S-007 — sati po `author_time`, sortirano uzlazno (2026-09-17)

**Kontekst:** tablica računa razmak po datumu autora, a redoslijed uzima iz git loga (datum commita);
cherry-pick/rebase daju negativan razmak (`5233a0a`, `5da7119`, `1e2d157` → −144,1 h).
**Odluka:** commiti se sortiraju po `author_time` prije proxyja; pragovi 2 h / 0,5 h ostaju.
**Posljedice:** paritet s tablicom se u satima **namjerno** ne održava; test to tvrdi izričito.

## S-008 — engleski identifikatori u jezgri, natpisi u sučelju (2026-09-17)

**Kontekst:** projekt se objavljuje; Leon piše hrvatski.
**Odluka:** kod, JSON ključevi i vrijednosti enum-a su engleski (`work_kind = "debugging"`); hrvatske i
engleske natpise daje sučelje/CLI tablica.
**Posljedice:** jezgra je objavljiva bez prijevoda; HR/EN je posao sučelja od prvog dana; docs ostaju hrvatski.

## S-009 — SQLite tek u M2 (2026-09-17)

**Kontekst:** M1 je CLI koji sve izvodi iz gita na zahtjev; povijest ocjena treba tek ekranu.
**Odluka:** M1 bez pohrane osim `.sokratis/` datoteka; M2 uvodi SQLite za snimke i keš.
**Posljedice:** M1 je manji; „git je izvor istine" se dokazuje prije nego se uvede keš koji bi to mogao zamagliti.

## S-010 — jedna činjenica, jedno mjesto (2026-09-17)

**Kontekst:** preuzeto iz Sokrat Studyja (ADR-027): ista cigla pisana u četiri dokumenta pa usklađivana danima.
**Odluka:** što sustav radi = kod + testovi; zašto = ovdje; što vrijedi sad = `CLAUDE.md` + `product/` +
aktivni spec; što se dogodilo = `records/`. Duplikat se briše, ne sinkronizira; brojka u prozi pokazuje na izvor.
**Posljedice:** spec ne nosi odluke nego linka; `CLAUDE.md` ne nosi povijest; Sokratis to sam mjeri (dogfooding).
