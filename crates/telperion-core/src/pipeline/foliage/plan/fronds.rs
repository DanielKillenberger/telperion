//! A frond crown in the plan. Fronds are placements at the stem apices, not
//! wood, so no run describes them: every leaflet of every frond, living or
//! dead, is described by its own oriented box. Each is drawn on the frond's
//! stream exactly as placement draws it (`rosette::fronds`,
//! `leaflet::leaflets`, `leaflet::drawn`), so the planned leaflet stands
//! where the placed one does, and no instance or matrix is built.
use super::Descriptor;
use crate::{
    math::Vec3,
    pipeline::foliage::{
        leaflet::{self, Leaflet},
        organs,
        rosette::{self, frame},
        CanopyParams, Element,
    },
    rng::Rng,
    tree::Tree,
    Error, Result,
};

/// What packing a leaf into twelve bytes may move a corner by: a millimetre
/// for its position and scale, and half a percent of the corner's distance
/// from the attachment for the rotation's ten-bit codes.
const MARGIN: (f64, f64) = (0.001, 0.005);

/// Appends a box for every leaflet of every rosette and returns how many,
/// before any cull: each apex its own whole fronds and each frond its own
/// leaflets (fn-277), as placement draws them. `system` names each node's
/// limb system, a leaflet taking its apex's. Over the instance budget is an
/// error, as placement's count is.
pub(super) fn describe(
    tree: &Tree,
    p: CanopyParams,
    element: &Element,
    seed: u32,
    system: &[u32],
    out: &mut Vec<Descriptor>,
) -> Result<u32> {
    if tree.nodes.len() < 2 || p.size == 0.0 || !rosette::bearing(&p) {
        return Ok(0);
    }
    let budget = || Error::ResourceLimit("foliage instance budget");
    let shape = Shape::of(element);
    let mut total = 0usize;
    for apex in &rosette::rosettes(tree) {
        let system = system[apex.apex];
        let (stem, node) = (rosette::stem_key(tree, apex), rosette::stem_node(tree, apex));
        let at = organs::apex(&p, node);
        if !rosette::bearing(&at) {
            continue;
        }
        let crown = super::super::Crown::of(tree, &at);
        for frond in rosette::fronds(apex, &at, crown, super::super::blade(element)) {
            let key = organs::frond_key(node, frond.serial);
            let per = organs::frond_leaflets(&at, frond.canopy.leaflet_count, key);
            let c = organs::placement(&frond.canopy, key);
            total = total.checked_add(per).filter(|t| *t <= p.max_instances && *t <= u32::MAX as usize).ok_or_else(budget)?;
            out.try_reserve(per).map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
            let mut rng = rosette::stream(seed, stem, &frond);
            if per == 1 {
                // One blade at the station, turned as `fan` turns it.
                let blade = Leaflet {
                    at: frond.at,
                    axis: frond.heading,
                    run: frond.radial,
                    share: frond.split.map(|s| rosette::lone(&s, &c)),
                };
                out.push(shape.boxed(&blade, apex.axis, &c, &mut rng, system));
                continue;
            }
            let (_, side) = frame(frond.heading);
            for leaf in leaflet::leaflets(frond.at, frond.heading, c, per, frond.split) {
                out.push(shape.boxed(&leaf, side, &c, &mut rng, system));
            }
        }
    }
    Ok(total as u32)
}

/// The leaf element's box in its own frame: its centre and half extents.
struct Shape {
    centre: Vec3,
    half: Vec3,
}
impl Shape {
    fn of(element: &Element) -> Self {
        let first = element.positions.first().copied().unwrap_or(Vec3::ZERO);
        let (lo, hi) = element
            .positions
            .iter()
            .fold((first, first), |(lo, hi), v| {
                (
                    Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z)),
                    Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z)),
                )
            });
        Self {
            centre: (lo + hi) * 0.5,
            half: (hi - lo) * 0.5,
        }
    }
    /// The oriented box one drawn leaflet stands in, with the packing margin:
    /// its run along the leaf's axis, its side along its flank and its
    /// radius across its face.
    fn boxed(
        &self,
        leaf: &Leaflet,
        normal: Vec3,
        c: &CanopyParams,
        rng: &mut Rng,
        system: u32,
    ) -> Descriptor {
        let ([flank, axis, face], scale) = leaflet::drawn(leaf.axis, leaf.run, normal, c, rng);
        let k = scale * leaf.share.unwrap_or(1.0);
        let (centre, half) = (self.centre * k, self.half * k);
        let reach = (centre.length() + half.length()) * MARGIN.1 + MARGIN.0;
        let at = leaf.at + flank * centre.x + axis * centre.y + face * centre.z;
        let run = axis * (half.y + reach);
        Descriptor {
            endpoints: [at - run, at + run],
            radii: [half.z + reach; 2],
            count: 1,
            system,
            side: flank * (half.x + reach),
            stand: None,
        }
    }
}
