use crate::{Commit, DayStats, Delivery, Profile};
use std::collections::BTreeMap;
pub fn day_stats(
    commits: &[Commit],
    deliveries: &[Delivery],
    hours: &BTreeMap<String, f64>,
    p: &Profile,
) -> Vec<DayStats> {
    let _ = (commits, deliveries, hours, p);
    todo!("cigla M1/9")
}
