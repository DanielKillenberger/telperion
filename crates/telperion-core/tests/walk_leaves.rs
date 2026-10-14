//! fn-277: a walk between leaf forms changes the leaves by degree. The
//! film (fn-272 FILM.md, defects 2 and 5) saw the oak to palm walk at 40
//! drop from 871,168 leaves to 4 in one frame, and the spruce to birch and
//! birch to oak walks step 25 to 61 % between frames, each where a count
//! of organs rounded across a whole number (CAUSE.md). Each test walks
//! those places at fine steps of `t`, at seeds 1, 7 and 42, built as the
//! film builds a frame (`blend::families`, then `mesh::build`), and holds
//! every pair of consecutive frames to the oracle (R3).
use telperion_core::{blend, mesh, presets::Preset, Family};
use Preset::{DatePalm, NorwaySpruce, OregonWhiteOak, SilverBirch};

const SEEDS: [u32; 3] = [1, 7, 42];
const AGE: f64 = 40.0;
/// The oracle's bound: the most two consecutive frames' leaf counts may
/// differ by, as a ratio. Measured (BOUND.md) on walks whose leaf forms do
/// not change, at steps of 0.001: the largest ratio there, widened.
const STEP: f64 = 1.2;

fn end(preset: Preset, seed: u32) -> Family {
    let mut f = preset.parameters();
    (f.age, f.skeleton.seed) = (AGE, seed);
    f
}

/// The oracle (R3): every pair of consecutive frames from `t0` to `t1` in
/// `n` steps whose leaf counts differ by more than `STEP`, at each seed.
/// Two frames that bear no leaf are dormant.
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

/// The crown rises and the wood's leaves go by degree: fronds and
/// leaflets from none, the cauline share from all.
#[test]
fn oak_to_palm_keeps_its_leaves_by_degree() {
    let walk = (OregonWhiteOak, DatePalm);
    let mut found = jumps(walk, (0.0, 0.006), 6);
    found.extend(jumps(walk, (0.009, 0.013), 4));
    assert!(found.is_empty(), "{found:#?}");
}

/// The birch's short shoots come in by degree where its cluster count
/// once rounded: 0 to 1 at a sixth, 1 to 2 at a half.
#[test]
fn spruce_to_birch_changes_its_leaves_by_degree() {
    let walk = (NorwaySpruce, SilverBirch);
    let mut found = jumps(walk, (0.162, 0.172), 5);
    found.extend(jumps(walk, (0.495, 0.505), 5));
    assert!(found.is_empty(), "{found:#?}");
}

/// And go by degree: 3 to 2 at a sixth, 1 to 0 at five sixths.
#[test]
fn birch_to_oak_changes_its_leaves_by_degree() {
    let walk = (SilverBirch, OregonWhiteOak);
    let mut found = jumps(walk, (0.162, 0.172), 5);
    found.extend(jumps(walk, (0.828, 0.838), 5));
    assert!(found.is_empty(), "{found:#?}");
}
