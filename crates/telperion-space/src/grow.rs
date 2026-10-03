//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops. Every draw is
//! keyed to the lineage it decides (`lineage.rs`), and its lead past its
//! bound is recorded; once the tree has grown, `presence.rs` sizes every
//! element from those leads, so a setting that crosses a draw grows the
//! element in from nothing.
use crate::error::{refuse, Error, Result};
use crate::geometry::place;
use crate::girth::thicken;
use crate::lineage::{
    self, above, below, Key, ABORTION, CONTINUATION, RELAY, RELAY_BUD, VIABILITY, ZONE,
};
use crate::presence::{assign, Draws, Windows};
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
        next: Vec::new(),
        leads: Vec::new(),
        node_buds: Vec::new(),
        grown: 0,
        budget: request.budget,
    };
    for cycle in 1..=request.age {
        grower.step(cycle)?;
    }
    for apex in &grower.live {
        grower.draws[apex.axis].alive = true;
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
    place(&mut structure, species)?;
    thicken(&mut structure, species);
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

fn bud(key: Key, pa: usize, birth: u32, origin: Origin) -> Axis {
    Axis {
        lineage: key.0,
        pa,
        birth,
        origin,
        vigour: 1.0,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: Vec::new(),
        units: Vec::new(),
        alive: 0.0,
        rank: 0.0,
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
    /// One zone's node leads and its nodes (order draw, draw index, lead,
    /// buds), reused.
    leads: Vec<f64>,
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
                self.stop(apex, cycle, above(u, state.viability));
                continue;
            }
            let survive = below(u, state.viability);
            let survive = self.windows.presence(survive, self.windows.wood(pa, cycle));
            self.draws[apex.axis].units.push([survive, 1.0]);
            self.grow_unit(apex, pa, unit, cycle)?;
            apex.units += 1;
            if state.abortion > 0.0 {
                let u = unit.child(ABORTION).unit();
                if u < state.abortion {
                    self.axes[apex.axis].apex_end = Some(cycle);
                    self.stop(apex, cycle, below(u, state.abortion));
                    continue;
                }
                let wood = self.windows.wood(pa, cycle + 1);
                let persist = self.windows.presence(above(u, state.abortion), wood);
                let units = &mut self.draws[apex.axis].units;
                units.last_mut().unwrap()[1] = persist;
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
                    self.sprout(key, next, cycle, origin, [1.0; 2]);
                }
                None => self.stop(apex, cycle, f64::INFINITY),
            }
        }
        self.live = std::mem::replace(&mut self.next, live);
        self.next.clear();
        Ok(())
    }

    /// A new bud that grows from the next cycle, made by draws with these
    /// presences.
    fn sprout(&mut self, key: Key, pa: usize, cycle: u32, origin: Origin, made: [f64; 2]) {
        self.next.push(Apex {
            axis: self.axes.len(),
            units: 0,
        });
        self.axes.push(bud(key, pa, cycle, origin));
        self.draws.push(Draws {
            birth: made,
            ..Draws::default()
        });
    }

    /// The apex has stopped, `stopped` log-odds past its last draw; a relay
    /// bud of its PA may take over at its last node, or at its base if it
    /// grew none.
    fn stop(&mut self, apex: Apex, cycle: u32, stopped: f64) {
        let axis = &self.axes[apex.axis];
        let relay = self.species.states[axis.pa].relay;
        if relay <= 0.0 {
            return;
        }
        let key = Key(axis.lineage);
        let u = key.child(RELAY).unit();
        if u < relay {
            let origin = Origin::Relay { parent: apex.axis };
            let pa = axis.pa;
            let wood = self.windows.wood(pa, cycle + 1);
            let made = [
                self.windows.presence(stopped, wood),
                self.windows.presence(below(u, relay), wood),
            ];
            self.sprout(key.child(RELAY_BUD), pa, cycle, origin, made);
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
                    };
                    let key = zone_key.child(drawn).child(slot as u64);
                    let wood = self.windows.wood(lateral_pa, cycle + 1);
                    let made = [self.windows.presence(lead, wood), 1.0];
                    self.sprout(key, lateral_pa, cycle, origin, made);
                }
            }
        }
        self.node_buds = nodes;
        self.leads = leads;
        Ok(())
    }
}
