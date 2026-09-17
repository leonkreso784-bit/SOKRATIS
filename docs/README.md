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
| **`archive/`** | **ISPUNJENI SPECOVI** — referenca, **nikad izvor istine** | kad milestone završi |
| **`superpowers/plans/`** | planovi implementacije (cigla po cigla) za aktivni spec | po milestoneu |

**Što još NE postoji i kad nastaje:** `records/BUGS.md` s prvim bugom · `records/HISTORY.md` s prvim
zatvorenim milestoneom · `ideas/` s prvom idejom koja je prevelika za redak u backlogu.

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

## `plan/` — što sada

Spec Milestonea 1 je **ispunjen i arhiviran** ([archive/ARHITEKTURA_M1.md](./archive/ARHITEKTURA_M1.md));
spec za M2 (desktop) **tek dolazi**. Do tada `plan/` nosi samo živi roadmap.

| Dokument | Svrha |
|---|---|
| [ROADMAP.md](./plan/ROADMAP.md) | 🟩 **živ** — milestonei M0–M3, status, što je sljedeće |

## `architecture/` — što je izgrađeno

| Dokument | Svrha |
|---|---|
| [ARCHITECTURE.md](./architecture/ARCHITECTURE.md) | Sustav kakav stoji u `crates/`: granice crateova, tok podataka, **sva polja profila sa zadanim vrijednostima**, formati `.sokratis/*.json`, izlazni kodovi CLI-ja |

## `workflow/` — kako radimo

| Dokument | Svrha |
|---|---|
| [TESTING.md](./workflow/TESTING.md) | Vrste testova, fixture-politika, paritet s RAD.xlsx, brane prije commita |
| [RUST.md](./workflow/RUST.md) | Rust-konvencije, dopušteni crateovi, pravilo „zašto Rust ovako", pojmovnik koji raste |
| [AGENTI.md](./workflow/AGENTI.md) | Više agenata na više grana: uloge (orkestrator · graditelj · recenzent · čuvar dokumentacije), tokovi i stabla, protokol po cigli, spajanje, compact |

## `records/` — povijest

| Dokument | Svrha |
|---|---|
| [PROGRESS.md](./records/PROGRESS.md) | Dnevnik rada po sesijama (format isti kao u Sokrat Studyju — Sokratis ga sam parsira) |
| [CHANGELOG.md](./records/CHANGELOG.md) | Isporuke po datumu |
| [DECISIONS.md](./records/DECISIONS.md) | Odluke S-001… i zašto |
| [BACKLOG.md](./records/BACKLOG.md) | Parkiralište: što čeka, što je odbijeno i zašto |

## `archive/` — ispunjeni specovi

**Nikad izvor istine.** Ovdje se čita zašto je nešto bilo zamišljeno; što danas stoji u kodu govori
`architecture/`, a što vrijedi sada `../CLAUDE.md`.

| Dokument | Svrha |
|---|---|
| [ARHITEKTURA_M1.md](./archive/ARHITEKTURA_M1.md) | ✅ ISPUNJEN 2026-09-17 — spec arhitekture i opsega Milestonea 1 (jezgra · io · CLI) |

## `superpowers/plans/` — planovi implementacije

Nastaju iz aktivnog speca, jedan po milestoneu.

| Dokument | Svrha |
|---|---|
| [2026-09-17-m1-jezgra-i-cli.md](./superpowers/plans/2026-09-17-m1-jezgra-i-cli.md) | **Plan M1** — ✅ IZVRŠEN 2026-09-17: 22 cigle u 8 tokova (kostur · fixture · parse · metrike · docs+pravila · io · cli · integracija) s testovima i kodom po koraku; vlasništvo datoteka po toku. Zapis plana, ne izvor istine |
