//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops. Every draw is
//! keyed to the lineage it decides (`lineage.rs`), and its lead past its
//! bound is recorded; once the tree has grown, `presence.rs` sizes every
//! element from those leads, so a setting that crosses a draw grows the
//! element in from nothing.
mod advance;
mod balance;
mod parallel;
mod relay;
mod shoot;
mod sketch;
mod stop;
#[cfg(test)]
mod tests;
mod unit;
mod wake;

use crate::dormant::{Sleeper, Woken};
use crate::error::{refuse, Error, Result};
use crate::geometry::{place, scale};
use crate::girth::{attachments, disused, thicken, Girth};
use crate::light::Light;
use crate::lineage::Key;
use crate::presence::{assign, Draws, Windows};
use crate::sag;
use crate::shed::{lasting, shed};
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
use shoot::Rules;
use sketch::Sketch;

/// What to grow: cycles, the seed, the phytomer budget and the site's light.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
    pub age: u32,
    pub seed: u64,
    /// The most phytomers the tree may grow, the shed ones included.
    pub budget: u32,
    pub light: Light,
}

/// A bud of a node: its PA and lead, and the share its place stands at;
/// or none for a bare bud.
type Placed = Option<(usize, f64, f64)>;

/// A living apex: the time it has spent in its current PA.
#[derive(Debug, Clone, Copy)]
struct Apex {
    axis: usize,
    spent: f64,
}

/// The stages a tree passes through, reported as each ends: per cycle its
/// growth, its rough layout, its full lay every few cycles and the sweep
/// of its light (where light shades), then once its sizes, shedding and
/// girth, and its final lay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Grown,
    Sketched,
    /// The full lay of the living tree, every `RELAY_EVERY` cycles.
    Relaid,
    Lit,
    Settled,
    Laid,
}

/// Grows, sheds and places the tree.
pub fn grow(species: &Species, request: Request) -> Result<Structure> {
    grow_staged(species, request, &mut |_| {})
}

/// `grow`, reporting each stage as it ends.
pub fn grow_staged(
    species: &Species,
    request: Request,
    stage: &mut dyn FnMut(Stage),
) -> Result<Structure> {
    run(species, request, stage, true, Options::default()).map(|g| g.structure)
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
}

