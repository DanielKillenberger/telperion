//! The rings read in place from the ring step against the rings a sweep
//! computes.
use super::*;
use crate::presets::{by_identity, Preset, CATALOGUE, IN_WORK};

/// Every node's ring neighbourhood holds the same points, and a seat
/// projected from its axis lands on the same point, on every shipped and
/// in-work family seated on its rings: fork sockets, caps, the flare and the
/// palm's leaf bases, under the parallel ring step and the serial one; and
/// the drawn rings make the wood a plain build does.
#[test]
fn contacts_read_from_the_wood_are_the_swept_ones() {
    for &(_, id, _, _) in CATALOGUE.iter().chain(IN_WORK) {
        let mut family =
            by_identity(id).unwrap_or_else(|_| Preset::from_id(id).unwrap().parameters());
        family.canopy.surface_contact = 1.0;
        let tree = crate::mesh::grow(&family).unwrap();
        let (height, params) = (family.skeleton.envelope.height, &family.surface);
        let swept = AttachmentSurface::new(&tree, height, params).unwrap();
        let plain = build(&tree, height, params).unwrap();
        for drawn in [true, false] {
            let sweep = Sweep { drawn, edges: true };
            // Drawn rings as the parallel step sweeps them, bare ones in turn.
            let rings = super::rings::rings_mode(&tree, height, params, sweep, drawn).unwrap();
            let read = AttachmentSurface::on_wood(&rings).unwrap();
            for (node, n) in tree.nodes.iter().enumerate().skip(1) {
                assert_eq!(read.signature(node), swept.signature(node), "{id} {node}");
                let parent = tree.nodes[n.parent.unwrap() as usize].position;
                let origin = (parent + n.position) * 0.5;
                let side = (n.position - parent).cross(Vec3::new(0.3, 0.1, 0.9));
                if side.length_squared() <= 0.0 {
                    continue;
                }
                let radial = side * (1.0 / side.length_squared().sqrt());
                let seat = |s: &AttachmentSurface| s.point(node, origin, radial, n.radius);
                assert_eq!(seat(&read), seat(&swept), "{id} {node}");
            }
            drop(read);
            if drawn {
                let faces = faces(&rings, &tree, height, params).unwrap();
                assert_eq!(rings.into_mesh(faces), plain, "{id}: wood");
            }
        }
    }
}
