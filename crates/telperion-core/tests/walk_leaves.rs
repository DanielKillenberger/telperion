//! fn-277: a walk between leaf forms changes the leaves by degree. The
//! film (fn-272 FILM.md, defects 2 and 5) saw the oak to palm walk at 40
//! drop from 871,168 leaves to 4 in one frame, and the spruce to birch and
//! birch to oak walks step 25 to 61 % between frames, each where a count
//! of organs rounded across a whole number (CAUSE.md). Each test walks
//! those places at fine steps of `t`, at seeds 1, 7 and 42, built as the
//! film builds a frame (`blend::families`, then the pipeline), and holds
//! every pair of consecutive frames to the oracle (R3).
//!
//! The oracle covers leaf form, each form per its bearer (host,
//! 2026-10-14): the wood's leaves per metre of leaf-bearing wood, the short
//! shoots' per tip, the crown's per stem apex, so a step in the wood itself
//! (fn-273) does not count against it. A form both frames bear none of is
//! dormant. `WALK_FORMS=<from>,<to>,<age>,<seed>,<file of shares>` runs
//! the ignored `a_listed_walk_keeps_its_forms` on any walk's frames.
use std::collections::BTreeMap;
use telperion_core::{blend, pipeline::executor, presets::by_identity, presets::Preset, Family};
use Preset::{DatePalm, NorwaySpruce, OregonWhiteOak, SilverBirch};

const SEEDS: [u32; 3] = [1, 7, 42];
const AGE: f64 = 40.0;
/// The oracle's bound: the most a form's leaves per bearer may differ by
/// between consecutive frames, as a ratio. BOUND.md states how it was
/// measured.
const STEP: f64 = 1.2;
const FORMS: [&str; 3] = ["wood", "short shoots", "crown"];

fn end(preset: Preset, seed: u32) -> Family {
    let mut f = preset.parameters();
    (f.age, f.skeleton.seed) = (AGE, seed);
    f
}

/// A frame's leaves by form, each per its bearer: the wood's per metre of
/// leaf-bearing wood, the short shoots' per tip, the crown's per stem
/// apex; none where the frame bears none of that form.
fn forms(f: &Family) -> [Option<f64>; 3] {
    let x = executor::grow(f).unwrap().expansion().unwrap();
    let total = x.mesh().unwrap().foliage.instances.leaves.len() as f64;
    // The same tree's wood and short shoots, its crown set aside.
    let mut wood = f.clone();
    wood.canopy.rosette_fronds = 0.0;
    let w = executor::expand(x.tree().clone(), &wood).unwrap();
    let segments = w.stations().unwrap().expect("the wood's stations").segments;
    let (mut metres, mut leaves, mut tips, mut held) = (BTreeMap::new(), 0.0, 0.0, 0.0);
    for s in &segments {
        match s.cluster {
            Some(_) => (tips, held) = (tips + 1.0, held + f64::from(s.count)),
            None => {
                metres.insert((s.key, s.along.to_bits()), s.span);
                leaves += f64::from(s.count);
            }
        }
    }
    let length: f64 = metres.values().sum();
    let crown = total - leaves - held;
    let apices = x.tree().stem_apices().len() as f64;
    let per = |n: f64, bearers: f64| (n > 0.0 && bearers > 0.0).then(|| n / bearers);
    [per(leaves, length), per(held, tips), per(crown, apices)]
}

/// The oracle on consecutive frames' forms at shares `ts`.
fn oracle(name: &str, ts: &[f64], frames: &[[Option<f64>; 3]]) -> Vec<String> {
    let mut out = Vec::new();
    for i in 1..frames.len() {
        for (k, form) in FORMS.iter().enumerate() {
            if let (Some(x), Some(y)) = (frames[i - 1][k], frames[i][k]) {
                if x.max(y) > STEP * x.min(y) {
                    out.push(format!("{name}: {form} {x:.4} at {:.4}, {y:.4} at {:.4}", ts[i - 1], ts[i]));
                }
            }
        }
    }
    out
}

/// Every pair of consecutive frames from `t0` to `t1` in `n` steps whose
/// forms the oracle refuses, at each seed.
fn jumps(walk: (Preset, Preset), (t0, t1): (f64, f64), n: usize) -> Vec<String> {
    let mut out = Vec::new();
    for seed in SEEDS {
        let (a, b) = (end(walk.0, seed), end(walk.1, seed));
        let ts: Vec<f64> = (0..=n).map(|i| t0 + (t1 - t0) * i as f64 / n as f64).collect();
        let frames: Vec<_> = ts.iter().map(|&t| forms(&blend::families(&a, &b, t).unwrap())).collect();
        out.extend(oracle(&format!("{walk:?} seed {seed}"), &ts, &frames));
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

/// Any walk's listed frames held to the oracle, each frame's forms
/// printed: the film's walks are checked this way (FILM-WALKS.md).
#[test]
#[ignore = "evidence: WALK_FORMS names the walk and its frames"]
fn a_listed_walk_keeps_its_forms() {
    let spec = std::env::var("WALK_FORMS").expect("WALK_FORMS=<from>,<to>,<age>,<seed>,<file>");
    let [from, to, age, seed, file] = spec.split(',').collect::<Vec<_>>()[..] else {
        panic!("WALK_FORMS=<from>,<to>,<age>,<seed>,<file>");
    };
    let end = |id: &str| {
        let mut f = by_identity(id).unwrap();
        (f.age, f.skeleton.seed) = (age.parse().unwrap(), seed.parse().unwrap());
        f
    };
    let (a, b) = (end(from), end(to));
    let text = std::fs::read_to_string(file).unwrap();
    let ts: Vec<f64> = text.lines().map(|l| l.trim().parse().unwrap()).collect();
    let frames: Vec<_> = ts.iter().map(|&t| forms(&blend::families(&a, &b, t).unwrap())).collect();
    for (t, f) in ts.iter().zip(&frames) {
        println!("{t:.4}\t{}", f.map(|v| v.map_or("-".into(), |v| format!("{v:.4}"))).join("\t"));
    }
    let found = oracle(&format!("{from} to {to}"), &ts, &frames);
    assert!(found.is_empty(), "{found:#?}");
}
