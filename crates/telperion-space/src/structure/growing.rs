//! A phytomer as a grower holds it (fn-358, READING.md): only what growth
//! reads between cycles, at the width it was written. The rough layout
//! writes every frame, scale, rank, size and grown share as an `f32`, so
//! each is held as one and read back as the same `f64`; the radius is laid
//! only by the settle, which widens each phytomer into the tree's
//! (`Growing::settled`).
use super::{Axis, Phytomer, Vec3};

/// What growth and the settle read of a phytomer, at the settled tree's
/// widths, and what the walk lays in it.
pub(crate) trait Node {
    fn cycle(&self) -> u32;
    fn key(&self) -> u64;
    fn tip(&self) -> Vec3;
    fn heading(&self) -> Vec3;
    fn side(&self) -> Vec3;
    fn radius(&self) -> f64;
    fn scale(&self) -> f64;
    fn grown(&self) -> f64;
    fn size(&self) -> f64;
    fn rank(&self) -> f64;
    /// Laid by the walk (`Layer::step`): its node, and its direction and
    /// side there.
    fn lay(&mut self, tip: [f32; 3], heading: [f32; 3], side: [f32; 3]);
    /// Its whole section and its sapwood, m² (`girth::lay`).
    fn wood(&mut self) -> (&mut f32, &mut f32);
}

/// A growing phytomer: `Phytomer`'s fields growth reads, each at the width
/// growth wrote it, and no radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Growing {
    pub cycle: u32,
    pub key: u64,
    pub tip: [f32; 3],
    pub heading: [f32; 3],
    pub side: [f32; 3],
    pub scale: f32,
    pub grown: f32,
    pub size: f32,
    pub rank: f32,
    pub section: f32,
    pub sapwood: f32,
}

/// `v` as the tree's vector.
fn wide(v: [f32; 3]) -> Vec3 {
    Vec3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[2]))
}

impl Growing {
    /// The phytomer grown in `cycle` at the node keyed `key`, not yet laid.
    pub fn new(cycle: u32, key: u64) -> Self {
        Self {
            cycle,
            key,
            tip: [0.0; 3],
            heading: [0.0; 3],
            side: [0.0; 3],
            scale: 1.0,
            grown: 0.0,
            size: 1.0,
            rank: 0.0,
            section: 0.0,
            sapwood: 0.0,
        }
    }

    /// The settled tree's phytomer: every field widened, to the bit, and
    /// no radius yet.
    pub fn settled(&self) -> Phytomer {
        Phytomer {
            cycle: self.cycle,
            tip: wide(self.tip),
            heading: wide(self.heading),
            side: wide(self.side),
            radius: 0.0,
            scale: f64::from(self.scale),
            key: self.key,
            rank: f64::from(self.rank),
            size: f64::from(self.size),
            grown: f64::from(self.grown),
            section: self.section,
            sapwood: self.sapwood,
        }
    }
}

impl Node for Growing {
    fn cycle(&self) -> u32 {
        self.cycle
    }
    fn key(&self) -> u64 {
        self.key
    }
    fn tip(&self) -> Vec3 {
        wide(self.tip)
    }
    fn heading(&self) -> Vec3 {
        wide(self.heading)
    }
    fn side(&self) -> Vec3 {
        wide(self.side)
    }
    fn radius(&self) -> f64 {
        0.0
    }
    fn scale(&self) -> f64 {
        f64::from(self.scale)
    }
    fn grown(&self) -> f64 {
        f64::from(self.grown)
    }
    fn size(&self) -> f64 {
        f64::from(self.size)
    }
    fn rank(&self) -> f64 {
        f64::from(self.rank)
    }
    fn lay(&mut self, tip: [f32; 3], heading: [f32; 3], side: [f32; 3]) {
        (self.tip, self.heading, self.side) = (tip, heading, side);
    }
    fn wood(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.section, &mut self.sapwood)
    }
}

impl Node for Phytomer {
    fn cycle(&self) -> u32 {
        self.cycle
    }
    fn key(&self) -> u64 {
        self.key
    }
    fn tip(&self) -> Vec3 {
        self.tip
    }
    fn heading(&self) -> Vec3 {
        self.heading
    }
    fn side(&self) -> Vec3 {
        self.side
    }
    fn radius(&self) -> f64 {
        self.radius
    }
    fn scale(&self) -> f64 {
        self.scale
    }
    fn grown(&self) -> f64 {
        self.grown
    }
    fn size(&self) -> f64 {
        self.size
    }
    fn rank(&self) -> f64 {
        self.rank
    }
    fn lay(&mut self, tip: [f32; 3], heading: [f32; 3], side: [f32; 3]) {
        (self.tip, self.heading, self.side) = (wide(tip), wide(heading), wide(side));
    }
    fn wood(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.section, &mut self.sapwood)
    }
}

impl Axis<Growing> {
    /// The settled tree's axis: its phytomers widened (`Growing::settled`).
    pub(crate) fn settled(self) -> Axis {
        let phytomers = self.phytomers.iter().map(Growing::settled).collect();
        Axis {
            lineage: self.lineage,
            pa: self.pa,
            birth: self.birth,
            origin: self.origin,
            vigour: self.vigour,
            blend: self.blend,
            apex_end: self.apex_end,
            base: self.base,
            heading: self.heading,
            side: self.side,
            phytomers,
            units: self.units,
            alive: self.alive,
            rank: self.rank,
            sleep: self.sleep,
            phase: self.phase,
            joined: self.joined,
        }
    }
}
