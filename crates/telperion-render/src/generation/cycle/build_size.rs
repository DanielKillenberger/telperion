//! How a build is sized as it grows (DESIGN.md section 5): each cycle's
//! limits from the census read a cycle late, the sizes the next two
//! cycles need, and the sizes a flag raises when the build begins again.
//! A limit or bound costs empty dispatches where it reaches past what a
//! cycle needs; one short begins the build again from the seed, so every
//! margin here is measured against the presets (fn-256 task 4).
use super::build_grow::Sizes;
use super::build_light::refusals;
use super::build_plan::Bounds;
use super::census::{self as c, Flag, PASSES, PASS_LEFT};
use super::gate::{levels_of, rec, Limits};
use super::lay_device::DEEP;
use super::light_program::{SWEEP_WG, SWEEP_WORK};
use crate::Result;

fn twice(have: u32, count: u32) -> u32 {
    have.max(count).saturating_mul(2)
}

/// Whether `flag` names a fault (or the budget) rather than a size.
fn fault(flag: Flag) -> bool {
    matches!(flag, Flag::Budget | Flag::UnitFaults | Flag::HierFault | Flag::LayFault | Flag::Collapsed)
}

/// The limits' slacks, by what each limits.
pub(super) const SLACKS: usize = 10;
const S_LIVE: usize = 0;
const S_DEPOSITS: usize = 1;
const S_LIGHTS: usize = 2;
const S_SWEEPS: usize = 3;
const S_ROUNDS: usize = 4;
const S_PASSES: usize = 5;
const S_LEVELS: usize = 6;
const S_LAYGENS: usize = 7;
const S_SCANS: usize = 8;
const S_UNIT_ROUNDS: usize = 9;

/// The slacks a build starts from: one past the projection, two for the
/// hierarchy's generations, the layout's and the units' rounds, which
/// jump by one a cycle where nothing foretells it (Telperion 82 at 51).
/// A recorded round past what a cycle needs costs an empty dispatch; one
/// short begins the build again.
pub(super) const SLACK: [u32; SLACKS] = {
    let mut s = [1; SLACKS];
    (s[S_ROUNDS], s[S_LAYGENS], s[S_UNIT_ROUNDS]) = (2, 2, 2);
    s
};

/// The slack a flag names where its count passed the cycle's limit below
/// the plan's bound (a wider slack answers it); none where it passed the
/// bound.
fn short(flag: Flag, census: &[u32], b: &Bounds) -> Option<usize> {
    let at = |w: usize, bound: u32, k: usize| (census[w] < bound).then_some(k);
    match flag {
        Flag::UnitRanges => at(rec::UNITS, b.units, S_LIVE),
        Flag::DrawRanges => at(rec::DRAWS, b.lay.draws, S_LIVE),
        Flag::DepositRanges => at(rec::DEPOSITS, b.lay.deposits, S_DEPOSITS),
        Flag::LightRanges => at(rec::LIGHTS, b.lay.lights, S_LIGHTS),
        Flag::Sweeps => at(rec::SWEEPS, b.lay.sweeps, S_SWEEPS),
        Flag::HierGens => at(rec::ROUNDS, b.hier.gens, S_ROUNDS),
        Flag::UnitRounds => (census[rec::UNIT_ROUNDS].min(census[rec::UNIT_ROUNDS + 1]) < b.rounds).then_some(S_UNIT_ROUNDS),
        Flag::HierPasses => at(rec::ROUNDS + 1, b.hier.passes, S_PASSES),
        Flag::HierLevels => at(rec::LEVELS, b.hier.levels, S_LEVELS),
        Flag::LayGenerations => at(rec::LAYGENS, b.lay.generations, S_LAYGENS),
        Flag::HierScans | Flag::LiveScan | Flag::GrewScan | Flag::UnitScan | Flag::SleepScan | Flag::LayScan => Some(S_SCANS),
        _ => None,
    }
}

