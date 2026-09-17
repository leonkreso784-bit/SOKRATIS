use super::Rule;
use crate::{Context, Signal};
pub struct DocsLag;
impl Rule for DocsLag {
    fn id(&self) -> &'static str {
        "docs-lag"
    }
    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let _ = ctx;
        todo!("cigla M1/14")
    }
}
