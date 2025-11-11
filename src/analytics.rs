use crate::mood::Entry;
use std::collections::HashMap;

pub fn mood_summary(entries: &[Entry]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();
    for entry in entries {
        *summary.entry(entry.mood.clone()).or_insert(0) += 1;
    }
    summary
}
