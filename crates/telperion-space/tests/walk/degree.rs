//! The walk tests' verdict: each walk's steepest step against the stated
//! bound, and its steepest steps split finer, which a jump survives.
use super::{refine, spread, steps, Setting, Step, STEPS};

/// The stated multiple: no step moves the total length, height, spread,
/// any branch's base or tip, or the wood made or unmade, by more than this
/// share of the tree per unit of the setting's scale. A draw grows in over
/// a log-odds window in proportion to the wood it decides, so no crossing
/// moves the tree faster than about its length per `SPAN` log-odds; linear
/// settings move it in proportion. The bound stands at the first round's 30.
pub const MULTIPLE: f64 = 30.0;

/// The walks held to their own bound: apical control under light on the
/// trunk and the limbs (fn-226, host decision 2026-10-08). With the re-lay
/// deleted, light is read at the rough layout's places, and their steepest
/// crossing is a light flip 7 to 10 % steeper (31.1 and 32.1): a slope
/// that refinement shrinks, not a jump (R2.md, "The walk's cause"). The
/// re-lay that kept them under 30 cost 15 to 44 % of the lit oak's build
/// (R5-R6.md); the host keeps today's model and bounds these two at 33.
/// The jump check holds them as it holds every walk.
pub const ROUGH_LAYOUT: [(&str, f64); 2] = [
    ("light: states[0].apical_control", 33.0),
    ("light: states[1].apical_control", 33.0),
];

/// The walks whose made branches are held at the crossing to their own
/// share of the tree (fn-231, host decision 2026-10-08): a lifespan's.
/// The crossing is bisected to where a branch is made, and elsewhere its
/// wood there is at most `VANISHING`. A lifespan is read on the tick grid
/// (2^-20 of a cycle), so no bisection reaches below one tick, and the
/// unit a lifespan's fraction cuts lays its next node by a draw whose lead
/// one tick moves by about n 2^-20 over the node window: a node made at the
/// crossing stands at about 1e-4 and its lateral at up to 2.5e-6 of the
/// tree (ENDS.md, "The fourth answer built"). One tick's branch is held at
/// 1e-5; under the crowded nodes before, one tick sized it at 9.5e-7, which
/// `VANISHING` held.
pub const TICK_RESOLUTION: [(&str, f64); 3] = [
    ("states[0].lifespan", 1e-5),
    ("states[1].lifespan", 1e-5),
    ("states[2].lifespan", 1e-5),
];

/// A made branch's share of the tree at its crossing: at most this, but
/// for `TICK_RESOLUTION`.
pub const VANISHING: f64 = 1e-6;

/// The share of the tree a walk's made branch may hold at its crossing.
pub fn vanishing(name: &str) -> f64 {
    TICK_RESOLUTION
        .iter()
        .find(|(walk, _)| *walk == name)
        .map_or(VANISHING, |&(_, bound)| bound)
}

/// The bound a walk's steepest step is held to: `MULTIPLE`, but for the
/// named exceptions.
pub fn bound(name: &str) -> f64 {
    ROUGH_LAYOUT
        .iter()
        .find(|(walk, _)| *walk == name)
        .map_or(MULTIPLE, |&(_, bound)| bound)
}
pub const SEEDS: [u64; 3] = [1, 2, 3];
/// The levels a walk's steepest steps are split into eighths.
pub const REFINE: u32 = 6;

/// `WALK=<substring>` keeps the settings whose name holds it, so a new
/// setting is proven alone before the gate; unset, every setting is kept.
pub fn named(all: Vec<Setting>) -> Vec<Setting> {
    match std::env::var("WALK") {
        Ok(part) => all.into_iter().filter(|s| s.name.contains(&part)).collect(),
        Err(_) => all,
    }
}

/// Every walk's steps at every seed, by setting.
pub type Walks = Vec<Vec<(u64, Vec<Step>)>>;

/// The pieces a walk is cut into, each starting from its own first tree.
const PIECES: u32 = 8;

