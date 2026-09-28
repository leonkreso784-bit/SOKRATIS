# Sokratis — Dokumentacija

**Ovo je jedini ulaz.** Sve ostalo je u mapama ispod, složeno **po ulozi dokumenta**, ne po temi.
Model je preuzet iz Sokrat Studyja; uzeto je samo ono što ovom projektu treba.

> **Brzi kontekst za sesiju:** [`../CLAUDE.md`](../CLAUDE.md) — auto-učitava se, sažima stack, pravila i trenutno stanje.

---

## Kako je ovo složeno

| mapa | uloga | mijenja se |
|---|---|---|
| **`product/`** | **ŠTO** gradimo — definicija + kriteriji prihvaćanja | rijetko, uz odluku |
| **`plan/`** | **ŠTO SADA** — najviše **jedan** aktivni spec + roadmap | stalno |
| **`architecture/`** | **ŠTO JE IZGRAĐENO** — sustav kakav stoji u `crates/`, bez kronologije | kad kod pomakne granicu |
| **`workflow/`** | **KAKO RADIMO** — testiranje, Rust-konvencije i pojmovnik | povremeno |
| **`records/`** | **POVIJEST** — dnevnik, changelog, odluke, backlog | stalno |
| **`archive/`** | **ISPUNJENO** (specovi, planovi cigli, zapisi namjere) — referenca, **nikad izvor istine** | isti dan kad dokument ispuni svrhu (S-031) |

**Što još NE postoji i kad nastaje:** `superpowers/plans/` s planom cigli sljedećeg aktivnog speca (M3; planovi M1, M2 i 1.0.0 su u `archive/`) · `records/BUGS.md` s prvim bugom · `records/HISTORY.md` kad
zatvorenih milestonea bude toliko da ih `CHANGELOG.md` više ne drži pregledno (M1 je zasad cijeli u
0.1.0, pa bi drugi dokument bio duplikat — S-010) · `ideas/` s prvom idejom koja je prevelika za
redak u backlogu.

### Gdje što ide — **jedna činjenica, jedno mjesto** (S-010)

| vrsta znanja | JEDINO mjesto | ostali |
|---|---|---|
| **što sustav radi** | **kod + testovi** | dokument to samo *opisuje*, nikad ne *definira* |
| **kakav je sustav danas** | `architecture/ARCHITECTURE.md` | spec u `archive/` govori samo što je bilo zamišljeno |
| **zašto je tako** | `records/DECISIONS.md` (S-xxx) | ostali linkaju odluku |
| **što vrijedi sad** | `../CLAUDE.md` + `product/` + aktivni spec | **nikad dnevnik, nikad `archive/`** |
| **što se kad dogodilo** | `records/PROGRESS.md` (sesija) · `records/CHANGELOG.md` (isporuka) | `CLAUDE.md` ih ne ponavlja |
| **što nije riješeno** | `records/BACKLOG.md` | plan ne nosi tuđe stavke |
| **što Rust-pojam znači** | `workflow/RUST.md` (pojmovnik) | zaglavlje cigle kaže samo *zašto ovdje* |

**Duplikat se briše, ne sinkronizira.** Rub koji prepoznaš isti čas dobiva test.

### Pravila

1. **Jedan aktivni plan.** `plan/` ima najviše jedan spec. Ispunjen → `archive/` isti dan, s datumom
   i pečatom „referenca, ne izvor istine". Između milestonea `plan/` nosi samo `ROADMAP.md`.
2. **`product/` nije dnevnik.** Svaka mogućnost ima kriterij prihvaćanja oblika *„gotovo kad korisnik može ‹X›"*.
3. **`records/` nije izvor istine.** Povijest objašnjava zašto, ne što vrijedi sad.
4. **Svaki `.md` je naveden ovdje.** Dokument koji nije u indeksu je duh. (Sokratis će ovo sam mjeriti — dogfooding.)

---

## `product/` — što gradimo

| Dokument | Svrha |
|---|---|
| [PRD.md](./product/PRD.md) | Što gradimo, za koga, opseg po milestoneima, ne-ciljevi, rječnik |
| [NALAZI_LEON_2026-09-23.md](./product/NALAZI_LEON_2026-09-23.md) | Leonovi nalazi nad instaliranom „1.0.0-pre" s provjerenim uzrocima (metrike vide samo `main`, premalo/nečitki grafovi, X mora zatvoriti + animacija svaki put, konzolni prozor `git`-a, više projekata, kartica → nadzorna ploča) — **pretvoreno 2026-09-24** brainstormingom u S-032…S-037 i spec [ARHITEKTURA_1_0.md](./archive/ARHITEKTURA_1_0.md); ovdje ostaje kao izvorni zapis |

## `plan/` — što sada

