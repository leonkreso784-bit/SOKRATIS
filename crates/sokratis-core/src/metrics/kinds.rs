use crate::{Commit, KindStats, Patterns, WorkKind};
use std::collections::HashMap;
pub fn effective_kind(c: &Commit, overrides: &HashMap<String, WorkKind>, p: &Patterns) -> WorkKind {
    let _ = (c, overrides, p);
    todo!("cigla M1/10")
}
pub fn kind_stats(
    commits: &[Commit],
    overrides: &HashMap<String, WorkKind>,
    p: &Patterns,
) -> Vec<KindStats> {
    let _ = (commits, overrides, p);
    todo!("cigla M1/10")
}
