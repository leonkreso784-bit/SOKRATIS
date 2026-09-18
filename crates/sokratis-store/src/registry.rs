//! ZAŠTO RUST OVAKO (cigla M2/15 — registar projekata)
//! `impl Store` u DRUGOJ datoteci: Rust dopušta više `impl` blokova istog tipa po modulima, pa
//! `store.rs` drži vezu i migracije, a svaki „posao" svoju datoteku. `query_row` + zatvarač koji
//! mapira redak u struct; `params![]` makro veže vrijednosti bez ručnog formatiranja SQL-a (nema
//! injekcije, nema quotinga). `PathBuf` ↔ `String` preko `to_string_lossy`.
use crate::{Store, StoreError};
use rusqlite::{OptionalExtension, params};
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Redak tablice `project` — jedan praćeni projekt (identitet = `git_common_dir`, S-015).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProjectRecord {
    pub id: i64,
    pub name: String,
    pub root_path: PathBuf,
    pub git_common_dir: PathBuf,
    pub added_at: i64,
    pub last_seen_at: i64,
}

/// Redak tablice `project_worktree` — jedno radno stablo projekta (grana + kad je zadnji put viđeno).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WorktreeRecord {
    pub path: PathBuf,
    pub branch: String,
    pub seen_at: i64,
}

fn row_to_project(r: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectRecord> {
    Ok(ProjectRecord {
        id: r.get(0)?,
        name: r.get(1)?,
        root_path: PathBuf::from(r.get::<_, String>(2)?),
        git_common_dir: PathBuf::from(r.get::<_, String>(3)?),
        added_at: r.get(4)?,
        last_seen_at: r.get(5)?,
    })
}
const COLS: &str = "id, name, root_path, git_common_dir, added_at, last_seen_at";

impl Store {
    /// Dodaje projekt; identitet je `git_common_dir` (S-015) pa dva radna stabla istog repozitorija
    /// dijele isti zapis. Duplikat vraća ime već upisanog projekta, ne tiho spaja.
    pub fn add_project(
        &self,
        name: &str,
        root_path: &Path,
        git_common_dir: &Path,
        now: i64,
    ) -> Result<ProjectRecord, StoreError> {
        let common = git_common_dir.to_string_lossy().to_string();
        if let Some(existing) = self
            .conn
            .query_row(
                "SELECT name FROM project WHERE git_common_dir = ?1",
                [&common],
                |r| r.get::<_, String>(0),
            )
            .optional()?
        {
            return Err(StoreError::AlreadyTracked { name: existing });
        }
        self.conn.execute(
            "INSERT INTO project (name, root_path, git_common_dir, added_at, last_seen_at) VALUES (?1, ?2, ?3, ?4, ?4)",
            params![name, root_path.to_string_lossy(), common, now],
        )?;
        self.project(self.conn.last_insert_rowid())
    }

    /// Svi projekti, po imenu (bez obzira na velika/mala slova) — za popis u sučelju.
    pub fn list_projects(&self) -> Result<Vec<ProjectRecord>, StoreError> {
        let mut st = self.conn.prepare(&format!(
            "SELECT {COLS} FROM project ORDER BY name COLLATE NOCASE"
        ))?;
        let rows = st.query_map([], row_to_project)?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    /// Jedan projekt po ID-u; `NoSuchProject` ako je uklonjen ili nikad nije postojao.
    pub fn project(&self, id: i64) -> Result<ProjectRecord, StoreError> {
        self.conn
            .query_row(
                &format!("SELECT {COLS} FROM project WHERE id = ?1"),
                [id],
                row_to_project,
            )
            .optional()?
            .ok_or(StoreError::NoSuchProject(id))
    }

    /// Preimenuje projekt (samo prikazno ime; identitet ostaje `git_common_dir`).
    pub fn rename_project(&self, id: i64, name: &str) -> Result<(), StoreError> {
        self.changed_one(
            self.conn.execute(
                "UPDATE project SET name = ?1 WHERE id = ?2",
                params![name, id],
            )?,
            id,
        )
    }

    /// Uklanja projekt; `ON DELETE CASCADE` u shemi briše i njegova radna stabla i postavke.
    pub fn remove_project(&self, id: i64) -> Result<(), StoreError> {
        self.changed_one(
            self.conn
                .execute("DELETE FROM project WHERE id = ?1", [id])?,
            id,
        )
    }

    /// Bilježi da je projekt upravo viđen (npr. pri otvaranju sučelja) — za sortiranje po nedavnosti.
    pub fn touch_project(&self, id: i64, now: i64) -> Result<(), StoreError> {
        self.changed_one(
            self.conn.execute(
                "UPDATE project SET last_seen_at = ?1 WHERE id = ?2",
                params![now, id],
            )?,
            id,
        )
    }

    /// Zamjenjuje CIJELI skup radnih stabala projekta onim što je git upravo prijavio — nema
    /// spajanja starog i novog, jer stara stabla mogu biti obrisana s diska.
    pub fn set_worktrees(
        &self,
        id: i64,
        worktrees: &[(PathBuf, String)],
        now: i64,
    ) -> Result<(), StoreError> {
        self.project(id)?;
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("DELETE FROM project_worktree WHERE project_id = ?1", [id])?;
        for (path, branch) in worktrees {
            tx.execute(
                "INSERT INTO project_worktree (project_id, path, branch, seen_at) VALUES (?1, ?2, ?3, ?4)",
                params![id, path.to_string_lossy(), branch, now],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Radna stabla projekta, po putanji.
    pub fn worktrees(&self, id: i64) -> Result<Vec<WorktreeRecord>, StoreError> {
        let mut st = self.conn.prepare("SELECT path, branch, seen_at FROM project_worktree WHERE project_id = ?1 ORDER BY path")?;
        let rows = st.query_map([id], |r| {
            Ok(WorktreeRecord {
                path: PathBuf::from(r.get::<_, String>(0)?),
                branch: r.get(1)?,
                seen_at: r.get(2)?,
            })
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn changed_one(&self, n: usize, id: i64) -> Result<(), StoreError> {
        if n == 1 {
            Ok(())
        } else {
            Err(StoreError::NoSuchProject(id))
        }
    }
}
