//! ZAŠTO RUST OVAKO (cigla M2/1 — kostur cratea `sokratis-store`)
//! Pohrana je SVOJ crate (S-013): ovisi o `sokratis-core` (tipovi), NE o `sokratis-io` — pa se
//! testira nad `:memory:` bazom bez gita. Ono što drži: istina koju git ne zna (registar,
//! postavke) i pogodnost koja se izvodi iznova (snimke, keš) — S-014.
//!
//! ZAŠTO RUST OVAKO (cigla M2/15 — registar)
//! `pub mod registry` + re-export: pozivatelj (desktop, T29) piše `sokratis_store::ProjectRecord`
//! umjesto `sokratis_store::registry::ProjectRecord` — modul je organizacija koda, ne dio API-ja.
//!
//! ZAŠTO RUST OVAKO (cigla M2/16 — postavke)
//! `settings.rs` nema vlastite tipove (samo `String` ključ→vrijednost) pa nema što re-eksportirati —
//! `impl Store` u njemu je vidljiv kroz `pub use store::Store` iznad, isti modul-je-organizacija
//! princip kao kod registra.
//!
//! ZAŠTO RUST OVAKO (cigla M2/17 — snimke brojki)
//! `snapshots.rs` uvodi jedan javni tip (`TrendPoint`) i jednu slobodnu funkciju (`canonical_json`)
//! uz `impl Store` — oboje re-eksportirano istim obrascem kao registar.
//!
//! ZAŠTO RUST OVAKO (cigla M2/18 — keš sirovih commita po SHA)
//! `cache.rs` ne uvodi vlastite tipove (samo `impl Store` nad `Commit`/`FileChange` iz
//! `sokratis-core`) — isti „modul je organizacija, ne API" princip kao `settings.rs`, pa nema
//! novog re-exporta.
pub mod cache;
pub mod error;
pub mod registry;
pub mod settings;
pub mod snapshots;
pub mod store;
pub use error::StoreError;
pub use registry::{ProjectRecord, WorktreeRecord};
pub use snapshots::{TrendPoint, canonical_json};
pub use store::Store;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_store_applies_all_migrations() {
        let s = Store::open_in_memory().unwrap();
        assert_eq!(s.schema_version().unwrap(), 1);
    }
}