/// Every walk's steps at every seed, every piece of every walk in
/// parallel.
pub fn walks(all: &[Setting]) -> Walks {
    let per = SEEDS.len() * PIECES as usize;
    let mut pieces = spread(all.len() * per, |k| {
        let (setting, seed, piece) = (
            &all[k / per],
            SEEDS[k % per / PIECES as usize],
            k as u32 % PIECES,
        );
        steps(
            setting,
            seed,
            piece * STEPS / PIECES,
            (piece + 1) * STEPS / PIECES,
        )
    })
    .into_iter();
    all.iter()
        .map(|_| {
            SEEDS
                .iter()
                .map(|&seed| {
                    (
                        seed,
                        pieces.by_ref().take(PIECES as usize).flatten().collect(),
                    )
                })
                .collect()
        })
        .collect()
}

/// Each walk's steepest step against the bound, and its three steepest
/// split finer: what fails. Only the settings `named` keeps are walked.
pub fn by_degree(all: Vec<Setting>) -> Vec<String> {
    let all = named(all);
    judge(&all, &walks(&all))
}

/// `by_degree` on walks already made.
pub fn judge(all: &[Setting], walked: &Walks) -> Vec<String> {
    // The steps refined: each walk's three steepest, split finer, every
    // walk's at once.
    let mut chosen: Vec<(usize, u64, &Step)> = Vec::new();
    for (i, seeds) in walked.iter().enumerate() {
        let mut steps: Vec<(u64, &Step)> = seeds
            .iter()
            .flat_map(|(seed, steps)| steps.iter().map(move |s| (*seed, s)))
            .collect();
        steps.sort_by(|a, b| b.1.slope.total_cmp(&a.1.slope));
        // Split to a 4096th of a step: a draw near 0 or 1 grows its element
        // in over as little as a 2000th of one, where its log-odds lead
        // runs fastest (fn-206, bisected): a slope there, not a jump.
        let steepest = steps.into_iter().take(3).filter(|(_, s)| s.slope > 0.0);
        chosen.extend(steepest.map(|(seed, step)| (i, seed, step)));
    }
    // Under f32 (fn-254) a refinement stops at the setting's own
    // resolution, and a change no more than four times the noise floor
    // there is the arithmetic's, not the model's.
    let refined = spread(chosen.len(), |k| {
        let (i, seed, step) = chosen[k];
        let refined = refine(&all[i], seed, step.from, step.to, REFINE, 8);
        let (jumps, floor) = refined.jumps(&all[i], seed, 20.0);
        (refined.changes, jumps, floor)
    });
    let mut failed = Vec::new();
    for (i, (setting, seeds)) in all.iter().zip(walked).enumerate() {
        let (seed, worst) = seeds
            .iter()
            .flat_map(|(seed, steps)| steps.iter().map(move |s| (*seed, s)))
            .max_by(|a, b| a.1.slope.total_cmp(&b.1.slope))
            .unwrap();
        println!(
            "{:36} {:7.2}  seed {seed} at {:.4}: {}",
            setting.name, worst.slope, worst.to, worst.what
        );
        if worst.slope > bound(&setting.name) {
            failed.push(format!(
                "{}: {:.1} per unit in {}",
                setting.name, worst.slope, worst.what
            ));
        }
        // The steepest steps are slopes, not jumps: split finer they shrink.
        // Six levels of eight (host, 2026-10-05, decision 9): a jump does
        // not shrink under refinement, while a steep crossing does, but
        // three levels left fn-197's sky walk short of its 20-fold shrink
        // at a crossing that went on shrinking to 2e-5.
        let own = chosen.iter().zip(&refined).filter(|((j, ..), _)| *j == i);
        for ((_, seed, step), (changes, jumps, floor)) in own {
            if let Some(floor) = floor {
                println!(
                    "{} seed {seed} at {:.4}: noise floor {floor:.3e} over {} f32 places, changes {changes:?}",
                    setting.name, step.to, super::measure::ULPS
                );
            }
            if *jumps {
                failed.push(format!(
                    "{} seed {seed} at {:.4}: a jump ({changes:?}, floor {floor:?})",
                    setting.name, step.to
                ));
            }
        }
    }
    failed
}
