//! The tree the core generates, pinned. Every literal here was recorded on the
//! commit before the foliage element gained its levels, so the pins are a
//! photograph of the old tree rather than a description of the new one: if a
//! level ever moved a leaf, merged one or dropped one, these numbers move too.
//! No device is needed; this is the core's own arithmetic.
use telperion_core::{
    branching,
    mesh::{self, Detail},
    presets::Preset,
};

/// FNV-1a over the bytes, the pattern the branching audit already pins with.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

struct Pin {
    id: &'static str,
    wood_vertices: usize,
    wood_triangles: usize,
    instances: usize,
    min: [f64; 3],
    max: [f64; 3],
    /// Node positions and parent links, base to tip.
    skeleton: u64,
    /// Every retained instance matrix as the renderer receives it.
    placement: u64,
    /// The element's positions and its whole index list.
    element: u64,
}

const SEED: u32 = 7;
const PINS: [Pin; 2] = [
    Pin {
        id: "oregon-white-oak",
        wood_vertices: 2725520,
        wood_triangles: 5280080,
        instances: 555204,
        min: [-12.552015155569293, -0.09600000083446503, -8.909661746289867],
        max: [7.6250192030221, 17.548194502208634, 12.548721025940232],
        skeleton: 13355296115593757449,
        placement: 7063868679653336678,
        element: 11695892805494521029,
    },
    Pin {
        id: "norway-spruce",
        wood_vertices: 3575758,
        wood_triangles: 6911320,
        instances: 7895664,
        min: [-4.401611767518741, -0.05999999865889549, -4.4050304090584245],
        max: [4.039314475833026, 15.002932289485305, 4.19975920363733],
        skeleton: 4142571929448477079,
        placement: 2060019899569959725,
        element: 15959525298297657089,
    },
];

#[test]
fn oak_and_spruce_meshes_are_the_tree_recorded_before_the_levels() {
    for pin in &PINS {
        let id = pin.id;
        let mut family = Preset::from_id(id).expect("preset identity").parameters();
        family.skeleton.seed = SEED;
        let tree = branching::generate(&family.skeleton, family.radii)
            .unwrap_or_else(|e| panic!("{id}: {e}"))
            .tree;
        assert_eq!(
            fnv(tree.nodes.iter().skip(1).flat_map(|n| {
                [n.position.x, n.position.y, n.position.z]
                    .into_iter()
                    .flat_map(f64::to_le_bytes)
                    .chain(n.parent.expect("non-root parent").to_le_bytes())
            })),
            pin.skeleton,
            "{id}: skeleton moved"
        );

        let m = mesh::build(&family, Detail::Full).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(
            (m.wood_vertices(), m.wood_triangles(), m.foliage_instances()),
            (pin.wood_vertices, pin.wood_triangles, pin.instances),
            "{id}: mesh counts moved"
        );
        assert_eq!(
            (
                [m.bounds.min.x, m.bounds.min.y, m.bounds.min.z],
                [m.bounds.max.x, m.bounds.max.y, m.bounds.max.z]
            ),
            (pin.min, pin.max),
            "{id}: mesh bounds moved"
        );
        assert_eq!(
            fnv(
                m.foliage
                    .instances
                    .matrices
                    .iter()
                    .flatten()
                    .flat_map(|v| v.to_le_bytes())
            ),
            pin.placement,
            "{id}: leaf placement moved"
        );
        let e = &m.foliage.element;
        assert_eq!(
            fnv(e
                .positions
                .iter()
                .flat_map(|p| [p.x, p.y, p.z])
                .flat_map(f64::to_le_bytes)
                .chain(e.indices.iter().flat_map(|i| i.to_le_bytes()))),
            pin.element,
            "{id}: the element itself moved"
        );
    }
}
