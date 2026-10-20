//! fn-301 R1: what each shipped preset draws below its ground, the plane
//! y = 0 through the root. For every catalogue preset at ages 6, 8, 10, 12,
//! 15, 20 and its own, at seeds 1, 7 and 42, it reads the lowest drawn point
//! of the wood, of the leaves and of the flowers, and how many leaf and
//! flower instances have a vertex below the ground.
//!
//! The wood is read from its curve: each ring's lowest point, its radius
//! (lobes included) times the vertical reach of the ring's plane, or a
//! shaped run's extent. The trunk's foot is not counted: its buried ring
//! (`flareDepth`, sunk below the root by design) and its ring at the root,
//! which stands on the ground and leans with the trunk. Leaves and flowers are read at
//! every element vertex under each stored transform.
//!
//! Red on the base for the date palm (fn-283 measured it); ignored until
//! contact lands, and run by name: `cargo test -p telperion-core --test
//! ground_reach -- --ignored --nocapture`.
use telperion_core::{
    mesh::{self, Foliage},
    presets::{self, CATALOGUE},
    surface::Curve,
};

const AGES: [f64; 6] = [6., 8., 10., 12., 15., 20.];
const SEEDS: [u32; 3] = [1, 7, 42];
/// What "below" admits: a leaf transform is quantised against the crown's
/// box, so a vertex laid on the ground may read a hair under it.
const BELOW: f64 = -1e-4;

/// The lowest point of the drawn wood, its buried foot left out.
fn wood_low(curve: &Curve) -> f64 {
    let mut low = f64::INFINITY;
    for run in &curve.runs {
        let points = &curve.points[run.first as usize..(run.first + run.count) as usize];
        for p in points {
            if run.trunk && p.centre.y <= 0. {
                continue;
            }
            let reach = match run.section {
                Some(_) => p.radius,
                None => {
                    let plane = (p.normal.y * p.normal.y + p.binormal.y * p.binormal.y).sqrt();
                    p.radius * (1. + curve.lobe_depth) * plane
                }
            };
            low = low.min(p.centre.y - reach);
        }
    }
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
                let wood = wood_low(&m.curve);
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
