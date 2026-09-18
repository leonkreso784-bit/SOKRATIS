//! ZAŠTO RUST OVAKO (cigla M2/1 — kostur pohrane; puni se od M2/15)
//! `Store` POSJEDUJE `rusqlite::Connection`. `include_str!` uvlači SQL migracije u binarnu datoteku
//! pri kompilaciji — nema datoteka koje bi se mogle izgubiti uz instalaciju. `pub(crate)` na `conn`
//! znači: drugi moduli OVOG cratea (registar, postavke, snimke) smiju do veze, vanjski svijet ne.
use crate::StoreError;
use rusqlite::Connection;
use std::path::Path;

const MIGRATIONS: &[(i64, &str)] = &[(1, include_str!("migrations/0001_init.sql"))];

pub struct Store {
    pub(crate) conn: Connection,
}

impl Store {
    /// Otvara (ili stvara) bazu na putanji; roditeljska mapa nastaje ako je nema.
    pub fn open(path: &Path) -> Result<Store, StoreError> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let conn = Connection::open(path).map_err(|_| StoreError::Open(path.to_path_buf()))?;
        Self::init(conn)
    }

    /// Baza u memoriji — za testove; nestaje s vezom.
    pub fn open_in_memory() -> Result<Store, StoreError> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Store, StoreError> {
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL);",
        )?;
        let current: i64 = conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |r| r.get(0),
        )?;
        for (version, sql) in MIGRATIONS {
            if *version > current {
                conn.execute_batch(sql)?;
                conn.execute(
                    "INSERT INTO schema_version (version) VALUES (?1)",
                    [version],
                )?;
            }
        }
        Ok(Store { conn })
    }

    /// Najviša primijenjena migracija (0 = prazna baza bez sheme).
    pub fn schema_version(&self) -> Result<i64, StoreError> {
        Ok(self.conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |r| r.get(0),
        )?)
    }
}
