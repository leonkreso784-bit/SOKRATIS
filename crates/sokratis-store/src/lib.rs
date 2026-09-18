//! ZAŠTO RUST OVAKO (cigla M2/1 — kostur cratea `sokratis-store`)
//! Pohrana je SVOJ crate (S-013): ovisi o `sokratis-core` (tipovi), NE o `sokratis-io` — pa se
//! testira nad `:memory:` bazom bez gita. Ono što drži: istina koju git ne zna (registar,
//! postavke) i pogodnost koja se izvodi iznova (snimke, keš) — S-014.
pub mod error;
pub mod store;
pub use error::StoreError;
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
