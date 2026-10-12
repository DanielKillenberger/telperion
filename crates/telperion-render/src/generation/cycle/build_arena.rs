//! A build's tables sized once (the host's rulings of 2026-10-12, fn-256
//! task 5): from the closed form's expected kept phytomers, a hint and
//! never a bound, each table past its measured most against the hint by
//! a stated margin; and the plan's bounds stated from those tables, so a
//! build records its cycle once and never begins again. A table its hint
//! undersized doubles once (`build_size::grown`, counted as a move).
use super::build_grow::{Compact, Sizes};
use super::build_plan::Bounds;
use super::census as c;
use super::state::Caps;
use crate::Result;

/// The tables a build sizes: axes, units, phytomers, sleeping, living and
/// woken buds, the cycle's leaves, the lattice's nodes and its columns.
pub(super) const TABLES: usize = 9;

/// Each table's most rows at a cycle's end against the hint (`TABLES`'
/// order), over every shipped preset at its default age, seeds 1, 7, 42
/// and its own, compacted after each cycle (`arena_tests::arena_ratios`,
/// 2026-10-12): the most of each, where the spruce sets the axes and the
/// units, the beech the phytomers and leaves, the Two Trees the lattice.
pub(super) const ARENA: [f64; TABLES] = [0.6501, 1.1469, 1.9038, 0.0115, 0.1540, 0.0064, 0.4938, 0.2766, 0.0187];
/// The retired rows the records hold at most against the hint, where a
/// PA keeps them: the ordinary's at seed 42 (the oak's at most 0.88;
/// `arena_ratios`, 2026-10-12).
pub(super) const RECORDS: f64 = 1.7450;

/// The most rows a table holds: its words index it in 32 bits, its
/// widest table (the pencils) 59 words a row.
const MOST_ROWS: u32 = 1 << 26;

/// How far past the measured most a table is sized: room for a
/// compaction made only when a table runs tight (`Compact::Tight`).
pub(super) const MARGIN: f64 = 1.5;

/// What each table holds at the census's cycle's end.
pub(super) fn tables(census: &[u32]) -> [u32; TABLES] {
    let w = |k: usize| census[k];
    let leaves = w(c::PHYTS).saturating_sub(w(c::PHYTS0));
    [w(c::AXES), w(c::UNITS), w(c::PHYTS), w(c::SLEEPERS), w(c::NEXT), w(c::WOKEN), leaves, w(c::GRID + 6), w(c::GRID + 3) * w(c::GRID + 4)]
}

/// The closed form's expected kept phytomers of `species` at `age`, its
/// light's terms let go where it reads light (the form is light-neutral).
pub(super) fn hint(species: &telperion_space::Species, age: u32) -> Result<f64> {
    let mut unshaded = species.clone();
    for s in &mut unshaded.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    let kept = telperion_space::expected_kept(&unshaded, age.max(1)).map_err(|e| super::super::io::error(e.to_string()))?;
    Ok(kept.unwrap_or(0.0))
}

/// The most hierarchy levels a cycle's sums sweep (`hier_build`'s
/// bound), a step's generations and a round's placing passes, the
/// layout's generations, and the units' rounds a cycle runs: stated, past
/// any a shipped preset reached (levels 41, generations 6, placing passes
/// 4, the layout's 6; `arena_tests`, 2026-10-12). Every cycle records all
/// five units rounds: four refused the ordinary at cycle 18.
const LEVELS: u32 = 63;
const GENS: u32 = 8;
const PASSES: u32 = 6;
const LAY_GENERATIONS: u32 = 15;
const ROUNDS: u32 = 5;

impl Sizes {
    /// A build's tables sized once from `hint`, the least a build takes
    /// below that, and its bounds stated from them; refused by name where
    /// a table would pass the rows the device's tables index.
    pub(super) fn hinted(hint: f64) -> Result<Self> {
        let most = ARENA.iter().chain([&RECORDS]).fold(0.0, |m: f64, r| m.max(*r));
        if !(hint * most * MARGIN <= f64::from(MOST_ROWS)) {
            let what = format!("the tree's expected {hint:.0} kept phytomers size its tables past the {MOST_ROWS} rows the device indexes; grow it younger");
            return Err(super::super::io::error(what));
        }
        let least = Sizes::least();
        let rows = |k: usize, floor: u32| ((hint * ARENA[k] * MARGIN).ceil() as u32).max(floor);
        let l = least.caps;
        let caps = Caps {
            axes: rows(0, l.axes).next_multiple_of(64),
            units: rows(1, l.units),
            phyts: rows(2, l.phyts),
            sleepers: rows(3, l.sleepers),
            next: rows(4, l.next).next_multiple_of(64),
            woken: rows(5, l.woken),
        };
        let tips = caps.next.max(least.tips);
        let points = (rows(6, 0) + tips).max(least.points);
        let records = (hint * RECORDS * MARGIN).ceil() as u32;
        let sizes = Sizes { caps, points, tips, nodes: rows(7, least.nodes), columns: rows(8, least.columns), records, compact: Compact::Tight, ..least };
        Ok(Sizes { bounds: sizes.stated(), ..sizes })
    }

    /// The plan's bounds from the tables: each list's ranges at the most
    /// its table holds, the sweep's layer ranges at the most a lattice of
    /// its nodes takes stood as a cube twice over, the rest stated.
    pub(super) fn stated(&self) -> Bounds {
        let mut b = Bounds { ranges: self.bounds.ranges, ..Bounds::default() };
        let r = b.ranges;
        let (next, nodes) = (self.caps.next, self.nodes);
        b.units = next.div_ceil(r.units) + 1;
        b.lay.draws = next.div_ceil(r.draw) + 1;
        b.lay.deposits = self.points.div_ceil(r.deposit) + 1;
        b.lay.lights = (9 * nodes).div_ceil(r.paths).max(nodes.div_ceil(r.nodes)) + 1;
        b.lay.sweeps = 2 * (f64::from(nodes).cbrt().ceil() as u32) + 8;
        b.lay.generations = LAY_GENERATIONS;
        (b.hier.levels, b.hier.gens, b.hier.passes, b.rounds) = (LEVELS, GENS, PASSES, ROUNDS);
        b
    }

    /// The most each census word reaches: a table's count its rows, none
    /// for the rest (`given::Span`).
    pub(super) fn ceilings(&self) -> Vec<u32> {
        let mut out = vec![u32::MAX; c::WORDS];
        let caps = self.caps;
        for (words, cap) in [
            (&[c::AXES, c::AXES0, c::LAID, c::LAID0, c::BUILT][..], caps.axes),
            (&[c::UNITS, c::UNITS0], caps.units),
            (&[c::PHYTS, c::PHYTS0], caps.phyts),
            (&[c::SLEEPERS, c::SLEEPERS0], caps.sleepers),
            (&[c::WOKEN, c::WOKEN0], caps.woken),
            (&[c::NEXT, c::LIVE, c::LIVE0, c::GREW], caps.next),
            (&[c::GRID + 6], self.nodes),
        ] {
            words.iter().for_each(|&w| out[w] = cap);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The integration review's oak at 500: a hint whose tables pass the
    /// rows the device indexes is refused by name, never wrapped.
    #[test]
    fn a_hint_past_the_tables_is_refused_by_name() {
        let refused = Sizes::hinted(86.6e9).expect_err("refused");
        assert!(refused.to_string().contains("grow it younger"), "{refused}");
        let telperion = Sizes::hinted(2_970_013.0).expect("sized");
        assert!(telperion.caps.axes % 64 == 0 && telperion.caps.phyts > 2_970_013);
    }
}
