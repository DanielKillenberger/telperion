//! A build grown to its end (DESIGN.md section 5, the host's rulings of
//! 2026-10-12). Its tables are sized once from its hint
//! (`build_arena.rs`); the host reads the census a cycle late, compacts
//! the arena in place where it runs tight (`compact.rs`), and doubles a
//! table its hint undersized before the next two cycles could pass it
//! (`build_size::grown`, a move on the queue, `build_move.rs`). A count
//! past any table or limit stops the build by name: no build begins
//! again.
use super::build::{Build, Kit, Late, Tables};
use super::build_arena::{Hint, TABLES};
use super::build_light::Lit;
use super::build_plan::{Bounds, Phases};
use super::build_size::{grown, limits, refusal};
use super::census as c;
use super::gate::Limits;
use super::given::{Over, Pending, Span};
use super::plan::Plan;
use super::state::Caps;
use crate::{Gpu, Result};
use std::collections::VecDeque;
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
    pub bounds: Bounds,
    /// The most axes a hierarchy block holds: speed only (HIERARCHY.md).
    pub block_cap: u32,
    /// When the arena is compacted in place (`compact.rs`).
    pub compact: Compact,
    /// The most units one job grows in a cycle (`Hint::units`).
    pub units: u32,
}

impl Sizes {
    /// The smallest a build takes, its bounds stated from its tables.
    pub(super) fn least() -> Self {
        let caps = Caps { axes: 4096, units: 4096, phyts: 16384, sleepers: 4096, next: 4096, woken: 1024 };
        let s = Self { caps, points: 16384, tips: 4096, nodes: 4096, bounds: Bounds::default(), block_cap: super::hier_build::CAP, compact: Compact::Off, units: 1 };
        Self { bounds: s.stated(), ..s }
    }

