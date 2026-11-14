//! fn-277: a walk between leaf forms changes the leaves by degree. The
//! film (fn-272 FILM.md, defects 2 and 5) saw the oak to palm walk at 40
//! drop from 871,168 leaves to 4 in one frame, and the spruce to birch and
//! birch to oak walks step 25 to 61 % between frames, each where a count
//! of organs rounded across a whole number (CAUSE.md). Each test walks
//! those places at fine steps of `t`, built as the film builds a frame
//! (`blend::families`, then the pipeline), and holds every pair of
//! consecutive frames to the oracle (R3). The gate walks seed 1; the
//! ignored `every_walk_changes_its_leaves_by_degree_at_every_seed` walks
//! seeds 1, 7 and 42 (host, fn-304 ruling 1).
//!
//! The oracle tests the draw (host ruling 5b, 2026-10-14; the law is
//! `walk_law.rs`): in each frame, each form's drawn count lies within
//! four sigma of its expectation from the frame's rows, the sampling noise
//! of the keyed draws on the fractional parts, and a form with fewer than
//! 30 organs in the frame is dormant, since whole organs cannot track a
//! fraction there. The wood is counted by station. `WALK_FORMS=<from>,
//! <to>,<age>,<seed>,<file of shares>` runs the ignored
//! `a_listed_walk_draws_its_law` on any walk's frames.
use telperion_core::{blend, pipeline::executor, presets::by_identity, presets::Preset, Family};
use Preset::{DatePalm, NorwaySpruce, OregonWhiteOak, SilverBirch};

const SEEDS: [u32; 3] = [1, 7, 42];
const AGE: f64 = 40.0;
/// The noise a draw may stand off its expectation by, in sigmas, and the
/// fewest organs a form is held at.
const SIGMAS: f64 = 4.0;
const ORGANS: f64 = 30.0;

fn end(preset: Preset, seed: u32) -> Family {
    let mut f = preset.parameters();
    (f.age, f.skeleton.seed) = (AGE, seed);
    f
}

/// One form's draw in a frame: its organs, the count drawn on them, its
/// expectation and the sampling noise about it.
struct Draw {
    form: &'static str,
    organs: f64,
    drawn: f64,
    expected: f64,
    sigma: f64,
}

impl Draw {
    /// How many sigmas the draw stands off its expectation; none where
    /// the form is dormant.
    fn off(&self) -> Option<f64> {
        (self.organs >= ORGANS).then(|| {
            let miss = (self.drawn - self.expected).abs();
            if miss < 1e-6 {
                0.0
            } else {
                miss / self.sigma
            }
        })
    }
}

/// The mean and variance of a whole count drawn either side of `count`.
fn split(count: f64) -> (f64, f64) {
    let more = count - count.floor();
    (count, more * (1.0 - more))
}

/// What one family's wood and short shoots carry on `tree`: the wood's
/// leaves, the short shoots' leaves and the leaves their clusters hold,
/// the runs of wood and the tips that keep theirs.
fn counted(tree: &telperion_core::tree::Tree, f: &Family) -> [f64; 5] {
    let x = executor::expand(tree.clone(), f).unwrap();
    let segments = x.stations().unwrap().expect("the wood's stations").segments;
    let (mut wood, mut short, mut held, mut tips) = (0.0, 0.0, 0.0, 0.0);
    let mut runs = std::collections::BTreeSet::new();
    for s in &segments {
        match s.cluster {
            Some((_, leaves)) => {
                (short, held, tips) = (
                    short + f64::from(s.count),
                    held + f64::from(leaves),
                    tips + 1.0,
                )
            }
            None => {
                wood += f64::from(s.count);
                runs.insert(s.key);
            }
        }
    }
    [wood, short, held, runs.len() as f64, tips]
}