/// `s` raised for every flag in `flags`, from `census`; refused where a
/// flag names a fault or the budget and no size passed (a size passed
/// can fault what reads past it). A count past only the cycle's limit
/// widens that limit's slack.
pub(super) fn raised(mut s: Sizes, flags: &[(Flag, u32)], census: &[u32], budget: u32) -> Result<Sizes> {
    let refused = |what: String| super::super::io::error(what);
    // Past the engine's budget the build is refused by name, whatever
    // else the cycle passed: no table grows past it.
    if flags.iter().any(|f| f.0 == Flag::Budget) {
        return Err(refused(format!("the tree grows past the engine's budget of {budget} phytomers")));
    }
    let sized = flags.iter().any(|f| !fault(f.0) && !(f.0 == Flag::LightErrors && f.1 != 4));
    let bounds = s.bounds;
    for k in flags.iter().filter_map(|f| short(f.0, census, &bounds)) {
        s.slack[k] *= 2;
    }
    for &(flag, value) in flags.iter().filter(|f| !(sized && fault(f.0)) && short(f.0, census, &bounds).is_none()) {
        let (caps, b) = (&mut s.caps, &mut s.bounds);
        match flag {
            Flag::Axes => caps.axes = twice(caps.axes, census[c::AXES]).next_multiple_of(64),
            Flag::DrawRanges => b.lay.draws = census[c::GREW].div_ceil(b.ranges.draw) + 1,
            Flag::UnitRanges => b.units = census[c::LIVE].div_ceil(b.ranges.units) + 1,
            Flag::UnitRounds => b.rounds = (2 * b.rounds).min(64),
            Flag::LightRanges => b.lay.lights *= 2,
            Flag::Units => caps.units = twice(caps.units, census[c::UNITS]),
            Flag::Phyts => caps.phyts = twice(caps.phyts, census[c::PHYTS]).min(budget),
            Flag::Sleepers => caps.sleepers = twice(caps.sleepers, census[c::SLEEPERS]),
            Flag::Next => caps.next = twice(caps.next, census[c::NEXT]).next_multiple_of(64),
            Flag::Woken => caps.woken = twice(caps.woken, census[c::WOKEN]),
            Flag::HierGens => b.hier.gens = twice(b.hier.gens, census[c::GENS]),
            Flag::HierPasses => b.hier.passes *= 2,
            Flag::HierLevels => b.hier.levels = (census[c::LEVELS] + 6).min(63),
            Flag::LayGenerations if b.lay.generations < 15 => b.lay.generations = (2 * b.lay.generations).min(15),
            Flag::DepositRanges => b.lay.deposits = leaves(census).div_ceil(b.ranges.deposit) + 1,
            Flag::Sweeps => b.lay.sweeps *= 2,
            Flag::Points => (s.points, s.tips) = (twice(s.points, leaves(census) + census[c::NEXT]), twice(s.tips, census[c::NEXT])),
            Flag::LightErrors if value == 4 => s.nodes = twice(s.nodes, census[c::GRID + 6]),
            // Widened by the grow loop, which holds the cycle's dispatches.
            Flag::Given => {}
            Flag::LightErrors => return Err(refused(format!("the light refused: {:?}", refusals(value)))),
            _ => return Err(refused(format!("the build stopped: {flag:?} ({value:#x})"))),
        }
    }
    Ok(s)
}

fn leaves(census: &[u32]) -> u32 {
    census[c::PHYTS] - census[c::PHYTS0]
}

