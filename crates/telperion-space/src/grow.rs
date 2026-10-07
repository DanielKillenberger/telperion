//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops. Every draw is
//! keyed to the lineage it decides (`lineage.rs`), and its lead past its
//! bound is recorded; once the tree has grown, `presence.rs` sizes every
//! element from those leads, so a setting that crosses a draw grows the
//! element in from nothing.
mod advance;
pub(crate) mod alone;
mod balance;
pub(crate) mod couple;
mod draw;
pub(crate) mod oracle;
mod parallel;
pub(crate) mod partition;
pub(crate) mod realized;
mod relay;
mod settle;
mod shoot;
mod sketch;
mod stage;
pub(crate) mod stepped;
mod stop;
pub(crate) mod subtree;
#[cfg(test)]
mod tests;
mod unit;
mod wake;

use crate::dormant::{Sleeper, Woken};
use crate::error::{refuse, Error, Result};
use crate::fine::lineage_order;
use crate::fixed::presence::{q, real, times, ONE};
use crate::light::Light;
use crate::lineage::Key;
use crate::presence::{Draws, Windows};
use crate::shed::lasting;
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
pub(crate) use shoot::Rules;
use sketch::Sketch;
pub use stage::{Pass, Stage};
pub(crate) use unit::node_wood;

/// What to grow: cycles, the seed, the phytomer budget and the site's light.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
    pub age: u32,
    pub seed: u64,
    /// The most phytomers the tree may grow, the shed ones included.
    pub budget: u32,
    pub light: Light,
}

/// A bud of a node: its PA and lead (log-odds, Q32), and the share its
/// place stands at; or none for a bare bud.
type Placed = Option<(usize, i64, f64)>;

/// A living apex: the time it has spent in its current PA.
#[derive(Debug, Clone, Copy)]
struct Apex {
    axis: usize,
    spent: f64,
}

/// Grows, sheds and places the tree, in its canonical lineage order, by
/// the stepped executor (fn-211 task 12, host decision (b)): every fine
/// root at the cut N = 1,024 its own grower, run ahead to its next
/// synchronisation point and settled per root.
pub fn grow(species: &Species, request: Request) -> Result<Structure> {
    stepped::build(species, request).map(lineage_order)
}

/// Today's path before task 12, kept for its measurement until it is
/// deleted: the whole-tree engine where any bud reads light or the tree
/// as grown, the realized partition otherwise.
pub fn grow_today(species: &Species, request: Request) -> Result<Structure> {
    if species.reads_light() || keeps(species) {
        return grow_whole(species, request);
    }
    species.validate()?;
    request.light.validate()?;
    if request.age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let fine = crate::fine::grow_fine(species, request, crate::fine::Plan::default())?;
    Ok(lineage_order(fine.structure))
}

/// Today's whole-tree engine, every axis grown in one grower: the tests'
/// reference for `grow`, in the same canonical order.
pub fn grow_whole(species: &Species, request: Request) -> Result<Structure> {
    grow_staged(species, request, &mut |_| {})
}

/// `grow_whole`, reporting each stage as it ends.
pub fn grow_staged(
    species: &Species,
    request: Request,
    stage: &mut dyn FnMut(Stage),
) -> Result<Structure> {
    run(species, request, stage, true, Options::default()).map(|g| lineage_order(g.structure))
}

/// How the engine grows a tree, never what it grows: every choice here
/// leaves the tree the same to the bit, which the tests hold.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Options {
    pub certain: Certain,
    /// The cores a cycle grows on; none, all the machine has (fn-210,
    /// lever 2); and the living apexes in each share of it.
    pub threads: Option<usize>,
    pub share: usize,
    /// Whether the light work no bud reads is done as well: the rough
    /// layout where light shades but no bud reads it, and the last
    /// cycle's layout, full lay and sweep (fn-210, lever 3).
    pub unread_light: bool,
    /// Whether the tree stops once settled, before its girth and lay,
    /// keeping every lateral bud made as a record (`bounded.rs`, fn-209
    /// option 4): what a bounded build resolves chain by chain.
    pub settle: bool,
    /// Whether a laid tree keeps its lateral buds' records as a settled
    /// one does (fn-211.4: a laid tree surveyed at a cut).
    pub records: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            certain: Certain::Skip,
            threads: None,
            share: parallel::SHARE,
            unread_light: false,
            settle: false,
            records: false,
        }
    }
}

