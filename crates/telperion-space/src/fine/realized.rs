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
use super::expansion::{Lived, Loaded, RootSeed, Roots};
use super::{each, pick, Plan, Work};
use crate::error::Result;
use crate::fixed::feedback::total::Total;
use crate::geometry::{place_from, scale};
use crate::girth::{attachments_given, disused_below, leaf_partials, thicken_given, Girth, Stand};
use crate::grow::realized::{self, Handed};
use crate::grow::Rules;
use crate::sag::{any, levers_given, Lever, Load};
use crate::shed::{shed_given, Living, Shed};
use crate::species::Species;
use crate::structure::{Axis, Origin, Phytomer, Structure, Vec3};
use std::time::Instant;

/// One fine root carried through the phases.
pub(crate) struct Root {
    /// The node of its bearer it stands on.
    pub node: usize,
    pub woken: bool,
    /// Its first axis's PA and birth.
    pub pa: usize,
    pub birth: u32,
    tree: Structure,
    levers: Vec<Vec<Lever>>,
    /// Where it keeps its tree as grown (any PA's `retained` or
    /// `leaf_girth`): its axes before shedding, its sheds' index and
    /// fades, its bearer's base and node's drawn scale as grown, and the
    /// shed pipes and leaf mean it thickens by (task 12's exchange).
    grown: Option<Box<AsGrown>>,
    girth: Girth,
    /// What it reported up: its living, base radius and load.
    pub living: Living,
    pub radius: f64,
    pub load: Load,
    pub work: Work,
}

/// A fine root's first axis as its bearer made it: the node it stands
/// on, whether it woke, its PA and birth.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Made {
    pub node: usize,
    pub woken: bool,
    pub pa: usize,
    pub birth: u32,
}

impl Made {
    pub fn of(axis: &Axis) -> Self {
        let Origin::Lateral { node, woken, .. } = axis.origin else {
            unreachable!("a fine root is a lateral bud")
        };
        Self {
            node,
            woken,
            pa: axis.pa,
            birth: axis.birth,
        }
    }
}

