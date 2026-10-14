//! fn-277 R1: a walk between leaf forms changes the leaves by degree. The
//! film (fn-272 FILM.md, defects 2 and 5) saw the oak to palm walk at 40
//! drop from 871,168 leaves to 4 in one frame, and the spruce to birch and
//! birch to oak walks step 25 to 61 % between frames. Each test walks one
//! such jump at fine steps of `t`, at seeds 1, 7 and 42, built as the film
//! builds a frame (`blend::families`, then `mesh::build`).
//!
//! `STEP` is provisional: the largest ratio between consecutive frames seen
//! off the jumps in these windows is 1.10 (oak to palm, 0.002 to 0.003);
//! R3 replaces it with the bound measured on the walks that pass.
use telperion_core::{blend, mesh, presets::Preset, Family};
use Preset::{DatePalm, NorwaySpruce, OregonWhiteOak, SilverBirch};

const SEEDS: [u32; 3] = [1, 7, 42];
const AGE: f64 = 40.0;
const STEP: f64 = 1.2;

fn end(preset: Preset, seed: u32) -> Family {
    let mut f = preset.parameters();
    (f.age, f.skeleton.seed) = (AGE, seed);
    f
}

/// Every pair of consecutive frames from `t0` to `t1` in `n` steps whose
/// leaf counts differ by more than `STEP`, at each seed.
fn jumps(walk: (Preset, Preset), (t0, t1): (f64, f64), n: usize) -> Vec<String> {
    let mut out = Vec::new();
    for seed in SEEDS {
        let (a, b) = (end(walk.0, seed), end(walk.1, seed));
        let leaves = |t: f64| {
            let f = blend::families(&a, &b, t).unwrap();
            mesh::build(&f).unwrap().foliage_instances() as f64
        };
        let ts: Vec<f64> = (0..=n).map(|i| t0 + (t1 - t0) * i as f64 / n as f64).collect();
        let counts: Vec<f64> = ts.iter().map(|&t| leaves(t)).collect();
        for i in 1..counts.len() {
            let (x, y) = (counts[i - 1], counts[i]);
            if x.max(y) > STEP * x.min(y) {
                out.push(format!("{walk:?} seed {seed}: {x} at {:.4}, {y} at {:.4}", ts[i - 1], ts[i]));
            }
        }
    }
    out
}

#[test]
fn oak_to_palm_keeps_its_leaves_by_degree() {
    let walk = (OregonWhiteOak, DatePalm);
    let mut found = jumps(walk, (0.003, 0.006), 3);
    found.extend(jumps(walk, (0.009, 0.013), 4));
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn spruce_to_birch_changes_its_leaves_by_degree() {
    let found = jumps((NorwaySpruce, SilverBirch), (0.162, 0.172), 5);
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn birch_to_oak_changes_its_leaves_by_degree() {
    let found = jumps((SilverBirch, OregonWhiteOak), (0.828, 0.838), 5);
    assert!(found.is_empty(), "{found:#?}");
}
