//! ZAŠTO RUST OVAKO (cigla M2/18 — keš sirovih commita po SHA)
//! `put_commits` umeće mnogo redaka u JEDNOJ transakciji s pripremljenom `INSERT OR IGNORE`
//! izjavom (S-014, isti obrazac kao M2/15–M2/17): 2000 commita bez transakcije je 2000 fsync-ova.
//! Broj NOVIH redaka je zbroj povratne vrijednosti `execute` po retku (1 umetnut, 0 ignoriran jer
//! SHA već postoji) — SQL sam broji, kod ne pogađa. `files_json` čuva `Vec<FileChange>` kao JSON
//! jer SQLite nema ugniježđene tipove; parsira se IZVAN `query_map` zatvarača (koji smije vratiti
//! samo `rusqlite::Result`) da pokvaren JSON ostane tipizirana `StoreError::Json`, ne panika.
use crate::{Store, StoreError};
use rusqlite::params;
use sokratis_core::{Commit, FileChange};

impl Store {
    /// Upisuje sirove commite u keš. `INSERT OR IGNORE` znači da POSTOJEĆI (project_id, sha) par
    /// ostaje netaknut — SHA je adresa sadržaja, zapis se nikad ne mijenja (brif). Vraća broj NOVO
    /// umetnutih redaka, ne broj poslanih commita. Nepostojeći projekt je `NoSuchProject` PRIJE
    /// otvaranja transakcije (isti obrazac kao `save_snapshot`/`set_worktrees`).
    pub fn put_commits(&self, id: i64, commits: &[Commit]) -> Result<usize, StoreError> {
        self.project(id)?;
        let tx = self.conn.unchecked_transaction()?;
        let mut inserted = 0usize;
        {
            let mut stmt = tx.prepare(
                "INSERT OR IGNORE INTO commit_cache
                 (project_id, sha, author_time, commit_time, date, commit_date, subject, files_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            )?;
            for commit in commits {
                let files_json = serde_json::to_string(&commit.files)?;
                inserted += stmt.execute(params![
                    id,
                    commit.sha,
                    commit.author_time,
                    commit.commit_time,
                    commit.date,
                    commit.commit_date,
                    commit.subject,
                    files_json,
                ])?;
            }
        }
        tx.commit()?;
        Ok(inserted)
    }

    /// Keširani commiti s `commit_date >= since` (git po `commit_date` filtrira `--since`),
    /// poredano po `author_time` (S-007: jezgra zbraja sate po poretku commita) — SEKUNDARNO po
    /// `sha` da poredak ostane STABILAN kad dva commita dijele isti `author_time` (SQLite ga inače
    /// ne jamči bez eksplicitnog ORDER BY na jedinstvenom stupcu). Prazan `since` vraća SVE — prazan
    /// string je leksikografski manji ili jednak svakom datumu. Nepostojeći projekt vraća prazan
    /// popis, ne grešku (čitanje, ne pisanje, kao `worktrees`/`project_setting`).
    pub fn cached_commits(&self, id: i64, since: &str) -> Result<Vec<Commit>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT sha, author_time, commit_time, date, commit_date, subject, files_json
             FROM commit_cache
             WHERE project_id = ?1 AND commit_date >= ?2
             ORDER BY author_time, sha",
        )?;
        let rows = stmt.query_map(params![id, since], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
            ))
        })?;
        let mut commits = Vec::new();
        for row in rows {
            let (sha, author_time, commit_time, date, commit_date, subject, files_json) = row?;
            let files: Vec<FileChange> = serde_json::from_str(&files_json)?;
            commits.push(Commit {
                sha,
                author_time,
                commit_time,
                date,
                commit_date,
                subject,
                files,
            });
        }
        Ok(commits)
    }

    /// Najveći `commit_date` u kešu tog projekta — budući potrošač (`io`) njime gradi
    /// `git log --since=<ovo>` za inkrementalno dohvaćanje. `None` znači prazan keš (ili nepostojeći
    /// projekt — `MAX` nad praznim skupom je `NULL`, čitanje pa opet ne diže grešku).
    pub fn newest_cached_commit_date(&self, id: i64) -> Result<Option<String>, StoreError> {
        let newest: Option<String> = self.conn.query_row(
            "SELECT MAX(commit_date) FROM commit_cache WHERE project_id = ?1",
            [id],
            |r| r.get(0),
        )?;
        Ok(newest)
    }
}

#[cfg(test)]
mod tests {
    // Ovaj test NAMJERNO živi ovdje, ne u `tests/cache.rs`: mora ručno pokvariti `files_json` u
    // bazi da dokaže da čitanje ne panicira nego vraća `StoreError::Json`, a integracijski test
    // vidi samo javni API cratea, bez pristupa `self.conn` (`pub(crate)`, isti razlog kao u
    // dokumentaciji `tests/registry.rs` uz cascade-test).
    use crate::{Store, StoreError};
    use std::path::Path;

    #[test]
    fn corrupted_files_json_is_typed_error_not_panic() {
        let s = Store::open_in_memory().unwrap();
        let p = s
            .add_project("SS", Path::new(r"C:\ss"), Path::new(r"C:\ss\.git"), 1)
            .unwrap();
        s.conn
            .execute(
                "INSERT INTO commit_cache
                 (project_id, sha, author_time, commit_time, date, commit_date, subject, files_json)
                 VALUES (?1, 'zzz', 1, 1, '2026-09-01', '2026-09-01', 'x', 'ovo nije json')",
                [p.id],
            )
            .unwrap();
        match s.cached_commits(p.id, "") {
            Err(StoreError::Json(_)) => {}
            other => panic!("{other:?}"),
        }
    }
}
