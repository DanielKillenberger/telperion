//! The workgroups the host gives each dispatch whose size only the
//! device knows (DESIGN.md section 7a: direct dispatches only). The host
//! bounds every census word from the census it read a cycle late and the
//! one before: a count that only grows is at least what it was read, and
//! every count at most what it was plus the more of two cycles at the
//! rate it grew and three times its last step (and an eighth, for a
//! count that also falls), that reach times the word's width. Each
//! dispatch is given the most its expression reaches over those bounds;
//! its invocations exit past the true count. A table's count reaches no
//! further than its rows (`Sizes::ceilings`), and a count the arena's
//! compaction cut since it was read is at least what the census read
//! after the compaction holds. The device
//! flags a cycle where a dispatch was given too few (`Flag::Given`), and
//! the build stops there by name (fn-256 task 5: no build begins again).
use super::census as c;
use super::device::{T_UNITS, T_WAKE};
use super::gate::{Limits, REC};
use super::plan::{Gated, Plan, Size, Step};
use super::vm::E;
use std::rc::Rc;

/// The census words that never fall from one cycle to the next: the
/// state's counts, the axes laid and built, the blocks and the levels.
/// A level's first block is not one: a block placed in a shallower level
/// moves every deeper level's first block on, and a step that reorders
/// the levels moves it back (Telperion 82 seed 42).
fn grows(w: usize) -> bool {
    let counts = [c::AXES, c::UNITS, c::PHYTS, c::LAID, c::BUILT, c::GROWN, c::AXES0, c::UNITS0, c::PHYTS0, c::LAID0];
    counts.contains(&w) || [c::BLOCKS0, c::LEVELS0, c::LEVELS].contains(&w)
}

/// A table's count's words at its cycle's start and end in one census.
fn within(w: usize) -> Option<(usize, usize)> {
    match w {
        c::AXES | c::AXES0 => Some((c::AXES0, c::AXES)),
        c::UNITS | c::UNITS0 => Some((c::UNITS0, c::UNITS)),
        c::PHYTS | c::PHYTS0 => Some((c::PHYTS0, c::PHYTS)),
        c::LAID | c::LAID0 | c::BUILT => Some((c::LAID0, c::LAID)),
        c::SLEEPERS | c::SLEEPERS0 => Some((c::SLEEPERS0, c::SLEEPERS)),
        c::WOKEN | c::WOKEN0 => Some((c::WOKEN0, c::WOKEN)),
        _ => None,
    }
}

/// How far past its projection each census word reaches, in its reach:
/// once (the gate, `arena_tests`, gave no dispatch too few at it, and
/// twice cost Telperion 61 ms).
const WIDTH: u32 = 1;

/// The least a table's count is given: 64 workgroups.
const FLOOR: u32 = 4096;

/// The lattice's nodes two cycles on from the census `read` and the one
/// before: they grow as its box's volume, each side grown by three of its
/// last steps, two cells at least, twice over. The dispatch over them is
/// given this, and the node table grows to hold it (`build_size::grown`,
/// fn-276: the Laurelin at 20 stepped all three sides at once).
pub(super) fn boxed(read: &[u32], prev: Option<&[u32]>) -> u32 {
    let side = |k: usize| {
        let now = read[c::GRID + 3 + k];
        let step = now.saturating_sub(prev.map_or(0, |p| p[c::GRID + 3 + k]));
        u64::from(now + (3 * step).max(2))
    };
    (2 * side(0) * side(1) * side(2)).min(u64::from(u32::MAX)) as u32
}

/// Each census word's least and most this cycle.
pub(super) struct Span {
    lo: Vec<u32>,
    hi: Vec<u32>,
}

