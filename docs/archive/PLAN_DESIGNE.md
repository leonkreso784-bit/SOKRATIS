# PLAN_DESIGNE — zapis namjere: izgled, dodaci i put do verzije 1.0.0

> **Status:** ✅ ISPUNJEN 2026-09-21 — **referenca, ne izvor istine.** Leonov zapis namjere je istog dana
> pretvoren u rez za 1.0.0: odluke **S-024…S-031** ([DECISIONS.md](../records/DECISIONS.md)), dopuna aktivnog
> speca ([ARHITEKTURA_M2.md §13](./ARHITEKTURA_M2.md)) i stavke za drugu verziju i kasnije
> ([BACKLOG.md](../records/BACKLOG.md)). Tekst ispod je ostavljen kako ga je Leon zapisao.

## Odakle ovo

Leon je 2026-09-20 prvi put vidio sučelje uživo (okvir, Pregled, Tempo, Vrste rada, Pokazatelji, Faze —
na mock-podacima iz snapshota). Ocjena: smjer je dobar, ali **na izgledu i dodacima treba još dosta rada**,
a na **dokumentaciju treba pripaziti** jer je u kratkom vremenu izgrađeno puno. Popravak dokumentacije i
detalji NISU dio ovog zapisa — to je posao sljedeće sesije.

## 1 · Izgled i ponašanje (želje)

- **Animacija pokretanja** se mora dovršiti do kraja (splash postoji; još nije „gotova stvar").
- **Nakon animacije grafovi se otvaraju animirano** — ne pojave se odjednom.
- **Svaki prelazak na drugi pogled animirano učitava** rezultat ili graf.
- **Klik na graf, broj ili znamenku otvara karticu s objašnjenjem** — o čemu podatak govori, kako je
  izračunat, što znači (svaka brojka s dokazom, isto načelo kao signali).
- **Biranje teme seli se u Postavke**, zajedno s **jezikom** — gornja traka ostaje čista.
- **Postavke postaju pravo mjesto**: uz temu i jezik dobivaju još opcija da aplikacija izgleda
  profesionalnije i da korisnik može **dodavati svoje stvari** (vlastite dodatke koji mu koriste).

## 2 · Sadržaj (želje)

- **Još korisnih statističkih podataka i grafova** povrh postojećih 18 pokazatelja.
- Kasnije, zasebnim planom: prilagoditi sve tako da **nije samo za git** i da radi i **za timove**, ne samo
  za jednog autora.

## 3 · Rez: gdje završava prva verzija

Ovo je već sada jako puno za projekt koji Leon gradi **za sebe**. Zato je prvi korak sljedeće sesije
**plan koji kaže do koje cigle radimo da zatvorimo verziju 1.0.0** — sve iznad te crte ide u drugu verziju.
Bez tog reza se prva verzija nikad ne zatvara.

## 4 · Kako se verzije koriste (ultimativni test)

1. **1.0.0 se završi i odmah koristi** za ono zbog čega je sve i krenulo — zamjenu za `RAD.xlsx`.
2. **Druga verzija se gradi dok 1.0.0 nadgleda njezinu izgradnju** (Sokratis mjeri Sokratis).
3. **Istodobno 1.0.0 prati i Sokrat Study** — dakle dva zasebna projekta odjednom, možda i treći.
4. Iz te stvarne uporabe dolazi bolji pregled što se **stvarno** želi graditi dalje.

Dugoročno: Leon ima više ideja koje želi razviti jer ih smatra korisnima — moguće i kao proizvod koji se
može prodati. To je razlog da se prva verzija zatvori čisto, a ne da raste bez kraja.

## Što ovaj zapis NIJE

Nije popis cigli, nije odluka o arhitekturi, nije obećanje opsega. Sljedeća sesija ga pretvara u:
(1) rez za 1.0.0, (2) odluke u `DECISIONS.md`, (3) spec/plan po uobičajenom putu (brainstorming → spec →
plan cigli). Do tada se po njemu ništa ne gradi.
