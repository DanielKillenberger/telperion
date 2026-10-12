//! A build grown to its end (DESIGN.md section 5). The host reads the
//! census a cycle late: where a count, at the rate it grew, would pass
//! its table's capacity or its loop's bound within the next two cycles,
//! the build moves into larger tables on the queue (`build_move.rs`) and
//! its cycle is recorded again; where a count passed one already, the
//! build begins again from the seed with that size raised, which leaves
//! the same tree. Never an expectation: every size is a count's.
use super::build::{Build, Kit, Late, Tables};
use super::build_light::Lit;
use super::build_plan::Bounds;
use super::build_size::{ahead, limits, raised, SLACK, SLACKS};
use super::census::{self as c, Flag, PASS_LEFT};
use super::given::{Over, Span, Widths};
use super::state::Caps;
use crate::{Gpu, Result};
use telperion_space::Capture;

/// The cycles a build grows short of: its time spent and grown is held in
/// 32-bit ticks, 2^20 a cycle, signed where an age's share is cut.
pub(super) const MOST_CYCLES: u32 = 2048;

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
        Self { caps, points: 16384, tips: 4096, nodes: 4096, bounds: Bounds::default(), block_cap: super::hier_build::CAP, slack: SLACK, widths: Widths::default(), compact: false }
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

/// `upto` cycles grown from `sizes`: the build, its sizes and how it grew.
pub(super) async fn grow(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, (mut sizes, upto): (Sizes, u32)) -> Result<(Build, Sizes, Grew)> {
    let cycles = upto.max(t.age);
    if cycles >= MOST_CYCLES {
        return Err(super::super::io::error(format!("a tree of {cycles} cycles: the device holds its time to {MOST_CYCLES} cycles")));
    }
    let mut grew = Grew::default();
    'begin: for again in 0..64 {
        grew.again = again;
        let mut b = Build::new(gpu, t, (tables, sizes.caps, sizes.block_cap), sizes.lit())?;
        t.within_budget(gpu)?;
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
            let clock = super::super::Clock::now();
            let (stamped, sent) = b.submit(gpu, (kit, t), (&run.0, &run.1, &limits, &span), (&mut late, cycle));
            pending.push_back(sent);
            grew.recording += clock.elapsed_ms() / 1e3;
            grew.stamped.extend(stamped);
            if cycle == 1 {
                continue;
            }
            let clock = super::super::Clock::now();
            let read = late.take(gpu, cycle - 1).await?;
            grew.waiting += clock.elapsed_ms() / 1e3;
            let sent = pending.pop_front().expect("a cycle sent");
            sent.tally(&mut grew.over, &read);
            if !c::raised(&read).is_empty() {
                if read[Flag::Given.word()] != 0 {
                    sent.widen(&mut sizes.widths, &read);
                }
                grew.why.push((cycle - 1, c::raised(&read).iter().map(|f| f.0).collect()));
                if std::env::var("CYCLE_TRACE").is_ok() {
                    eprintln!("TRACE begun again after cycle {}", cycle - 1);
                    eprintln!("TRACE raised {:?} rec {:?} gens {} levels {} passes left {:?}", c::raised(&read), &read[c::REC..c::REC + 15], read[c::GENS], read[c::LEVELS], &read[PASS_LEFT..PASS_LEFT + 24]);
                }
                sizes = raised(sizes, &c::raised(&read), &read, t.budget)?;
                // The cycle in flight read back before its buffers go.
                late.take(gpu, cycle).await?;
                continue 'begin;
            }
            let clock = super::super::Clock::now();
            // A move makes room for the next cycle; after the last there
            // is none, and a compaction then would leave the build holding
            // the last census's rows.
            let next = (cycle < upto).then(|| ahead(sizes, &read, last.as_deref(), t.budget)).flatten();
            if let Some(next) = next {
                b = b.moved(gpu, (t, tables), (sizes, next))?;
                t.within_budget(gpu)?;
                let plan = b.plan(gpu, kit, t, next.bounds)?;
                let mut e = gpu.encoder(&Default::default());
                e.copy_buffer_to_buffer(&run.0.census, 0, &plan.0.census, 0, run.0.census.size());
                gpu.submit([e.finish()]);
                (run, sizes) = (plan, next);
                grew.moves += 1;
                if b.stashed.is_some() {
                    b.compact(gpu, kit, &run.0.census).await?;
                }
                grew.moved_at.push((cycle, b.retire.rec_n));
            }
            grew.moving += clock.elapsed_ms() / 1e3;
            prev = last.replace(read);
        }
        let read = late.take(gpu, upto).await?;
        let sent = pending.pop_front().expect("a cycle sent");
        sent.tally(&mut grew.over, &read);
        if !c::raised(&read).is_empty() {
            if read[Flag::Given.word()] != 0 {
                sent.widen(&mut sizes.widths, &read);
            }
            grew.why.push((upto, c::raised(&read).iter().map(|f| f.0).collect()));
            sizes = raised(sizes, &c::raised(&read), &read, t.budget)?;
            continue;
        }
        b.counted(&read);
        if std::env::var("CYCLE_TRACE").is_ok() {
            trace(&grew, &sizes, upto);
        }
        return Ok((b, sizes, grew));
    }
    Err(super::super::io::error("a build that never fitted".into()))
}

/// How a build grew, to stderr: its restarts and moves, the host's
/// seconds, and its dispatches by phase and a cycle.
fn trace(grew: &Grew, sizes: &Sizes, upto: u32) {
    let mut by: Vec<(&str, usize)> = Vec::new();
    for p in &grew.stamped {
        match by.iter_mut().find(|b| b.0 == *p) {
            Some(b) => b.1 += 1,
            None => by.push((p, 1)),
        }
    }
    let n = grew.stamped.len();
    eprintln!("TRACE phases {by:?}");
    eprintln!(
        "TRACE grown: again {}, moves {}, recording {:.3} s, waiting {:.3} s, moving {:.3} s; dispatches {n} ({:.0} a cycle); caps {:?}",
        grew.again, grew.moves, grew.recording, grew.waiting, grew.moving, n as f64 / f64::from(upto), sizes.caps
    );
}

/// `grow`, blocked on: the tests' builds.
#[cfg(test)]
pub(super) fn grow_now(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, at: (Sizes, u32)) -> Result<(Build, Sizes, Grew)> {
    pollster::block_on(grow(gpu, kit, t, tables, at))
}