impl Span {
    /// From `read` and the census before it (none where only the seed's
    /// was read), each word no further than its `ceiling`, a count that
    /// grows at least what the census after a compaction since the read
    /// holds (`cut`), where there was one; the cycle and its limits exact.
    pub(super) fn new(read: &[u32], prev: Option<&[u32]>, (ceiling, cut): (&[u32], Option<&[u32]>), (cycle, limits): (u32, &Limits)) -> Self {
        let (mut lo, mut hi) = (vec![0; c::WORDS], vec![0; c::WORDS]);
        // Read only the seed: the first cycles' counts, a light's lattice
        // above all, owe nothing to it.
        let least = if prev.is_none() { 256 } else { 0 };
        for w in 0..c::WORDS {
            // A table's count by its own cycle's growth, its start against
            // its end, which a compaction between two reads does not hide.
            let (before, now) = match within(w) {
                Some((start, end)) => (read[start], read[end]),
                None => (prev.map_or(0, |p| p[w]), read[w]),
            };
            let (step, most) = (now.abs_diff(before), now.max(before));
            // Two cycles on at the rate it grew (fast while the tree is
            // young), or three of its last steps (as it slows).
            let rate = (f64::from(now) / f64::from(before.max(1))).clamp(1.0, 4.0);
            let grown = (f64::from(most) * (rate * rate - 1.0)).min(f64::from(u32::MAX)) as u32;
            let past = if grows(w) { 64 } else { 64 + most / 8 };
            let reach = step.saturating_mul(3).max(grown).saturating_add(past).saturating_mul(WIDTH);
            // A table's count at least `FLOOR` within its rows: a young
            // tree's counts outgrow any projection (the lattice's nodes at
            // cycle 8, five times in two cycles), and cost little below it.
            let floor = if ceiling[w] == u32::MAX { least } else { least.max(FLOOR) };
            hi[w] = most.saturating_add(reach).max(floor).min(ceiling[w]);
            // After a compaction, a table's counts start from what it kept:
            // its count at the end of the compacted cycle.
            let kept = match (cut, within(w)) {
                (Some(cut), Some((_, end))) => cut[end],
                (Some(cut), None) => cut[w],
                (None, _) => read[w],
            };
            lo[w] = if grows(w) { kept.min(read[w]).min(hi[w]) } else { 0 };
        }
        let nodes = c::GRID + 6;
        hi[nodes] = hi[nodes].max(boxed(read, prev)).min(ceiling[nodes]);
        // What a cycle's units and wake make, the axes, phytomers, units
        // and sleeping buds (`plan_made`'s order), is each table's end less
        // its start, held as the table is (fn-276, host 2026-10-14): never
        // a projection of its own, which a burst passed (the beech at 26,
        // seed 1; a walk frame's cycle 5; the oak's new axes at 9, seed 1,
        // fn-286: 224, 270, 372 then 591 a cycle).
        for (k, (end, start)) in [(c::AXES, c::AXES0), (c::PHYTS, c::PHYTS0), (c::UNITS, c::UNITS0), (c::SLEEPERS, c::SLEEPERS0)].into_iter().enumerate() {
            let made = hi[end].saturating_sub(lo[start]);
            for w in [c::MADE + T_UNITS + k, c::MADE + T_WAKE + k] {
                (lo[w], hi[w]) = (0, made);
            }
        }
        (lo[c::CYCLE], hi[c::CYCLE]) = (cycle, cycle);
        for (k, v) in limits.words().into_iter().enumerate() {
            (lo[REC + k], hi[REC + k]) = (v, v);
        }
        Self { lo, hi }
    }

    /// The most `e` reaches.
    fn most(&self, e: &E) -> u32 {
        e.span(&self.lo, &self.hi).1
    }
}

/// The bounded sizes the cycle's recorded steps take, each with whether
/// it sizes a dispatch (its workgroups) or a clear (its rows).
fn recorded<'a>(steps: &'a [&[Gated]], limits: &'a Limits) -> impl Iterator<Item = (u32, bool)> + 'a {
    steps.iter().flat_map(|s| s.iter()).filter_map(move |g| match g.step {
        Step::Dispatch { size: Size::Bounded(k), .. } if g.gate.iter().all(|c| limits.allows(c)) => Some((k, true)),
        Step::Zeroed { size: Size::Bounded(k), .. } if g.gate.iter().all(|c| limits.allows(c)) => Some((k, false)),
        _ => None,
    })
}

/// Each bounded size's workgroups this cycle: the most its expression
/// reaches where a dispatch the cycle records takes it, and past any
/// count (never flagged) where none does.
pub(super) fn given(plan: &Plan, span: &Span, (steps, limits): (&[&[Gated]], &Limits)) -> Vec<u32> {
    let mut out = vec![u32::MAX; plan.sized.len()];
    for (k, _) in recorded(steps, limits) {
        out[k as usize] = span.most(&plan.sized[k as usize]);
    }
    out
}

/// The workgroups a build was given and those its true counts needed,
/// tallied from each cycle's census as it is read.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Over {
    pub given: u64,
    pub needed: u64,
}

/// A cycle's recorded bounded dispatches, until its census is read.
pub(super) struct Pending {
    sized: Rc<Vec<E>>,
    dispatched: Vec<(u32, u32, bool)>,
}

impl Pending {
    pub(super) fn new(plan: &Plan, steps: &[&[Gated]], (limits, given): (&Limits, &[u32])) -> Self {
        let dispatched = recorded(steps, limits).map(|(k, d)| (k, given[k as usize], d)).collect();
        Self { sized: plan.sized.clone(), dispatched }
    }

