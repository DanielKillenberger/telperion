//! A build grown to its end (DESIGN.md section 5, the host's rulings of
//! 2026-10-12). Its tables are sized once from its hint
//! (`build_arena.rs`); the host reads the census a cycle late, compacts
//! the arena in place where it runs tight (`compact.rs`), and doubles a
//! table its hint undersized before the next two cycles could pass it
//! (`build_size::grown`, a move on the queue, `build_move.rs`). A count
//! past any table or limit stops the build by name: no build begins
//! again.
use super::build::{Build, Kit, Late, Tables};
use super::build_arena::TABLES;
use super::build_light::Lit;
use super::build_plan::Bounds;
use super::build_size::{grown, limits, refusal};
use super::census as c;
use super::given::{Over, Span};
use super::state::Caps;
use crate::{Gpu, Result};
use telperion_space::Capture;

/// The phase a compaction's dispatches are counted in: a build's, not a
/// cycle's (`trace`).
pub(super) const COMPACTION: &str = "compaction";

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
    /// The retired rows the records hold, where a PA keeps them.
    pub records: u32,
    pub bounds: Bounds,
    /// The most axes a hierarchy block holds: speed only (HIERARCHY.md).
    pub block_cap: u32,
    /// When the arena is compacted in place (`compact.rs`).
    pub compact: Compact,
}

impl Sizes {
    /// The smallest a build takes, its bounds stated from its tables.
    pub(super) fn least() -> Self {
        let caps = Caps { axes: 4096, units: 4096, phyts: 16384, sleepers: 4096, next: 4096, woken: 1024 };
        let s = Self { caps, points: 16384, tips: 4096, nodes: 4096, records: 0, bounds: Bounds::default(), block_cap: super::hier_build::CAP, compact: Compact::Off };
        Self { bounds: s.stated(), ..s }
    }

    pub(super) fn lit(&self) -> (Lit, u32) {
        (Lit { leaves: self.points - self.tips.min(self.points), tips: self.tips }, self.nodes)
    }
}

/// When the arena is compacted in place: never (the tests' reference),
/// where a phytomer, axis or unit table's rows two cycles on, at the rate
/// they grew, would pass `TIGHT` of its rows, or (the tests') after every
/// cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Compact {
    Off,
    Tight,
    #[cfg_attr(not(test), allow(dead_code))]
    Every,
}

/// The share of a table's rows its projection may reach before the arena
/// is compacted: each compaction costs Telperion about 24 ms; at half,
/// Telperion compacted 7 times (2,028 ms warm), at 0.7 four (1,973 ms), at
/// 0.85 three (1,932 ms), each preset with no move (fn-256 task 6). Below
/// the seven eighths at which a table would double instead (`grown`).
const TIGHT: f64 = 0.85;

impl Compact {
    /// Whether to compact now, the census read and the one before it.
    fn now(self, caps: &Caps, (read, prev): (&[u32], Option<&[u32]>)) -> bool {
        let ahead = |w: usize| {
            let r = prev.map_or(2.0, |p| (f64::from(read[w]) / f64::from(p[w].max(1))).clamp(1.0, 4.0));
            f64::from(read[w]) * r * r
        };
        let tight = |w: usize, cap: u32| ahead(w) > TIGHT * f64::from(cap);
        match self {
            Compact::Off => false,
            Compact::Every => true,
            Compact::Tight => tight(c::PHYTS, caps.phyts) || tight(c::AXES, caps.axes) || tight(c::UNITS, caps.units),
        }
    }
}

/// How a build grew: its moves and compactions, the most each table held,
/// and the phase of each dispatch it recorded, in order.
#[derive(Debug, Clone, Default)]
pub(super) struct Grew {
    pub moves: u32,
    /// The in-place compactions it made.
    pub compactions: u32,
    /// The most each table held at a cycle's end, before any compaction
    /// after it (`build_arena::tables`).
    pub most: [u32; TABLES],
    /// The cycle after which each move was made, and the records held
    /// after it.
    pub moved_at: Vec<(u32, u32)>,
    pub stamped: Vec<&'static str>,
    /// Host seconds recording and submitting cycles, waiting for the
    /// census, and moving and compacting.
    pub recording: f64,
    pub waiting: f64,
    pub moving: f64,
    /// The workgroups its bounded dispatches were given and needed.
    pub over: Over,
    /// The tests' trace (fn-256 task 7): the dispatches recorded by each
    /// cycle's end, and each cycle's census as read.
    #[cfg(test)]
    pub ends: Vec<usize>,
    #[cfg(test)]
    pub censuses: Vec<Vec<u32>>,
}

