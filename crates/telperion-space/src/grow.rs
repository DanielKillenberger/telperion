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
mod unit;
mod wake;

use crate::dormant::{Sleeper, Woken};
use crate::error::{refuse, Error, Result};
use crate::geometry::{place, scale};
use crate::girth::{attachments, disused, thicken, Girth};
use crate::light::Light;
use crate::lineage::{above, below, Key, ABORTION, CONTINUATION, RELAY, VIABILITY};
use crate::presence::{assign, Draws, Windows, SPAN};
use crate::sag;
use crate::shed::shed;
use crate::species::{PaState, Species, MAX_BUDS};
use crate::structure::{Axis, Origin, Structure, Vec3};
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

/// A bud of a node: its PA and lead, or none for a bare bud.
type Bud = Option<(usize, f64)>;

/// A living apex: the growth units it has grown in its current PA.
#[derive(Debug, Clone, Copy)]
struct Apex {
    axis: usize,
    units: u32,
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
    let mut grower = Grower {
        species,
        laterals: species.states.iter().map(PaState::laterals).collect(),
        axes: vec![bud(root, 0, 0, Origin::Seed)],
        windows: Windows::new(species, request.age),
        draws: vec![Draws {
            birth: [1.0; 2],
            ..Draws::default()
        }],
        live: vec![Apex { axis: 0, units: 0 }],
        units: vec![0],
        successor: vec![None],
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
        grower.draws[apex.axis].alive = true;
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
    /// Each axis's growth units in its PA when its apex stopped, or at its
    /// birth while it lives: what a relay of it carries on.
    units: Vec<u32>,
    /// Each axis's continuation or relay: what carries it on.
    successor: Vec<Option<usize>>,
    /// The sleeping buds that wake in each cycle, and those that woke.
    asleep: Vec<Vec<Sleeper>>,
    woken: Vec<Woken>,
    age: u32,
    /// One zone's node leads and its nodes (order draw, draw index, lead,
    /// buds), reused.
    leads: Vec<f64>,
    node_buds: Vec<(f64, u64, f64, [Bud; MAX_BUDS as usize])>,
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
    /// then its abortion or its move to its next PA.
    fn advance(&mut self, mut apex: Apex, cycle: u32) -> Result<()> {
        let pa = self.axes[apex.axis].pa;
        let state = &self.species.states[pa];
        let unit = Key(self.axes[apex.axis].lineage).child(u64::from(apex.units) + 1);
        // A woken bud's first unit runs only its share of the year's
        // risks (`dormant.rs`).
        let draws = &self.draws[apex.axis];
        let share = if draws.units.is_empty() && draws.sleep > 0.0 {
            1.0 - draws.sleep
        } else {
            1.0
        };
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
        let u = unit.child(VIABILITY).unit();
        if u >= viability {
            self.axes[apex.axis].apex_end = Some(cycle - 1);
            self.draws[apex.axis].failed = true;
            // The unit that failed is spent: a relay's first unit draws
            // anew.
            self.stop(apex, cycle, above(u, viability), unit, apex.units + 1);
            return Ok(());
        }
        let survive = below(u, viability);
        let wood = self.windows.wood(pa, cycle);
        let survive = self.windows.decided(survive, wood, stop_stake(state.relay));
        // And its subtrees' survival of shedding on their balance.
        let survive = survive * self.kept(apex.axis);
        self.draws[apex.axis].units.push([survive, 1.0]);
        if self.sketch.is_some() {
            let size = self.size(apex.axis);
            self.draws[apex.axis].sizes.push(size);
            if self.species.states.iter().any(|s| s.leaf_girth > 0.0) {
                self.draws[apex.axis].lights.push(light);
            }
        }
        self.grow_unit(apex, pa, unit, cycle)?;
        apex.units += 1;
        // An apex that has spent its PA's lifespan moves on; it does not
        // also abort.
        let draws = &self.draws[apex.axis];
        let abortion = state.abortion_at(draws.units.len() + draws.aged as usize);
        let abortion = if share < 1.0 {
            1.0 - (1.0 - abortion).powf(share)
        } else {
            abortion
        };
        if abortion > 0.0 && apex.units < state.lifespan {
            let u = unit.child(ABORTION).unit();
            if u < abortion {
                self.axes[apex.axis].apex_end = Some(cycle);
                self.stop(apex, cycle, below(u, abortion), unit, apex.units);
                return Ok(());
            }
            let wood = self.windows.wood(pa, cycle + 1);
            let persist = self
                .windows
                .decided(above(u, abortion), wood, stop_stake(state.relay));
            let units = &mut self.draws[apex.axis].units;
            units.last_mut().unwrap()[1] = persist;
        }
        if apex.units < state.lifespan {
            self.next.push(apex);
            return Ok(());
        }
        self.axes[apex.axis].apex_end = Some(cycle);
        match state.next {
            Some(next) => {
                let key = Key(self.axes[apex.axis].lineage).child(CONTINUATION);
                let origin = Origin::Continuation { parent: apex.axis };
                self.successor[apex.axis] = Some(self.axes.len());
                self.sprout(key, next, cycle, origin, [1.0; 2]);
            }
            None => self.stop(apex, cycle, f64::INFINITY, unit, apex.units),
        }
        Ok(())
    }

    /// A new bud that grows from the next cycle, made by draws with these
    /// presences.
    /// A relay carries on its axis's growth units in its PA.
    fn sprout(&mut self, key: Key, pa: usize, cycle: u32, origin: Origin, made: [f64; 2]) {
        let units = match origin {
            Origin::Relay { parent, .. } => self.units[parent],
            _ => 0,
        };
        self.units.push(units);
        self.successor.push(None);
        self.next.push(Apex {
            axis: self.axes.len(),
            units,
        });
        self.axes.push(bud(key, pa, cycle, origin));
        self.draws.push(Draws {
            birth: made,
            ..Draws::default()
        });
    }

    /// The apex has stopped in the growth unit keyed `unit`, `stopped`
    /// log-odds past its last draw, having spent `spent` units; a relay bud
    /// of its PA may take over. The relay is the axis's continuation: it
    /// carries its lineage and the units spent, so its growth units draw
    /// what the apex's would have, and it stands between the axis's tip
    /// and its PA's `relay_at` along it by the stop's presence
    /// (`geometry.rs`).
    fn stop(&mut self, apex: Apex, cycle: u32, stopped: f64, unit: Key, spent: u32) {
        self.units[apex.axis] = spent;
        let axis = &self.axes[apex.axis];
        let relay = self.species.states[axis.pa].relay;
        if relay <= 0.0 {
            return;
        }
        let key = Key(axis.lineage);
        let u = unit.child(RELAY).unit();
        if u < relay {
            let origin = Origin::Relay {
                parent: apex.axis,
                node: 0,
            };
            let pa = axis.pa;
            let wood = self.windows.wood(pa, cycle + 1);
            let made = [
                self.windows.decided(stopped, wood, stop_stake(relay)),
                self.windows.presence(below(u, relay), wood),
            ];
            // The relay moves over the widest window: its move is a growth
            // unit's length whatever wood it carries.
            let blend = (stopped / SPAN).clamp(0.0, 1.0);
            self.successor[apex.axis] = Some(self.axes.len());
            self.sprout(key, pa, cycle, origin, made);
            self.axes.last_mut().unwrap().blend = blend;
        }
    }
}
