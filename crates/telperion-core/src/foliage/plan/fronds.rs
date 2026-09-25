//! A frond crown in the plan. Fronds are placements at the stem apices, not
//! wood, so no run describes them: each frond, living or dead, is walked
//! along its arched rachis in a few chords from the apex, on the spiral,
//! frame and scale the placement hangs it at (`rosette::fronds`), and each
//! chord counts the leaflets the placement strings along it. Nothing here
//! draws: the rachis's arch is fixed by the rows, and a leaflet's own draws
//! turn and scale it about a point that lies on the rachis.
use super::Descriptor;
use crate::{
    foliage::{
        rosette::{self, frame},
        CanopyParams,
    },
    tree::Tree,
    Error, Result,
};

/// Chords a rachis is walked in: at the palm's 7 m rachis and -0.6 arch,
/// each chord stands at most 0.12 m off the curve.
const CHORDS: u32 = 3;

/// Appends every rosette's frond chords and returns the leaves they carry,
/// before any cull. `extent` is the leaf element's farthest vertex from its
/// attachment; `system` names each node's limb system, a frond taking its
/// apex's. Over the instance budget is an error, as placement's count is.
pub(super) fn describe(
    tree: &Tree,
    p: CanopyParams,
    extent: f64,
    system: &[u32],
    out: &mut Vec<Descriptor>,
) -> Result<u32> {
    let budget = || Error::ResourceLimit("foliage instance budget");
    if tree.nodes.len() < 2 || p.size == 0.0 {
        return Ok(0);
    }
    let apices = rosette::rosettes(tree);
    let per = rosette::leaflets(&p) as u32;
    let fronds = p.rosette_fronds + rosette::skirt(&p);
    let total = (apices.len() as u64) * u64::from(fronds) * u64::from(per);
    if total > p.max_instances as u64 || total > u64::from(u32::MAX) {
        return Err(budget());
    }
    let chords = if per > 1 { CHORDS } else { 1 };
    out.try_reserve(apices.len() * (fronds * chords) as usize)
        .map_err(|_| Error::ResourceLimit("foliage plan allocation"))?;
    for apex in &apices {
        for frond in rosette::fronds(apex, &p) {
            let c = frond.canopy;
            let blade = extent * c.size * (1.0 + c.size_variation);
            if per == 1 {
                out.push(Descriptor {
                    endpoints: [frond.at; 2],
                    radii: [blade; 2],
                    count: 1,
                    system: system[apex.apex],
                });
                continue;
            }
            // The rachis `fan` strings the leaflets along, leaflet `i` at
            // `t = (i + 1) / per`.
            let (lift, _) = frame(frond.heading);
            let (length, arch) = (c.rachis_length, c.rachis_arch);
            let at =
                |t: f64| frond.at + frond.heading * (length * t) + lift * (arch * length * t * t);
            let step = 1.0 / f64::from(chords);
            // A parabola stands at most a quarter of its arch times the
            // chord's parameter span squared off that chord.
            let sagitta = arch.abs() * length * step * step / 4.0;
            for j in 0..chords {
                let count = per * (j + 1) / chords - per * j / chords;
                if count == 0 {
                    continue;
                }
                let (t0, t1) = (f64::from(j) * step, f64::from(j + 1) * step);
                out.push(Descriptor {
                    endpoints: [at(t0), at(t1)],
                    radii: [sagitta + blade; 2],
                    count,
                    system: system[apex.apex],
                });
            }
        }
    }
    Ok(total as u32)
}
