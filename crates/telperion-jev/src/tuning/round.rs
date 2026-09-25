//! What the rounds have already tried from the tree the loop stands on, for
//! the digest the next proposal reads.
use super::{engine::Run, evaluation::Trial};

impl Run {
    fn current_candidate_key(&self) -> Option<String> {
        self.current
            .and_then(|i| self.trials.get(i))
            .map(|t| t.key.clone())
    }

    /// Every attempt made from the candidate that is current now, in order.
    /// The digest folds these down; nothing else reads them in full.
    pub fn attempts_here_trials(&self) -> Vec<&Trial> {
        let Some(key) = self.current_candidate_key() else {
            return vec![];
        };
        self.trials
            .iter()
            .filter(|t| {
                self.measured_here(&t.identity) && t.round > 0 && t.base.as_deref() == Some(&key)
            })
            .collect()
    }
}
