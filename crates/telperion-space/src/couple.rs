//! The oak under fn-209's cut (fn-211.4; DESIGN-LIGHT.md after Astra's
//! review, rewritten from probe P4): a fine root's history-dependent
//! aggregate state and its records.
//!
//! In the coupling pass a fine root is an `Aggregate` advanced every
//! cycle under its actual inputs (the light it reads, its kept share and
//! each PA's unit size): living cohorts by PA and units spent, its wood
//! and the leaves its last unit grew. Its transitions are the engine's
//! unit rules in expectation, evaluated from the current parameters:
//! survival under shade, each zone's nodes and laterals, abortion and its
//! relay, a failed unit's relay, an age's end and its continuation. Its
//! explicit approximations: whole-cycle units; sleeping buds are not
//! carried; the balance shedding of laterals inside the subtree is not
//! carried (only the root's own); a cohort's units are all of its PA's
//! size; presences near a bound are taken whole.
//!
//! The pass writes, per fine root per cycle, the light it read, its
//! vigour share and its kept share, and the cycle its balance shed it
//! (`Records`): what its subtree is grown from.
use crate::species::Species;
use crate::transcendental::Transcendental;

/// Living apexes below this weight are dropped.
const NEGLIGIBLE: f64 = 1e-9;

/// One cohort of living apexes: their PA, the whole units spent in it,
/// and their expected number per root.
#[derive(Debug, Clone, Copy)]
struct Cohort {
    pa: u8,
    spent: u8,
    w: f64,
}

/// A fine subtree's aggregate state, per unit of its root's base scale.
#[derive(Debug, Clone, Default)]
pub(crate) struct Aggregate {
    cohorts: Vec<Cohort>,
    /// Its wood grown so far (internode × size per phytomer).
    pub wood: f64,
    /// The leaf area its last unit grew: none once it is extinct.
    pub leaves: f64,
    /// The phytomers it is expected to have grown, and in its last unit.
    pub grown: f64,
    pub last: f64,
}

/// What a fine root reads in one cycle.
pub(crate) struct Inputs<'a> {
    pub light: f64,
    pub kept: f64,
    /// Each PA's unit size this cycle.
    pub size: &'a [f64],
}

impl Aggregate {
    /// A bud of `pa` that has spent `spent` units in it.
    pub fn new(pa: usize, spent: f64) -> Self {
        Self {
            cohorts: vec![Cohort {
                pa: pa as u8,
                spent: spent.round() as u8,
                w: 1.0,
            }],
            ..Self::default()
        }
    }

    /// Its living apexes.
    pub fn living(&self) -> f64 {
        self.cohorts.iter().map(|c| c.w).sum()
    }

    /// Its living apexes by PA, times `scale`, added into `by_pa`.
    pub fn living_by_pa(&self, scale: f64, by_pa: &mut [f64]) {
        for c in &self.cohorts {
            by_pa[c.pa as usize] += c.w * scale;
        }
    }

    pub fn alive(&self) -> bool {
        !self.cohorts.is_empty()
    }

    /// One cycle's growth unit of every living apex; an extinct aggregate
    /// grows nothing and casts no leaves (probe P4 kept casting its last
    /// unit's leaves, about twice the realized area at 80).
    pub fn advance(&mut self, species: &Species, laterals: &[Vec<Vec<f64>>], at: Inputs) {
        self.leaves = 0.0;
        self.last = 0.0;
        let mut next: Vec<Cohort> = Vec::with_capacity(self.cohorts.len() * 2);
        let mut push = |pa: usize, spent: f64, w: f64| {
            if w > NEGLIGIBLE {
                let spent = spent.min(250.0) as u8;
                next.push(Cohort {
                    pa: pa as u8,
                    spent,
                    w,
                });
            }
        };
        for c in std::mem::take(&mut self.cohorts) {
            let pa = c.pa as usize;
            let state = &species.states[pa];
            let mut v = state.viability;
            if state.shade_hazard > 0.0 && at.light < 1.0 {
                v = v.powf_fixed(at.light.max(1e-6).powf_fixed(-state.shade_hazard));
            }
            let w = c.w * at.kept;
            let spent = f64::from(c.spent) + 1.0;
            // A unit that failed is spent; its relay grows from the next.
            push(pa, spent, w * (1.0 - v) * state.relay_failed);
            let grows = w * v;
            let size = at.size[pa];
            for (z, zone) in state.zones.iter().enumerate() {
                let nodes = grows * zone.nodes.mean();
                self.last += nodes;
                self.leaves += nodes * state.leaf_area * size;
                self.wood += nodes * state.internode * size;
                for (j, &p) in laterals[pa][z].iter().enumerate() {
                    if let (true, Some((lived, reach))) = (p > 0.0, species.lived(j)) {
                        push(lived, 0.0, nodes * zone.buds * p * reach);
                    }
                }
            }
            if spent >= state.life() {
                let (next_pa, go) = species.successor(pa).unwrap_or((pa, 0.0));
                push(next_pa, 0.0, grows * go);
                push(pa, spent, grows * (1.0 - go) * state.relay_ended);
            } else {
                let a = state.abortion_at(spent);
                push(pa, spent, grows * a * state.relay);
                push(pa, spent, grows * (1.0 - a));
            }
        }
        self.grown += self.last;
        next.sort_unstable_by_key(|c| (c.pa, c.spent));
        next.dedup_by(|b, a| {
            let same = a.pa == b.pa && a.spent == b.spent;
            if same {
                a.w += b.w;
            }
            same
        });
        self.cohorts = next;
    }
}

