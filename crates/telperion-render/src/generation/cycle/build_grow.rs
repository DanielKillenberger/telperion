//! A build grown to its end (DESIGN.md section 5). The host reads the
//! census a cycle late: where a count, at the rate it grew, would pass
//! its table's capacity or its loop's bound within the next two cycles,
//! the build moves into larger tables on the queue (`build_move.rs`) and
//! its cycle is recorded again; where a count passed one already, the
//! build begins again from the seed with that size raised, which leaves
//! the same tree. Never an expectation: every size is a count's.
use super::build::{Build, Kit, Late, Tables};
use super::build_light::{refusals, Lit};
use super::build_plan::Bounds;
use super::census::{self as c, Flag, PASSES, PASS_LEFT};
use super::gate::{levels_of, rec, Limits};
use super::given::{Over, Span, Widths};
use super::lay_device::DEEP;
use super::light_program::{SWEEP_WG, SWEEP_WORK};
use super::state::Caps;
use crate::{Gpu, Result};
use telperion_space::Capture;

/// What a build is sized for: the state's tables, the light's points,
/// tips and lattice nodes, and the loops' bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Sizes {
    pub caps: Caps,
    pub points: u32,
    pub tips: u32,
    pub nodes: u32,
    pub bounds: Bounds,
    /// The most axes a hierarchy block holds: speed only (HIERARCHY.md).
    pub block_cap: u32,
    /// How far past its projection each of the cycle's limits reaches
    /// (`Slack`): doubled for a limit each time a count passes it.
    pub slack: [u32; SLACKS],
    /// How far past its projection each census word reaches, for the
    /// workgroups a dispatch is given (`given.rs`).
    pub widths: Widths,
    /// Whether a move into larger phytomer tables compacts out the
    /// retired subtrees' phytomers (DESIGN.md section 6).
    pub compact: bool,
}

impl Sizes {
    /// The smallest a build starts from.
    pub(super) fn least() -> Self {
        let caps = Caps { axes: 4096, units: 4096, phyts: 16384, sleepers: 4096, next: 4096, woken: 1024 };
        Self { caps, points: 16384, tips: 4096, nodes: 4096, bounds: Bounds::default(), block_cap: super::hier_build::CAP, slack: [1; SLACKS], widths: Widths::default(), compact: false }
    }

    pub(super) fn lit(&self) -> (Lit, u32) {
        (Lit { leaves: self.points - self.tips.min(self.points), tips: self.tips }, self.nodes)
    }
}

/// How a build grew: begun again, moved into larger tables, and the
/// phase of each dispatch it recorded, in order.
#[derive(Debug, Clone, Default)]
pub(super) struct Grew {
    pub again: u32,
    pub moves: u32,
    /// The cycle after which each move of the last begin was made, and
    /// the records held after it.
    pub moved_at: Vec<(u32, u32)>,
    pub stamped: Vec<&'static str>,
    /// Why it began again: each time's flags and the cycle read.
    pub why: Vec<(u32, Vec<Flag>)>,
    /// Host seconds recording and submitting cycles, waiting for the
    /// census, and moving.
    pub recording: f64,
    pub waiting: f64,
    pub moving: f64,
    /// The workgroups its bounded dispatches were given and needed.
    pub over: Over,
}

fn twice(have: u32, count: u32) -> u32 {
    have.max(count).saturating_mul(2)
}

/// Whether `flag` names a fault (or the budget) rather than a size.
fn fault(flag: Flag) -> bool {
    matches!(flag, Flag::Budget | Flag::UnitFaults | Flag::HierFault | Flag::LayFault | Flag::Collapsed)
}

