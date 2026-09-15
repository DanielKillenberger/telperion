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
//! fn-34 adds european-beech and silver-birch pins from the first seed-7
//! photograph of those value tables; re-pinned once on 2026-09-14 after the
//! round-3 tuning against the matched reference pairs (fn-36).
//! fn-37 re-pins the silver birch once more, and only the silver birch: its
//! table is the first to state the curtain as rows - a full hang, a three and
//! a half metre pendulous run, nine degrees between neighbouring shoots - over
//! an envelope with almost no bare trunk under it and a shell that keeps less
//! than half its depth. The oak, the
//! spruce, the beech and the Two Trees are byte-identical: the spruce states
//! the values that reproduce the constants the curtain used to carry, and
//! every other table leaves hang at zero, which reaches nothing.
//! fn-39 re-pins the beech and the birch once more: their tables are the
//! first to state the crown outline's amplitude and wavelength, and the
//! beech's twig is a shade shorter so the lobes' per-seed noise stays under
//! the node ceiling. Every other table leaves the amplitude at zero, which
//! is the smooth shell to the byte; the element pins do not move.
//! fn-34 round 5b re-pins the beech alone, on the owner's verdict that the
//! round-5 beech "bends too much": its leader keeps its dominance, its limbs
//! leave at 38 degrees and rise, and its axes are barely crooked. Round 5c
//! re-pins it again on the owner's next note ("much more thick core trunks
//! for almost the entire height"): half the taper per metre, a lower fork
//! exponent, and the limbs at 42 degrees.
//! fn-45 re-pins the beech alone, once, on the owner's round-5c note that
//! the winter pair has "too many small branches ... Fewer larger branches
//! compared to ours which has many more thinner ones directly attached to
//! the trunk". Its fork exponent rises to 2.8, near Murray's law, so the core
//! keeps its girth up through the crown; its scaffold throws two limbs a
//! station instead of four, a station further apart and longer; and the twig
//! layer states its own depth and drops from five laterals a station to
//! three, which is where the nodes for the thicker core come from; and its
//! leader keeps more of that girth past the first fork, at 0.57 apical
//! dominance. Round 6b then reads the pair against the photograph again: the
//! limbs leave at 32 degrees instead of 42 and every order of them rises, the
//! local shoots follow their limb up at 32 degrees instead of standing off it
//! at 45, the crown's widest point moves down its own depth, the per-metre
//! taper halves again, and every shoot under a twentieth of the trunk's
//! radius carries leaves, which doubles the leaf count without a node.
//! Round 6c reads the leaf-on pair: the crown was a vase of co-dominant
//! stems under a flat umbrella of leaves. The limbs leave at 58 degrees and
//! their side branches are held level, the crown starts at a twentieth of the
//! height and rounds at the top, the fork exponent comes down to 2.6 off the
//! cliff 2.8 sat on, four twig laterals a station fill the crown and a twig a
//! shade shorter pays for them, a local shoot's own shoots are two-ranked,
//! and the leaves lean along their shoots, lie flat and are a tenth larger. The oak, the
//! spruce, the birch and the Two Trees are byte-identical: the generation row
//! sits at the top of its rail on every other table, which reaches nothing.
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
const PINS: [Pin; 4] = [
    Pin {
        id: "oregon-white-oak",
        wood_vertices: 4262170,
        wood_triangles: 8255000,
        instances: 869310,
        min: [
            -13.163122928115051,
            -0.09600000083446503,
            -13.242490423042556,
        ],
        max: [13.217684715842124, 23.557227415847606, 13.003187181590542],
        skeleton: 14986275773972546726,
        placement: 15624359871475047912,
        element: 4207404028969543471,
    },
    Pin {
        id: "norway-spruce",
        wood_vertices: 2888144,
        wood_triangles: 5580040,
        instances: 7012326,
        min: [-3.89500647744516, -0.05999999865889549, -4.197530933827597],
        max: [4.337495164451377, 15.0, 3.8062214356137005],
        skeleton: 12735573889651776723,
        placement: 8171270653015517335,
        element: 7287062639823569932,
    },
    Pin {
        id: "european-beech",
        wood_vertices: 5251044,
        wood_triangles: 10176480,
        instances: 2106977,
        min: [
            -16.327082392258934,
            -0.12800000607967377,
            -15.46583579437701,
        ],
        max: [16.9698603285396, 31.71231641260803, 15.460026573076002],
        skeleton: 7890291168547906776,
        placement: 15585212279580184080,
        element: 15097586524950800877,
    },
    Pin {
        id: "silver-birch",
        wood_vertices: 1616192,
        wood_triangles: 3132920,
        instances: 761413,
        min: [
            -6.8248784154548785,
            -0.07199999690055847,
            -6.5026954825750005,
        ],
        max: [6.549354711566223, 14.489135589258993, 6.328546476364245],
        skeleton: 3822858689754929182,
        placement: 18296506987449200033,
        element: 1872173242819532549,
    },
];

#[test]
fn shipped_species_meshes_are_the_tree_recorded_before_the_levels() {
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

/// Prints every pin field for the two fn-34 species, to re-pin after a
/// value change: `cargo test --release --test identity -- --ignored --nocapture print_pins`.
#[test]
#[ignore]
fn print_pins() {
    for id in ["european-beech", "silver-birch"] {
        let mut family = Preset::from_id(id).unwrap().parameters();
        family.skeleton.seed = SEED;
        let tree = branching::generate(&family.skeleton, family.radii)
            .unwrap()
            .tree;
        let skeleton = fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
                .chain(n.parent.expect("non-root parent").to_le_bytes())
        }));
        let m = mesh::build(&family, Detail::Full).unwrap();
        let placement = fnv(m
            .foliage
            .instances
            .matrices
            .iter()
            .flatten()
            .flat_map(|v| v.to_le_bytes()));
        let e = &m.foliage.element;
        let element = fnv(e
            .positions
            .iter()
            .flat_map(|p| [p.x, p.y, p.z])
            .flat_map(f64::to_le_bytes)
            .chain(e.indices.iter().flat_map(|i| i.to_le_bytes())));
        println!(
            "PIN {id} {} {} {} {:?} {:?} {skeleton} {placement} {element}",
            m.wood_vertices(),
            m.wood_triangles(),
            m.foliage_instances(),
            [m.bounds.min.x, m.bounds.min.y, m.bounds.min.z],
            [m.bounds.max.x, m.bounds.max.y, m.bounds.max.z]
        );
    }
}