/// A root's tree as grown, kept for the exchange.
struct AsGrown {
    axes: Vec<Axis>,
    index: Vec<usize>,
    fade: Vec<f64>,
    stand: (i64, f64),
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
        let made = Made::of(&handed.bud.axis);
        Self::shed_grown(species, rules.age, made, assigned, started)
    }

    /// Phase 1's end for a subtree grown by any grower: its bearer's stub
    /// (axis 0), then its axes, marked and assigned; shed by what it
    /// realized (the stepped executor's roots, fn-211 task 12).
    pub fn shed_grown(
        species: &Species,
        age: u32,
        made: Made,
        assigned: realized::Assigned,
        started: Instant,
    ) -> Self {
        let Made {
            node,
            woken,
            pa,
            birth,
        } = made;
        let mut axes = assigned.axes;
        let made = axes.len() - 1;
        axes[0].phytomers = vec![node_phytomer()];
        if let Origin::Lateral { node, .. } = &mut axes[1].origin {
            *node = 0;
        }
        let kept = crate::grow::keeps(species).then(|| axes.clone());
        let (shed, living) = shed_given(axes, species, age, &|_| None, Some(1));
        let grown = kept.map(|axes| {
            Box::new(AsGrown {
                axes,
                index: shed.index.clone(),
                fade: shed.fade.clone(),
                stand: (0, 0.0),
            })
        });
        let tree = Structure {
            age,
            pas: species.states.len(),
            axes: shed.axes,
        };
        let work = Work {
            chains: 1,
            grown: assigned.grown,
            cycles: u64::from(age - birth),
            axes: made,
            micros: started.elapsed().as_secs_f64() * 1e6,
        };
        Self {
            node,
            woken,
            pa,
            birth,
            tree,
            levers: Vec::new(),
            grown,
            girth: Girth::default(),
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
        let radii = thicken_given(&mut self.tree, species, &self.girth, &|_| None, 1);
        self.radius = radii.get(1).copied().unwrap_or(0.0);
        self.work.micros += started.elapsed().as_secs_f64() * 1e6;
    }

    /// The exchange's first half (fn-211 task 12; PHASES.md, the settle):
    /// down, its bearer's base and its node's drawn scale as grown (Q32,
    /// real); up, its subtree's leaf partials as grown.
    pub fn partials(&mut self, species: &Species, stand: (i64, f64)) -> [Total; 2] {
        let age = self.tree.age;
        let g = self.grown.as_mut().expect("a tree kept as grown");
        g.stand = stand;
        g.axes[0].phytomers[0].scale = stand.1;
        let from = Stand {
            top: Some(stand.0),
            first: 1,
        };
        match species.states.iter().any(|s| s.leaf_girth > 0.0) {
            true => leaf_partials(&g.axes, species, (age, 1), from),
            false => [Total::default(); 2],
        }
    }

    /// The exchange's second half: down, the tree's leaf mean and its
    /// bearers' fades at its base (Q32); its own shed pipes kept; up, its
    /// base radius as grown.
    pub fn attach(&mut self, species: &Species, mean: Option<f64>, above: i64) -> f64 {
        let age = self.tree.age;
        let g = self.grown.as_ref().expect("a tree kept as grown");
        let from = Stand {
            top: Some(g.stand.0),
            first: 1,
        };
        let links = attachments_given(&g.axes, species, (age, 1), (from, mean), &|_| None);
        let shed = Shed {
            axes: Vec::new(),
            index: g.index.clone(),
            fade: g.fade.clone(),
        };
        self.girth = disused_below(&links, &shed, species, Some(above));
        self.grown = None;
        links.radius(1)
    }

    /// Phase 3: laid from its bearer's laid frame at its node, and its
    /// levers gathered where any PA sags.
    pub fn lay(&mut self, species: &Species, bearer: &Axis) -> Result<()> {
        self.lay_on(species, stub(bearer, self.node))
    }

    /// Phase 3 from its bearer's stub as `stub` makes it.
    pub fn lay_on(&mut self, species: &Species, bearer: Axis) -> Result<()> {
        let started = Instant::now();
        self.tree.axes[0] = bearer;
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
        self.bend_on(species, stub(bearer, self.node))
    }

    /// Phase 4 from its bearer's stub as `stub` makes it.
    pub fn bend_on(&mut self, species: &Species, bearer: Axis) -> Result<()> {
        let started = Instant::now();
        self.tree.axes[0] = bearer;
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
pub(crate) fn stub(bearer: &Axis, node: usize) -> Axis {
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

/// The CPU reference executor's roots, grown and settled `plan.chunk` at
/// a time in the plan's order on `threads` cores: grown from their seeds
/// by `rules`, or none where they are held grown already (the stepped
/// executor's, fn-211 task 12).
pub(crate) struct Cpu<'s, 'r> {
    pub species: &'s Species,
    pub rules: Option<&'r Rules<'s>>,
    pub plan: Plan,
    pub threads: usize,
    pub roots: Vec<Root>,
}

impl Roots for Cpu<'_, '_> {
    fn grow(&mut self, seeds: &[RootSeed]) -> Result<(Vec<Lived>, u64)> {
        let species = self.species;
        if let Some(rules) = self.rules {
            let mut slots: Vec<&RootSeed> = seeds.iter().collect();
            let grown = each(&mut slots, self.plan, self.threads, &|_, s| {
                Root::grow(species, rules, &s.handed)
            });
            self.roots = grown.into_iter().collect::<Result<Vec<Root>>>()?;
        }
        let living = self.roots.iter().map(|r| (r.living.lived, r.living.alive));
        let phytomers = self.roots.iter().map(|r| r.work.grown).sum();
        Ok((living.collect(), phytomers))
    }

    fn girth(&mut self, live: &[usize], scales: &[(f64, f64)]) -> Vec<f64> {
        let species = self.species;
        let mut alive = pick(&mut self.roots, live);
        each(&mut alive, self.plan, self.threads, &|j, r| {
            r.girth(species, scales[j])
        });
        live.iter().map(|&k| self.roots[k].radius).collect()
    }

    fn lay(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<Vec<Loaded>> {
        let species = self.species;
        let mut alive = pick(&mut self.roots, live);
        let laid = each(&mut alive, self.plan, self.threads, &|j, r| {
            r.lay(species, &tree.axes[bearers[j]])
        });
        laid.into_iter().collect::<Result<()>>()?;
        let load = |k: usize| self.roots[k].load.real();
        Ok(live.iter().map(|&k| load(k)).collect())
    }

    fn grown_partials(&mut self, stands: &[(i64, f64)]) -> Result<Vec<[Total; 2]>> {
        let species = self.species;
        Ok(each(&mut self.roots, self.plan, self.threads, &|k, r| {
            r.partials(species, stands[k])
        }))
    }

    fn grown_radii(&mut self, mean: Option<f64>, above: &[i64]) -> Result<Vec<f64>> {
        let species = self.species;
        Ok(each(&mut self.roots, self.plan, self.threads, &|k, r| {
            r.attach(species, mean, above[k])
        }))
    }

    fn bend(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<()> {
        let species = self.species;
        let mut alive = pick(&mut self.roots, live);
        let bent = each(&mut alive, self.plan, self.threads, &|j, r| {
            r.bend(species, &tree.axes[bearers[j]])
        });
        bent.into_iter().collect()
    }
}
