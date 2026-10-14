//! A build's tables sized (the host's rulings of 2026-10-12, fn-256
//! task 5): from the closed form's expected kept phytomers, a hint and
//! never a bound, each table from its measured most against the hint by
//! a stated margin; and the plan's bounds stated from those tables, so a
//! build records its cycle once and never begins again. A table its hint
//! undersized grows by observed counts (`build_size::grown`, each growth
//! a move).
use super::build_grow::{Compact, Sizes};
use super::build_plan::Bounds;
use super::census as c;
use super::state::Caps;
use crate::Result;

/// The tables a build sizes: axes, units, phytomers, sleeping, living and
/// woken buds, the cycle's leaves and the lattice's nodes.
pub(super) const TABLES: usize = 8;

/// Each table's most rows at a cycle's end against its hint (`TABLES`'
/// order; the axes' against the expected axes, the rest against the
/// expected kept phytomers), over every shipped preset at its default age,
/// seeds 1, 7, 42 and its own, compacted after each cycle, the axes inside
/// retired subtrees let go on every preset (`arena_tests::arena_ratios`,
/// fn-270 task 3, `raw/t3-l3-ratios.log`): the most of each, where the
/// beech at 42 sets the axes, phytomers, living buds and leaves, the
/// spruce at 42 the units and the woken, the ordinary the sleeping buds,
/// Laurelin the lattice. The axes run 0.39 (the oak and Telperion at seed
/// 1) to 1.43 (the beech at 42) of their hint.
pub(super) const ARENA: [f64; TABLES] = [1.4303, 1.0235, 2.2452, 0.0115, 0.1397, 0.0064, 0.4604, 0.2743];

/// The most rows a table holds: its words index it in 32 bits, its
/// widest table (the pencils) 59 words a row.
const MOST_ROWS: u32 = 1 << 26;

/// How far past the measured most a table starts (fn-270 R4, the host's
/// ruling: tables start from the hint and grow by observed counts,
/// `build_size::grown`). Measured on every preset at margins 0.5 to 1.5
/// and growth 1.5 and 2 (`MEMORY.md`): at 1.0 the Two Trees peak least
/// and stay within a twentieth of their warm build time; at 1.5 no
/// preset moves and every table is a third larger than it needs.
pub(super) const MARGIN: f64 = 1.0;

/// What each table holds at the census's cycle's end (the living, at most
/// within it).
pub(super) fn tables(census: &[u32]) -> [u32; TABLES] {
    let w = |k: usize| census[k];
    let leaves = w(c::PHYTS).saturating_sub(w(c::PHYTS0));
    // The living table holds the most of the living the cycle left, after
    // its balance, and the axes that grew.
    let living = w(c::NEXT).max(w(c::LIVE)).max(w(c::GREW));
    [w(c::AXES), w(c::UNITS), w(c::PHYTS), w(c::SLEEPERS), living, w(c::WOKEN), leaves, w(c::GRID + 6)]
}

/// What the closed form expects of a build: its kept phytomers, and its
/// axes, which every axis-indexed table is sized from (the oak at 80 held
/// a fifth of its kept phytomers' axes, the spruce two thirds: one ratio
/// for both sized the oak past 4 GiB; fn-256 task 6).
#[derive(Debug, Clone, Copy)]
pub(super) struct Hint {
    pub kept: f64,
    pub axes: f64,
    /// The most units one job grows in a cycle (`most_units`, fn-276):
    /// its units rounds and the generations it adds a cycle.
    pub units: u32,
}

/// The closed form's expectations of `species` at `age`, its light's
/// terms let go where it reads light (the form is light-neutral).
pub(super) fn hint(species: &telperion_space::Species, age: u32) -> Result<Hint> {
    let mut unshaded = species.clone();
    for s in &mut unshaded.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    let refused = |e: telperion_space::Error| super::super::io::error(e.to_string());
    let kept = telperion_space::expected_kept(&unshaded, age.max(1)).map_err(refused)?;
    let axes = telperion_space::expected_axes(&unshaded, age.max(1)).map_err(refused)?;
    Ok(Hint { kept: kept.unwrap_or(0.0), axes, units: telperion_space::most_units(species) })
}

/// The most hierarchy levels a cycle's sums sweep (`hier_build`'s
/// bound), a step's generations and a round's placing passes, the
/// layout's generations, and the units' rounds a cycle runs: stated, past
/// any a shipped preset reached (levels 41, generations 6, placing passes
/// 4, the layout's 6; `arena_tests`, 2026-10-12; units rounds 5, the
/// ordinary's, fn-256 task 12). A cycle records the units rounds the
/// census read used and a slack (`build_size::limits`), up to these.
const LEVELS: u32 = 63;
const GENS: u32 = 8;
const PASSES: u32 = 6;
const LAY_GENERATIONS: u32 = 15;
const ROUNDS: u32 = 8;

