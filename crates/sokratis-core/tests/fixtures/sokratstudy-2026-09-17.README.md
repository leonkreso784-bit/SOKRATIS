# Fixture pariteta — Sokrat Study, main @ 090bd0c11f9ceafb064b2bf3f5dd06adac4a382d, snimljeno 2026-09-17

- `.log` = `git log main --since=2026-08-02 --reverse --date=format:%Y-%m-%d --format=@@%h|%at|%ct|%ad|%cd|%s --numstat`
  (od 2026-08-02 jer se zatvorene faze BROJE iz commita; metrike filtriraju `commit_date >= 2026-08-29`).
- `.PROGRESS.md` / `.RASPORED.md` = `git show main:<put>` s istog commita.
- `.expected.json` = list Sažetak knjige koju je `rad-xlsx.py` generirao nad ISTIM stanjem, bez ručnih overridea
  (svjež IZLAZ → nema `vrsta (ručno)`), izvučeno skriptom `extract_expected.py`.
- `hours_legacy` su sati Python-proxyja S KVAROM (S-007); test pariteta ih uspoređuje samo na danima bez
  cherry-pickova i tvrdi da su Sokratisovi sati ≥ 0.

## `.expected.json` je regeneriran s punim danom (M1/2b) — drugi kvar tablice

`git log --since=<goli datum>` (bez sata) ne znači ponoć toga dana — git kroz `approxidate` datumu bez
sata pridaje **trenutno doba dana** pokretanja naredbe. Izmjereno na Sokrat Studyju
(`main`, `docs/records/RAD.xlsx`, isti commit kao ova fixtura):
`git log main --since=2026-08-29` (pokrenuto navečer) vrati **183** commita, a
`git log main --since='2026-08-29 00:00'` vrati **190** — razlika su commiti s jutra 29.08. koji
upadaju u prozor prije "trenutnog sata" nekog ranijeg pokretanja.

`rad-xlsx.py` (redak 154: `git('log', '--since=' + od, …)`) šalje goli datum `OD = '2026-08-29'`
izravno u `--since`, pa **`RAD.xlsx` ovisi o dobu dana kad je skripta pokrenuta** — isti dan zna dati
drukčiji zbroj commita/redaka/isporuka ovisno o satu pokretanja (isti mehanizam vrijedi i za
`commiti_u_razdoblju` na retku 178, koji broji commite unutar raspona zatvorene faze — tamo je bug
utjecao i na `MREŽA — sanacija A–E` i `RAČUN R1`, čiji su rasponi također bili "pogođeni" satom
pokretanja u prijašnjoj snimci fixturea).

Sokratis računa **cijeli dan** (`commit_date >= since`, bez sata) — deterministički, neovisno o dobu
dana pokretanja `sokratis report`. Da referenca za paritet ima istu semantiku, ova fixtura je
regenerirana iz **kopije** `rad-xlsx.py` (`rad-xlsx-fixture.py` u scratchu, ne dira original) u kojoj
su oba poziva `git log --since=` dobila eksplicitnu ponoć (`+ ' 00:00'`, redci 154 i 178); `--until`
(redak 178) već je imao `23:59` pa je ostao. Stanje repozitorija je isto (`main` na istom SHA-u kao
`.sha`), mijenja se samo semantika `--since`.

Rezultat: **190** commita umjesto 183/185 (`indicators.commits`), `days["2026-08-29"].commits` = **10**
umjesto 5 (jutarnji commiti 29.08. koji su prije bili odsječeni), sve kumulativne/ukupne brojke i udjeli
pomaknuti u skladu s tih 5 dodatnih commita na prvom danu, plus dva zatvorena raspona faza
(`MREŽA — sanacija A–E`, `RAČUN R1`) koja su prije brojala manje commita jer im je `--since` na početku
raspona isto nosio bare-datum kvar.

Ovo je **drugi kvar tablice** koji Sokratis ispravlja (prvi je S-007, negativni sati zbog
cherry-pickova): oba dijele isti uzrok — Python skripta šalje git-u ulaz koji ovisi o kontekstu
pokretanja (redoslijed/vrijeme), a ne samo o stanju repozitorija.

## `hours_fixed` (M1/2c) — referenca za paritet sati (S-007)

`hours_legacy` je Python-proxy **s kvarom**: `sati_po_danu()` u `rad-xlsx.py` prolazi commite u
redoslijedu `git log --reverse` (commit-datum), ne autorovom datumu. Cherry-pick nosi stari autorov
datum, ali nov commit-datum — u tom redoslijedu razmak između "prethodnog" i "trenutnog" commita
ispadne negativan pa se cijeli razmak (satima, ne minutama) oduzme od dana. To je vidljivo u
`hours_legacy` za 2026-09-06 (**−195,3 h**) i 2026-09-08 (**−21,9 h**).

Sokratis (T8) sortira commite za satni proxy po `author_time` (S-007), pa se u referenci treba
vidjeti "što bi Python dao da je sortiranje ispravljeno" — ne tvrdnja "≥ 0", nego brojka za pravi
paritet. Dobiveno je u scratch-kopiji `rad-xlsx-fixture-sorted.py`
(`C:\Users\leonk\AppData\Local\Temp\sokratis-fixture\`, izvedena iz T2b-kopije
`rad-xlsx-fixture.py`), gdje je **jedina** izmjena: `commiti_od()` dodatno čita `%at` (autorov
unix-timestamp; skripta je prije čuvala samo formatirani `%ad` bez sekunde/unix-vremena), a
`sati_po_danu()` prolazi commite `sorted(commiti, key=lambda z: z['ts_autor'])` umjesto izvornog
redoslijeda. Ništa drugo u skripti nije mijenjano (izlazna datoteka je preusmjerena na
`RAD-fixture-sorted.xlsx` da se ne pregazi T2b-fixtura). Rezultat pokretanja: isti brojevi commita
(190), isporuka (105), faza (11) i vizija (21) kao T2b — mijenjaju se samo sati.

`days.<dan>.hours_fixed` i `indicators.hours_fixed` / `indicators.commits_per_hour_fixed` su upisani
iz sortirane knjige (list Sažetak → „TEMPO PO DANU" stupac „sati" i „KVALITETA I BRZINA" retci
„sati rada (git-hours proxy)" / „commita po satu (proxy)"). Provjereno: nijedan `hours_fixed` po
danu nije negativan; zbroj po danima (85,0) == `indicators.hours_fixed` (85); JSON je i dalje
bajt-za-bajt isti osim dodanih ključeva.

Od 14 dana u ovoj fixturi **10 se poklapa** s `hours_legacy` (razmak između commita tog dana nije
bio pogođen krivim redoslijedom), a **4 se razlikuju**:

| dan | hours_legacy | hours_fixed | razlog |
|---|---|---|---|
| 2026-09-06 | −195,3 | 5,9 | cherry-pick — commit-redoslijed obrnuo razmak u negativan |
| 2026-09-08 | −21,9 | 1,1 | isto |
| 2026-09-09 | 3,2 | 3,3 | susjedni dan — sesija koja prelazi ponoć drukčije zatvara razmak kad je 06.09. ispravljen |
| 2026-09-12 | 2,6 | 2,5 | isto, posljedica susjednog popravka (zaokruživanje po danu) |

Ovo je manji uzorak (14 dana od 2026-08-29) nego pun log koji su vidjeli graditelj T8 i recenzent
(33 od 37 dana poklapanje) — omjer poklapanja ovdje (10/14) nije izravno usporediv s tom brojkom,
razlika je očekivana zbog kraćeg prozora i različitog skupa dana.
