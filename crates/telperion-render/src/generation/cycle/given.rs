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
        // The lattice's nodes grow as its box's volume: each side grown by
        // three of its last steps, two cells at least, twice over.
        let side = |k: usize| {
            let now = read[c::GRID + 3 + k];
            let step = now.saturating_sub(prev.map_or(0, |p| p[c::GRID + 3 + k]));
            u64::from(now + (3 * step).max(2))
        };
        let boxed = (2 * side(0) * side(1) * side(2)).min(u64::from(u32::MAX)) as u32;
        let nodes = c::GRID + 6;
        hi[nodes] = hi[nodes].max(boxed).min(ceiling[nodes]);
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

/// The bounded sizes the cycle's recorded dispatches take.
fn recorded<'a>(steps: &'a [&[Gated]], limits: &'a Limits) -> impl Iterator<Item = u32> + 'a {
    steps.iter().flat_map(|s| s.iter()).filter_map(move |g| match g.step {
        Step::Dispatch { size: Size::Bounded(k), .. } if g.gate.iter().all(|c| limits.allows(c)) => Some(k),
        _ => None,
    })
}

/// Each bounded size's workgroups this cycle: the most its expression
/// reaches where a dispatch the cycle records takes it, and past any
/// count (never flagged) where none does.
pub(super) fn given(plan: &Plan, span: &Span, (steps, limits): (&[&[Gated]], &Limits)) -> Vec<u32> {
    let mut out = vec![u32::MAX; plan.sized.len()];
    for k in recorded(steps, limits) {
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
    dispatched: Vec<(u32, u32)>,
}

impl Pending {
    pub(super) fn new(plan: &Plan, steps: &[&[Gated]], (limits, given): (&Limits, &[u32])) -> Self {
        let dispatched = recorded(steps, limits).map(|k| (k, given[k as usize])).collect();
        Self { sized: plan.sized.clone(), dispatched }
    }

    /// Each dispatch given fewer workgroups than `census` asks: its size's
    /// expression, what it was given and what it needed.
    pub(super) fn short(&self, census: &[u32]) -> Vec<String> {
        (self.dispatched.iter())
            .filter(|&&(k, given)| self.sized[k as usize].eval(census) > given)
            .map(|&(k, given)| format!("{:?} given {given} needed {}", self.sized[k as usize], self.sized[k as usize].eval(census)))
            .collect()
    }

    /// Tallied against the cycle's `census`.
    pub(super) fn tally(&self, over: &mut Over, census: &[u32]) {
        for &(k, given) in &self.dispatched {
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
}
