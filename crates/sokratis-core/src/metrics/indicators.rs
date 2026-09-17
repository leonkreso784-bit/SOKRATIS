use crate::{Commit, DayStats, Delivery, Indicator, Patterns, Phase, Profile, WorkKind};
use std::collections::HashMap;
pub struct IndicatorInput<'a> {
    pub commits: &'a [Commit],
    pub deliveries: &'a [Delivery],
    pub days: &'a [DayStats],
    pub phases: &'a [Phase],
    pub overrides: &'a HashMap<String, WorkKind>,
}
pub fn indicators(input: &IndicatorInput<'_>, profile: &Profile, p: &Patterns) -> Vec<Indicator> {
    let _ = (input, profile, p);
    todo!("cigla M1/11")
}
