//! The tree the core generates, pinned. The scaffold literals were re-recorded
//! when fn-24 rebuilt the scaffold from the habit trait table, and the element
//! hashes and the oak's bounds again when fn-24 rebuilt the element from the
//! outline traits; before that they were the tree as it stood before the
//! foliage element gained its levels. The
//! pins are a photograph, not a description: if a level ever moves a leaf,
//! merges one or drops one, these numbers move too.
//! fn-11 re-pins skeleton/placement bytes once for pure-Rust libm parity.
//! Counts, bounds and element pins are unchanged.
//! fn-27 permutes wood vertices and indices into descending run radius. These
//! pins do not hash wood order: their existing counts, bounds, placement and
//! element identities stay unchanged, so no literal needs re-pinning.
//! fn-30 re-pins once for calibrated growth at derived maturity (seed 7).
//! REPORT.md records convergence before this move: oak nodes 139040 -> 196901,
//! crossover 3602 -> 5394; spruce 90439 -> 76386, crossover 17573 -> 21442.
//! Annual frontier visits change wood and living-shoot placements; element
//! hashes stay fixed. This test now hashes the production grown skeleton too.
//! Node bounds and the resulting mesh bounds/counts are in .flow/evidence/fn30/REPORT.md.
//! fn-31 re-pins once after CONVERGENCE.md records the final growth populations:
//! oak 196901 -> 19513 nodes, spruce 76386 -> 65604. Establishment, retained
//! structural buds and age-dependent pipe scale change the wood and contacts;
//! both element hashes remain unchanged. The report records mesh bounds/counts.
//! No device is needed; this is the core's own arithmetic.
use telperion_core::{
    branching::Specimen,
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
        wood_vertices: 717466,
        wood_triangles: 1375440,
        instances: 132875,
        min: [
            -12.141538173961731,
            -0.32723867893218994,
            -12.581686986441456,
        ],
        max: [13.143301523266894, 24.020887122881813, 13.116854407483443],
        skeleton: 17147586106779270108,
        placement: 3545768933671885177,
        element: 4207404028969543471,
    },
    Pin {
        id: "norway-spruce",
        wood_vertices: 2186602,
        wood_triangles: 4214200,
        instances: 4810878,
        min: [-4.065886488186079, -0.2574246823787689, -3.9355183191281506],
        max: [3.846896851279939, 15.0, 3.757709161827148],
        skeleton: 16388404269218903040,
        placement: 11831058783261786492,
        element: 7287062639823569932,
    },
];

#[test]
fn oak_and_spruce_meshes_pin_the_production_tree_at_derived_maturity() {
    for pin in &PINS {
        let id = pin.id;
        let mut family = Preset::from_id(id).expect("preset identity").parameters();
        family.skeleton.seed = SEED;
        let tree = Specimen::build(&family)
            .unwrap_or_else(|e| panic!("{id}: {e}"))
            .read()
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
            fnv(m
                .foliage
                .instances
                .matrices
                .iter()
                .flatten()
                .flat_map(|v| v.to_le_bytes())),
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