    /// The bytes a first move adds beside these tables: the largest group
    /// it holds twice (`build_move.rs`: the phytomers, the axes, or the
    /// units and lists) made anew `GROWTH` times larger.
    pub(super) fn first_move(&self) -> u64 {
        let w = super::state::State::words(self.caps);
        let groups = [w[3] + w[4], w[0] + w[1], w[2] + w[5]];
        let most = groups.into_iter().max().unwrap_or(0);
        (4.0 * most as f64 * super::build_size::GROWTH).ceil() as u64
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
    /// Whether to compact now, by the census read: a table's rows two
    /// cycles on, each growing as the census's own cycle did (its end
    /// against its start, which no compaction between cycles hides).
    fn now(self, caps: &Caps, read: &[u32]) -> bool {
        let ahead = |(start, end): (usize, usize)| {
            let r = (f64::from(read[end]) / f64::from(read[start].max(1))).clamp(1.0, 4.0);
            f64::from(read[end]) * r * r
        };
        let tight = |w: (usize, usize), cap: u32| ahead(w) > TIGHT * f64::from(cap);
        match self {
            Compact::Off => false,
            Compact::Every => true,
            Compact::Tight => tight((c::PHYTS0, c::PHYTS), caps.phyts) || tight((c::AXES0, c::AXES), caps.axes) || tight((c::UNITS0, c::UNITS), caps.units),
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
    /// The cycle after which each move was made.
    pub moved_at: Vec<u32>,
    pub stamped: Vec<&'static str>,
    /// Host seconds recording and submitting cycles, waiting for the
    /// census, moving and compacting, and making a paused build's scratch
    /// and recorded cycle again (`build_pause.rs`).
    pub recording: f64,
    pub waiting: f64,
    pub moving: f64,
    pub rebuilt: f64,
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

/// `upto` cycles grown from `sizes`: the build, its sizes and how it
/// grew; the tests'.
#[cfg(test)]
pub(super) async fn grow(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, (sizes, upto): (Sizes, u32)) -> Result<(Build, Sizes, Grew)> {
    let g = Growing::begin(gpu, kit, t, tables, sizes).await?;
    let g = g.grow_to(gpu, (kit, t, tables), (upto, true)).await?;
    Ok((g.b, g.sizes, g.grew))
}

/// `grow` from the tables `hint` starts a build with under the budget
/// (`Sizes::margins`).
pub(super) async fn grow_hinted(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, (hint, upto): (Hint, u32)) -> Result<(Build, Sizes, Grew)> {
    let g = Growing::within(gpu, kit, t, tables, hint).await?;
    let g = g.grow_to(gpu, (kit, t, tables), (upto, true)).await?;
    Ok((g.b, g.sizes, g.grew))
}

/// A build growing (fn-270 R2): the build, its recorded cycle, its sizes
/// and how it grew, and what its loop carries from one cycle to the
/// next, so it grows on from any cycle it paused at as it would have
/// grown in one call.
pub(super) struct Growing {
    pub b: Build,
    /// The recorded cycle: none while paused (`build_pause.rs`).
    pub run: Option<(Plan, Phases)>,
    pub sizes: Sizes,
    pub grew: Grew,
    late: Late,
    /// The last two censuses read, a cycle late.
    prev: Option<Vec<u32>>,
    last: Option<Vec<u32>>,
    pending: VecDeque<Pending>,
    /// The last cycle after which the arena was compacted, and the census
    /// read after it: a census read before it counts more rows than the
    /// cycle begins with.
    compacted: Option<u32>,
    cut: Vec<u32>,
    /// The last cycle submitted's limits and workgroups given, which
    /// retirement's inside marks are taken under (`Build::inside`).
    submitted: (Limits, Vec<u32>),
    /// The cycles grown, and where it paused, the census its last cycle
    /// left, read early: the next cycle reads it in place of its own.
    pub committed: u32,
    pub(super) tail: Option<Vec<u32>>,
}

impl Growing {
    /// The build seeded from the largest tables `hint` gives that the
    /// device budget holds beside their first move (`Sizes::margins`,
    /// `Sizes::first_move`); refused by name where not even the least do.
    /// A start that held the budget with no room to move its largest group
    /// was refused mid-build (the spruce at 62, while 63 started smaller
    /// and built; fn-276 task 2).
    pub(super) async fn within(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, hint: Hint) -> Result<Self> {
        let mut refused = None;
        for sizes in Sizes::margins(hint) {
            let sizes = sizes?;
            let begun = Self::begin(gpu, kit, t, tables, sizes).await;
            let room = |g: Self| super::super::io::fits(gpu, sizes.first_move(), "first move").map(|()| g);
            match begun.and_then(room) {
                Err(e) if super::super::io::over_budget(&e) => refused = Some(e),
                begun => return begun,
            }
        }
        Err(refused.expect("a margin tried"))
    }

    /// The build seeded from `sizes`.
    pub(super) async fn begin(gpu: &Gpu, kit: &Kit, t: &Tables, tables: &Capture, sizes: Sizes) -> Result<Self> {
        if t.age >= MOST_CYCLES {
            return Err(super::super::io::error(format!("a tree of {} cycles: the device holds its time to {MOST_CYCLES} cycles", t.age)));
        }
        let mut grew = Grew::default();
        // A build let go before this one freed first (`Ledger::destroy`).
        super::super::io::complete(gpu).await?;
        let b = Build::new(gpu, t, (tables, sizes.caps, sizes.block_cap), sizes.lit())?;
        let run = b.plan(gpu, kit, t, sizes.bounds)?;
        grew.stamped = b.seeded(gpu, kit, (&run.0, &run.1));
        let submitted = (limits(&sizes, None, None), Vec::new());
        let (late, pending) = (Late::new(gpu)?, VecDeque::new());
        Ok(Self { b, run: Some(run), sizes, grew, late, prev: None, last: None, pending, compacted: None, cut: Vec::new(), submitted, committed: 0, tail: None })
    }

    /// Every cycle up to `upto` grown. Where it `ends` there, nothing
    /// after its last cycle is readied; else the build moves and compacts
    /// after it as it would before the next, and reads its census early.
    /// A count past any table or limit stops it by name, and a build that
    /// stopped is gone.
    pub(super) async fn grow_to(self, gpu: &Gpu, (kit, t, tables): (&Kit, &Tables, &Capture), (upto, ends): (u32, bool)) -> Result<Self> {
        if upto >= MOST_CYCLES {
            return Err(super::super::io::error(format!("a tree of {upto} cycles: the device holds its time to {MOST_CYCLES} cycles")));
        }
        let mut g = self.resumed(gpu, kit, t, tables).await?;
        // The cycle after which nothing is readied.
        let end = if ends { upto } else { t.age };
        for cycle in g.committed + 1..=upto {
            g = g.cycle(gpu, (kit, t, tables), (cycle, end)).await?;
        }
        g.finish(gpu, kit, t, (upto, ends)).await?;
        Ok(g)
    }

    /// Cycle `cycle` recorded and submitted; the census of the one before
    /// read and, before `end`, the arena moved and compacted for the next.
    async fn cycle(mut self, gpu: &Gpu, (kit, t, tables): (&Kit, &Tables, &Capture), (cycle, end): (u32, u32)) -> Result<Self> {
        let sizes = self.sizes;
        let (last, prev) = (&self.last, &self.prev);
        let limits = limits(&sizes, last.as_deref(), prev.as_deref());
        let seed = Build::seed_census();
        let (seen, before) = match (last, prev) {
            (Some(l), p) => (l.as_slice(), Some(p.as_deref().unwrap_or(&seed))),
            (None, _) => (seed.as_slice(), None),
        };
        let fallen = self.compacted.is_some_and(|k| k + 2 >= cycle).then_some(self.cut.as_slice());
        let span = Span::new(seen, before, (&sizes.ceilings(), fallen), (cycle, &limits));
        if std::env::var("CYCLE_TRACE").is_ok() {
            eprintln!("TRACE cycle {cycle} {limits:?}; light nodes {} points {} tips {}", sizes.nodes, sizes.points, sizes.tips);
            if let Some(l) = last {
                let n = (l[c::LEVELS] as usize).min(63);
                let widths: Vec<u32> = (0..n).map(|k| l[c::LVL + k + 1] - l[c::LVL + k]).collect();
                eprintln!("TRACE levels {} blocks by level {widths:?} live {} gens {} passes {} laygens {}", l[c::LEVELS], l[c::NEXT], l[c::GENS], super::build_size::passes(l), (0..super::lay_device::DEEP as usize).filter(|&g| l[c::LAY_DEEP + g] != 0).count());
            }
        }
        let clock = super::super::Clock::now();
        let run = self.run.as_ref().expect("a recorded cycle");
        let (stamped, sent, given) = self.b.submit(gpu, (kit, t), (&run.0, &run.1, &limits, &span), (&mut self.late, cycle));
        self.pending.push_back(sent);
        self.submitted = (limits, given);
        let grew = &mut self.grew;
        grew.recording += clock.elapsed_ms() / 1e3;
        grew.stamped.extend(stamped);
        #[cfg(test)]
        grew.ends.push(grew.stamped.len());
        if cycle == 1 {
            return Ok(self);
        }
        let read = match self.tail.take() {
            // The census a pause read early, its stop already checked.
            Some(read) => read,
            None => self.read(gpu, (cycle - 1, t.budget)).await?,
        };
        let mut g = self.ready(gpu, (kit, t, tables), (cycle, end), &read).await?;
        g.prev = g.last.replace(read);
        Ok(g)
    }

    /// The census of `cycle`, read and held to every table and limit; the
    /// cycle in flight read back before its buffers go where it stops.
    async fn read(&mut self, gpu: &Gpu, (cycle, budget): (u32, u32)) -> Result<Vec<u32>> {
        let clock = super::super::Clock::now();
        let read = self.late.take(gpu, cycle).await?;
        self.grew.waiting += clock.elapsed_ms() / 1e3;
        #[cfg(test)]
        self.grew.censuses.push(read.clone());
        let sent = self.pending.pop_front().expect("a cycle sent");
        if let Err(stopped) = self.grew.read((&read, cycle), &sent, budget) {
            if !self.pending.is_empty() {
                self.late.take(gpu, cycle + 1).await?;
            }
            return Err(stopped);
        }
        Ok(read)
    }

    /// The arena compacted and moved after `cycle`, by the census `read`
    /// of the one before: neither at `end`, after which no census is read.
    /// A compaction comes first, so a table grows only by the rows it
    /// keeps (fn-270 R4: the tables grow by observed counts).
    async fn ready(mut self, gpu: &Gpu, (kit, t, tables): (&Kit, &Tables, &Capture), (cycle, end): (u32, u32), read: &[u32]) -> Result<Self> {
        let clock = super::super::Clock::now();
        let sizes = self.sizes;
        let fresh = self.compacted.is_none_or(|k| k + 1 < cycle) || sizes.compact == Compact::Every;
        let compacting = cycle < end && fresh && sizes.compact.now(&sizes.caps, read);
        let mut run = self.run.take().expect("a recorded cycle");
        if compacting {
            // The inside marks a compaction drops by.
            self.grew.stamped.extend(self.b.inside(gpu, kit, (&run.0, &run.1), (cycle, &self.submitted.0, &self.submitted.1)));
            let traced = std::env::var("CYCLE_TRACE").is_ok();
            if traced {
                super::super::io::complete(gpu).await?;
            }
            let clock = super::super::Clock::now();
            let made = self.b.compact_in_place(gpu, kit, (&run.0.census, cycle), reach(read)).await?;
            self.grew.stamped.extend(std::iter::repeat_n(COMPACTION, made));
            if traced {
                super::super::io::complete(gpu).await?;
                eprintln!("TRACE compacted after cycle {cycle} in {:.1} ms", clock.elapsed_ms());
            }
            (self.grew.compactions, self.compacted) = (self.grew.compactions + 1, Some(cycle));
            let words = super::super::io::read_async(gpu, &run.0.census, 4 * c::WORDS as u64).await?;
            self.cut = bytemuck::cast_slice(&words).to_vec();
        }
        // The rows a compaction kept stand for the census read before it.
        // A compaction after this cycle counts its end, one before the
        // next's; one after the cycle before stands for the census read.
        let fallen = match self.compacted {
            Some(k) if k == cycle => Some((self.cut.as_slice(), 1)),
            Some(k) if k + 1 == cycle => Some((self.cut.as_slice(), 2)),
            _ => None,
        };
        // A move makes room for the next two cycles.
        if let Some(next) = (cycle < end).then(|| grown(sizes, (read, self.last.as_deref(), fallen), t.budget)).flatten() {
            self.b = self.b.moved(gpu, (t, tables), (sizes, next)).await?;
            let plan = self.b.plan(gpu, kit, t, next.bounds)?;
            let mut e = gpu.encoder(&Default::default());
            e.copy_buffer_to_buffer(&run.0.census, 0, &plan.0.census, 0, run.0.census.size());
            gpu.submit([e.finish()]);
            (run, self.sizes) = (plan, next);
            self.grew.moves += 1;
            self.grew.moved_at.push(cycle);
        }
        self.run = Some(run);
        self.grew.moving += clock.elapsed_ms() / 1e3;
        Ok(self)
    }

    /// The build after `upto`: retirement's inside marks the settle reads
    /// its retired roots by, and the last census read. Where it goes on,
    /// that census is held for the next cycle, and the arena was readied
    /// after `upto` as before it: the counts are the census's as the
    /// readying left them, read as the build pauses (`build_pause.rs`).
    async fn finish(&mut self, gpu: &Gpu, kit: &Kit, t: &Tables, (upto, ends): (u32, bool)) -> Result<()> {
        self.committed = upto;
        let run = self.run.as_ref().expect("a recorded cycle");
        self.grew.stamped.extend(self.b.inside(gpu, kit, (&run.0, &run.1), (upto, &self.submitted.0, &self.submitted.1)));
        let read = self.read(gpu, (upto, t.budget)).await?;
        self.b.goes_on = !ends;
        if !ends {
            self.tail = Some(read);
            return Ok(());
        }
        self.b.counted(&read);
        if std::env::var("CYCLE_TRACE").is_ok() {
            trace(&self.grew, &self.sizes, upto);
        }
        Ok(())
    }
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
