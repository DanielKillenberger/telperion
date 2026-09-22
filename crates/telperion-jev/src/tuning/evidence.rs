//! Which revisions' measured evidence a run may still use.
//!
//! `Config::identity()` covers the budget caps, so an accepted cap-only resume
//! gives the run a new identity while the trials it just preserved keep the
//! identity they were measured under. Without this, one such resume made every
//! earlier trial invisible and a second preserving resume impossible.
//!
//! Evidence measured under A is usable under B exactly when the run holds an
//! accepted authorization that says so: `preserve_evidence`, `identity == A`,
//! `next_identity == B`. Accepting it already proved the configuration
//! unchanged apart from the named caps and re-checked the artifact bytes.
//! Nothing here rewrites a stored identity or changes what a trial records.
use super::engine::Run;

impl Run {
    /// The current identity, plus every earlier one reachable through an
    /// unbroken chain of preserving resumes. A resume that did not preserve
    /// its evidence ends the walk: nothing before it is reachable.
    pub fn evidence_identities(&self) -> Vec<String> {
        let mut chain = vec![self.identity.clone()];
        for decision in self.authorizations.iter().rev() {
            if !decision.preserve_evidence {
                break;
            }
            let bridges = decision
                .next_identity
                .as_deref()
                .is_some_and(|next| chain.iter().any(|known| known == next));
            if bridges && !chain.contains(&decision.identity) {
                chain.push(decision.identity.clone());
            }
        }
        chain
    }

    /// True when evidence measured under `identity` is still this run's to use.
    pub fn measured_here(&self, identity: &str) -> bool {
        self.evidence_identities().iter().any(|k| k == identity)
    }
}
