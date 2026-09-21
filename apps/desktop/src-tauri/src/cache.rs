//! ZAŠTO RUST OVAKO (cigla M2/29 — adapter keša commita)
//! `sokratis_io::CommitCache` je trait bez ovisnosti o `sokratis-store` (S-013) — `io` NE zna tko
//! pamti commite, pa je ovaj adapter JEDINO mjesto koje ih spaja. Drži `&Mutex<Store>` (ne `Store`
//! izravno) i zaključava ga PRI SVAKOM POZIVU — kratko, samo unutar `cached`/`store`, nikad dulje.
//! Zato pozivatelj (`compute` u `commands.rs`) NE SMIJE držati bravu storea dok zove
//! `Project::input_cached`: std `Mutex` nije reentrantan, drugi `lock()` iz iste niti bi se zauvijek
//! blokirao (zamka opisana u zapisniku orkestratora za ovu ciglu).
use sokratis_core::Commit;
use sokratis_io::CacheError;
use sokratis_store::Store;
use std::sync::{Mutex, MutexGuard};

/// Jedan projekt gledan kao keš: ključ retka je KRATKI SHA (`Store::cached_commits`/`put_commits`)
/// — ovaj adapter ga ne dira, samo prosljeđuje `project_id` storeu.
pub struct StoreCache<'a> {
    pub store: &'a Mutex<Store>,
    pub project_id: i64,
}

impl StoreCache<'_> {
    /// Otrovan mutex (druga nit je pukla dok je brava bila zauzeta) pretvara se u poruku PRIJE
    /// boksanja: `PoisonError` posuđuje `MutexGuard` i nije `'static`, pa ga `CacheError`
    /// (`Box<dyn Error + Send + Sync>`, implicitno `'static`) ne može spremiti izravno.
    fn lock(&self) -> Result<MutexGuard<'_, Store>, CacheError> {
        self.store
            .lock()
            .map_err(|e| CacheError::from(e.to_string()))
    }
}

impl sokratis_io::CommitCache for StoreCache<'_> {
    fn cached(&self) -> Result<Vec<Commit>, CacheError> {
        self.lock()?
            .cached_commits(self.project_id, "")
            .map_err(CacheError::from)
    }

    fn store(&self, commits: &[Commit]) -> Result<(), CacheError> {
        self.lock()?
            .put_commits(self.project_id, commits)
            .map(|_inserted| ())
            .map_err(CacheError::from)
    }
}
