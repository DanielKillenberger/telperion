//! What a stage spent: Firecrawl credits with the method they were counted
//! by, and Jev calls, summed over every run that wrote its artifact.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::adapter::Spent;
use super::canon::read_json;

/// What a stage spent over every run that wrote its artifact: Firecrawl
/// credits with the method they were counted by, and Jev calls.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Cost {
    pub runs: u32,
    pub jev_calls: u32,
    pub firecrawl_credits: u32,
    pub firecrawl_method: String,
}

impl Cost {
    /// One run's adapter spend, to sit in the header before `write` sums it
    /// with the earlier runs.
    pub fn from_spent(spent: &Spent) -> Cost {
        Cost {
            runs: 0,
            jev_calls: 0,
            firecrawl_credits: spent.credits(),
            firecrawl_method: if spent.calls == 0 {
                String::new()
            } else {
                spent.method().into()
            },
        }
    }

    /// Adds `other` into this cost; counting methods that differ are both named.
    pub fn add(&mut self, other: &Cost) {
        self.runs += other.runs;
        self.jev_calls += other.jev_calls;
        self.firecrawl_credits += other.firecrawl_credits;
        let named = self
            .firecrawl_method
            .split("; ")
            .any(|method| method == other.firecrawl_method);
        if !other.firecrawl_method.is_empty() && !named {
            if !self.firecrawl_method.is_empty() {
                self.firecrawl_method.push_str("; ");
            }
            self.firecrawl_method.push_str(&other.firecrawl_method);
        }
    }
}

/// The cost the artifact at `path` already carries, summed over its runs.
pub fn earlier_cost(path: &Path) -> Cost {
    if !path.exists() {
        return Cost::default();
    }
    read_json(path)
        .ok()
        .and_then(|value| serde_json::from_value(value["cost"].clone()).ok())
        .unwrap_or_default()
}