/// Every form's draw in one frame.
fn draws(f: &Family) -> Vec<Draw> {
    let x = executor::grow(f).unwrap().expansion().unwrap();
    let tree = x.tree().clone();
    let total = x.mesh().unwrap().foliage.instances.leaves.len() as f64;
    let c = f.canopy;
    let with = |edit: &dyn Fn(&mut Family)| {
        let mut g = f.clone();
        g.canopy.rosette_fronds = 0.0;
        edit(&mut g);
        g
    };
    let [wood, short, held, runs, tips] = counted(&tree, &with(&|_| ()));
    let [stations, _, _, _, _] = counted(&tree, &with(&|g| g.canopy.leaflet_count = 1.0));
    let [_, _, _, all_runs, all_tips] = counted(
        &tree,
        &with(&|g| (g.canopy.cauline, g.canopy.leaflet_count) = (1.0, 1.0)),
    );
    let mut one = f.clone();
    one.canopy.leaflet_count = 1.0;
    let ones = executor::expand(tree.clone(), &one)
        .unwrap()
        .mesh()
        .unwrap()
        .foliage
        .instances
        .leaves
        .len() as f64;
    let (crown, fronds) = (total - wood - short, ones - stations - held);
    let grouped = c.rachis_length > 0.0 && c.leaflet_count > 1.0;
    let (each, var) = if grouped {
        split(c.leaflet_count)
    } else {
        (1.0, 0.0)
    };
    let leaflets = |form, organs: f64, drawn| Draw {
        form,
        organs,
        drawn,
        expected: organs * each,
        sigma: (organs * var).sqrt(),
    };
    let bearers = all_runs + all_tips;
    let mut out = vec![
        leaflets("leaflets per station", stations, wood),
        leaflets("leaflets per cluster leaf", held, short),
        leaflets("leaflets per frond", fronds, crown),
        Draw {
            form: "cauline runs and tips",
            organs: bearers,
            drawn: runs + tips,
            expected: bearers * c.cauline,
            sigma: (bearers * c.cauline * (1.0 - c.cauline)).sqrt(),
        },
        Draw {
            form: "fronds per apex",
            organs: tree.stem_apices().len() as f64,
            drawn: 0.0,
            expected: 0.0,
            sigma: 0.0,
        },
    ];
    // A short shoot holds its cluster's whole leaves (one where its age
    // states less than one). Where short-shooting ages carry unlike
    // counts, each counts by how much the frame grows in it (fn-308 host
    // ruling 2), and an age it does not grow in is dormant.
    let counts: Vec<(usize, f64)> = f
        .space
        .species
        .iter()
        .flat_map(|s| s.states.iter().enumerate())
        .map(|(k, s)| (k, s.cluster_leaves))
        .filter(|&(_, n)| n > 0.0)
        .collect();
    let held_by = |n: f64| if n >= 1.0 { split(n) } else { (1.0, 0.0) };
    let weighed = match counts.first() {
        Some(&(_, n)) if counts.iter().all(|&(_, m)| m == n) => Some(held_by(n)),
        Some(_) => {
            let s = f.space.species.as_ref().unwrap();
            let grown = telperion_space::expected_counts(s, f.age as u32).unwrap();
            let reached: Vec<_> = (counts.iter())
                .map(|&(k, n)| (grown.total(k), held_by(n)))
                .filter(|&(w, _)| w > 0.0)
                .collect();
            let all: f64 = reached.iter().map(|(w, _)| w).sum();
            (all > 0.0).then(|| {
                reached.iter().fold((0.0, 0.0), |(m, v), (w, (mean, var))| {
                    (m + w / all * mean, v + w / all * var)
                })
            })
        }
        None => None,
    };
    if let Some((mean, var)) = weighed {
        out.push(Draw {
            form: "leaves per short shoot",
            organs: tips,
            drawn: held,
            expected: tips * mean,
            sigma: (tips * var).sqrt(),
        });
    }
    assert!(
        out[4].organs < ORGANS,
        "fronds per apex on {} apices is not held",
        out[4].organs
    );
    out
}

/// The draws the oracle refuses on frames at shares `ts`.
fn refused(name: &str, ts: &[f64], frames: &[Vec<Draw>]) -> Vec<String> {
    let mut out = Vec::new();
    for (t, frame) in ts.iter().zip(frames) {
        for d in frame {
            if d.off().is_some_and(|off| off > SIGMAS) {
                let Draw {
                    form,
                    organs,
                    drawn,
                    expected,
                    sigma,
                } = d;
                out.push(format!("{name} at {t:.4}: {form}, {drawn} drawn on {organs} against {expected:.1} (sigma {sigma:.2})"));
            }
        }
    }
    out
}

