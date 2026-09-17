# TESTING — kako dokazujemo da radi

> Preuzeto iz Sokrat Studyja ono što vrijedi za Rust CLI bez ekrana. Playwright i vizualne brane
> dolaze tek s M2 i dobit će svoj odjeljak tada.

## 1 · Tri vrste testova

| vrsta | gdje | što tvrdi | ulaz |
|---|---|---|---|
| **jedinični (core)** | `crates/sokratis-core/src/**` uz kod (`#[cfg(test)]`) i `tests/` | parser vraća točno ove strukture; metrika daje točno ovu brojku; pravilo daje točno ovaj signal s dokazom | **tekst fixture** — nikad živi git |
| **integracijski (io)** | `crates/sokratis-io/tests/` | git-proces, radna stabla, profil, ručni podaci | **privremeni repo** stvoren u testu |
| **snapshot (cli)** | `crates/sokratis-cli/tests/` | JSON izlaz i izlazni kod za poznat repo | privremeni repo |

**Test-prvo** (CLAUDE.md #7): fixture → očekivano → implementacija. Rub koji prepoznaš odmah dobiva test.

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

1. **Snimi ulaz** (jednom, iz `sokratstudy.dev` na `main`, isti raspon kao tablica, od 2026-08-29):

   ```
   git log --since=2026-08-29 --reverse --format=@@%h|%at|%ct|%s --numstat > tests/fixtures/sokratstudy-2026-09-16.log
   ```

   plus kopije `docs/records/PROGRESS.md` i `docs/plan/RASPORED.md` s istog commita (`git show <sha>:<put>`).
2. **Snimi očekivano** iz lista Sažetak `RAD.xlsx` generirane 2026-09-16: tempo po danu, vrste,
   18 pokazatelja, faze — u `tests/fixtures/sokratstudy-2026-09-16.expected.json`.
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
(pouka iz Sokrat Studyja). CI (GitHub Actions) dolazi s M3, kad postoji remote.

## 5 · Što se NE testira testom

- Izgled i sučelje — M2, Playwright, vlastiti odjeljak.
- Performanse — tek kad postoji mjerenje koje kaže da je sporo; `gix` je odgovor koji čeka pitanje.
