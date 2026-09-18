//! ZAŠTO RUST OVAKO (cigla M2/4 — zbroj vizija po stanju)
//! `BTreeMap<&str, u32>` broji i SORTIRA u jednom prolazu: ključevi su posuđeni iz ulaza (`&str`),
//! pa nema kopiranja, a `entry().or_insert(0)` je idiom „povećaj ili započni od nule".
use crate::{Vision, VisionTotal};
use std::collections::BTreeMap;

pub fn vision_totals(visions: &[Vision]) -> Vec<VisionTotal> {
    let mut by_state: BTreeMap<&str, u32> = BTreeMap::new();
    for v in visions {
        *by_state.entry(v.state.as_str()).or_insert(0) += 1;
    }
    by_state
        .into_iter()
        .map(|(state, count)| VisionTotal {
            state: state.to_string(),
            count,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(state: &str) -> Vision {
        Vision {
            title: "t".into(),
            source: "s".into(),
            state: state.into(),
            percent: None,
            note: String::new(),
        }
    }

    #[test]
    fn counts_per_state_sorted_by_state_name() {
        let out = vision_totals(&[v("idea"), v("done"), v("idea"), v("running")]);
        let got: Vec<(&str, u32)> = out.iter().map(|t| (t.state.as_str(), t.count)).collect();
        assert_eq!(got, vec![("done", 1), ("idea", 2), ("running", 1)]);
    }

    #[test]
    fn empty_input_gives_empty_totals() {
        assert!(vision_totals(&[]).is_empty());
    }
}
