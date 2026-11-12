//! The windows (fn-192 R2, fn-355): the expected wood each draw decides, as
//! a share of the whole tree's. A draw of PA k in cycle c looks `LOOK`
//! cycles on: its wood is what a bud of k grows over the look, its own
//! stages and the ones it goes on into, and the whole is the tree expected
//! at c and the look. Nothing a draw reads depends on the age a tree is
//! asked for, so the tree at a year is the same on the way to any later
//! one.
use super::LOOK;
use crate::closed_form::expected_log_lengths;
use crate::dormant::{last_stage, wakes_in};
use crate::species::{Species, Zone};
use crate::transcendental::Transcendental;
use std::sync::Arc;

/// Where a draw stands: the PA whose wood it decides, the cycle, and how
/// far through the cycle (none for its start).
pub(crate) type At = (usize, u32, Option<f64>);

/// The expected wood each draw decides, as a share of the whole tree's.
/// Its tables are shared: every fine root's grower holds the tree's
/// (`Rules`), and a copy of them a root cost most of a root's growth in
/// Wasm.
#[derive(Clone)]
pub(crate) struct Windows {
    /// expected[m][k]: the log of the metres a bud of PA k grows in m
    /// cycles; an entry does not depend on how far the table runs.
    expected: Arc<Vec<Vec<f64>>>,
    /// The years a sleeping bud of each PA may sleep before it ages past
    /// its last stage; none for a PA no zone puts to sleep.
    stages: Arc<Vec<f64>>,
    /// woods[c][k]: the share of a draw of PA k at the start of cycle c,
    /// as the grower reads it (host decision 8): taken once a table.
    woods: Arc<Vec<Vec<f32>>>,
    /// The last year a draw may be made in.
    year: u32,
}

/// The look, in whole rows of the table.
const ROWS: usize = LOOK as usize;

impl Windows {
    /// The windows of every draw a tree makes up to `year`, the year's
    /// look and its sleeping buds' waking included.
    pub fn new(species: &Species, year: u32) -> Self {
        let pas = species.states.len();
        // Only an age some zone puts to sleep, under a release law that
        // wakes it, is ever summed or recorded asleep.
        let sleeps = |k: usize| {
            let mut zones = species.states.iter().flat_map(|s| &s.zones);
            zones.any(|z| z.rate > 0.0 && z.dormant.get(k).is_some_and(|&p| p > 0.0))
        };
        let stages: Vec<f64> = (0..pas)
            .map(|k| match sleeps(k) {
                true => last_stage(species, k),
                false => 0.0,
            })
            .collect();
        let slept = stages.iter().fold(0.0_f64, |a, &b| a.max(b)).ceil() as u32;
        let rows = year + ROWS as u32 + slept + 2;
        let mut windows = Self {
            expected: Arc::new(expected_log_lengths(species, rows)),
            stages: Arc::new(stages),
            woods: Arc::default(),
            year,
        };
        let woods = (0..=year + 1)
            .map(|c| {
                (0..pas)
                    .map(|k| crate::contract::exp(crate::contract::log32(windows.log_wood(k, c))))
                    .collect()
            })
            .collect();
        windows.woods = Arc::new(woods);
        windows
    }

    /// The last year it serves.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// The log of the whole tree's expected metres at the end of the look
    /// of a draw in `cycle`.
    fn whole(&self, cycle: u32) -> f64 {
        self.expected[cycle as usize - 1 + ROWS][0]
    }

    /// The log of the share a draw of `pa` in `cycle` decides: none where
    /// it decides none (FINE-CONTRACT.md D10's table).
    pub fn log_wood(&self, pa: usize, cycle: u32) -> f64 {
        let log = self.expected[ROWS][pa];
        if log == f64::NEG_INFINITY {
            return log;
        }
        log - self.whole(cycle)
    }

    /// `log_wood`'s share itself.
    pub fn wood(&self, pa: usize, cycle: u32) -> f64 {
        let log = self.log_wood(pa, cycle);
        if log == f64::NEG_INFINITY {
            return 0.0;
        }
        log.exp_fixed()
    }

    /// The chance that a draw made with probability `p` is made within its
    /// window, at a presence below 1, deciding `decided` of `wood`: where
    /// a stop all but made grows both outcomes (host decision 11).
    pub fn borderline(&self, p: f64, wood: f64, decided: f64) -> f64 {
        use super::{FLOOR, RATE, SPAN};
        let window = (RATE * wood * decided).clamp(FLOOR * decided, SPAN);
        if p <= 0.0 || p >= 1.0 || window <= 0.0 {
            return 0.0;
        }
        let logit = (p / (1.0 - p)).ln_fixed();
        p - 1.0 / (1.0 + (window - logit).exp_fixed())
    }

    /// The wood a sleeping bud of `pa` on a node grown in `cycle` is
    /// expected to grow, over the years it may wake in: until it would
    /// age past its last stage.
    fn dormant(&self, zone: &Zone, pa: usize, cycle: u32) -> f64 {
        (0..self.stages[pa].ceil() as u32)
            .map(|s| wakes_in(zone, s) * self.wood(pa, cycle + 1 + s))
            .sum()
    }

    /// The wood each PA's sleeping bud on a node of `zone` grown in `cycle`
    /// is expected to grow, into `woods` (none where the zone bears none),
    /// and the wood the sleeping buds of one bud place are expected to.
    pub fn sleeping(&self, zone: &Zone, cycle: u32, woods: &mut Vec<f64>) -> f64 {
        woods.clear();
        let mut expected = 0.0;
        for (pa, &p) in zone.dormant.iter().enumerate() {
            let wood = if p > 0.0 {
                self.dormant(zone, pa, cycle)
            } else {
                0.0
            };
            expected += p * wood;
            woods.push(wood);
        }
        expected
    }

    /// The share a draw of `pa` at the start of `cycle` decides, as the
    /// grower reads it: what sizes its window.
    pub fn start(&self, pa: usize, cycle: u32) -> f32 {
        self.woods[cycle as usize][pa]
    }

    /// `metres` laid in `cycle` as a share of the whole tree's expected
    /// wood at the end of its look.
    pub fn share(&self, metres: f64, cycle: u32) -> f64 {
        metres * (-self.whole(cycle)).exp_fixed()
    }

    /// The window of a draw at `at` deciding `stake` of its wood, or from
    /// `used` of the way through its cycle (host decision 8,
    /// `contract::presence`).
    pub fn window(&self, (pa, cycle, used): At, stake: f64) -> f32 {
        use super::{FLOOR, RATE, SPAN};
        use crate::contract::{presence, ticks};
        let log = |c: u32| crate::contract::log32(self.log_wood(pa, c));
        let wood = match used {
            None => self.start(pa, cycle),
            Some(used) => presence::wood_at((log(cycle), log(cycle + 1)), ticks(used)),
        };
        let window = ((RATE * stake) as f32, (FLOOR * stake) as f32, SPAN as f32);
        presence::window(wood, window)
    }
}
