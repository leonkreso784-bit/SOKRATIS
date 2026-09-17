use super::Rule;
use crate::{Context, Signal};
pub struct UnmergedBranches;
impl Rule for UnmergedBranches {
    fn id(&self) -> &'static str {
        "unmerged-branches"
    }
    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let _ = ctx;
        todo!("cigla M1/13")
    }
}
