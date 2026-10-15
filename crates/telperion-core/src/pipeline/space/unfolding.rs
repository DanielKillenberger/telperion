//! A tree drawn within its year as the conversion reads it (fn-270 R5,
//! R5a, R5c): each phytomer's leaf share from the presentation, the
//! leaves the wood's age keeps now and kept a year before, and each
//! node's seasonal leaf: falling, colouring, growing again, passing its
//! lifespan.
use super::PaRow;
use crate::tree::{kept_share, Unfolded};
use telperion_space::unfold::YEAR;

/// A tree drawn within its year: each axis's leaves' shares by phytomer
/// (empty where all are whole), how far into the year it is drawn, and
/// the first of the axes gone this year, which follow the ones grown.
#[derive(Debug, Clone, Copy)]
pub(in crate::pipeline) struct Unfolding<'a> {
    pub leaves: &'a [Vec<f32>],
    pub fraction: f64,
    pub gone: usize,
}

impl Unfolding<'_> {
    pub(super) const WHOLE: Self = Self { leaves: &[], fraction: 1.0, gone: usize::MAX };

    /// The thinnest wood axis `axis` draws: `FINEST`, and on a subtree gone
    /// this year the thinnest it drew a year before as it fades, so its
    /// wood is judged once, on its radius then (fn-286; host, 2026-10-15).
    pub(super) fn least(&self, axis: usize) -> f64 {
        match self.within() && axis >= self.gone {
            true => super::FINEST * telperion_space::unfold::kept(self.fraction),
            false => super::FINEST,
        }
    }

    /// Whether the tree is drawn within its year.
    pub(super) fn within(&self) -> bool {
        self.fraction < 1.0
    }

    /// The year `structure` is drawn in, by its seed's PA (fn-270 R5b).
    pub(super) fn year(&self, structure: &telperion_space::Structure, rows: &[PaRow]) -> Option<crate::tree::Year> {
        let s = rows.get(structure.axes.first()?.pa)?.season;
        let age = f64::from(structure.age) - 1.0 + self.fraction.min(1.0);
        Some(crate::tree::Year { age, budburst: s.budburst, elongation: s.elongation, expansion: s.expansion })
    }

    /// The leaves wood of `cycle` keeps in a tree of `age` years under
    /// `retention`, and kept a year before: the same at the whole year
    /// and on this year's wood.
    pub(super) fn kept(&self, retention: (u32, u16), age: u32, cycle: u32) -> (u16, u16) {
        let now = kept_share(retention, age.saturating_sub(cycle));
        if !self.within() || cycle >= age {
            return (now, now);
        }
        (now, kept_share(retention, age - 1 - cycle))
    }

    /// The leaves of the node phytomer `place` of axis `axis` draws, of
    /// PA `row`, grown in `cycle` of a tree `age` years old: this year's
    /// wood grows its leaves by the presentation's share and passes none;
    /// older wood's leaves pass their lifespan by degree, colour and fall,
    /// and grow again at budburst; a gone subtree's all pass and fall,
    /// over the season's fall or as its wood thins, whichever is ahead,
    /// and grow none again.
    pub(super) fn node(&self, (axis, place): (usize, usize), row: &PaRow, (age, cycle): (u32, u32)) -> Unfolded {
        let s = &row.season;
        let tau = self.fraction * YEAR;
        let grow = self.leaves.get(axis).and_then(|l| l.get(place)).copied().unwrap_or(1.0);
        let passing = match cycle < age {
            true => s.passing(f64::from(age - 1 - cycle)),
            false => 0.0,
        };
        let gone = axis >= self.gone;
        let fall = s.fallen(tau) as f32;
        Unfolded {
            grow,
            fall: if gone { fall.max(1.0 - grow) } else { fall },
            colour: if tau < s.budburst { s.colour(tau) as f32 } else { 0.0 },
            regrow: if gone { 0.0 } else { s.regrow(tau) as f32 },
            life: if gone { u16::MAX } else { (passing * f64::from(u16::MAX)).round() as u16 },
        }
    }
}
