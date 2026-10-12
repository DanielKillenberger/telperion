//! Phases (i) to (iii) across the cut (PHASES.md): each stand's totals
//! and lit presence reported to its bearer, the coarse grower's
//! decisions and vigour handed down, its per-PA sums with the stands'.
use super::Executor;
use crate::allocation::{size_sums, sized, split};

impl Executor<'_> {
    /// Phase (i): each stand's totals at its placeholder, the coarse
    /// grower's decisions there handed down.
    pub(super) fn balance(&mut self, cycle: u32) {
        if !self.coarse.balances() {
            return;
        }
        let given = self.stand_totals();
        let rows = self.coarse.rows().expect("laid");
        let totals = self.coarse.totals(&rows, &given);
        let (shed, kept) = self.coarse.shed_by(cycle, &rows, &totals, None);
        self.stand_kept(cycle, &shed, &kept);
    }

    /// Phases (ii) and (iii): each stand's lit presence reported to its
    /// bearer; the coarse grower's per-PA sums with the stands', and every
    /// bud sized by them; each PA's sums.
    pub(super) fn allot(&mut self) -> Vec<[f32; 2]> {
        if !self.coarse.allots() {
            return Vec::new();
        }
        let rows = self.coarse.rows().expect("laid");
        let mut inputs = self.coarse.inputs(&rows);
        for (at, lit) in self.stand_lit() {
            inputs.lit[at] = lit;
        }
        let (_, got) = split(&rows.links, &inputs.lit, &inputs.lambda, None);
        let pas = self.coarse.species.states.len();
        let sums = size_sums(&got, &inputs.presence, &inputs.psi, &rows.pa, pas);
        let mut total: Vec<f32> = sums.iter().flatten().copied().collect();
        self.stand_sums(&got, &mut total);
        let factors: Vec<[f32; 2]> = total.chunks(2).map(|s| [s[0], s[1]]).collect();
        let sizes = sized(&got, &inputs.presence, &inputs.psi, &rows.pa, &factors);
        self.coarse.set_sizes(&sizes);
        factors
    }
}
