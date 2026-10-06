//! A fine root on the CPU reference executor under realized mechanics
//! (fn-209 F1 task 6): grown whole by the engine's own grower from its
//! bud alone, then settled through the four phases its bearer above the
//! cut settles through, each phase reading only what the bearer hands
//! down and reporting its totals up (one record per root, never per
//! node):
//!
//! 1. grown, marked, assigned and shed by its realized subtree; up, its
//!    subtree's living (`shed.rs`, `Living`), which its bearer's fade
//!    reads;
//! 2. down, its bearer's scale at its base and its node's drawn scale;
//!    scaled and thickened; up, its base radius;
//! 3. down, its bearer's laid frame at its node; laid, its levers
//!    gathered; up, its load at its base (mass and first moment);
//! 4. down, its bearer's frame as laid under sag; laid again under its
//!    levers, from its own first lay's frames.
//!
//! The root's tree holds its bearer as a stub (axis 0), whose one
//! phytomer stands for the node the root grows on, so every pass is the
//! whole tree's own, run on the root's axes.
use super::Work;
use crate::error::Result;
use crate::geometry::{place_from, scale};
use crate::girth::{thicken_given, Girth};
use crate::grow::realized::{self, Handed};
use crate::grow::Rules;
use crate::sag::{any, levers_given, Lever, Load};
use crate::shed::{shed_given, Living};
use crate::species::Species;
use crate::structure::{Axis, Origin, Phytomer, Structure, Vec3};
use std::time::Instant;

/// One fine root carried through the phases.
pub(crate) struct Root {
    /// Its stand-in's index above the cut, as grown.
    pub at: usize,
    /// The node of its bearer it stands on.
    pub node: usize,
    pub woken: bool,
    /// Its first axis's PA and birth.
    pub pa: usize,
    pub birth: u32,
    tree: Structure,
    levers: Vec<Vec<Lever>>,
    /// What it reported up: its living, base radius and load.
    pub living: Living,
    pub radius: f64,
    pub load: Load,
    pub work: Work,
}

impl Root {
    /// Phase 1: grows `handed`'s subtree whole, marks, assigns and sheds
    /// it by what it realized.
    pub fn grow(species: &Species, rules: &Rules, handed: &Handed) -> Result<Self> {
        let started = Instant::now();
        let assigned = realized::root(rules, &handed.bud)?;
        Ok(Self::shed(species, rules, handed, assigned, started))
    }

    /// Phase 1's end: `assigned`, `handed`'s subtree as grown, marked and
    /// assigned, shed by what it realized.
    pub fn shed(
        species: &Species,
        rules: &Rules,
        handed: &Handed,
        assigned: realized::Assigned,
        started: Instant,
    ) -> Self {
        let Origin::Lateral { node, woken, .. } = handed.bud.axis.origin else {
            unreachable!("a fine root is a lateral bud")
        };
        let mut axes = assigned.axes;
        let made = axes.len() - 1;
        axes[0].phytomers = vec![node_phytomer()];
        if let Origin::Lateral { node, .. } = &mut axes[1].origin {
            *node = 0;
        }
        let age = rules.age;
        let (shed, living) = shed_given(axes, species, age, &|_| None, Some(1));
        let tree = Structure {
            age,
            pas: species.states.len(),
            axes: shed.axes,
        };
        let work = Work {
            chains: 1,
            grown: assigned.grown,
            cycles: u64::from(age - handed.bud.axis.birth),
            axes: made,
            micros: started.elapsed().as_secs_f64() * 1e6,
        };
        Self {
            at: handed.at,
            node,
            woken,
            pa: handed.bud.axis.pa,
            birth: handed.bud.axis.birth,
            tree,
            levers: Vec::new(),
            living,
            radius: 0.0,
            load: Load::default(),
            work,
        }
    }

    /// Its tree: its bearer's stub (axis 0), then its axes.
    pub fn axes(&self) -> &[Axis] {
        &self.tree.axes
    }