impl Default for Options {
    fn default() -> Self {
        Self {
            certain: Certain::Skip,
            threads: None,
            share: parallel::SHARE,
            unread_light: false,
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
}

/// The tree as `grow` grows it, with every phytomer where the rough
/// layout grown with it put it, before the final lay: what light read as
/// the tree grew. Without shade it has no layout and is refused.
pub fn sketch(species: &Species, request: Request) -> Result<Structure> {
    if !request.light.shades() {
        return refuse(
            "light.extinction",
            "a rough layout is grown only where light shades",
        );
    }
    run(species, request, &mut |_| {}, false, Options::default()).map(|g| g.structure)
}

pub(crate) fn run(
    species: &Species,
    request: Request,
    stage: &mut dyn FnMut(Stage),
    lay: bool,
    options: Options,
) -> Result<Grown> {
    species.validate()?;
    request.light.validate()?;
    if request.age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let root = Key::root(request.seed);
    // The seed grows in the first age it does not pass through, as far as
    // it reaches it.
    let (seed, reach) = species.lived(0).ok_or(Error::Collapsed)?;
    let laterals: Vec<_> = species.states.iter().map(PaState::laterals).collect();
    // The rough layout is grown only where light shades and a bud
    // reads it, or where `sketch` asks for it: without a reader the
    // tree is the same to the bit (fn-197's `tests/light.rs`).
    let read = species.reads_light() || !lay || options.unread_light;
    let sketch = (request.light.shades() && read).then(|| Sketch::new(request.light));
    // What shedding will drop is grown only where something reads it
    // while it lives: the light, or the girth of what bore it.
    let keeps = species
        .states
        .iter()
        .any(|s| s.retained > 0.0 || s.leaf_girth > 0.0);
    let lasting = if options.certain != Certain::Grow && sketch.is_none() && !keeps {
        lasting(species, &laterals)
    } else {
        vec![None; species.states.len()]
    };
    let threads = options.threads.unwrap_or_else(|| {
        std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
    });
    let mut grower = Grower {
        rules: Rules {
            species,
            laterals,
            lasting,
            certain: options.certain,
            windows: Windows::new(species, request.age),
            age: request.age,
            budget: request.budget,
            grown: 0,
        },
        species,
        threads,
        share: options.share,
        horizon: vec![f64::NEG_INFINITY],
        marked: Vec::new(),
        axes: vec![bud(root.onto(seed), seed, 0, Origin::Seed)],
        draws: vec![Draws {
            birth: [reach, 1.0],
            ..Draws::default()
        }],
        live: vec![Apex {
            axis: 0,
            spent: 0.0,
        }],
        units: vec![0.0],
        successor: vec![Vec::new()],
        roots: vec![root.0],
        asleep: vec![Vec::new(); request.age as usize + 2],
        woken: Vec::new(),
        sketch,
        leaves: Vec::new(),
    };
    let mut advanced = Vec::new();
    for cycle in 1..=request.age {
        advanced.clear();
        advanced.extend(grower.live.iter().map(|a| a.axis));
        grower.step(cycle)?;
        stage(Stage::Grown);
        // The last cycle's layout, full lay and light are read by no bud,
        // and the final lay rewrites every place (fn-198, lever c1); only
        // `sketch` keeps them.
        if grower.sketch.is_some() && (cycle < request.age || !lay || options.unread_light) {
            grower.sketch(cycle, &advanced)?;
            stage(Stage::Sketched);
            if cycle % relay::RELAY_EVERY == 0 {
                grower.relay(cycle)?;
                stage(Stage::Relaid);
            }
            grower.shade();
            stage(Stage::Lit);
        }
    }
    for apex in &grower.live {
        let living = grower.living(*apex);
        let draws = &mut grower.draws[apex.axis];
        (draws.alive, draws.living) = (true, living);
    }
    for woken in &grower.woken {
        let kept = woken.kept(&grower.axes, &grower.draws, &grower.successor, request.age);
        grower.draws[woken.axis].birth[1] *= kept;
    }
    assign(&mut grower.axes, &grower.draws);
    // The tree as grown, where any PA keeps shed pipes or thickens by its
    // leaves: what each shed branch had laid down, and the one population
    // the leaf term's mean is taken over at any retained share.
    let grown = keeps.then(|| attachments(&grower.axes, species, request.age));
    let shed = shed(grower.axes, species, request.age);
    let girth = grown.map_or_else(Girth::default, |g| disused(&g, &shed, species));
    let mut structure = Structure {
        age: request.age,
        pas: species.states.len(),
        axes: shed.axes,
    };
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    scale(&mut structure, species);
    // Girth needs no geometry; placing reads it where wood meets the
    // ground.
    thicken(&mut structure, species, &girth);
    stage(Stage::Settled);
    let (count, marked) = (grower.rules.grown, grower.marked);
    let done = move |structure| Grown {
        structure,
        grown: count,
        marked,
    };
    if !lay {
        return Ok(done(structure));
    }
    place(&mut structure, species, None)?;
    // Sag bends the tree as it stands under the load it carries, and
    // leaves its girth as it was.
    if sag::any(species) {
        let levers = sag::levers(&structure, species);
        place(&mut structure, species, Some(&levers))?;
    }
    // Wood that stands exactly at its draw has no size.
    if structure
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .all(|p| p.scale == 0.0)
    {
        return Err(Error::Collapsed);
    }
    stage(Stage::Laid);
    Ok(done(structure))
}

/// The share of an apex's expected wood a stop decides: a stop with no
/// relay ends it all, and a stop that relays decides only the difference
/// between stopping and relaying and carrying on. A relay carries on the
/// axis's PA and growth units, so it is expected to grow what the apex
/// would have, and the difference is the share that does not relay.
fn stop_stake(relay: f64) -> f64 {
    1.0 - relay
}

/// A survival over a share of the year: to the share, itself whole.
fn shared(p: f64, share: f64) -> f64 {
    if share < 1.0 {
        p.powf(share)
    } else {
        p
    }
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
}

impl Grower<'_> {
    /// One cycle: each living lateral subtree's balance and each apex's
    /// share of the tree's vigour, then every living apex's growth unit,
    /// on every core (`parallel.rs`), then the sleeping buds that wake.
    fn step(&mut self, cycle: u32) -> Result<()> {
        self.balance(cycle);
        self.allot();
        let live = std::mem::take(&mut self.live);
        self.grow_live(live, cycle)?;
        self.wake(cycle)
    }

    /// How much an apex living at the tree's age lives on: whole while a
    /// cycle or more of its age is left, and otherwise the share left plus
    /// the rest as far as its continuation or relay carries it on, so an
    /// apex an age all but spent lives as little as one that has ended.
    fn living(&self, apex: Apex) -> f64 {
        let pa = self.axes[apex.axis].pa;
        let state = &self.species.states[pa];
        let left = (state.lifespan - apex.spent).clamp(0.0, 1.0);
        let go = self.species.successor(pa).map_or(0.0, |(_, go)| go);
        left + (1.0 - left) * (go + (1.0 - go) * state.relay_ended)
    }
}
