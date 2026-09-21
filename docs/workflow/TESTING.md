# TESTING — kako dokazujemo da radi

> Preuzeto iz Sokrat Studyja ono što vrijedi za Rust CLI bez ekrana. Sučelje M2 se testira `vitest`-om
> i `svelte-check`-om (**ne** Playwrightom — spec [ARHITEKTURA_M2.md](../plan/ARHITEKTURA_M2.md) §9);
> što se od toga danas može pokrenuti piše u tablici ispod.

## 1 · Vrste testova

| vrsta | gdje | što tvrdi | ulaz |
|---|---|---|---|
| **jedinični (core)** | `crates/sokratis-core/src/**` uz kod (`#[cfg(test)]`) i `tests/` | parser vraća točno ove strukture; metrika daje točno ovu brojku; pravilo daje točno ovaj signal s dokazom | **tekst fixture** — nikad živi git |
| **integracijski (io)** | `crates/sokratis-io/tests/` | git-proces, radna stabla, profil, ručni podaci | **privremeni repo** stvoren u testu |
| **izlazni kodovi (cli)** | `crates/sokratis-cli/tests/cli.rs` | ugovor prema preflightu: **0** (nema signala) i **2** (Alert) nad repoom s poznatom poviješću, **3** za pogrešnu uporabu i za putanju koja nije repozitorij, **0** za `--help`/`--version` | **privremeni repo** (`tests/common/mod.rs`) |
| **pohrana (store)** — *od M2* | `crates/sokratis-store/src/**` (`#[cfg(test)]`) | migracije se primijene od prazne baze; registar, postavke, snimke i keš sirovih commita (M2/15–18) rade, ali nemaju pozivatelja izvan testova (`docs/architecture/ARCHITECTURE.md` §11) | **`:memory:` baza** (`Store::open_in_memory`) — bez gita i bez datoteka |
| **sučelje (TS/Svelte)** — *od M2* | `apps/desktop/tests/**`, uz komponente | oblikovanje brojki, i18n ključevi, kontrast tokena, grafovi s praznim ulazom | `npm run check` u `apps/desktop`: danas `svelte-check` + `vitest`; brane za i18n i kontrast dolaze s ciglama M2/20–21 |

**Test-prvo** (CLAUDE.md #7): fixture → očekivano → implementacija. Rub koji prepoznaš odmah dobiva test.

**Snapshot `Report`-a** je ugovor prema sučelju (S-022): `insta` je **dev-ovisnost `core`-a od
`M2/1a`**, a sam test postoji od cigle M2/2 (2026-09-18) —
`crates/sokratis-core/tests/snapshot.rs` + `tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap`
(467 redaka, 15 ključeva na vrhu). Od sada svaka promjena oblika JSON-a mora biti **namjerna**:
`INSTA_UPDATE=always cargo test -p sokratis-core --test snapshot` prepiše snimku, `git diff` pokaže
točno što se promijenilo, a rečenica u commitu kaže koje polje i zašto. Provjeri prije `git add` da
nije ostao `.snap.new` (insta ga ostavi kad snimka još ne postoji ili se pending-diff nije razriješio).

## 2 · Fixture-politika

- **Core testira tekst.** `git log` izlaz se snimi u `tests/fixtures/*.log` jednom, tekstualno, i
  više se ne mijenja. Isto za `PROGRESS.md` i `RASPORED.md` snimke. Test koji ovisi o živom repou
  nije test nego lutrija.
- **Fixture je mala i imenovana po onome što dokazuje:** `hours-cherry-pick.log`, ne `test1.log`.
- **Privremeni repo za io:** `git init` u `tempdir`, commiti s fiksnim vremenima:

  ```
  GIT_AUTHOR_DATE="2026-09-06T05:16:40+02:00" GIT_COMMITTER_DATE="2026-09-08T21:19:43+02:00" git commit -m "..."
  ```

  Tako se cherry-pick i rebase reproduciraju deterministički.

## 3 · Test pariteta s `RAD.xlsx` (izlazni uvjet M1)

Dokazuje da Sokratis daje **iste brojke** kao Python skripta za isti ulaz, i da su **jedina**
razlika sati (ispravak S-007).

1. **Snimi ulaz** (jednom, iz `sokratstudy.dev` na `main`; od 2026-08-02 jer se zatvorene faze broje iz
   commita, a metrike filtriraju `commit_date >= 2026-08-29`):

   ```
   git log main --since="2026-08-02 00:00" --reverse --date=format:%Y-%m-%d --format=@@%h|%at|%ct|%ad|%cd|%s --numstat > tests/fixtures/sokratstudy-2026-09-17.log
   ```

   ⚠️ ` 00:00` je obavezan (S-011): bez sata `git log --since` uzima trenutno doba dana, pa bi
   ponovna snimka pomaknula lijevi rub i prvu zatvorenu fazu. Zašto je snimka takva kakva je i što se
   promijenilo prema staroj: `crates/sokratis-core/tests/fixtures/sokratstudy-2026-09-17.README.md`.

   plus `git show main:docs/records/PROGRESS.md` i `git show main:docs/plan/RASPORED.md` s istog commita
   (SHA u `.sha` datoteci). Točan postupak: plan M1, cigla T2.
2. **Snimi očekivano** iz lista Sažetak knjige koju `rad-xlsx.py` generira nad ISTIM stanjem (kopija skripte
   sa svježim izlazom, bez ručnih overridea): tempo po danu, vrste, 18 pokazatelja, faze — u
   `tests/fixtures/sokratstudy-2026-09-17.expected.json`.
3. **Test tvrdi:** sve jednako **osim** `hours` po danu i ukupno; za sate tvrdi novu vrijednost i
   u komentaru navodi staru (−144,1 h) i razlog.

⚠️ Fixture se snima s **točno onog commita** s kojeg je tablica generirana; inače paritet uspoređuje
dva različita svijeta i pada iz krivog razloga.

## 4 · Brane prije commita

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Cigla koja dira `apps/desktop` (tokovi SUČELJE i DESKTOP) uz to vrti **`npm run check`** u
`apps/desktop` — `npm install` je odrađen 2026-09-18 (`M2/1b`), `node_modules` postoji i brana je
zelena (svelte-check 0 grešaka + vitest). Rust-tokovi u svojim stablima vrte brane **bez** desktop
cratea; razlog i naredba: [`AGENTI.md`](./AGENTI.md) §5.

Crveno ne ide u commit. **Izlazni kod 1 nije dokaz da je pao test koji testiraš** — čita se poruka
(pouka iz Sokrat Studyja). CI (GitHub Actions) dolazi s M3, kad postoji remote. Koliko testova ima
danas piše [`../records/CHANGELOG.md`](../records/CHANGELOG.md) — brojka prepisana ovamo bi ostarila.

## 5 · Što se NE testira testom

- **Ljuska i izgled** — `sokratis-desktop` po S-013 nema logike, pa nema ni jediničnih testova: splash
  do kraja i preskočen, `prefers-reduced-motion`, tray, autostart, druga instanca i obavijest na Alert
  idu kao **ručna lista provjere sa snimkama** u izvještaju cigle (spec M2 §9).
- **Performanse** — danas se ne testiraju; M2/11 prvo **mjeri** (brojač git-procesa po izvještaju) i
  tek to mjerenje nosi brojku u test; `gix` je odgovor koji i dalje čeka pitanje.
