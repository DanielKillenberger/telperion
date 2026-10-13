//! How a build is held to its sizes as it grows (DESIGN.md section 5, the
//! host's rulings of 2026-10-12): each cycle's limits from the bounds its
//! tables state (`build_arena.rs`) and, for the few that step with the
//! tree, from the census read a cycle late; a table its hint undersized
//! doubled once before it overflows; and a count past any limit or table
//! refused by name, never begun again.
use super::build_grow::{Compact, Sizes};
use super::build_light::refusals;
use super::census::{self as c, Flag, PASSES, PASS_LEFT};
use super::gate::Limits;
use super::lay_device::DEEP;
use super::plan_units::rounds_used;
use crate::RenderError;

/// The limits that step with the tree, each past its projection two
/// cycles on: the hierarchy's generations and placing passes, the
/// layout's generations and the sweep's layer ranges. None of them ever
/// passed these on a shipped preset (`arena_tests`, 2026-10-12); one that
/// does is refused by name. The block levels need none: the deepest are
/// folded, however many (`fold.wgsl`).
const STEP: [u32; 4] = [2, 1, 2, 2];

/// The units rounds a cycle records past those the census read used,
/// the living apexes' and the wake's: the most either rose over the two
/// cycles a limit looks ahead, every shipped preset at its default age
/// and seeds 1, 7 and 42 (the ordinary and the birch 2, the ordinary's
/// wake 3; `arena_tests`, fn-256 task 12). A job left past them is
/// flagged (`Flag::UnitRounds`) and the build refused by name.
const ROUND_SLACK: [u32; 2] = [2, 3];

/// The blocks a folded sweep's workgroup takes at once (`fold.wgsl`'s
/// `FOLD`): a level of no more is folded.
const FOLD: u32 = 8;

/// The block levels a sweep folds, by their widths in the census read:
/// from the top while each is narrow, and the deepest up while each is;
/// the levels between are dispatched one by one.
fn folds(read: &[u32], most: u32) -> (u32, u32) {
    let levels = read[c::LEVELS].min(most);
    let narrow = |l: u32| read[c::LVL + l as usize + 1].saturating_sub(read[c::LVL + l as usize]) <= FOLD;
    let top = (0..levels).take_while(|&l| narrow(l)).count() as u32;
    let tail = (top..levels).rev().take_while(|&l| narrow(l)).last().unwrap_or(levels);
    (top, tail)
}

/// A count past a table or limit, by name: the build stops there.
pub(super) fn refusal(cycle: u32, flags: &[(Flag, u32)], budget: u32) -> RenderError {
    let say = |what: String| super::super::io::error(what);
    if flags.iter().any(|f| f.0 == Flag::Budget) {
        return say(format!("the tree grows past the engine's budget of {budget} phytomers"));
    }
    if let Some(&(_, value)) = flags.iter().find(|f| f.0 == Flag::LightErrors) {
        return say(format!("the light refused at cycle {cycle}: {:?}", refusals(value)));
    }
    let named: Vec<String> = flags.iter().map(|f| format!("{:?} ({:#x})", f.0, f.1)).collect();
    say(format!("the build passed its sizes at cycle {cycle}: {}", named.join(", ")))
}

/// The layer ranges a sweep over a grid of `size` cells takes.
fn sweeps_of(size: [u32; 3]) -> u32 {
    super::build_light::sweeps_of(size[2])
}

/// The grid's size in the census's cycle.
fn grid(census: &[u32]) -> [u32; 3] {
    [census[c::GRID + 3], census[c::GRID + 4], census[c::GRID + 5]]
}

/// The layer ranges the sweep may take two cycles on: each of the grid's
/// sides grown by three of its last steps, two cells at least. A sweep's
/// share of layers steps down as its layer's area grows, so its count
/// jumps where the grid grows little (Telperion 82: 29 to 54 in a cycle).
fn sweeps_ahead(read: &[u32], prev: Option<&[u32]>) -> u32 {
    let (now, before) = (grid(read), prev.map_or([0; 3], grid));
    sweeps_of(std::array::from_fn(|k| now[k] + (3 * now[k].saturating_sub(before[k])).max(2)))
}

