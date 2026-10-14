//! A rosette's stem establishing with its crown (fn-275 R2a, host). The
//! stem's own clock is the height it has laid: a stem phytomer whose tip
//! was laid `h` metres up reached it at the crown's age `t = h / rise`,
//! its trunk PA's rise a year. It is drawn at the girth of an organ made
//! then (`foliage::establishment`), never thinner than the thinnest wood
//! drawn, and lowered by the rise its stem waited on the crown
//! (`foliage::waited`), so the stem stays at the ground until the crown is
//! established and then elongates. Each is a closed form of the laid
//! height alone, so the lowering composes along the stem with no sum, and
//! a shoot drawn part grown within its year is lowered by the height it
//! has reached, never past the wood below it. Every axis off the stem is
//! carried by its bearing node's lowering and keeps its own lengths and
//! girth. `cycle/establish.wgsl` is its twin on the device.
use super::FINEST;
use crate::pipeline::foliage::{establishment, waited, CanopyParams};
use std::borrow::Cow;
use telperion_space::{Coarse, Origin, Species, Structure};

/// The rows a stem establishes by: the crown's establishment years and
/// early share, and its trunk PA's rise a year.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Establishing {
    pub years: f64,
    pub early: f64,
    pub rise: f64,
}

impl Establishing {
    /// A crown that stands a rosette and states an establishment, on the
    /// species' first trunk PA, which rises; none otherwise (dormant).
    pub(super) fn of(canopy: &CanopyParams, species: &Species, trunk: &[usize]) -> Option<Self> {
        if canopy.rosette_fronds == 0.0 || canopy.frond_establishment <= 0.0 {
            return None;
        }
        let state = species.states.get(*trunk.first()?)?;
        let nodes: f64 = state.zones.iter().map(|z| z.nodes.mean()).sum();
        let rise = state.internode * nodes;
        (rise > 0.0).then_some(Self { years: canopy.frond_establishment, early: canopy.frond_early_share, rise })
    }

    /// The device's words (`dress.wgsl`'s `Cfg`): none where dormant.
    pub fn words(this: Option<Self>) -> [f32; 3] {
        this.map_or([0.0; 3], |e| [e.years, e.early, e.rise].map(|v| v as f32))
    }

    /// The metres a stem tip laid `h` up is lowered by.
    fn lowered(&self, h: f64) -> f64 {
        self.rise * waited(self.years, h / self.rise)
    }

    /// A stem phytomer's radius `r` at its tip laid `h` up.
    fn girth(&self, r: f64, h: f64) -> f64 {
        (r * establishment(self.early, self.years, h / self.rise)).max(r.min(FINEST))
    }

    /// `structure` with its stem established; untouched where dormant.
    pub(super) fn apply<'s>(this: Option<Self>, structure: &'s Structure, trunk: &[usize]) -> Cow<'s, Structure> {
        let Some(e) = this else {
            return Cow::Borrowed(structure);
        };
        let mut out = structure.clone();
        e.lower(&mut out, trunk);
        Cow::Owned(out)
    }

    /// A coarse tree established, each fine root's stand-in carried with
    /// the axis it stands for.
    pub(super) fn coarse(this: Option<Self>, mut coarse: Coarse, trunk: &[usize]) -> Coarse {
        if let Some(e) = this {
            let carried = e.lower(&mut coarse.structure, trunk);
            for s in &mut coarse.stands {
                s.base.z -= carried[s.axis];
            }
        }
        coarse
    }

    /// Lowers and narrows the stem of `structure` and carries what it
    /// bears; each axis's base lowering, by index.
    fn lower(&self, structure: &mut Structure, trunk: &[usize]) -> Vec<f64> {
        // Each axis's phytomers' lowering, parents before what they bear.
        let mut lowering: Vec<Vec<f64>> = Vec::with_capacity(structure.axes.len());
        let mut bases = Vec::with_capacity(structure.axes.len());
        for axis in &mut structure.axes {
            let bearing = match axis.origin {
                Origin::Seed => 0.0,
                Origin::Lateral { parent, node, .. } | Origin::Relay { parent, node } => {
                    lowering[parent].get(node).copied().unwrap_or(0.0)
                }
                Origin::Continuation { parent } => lowering[parent].last().copied().unwrap_or(0.0),
            };
            axis.base.z -= bearing;
            bases.push(bearing);
            let own = trunk.contains(&axis.pa);
            let mine = (axis.phytomers.iter_mut())
                .map(|p| {
                    let lower = match own {
                        true => {
                            let h = p.tip.z;
                            p.radius = self.girth(p.radius, h);
                            self.lowered(h)
                        }
                        false => bearing,
                    };
                    p.tip.z -= lower;
                    lower
                })
                .collect();
            lowering.push(mine);
        }
        bases
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presets::Preset;
    use telperion_space::{grow, Request};

    /// R2a (host): off the stem an axis is carried by its bearing node's
    /// lowering and keeps its own lengths and girth; the stem's phytomers
    /// are lowered and narrowed, never raised. The oak, with a rosette's
    /// rows stated, stands in for a branched tree that bears one.
    #[test]
    fn an_axis_off_the_stem_is_carried_whole() {
        let family = Preset::OregonWhiteOak.parameters();
        let species = family.space.species.unwrap();
        let request = Request { age: 8, seed: 3, budget: 1 << 24, light: family.space.light };
        let before = grow(&species, request).unwrap();
        let trunk = &family.space.trunk;
        let e = Some(Establishing { years: 6.75, early: 0.1, rise: 0.4 });
        let after = Establishing::apply(e, &before, trunk);
        let mut carried = 0;
        for (a, b) in before.axes.iter().zip(&after.axes) {
            let drops: Vec<f64> = a.phytomers.iter().zip(&b.phytomers).map(|(p, q)| p.tip.z - q.tip.z).collect();
            assert!(drops.iter().all(|d| *d >= 0.0), "a phytomer was raised");
            if trunk.contains(&a.pa) {
                assert!(a.phytomers.iter().zip(&b.phytomers).all(|(p, q)| q.radius <= p.radius));
                continue;
            }
            let base = a.base.z - b.base.z;
            assert!(drops.iter().all(|d| (d - base).abs() < 1e-12), "an axis off the stem bent");
            assert!(a.phytomers.iter().zip(&b.phytomers).all(|(p, q)| p.radius == q.radius));
            carried += usize::from(base > 0.0);
        }
        assert!(carried > 0, "no axis off the stem was carried");
        assert_eq!(Establishing::apply(None, &before, trunk).as_ref(), &before, "dormant moved");
    }
}
