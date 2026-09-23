# Leonovi nalazi nad instaliranom „1.0.0-pre" (2026-09-23) — što mora u brainstorming prije 1.0.0

> **Uloga dokumenta:** zapis Leonovih zahtjeva za promjenu, riječ po riječ gdje su kratke, s uzrokom koji je orkestrator
> provjerio u kodu i gitu ISTE večeri. **Nije spec ni plan** — sljedeća sesija ga uzima kao ulaz u brainstorming
> (jedno pitanje odjednom) i iz njega nastaju odluke u `records/DECISIONS.md` i cigle u planu. Dio nalaza su kvarovi
> (popravak u krugu popravaka S5), dio su promjene odluka (S-005, S-019, S-020, S-018), a dio je nov opseg.
> Leonova ocjena: **„sve ovo je očito problem arhitekture"** — traži velike promjene, ne poliranje.

## 1. „Nemam informacija uopće od 13. 9. — ZAŠTO?"  ← najveći problem

**Leon:** radi na projektu (Sokrat Study) neprekidno, a Sokratis pokazuje zadnje datume 13. 9.

**Uzrok (provjeren):** Sokratis računa metrike **samo nad zadanom granom** (`main`) — to je namjerna odluka iz M1 radi
pariteta s `RAD.xlsx` (S-005; `ARCHITECTURE.md`: „grane izvan zadane ulaze samo u signale"). Zadnji commit na `main`-u
Sokrat Studyja je **2026-09-13** (`090bd0c`). Leon od tada radi u **granama u stablima** — `feat/f6-mcp` (23. 9.),
`feat/f3-dvojezicnost` (17. 9.), `feat/f2-mail`/`f2-zid` (15. 9.), `f2-tema-racun`/`f2-zid-radionica`/`f2-slike` (14. 9.)
— šest stabala. Sav taj rad Sokratis vidi **samo kao signal „nespojene grane"**, ne kao commite, sate, dane, isporuke.
CLI potvrđuje: `sokratis report sokratstudy.dev --json` → `branch: main`, `zadnji dan: 2026-09-13`, 190 commita.

**Što to znači:** za Leonov način rada (dugotrajne grane po feature-u, spajanje rijetko) mjerenje „samo `main`" je
**krivo mjerilo** — pokazuje mrtvo more dok on radi najviše. Treba odluka:
- (a) metrike nad **svim granama** (`git log --all` ili unija stabala), s dedupliciranjem po SHA i označavanjem grane u Dnevniku;
- (b) metrike nad **granama koje su u stablima** (`git worktree list`) — to je „ono na čemu radim";
- (c) ostaviti `main` + prikazati **jasno upozorenje** „X commita u N grana nije u ovoj brojci" (najmanje, ali ne rješava frustraciju).
Paritet s `RAD.xlsx` (test snimke) tada mijenja značenje — ili ostaje kao „zadana grana" način, a novi način je zadani.
Ovo je promjena u **jezgri + io + profilu** (`GitSource::log(branch, …)`, `Project::input`, `profile.default_branch`) i u
ugovoru `Report` (grana po commitu) — zato je „problem arhitekture", ne cigla sučelja.

## 2. „Jako malo grafova — samo tri, a treba ih biti puno više; ružni su, ne pišu datumi, JAKO nepregledno"

**Stanje:** četiri vrste grafa (stupci · linija · prsten · sparkline) u tri pogleda (Tempo: stupci + kumulativna linija;
Vrste rada: prsten; Pokazatelji: 18 sparklinea). Osi: `Bars` ima samo Y-oznake (brojke), **X-os bez datuma** (datumi su
samo u `<title>` tooltipu); `Line` nema nikakvih oznaka; `Sparkline` nema osi. Grafovi su namjerno minimalni (S-018
„vlastiti SVG", 600×160 viewBox), bez legende osim u Vrstama rada.

**Što Leon traži:** puno više grafova, čitljive (datumi na X-osi, mreža, legenda, jasne boje), pregledne. To je
**S-018 revizija** (vlastiti SVG vs. biblioteka grafova — pravilo #6 o ovisnostima, ali čitljivost je zahtjev), plus
**popis grafova** koje želi (za brainstorming: po danu/tjednu/mjesecu, po grani, sati vs. commiti, faze kao Gantt, docs
ocjena kroz vrijeme, signali kroz vrijeme, usporedba projekata …). Trend-podaci za „kroz vrijeme" već postoje u
`store` (dnevne snimke), ali samo za pokazatelje.

## 3. „Kad stisnem X, aplikacija se stvarno cijela zatvori — prije toga upit; kad je opet otvorim, uvijek animacija na početku"

**Stanje:** X **sakriva u tray** (S-020, plan T32), a animacija se pokazuje **jednom po pokretanju procesa** (S-019) —
zato nakon X-a i ponovnog klika nema ni animacije ni „novog" pokretanja (proces nikad nije izašao). Danas se stvarno
izlazi samo kroz tray → **Izađi**. Upravo je to zbunilo Leona (drugi klik na Sokratis dao je samo splash druge instance
koja se odmah ugasi — poznati rub S2 „druga instanca").

**Promjena odluke S-020 → S-020b:** X = **upit** („Zatvoriti Sokratis? Nadzor projekata staje dok ga ponovno ne
pokreneš." — gumbi Zatvori / Odustani / (možda) Sakrij u traku) → potpun izlaz procesa. Time S-019 automatski daje
animaciju pri svakom pokretanju. Otvorena pitanja za brainstorming: treba li tray uopće ostati; autostart tada znači
„pokreni minimizirano" ili ne.

## 4. „Jako mi ide na živce što se PowerShell stalno otvara kad nešto novo biram"

**Uzrok (provjeren, KVAR):** `crates/sokratis-io/src/git.rs:175` i `:196` pokreću `git` kroz `std::process::Command`
**bez `CREATE_NO_WINDOW`**; instalirana aplikacija je GUI (`windows_subsystem = "windows"`, `main.rs:5`), pa Windows za
svaki `git` proces nakratko otvori konzolni prozor — pri svakoj promjeni raspona, osvježenju, watcheru. Nije PowerShell
nego konzola `git.exe`. **Popravak je jedna cigla u `io`** (`#[cfg(windows)] cmd.creation_flags(0x0800_0000)` uz
`std::os::windows::process::CommandExt`, s testom da se zastavica postavlja) — **ide u krug popravaka S5 prije 1.0.0.**
Isto vrijedi za svaki budući `Command` (pravilo u `RUST.md`).

## 5. „Mogućnost da stavim koliko god želim projekata i da ih pratim kako rade"

**Stanje:** registar podržava neograničen broj projekata (`store`, `add_project`), Pregled ih sve crta u karticama.
Leonov dojam da je ograničeno vjerojatno dolazi iz **birača projekta u gornjoj traci** (jedan odabran) i **jednog
izvještaja odjednom** (`app.report` samo za `currentId`). Traži se **stvarno praćenje više projekata usporedno** —
usporedba na Pregledu (BACKLOG želja od 22. 9.: „redak s omjerom na karticama"), možda više izvještaja u memoriji.

## 6. „Na Pregledu, kad stavim projekt, želim stisnuti na karticu imena projekta i da mi se tamo otvaraju grafovi i sve"

**Stanje:** klik na karticu (od T42 samo na naslov, R26) **odabere** projekt i učita izvještaj, ali pogled ostaje
**Pregled** — korisnik mora sam kliknuti Tempo/Vrste rada… u izborniku. Leon očekuje **nadzornu ploču projekta**: klik
na karticu → stranica TOG projekta sa svim grafovima i brojkama (ne devet zasebnih pogleda). To je promjena §6.1/§6.2
speca (okvir + pogledi) — spada u brainstorming s točkom 2 (koji grafovi) i točkom 5 (više projekata).

## Kako dalje (prijedlog orkestratora, Leon odlučuje)

| # | vrsta | gdje ide |
|---|---|---|
| 4 konzolni prozor | **kvar** | krug popravaka S5, prije 1.0.0 (jedna cigla u `io`) |
| 3 X zatvara + upit + animacija | promjena odluke S-019/S-020 | S5 ako Leon potvrdi tekst upita — mala cigla (`lib.rs` handler + dijalog + i18n) |
| 1 grane izvan `main`-a | **promjena arhitekture** (S-005, paritet) | brainstorming → odluka → cigle u jezgri/io/profilu; **1.0.0 ne smije izaći dok Leonov vlastiti rad nije vidljiv** — ili se 1.0.0 odgađa |
| 2 grafovi | promjena S-018 + nov opseg | brainstorming (popis grafova, osi/datumi/legenda, biblioteka da/ne) |
| 5 više projekata usporedno | nov opseg | brainstorming s 6 |
| 6 kartica → nadzorna ploča projekta | promjena speca §6 | brainstorming s 2 i 5 |

**Za sljedeću sesiju:** početi **brainstormingom nad ovim dokumentom** (jedno pitanje odjednom, kao za M2 spec), ne
izvedbom S5 kakva je bila planirana. Tek nakon odluka: što od toga ulazi u 1.0.0 (i pomiče li se izdanje), a što u M3.
Izvedba T35 (verzija 1.0.0) stoji napola u stablu `sokratis.rel` — ne dirati dok se ne odluči je li 1.0.0 još ovaj rez.