/// What becomes of a lateral certain to be shed by the tree's age where
/// nothing reads it while it lives (fn-210, lever 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum Certain {
    /// It is grown.
    Grow,
    /// It is grown, and its lineage marked.
    Mark,
    /// It is not grown.
    Skip,
}

/// The tree, the phytomers grown for it, the shed ones included, and the
/// lineages of the laterals marked certain to be shed.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct Grown {
    pub structure: Structure,
    pub grown: u64,
    pub marked: Vec<u64>,
    /// With `Options::settle`, every lateral bud made whose bearer is
    /// kept, by the kept tree's indices.
    pub records: Vec<Record>,
    /// The leaf term's mean over the tree as grown, where any PA sets it.
    pub leaf_mean: Option<f64>,
    /// Each grown axis's index in the shed tree (`usize::MAX`: shed).
    pub index: Vec<usize>,
}

/// A lateral bud as its bearer made it, before shedding: where it stands,
/// its PA, birth, lineage and vigour as made, and its kept axis, if any.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Record {
    pub origin: Origin,
    pub pa: usize,
    pub birth: u32,
    pub lineage: u64,
    pub vigour: f64,
    pub axis: Option<usize>,
}

/// The tree as `grow` grows it, with every phytomer where the rough
/// layout grown with it put it, before the final lay: what light read as
/// the tree grew, in the same lineage order. Without shade it has no
/// layout and is refused.
pub fn sketch(species: &Species, request: Request) -> Result<Structure> {
    if !request.light.shades() {
        return refuse(
            "light.extinction",
            "a rough layout is grown only where light shades",
        );
    }
    run(species, request, &mut |_| {}, false, Options::default())
        .map(|g| lineage_order(g.structure))
}

pub(crate) fn run(
    species: &Species,
    request: Request,
    stage: &mut dyn FnMut(Stage),
    lay: bool,
    options: Options,
) -> Result<Grown> {
    run_beside(species, request, (stage, lay), options, Beside::default()).map(|g| g.0)
}

/// What fn-211.4 adds to a run: the light oracle, or the coupling pass.
#[derive(Default)]
pub(crate) struct Beside {
    pub oracle: Option<oracle::Oracle>,
    pub couple: Option<couple::Pass>,
}

/// What they hand back.
pub(crate) struct Besides {
    pub oracle: Option<Box<oracle::Oracle>>,
    pub measured: Option<subtree::Measured>,
}

fn run_beside(
    species: &Species,
    request: Request,
    (stage, lay): (&mut dyn FnMut(Stage), bool),
    options: Options,
    beside: Beside,
) -> Result<(Grown, Besides)> {
    let started = std::time::Instant::now();
    let (mut grower, keeps) = start(species, request, lay, options)?;
    grower.oracle = beside.oracle.map(Box::new);
    grower.couple = beside.couple.map(Box::new);
    stage(Stage::Pass(Pass::Windows));
    let mut advanced = Vec::new();
    for cycle in 1..=request.age {
        // The last cycle's layout, full lay and light are read by no bud,
        // and the final lay rewrites every place (fn-198, lever c1); only
        // `sketch` keeps them.
        let sketching = cycle < request.age || !lay || options.unread_light;
        grower.year(cycle, sketching, &mut advanced, stage)?;
    }
    let measured = match grower.couple.is_some() {
        true => Some(grower.grow_fine_all(started)?),
        false => None,
    };
    let oracle = grower.oracle.take();
    let grown = grower.settle((request, options), (lay, keeps), stage)?;
    Ok((grown, Besides { oracle, measured }))
}

