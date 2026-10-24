//! The real leaflet shapes a planned palm is held to (fn-150): each placed
//! leaf's own oriented box, read from its stored transform, as a plan a
//! field reads.
#![allow(dead_code)]
use telperion_core::{
    foliage::{
        plan::{Descriptor, Plan},
        Element, Instances,
    },
    math::Vec3,
};

/// `leaves` as one oriented box each, no margin.
pub fn leaflets(leaves: &Instances, element: &Element) -> Plan {
    let first = element.positions[0];
    let (lo, hi) = element
        .positions
        .iter()
        .fold((first, first), |(lo, hi), v| {
            (
                Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z)),
                Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z)),
            )
        });
    let (centre, half) = ((lo + hi) * 0.5, (hi - lo) * 0.5);
    let descriptors = leaves
        .matrices()
        .map(|m| {
            let column =
                |c: usize| Vec3::new(m[c * 4] as f64, m[c * 4 + 1] as f64, m[c * 4 + 2] as f64);
            let [x, y, z] = [column(0), column(1), column(2)];
            let at = Vec3::new(m[12] as f64, m[13] as f64, m[14] as f64)
                + x * centre.x
                + y * centre.y
                + z * centre.z;
            Descriptor {
                endpoints: [at - y * half.y, at + y * half.y],
                radii: [z.length() * half.z; 2],
                count: 1,
                system: 0,
                side: x * half.x,
            }
        })
        .collect();
    Plan {
        descriptors,
        total: leaves.len() as u32,
        blade: 0.0,
        seat: 1.0,
        rachis: 0.0,
    }
}
