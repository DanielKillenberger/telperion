//! What one share of a cycle's growth works on (fn-210, lever 2). Every
//! draw is keyed to its lineage, and a growth unit touches only its own
//! axis and the axes it makes, so a cycle's living apexes grow apart, a
//! share on each core, each `Shoot` numbering the axes it makes from the
//! tree's count as if it came first; `parallel.rs` merges them in the
//! apexes' order, and the tree is the one grown in turn.
use super::sketch::Pencil;
use super::{bud, Apex, Certain, Placed};
use crate::dormant::{Sleeper, Woken};
use crate::lineage::Key;
use crate::presence::{Draws, Windows};
use crate::species::{Species, MAX_BUDS};
use crate::structure::{Axis, Origin};

/// What every growth unit reads and none writes: the species, its
/// laterals as drawn, the bound on each PA's life, the windows, and the
/// phytomers grown before the cycle, against the budget.
pub(super) struct Rules<'a> {
    pub species: &'a Species,
    /// Each PA's zones' lateral probabilities as drawn (`PaState::laterals`).
    pub laterals: Vec<Vec<Vec<f64>>>,
    /// Each PA's bound on its subtree's life (`shed::lasting`); none
    /// throughout where shedding is not decided before growth.
    pub lasting: Vec<Option<f64>>,
    pub certain: Certain,
    pub windows: Windows,
    pub age: u32,
    pub budget: u32,
    pub grown: u64,
}

/// What the rough layout gives a growing apex (`sketch.rs`): its light,
/// its unit's size and its subtrees' survival of shedding, by axis; none
/// for an axis made this cycle, which has no pencil yet.
#[derive(Clone, Copy)]
pub(super) struct Reads<'g> {
    pub pencils: &'g [Pencil],
    /// Whether a rough layout is grown at all, and whether any PA reads
    /// its leaves' light into its girth.
    pub sketching: bool,
    pub lights: bool,
}

/// A living axis a share grows, in the tree where it stands: each share
/// holds its own apexes' axes, which no other share touches.
pub(super) struct Home<'g> {
    pub axis: &'g mut Axis,
    pub draws: &'g mut Draws,
    pub units: &'g mut f64,
    pub successor: &'g mut Vec<usize>,
}

/// One axis a share made, beside its `Axis`.
pub(super) struct Line {
    pub axis: Axis,
    pub draws: Draws,
    /// Its time spent in its PA when its apex stopped, or at its birth
    /// while it lives: what a relay of it carries on.
    pub units: f64,
    /// Its continuation or relay, or both where a stop it all but made
    /// grows both (host decision 11): what carries it on.
    pub successor: Vec<usize>,
    /// The lineage that began its chain of ages: the seed's, a lateral's
    /// own, or the one its continuation or relay carries on.
    pub root: u64,
    /// The most, past its subtree's last growth, that it or a lateral or
    /// relay it stands on keeps it before it is shed: a delay and the
    /// cycle its fade runs over.
    pub horizon: f64,
}

/// A share of a cycle's growth: the living axes it advances (`homes`, in
/// its apexes' order), and the axes it makes, numbered from `base`.
pub(super) struct Shoot<'g, 'a> {
    pub rules: &'g Rules<'a>,
    pub reads: Reads<'g>,
    /// The tree's roots and horizons, which no growth unit changes.
    pub roots: &'g [u64],
    pub horizons: &'g [f64],
    pub base: usize,
    pub homes: Vec<Home<'g>>,
    /// The home whose apex grows now, and its axis.
    pub home: (usize, usize),
    pub made: Vec<Line>,
    pub next: Vec<Apex>,
    /// The sleeping buds put to sleep, by the cycle they wake in.
    pub asleep: Vec<(usize, Sleeper)>,
    pub woken: Vec<Woken>,
    pub marked: Vec<u64>,
    /// The phytomers this share grew.
    pub grown: u64,
    /// One zone's node leads and its nodes (order draw, draw index, lead,
    /// buds), reused.
    pub leads: Vec<f64>,
    pub node_buds: Vec<(f64, u64, f64, [Placed; MAX_BUDS as usize])>,
    /// One zone's sleeping buds' expected wood per PA, reused.
    pub sleeping: Vec<f64>,
}

impl<'g, 'a> Shoot<'g, 'a> {
    pub fn new(rules: &'g Rules<'a>, reads: Reads<'g>, tree: (&'g [u64], &'g [f64])) -> Self {
        let (roots, horizons) = tree;
        Self {
            rules,
            reads,
            roots,
            horizons,
            base: roots.len(),
            homes: Vec::new(),
            home: (0, usize::MAX),
            made: Vec::new(),
            next: Vec::new(),
            asleep: Vec::new(),
            woken: Vec::new(),
            marked: Vec::new(),
            grown: 0,
            leads: Vec::new(),
            node_buds: Vec::new(),
            sleeping: Vec::new(),
        }
    }