/// The grower of `request` before its first cycle, and whether any PA
/// reads the tree as grown (its shed pipes or its leaves' girth).
fn start<'a>(
    species: &'a Species,
    request: Request,
    lay: bool,
    options: Options,
) -> Result<(Grower<'a>, bool)> {
    species.validate()?;
    request.light.validate()?;
    if request.age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let laterals: Vec<_> = species.states.iter().map(PaState::laterals).collect();
    // The rough layout is grown only where light shades and a bud
    // reads it, or where `sketch` asks for it: without a reader the
    // tree is the same to the bit (fn-197's `tests/light.rs`).
    let read = species.reads_light() || !lay || options.unread_light;
    let sketch = (request.light.shades() && read).then(|| Sketch::new(request.light));
    // What shedding will drop is grown only where something reads it
    // while it lives: the light, or the girth of what bore it.
    let keeps = keeps(species);
    let lasting = if options.certain != Certain::Grow && sketch.is_none() && !keeps {
        lasting(species, &laterals)
    } else {
        vec![None; species.states.len()]
    };
    let threads = options.threads.unwrap_or_else(crate::cores::available);
    let rules = Rules {
        species,
        laterals: laterals.into(),
        lasting,
        certain: options.certain,
        windows: Windows::new(species, request.age),
        age: request.age,
        budget: request.budget,
        grown: 0,
        alone: false,
        hands: None,
    };
    let grower = Grower::seeded(rules, (threads, options.share), sketch, request.seed)?;
    Ok((grower, keeps))
}

/// The share of an apex's expected wood a stop decides: a stop with no
/// relay ends it all, and a stop that relays decides only the difference
/// between stopping and relaying and carrying on. A relay carries on the
/// axis's PA and growth units, so it is expected to grow what the apex
/// would have, and the difference is the share that does not relay.
fn stop_stake(relay: f64) -> f64 {
    1.0 - relay
}

fn bud(key: Key, pa: usize, birth: u32, origin: Origin) -> Axis {
    Axis {
        lineage: key.0,
        pa,
        birth,
        origin,
        vigour: 1.0,
        blend: 1.0,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: Vec::new(),
        units: Vec::new(),
        alive: 0.0,
        rank: 0.0,
        sleep: 0.0,
    }
}

struct Grower<'a> {
    /// What every growth unit reads (`shoot.rs`).
    rules: Rules<'a>,
    species: &'a Species,
    /// The cores a cycle grows on, and the living apexes in each share.
    threads: usize,
    share: usize,
    /// Each axis's horizon: the most, past its subtree's last growth, that
    /// it or a lateral or relay it stands on keeps it before it is shed
    /// (a delay and the cycle its fade runs over).
    horizon: Vec<f64>,
    marked: Vec<u64>,
    axes: Vec<Axis>,
    /// Each axis's draws' presences, by index.
    draws: Vec<Draws>,
    live: Vec<Apex>,
    /// Each axis's time spent in its PA when its apex stopped, or at its
    /// birth while it lives: what a relay of it carries on.
    units: Vec<f64>,
    /// Each axis's continuation or relay, or both where a stop it all but
    /// made grows both (host decision 11): what carries it on.
    successor: Vec<Vec<usize>>,
    /// The lineage that began each axis's chain of ages: the seed's, a
    /// lateral's own, or the one its continuation or relay carries on.
    roots: Vec<u64>,
    /// The sleeping buds that wake in each cycle, and those that woke.
    asleep: Vec<Vec<Sleeper>>,
    woken: Vec<Woken>,
    /// The rough layout, where light shades, and the leaves of the cycle.
    sketch: Option<Sketch>,
    leaves: Vec<(Vec3, f64)>,
    /// Where a chain grows alone, the sleeping buds that woke on it.
    wakes: Vec<alone::Pending>,
    /// fn-211.4's light oracle, where the tree is grown under one; the
    /// coupling pass, where it is one; a fine subtree's records, where
    /// it is one.
    oracle: Option<Box<oracle::Oracle>>,
    couple: Option<Box<couple::Pass>>,
    fine: Option<Box<subtree::Reads>>,
    /// Above a cut, each sleeping bud handed on as it woke: its
    /// stand-in's index and its waking (`realized.rs`).
    handed: Vec<(usize, alone::Pending)>,
}

/// Whether any PA reads the tree as grown: its shed pipes or its leaves'
/// girth.
pub(crate) fn keeps(species: &Species) -> bool {
    species
        .states
        .iter()
        .any(|s| s.retained > 0.0 || s.leaf_girth > 0.0)
}

