# TESTING — kako dokazujemo da radi

> Preuzeto iz Sokrat Studyja ono što vrijedi za Rust CLI bez ekrana. Playwright i vizualne brane
> dolaze tek s M2 i dobit će svoj odjeljak tada.

## 1 · Tri vrste testova

| vrsta | gdje | što tvrdi | ulaz |
|---|---|---|---|
| **jedinični (core)** | `crates/sokratis-core/src/**` uz kod (`#[cfg(test)]`) i `tests/` | parser vraća točno ove strukture; metrika daje točno ovu brojku; pravilo daje točno ovaj signal s dokazom | **tekst fixture** — nikad živi git |
| **integracijski (io)** | `crates/sokratis-io/tests/` | git-proces, radna stabla, profil, ručni podaci | **privremeni repo** stvoren u testu |
| **izlazni kodovi (cli)** | `crates/sokratis-cli/tests/cli.rs` | ugovor prema preflightu: **0** (nema signala) i **2** (Alert) nad repoom s poznatom poviješću, **3** za pogrešnu uporabu i za putanju koja nije repozitorij, **0** za `--help`/`--version` | **privremeni repo** (`tests/common/mod.rs`) |

**Test-prvo** (CLAUDE.md #7): fixture → očekivano → implementacija. Rub koji prepoznaš odmah dobiva test.

**Snapshot-testova nema** i `insta` nije ovisnost: snimka oblika `Report`-a ima smisla kad se JSON
zaključa kao ugovor prema Tauriju (M2) — do tada bi zamrznula oblik koji se još mijenja.
Parkirano u [`../records/BACKLOG.md`](../records/BACKLOG.md).

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

Crveno ne ide u commit. **Izlazni kod 1 nije dokaz da je pao test koji testiraš** — čita se poruka
(pouka iz Sokrat Studyja). CI (GitHub Actions) dolazi s M3, kad postoji remote. Koliko testova ima
danas piše [`../records/CHANGELOG.md`](../records/CHANGELOG.md) (0.1.0) — brojka prepisana ovamo bi
ostarila.

## 5 · Što se NE testira testom

- Izgled i sučelje — M2, Playwright, vlastiti odjeljak.
- Performanse — tek kad postoji mjerenje koje kaže da je sporo; `gix` je odgovor koji čeka pitanje.
