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
