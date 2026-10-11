//! The workgroups the host gives each dispatch whose size only the
//! device knows (DESIGN.md section 7a: direct dispatches only). The host
//! bounds every census word from the census it read a cycle late and the
//! one before: a count that only grows is at least what it was read, and
//! every count at most what it was plus the more of two cycles at the
//! rate it grew and three times its last step (and an eighth, for a
//! count that also falls), that reach times the word's width. Each
//! dispatch is given the most its expression reaches over those bounds;
//! its invocations exit past the true count, and the device flags a
//! cycle where it was given too few (`Flag::Given`), so the build begins
//! again with the words that dispatch reads widened.
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

/// How far past its projection each census word reaches: a power of two,
/// doubled each time a dispatch that reads it was given too few.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct Widths([u8; c::WORDS]);

impl Default for Widths {
    fn default() -> Self {
        Self([0; c::WORDS])
    }
}

impl std::fmt::Debug for Widths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.0.iter().enumerate().filter(|w| *w.1 > 0).map(|(w, s)| (w, 1u32 << s))).finish()
    }
}

/// Each census word's least and most this cycle.
pub(super) struct Span {
    lo: Vec<u32>,
    hi: Vec<u32>,
}

impl Span {
    /// From `read` and the census before it (none where only the seed's
    /// was read), under `widths`; the cycle and its limits exact.
    pub(super) fn new(read: &[u32], prev: Option<&[u32]>, widths: &Widths, (cycle, limits): (u32, &Limits)) -> Self {
        let (mut lo, mut hi) = (vec![0; c::WORDS], vec![0; c::WORDS]);
        // Read only the seed: the first cycles' counts, a light's lattice
        // above all, owe nothing to it.
        let least = if prev.is_none() { 256 } else { 0 };
        for w in 0..c::WORDS {
            let before = prev.map_or(0, |p| p[w]);
            let (step, most) = (read[w].abs_diff(before), read[w].max(before));
            // Two cycles on at the rate it grew (fast while the tree is
            // young), or three of its last steps (as it slows).
            let rate = (f64::from(read[w]) / f64::from(before.max(1))).clamp(1.0, 4.0);
            let grown = (f64::from(most) * (rate * rate - 1.0)).min(f64::from(u32::MAX)) as u32;
            let past = if grows(w) { 64 } else { 64 + most / 8 };
            let reach = step.saturating_mul(3).max(grown).saturating_add(past).saturating_mul(1 << widths.0[w]);
            hi[w] = most.saturating_add(reach).max(least);
            lo[w] = if grows(w) { read[w] } else { 0 };
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

    /// Tallied against the cycle's `census`.
    pub(super) fn tally(&self, over: &mut Over, census: &[u32]) {
        for &(k, given) in &self.dispatched {
            over.given += u64::from(given);
            over.needed += u64::from(self.sized[k as usize].eval(census).min(given));
        }
    }

    /// The words of each dispatch given fewer workgroups than `census`
    /// asks, widened; every word where none was (its census unread).
    pub(super) fn widen(&self, widths: &mut Widths, census: &[u32]) {
        let mut words = Vec::new();
        for &(k, given) in &self.dispatched {
            let e = &self.sized[k as usize];
            if e.eval(census) > given {
                e.words(&mut words);
            }
        }
        if words.is_empty() {
            words.extend(0..c::WORDS);
        }
        words.sort_unstable();
        words.dedup();
        for w in words {
            widths.0[w] = (widths.0[w] + 2).min(16);
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
        let span = Span::new(&read, Some(&prev), &Widths::default(), (3, &Limits::full(&Bounds::default())));
        assert!(span.most(&width) >= width.eval(&now), "{} under {}", span.most(&width), width.eval(&now));
    }
}