/// The layer ranges a sweep over a grid of `size` cells takes.
fn sweeps_of(size: [u32; 3]) -> u32 {
    let per = (SWEEP_WORK / (size[0] * size[1]).div_ceil(SWEEP_WG).max(1)).max(1);
    size[2].div_ceil(per)
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

/// The sizes the next two cycles need, from `read` and the cycle before
/// it (`prev`) at the rate each count grew; none where `s` holds them.
pub(super) fn ahead(s: Sizes, read: &[u32], prev: Option<&[u32]>, budget: u32) -> Option<Sizes> {
    let ahead = |w: usize, x: u32| -> u32 {
        let r = prev.map_or(2.0, |p| (f64::from(read[w]) / f64::from(p[w].max(1))).clamp(1.0, 4.0));
        (f64::from(x) * r * r).ceil() as u32
    };
    // A table takes a quarter over its projection, no more: the Two Trees'
    // peak (6.1 GB) is at the page's 6 GiB, and half again on tables under
    // 2^20 or even 2^17 rows put Telperion or Laurelin out of device
    // memory in the repeat test (fn-256 task 4).
    let fit = |have: u32, need: u32| if need > have { need + need / 4 } else { have };
    // A small table (sleeping, living and woken buds) moves when its
    // quarter over the projection would pass its rows, and doubles. The
    // large tables and the light's points cannot spare the memory:
    // Telperion's peak is 6.1 GB of the page's 6 GiB.
    let small = |have: u32, need: u32| {
        let want = need + need / 4;
        if want > have { want.max(2 * have) } else { have }
    };
    let mut n = s;
    let caps = &mut n.caps;
    caps.axes = fit(caps.axes, ahead(c::AXES, read[c::AXES])).next_multiple_of(64);
    caps.units = fit(caps.units, ahead(c::UNITS, read[c::UNITS]));
    // No build grows phytomers past its budget (`Flag::Budget`), so no
    // table holds more.
    let need = ahead(c::PHYTS, read[c::PHYTS]);
    caps.phyts = fit(caps.phyts, need).min(budget).max(caps.phyts);
    caps.sleepers = small(caps.sleepers, ahead(c::SLEEPERS, read[c::SLEEPERS]));
    // The living after a cycle are at most those before and the cycle's
    // new axes, which grow as the axes do.
    let born = read[c::AXES] - read[c::AXES0];
    let next = read[c::NEXT] + 2 * ahead(c::AXES, born);
    caps.next = small(caps.next, next.max(ahead(c::NEXT, read[c::NEXT]))).next_multiple_of(64);
    caps.woken = small(caps.woken, ahead(c::WOKEN, read[c::WOKEN]));
    let lit = ahead(c::PHYTS, leaves(read));
    n.tips = fit(n.tips, ahead(c::NEXT, read[c::NEXT]));
    n.points = fit(n.points, lit + n.tips);
    n.nodes = fit(n.nodes, 2 * read[c::GRID + 6]);
    // A bound a count passes is raised by half again, so the plan is
    // recorded anew a few times, not each cycle; a bound costs nothing
    // past the cycle's limit, which the census sets.
    let raise = |have: u32, need: u32| if need > have { need.max(have + have.div_ceil(2)) } else { have };
    let b = &mut n.bounds;
    b.units = raise(b.units, next.div_ceil(b.ranges.units));
    b.lay.draws = raise(b.lay.draws, next.div_ceil(b.ranges.draw));
    // The lattice's nodes, and over the sky's nine directions at most.
    let nodes = 2 * read[c::GRID + 6];
    b.lay.lights = raise(b.lay.lights, (9 * nodes).div_ceil(b.ranges.paths).max(nodes.div_ceil(b.ranges.nodes)));
    b.lay.deposits = raise(b.lay.deposits, lit.div_ceil(b.ranges.deposit));
    b.lay.sweeps = raise(b.lay.sweeps, sweeps_ahead(read, prev) + 2);
    b.hier.levels = raise(b.hier.levels, read[c::LEVELS] + 3).min(63);
    b.hier.gens = raise(b.hier.gens, read[c::GENS] + 2);
    let deepest = (0..DEEP as usize).rev().find(|&g| read[c::LAY_DEEP + g] != 0).unwrap_or(0) as u32;
    b.lay.generations = raise(b.lay.generations, deepest + 3).min(15);
    (n != s).then_some(n)
}

/// The passes a round took in the census's cycle at most: the first, and
/// each after a pass that left an axis unplaced.
fn passes(read: &[u32]) -> u32 {
    (0..read[c::GENS].min(c::ROUNDS as u32) as usize)
        .map(|r| 1 + (0..PASSES).filter(|p| read[PASS_LEFT + PASSES * r + p] > 0).count() as u32)
        .max()
        .unwrap_or(1)
}

/// The cycle's limits from the census read a cycle late (`read`), and
/// the one before (`prev`): each count two cycles on at the rate it grew,
/// past it by its slack; the plan's bounds where nothing has been read.
pub(super) fn limits(s: &Sizes, read: Option<&[u32]>, prev: Option<&[u32]>) -> Limits {
    let b = &s.bounds;
    let Some(read) = read else {
        return Limits::full(b);
    };
    let m = |k: usize| s.slack[k];
    let ahead = |(w, k): (usize, usize), x: u32| -> u32 {
        let r = prev.map_or(2.0, |p| (f64::from(read[w]) / f64::from(p[w].max(1))).clamp(1.0, 4.0));
        (f64::from(x) * r * r * f64::from(m(k))).ceil() as u32
    };
    let born = read[c::AXES] - read[c::AXES0];
    let live = read[c::NEXT] * m(S_LIVE) + 2 * ahead((c::AXES, S_LIVE), born);
    let lit = ahead((c::PHYTS, S_DEPOSITS), leaves(read));
    let nodes = 2 * m(S_LIGHTS) * read[c::GRID + 6];
    let r = b.ranges;
    let deepest = (0..DEEP as usize).filter(|&g| read[c::LAY_DEEP + g] != 0).count() as u32;
    let axes = ahead((c::AXES, S_SCANS), read[c::AXES]);
    let living = levels_of(read[c::NEXT] * m(S_SCANS) + 2 * ahead((c::AXES, S_SCANS), born));
    let scans = [
        levels_of(axes),
        levels_of(ahead((c::AXES, S_SCANS), read[c::BLOCKS0] + born)),
        levels_of(ahead((c::AXES, S_SCANS), born)),
        levels_of(read[c::LEVELS] + read[c::GENS] + 8 * m(S_SCANS)),
        living,
        levels_of(axes),
        living,
        levels_of(ahead((c::SLEEPERS, S_SCANS), read[c::SLEEPERS])),
        levels_of(ahead((c::AXES, S_SCANS), born)),
    ];
    Limits {
        units: live.div_ceil(r.units),
        draws: live.div_ceil(r.draw),
        deposits: lit.div_ceil(r.deposit),
        lights: (9 * nodes).div_ceil(r.paths).max(nodes.div_ceil(r.nodes)),
        sweeps: sweeps_ahead(read, prev) + m(S_SWEEPS),
        rounds: read[c::GENS] + m(S_ROUNDS),
        passes: passes(read) + m(S_PASSES),
        levels: read[c::LEVELS] + 1 + m(S_LEVELS),
        laygens: deepest + m(S_LAYGENS),
        scans,
        unit_rounds: [0, 1].map(|w| super::plan_units::rounds_used(read, w) + m(S_UNIT_ROUNDS)),
    }
    .within(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A census whose phytomers grew from `before` to `now`.
    fn census(before: u32, now: u32) -> (Vec<u32>, Vec<u32>) {
        let (mut prev, mut read) = (vec![0; c::WORDS], vec![0; c::WORDS]);
        (prev[c::PHYTS], read[c::PHYTS]) = (before, now);
        (prev, read)
    }

    /// Astra's finding: a projection past the engine's budget sizes no
    /// phytomer table past it.
    #[test]
    fn no_phytomer_table_grows_past_the_budget() {
        let budget = 20_000_000;
        let (prev, read) = census(10_000_000, 19_000_000);
        let next = ahead(Sizes::least(), &read, Some(&prev), budget).expect("a move");
        assert_eq!(next.caps.phyts, budget);
        let flags = [(Flag::Phyts, 1)];
        let raised = raised(Sizes::least(), &flags, &read, budget).expect("a size");
        assert_eq!(raised.caps.phyts, budget);
    }

    #[test]
    fn a_build_past_the_budget_is_refused_by_name() {
        let (_, read) = census(0, 20_000_001);
        let flags = [(Flag::Budget, 1), (Flag::Phyts, 1)];
        let said = raised(Sizes::least(), &flags, &read, 20_000_000).expect_err("refused");
        assert!(said.to_string().contains("budget of 20000000 phytomers"), "{said}");
    }
}