/// Every frame from `t0` to `t1` in `n` steps whose draws the oracle
/// refuses, at each of `seeds`.
fn jumps(walk: (Preset, Preset), (t0, t1): (f64, f64), n: usize, seeds: &[u32]) -> Vec<String> {
    let mut out = Vec::new();
    for &seed in seeds {
        let (a, b) = (end(walk.0, seed), end(walk.1, seed));
        let ts: Vec<f64> = (0..=n)
            .map(|i| t0 + (t1 - t0) * i as f64 / n as f64)
            .collect();
        let frames: Vec<_> = ts
            .iter()
            .map(|&t| draws(&blend::families(&a, &b, t).unwrap()))
            .collect();
        out.extend(refused(&format!("{walk:?} seed {seed}"), &ts, &frames));
    }
    out
}

/// The crown rises and the wood's leaves go by degree: fronds and
/// leaflets from none, the cauline share from all.
fn oak_to_palm(seeds: &[u32]) -> Vec<String> {
    let walk = (OregonWhiteOak, DatePalm);
    let mut found = jumps(walk, (0.0, 0.006), 6, seeds);
    found.extend(jumps(walk, (0.009, 0.013), 4, seeds));
    found
}

/// The birch's short shoots come in by degree where its cluster count
/// once rounded: 0 to 1 at a sixth, 1 to 2 at a half.
fn spruce_to_birch(seeds: &[u32]) -> Vec<String> {
    let walk = (NorwaySpruce, SilverBirch);
    let mut found = jumps(walk, (0.162, 0.172), 5, seeds);
    found.extend(jumps(walk, (0.495, 0.505), 5, seeds));
    found
}

/// And go by degree: 3 to 2 at a sixth, 1 to 0 at five sixths.
fn birch_to_oak(seeds: &[u32]) -> Vec<String> {
    let walk = (SilverBirch, OregonWhiteOak);
    let mut found = jumps(walk, (0.162, 0.172), 5, seeds);
    found.extend(jumps(walk, (0.828, 0.838), 5, seeds));
    found
}

#[test]
fn oak_to_palm_keeps_its_leaves_by_degree() {
    let found = oak_to_palm(&[1]);
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn spruce_to_birch_changes_its_leaves_by_degree() {
    let found = spruce_to_birch(&[1]);
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn birch_to_oak_changes_its_leaves_by_degree() {
    let found = birch_to_oak(&[1]);
    assert!(found.is_empty(), "{found:#?}");
}

/// The three walks at every seed:
/// `cargo test --profile ci -p telperion-core --test walk_leaves -- --ignored every_walk`
#[test]
#[ignore = "fn-304: the three walks at seeds 1, 7 and 42, outside the gate; about 7 minutes"]
fn every_walk_changes_its_leaves_by_degree_at_every_seed() {
    let mut found = oak_to_palm(&SEEDS);
    found.extend(spruce_to_birch(&SEEDS));
    found.extend(birch_to_oak(&SEEDS));
    assert!(found.is_empty(), "{found:#?}");
}

/// Any walk's listed frames held to the oracle, each frame's draws
/// printed: the film's walks are checked this way (FILM-WALKS.md).
#[test]
#[ignore = "evidence: WALK_FORMS names the walk and its frames"]
fn a_listed_walk_draws_its_law() {
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
    let frames: Vec<_> = ts
        .iter()
        .map(|&t| draws(&blend::families(&a, &b, t).unwrap()))
        .collect();
    for (t, frame) in ts.iter().zip(&frames) {
        let offs: Vec<String> = frame
            .iter()
            .map(|d| d.off().map_or("-".into(), |o| format!("{o:.2}")))
            .collect();
        println!("{t:.4}\t{}", offs.join("\t"));
    }
    let found = refused(&format!("{from} to {to}"), &ts, &frames);
    assert!(found.is_empty(), "{found:#?}");
}
