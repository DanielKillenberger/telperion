//! The grown tree: axes of phytomers, each axis one development axis (a run
//! of one PA). A leafy axis whose apex changed PA is a chain of development
//! axes joined by `Origin::Continuation`. Parents precede their children.
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn dot(self, o: Self) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    /// The unit vector along this one; none for a vector of no length.
    pub fn unit(self) -> Option<Self> {
        let length = self.length();
        (length > 1e-12).then(|| self * (1.0 / length))
    }
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

/// Where an axis starts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Origin {
    /// The seed bud, at the ground.
    Seed,
    /// A lateral bud: `slot` of the `whorl` buds at phytomer `node` of axis `parent`.
    Lateral {
        parent: usize,
        node: usize,
        slot: u8,
        whorl: u8,
    },
    /// The parent's apex, changed to this axis's PA.
    Continuation { parent: usize },
    /// A relay bud of the parent's PA, made when its apex stopped, standing
    /// at the parent PA's `relay_at` along it: at phytomer `node`'s span
    /// (set when the tree is placed).
    Relay { parent: usize, node: usize },
}

impl Origin {
    /// The axis this one grows from; none for the seed.
    pub fn parent(self) -> Option<usize> {
        match self {
            Origin::Seed => None,
            Origin::Lateral { parent, .. }
            | Origin::Continuation { parent }
            | Origin::Relay { parent, .. } => Some(parent),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    /// The hash of the bud's path from the root: the same bud under any
    /// settings at the same seed.
    pub lineage: u64,
    /// The physiological age, an index into the species' reference axis.
    pub pa: usize,
    /// The cycle the bud was made; its first growth unit grows the next cycle.
    pub birth: u32,
    pub origin: Origin,
    /// The axis's size at birth relative to what bears it, 0 to 1: a branch
    /// a setting has just made, or is about to unmake or shed, is small.
    pub vigour: f64,
    /// The cycle the apex stopped (died, ended, or changed PA); none, it lives.
    pub apex_end: Option<u32>,
    pub base: Vec3,
    /// The unit growth direction and the unit side its first lateral faces.
    pub heading: Vec3,
    pub side: Vec3,
    /// Base to tip.
    pub phytomers: Vec<Phytomer>,
    /// The presence of each growth unit its apex grew, base to tip.
    pub(crate) units: Vec<f64>,
    /// Its apex's presence at the tree's age: every survival it passed,
    /// barely or not; 0 for a stopped apex.
    pub(crate) alive: f64,
    /// Its nodes counted by their presence: the phyllotactic rank of the next.
    pub(crate) rank: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phytomer {
    /// The cycle the phytomer grew in, from 1.
    pub cycle: u32,
    /// Its node, the internode's upper end.
    pub tip: Vec3,
    /// The unit direction its internode grew in, and the unit side its
    /// node's first bud faces, carried along the axis as it bends.
    pub heading: Vec3,
    pub side: Vec3,
    /// Its internode's radius in metres, by the pipe model.
    pub radius: f64,
    /// Its length and girth relative to a fully grown phytomer, 0 to 1:
    /// every presence on its lineage multiplied.
    pub scale: f64,
    /// The key of its node's draw: the same node under any settings.
    pub(crate) key: u64,
    /// Its place in the phyllotaxis: the nodes below it counted by their
    /// presence, so a node growing in turns the ones above it by degree.
    pub(crate) rank: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    /// Cycles grown.
    pub age: u32,
    /// Physiological ages on the species' reference axis.
    pub pas: usize,
    pub axes: Vec<Axis>,
}

/// A count of phytomers per PA and per cycle of growth (1..=cycles).
#[derive(Debug, Clone, PartialEq)]
pub struct CountTable {
    pub pas: usize,
    pub cycles: usize,
    cells: Vec<f64>,
}

impl CountTable {
    pub fn new(pas: usize, cycles: usize) -> Self {
        Self {
            pas,
            cycles,
            cells: vec![0.0; pas * cycles],
        }
    }
    /// The count of PA `pa` grown in `cycle` (from 1).
    pub fn get(&self, pa: usize, cycle: usize) -> f64 {
        self.cells[pa * self.cycles + cycle - 1]
    }
    pub fn add(&mut self, pa: usize, cycle: usize, count: f64) {
        self.cells[pa * self.cycles + cycle - 1] += count;
    }
    /// All phytomers of `pa`.
    pub fn total(&self, pa: usize) -> f64 {
        self.cells[pa * self.cycles..(pa + 1) * self.cycles]
            .iter()
            .sum()
    }
}

impl Structure {
    pub fn phytomer_count(&self) -> usize {
        self.axes.iter().map(|axis| axis.phytomers.len()).sum()
    }

    pub fn counts(&self) -> CountTable {
        let mut table = CountTable::new(self.pas, self.age as usize);
        for axis in &self.axes {
            for phytomer in &axis.phytomers {
                table.add(axis.pa, phytomer.cycle as usize, 1.0);
            }
        }
        table
    }
}