/// The limits' slacks, by what each limits.
const SLACKS: usize = 9;
const S_LIVE: usize = 0;
const S_DEPOSITS: usize = 1;
const S_LIGHTS: usize = 2;
const S_SWEEPS: usize = 3;
const S_ROUNDS: usize = 4;
const S_PASSES: usize = 5;
const S_LEVELS: usize = 6;
const S_LAYGENS: usize = 7;
const S_SCANS: usize = 8;

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
fn raised(mut s: Sizes, flags: &[(Flag, u32)], census: &[u32]) -> Result<Sizes> {
    let refused = |what: String| super::super::io::error(what);
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
            Flag::LightRanges => b.lay.lights *= 2,
            Flag::Units => caps.units = twice(caps.units, census[c::UNITS]),
            Flag::Phyts => caps.phyts = twice(caps.phyts, census[c::PHYTS]),
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

/// The layer ranges the sweep took in the census's cycle.
fn sweeps(census: &[u32]) -> u32 {
    let layer = census[c::GRID + 3] * census[c::GRID + 4];
    let per = (SWEEP_WORK / layer.div_ceil(SWEEP_WG).max(1)).max(1);
    census[c::GRID + 5].div_ceil(per)
}

/// The sizes the next two cycles need, from `read` and the cycle before
/// it (`prev`) at the rate each count grew; none where `s` holds them.
fn ahead(s: Sizes, read: &[u32], prev: Option<&[u32]>) -> Option<Sizes> {
    let ahead = |w: usize, x: u32| -> u32 {
        let r = prev.map_or(2.0, |p| (f64::from(read[w]) / f64::from(p[w].max(1))).clamp(1.0, 4.0));
        (f64::from(x) * r * r).ceil() as u32
    };
    let fit = |have: u32, need: u32| if need > have { need + need / 4 } else { have };
    let mut n = s;
    let caps = &mut n.caps;
    caps.axes = fit(caps.axes, ahead(c::AXES, read[c::AXES])).next_multiple_of(64);
    caps.units = fit(caps.units, ahead(c::UNITS, read[c::UNITS]));
    caps.phyts = fit(caps.phyts, ahead(c::PHYTS, read[c::PHYTS]));
    caps.sleepers = fit(caps.sleepers, ahead(c::SLEEPERS, read[c::SLEEPERS]));
    // The living after a cycle are at most those before and the cycle's
    // new axes, which grow as the axes do.
    let born = read[c::AXES] - read[c::AXES0];
    let next = read[c::NEXT] + 2 * ahead(c::AXES, born);
    caps.next = fit(caps.next, next.max(ahead(c::NEXT, read[c::NEXT]))).next_multiple_of(64);
    caps.woken = fit(caps.woken, ahead(c::WOKEN, read[c::WOKEN]));
    let lit = ahead(c::PHYTS, leaves(read));
    n.tips = fit(n.tips, ahead(c::NEXT, read[c::NEXT]));
    n.points = fit(n.points, lit + n.tips);
    n.nodes = fit(n.nodes, 2 * read[c::GRID + 6]);
    let b = &mut n.bounds;
    b.units = b.units.max(next.div_ceil(b.ranges.units));
    b.lay.draws = b.lay.draws.max(next.div_ceil(b.ranges.draw));
    // The lattice's nodes, and over the sky's nine directions at most.
    let nodes = 2 * read[c::GRID + 6];
    b.lay.lights = b.lay.lights.max((9 * nodes).div_ceil(b.ranges.paths).max(nodes.div_ceil(b.ranges.nodes)));
    b.lay.deposits = b.lay.deposits.max(lit.div_ceil(b.ranges.deposit));
    b.lay.sweeps = b.lay.sweeps.max(sweeps(read) + 2);
    b.hier.levels = b.hier.levels.max((read[c::LEVELS] + 3).min(63));
    b.hier.gens = b.hier.gens.max(read[c::GENS] + 2);
    let deepest = (0..DEEP as usize).rev().find(|&g| read[c::LAY_DEEP + g] != 0).unwrap_or(0) as u32;
    b.lay.generations = b.lay.generations.max((deepest + 3).min(15));
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
fn limits(s: &Sizes, read: Option<&[u32]>, prev: Option<&[u32]>) -> Limits {
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
        sweeps: sweeps(read) + m(S_SWEEPS),
        rounds: read[c::GENS] + m(S_ROUNDS),
        passes: passes(read) + m(S_PASSES),
        levels: read[c::LEVELS] + 1 + m(S_LEVELS),
        laygens: deepest + m(S_LAYGENS),
        scans,
    }
    .within(b)
}

/// `upto` cycles grown from `sizes`: the build, its sizes and how it grew.
pub(super) fn grow(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, (mut sizes, upto): (Sizes, u32)) -> Result<(Build, Sizes, Grew)> {
    let mut grew = Grew::default();
    'begin: for again in 0..64 {
        grew.again = again;
        let mut b = Build::new(gpu, t, (tables, sizes.caps, sizes.block_cap), sizes.lit())?;
        let mut run = b.plan(gpu, kit, t, sizes.bounds)?;
        grew.stamped = b.seeded(gpu, kit, (&run.0, &run.1));
        grew.moved_at.clear();
        let (mut late, mut prev, mut last) = (Late::new(gpu)?, None::<Vec<u32>>, None::<Vec<u32>>);
        let mut pending = std::collections::VecDeque::new();
        grew.over = Over::default();
        for cycle in 1..=upto {
            let limits = limits(&sizes, last.as_deref(), prev.as_deref());
            let seed = Build::seed_census();
            let (seen, before) = match (&last, &prev) {
                (Some(l), p) => (l.as_slice(), Some(p.as_deref().unwrap_or(&seed))),
                (None, _) => (seed.as_slice(), None),
            };
            let span = Span::new(seen, before, &sizes.widths, (cycle, &limits));
            if std::env::var("CYCLE_TRACE").is_ok() {
                eprintln!("TRACE begin {again} cycle {cycle} slack {:?} {limits:?}", sizes.slack);
            }
            let clock = std::time::Instant::now();
            let (stamped, sent) = b.submit(gpu, (kit, t), (&run.0, &run.1, &limits, &span), (&mut late, cycle));
            pending.push_back(sent);
            grew.recording += clock.elapsed().as_secs_f64();
            grew.stamped.extend(stamped);
            if cycle == 1 {
                continue;
            }
            let clock = std::time::Instant::now();
            let read = late.take(gpu, cycle - 1)?;
            grew.waiting += clock.elapsed().as_secs_f64();
            let sent = pending.pop_front().expect("a cycle sent");
            sent.tally(&mut grew.over, &read);
            if !c::raised(&read).is_empty() {
                if read[Flag::Given.word()] != 0 {
                    sent.widen(&mut sizes.widths, &read);
                }
                grew.why.push((cycle - 1, c::raised(&read).iter().map(|f| f.0).collect()));
                if std::env::var("CYCLE_TRACE").is_ok() {
                    eprintln!("TRACE raised {:?} rec {:?} gens {} levels {} passes left {:?}", c::raised(&read), &read[c::REC..c::REC + 15], read[c::GENS], read[c::LEVELS], &read[PASS_LEFT..PASS_LEFT + 24]);
                }
                sizes = raised(sizes, &c::raised(&read), &read)?;
                // The cycle in flight read back before its buffers go.
                late.take(gpu, cycle)?;
                continue 'begin;
            }
            let clock = std::time::Instant::now();
            // A move makes room for the next cycle; after the last there
            // is none, and a compaction then would leave the build holding
            // the last census's rows.
            let next = (cycle < upto).then(|| ahead(sizes, &read, last.as_deref())).flatten();
            if let Some(next) = next {
                b = b.moved(gpu, (t, tables), (sizes, next))?;
                let plan = b.plan(gpu, kit, t, next.bounds)?;
                let mut e = gpu.encoder(&Default::default());
                e.copy_buffer_to_buffer(&run.0.census, 0, &plan.0.census, 0, run.0.census.size());
                gpu.submit([e.finish()]);
                (run, sizes) = (plan, next);
                grew.moves += 1;
                if b.stashed.is_some() {
                    b.compact(gpu, kit, &run.0.census)?;
                }
                grew.moved_at.push((cycle, b.retire.rec_n));
            }
            grew.moving += clock.elapsed().as_secs_f64();
            prev = last.replace(read);
        }
        let read = late.take(gpu, upto)?;
        let sent = pending.pop_front().expect("a cycle sent");
        sent.tally(&mut grew.over, &read);
        if !c::raised(&read).is_empty() {
            if read[Flag::Given.word()] != 0 {
                sent.widen(&mut sizes.widths, &read);
            }
            grew.why.push((upto, c::raised(&read).iter().map(|f| f.0).collect()));
            sizes = raised(sizes, &c::raised(&read), &read)?;
            continue;
        }
        b.counted(&read);
        return Ok((b, sizes, grew));
    }
    Err(super::super::io::error("a build that never fitted".into()))
}