    /// Each dispatch given fewer workgroups than `census` asks: its size's
    /// expression, what it was given and what it needed.
    pub(super) fn short(&self, census: &[u32]) -> Vec<String> {
        (self.dispatched.iter())
            .filter(|&&(k, given, _)| self.sized[k as usize].eval(census) > given)
            .map(|&(k, given, _)| format!("{:?} given {given} needed {}", self.sized[k as usize], self.sized[k as usize].eval(census)))
            .collect()
    }

    /// Its dispatches' workgroups tallied against the cycle's `census`.
    pub(super) fn tally(&self, over: &mut Over, census: &[u32]) {
        for &(k, given, _) in self.dispatched.iter().filter(|d| d.2) {
            over.given += u64::from(given);
            over.needed += u64::from(self.sized[k as usize].eval(census).min(given));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::build_plan::Bounds;
    use super::*;

    /// A level's width as the sums' sweeps size it, where its first block
    /// fell between the census read and the cycle (Telperion 82 seed 42,
    /// whose level starts fall every few cycles).
    #[test]
    fn a_levels_width_is_bounded_where_its_first_block_falls() {
        let (mut prev, mut read, mut now) = (vec![0; c::WORDS], vec![0; c::WORDS], vec![0; c::WORDS]);
        (prev[c::LVL], prev[c::LVL + 1]) = (50, 200);
        (read[c::LVL], read[c::LVL + 1]) = (100, 200);
        (now[c::LVL], now[c::LVL + 1]) = (10, 250);
        let width = E::At(c::LVL + 1).sub(E::At(c::LVL));
        let span = Span::new(&read, Some(&prev), (&vec![u32::MAX; c::WORDS], None), (3, &Limits::full(&Bounds::default())));
        assert!(span.most(&width) >= width.eval(&now), "{} under {}", span.most(&width), width.eval(&now));
    }

    /// fn-276: the beech at 26, seed 1, made more phytomers in a cycle
    /// than its own last two cycles projected; the dispatch over them is
    /// given what its phytomer table's span holds, so a burst within the
    /// table is within it.
    #[test]
    fn a_burst_of_made_phytomers_is_within_its_table() {
        let (mut prev, mut read) = (vec![0; c::WORDS], vec![0; c::WORDS]);
        let at = |w: usize| c::MADE + w;
        (prev[c::PHYTS0], prev[c::PHYTS], prev[at(T_UNITS + 1)]) = (100_000, 105_000, 5_000);
        (read[c::PHYTS0], read[c::PHYTS], read[at(T_UNITS + 1)]) = (105_000, 110_000, 5_000);
        let mut ceiling = vec![u32::MAX; c::WORDS];
        (ceiling[c::PHYTS], ceiling[c::PHYTS0]) = (200_000, 200_000);
        let made = E::At(at(T_UNITS + 1)).add(E::At(at(T_WAKE + 1))).div(E::C(64));
        let span = Span::new(&read, Some(&prev), (&ceiling, None), (3, &Limits::full(&Bounds::default())));
        let mut now = read.clone();
        (now[at(T_UNITS + 1)], now[at(T_WAKE + 1)]) = (8_000, 600);
        assert!(span.most(&made) >= made.eval(&now), "{} under {}", span.most(&made), made.eval(&now));
    }

    /// fn-286: the oak at 9, seed 1, with its sourced sapling segment,
    /// made 224, 270, 372 then 591 new axes a cycle; the dispatch over
    /// them, given from the cycle 7 read, was given 9 workgroups for the
    /// 10 it needed. It is given what its axis table's span holds.
    #[test]
    fn a_burst_of_made_axes_is_within_its_table() {
        let (mut prev, mut read) = (vec![0; c::WORDS], vec![0; c::WORDS]);
        let at = |w: usize| c::MADE + w;
        (prev[c::AXES0], prev[c::AXES], prev[at(T_UNITS)]) = (282, 506, 224);
        (read[c::AXES0], read[c::AXES], read[at(T_UNITS)]) = (506, 776, 270);
        let mut ceiling = vec![u32::MAX; c::WORDS];
        (ceiling[c::AXES], ceiling[c::AXES0]) = (4096, 4096);
        let made = E::At(at(T_UNITS)).add(E::At(at(T_WAKE))).div(E::C(64));
        let span = Span::new(&read, Some(&prev), (&ceiling, None), (9, &Limits::full(&Bounds::default())));
        let mut now = read.clone();
        now[at(T_UNITS)] = 591;
        assert!(span.most(&made) >= made.eval(&now), "{} under {}", span.most(&made), made.eval(&now));
    }
}
