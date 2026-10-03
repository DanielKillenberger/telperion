//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops. Every draw is
//! keyed to the lineage it decides (`lineage.rs`), and every element a draw
//! makes carries its presence, so a setting that crosses a draw grows the
//! element in from nothing.
use crate::error::{refuse, Error, Result};
use crate::geometry::place;
use crate::lineage::{
    self, grow_in, Key, ABORTION, CONTINUATION, RELAY, RELAY_BUD, VIABILITY, ZONE,
};
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

/// A bud of a node: its PA and presence, or none for a bare bud.
type Bud = Option<(usize, f64)>;

/// A living apex: the growth units it has grown in its current PA and its
/// presence, the product of every survival it barely passed.
#[derive(Debug, Clone, Copy)]
struct Apex {
    axis: usize,
    units: u32,
    presence: f64,
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
        axes: vec![bud(root, 0, 0, Origin::Seed, 1.0)],
        live: vec![Apex {
            axis: 0,
            units: 0,
            presence: 1.0,
        }],
        next: Vec::new(),
        presence: Vec::new(),
        node_buds: Vec::new(),
        grown: 0,
        budget: request.budget,
    };
    for cycle in 1..=request.age {
        grower.step(cycle)?;
    }
    let mut structure = Structure {
        age: request.age,
        pas: species.states.len(),
        axes: shed(grower.axes, species, request.age),
    };
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    place(&mut structure, species)?;
    Ok(structure)
}

fn bud(key: Key, pa: usize, birth: u32, origin: Origin, vigour: f64) -> Axis {
    Axis {
        lineage: key.0,
        pa,
        birth,
        origin,
        vigour,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: Vec::new(),
        units: Vec::new(),
        rank: 0.0,
    }
}

struct Grower<'a> {
    species: &'a Species,
    /// Each PA's zones' lateral probabilities as drawn (`PaState::laterals`).
    laterals: Vec<Vec<Vec<f64>>>,
    axes: Vec<Axis>,
    live: Vec<Apex>,
    next: Vec<Apex>,
    /// One zone's node presences and its nodes (order draw, draw index,
    /// presence, buds), reused.
    presence: Vec<f64>,
    node_buds: Vec<(f64, u64, f64, [Bud; MAX_BUDS as usize])>,
    grown: u32,
    budget: u32,
}