    pub fn species(&self) -> &'a Species {
        self.rules.species
    }

    pub fn windows(&self) -> &'g Windows {
        &self.rules.windows
    }

    /// The home growing now, which axis `i` below `base` must be: a
    /// growth unit touches only its own axis and those it makes.
    fn at_home(&self, i: usize) -> usize {
        debug_assert_eq!(self.home.1, i, "a growth unit touches only its own axis");
        self.home.0
    }

    /// Axis `i`'s root (`Line::root`).
    pub fn root(&self, i: usize) -> u64 {
        if i >= self.base {
            self.made[i - self.base].root
        } else {
            self.roots[i]
        }
    }

    /// Axis `i`'s horizon (`Line::horizon`).
    pub fn horizon(&self, i: usize) -> f64 {
        if i >= self.base {
            self.made[i - self.base].horizon
        } else {
            self.horizons[i]
        }
    }

    pub fn axis(&self, i: usize) -> &Axis {
        if i >= self.base {
            return &self.made[i - self.base].axis;
        }
        &*self.homes[self.at_home(i)].axis
    }

    pub fn axis_mut(&mut self, i: usize) -> &mut Axis {
        if i >= self.base {
            return &mut self.made[i - self.base].axis;
        }
        let k = self.at_home(i);
        &mut *self.homes[k].axis
    }

    pub fn draws(&self, i: usize) -> &Draws {
        if i >= self.base {
            return &self.made[i - self.base].draws;
        }
        &*self.homes[self.at_home(i)].draws
    }

    pub fn draws_mut(&mut self, i: usize) -> &mut Draws {
        if i >= self.base {
            return &mut self.made[i - self.base].draws;
        }
        let k = self.at_home(i);
        &mut *self.homes[k].draws
    }

    /// Axis `i`'s time spent in its PA (`Line::units`).
    pub fn units_mut(&mut self, i: usize) -> &mut f64 {
        if i >= self.base {
            return &mut self.made[i - self.base].units;
        }
        let k = self.at_home(i);
        &mut *self.homes[k].units
    }

    pub fn successor_mut(&mut self, i: usize) -> &mut Vec<usize> {
        if i >= self.base {
            return &mut self.made[i - self.base].successor;
        }
        let k = self.at_home(i);
        &mut *self.homes[k].successor
    }

    /// The light axis `i`'s apex grows in: from the leaves of the cycle
    /// before, or whole where nothing shades.
    pub fn light(&self, i: usize) -> f64 {
        self.reads.pencils.get(i).map_or(1.0, |p| p.light)
    }

    /// Axis `i`'s growth unit's size this cycle (`allocation.rs`); whole
    /// where nothing shades.
    pub fn size(&self, i: usize) -> f64 {
        self.reads.pencils.get(i).map_or(1.0, |p| p.size)
    }

    /// What the subtrees' survival of their shedding this cycle makes of
    /// axis `i`'s unit: whole where nothing sheds.
    pub fn kept(&self, i: usize) -> f64 {
        self.reads.pencils.get(i).map_or(1.0, |p| p.kept)
    }

    /// The index the next axis made takes.
    pub fn len(&self) -> usize {
        self.base + self.made.len()
    }

    /// A new axis of `pa` growing from `origin`, its lineage `key`.
    pub fn push(
        &mut self,
        key: Key,
        pa: usize,
        cycle: u32,
        origin: Origin,
        line: (f64, u64, Draws),
    ) {
        let (units, root, draws) = line;
        let horizon = self.horizon_of(pa, origin);
        self.made.push(Line {
            axis: bud(key, pa, cycle, origin),
            draws,
            units,
            successor: Vec::new(),
            root,
            horizon,
        });
    }

    /// The horizon of a new axis of `pa` growing from `origin`.
    fn horizon_of(&self, pa: usize, origin: Origin) -> f64 {
        let Some(parent) = origin.parent() else {
            return f64::NEG_INFINITY;
        };
        let delay = self.species().states[pa].shedding;
        let horizon = self.horizon(parent);
        match origin {
            Origin::Lateral { .. } | Origin::Relay { .. } if delay.is_finite() => {
                horizon.max(delay + 1.0)
            }
            _ => horizon,
        }
    }

    /// Whether a lateral of `pa` made in `cycle` on axis `parent` is
    /// certain to be shed by the tree's age: its subtree's last growth
    /// ends by its bound, and from there its own delay and those of every
    /// lateral or relay it stands on have run out by the tree's age, so
    /// its fade there is 0 and no kept axis's fade reads it (`shed.rs`).
    pub fn shed_certain(&self, pa: usize, cycle: u32, parent: usize) -> bool {
        let (Some(last), delay) = (self.rules.lasting[pa], self.species().states[pa].shedding)
        else {
            return false;
        };
        let horizon = self.horizon(parent).max(delay + 1.0);
        let age = f64::from(self.rules.age);
        delay.is_finite() && horizon <= age - (f64::from(cycle) + last)
    }

    /// A new bud that grows from the next cycle, made by draws with these
    /// presences. A seed or lateral `key` is the bud's own, and its axis is
    /// keyed by its age's place on the reference axis from it, as every
    /// continuation of it is (`Key::onto`), so an age it passes through or
    /// grows in moves no key; a continuation's or relay's `key` is its
    /// axis's own. A relay carries on its axis's time spent in its PA.
    pub fn sprout(&mut self, key: Key, pa: usize, cycle: u32, origin: Origin, made: [f64; 2]) {
        let spent = match origin {
            Origin::Relay { parent, .. } => *self.units_mut(parent),
            _ => 0.0,
        };
        let (root, lineage) = match origin {
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                (self.root(parent), key)
            }
            Origin::Seed | Origin::Lateral { .. } => (key.0, key.onto(pa)),
        };
        self.next.push(Apex {
            axis: self.len(),
            spent,
        });
        let draws = Draws {
            birth: made,
            ..Draws::default()
        };
        self.push(lineage, pa, cycle, origin, (spent, root, draws));
    }

    /// The bud last sprouted grows from the next cycle, or, where the
    /// cycle's `used` share is not whole, in this one at the rest of it.
    pub fn carry_on(&mut self, cycle: u32, used: f64) -> crate::error::Result<()> {
        if used >= 1.0 {
            return Ok(());
        }
        let apex = self.next.pop().expect("sprouted");
        self.axis_mut(apex.axis).birth = cycle - 1;
        self.draws_mut(apex.axis).sleep = used;
        self.advance(apex, cycle)
    }
}