/// The sizes after doubling each table the next two cycles could pass,
/// from `read` and the cycle before it (`prev`); none where `s` holds
/// them: where its rows two cycles on at the rate they grew pass it, or,
/// for a table the arena compacts, where the rows its compaction since
/// the census kept (`cut`) would pass seven eighths of it. No phytomer
/// table grows past the engine's `budget`.
pub(super) fn grown(s: Sizes, (read, prev, cut): (&[u32], Option<&[u32]>, Option<&[u32]>), budget: u32) -> Option<Sizes> {
    let held = super::build_arena::tables(read);
    let kept = cut.map(super::build_arena::tables);
    let before = prev.map(super::build_arena::tables);
    // A table's growth by its own cycle (its start against its end, which
    // compaction does not hide), else against the census before.
    let starts = [c::AXES0, c::UNITS0, c::PHYTS0, c::SLEEPERS0];
    let compacted = s.compact != Compact::Off;
    let near = |k: usize, cap: u32| {
        // A table the arena compacts by the rows its last compaction kept;
        // none until one has (its compaction comes first).
        let now = match (compacted && k <= 2, kept) {
            (false, _) => f64::from(held[k]),
            (true, Some(kept)) => f64::from(kept[k]),
            (true, None) => return false,
        };
        let end = f64::from(held[k]);
        let start = match k {
            0..=3 => f64::from(read[starts[k]]),
            5 => f64::from(read[c::WOKEN0]),
            _ => before.map_or(end / 2.0, |b| f64::from(b[k])),
        };
        let r = (end / start.max(1.0)).clamp(1.0, 4.0);
        let room = if compacted && k <= 2 { 7.0 / 8.0 } else { 1.0 };
        now * r * r > room * f64::from(cap)
    };
    let twice = |k: usize, cap: u32| if near(k, cap) { cap.saturating_mul(2) } else { cap };
    let mut n = s;
    let caps = &mut n.caps;
    caps.axes = twice(0, caps.axes).next_multiple_of(64);
    caps.units = twice(1, caps.units);
    caps.phyts = twice(2, caps.phyts).min(budget).max(s.caps.phyts);
    caps.sleepers = twice(3, caps.sleepers);
    caps.next = twice(4, caps.next).next_multiple_of(64);
    caps.woken = twice(5, caps.woken);
    let leaves = n.points - n.tips;
    n.tips = twice(4, n.tips);
    n.points = twice(6, leaves) + n.tips;
    n.nodes = twice(7, n.nodes);
    if n == s {
        return None;
    }
    Some(Sizes { bounds: n.stated(), ..n })
}

/// The passes a round took in the census's cycle at most: the first, and
/// each after a pass that left an axis unplaced.
pub(super) fn passes(read: &[u32]) -> u32 {
    (0..read[c::GENS].min(c::ROUNDS as u32) as usize)
        .map(|r| 1 + (0..PASSES).filter(|p| read[PASS_LEFT + PASSES * r + p] > 0).count() as u32)
        .max()
        .unwrap_or(1)
}

/// The cycle's limits: each list's ranges past its count two cycles on,
/// twice over; the hierarchy's generations, passes and levels, the
/// layout's generations and the sweep's layer ranges past their counts by
/// their `STEP`; and the units rounds past those used by their
/// `ROUND_SLACK`; from the census read a cycle late (`read`, `prev` the
/// one before); each no further than its bound, the bounds where nothing
/// has been read.
pub(super) fn limits(s: &Sizes, read: Option<&[u32]>, prev: Option<&[u32]>) -> Limits {
    let b = &s.bounds;
    let full = Limits::full(b);
    // The first two cycles, read nothing yet, record every loop at its
    // bound.
    let Some(read) = read else {
        return full;
    };
    let deepest = (0..DEEP as usize).filter(|&g| read[c::LAY_DEEP + g] != 0).count() as u32;
    // The lists' ranges: the living, the grown axes, the cycle's leaves and
    // the lattice two cycles on at the rate they grew, twice over.
    let ahead = |w: usize, x: u32| {
        let r = prev.map_or(2.0, |p| (f64::from(read[w]) / f64::from(p[w].max(1))).clamp(1.0, 4.0));
        (2.0 * f64::from(x) * r * r).ceil() as u32
    };
    let born = read[c::AXES].saturating_sub(read[c::AXES0]);
    let live = 2 * read[c::NEXT] + 2 * ahead(c::AXES, born);
    let lit = ahead(c::PHYTS, read[c::PHYTS].saturating_sub(read[c::PHYTS0]));
    let r = b.ranges;
    let (top, tail) = folds(read, b.hier.levels);
    Limits {
        units: live.div_ceil(r.units),
        draws: live.div_ceil(r.draw),
        deposits: lit.div_ceil(r.deposit),
        sweeps: sweeps_ahead(read, prev) + STEP[3],
        rounds: read[c::GENS] + STEP[0],
        passes: passes(read) + STEP[1],
        top,
        tail,
        laygens: deepest + STEP[2],
        unit_rounds: std::array::from_fn(|w| rounds_used(read, w) + ROUND_SLACK[w]),
    }
    .within(b)
}