/// A fine root's records, per cycle from its first unit in the pass on:
/// the light it read, its vigour share and its kept share; and the cycle
/// its own balance or an ancestor's shed it.
#[derive(Debug, Clone, Default)]
pub(crate) struct Records {
    pub first: u32,
    pub light: Vec<f32>,
    pub vigour: Vec<f32>,
    pub kept: Vec<f32>,
    /// The light its aggregate collected (presence times light) for the
    /// vigour it was given.
    pub q: Vec<f32>,
    pub shed: Option<u32>,
}

impl Records {
    pub fn starting(first: u32) -> Self {
        Self {
            first,
            ..Self::default()
        }
    }

    fn at(v: &[f32], first: u32, cycle: u32, or: f64) -> f64 {
        cycle
            .checked_sub(first)
            .and_then(|k| v.get(k as usize))
            .map_or(or, |&x| f64::from(x))
    }

    fn set(v: &mut Vec<f32>, first: u32, cycle: u32, x: f64) {
        let k = (cycle - first) as usize;
        if v.len() <= k {
            v.resize(k + 1, 0.0);
        }
        v[k] = x as f32;
    }

    /// The light its buds read for their growth in `cycle`.
    pub fn light(&self, cycle: u32) -> f64 {
        Self::at(&self.light, self.first, cycle, 1.0)
    }

    pub fn vigour(&self, cycle: u32) -> f64 {
        Self::at(&self.vigour, self.first, cycle, 0.0)
    }

    pub fn kept(&self, cycle: u32) -> f64 {
        Self::at(&self.kept, self.first, cycle, 1.0)
    }

    pub fn set_light(&mut self, cycle: u32, x: f64) {
        Self::set(&mut self.light, self.first, cycle, x);
    }

    pub fn set_vigour(&mut self, cycle: u32, x: f64) {
        Self::set(&mut self.vigour, self.first, cycle, x);
    }

    pub fn q(&self, cycle: u32) -> f64 {
        Self::at(&self.q, self.first, cycle, 0.0)
    }

    pub fn set_q(&mut self, cycle: u32, x: f64) {
        Self::set(&mut self.q, self.first, cycle, x);
    }

    pub fn set_kept(&mut self, cycle: u32, x: f64) {
        Self::set(&mut self.kept, self.first, cycle, x);
    }

    pub fn bytes(&self) -> usize {
        4 * (self.light.len() + self.vigour.len() + self.kept.len() + self.q.len()) + 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An aggregate that dies out casts nothing in the cycles after.
    #[test]
    fn an_extinct_aggregate_casts_no_leaves() {
        let species = crate::oak();
        let laterals: Vec<_> = species.states.iter().map(|s| s.laterals()).collect();
        let size = vec![1.0; species.states.len()];
        let pa = (0..species.states.len())
            .find(|&p| species.states[p].shade_hazard > 0.0)
            .expect("a PA shade kills");
        let mut agg = Aggregate::new(pa, 0.0);
        // In deep shade and starving: its kept share falls every year.
        let dark = || Inputs {
            light: 1e-4,
            kept: 0.3,
            size: &size,
        };
        let mut cycles = 0;
        while agg.alive() && cycles < 500 {
            agg.advance(&species, &laterals, dark());
            cycles += 1;
        }
        assert!(!agg.alive(), "dies out");
        agg.advance(&species, &laterals, dark());
        assert_eq!((agg.leaves, agg.last), (0.0, 0.0));
    }
}
