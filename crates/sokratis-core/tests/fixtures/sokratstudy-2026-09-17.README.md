# Fixture pariteta — Sokrat Study, main @ 090bd0c11f9ceafb064b2bf3f5dd06adac4a382d, snimljeno 2026-09-17

- `.log` = `git log main --since=2026-08-02 --reverse --date=format:%Y-%m-%d --format=@@%h|%at|%ct|%ad|%cd|%s --numstat`
  (od 2026-08-02 jer se zatvorene faze BROJE iz commita; metrike filtriraju `commit_date >= 2026-08-29`).
- `.PROGRESS.md` / `.RASPORED.md` = `git show main:<put>` s istog commita.
- `.expected.json` = list Sažetak knjige koju je `rad-xlsx.py` generirao nad ISTIM stanjem, bez ručnih overridea
  (svjež IZLAZ → nema `vrsta (ručno)`), izvučeno skriptom `extract_expected.py`.
- `hours_legacy` su sati Python-proxyja S KVAROM (S-007); test pariteta ih uspoređuje samo na danima bez
  cherry-pickova i tvrdi da su Sokratisovi sati ≥ 0.