impl<'a> Grower<'a> {
    /// A grower holding the seed's bud of the youngest PA, about to grow
    /// its first cycle on `threads` cores in shares of `share` apexes.
    fn seeded(
        rules: Rules<'a>,
        (threads, share): (usize, usize),
        sketch: Option<Sketch>,
        seed: u64,
    ) -> Result<Self> {
        let (species, age) = (rules.species, rules.age);
        let root = Key::root(seed);
        // The seed grows in the first age it does not pass through, as
        // far as it reaches it.
        let (pa, reach) = species.lived(0).ok_or(Error::Collapsed)?;
        Ok(Self {
            rules,
            species,
            threads,
            share,
            horizon: vec![f64::NEG_INFINITY],
            marked: Vec::new(),
            axes: vec![bud(root.onto(pa), pa, 0, Origin::Seed)],
            draws: vec![Draws {
                birth: [q(reach), ONE].map(real),
                ..Draws::default()
            }],
            live: vec![Apex {
                axis: 0,
                spent: 0.0,
            }],
            units: vec![0.0],
            successor: vec![Vec::new()],
            roots: vec![root.0],
            asleep: vec![Vec::new(); age as usize + 2],
            woken: Vec::new(),
            sketch,
            leaves: Vec::new(),
            wakes: Vec::new(),
            handed: Vec::new(),
            oracle: None,
            couple: None,
            fine: None,
        })
    }
}

impl Grower<'_> {
    /// One cycle: each living lateral subtree's balance and each apex's
    /// share of the tree's vigour, then every living apex's growth unit,
    /// on every core (`parallel.rs`), then the sleeping buds that wake.
    fn step(&mut self, cycle: u32, report: &mut dyn FnMut(Stage)) -> Result<()> {
        let rows = self.rows();
        self.balance(cycle, rows.as_ref());
        self.hash_phase(cycle, 0);
        report(Stage::Pass(Pass::Balance));
        self.allot(rows.as_ref(), cycle);
        drop(rows);
        self.advance_fine(cycle);
        self.hash_phase(cycle, 1);
        report(Stage::Pass(Pass::Allot));
        let live = std::mem::take(&mut self.live);
        self.grow_live(live, cycle, report)?;
        self.wake(cycle)?;
        self.hash_phase(cycle, 2);
        report(Stage::Pass(Pass::Wake));
        Ok(())
    }

    /// One cycle: its growth, then, `sketching` where a rough layout is
    /// grown, its layout, any re-lay and its light.
    fn year(
        &mut self,
        cycle: u32,
        sketching: bool,
        advanced: &mut Vec<usize>,
        stage: &mut dyn FnMut(Stage),
    ) -> Result<()> {
        advanced.clear();
        advanced.extend(self.live.iter().map(|a| a.axis));
        self.step(cycle, stage)?;
        stage(Stage::Grown);
        if self.sketch.is_none() || !sketching {
            return Ok(());
        }
        self.sketch(cycle, advanced)?;
        stage(Stage::Pass(Pass::Sketch));
        stage(Stage::Sketched);
        if cycle % relay::RELAY_EVERY == 0 {
            self.relay(cycle, stage)?;
            stage(Stage::Relaid);
        }
        self.shade(cycle, stage);
        stage(Stage::Lit);
        Ok(())
    }

    /// How much an apex living at the tree's age lives on: whole while a
    /// cycle or more of its age is left, and otherwise the share left plus
    /// the rest as far as its continuation or relay carries it on, so an
    /// apex an age all but spent lives as little as one that has ended.
    fn living(&self, apex: Apex) -> f64 {
        let pa = self.axes[apex.axis].pa;
        let state = &self.species.states[pa];
        // Q32, floored in turn (host decision 8); the times on the grid.
        let left = q((state.life() - apex.spent).clamp(0.0, 1.0));
        let go = q(self.species.successor(pa).map_or(0.0, |(_, go)| go));
        let on = go + times(ONE - go, q(state.relay_ended));
        real(left + times(ONE - left, on))
    }
}