impl Grew {
    /// A census read: the most each table held, and the cycle's
    /// dispatches tallied; a count past a table or limit refused by name.
    fn read(&mut self, (read, cycle): (&[u32], u32), sent: &super::given::Pending, budget: u32) -> Result<()> {
        sent.tally(&mut self.over, read);
        let held = super::build_arena::tables(read);
        self.most = std::array::from_fn(|k| self.most[k].max(held[k]));
        let raised = c::raised(read);
        if raised.is_empty() {
            return Ok(());
        }
        if std::env::var("CYCLE_TRACE").is_ok() {
            eprintln!("TRACE raised after cycle {cycle}: {raised:?} rec {:?} gens {} levels {}", &read[c::REC..c::REC + 15], read[c::GENS], read[c::LEVELS]);
            sent.short(read).iter().for_each(|s| eprintln!("TRACE short {s}"));
        }
        Err(refusal(cycle, &raised, budget))
    }
}

/// `upto` cycles grown from `sizes`: the build, its sizes and how it grew.
pub(super) async fn grow(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, (mut sizes, upto): (Sizes, u32)) -> Result<(Build, Sizes, Grew)> {
    let cycles = upto.max(t.age);
    if cycles >= MOST_CYCLES {
        return Err(super::super::io::error(format!("a tree of {cycles} cycles: the device holds its time to {MOST_CYCLES} cycles")));
    }
    let mut grew = Grew::default();
    // A build let go before this one freed first (`Ledger::destroy`).
    super::super::io::complete(gpu).await?;
    let mut b = Build::new(gpu, t, (tables, sizes.caps, sizes.block_cap, sizes.records), sizes.lit())?;
    let mut run = b.plan(gpu, kit, t, sizes.bounds)?;
    grew.stamped = b.seeded(gpu, kit, (&run.0, &run.1));
    let (mut late, mut prev, mut last) = (Late::new(gpu)?, None::<Vec<u32>>, None::<Vec<u32>>);
    let mut pending = std::collections::VecDeque::new();
    // The last cycle after which the arena was compacted, and the census
    // read after it: a census read before it counts more rows than the
    // cycle begins with.
    let (mut compacted, mut cut) = (None::<u32>, Vec::new());
    // The last cycle submitted's limits and workgroups given, which
    // retirement's inside marks are taken under (`Build::inside`).
    let mut submitted = (limits(&sizes, None, None), Vec::new());
    for cycle in 1..=upto {
        let limits = limits(&sizes, last.as_deref(), prev.as_deref());
        let seed = Build::seed_census();
        let (seen, before) = match (&last, &prev) {
            (Some(l), p) => (l.as_slice(), Some(p.as_deref().unwrap_or(&seed))),
            (None, _) => (seed.as_slice(), None),
        };
        let fallen = compacted.is_some_and(|k| k + 2 >= cycle).then_some(cut.as_slice());
        let span = Span::new(seen, before, (&sizes.ceilings(), fallen), (cycle, &limits));
        if std::env::var("CYCLE_TRACE").is_ok() {
            eprintln!("TRACE cycle {cycle} {limits:?}; light nodes {} points {} tips {}", sizes.nodes, sizes.points, sizes.tips);
            if let Some(l) = &last {
                let n = (l[c::LEVELS] as usize).min(63);
                let widths: Vec<u32> = (0..n).map(|k| l[c::LVL + k + 1] - l[c::LVL + k]).collect();
                eprintln!("TRACE levels {} blocks by level {widths:?} live {} gens {} passes {} laygens {}", l[c::LEVELS], l[c::NEXT], l[c::GENS], super::build_size::passes(l), (0..super::lay_device::DEEP as usize).filter(|&g| l[c::LAY_DEEP + g] != 0).count());
            }
        }
        let clock = super::super::Clock::now();
        let (stamped, sent, given) = b.submit(gpu, (kit, t), (&run.0, &run.1, &limits, &span), (&mut late, cycle));
        pending.push_back(sent);
        submitted = (limits, given);
        grew.recording += clock.elapsed_ms() / 1e3;
        grew.stamped.extend(stamped);
        #[cfg(test)]
        grew.ends.push(grew.stamped.len());
        if cycle == 1 {
            continue;
        }
        let clock = super::super::Clock::now();
        let read = late.take(gpu, cycle - 1).await?;
        grew.waiting += clock.elapsed_ms() / 1e3;
        #[cfg(test)]
        grew.censuses.push(read.clone());
        let sent = pending.pop_front().expect("a cycle sent");
        if let Err(stopped) = grew.read((&read, cycle - 1), &sent, t.budget) {
            // The cycle in flight read back before its buffers go.
            late.take(gpu, cycle).await?;
            return Err(stopped);
        }
        let clock = super::super::Clock::now();
        // A move makes room for the next cycle; after the last there is
        // none, nor a compaction, whose census the build would not read.
        let fallen = compacted.is_some_and(|k| k + 2 >= cycle).then_some(cut.as_slice());
        let moving = (cycle < upto).then(|| grown(sizes, (&read, last.as_deref(), fallen), t.budget)).flatten();
        // A compaction after the cycle read counts nothing the census shows.
        let after = moving.unwrap_or(sizes);
        let fresh = compacted.is_none_or(|k| k + 1 < cycle) || after.compact == Compact::Every;
        let compacting = cycle < upto && fresh && after.compact.now(&after.caps, (&read, last.as_deref()));
        // The inside marks a compaction drops by, taken before a move
        // carries them over.
        if compacting {
            grew.stamped.extend(b.inside(gpu, kit, (&run.0, &run.1), (cycle, &submitted.0, &submitted.1)));
        }
        if let Some(next) = moving {
            b = b.moved(gpu, (t, tables), (sizes, next))?;
            let plan = b.plan(gpu, kit, t, next.bounds)?;
            let mut e = gpu.encoder(&Default::default());
            e.copy_buffer_to_buffer(&run.0.census, 0, &plan.0.census, 0, run.0.census.size());
            gpu.submit([e.finish()]);
            (run, sizes) = (plan, next);
            grew.moves += 1;
            grew.moved_at.push((cycle, b.retire.rec_n));
        }
        if compacting {
            let traced = std::env::var("CYCLE_TRACE").is_ok();
            if traced {
                super::super::io::complete(gpu).await?;
            }
            let clock = super::super::Clock::now();
            let made = b.compact_in_place(gpu, kit, (&run.0.census, cycle), reach(&read)).await?;
            grew.stamped.extend(std::iter::repeat_n(COMPACTION, made));
            if traced {
                super::super::io::complete(gpu).await?;
                eprintln!("TRACE compacted after cycle {cycle} in {:.1} ms", clock.elapsed_ms());
            }
            (grew.compactions, compacted) = (grew.compactions + 1, Some(cycle));
            let words = super::super::io::read_async(gpu, &run.0.census, 4 * c::WORDS as u64).await?;
            cut = bytemuck::cast_slice(&words).to_vec();
        }
        grew.moving += clock.elapsed_ms() / 1e3;
        prev = last.replace(read);
    }
    // The records a compaction grew past their rows moved as a table does.
    grew.moves += b.retire.rec_moves;
    // The inside marks the settle reads its retired roots by.
    grew.stamped.extend(b.inside(gpu, kit, (&run.0, &run.1), (upto, &submitted.0, &submitted.1)));
    let read = late.take(gpu, upto).await?;
    #[cfg(test)]
    grew.censuses.push(read.clone());
    let sent = pending.pop_front().expect("a cycle sent");
    grew.read((&read, upto), &sent, t.budget)?;
    b.counted(&read);
    if std::env::var("CYCLE_TRACE").is_ok() {
        trace(&grew, &sizes, upto);
    }
    Ok((b, sizes, grew))
}

