//! The rings read in place from a wood against the rings a sweep computes.
use super::*;
use crate::presets::{by_identity, Preset, CATALOGUE, IN_WORK};

/// Every node's ring neighbourhood holds the same points, and a seat
/// projected from its axis lands on the same point, on every shipped and
/// in-work family seated on its wood: fork sockets, caps, the flare and the
/// palm's leaf bases, under the parallel wood and the serial one.
#[test]
fn contacts_read_from_the_wood_are_the_swept_ones() {
    for &(_, id, _, _) in CATALOGUE.iter().chain(IN_WORK) {
        let mut family =
            by_identity(id).unwrap_or_else(|_| Preset::from_id(id).unwrap().parameters());
        family.canopy.surface_contact = 1.0;
        let tree = crate::pipeline::skeleton(&family).unwrap().tree;
        let (height, params) = (family.skeleton.envelope.height, &family.surface);
        let swept = AttachmentSurface::new(&tree, height, params).unwrap();
        let plain = build(&tree, height, params).unwrap();
        let mut serial = WoodWithContacts {
            mesh: SurfaceMesh::default(),
            edges: filled(tree.nodes.len(), None).unwrap(),
        };
        serial.mesh =
            build_mode(&tree, height, params, None, Some(&mut serial.edges), false).unwrap();
        for wood in [build_contacts(&tree, height, params).unwrap(), serial] {
            assert_eq!(wood.mesh, plain, "{id}: wood");
            let read = AttachmentSurface::on_wood(&wood, params).unwrap();
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
        }
    }
}
