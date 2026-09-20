//! ZAŠTO RUST OVAKO (cigla M2/16 — postavke, globalne i po projektu)
//! `INSERT … ON CONFLICT DO UPDATE` (SQLite "upsert") u jednom SQL-pozivu radi ono što bi inače
//! bilo "SELECT pa INSERT-ili-UPDATE" — bez utrke između čitanja i pisanja. Čitanje ide preko
//! `query_row(...).optional()`: red možda ne postoji (postavka nikad nije zapisana), a to NIJE
//! greška nego `None`. Pisanje postavke po projektu prvo poziva `self.project(id)?` (isti obrazac
//! kao `rename_project`/`touch_project`) da nepostojeći projekt vrati tipiziranu `NoSuchProject`,
//! a ne sirovu grešku stranog ključa iz SQLite-a.
use crate::{Store, StoreError};
use rusqlite::{OptionalExtension, params};

impl Store {
    /// Globalna postavka (tema, jezik, autostart, položaj prozora) — `None` ako nikad nije zapisana.
    pub fn setting(&self, key: &str) -> Result<Option<String>, StoreError> {
        Ok(self
            .conn
            .query_row("SELECT value FROM setting WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?)
    }

    /// Upisuje ili mijenja globalnu postavku.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT INTO setting (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    /// Postavka jednog projekta (raspon, zadnji pogled) — `None` ako nikad nije zapisana ili je
    /// projekt uklonjen (`ON DELETE CASCADE` je već odnio red).
    pub fn project_setting(&self, id: i64, key: &str) -> Result<Option<String>, StoreError> {
        Ok(self
            .conn
            .query_row(
                "SELECT value FROM project_setting WHERE project_id = ?1 AND key = ?2",
                params![id, key],
                |r| r.get(0),
            )
            .optional()?)
    }

    /// Upisuje ili mijenja postavku projekta; nepostojeći `id` je `NoSuchProject`, ne tihi uspjeh.
    pub fn set_project_setting(&self, id: i64, key: &str, value: &str) -> Result<(), StoreError> {
        self.project(id)?;
        self.conn.execute(
            "INSERT INTO project_setting (project_id, key, value) VALUES (?1, ?2, ?3)
             ON CONFLICT(project_id, key) DO UPDATE SET value = excluded.value",
            params![id, key, value],
        )?;
        Ok(())
    }
}
