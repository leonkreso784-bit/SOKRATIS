# Sokratis — Dokumentacija

**Ovo je jedini ulaz.** Sve ostalo je u mapama ispod, složeno **po ulozi dokumenta**, ne po temi.
Model je preuzet iz Sokrat Studyja; uzeto je samo ono što ovom projektu treba.

> **Brzi kontekst za sesiju:** [`../CLAUDE.md`](../CLAUDE.md) — auto-učitava se, sažima stack, pravila i trenutno stanje.

---

## Kako je ovo složeno

| mapa | uloga | mijenja se |
|---|---|---|
| **`product/`** | **ŠTO** gradimo — definicija + kriteriji prihvaćanja | rijetko, uz odluku |
| **`plan/`** | **ŠTO SADA** — točno **jedan** aktivni spec + roadmap | stalno |
| **`workflow/`** | **KAKO RADIMO** — testiranje, Rust-konvencije i pojmovnik | povremeno |
| **`records/`** | **POVIJEST** — dnevnik, changelog, odluke, backlog | stalno |
| **`superpowers/plans/`** | planovi implementacije (cigla po cigla) za aktivni spec | po milestoneu |

**Što još NE postoji i kad nastaje:** `architecture/` kad M1 isporuči kod (spec seli u `archive/`,
izgrađeno opisuje `ARCHITECTURE.md`) · `records/BUGS.md` s prvim bugom · `records/HISTORY.md` s prvim
zatvorenim milestoneom · `archive/` s prvim ispunjenim specom · `ideas/` s prvom idejom koja je prevelika za redak u backlogu.

### Gdje što ide — **jedna činjenica, jedno mjesto** (S-010)

| vrsta znanja | JEDINO mjesto | ostali |
|---|---|---|
| **što sustav radi** | **kod + testovi** | dokument to samo *opisuje*, nikad ne *definira* |
| **zašto je tako** | `records/DECISIONS.md` (S-xxx) | ostali linkaju odluku |
| **što vrijedi sad** | `../CLAUDE.md` + `product/` + aktivni spec | **nikad dnevnik** |
| **što se kad dogodilo** | `records/PROGRESS.md` (sesija) · `records/CHANGELOG.md` (isporuka) | `CLAUDE.md` ih ne ponavlja |
| **što nije riješeno** | `records/BACKLOG.md` | plan ne nosi tuđe stavke |
| **što Rust-pojam znači** | `workflow/RUST.md` (pojmovnik) | zaglavlje cigle kaže samo *zašto ovdje* |

**Duplikat se briše, ne sinkronizira.** Rub koji prepoznaš isti čas dobiva test.

### Pravila

1. **Jedan aktivni plan.** `plan/` ima točno jedan spec. Ispunjen → `archive/` isti dan, s datumom.
2. **`product/` nije dnevnik.** Svaka mogućnost ima kriterij prihvaćanja oblika *„gotovo kad korisnik može ‹X›"*.
3. **`records/` nije izvor istine.** Povijest objašnjava zašto, ne što vrijedi sad.
4. **Svaki `.md` je naveden ovdje.** Dokument koji nije u indeksu je duh. (Sokratis će ovo sam mjeriti — dogfooding.)

---

## `product/` — što gradimo

| Dokument | Svrha |
|---|---|
| [PRD.md](./product/PRD.md) | Što gradimo, za koga, opseg po milestoneima, ne-ciljevi, rječnik |

## `plan/` — što sada

| Dokument | Svrha |
|---|---|
| [ARHITEKTURA_M1.md](./plan/ARHITEKTURA_M1.md) | 🟩 **aktivni spec** — arhitektura cijelog sustava + precizan opseg Milestonea 1 |
| [ROADMAP.md](./plan/ROADMAP.md) | Milestonei M0–M3, status, što je sljedeće |

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

## `superpowers/plans/` — planovi implementacije

Nastaju iz aktivnog speca, jedan po milestoneu.

| Dokument | Svrha |
|---|---|
| [2026-09-17-m1-jezgra-i-cli.md](./superpowers/plans/2026-09-17-m1-jezgra-i-cli.md) | **Plan M1**: 22 cigle u 8 tokova (kostur · fixture · parse · metrike · docs+pravila · io · cli · integracija) s testovima i kodom po koraku; vlasništvo datoteka po toku |