impl Grower<'_> {
    fn step(&mut self, cycle: u32) -> Result<()> {
        let live = std::mem::take(&mut self.live);
        for mut apex in live.iter().copied() {
            let pa = self.axes[apex.axis].pa;
            let state = &self.species.states[pa];
            let unit = Key(self.axes[apex.axis].lineage).child(u64::from(apex.units) + 1);
            let u = unit.child(VIABILITY).unit();
            if u >= state.viability {
                self.axes[apex.axis].apex_end = Some(cycle - 1);
                self.stop(
                    apex,
                    cycle,
                    grow_in(u - state.viability, u, 1.0 - state.viability),
                );
                continue;
            }
            apex.presence *= grow_in(state.viability - u, 1.0 - u, state.viability);
            self.grow_unit(apex, pa, unit, cycle)?;
            self.axes[apex.axis].units.push(apex.presence);
            apex.units += 1;
            if state.abortion > 0.0 {
                let u = unit.child(ABORTION).unit();
                if u < state.abortion {
                    self.axes[apex.axis].apex_end = Some(cycle);
                    self.stop(
                        apex,
                        cycle,
                        grow_in(state.abortion - u, 1.0 - u, state.abortion),
                    );
                    continue;
                }
                apex.presence *= grow_in(u - state.abortion, u, 1.0 - state.abortion);
            }
            if apex.units < state.lifespan {
                self.next.push(apex);
                continue;
            }
            self.axes[apex.axis].apex_end = Some(cycle);
            match state.next {
                Some(next) => {
                    let key = Key(self.axes[apex.axis].lineage).child(CONTINUATION);
                    let origin = Origin::Continuation { parent: apex.axis };
                    self.sprout(key, next, cycle, origin, apex.presence);
                }
                None => self.stop(apex, cycle, 1.0),
            }
        }
        self.live = std::mem::replace(&mut self.next, live);
        self.next.clear();
        Ok(())
    }

    /// A new bud that grows from the next cycle.
    fn sprout(&mut self, key: Key, pa: usize, cycle: u32, origin: Origin, vigour: f64) {
        self.next.push(Apex {
            axis: self.axes.len(),
            units: 0,
            presence: 1.0,
        });
        self.axes.push(bud(key, pa, cycle, origin, vigour));
    }

    /// The apex has stopped, `stopped` past its last draw; a relay bud of
    /// its PA may take over at its last node, or at its base if it grew none.
    fn stop(&mut self, apex: Apex, cycle: u32, stopped: f64) {
        let axis = &self.axes[apex.axis];
        let relay = self.species.states[axis.pa].relay;
        if relay <= 0.0 {
            return;
        }
        let key = Key(axis.lineage);
        let u = key.child(RELAY).unit();
        if u < relay {
            let vigour = apex.presence * stopped * grow_in(relay - u, 1.0 - u, relay);
            let origin = Origin::Relay { parent: apex.axis };
            self.sprout(key.child(RELAY_BUD), axis.pa, cycle, origin, vigour);
        }
    }

    fn grow_unit(&mut self, apex: Apex, pa: usize, unit: Key, cycle: u32) -> Result<()> {
        let state = &self.species.states[pa];
        let mut nodes = std::mem::take(&mut self.node_buds);
        let mut presence = std::mem::take(&mut self.presence);
        for (z, zone) in state.zones.iter().enumerate() {
            let zone_key = unit.child(ZONE + z as u64);
            lineage::nodes(
                zone.nodes,
                zone_key.unit(),
                MAX_NODES_PER_ZONE,
                &mut presence,
            );
            let lateral = &self.laterals[pa][z];
            nodes.clear();
            for (i, &node_presence) in presence.iter().enumerate() {
                let node_key = zone_key.child(i as u64);
                let mut buds = [None; MAX_BUDS as usize];
                let mut order = 1.0f64;
                for (slot, bud) in buds[..zone.buds as usize].iter_mut().enumerate() {
                    let u = node_key.child(slot as u64).unit();
                    order = order.min(u);
                    *bud = lineage::bud(lateral, u);
                }
                nodes.push((order, i as u64, node_presence, buds));
            }
            // Acrotony: the nodes stand in order of their draws, so the
            // youngest lateral PA is on top, bare nodes at the base, and a
            // bud a setting makes or unmakes moves no node.
            nodes.sort_by(|a, b| b.0.total_cmp(&a.0));
            for &(_, drawn, node_presence, buds) in &nodes {
                self.grown += 1;
                if self.grown > self.budget {
                    return Err(Error::Budget { limit: self.budget });
                }
                let axis = &mut self.axes[apex.axis];
                let node = axis.phytomers.len();
                axis.phytomers.push(Phytomer {
                    cycle,
                    tip: Vec3::default(),
                    scale: apex.presence * node_presence,
                    rank: axis.rank,
                });
                axis.rank += node_presence;
                for (slot, bud) in buds[..zone.buds as usize].iter().enumerate() {
                    let Some((lateral_pa, vigour)) = *bud else {
                        continue;
                    };
                    let origin = Origin::Lateral {
                        parent: apex.axis,
                        node,
                        slot: slot as u8,
                        whorl: zone.buds,
                    };
                    let key = zone_key.child(drawn).child(slot as u64);
                    self.sprout(key, lateral_pa, cycle, origin, vigour);
                }
            }
        }
        self.node_buds = nodes;
        self.presence = presence;
        Ok(())
    }
}
