//! A rosette's stem establishing with its crown (fn-275 R2a, host): a stem
//! phytomer laid at the crown's age `t` is drawn at the girth of an organ
//! made then (`foliage::establishment`), and lowered by the rise its stem
//! waited on the crown (`foliage::waited`, at its trunk PA's rise a year),
//! so the stem stays at the ground until the crown is established and
//! then elongates. Each is a function of the age a phytomer was laid at
//! alone, so the lowering composes along the stem with no sum; every axis
//! off the stem is carried by its bearing node's lowering and keeps its
//! own lengths and girth. `dress.wgsl`'s `lowered` is its twin.
use crate::pipeline::foliage::{establishment, waited, CanopyParams};
use std::borrow::Cow;
use telperion_space::{Origin, Species, Structure};

/// The rows a stem establishes by: the crown's establishment years and
/// early share, and its trunk PA's rise a year, which its year's
/// phytomers lay evenly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Establishing {
    pub years: f64,
    pub early: f64,
    pub rise: f64,
}

impl Establishing {
    /// A crown that stands a rosette and states an establishment, on the
    /// species' first trunk PA; none otherwise (dormant).
    pub(super) fn of(canopy: &CanopyParams, species: &Species, trunk: &[usize]) -> Option<Self> {
        if canopy.rosette_fronds == 0 || canopy.frond_establishment <= 0.0 {
            return None;
        }
        let pa = *trunk.first()?;
        let state = species.states.get(pa)?;
        let nodes: f64 = state.zones.iter().map(|z| z.nodes.mean()).sum();
        Some(Self {
            years: canopy.frond_establishment,
            early: canopy.frond_early_share,
            rise: state.internode * nodes,
        })
    }

    /// The device's words (`dress.wgsl`'s `Cfg`): none where dormant.
    pub fn words(this: Option<Self>) -> [f32; 3] {
        this.map_or([0.0; 3], |e| [e.years, e.early, e.rise].map(|v| v as f32))
    }

    /// The crown's age a phytomer of `cycle` was laid at, `j` of the `n`
    /// its axis laid that cycle: evenly over the year, as its stem rises.
    fn laid(cycle: u32, j: usize, n: usize) -> f64 {
        f64::from(cycle) - 1.0 + (j + 1) as f64 / n as f64
    }

    /// The metres a stem phytomer laid at `t` is lowered by.
    fn lowered(&self, t: f64) -> f64 {
        self.rise * waited(self.years, t)
    }

    /// `structure` with its stem established; untouched where dormant.
    pub(super) fn apply<'s>(this: Option<Self>, structure: &'s Structure, trunk: &[usize]) -> Cow<'s, Structure> {
        let Some(e) = this else {
            return Cow::Borrowed(structure);
        };
        let mut out = structure.clone();
        // Each axis's phytomers' lowering, parents before what they bear.
        let mut lowering: Vec<Vec<f64>> = Vec::with_capacity(out.axes.len());
        for axis in &mut out.axes {
            let bearing = match axis.origin {
                Origin::Seed => 0.0,
                Origin::Lateral { parent, node, .. } | Origin::Relay { parent, node } => {
                    lowering[parent].get(node).copied().unwrap_or(0.0)
                }
                Origin::Continuation { parent } => lowering[parent].last().copied().unwrap_or(0.0),
            };
            axis.base.z -= bearing;
            let own = trunk.contains(&axis.pa);
            let cycles: Vec<u32> = axis.phytomers.iter().map(|p| p.cycle).collect();
            let mut mine = Vec::with_capacity(cycles.len());
            for (k, p) in axis.phytomers.iter_mut().enumerate() {
                let lower = match own {
                    true => {
                        let j = cycles[..k].iter().rev().take_while(|&&c| c == p.cycle).count();
                        let n = j + cycles[k..].iter().take_while(|&&c| c == p.cycle).count();
                        let t = Self::laid(p.cycle, j, n);
                        p.radius *= establishment(e.early, e.years, t);
                        e.lowered(t)
                    }
                    false => bearing,
                };
                p.tip.z -= lower;
                mine.push(lower);
            }
            lowering.push(mine);
        }
        Cow::Owned(out)
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
