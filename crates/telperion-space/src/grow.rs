//! The engine: a tree grows one growth unit per living apex per cycle, from
//! one seed bud of the youngest PA. A lateral bud made in a cycle grows its
//! first growth unit in the next; an apex that has spent its PA's lifespan
//! moves to its next PA, as a new development axis, or stops.
use crate::error::{refuse, Error, Result};
use crate::geometry::place;
use crate::rng::Rng;
use crate::species::{PaState, Species, MAX_BUDS};
use crate::structure::{Axis, Origin, Phytomer, Structure, Vec3};

/// What to grow: cycles, the random stream and the phytomer budget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
    pub age: u32,
    pub seed: u64,
    /// The most phytomers the tree may grow, the shed ones included.
    pub budget: u32,
}

const BARE: u8 = u8::MAX;
type Buds = [u8; MAX_BUDS as usize];

/// A living apex and the growth units it has grown in its current PA.
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
    let mut grower = Grower {
        species,
        rng: Rng::new(request.seed),
        axes: vec![bud(0, 0, Origin::Seed)],
        live: vec![Apex { axis: 0, units: 0 }],
        next: Vec::new(),
        nodes: Vec::new(),
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

fn bud(pa: usize, birth: u32, origin: Origin) -> Axis {
    Axis {
        pa,
        birth,
        origin,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: Vec::new(),
    }
}

struct Grower<'a> {
    species: &'a Species,
    rng: Rng,
    axes: Vec<Axis>,
    live: Vec<Apex>,
    next: Vec<Apex>,
    /// One zone's drawn nodes, reused.
    nodes: Vec<(u8, Buds)>,
    grown: u32,
    budget: u32,
}

impl Grower<'_> {
    fn step(&mut self, cycle: u32) -> Result<()> {
        let live = std::mem::take(&mut self.live);
        for mut apex in live.iter().copied() {
            let pa = self.axes[apex.axis].pa;
            let state = &self.species.states[pa];
            if !self.rng.chance(state.viability) {
                self.axes[apex.axis].apex_end = Some(cycle - 1);
                continue;
            }
            self.grow_unit(apex.axis, state, cycle)?;
            apex.units += 1;
            if apex.units < state.lifespan {
                self.next.push(apex);
                continue;
            }
            self.axes[apex.axis].apex_end = Some(cycle);
            if let Some(next) = state.next {
                let origin = Origin::Continuation { parent: apex.axis };
                self.next.push(Apex {
                    axis: self.axes.len(),
                    units: 0,
                });
                self.axes.push(bud(next, cycle, origin));
            }
        }
        self.live = std::mem::replace(&mut self.next, live);
        self.next.clear();
        Ok(())
    }

    fn grow_unit(&mut self, axis: usize, state: &PaState, cycle: u32) -> Result<()> {
        let mut nodes = std::mem::take(&mut self.nodes);
        for zone in &state.zones {
            nodes.clear();
            for _ in 0..self.rng.nodes(zone.nodes) {
                let mut buds = [BARE; MAX_BUDS as usize];
                for slot in &mut buds[..zone.buds as usize] {
                    *slot = self.rng.bud(&zone.lateral).map_or(BARE, |pa| pa as u8);
                }
                let youngest = buds.iter().copied().min().unwrap_or(BARE);
                nodes.push((youngest, buds));
            }
            // Acrotony: bare nodes at the base, the youngest lateral PA on top.
            nodes.sort_by_key(|node| std::cmp::Reverse(node.0));
            for &(_, buds) in &nodes {
                self.grown += 1;
                if self.grown > self.budget {
                    return Err(Error::Budget { limit: self.budget });
                }
                let node = self.axes[axis].phytomers.len();
                self.axes[axis].phytomers.push(Phytomer {
                    cycle,
                    tip: Vec3::default(),
                });
                for (slot, &pa) in buds[..zone.buds as usize].iter().enumerate() {
                    if pa == BARE {
                        continue;
                    }
                    let whorl = zone.buds;
                    let origin = Origin::Lateral {
                        parent: axis,
                        node,
                        slot: slot as u8,
                        whorl,
                    };
                    self.next.push(Apex {
                        axis: self.axes.len(),
                        units: 0,
                    });
                    self.axes.push(bud(pa as usize, cycle, origin));
                }
            }
        }
        self.nodes = nodes;
        Ok(())
    }
}

/// Drops every lateral axis, with all it bears, that has held no living apex
/// for more than its PA's shedding delay.
fn shed(axes: Vec<Axis>, species: &Species, age: u32) -> Vec<Axis> {
    // Parents precede children, so one backward pass carries each subtree's
    // last living cycle up to its root.
    let mut live_until: Vec<u32> = axes
        .iter()
        .map(|a| a.apex_end.unwrap_or(u32::MAX))
        .collect();
    for i in (1..axes.len()).rev() {
        let parent = parent(&axes[i]);
        live_until[parent] = live_until[parent].max(live_until[i]);
    }
    let mut index = vec![usize::MAX; axes.len()];
    let mut kept = Vec::with_capacity(axes.len());
    for (i, mut axis) in axes.into_iter().enumerate() {
        let lost = match axis.origin {
            Origin::Seed => false,
            Origin::Continuation { parent } => index[parent] == usize::MAX,
            Origin::Lateral { parent, .. } => {
                let delay = species.states[axis.pa].shedding;
                let idle = age.saturating_sub(live_until[i]);
                index[parent] == usize::MAX || delay.is_some_and(|d| idle > d)
            }
        };
        if lost {
            continue;
        }
        match &mut axis.origin {
            Origin::Seed => {}
            Origin::Continuation { parent } | Origin::Lateral { parent, .. } => {
                *parent = index[*parent]
            }
        }
        index[i] = kept.len();
        kept.push(axis);
    }
    kept
}

fn parent(axis: &Axis) -> usize {
    match axis.origin {
        Origin::Seed => 0,
        Origin::Continuation { parent } | Origin::Lateral { parent, .. } => parent,
    }
}
