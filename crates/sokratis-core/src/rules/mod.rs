//! ZAŠTO RUST OVAKO (cigla M1/13 — registar pravila)
//! `trait Rule` je ugovor; `Box<dyn Rule>` daje dinamički dispatch pa `default_rules()` vraća
//! popis raznorodnih pravila iza jednog tipa. Novo pravilo se dodaje ovdje jednim retkom —
//! pozivatelj (`evaluate_all`) se ne mijenja.
pub mod docs_lag;
pub mod unmerged_branches;
use crate::{Context, Signal};
pub trait Rule {
    fn id(&self) -> &'static str;
    fn evaluate(&self, ctx: &Context) -> Vec<Signal>;
}
pub fn default_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(unmerged_branches::UnmergedBranches),
        Box::new(docs_lag::DocsLag),
    ]
}
pub fn evaluate_all(rules: &[Box<dyn Rule>], ctx: &Context) -> Vec<Signal> {
    rules.iter().flat_map(|r| r.evaluate(ctx)).collect()
}
