//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops. Every draw is
//! keyed to the lineage it decides (`lineage.rs`), and its lead past its
//! bound is recorded; once the tree has grown, `presence.rs` sizes every
//! element from those leads, so a setting that crosses a draw grows the
//! element in from nothing.
mod balance;
mod relay;
mod sketch;
mod stop;
mod unit;
mod wake;

use crate::dormant::{Sleeper, Woken};
use crate::error::{refuse, Error, Result};
use crate::geometry::{place, scale};
use crate::girth::{attachments, disused, thicken, Girth};
use crate::light::Light;
use crate::lineage::{above, below, Key, ABORTION, END, ENDED, VIABILITY};
use crate::presence::{assign, Draws, Windows};
use crate::sag;
use crate::schedule::Carried;
use crate::shed::shed;
use crate::species::{PaState, Species, MAX_BUDS};
use crate::structure::{Axis, Origin, Structure, Vec3};
use sketch::Sketch;
use stop::Stop;

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
    run(species, request, stage, true)
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
    run(species, request, &mut |_| {}, false)
}

fn run(
    species: &Species,
    request: Request,
    stage: &mut dyn FnMut(Stage),
    lay: bool,
) -> Result<Structure> {
    species.validate()?;
    request.light.validate()?;
    if request.age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let root = Key::root(request.seed);
    // The seed grows in the first age it does not pass through, as far as
    // it reaches it.
    let (seed, reach) = species.lived(0).ok_or(Error::Collapsed)?;
    let mut grower = Grower {
        species,
        laterals: species.states.iter().map(PaState::laterals).collect(),
        axes: vec![bud(root.onto(seed), seed, 0, Origin::Seed)],
        windows: Windows::new(species, request.age),
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
        age: request.age,
        next: Vec::new(),
        leads: Vec::new(),
        node_buds: Vec::new(),
        sleeping: Vec::new(),
        grown: 0,
        budget: request.budget,
        sketch: request.light.shades().then(|| Sketch::new(request.light)),
        leaves: Vec::new(),
    };
    let mut advanced = Vec::new();
    for cycle in 1..=request.age {
        advanced.clear();
        advanced.extend(grower.live.iter().map(|a| a.axis));
        grower.step(cycle)?;
        stage(Stage::Grown);
        if grower.sketch.is_some() {
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
    let grown = species
        .states
        .iter()
        .any(|s| s.retained > 0.0 || s.leaf_girth > 0.0)
        .then(|| attachments(&grower.axes, species, request.age));
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
    if !lay {
        return Ok(structure);
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
    Ok(structure)
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
    species: &'a Species,
    /// Each PA's zones' lateral probabilities as drawn (`PaState::laterals`).
    laterals: Vec<Vec<Vec<f64>>>,
    axes: Vec<Axis>,
    windows: Windows,
    /// Each axis's draws' presences, by index.
    draws: Vec<Draws>,
    live: Vec<Apex>,
    next: Vec<Apex>,
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
    age: u32,
    /// One zone's node leads and its nodes (order draw, draw index, lead,
    /// buds), reused.
    leads: Vec<f64>,
    node_buds: Vec<(f64, u64, f64, [Placed; MAX_BUDS as usize])>,
    /// One zone's sleeping buds' expected wood per PA, reused.
    sleeping: Vec<f64>,
    grown: u32,
    budget: u32,
    /// The rough layout, where light shades, and the leaves of the cycle.
    sketch: Option<Sketch>,
    leaves: Vec<(Vec3, f64)>,
}

impl Grower<'_> {
    fn step(&mut self, cycle: u32) -> Result<()> {
        self.balance(cycle);
        self.allot();
        let live = std::mem::take(&mut self.live);
        for apex in live.iter().copied() {
            self.advance(apex, cycle)?;
        }
        self.wake(cycle)?;
        self.live = std::mem::replace(&mut self.next, live);
        self.next.clear();
        Ok(())
    }

    /// One living apex's growth unit in `cycle`: its survival, the unit,
    /// then its abortion or its move to its next PA. A unit that ends its
    /// age within the cycle hands the rest of it on (`schedule.rs`).
    fn advance(&mut self, mut apex: Apex, cycle: u32) -> Result<()> {
        let pa = self.axes[apex.axis].pa;
        let state = &self.species.states[pa];
        // A unit's draws are keyed by the cycle it grows in, so a unit that
        // a lifespan's fraction makes or unmakes renumbers none after it
        // (host decision 7).
        let unit = Key(self.axes[apex.axis].lineage).child(u64::from(cycle));
        // Its stop decisions are drawn apart for each relay of an age that
        // ended, the first of which may grow in the cycle its axis stopped
        // in: apart from that axis's, and the same whether the age ended
        // within the cycle or at its start (Codex rounds 3 and 4 on fn-206).
        let decide = match self.draws[apex.axis].ended_relays {
            0 => unit,
            n => unit.child(ENDED).child(u64::from(n)),
        };
        // The share of the cycle gone before the first unit: asleep, or to
        // the age before; its risks run over the unit's own share.
        let draws = &self.draws[apex.axis];
        let first = draws.units.is_empty();
        // The relay of an age that ended or a unit that failed grows a
        // whole first unit.
        let relay = draws.ended;
        let carried = Carried {
            spent: apex.spent,
            aged: 0.0,
            gone: if first { draws.sleep } else { 0.0 },
            relay: first && relay,
        };
        let step = carried.unit(state.lifespan, 1);
        let share = step.share;
        // Shade raises the apex's death hazard, ln survival = ln
        // viability x light^-phi (host, 2026-10-05): near-certain survival
        // moves gently as light does, and certain survival not at all.
        let light = self.light(apex.axis);
        let viability = if state.shade_hazard > 0.0 && light < 1.0 {
            state.viability.powf(light.powf(-state.shade_hazard))
        } else {
            state.viability
        };
        let viability = shared(viability, share);
        let u = decide.child(VIABILITY).unit();
        if u >= viability {
            self.axes[apex.axis].apex_end = Some(cycle - 1);
            self.draws[apex.axis].failed = true;
            // The unit that failed is spent: a relay's first unit draws
            // anew.
            let spent = apex.spent + share;
            self.stop(
                apex,
                (cycle, 1.0),
                above(u, viability),
                decide,
                spent,
                Stop::Failed,
            )?;
            return Ok(());
        }
        let survive = below(u, viability);
        let wood = self.windows.wood(pa, cycle);
        let survive = self
            .windows
            .decided(survive, wood, stop_stake(state.relay_failed));
        // And its subtrees' survival of shedding on their balance.
        let survive = survive * self.kept(apex.axis);
        self.draws[apex.axis].units.push([survive, 1.0]);
        self.draws[apex.axis].shares.push(share);
        if self.sketch.is_some() {
            let size = self.size(apex.axis);
            self.draws[apex.axis].sizes.push(size);
            if self.species.states.iter().any(|s| s.leaf_girth > 0.0) {
                self.draws[apex.axis].lights.push(light);
            }
        }
        self.grow_unit(apex, pa, unit, cycle)?;
        apex.spent += share;
        // An apex that has spent its PA's lifespan moves on; it does not
        // also abort. Its hazard counts the time its axis has grown, and
        // runs over the share this unit and the next leave it exposed
        // (`schedule.rs`).
        let draws = &self.draws[apex.axis];
        let grown = draws.aged + draws.shares.iter().sum::<f64>();
        let exposure = carried.exposure(state.lifespan, 1);
        let abortion = 1.0 - shared(1.0 - state.abortion_at(grown), exposure);
        if abortion > 0.0 && !step.ends {
            let u = decide.child(ABORTION).unit();
            if u < abortion {
                self.axes[apex.axis].apex_end = Some(cycle);
                self.stop(
                    apex,
                    (cycle, 1.0),
                    below(u, abortion),
                    decide,
                    apex.spent,
                    Stop::Aborted,
                )?;
                return Ok(());
            }
            // An apex that all but aborted carries on as itself and as the
            // relay it would have had, which at the bound is laid as its
            // continuation and draws what it would: the two sum, so no
            // stop near its bound fades the stem (host decision 11).
            let wood = self.windows.wood(pa, cycle + 1);
            let faded = self
                .windows
                .decided(above(u, abortion), wood, stop_stake(state.relay));
            let relayed = self.relayed(state.relay, decide, wood);
            let units = &mut self.draws[apex.axis].units;
            units.last_mut().unwrap()[1] = relayed + (1.0 - relayed) * faded;
        }
        if !step.ends {
            self.next.push(apex);
            return Ok(());
        }
        self.axes[apex.axis].apex_end = Some(cycle);
        let used = if step.same_cycle() { step.used } else { 1.0 };
        // Its end is drawn once for its age, whatever cycle it falls in,
        // so a lifespan crossing a whole cycle moves no draw (Codex round
        // 7 on fn-206); apart for each relay of an age that ended.
        let ends = Key(self.axes[apex.axis].lineage)
            .child(END)
            .child(u64::from(self.draws[apex.axis].ended_relays));
        self.move_on(apex, ends, cycle, used)
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

    /// The bud last sprouted grows from the next cycle, or, where the
    /// cycle's `used` share is not whole, in this one at the rest of it.
    fn carry_on(&mut self, cycle: u32, used: f64) -> Result<()> {
        if used >= 1.0 {
            return Ok(());
        }
        let apex = self.next.pop().expect("sprouted");
        self.axes[apex.axis].birth = cycle - 1;
        self.draws[apex.axis].sleep = used;
        self.advance(apex, cycle)
    }

    /// A new bud that grows from the next cycle, made by draws with these
    /// presences. A seed or lateral `key` is the bud's own, and its axis is
    /// keyed by its age's place on the reference axis from it, as every
    /// continuation of it is (`Key::onto`), so an age it passes through or
    /// grows in moves no key; a continuation's or relay's `key` is its
    /// axis's own. A relay carries on its axis's time spent in its PA.
    fn sprout(&mut self, key: Key, pa: usize, cycle: u32, origin: Origin, made: [f64; 2]) {
        let spent = match origin {
            Origin::Relay { parent, .. } => self.units[parent],
            _ => 0.0,
        };
        self.units.push(spent);
        self.successor.push(Vec::new());
        let (root, lineage) = match origin {
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                (self.roots[parent], key)
            }
            Origin::Seed | Origin::Lateral { .. } => (key.0, key.onto(pa)),
        };
        self.roots.push(root);
        self.next.push(Apex {
            axis: self.axes.len(),
            spent,
        });
        self.axes.push(bud(lineage, pa, cycle, origin));
        self.draws.push(Draws {
            birth: made,
            ..Draws::default()
        });
    }
}
