//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops. Every draw is
//! keyed to the lineage it decides (`lineage.rs`), and its lead past its
//! bound is recorded; once the tree has grown, `presence.rs` sizes every
//! element from those leads, so a setting that crosses a draw grows the
//! element in from nothing.
mod wake;

use crate::dormant::{Sleeper, Woken};
use crate::error::{refuse, Error, Result};
use crate::geometry::{place, scale};
use crate::girth::thicken;
use crate::lineage::{self, above, below, Key, ABORTION, CONTINUATION, RELAY, VIABILITY, ZONE};
use crate::presence::{assign, Draws, Windows, SPAN};
use crate::sag;
use crate::shed::shed;
use crate::species::{PaState, Species, MAX_BUDS, MAX_NODES_PER_ZONE};
use crate::structure::{Axis, Origin, Phytomer, Structure, Vec3};

/// What to grow: cycles, the seed and the phytomer budget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
    pub age: u32,
    pub seed: u64,
    /// The most phytomers the tree may grow, the shed ones included.
    pub budget: u32,
}

/// A bud of a node: its PA and lead, or none for a bare bud.
type Bud = Option<(usize, f64)>;

/// A living apex: the growth units it has grown in its current PA.
#[derive(Debug, Clone, Copy)]
struct Apex {
    axis: usize,
    units: u32,
}

/// Grows, sheds and places the tree.
pub fn grow(species: &Species, request: Request) -> Result<Structure> {
    species.validate()?;
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
    };
    for cycle in 1..=request.age {
        grower.step(cycle)?;
    }
    for apex in &grower.live {
        grower.draws[apex.axis].alive = true;
    }
    for woken in &grower.woken {
        let kept = woken.kept(&grower.axes, &grower.draws, &grower.successor, request.age);
        grower.draws[woken.axis].birth[1] *= kept;
    }
    assign(&mut grower.axes, &grower.draws);
    let mut structure = Structure {
        age: request.age,
        pas: species.states.len(),
        axes: shed(grower.axes, species, request.age),
    };
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    scale(&mut structure, species);
    // Girth needs no geometry; placing reads it where wood meets the
    // ground.
    thicken(&mut structure, species);
    place(&mut structure, species, None)?;
    // Sag bends the tree as it stands under the load it carries, and
    // leaves its girth as it was.
    if sag::any(species) {
        let torques = sag::torques(&structure, species);
        place(&mut structure, species, Some(&torques))?;
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
}

impl Grower<'_> {
    fn step(&mut self, cycle: u32) -> Result<()> {
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
        let viability = shared(state.viability, share);
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
        self.draws[apex.axis].units.push([survive, 1.0]);
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

    fn grow_unit(&mut self, apex: Apex, pa: usize, unit: Key, cycle: u32) -> Result<()> {
        let state = &self.species.states[pa];
        let mut nodes = std::mem::take(&mut self.node_buds);
        let mut leads = std::mem::take(&mut self.leads);
        for (z, zone) in state.zones.iter().enumerate() {
            let zone_key = unit.child(ZONE + z as u64);
            lineage::nodes(zone.nodes, zone_key.unit(), MAX_NODES_PER_ZONE, &mut leads);
            let lateral = &self.laterals[pa][z];
            // A node decides its internode and the buds it is expected to bear;
            // a PA its buds cannot carry takes no part.
            let expected: f64 = lateral
                .iter()
                .enumerate()
                .filter(|&(_, &p)| p > 0.0)
                .map(|(j, p)| p * self.windows.wood(j, cycle + 1))
                .sum();
            // And the sleeping buds it is expected to bear.
            let mut sleeping = std::mem::take(&mut self.sleeping);
            let expected = expected + self.windows.sleeping(zone, cycle, &mut sleeping);
            let node_wood = self.windows.share(state.internode) + f64::from(zone.buds) * expected;
            nodes.clear();
            for (i, &node_lead) in leads.iter().enumerate() {
                let node_key = zone_key.child(i as u64);
                let mut buds = [None; MAX_BUDS as usize];
                let mut order = 1.0f64;
                for (slot, bud) in buds[..zone.buds as usize].iter_mut().enumerate() {
                    let u = node_key.child(slot as u64).unit();
                    order = order.min(u);
                    *bud = lineage::bud(lateral, u);
                }
                nodes.push((order, i as u64, node_lead, buds));
            }
            // Acrotony: the nodes stand in order of their draws, so the
            // youngest lateral PA is on top, bare nodes at the base, and a
            // bud a setting makes or unmakes moves no node.
            nodes.sort_by(|a, b| b.0.total_cmp(&a.0));
            for &(_, drawn, node_lead, buds) in &nodes {
                self.grown += 1;
                if self.grown > self.budget {
                    return Err(Error::Budget { limit: self.budget });
                }
                let axis = &mut self.axes[apex.axis];
                let node = axis.phytomers.len();
                axis.phytomers.push(Phytomer {
                    cycle,
                    tip: Vec3::default(),
                    heading: Vec3::default(),
                    side: Vec3::default(),
                    radius: 0.0,
                    scale: 1.0,
                    key: zone_key.child(drawn).0,
                    rank: 0.0,
                });
                let node_presence = self.windows.presence(node_lead, node_wood);
                self.draws[apex.axis].nodes.push(node_presence);
                for (slot, bud) in buds[..zone.buds as usize].iter().enumerate() {
                    let Some((lateral_pa, lead)) = *bud else {
                        continue;
                    };
                    let origin = Origin::Lateral {
                        parent: apex.axis,
                        node,
                        slot: slot as u8,
                        whorl: zone.buds,
                        woken: false,
                    };
                    let key = zone_key.child(drawn).child(slot as u64);
                    let wood = self.windows.wood(lateral_pa, cycle + 1);
                    let made = [self.windows.presence(lead, wood), 1.0];
                    self.sprout(key, lateral_pa, cycle, origin, made);
                }
                if sleeping.iter().any(|&wood| wood > 0.0) {
                    let at = (apex.axis, node, self.draws[apex.axis].units.len() - 1);
                    let node_key = zone_key.child(drawn);
                    self.sleep(zone, &sleeping, node_key, at, cycle);
                }
            }
            self.sleeping = sleeping;
        }
        self.node_buds = nodes;
        self.leads = leads;
        Ok(())
    }
}