/// The sweep's layer ranges a plan holds at least: a lattice of 1,024
/// layers, 512 m, however narrow. A standing box is two nodes wide at
/// least on each axis (`span_main` takes a point's node and the next), so
/// the least arena's 4,096 nodes stand 1,024 layers at most. The cube a
/// small arena's nodes stood as refused a tall, narrow lattice its table
/// held (fn-256 task 15).
const SWEEPS: u32 = 1024 / super::build_light::LAYERS;

/// The units rounds and generations a cycle records at least where a
/// job grows `units` units in it (fn-276, host 2026-10-14): each unit
/// after the first takes a round to move or relay and one to grow, and
/// is an axis a generation deeper.
pub(super) fn floors(units: u32) -> [u32; 2] {
    [2 * units.max(1) - 1, units.max(1)]
}

impl Sizes {
    /// A build's tables sized once from `hint`, the least a build takes
    /// below that, and its bounds stated from them; refused by name where
    /// a table would pass the rows the device's tables index: the tests'.
    #[cfg(test)]
    pub(super) fn hinted(h: Hint) -> Result<Self> {
        Self::at_margin(h, MARGIN)
    }

    /// `hinted` at `margin` past each table's measured most.
    fn at_margin(h: Hint, margin: f64) -> Result<Self> {
        let most = ARENA[1..].iter().fold(0.0, |m: f64, r| m.max(*r));
        if !(h.kept * most * margin <= f64::from(MOST_ROWS) && h.axes * ARENA[0] * margin <= f64::from(MOST_ROWS)) {
            let what = format!("the tree's expected {:.0} kept phytomers and {:.0} axes size its tables past the {MOST_ROWS} rows the device indexes; grow it younger", h.kept, h.axes);
            return Err(super::super::io::error(what));
        }
        if h.units > super::device::FRAMES {
            let what = format!("the species grows {} units a cycle on one apex, past the {} a units job holds", h.units, super::device::FRAMES);
            return Err(super::super::io::error(what));
        }
        let least = Sizes::least();
        let rows = |k: usize, floor: u32| ((if k == 0 { h.axes } else { h.kept } * ARENA[k] * margin).ceil() as u32).max(floor);
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
        let sizes = Sizes { caps, points, tips, nodes: rows(7, least.nodes), compact: Compact::Tight, units: h.units, ..least };
        Ok(Sizes { bounds: sizes.stated(), ..sizes })
    }

    /// The tables a build starts from, from `hint`, capped to what the
    /// device budget leaves (fn-270 R4, host after task 3): at `MARGIN`
    /// where they fit, and only where the budget refuses them, at half the
    /// margin as often as it refuses, down to the least tables; growth by
    /// observed counts does the rest (`build_size::grown`).
    pub(super) fn margins(h: Hint) -> impl Iterator<Item = Result<Self>> {
        let least = Sizes::least().caps;
        let mut margin = Some(MARGIN);
        std::iter::from_fn(move || {
            let at = margin?;
            let sizes = Self::at_margin(h, at);
            let floor = sizes.as_ref().map_or(true, |s| s.caps == least || at < 1e-6);
            margin = (!floor).then_some(at / 2.0);
            Some(sizes)
        })
    }

    /// The plan's bounds from the tables: each list's ranges at the most
    /// its table holds, the sweep's layer ranges at the most a lattice of
    /// its nodes takes stood as a cube twice over or `SWEEPS`, the rest
    /// stated.
    pub(super) fn stated(&self) -> Bounds {
        let mut b = Bounds { ranges: self.bounds.ranges, ..Bounds::default() };
        let r = b.ranges;
        let (next, nodes) = (self.caps.next, self.nodes);
        b.units = next.div_ceil(r.units) + 1;
        b.lay.draws = next.div_ceil(r.draw) + 1;
        b.lay.deposits = self.points.div_ceil(r.deposit) + 1;
        b.lay.sweeps = (2 * (f64::from(nodes).cbrt().ceil() as u32) + 8).max(SWEEPS);
        // A job's units chained in a cycle take two rounds each after the
        // first, and add a generation each (`floors`).
        let [rounds, gens] = floors(self.units);
        b.lay.generations = LAY_GENERATIONS.max(gens);
        (b.hier.levels, b.hier.gens, b.hier.passes, b.rounds) = (LEVELS, GENS.max(gens), PASSES, ROUNDS.max(rounds));
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
        let refused = Sizes::hinted(Hint { kept: 86.6e9, axes: 1e6, units: 1 }).expect_err("refused");
        assert!(refused.to_string().contains("grow it younger"), "{refused}");
        let refused = Sizes::hinted(Hint { kept: 1e6, axes: 1e12, units: 1 }).expect_err("refused");
        assert!(refused.to_string().contains("grow it younger"), "{refused}");
        let telperion = Sizes::hinted(Hint { kept: 2_970_013.0, axes: 1e6, units: 1 }).expect("sized");
        assert!(telperion.caps.axes % 64 == 0 && telperion.caps.phyts > 2_970_013);
    }
}
