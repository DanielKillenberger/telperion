//! fn-301 R1: what each shipped preset draws below its ground, the plane
//! y = 0 through the root. For every catalogue preset at ages 6, 8, 10, 12,
//! 15, 20 and its own, at seeds 1, 7 and 42, it reads the lowest drawn point
//! of the wood, of the leaves and of the flowers, and how many leaf and
//! flower instances have a vertex below the ground.
//!
//! The wood is read from its drawn vertices, the sweep's own (fn-301 .3):
//! run by run in the curve's order, each ring's vertices and the run's two
//! caps. A trunk's run below the root is its foot, not counted: its buried
//! ring (`flareDepth`, sunk below the root by design) and its rings at the
//! root, which lean with the trunk (the palm's dip to 5 cm, on the base as
//! after). Every other run's vertices are kept off the ground by the
//! surface clearance. Leaves and flowers are read at every element vertex
//! under each stored transform.
//!
//! Red on the base for the date palm (fn-283 measured it); ignored until
//! contact lands, and run by name: `cargo test -p telperion-core --test
//! ground_reach -- --ignored --nocapture`.
use telperion_core::{
    mesh::{self, Foliage, TreeMesh},
    presets::{self, CATALOGUE},
    surface::SurfaceParams,
};

const AGES: [f64; 6] = [6., 8., 10., 12., 15., 20.];
const SEEDS: [u32; 3] = [1, 7, 42];
/// What "below" admits: a leaf transform is quantised against the crown's
/// box, so a vertex laid on the ground may read a hair under it.
const BELOW: f64 = -1e-4;

/// The lowest drawn vertex of the wood, its trunk's foot left out: the
/// mesh's vertices, `segments` a ring and two caps a run, in the curve's
/// run order, a trunk's below the root not counted.
fn wood_low(m: &TreeMesh, params: &SurfaceParams) -> f64 {
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let drawn = m.wood.flattened().positions;
    let (mut low, mut at) = (f64::INFINITY, 0);
    for run in &m.curve.runs {
        let vertices = run.count as usize * segments + 2;
        for v in at..at + vertices {
            let y = f64::from(drawn[3 * v + 1]);
            if !(run.trunk && y < 0.) {
                low = low.min(y);
            }
        }
        at += vertices;
    }
    assert_eq!(3 * at, drawn.len(), "the drawn wood is the curve's runs");
    low
}

/// The lowest vertex of a foliage and how many instances reach below.
fn foliage_low(f: &Foliage) -> (f64, usize) {
    let (mut low, mut under) = (f64::INFINITY, 0);
    for m in f.instances.matrices() {
        let mut least = f64::INFINITY;
        for &p in &f.element.positions {
            least = least.min(telperion_core::foliage::transform_point(&m, p).y);
        }
        low = low.min(least);
        under += usize::from(least < BELOW);
    }
    (low, under)
}

fn show(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.3}")
    } else {
        "-".into()
    }
}

#[test]
#[ignore = "fn-301 R1: red on the base until ground contact lands"]
fn nothing_drawn_passes_below_the_ground() {
    println!("preset\tage\tseed\twood_low\tleaf_low\tleaves_under\tflower_low\tflowers_under");
    let mut failures = Vec::new();
    for &(_, id, _, _) in CATALOGUE {
        let default = presets::by_identity(id).expect("a shipped preset").age;
        for age in AGES.iter().copied().chain([default]) {
            for seed in SEEDS {
                let mut f = presets::by_identity(id).expect("a shipped preset");
                (f.age, f.skeleton.seed) = (age, seed);
                let m = mesh::build(&f).unwrap_or_else(|e| panic!("{id} {age} {seed}: {e}"));
                let wood = wood_low(&m, &f.surface);
                let (leaf, leaves) = foliage_low(&m.foliage);
                let (flower, flowers) = m.organ.as_ref().map_or((f64::INFINITY, 0), foliage_low);
                println!(
                    "{id}\t{age}\t{seed}\t{}\t{}\t{leaves}\t{}\t{flowers}",
                    show(wood),
                    show(leaf),
                    show(flower)
                );
                if wood < BELOW || leaves > 0 || flowers > 0 {
                    failures.push(format!("{id} at {age}, seed {seed}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "below the ground: {failures:?}");
}