    /// Its kept phytomers, its bearer's stub aside.
    pub fn phytomers(&self) -> usize {
        self.tree
            .phytomer_count()
            .saturating_sub(self.tree.axes[0].phytomers.len())
    }

    /// Phase 2: scaled from its bearer's scale at its base and its node's
    /// drawn scale, and thickened.
    pub fn girth(&mut self, species: &Species, (base, drawn): (f64, f64)) {
        let started = Instant::now();
        let stub = &mut self.tree.axes[0];
        stub.vigour = base;
        stub.phytomers[0].scale = drawn;
        scale(&mut self.tree, species, 1);
        let radii = thicken_given(&mut self.tree, species, &Girth::default(), &|_| None, 1);
        self.radius = radii.get(1).copied().unwrap_or(0.0);
        self.work.micros += started.elapsed().as_secs_f64() * 1e6;
    }

    /// Phase 3: laid from its bearer's laid frame at its node, and its
    /// levers gathered where any PA sags.
    pub fn lay(&mut self, species: &Species, bearer: &Axis) -> Result<()> {
        let started = Instant::now();
        self.tree.axes[0] = stub(bearer, self.node);
        place_from(&mut self.tree, species, None, 1, 1)?;
        if any(species) {
            let (levers, loads) = levers_given(&self.tree, species, &|_| None, 1);
            self.levers = levers;
            self.load = loads.get(1).copied().unwrap_or_default();
        }
        self.work.micros += started.elapsed().as_secs_f64() * 1e6;
        Ok(())
    }

    /// Phase 4: laid again under its levers from its bearer's frame as
    /// laid under sag, each phytomer's lever turned from its first lay's
    /// frame.
    pub fn bend(&mut self, species: &Species, bearer: &Axis) -> Result<()> {
        let started = Instant::now();
        self.tree.axes[0] = stub(bearer, self.node);
        place_from(&mut self.tree, species, Some(&self.levers), 1, 1)?;
        self.work.micros += started.elapsed().as_secs_f64() * 1e6;
        Ok(())
    }

    /// Its axes, the stub dropped: its first on `bearer` at its node as
    /// axis `at`, the rest numbered from `offset`.
    pub fn into_axes(self, bearer: usize, (at, offset): (usize, usize)) -> Vec<Axis> {
        let node = self.node;
        let place = |p: usize| if p == 1 { at } else { offset + p - 2 };
        self.tree
            .axes
            .into_iter()
            .skip(1)
            .map(|mut axis| {
                match &mut axis.origin {
                    Origin::Seed => unreachable!("only the stub is the seed"),
                    Origin::Lateral {
                        parent, node: n, ..
                    } if *parent == 0 => {
                        (*parent, *n) = (bearer, node);
                    }
                    Origin::Lateral { parent, .. }
                    | Origin::Continuation { parent }
                    | Origin::Relay { parent, .. } => *parent = place(*parent),
                }
                axis
            })
            .collect()
    }
}

/// A node's phytomer before it is laid: its drawn scale is set in phase 2.
fn node_phytomer() -> Phytomer {
    Phytomer {
        cycle: 0,
        tip: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        radius: 0.0,
        scale: 0.0,
        key: 0,
        rank: 0.0,
        size: 1.0,
        light: 1.0,
    }
}

/// The bearer as a root's tree holds it: its phytomer at `node` alone,
/// standing on the point below it, as laid.
fn stub(bearer: &Axis, node: usize) -> Axis {
    let below = node
        .checked_sub(1)
        .map_or(bearer.base, |n| bearer.phytomers[n].tip);
    Axis {
        lineage: bearer.lineage,
        pa: bearer.pa,
        birth: bearer.birth,
        origin: Origin::Seed,
        vigour: 1.0,
        blend: 1.0,
        apex_end: None,
        base: below,
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: vec![bearer.phytomers[node]],
        units: Vec::new(),
        alive: 0.0,
        rank: 0.0,
        sleep: 0.0,
    }
}