#[cfg(test)]
mod tests {
    use super::super::device::T_USED;
    use super::*;

    /// A census whose phytomers grew from `before` to `now`.
    fn census(before: u32, now: u32) -> (Vec<u32>, Vec<u32>) {
        let (mut prev, mut read) = (vec![0; c::WORDS], vec![0; c::WORDS]);
        (prev[c::PHYTS], read[c::PHYTS]) = (before, now);
        (prev, read)
    }

    /// Astra's finding: a phytomer table that doubles grows no further
    /// than the engine's budget.
    #[test]
    fn no_phytomer_table_grows_past_the_budget() {
        let budget = 20_000_000;
        let (prev, read) = census(10_000_000, 19_000_000);
        let mut s = Sizes::least();
        s.caps.phyts = 16_000_000;
        let next = grown(s, (&read, Some(&prev), None), budget).expect("a move");
        assert_eq!(next.caps.phyts, budget);
    }

    /// fn-256 task 12: a cycle records the units rounds the census read
    /// used and their slack, the living apexes' and the wake's apart, no
    /// further than the plan's bound; a job past them is refused by name.
    #[test]
    fn units_rounds_follow_the_rounds_used() {
        let s = Sizes::least();
        let mut read = vec![0; c::WORDS];
        (read[c::MADE + T_USED], read[c::MADE + T_USED + 1]) = (3, 0);
        assert_eq!(limits(&s, Some(&read), None).unit_rounds, [3 + ROUND_SLACK[0], ROUND_SLACK[1]]);
        read[c::MADE + T_USED] = 64;
        assert_eq!(limits(&s, Some(&read), None).unit_rounds[0], s.bounds.rounds);
        assert_eq!(limits(&s, None, None).unit_rounds, [s.bounds.rounds; 2]);
        let said = refusal(9, &[(Flag::UnitRounds, 1)], 20_000_000);
        assert!(said.to_string().contains("UnitRounds"), "{said}");
    }

    /// A count past a table or limit stops the build by name.
    #[test]
    fn a_build_past_the_budget_is_refused_by_name() {
        let said = refusal(9, &[(Flag::Budget, 1), (Flag::Phyts, 1)], 20_000_000);
        assert!(said.to_string().contains("budget of 20000000 phytomers"), "{said}");
        let said = refusal(9, &[(Flag::Sweeps, 1)], 20_000_000);
        assert!(said.to_string().contains("Sweeps"), "{said}");
    }

    /// Codex's lattice (fn-256 task 15): 2 x 2 x 1,000 nodes fits the
    /// least arena's 4,096 and takes 63 ranges of 16 layers. The first
    /// cycles and a cycle that read it record them, on the least arena
    /// and on a large one.
    #[test]
    fn a_tall_narrow_lattice_is_swept() {
        let tall = [2, 2, 1000];
        let mut read = vec![0; c::WORDS];
        read[c::GRID + 3..c::GRID + 6].copy_from_slice(&tall);
        let mut large = Sizes::least();
        large.nodes = 1 << 22;
        large.bounds = large.stated();
        for s in [Sizes::least(), large] {
            for (now, prev) in [(None, None), (Some(&read[..]), Some(&read[..]))] {
                let sweeps = limits(&s, now, prev).sweeps;
                assert!(sweeps_of(tall) <= sweeps, "{} nodes: {sweeps} ranges", s.nodes);
            }
        }
    }
}
