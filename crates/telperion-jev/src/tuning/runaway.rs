//! The runaway guard (fn-117). With no cap on a run, what stops a loop that
//! keeps spending and keeps nothing is code counting the rounds in a row that
//! kept no adoption. At the count the run pauses and names the rounds and
//! what they spent; the owner's scoped resume starts the count again.
use super::engine::Run;
use super::state::Budget;
use serde::{Deserialize, Serialize};

/// Consecutive rounds without a kept adoption before the run pauses.
pub const ROUNDS: u64 = 5;
/// How a runaway pause reason begins.
pub const REASON: &str = "runaway";

/// The spend counters a round opens on.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Spend {
    pub tokens: u64,
    pub evaluations: u64,
    pub images: u64,
    pub visual_passes: u64,
}

impl Spend {
    pub fn of(budget: &Budget) -> Self {
        Self {
            tokens: budget.tokens,
            evaluations: budget.evaluations,
            images: budget.images,
            visual_passes: budget.visual_passes.unwrap_or(0),
        }
    }
    /// What was spent between this opening and `budget`.
    pub fn since(&self, budget: &Budget) -> Self {
        let now = Self::of(budget);
        Self {
            tokens: now.tokens.saturating_sub(self.tokens),
            evaluations: now.evaluations.saturating_sub(self.evaluations),
            images: now.images.saturating_sub(self.images),
            visual_passes: now.visual_passes.saturating_sub(self.visual_passes),
        }
    }
}

/// The trailing rounds that kept nothing, and the spend the first opened on.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Streak {
    pub rounds: Vec<u64>,
    pub opening: Spend,
}

impl Run {
    /// Records how a round ended: a kept adoption ends the streak, anything
    /// else extends it. `opening` is the spend the round started on.
    pub(super) fn close_round(
        &mut self,
        kept: bool,
        opening: Spend,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        if kept {
            self.unkept = None;
        } else {
            let round = self.budget.rounds;
            let streak = self.unkept.get_or_insert(Streak {
                rounds: vec![],
                opening,
            });
            streak.rounds.push(round);
        }
        save(self)
    }

    /// Refuses the next round once `limit` rounds in a row kept nothing.
    pub(super) fn runaway(&self, limit: u64) -> Result<(), String> {
        let Some(streak) = &self.unkept else {
            return Ok(());
        };
        if (streak.rounds.len() as u64) < limit {
            return Ok(());
        }
        let spent = streak.opening.since(&self.budget);
        let rounds = streak
            .rounds
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        Err(format!(
            "{REASON}: {} rounds in a row kept nothing (rounds {rounds}); they spent {} tokens, \
             {} evaluations, {} images and {} visual passes",
            streak.rounds.len(),
            spent.tokens,
            spent.evaluations,
            spent.images,
            spent.visual_passes
        ))
    }

    /// The owner's scoped resume of a runaway pause starts the count again.
    pub fn resume_runaway(&mut self) {
        if self
            .pause
            .as_ref()
            .is_some_and(|p| p.reason.starts_with(REASON))
        {
            self.unkept = None;
        }
    }
}