**Između milestonea** (od izdanja 1.0.0, 2026-09-28): `plan/` nosi samo ROADMAP — nema aktivnog speca;
sljedeći nastaje kad M3 krene. Gdje smo i što je sljedeće kaže ROADMAP („Gdje smo“), ne ovaj indeks.

| Dokument | Svrha |
|---|---|
| [ROADMAP.md](./plan/ROADMAP.md) | 🟩 **živ** — milestonei M0–M4, status, što je sljedeće |

## `architecture/` — što je izgrađeno

| Dokument | Svrha |
|---|---|
| [ARCHITECTURE.md](./architecture/ARCHITECTURE.md) | Sustav kakav stoji u `crates/` i `apps/`: granice četiriju crateova (+ ljuska), tok podataka, **sva polja profila sa zadanim vrijednostima**, formati `.sokratis/*.json`, izlazni kodovi CLI-ja, **§11 što stoji u kodu a ne radi** |

## `workflow/` — kako radimo

| Dokument | Svrha |
|---|---|
| [TESTING.md](./workflow/TESTING.md) | Vrste testova (core · io · cli · store · sučelje), fixture-politika, paritet s RAD.xlsx, brane prije commita |
| [RUST.md](./workflow/RUST.md) | Rust-konvencije, dopušteni crateovi, pravilo „zašto Rust ovako", pojmovnik koji raste (§4 Rust · §5 TS/Svelte) |
| [AGENTI.md](./workflow/AGENTI.md) | Više agenata na više grana: uloge (orkestrator · graditelj · recenzent · čuvar dokumentacije), tokovi i stabla, protokol po cigli, spajanje, compact |

## `records/` — povijest

| Dokument | Svrha |
|---|---|
| [PROGRESS.md](./records/PROGRESS.md) | Dnevnik rada po sesijama (format isti kao u Sokrat Studyju — Sokratis ga sam parsira) |
| [CHANGELOG.md](./records/CHANGELOG.md) | Isporuke po datumu |
| [DECISIONS.md](./records/DECISIONS.md) | Odluke S-001… i zašto |
| [BACKLOG.md](./records/BACKLOG.md) | Parkiralište: što čeka, što je odbijeno i zašto |

## `archive/` — ispunjeni specovi i zapisi

**Nikad izvor istine.** Ovdje se čita zašto je nešto bilo zamišljeno; što danas stoji u kodu govori
`architecture/`, a što vrijedi sada `../CLAUDE.md`.

| Dokument | Svrha |
|---|---|
| [ARHITEKTURA_M1.md](./archive/ARHITEKTURA_M1.md) | ✅ ISPUNJEN 2026-09-17 — spec arhitekture i opsega Milestonea 1 (jezgra · io · CLI) |
| [PLAN_DESIGNE.md](./archive/PLAN_DESIGNE.md) | ✅ ISPUNJEN 2026-09-21 — Leonov zapis namjere (izgled, dodaci, put do 1.0.0); pretvoren u rez S-024…S-031, spec M2 §13 i stavke u `records/BACKLOG.md` |
| [ARHITEKTURA_M2.md](./archive/ARHITEKTURA_M2.md) | ✅ ISPUNJEN 2026-09-24 (etape 1–2 u kodu; etapa 3 „izdanje" preuzeta u `ARHITEKTURA_1_0.md`) — spec desktopa: granice (`store`, `desktop` bez logike), Tauri ugovor, SQLite shema, watcher, pogledi, teme i znak, splash, ovisnosti, testovi, §13 rez za 1.0.0 |
| [ARHITEKTURA_1_0.md](./archive/ARHITEKTURA_1_0.md) | ✅ ISPUNJEN 2026-09-28 — spec drugog reza do 1.0.0 (S-032…S-038): metrike nad svim lokalnim granama, dnevnik kao unija stabala, ploča projekta s grafovima, X = upit → izlaz bez traya, lanci nespojenih grana |
| [2026-09-17-m1-jezgra-i-cli.md](./archive/2026-09-17-m1-jezgra-i-cli.md) | ✅ IZVRŠEN — plan cigli M1: 22 cigle u 8 tokova, testovi i kod po koraku, vlasništvo datoteka po toku |
| [2026-09-18-m2-desktop.md](./archive/2026-09-18-m2-desktop.md) | ✅ IZVRŠEN — plan cigli M2 (T1–T43) iz speca `ARHITEKTURA_M2.md`, uključujući rez za 1.0.0 (T36–T43) |
| [2026-09-24-1-0-0-grane-i-ploca.md](./archive/2026-09-24-1-0-0-grane-i-ploca.md) | ✅ IZVRŠEN — plan cigli drugog reza T44–T64 u 7 tokova (DESKTOP-2 · JEZGRA-2 · IO-2 · GRAFOVI · PLOČA · LANCI · IZDANJE), iz speca `ARHITEKTURA_1_0.md` |