/// The rows the axes, units and phytomers may hold at the end of the
/// cycle after `read`'s: its counts and four of its cycle's growth past
/// them, and 4,096 rows. A table that holds more is flagged by the
/// compaction (`Flag::Compaction`) and the build refused by name.
fn reach(read: &[u32]) -> [u32; 3] {
    let at = |(start, end): (usize, usize)| read[end].saturating_add(4 * read[end].saturating_sub(read[start])).saturating_add(4096);
    [at((c::AXES0, c::AXES)), at((c::UNITS0, c::UNITS)), at((c::PHYTS0, c::PHYTS))]
}

/// How a build grew, to stderr: its moves and compactions, the host's
/// seconds, and its dispatches by phase and a cycle.
fn trace(grew: &Grew, sizes: &Sizes, upto: u32) {
    let mut by: Vec<(&str, usize)> = Vec::new();
    for p in &grew.stamped {
        match by.iter_mut().find(|b| b.0 == *p) {
            Some(b) => b.1 += 1,
            None => by.push((p, 1)),
        }
    }
    let compacting = grew.stamped.iter().filter(|p| **p == COMPACTION).count();
    let n = grew.stamped.len() - compacting;
    eprintln!("TRACE phases {by:?}");
    eprintln!(
        "TRACE grown: moves {}, compactions {} ({compacting} dispatches), recording {:.3} s, waiting {:.3} s, moving {:.3} s; dispatches {n} ({:.0} a cycle); workgroups given {} for {} needed; caps {:?}",
        grew.moves, grew.compactions, grew.recording, grew.waiting, grew.moving, n as f64 / f64::from(upto), grew.over.given, grew.over.needed, sizes.caps
    );
}

/// `grow`, blocked on: the tests' builds.
#[cfg(test)]
pub(super) fn grow_now(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, at: (Sizes, u32)) -> Result<(Build, Sizes, Grew)> {
    pollster::block_on(grow(gpu, kit, t, tables, at))
}
