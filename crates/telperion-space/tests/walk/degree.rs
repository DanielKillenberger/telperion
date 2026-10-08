//! The walk tests' verdict: each walk's steepest step against the stated
//! bound, and its steepest steps split finer, which a jump survives.
use super::{refine, walk, Setting, Step};

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

/// Every walk's steps at every seed, the walks in parallel.
pub fn walks(all: &[Setting]) -> Walks {
    std::thread::scope(|scope| {
        let handles: Vec<_> = all
            .iter()
            .map(|s| scope.spawn(move || SEEDS.iter().map(|&seed| (seed, walk(s, seed))).collect()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

/// Each walk's steepest step against the bound, and its three steepest
/// split finer: what fails. Only the settings `named` keeps are walked.
pub fn by_degree(all: Vec<Setting>) -> Vec<String> {
    let all = named(all);
    judge(&all, &walks(&all))
}

/// `by_degree` on walks already made.
pub fn judge(all: &[Setting], walked: &Walks) -> Vec<String> {
    let mut failed = Vec::new();
    for (setting, seeds) in all.iter().zip(walked) {
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
        let mut steps: Vec<(u64, &Step)> = seeds
            .iter()
            .flat_map(|(seed, steps)| steps.iter().map(move |s| (*seed, s)))
            .collect();
        steps.sort_by(|a, b| b.1.slope.total_cmp(&a.1.slope));
        // Split to a 4096th of a step: a draw near 0 or 1 grows its element
        // in over as little as a 2000th of one, where its log-odds lead
        // runs fastest (fn-206, bisected): a slope there, not a jump.
        for (seed, step) in steps.iter().take(3).filter(|(_, s)| s.slope > 0.0) {
            let changes = refine(setting, *seed, step.from, step.to, REFINE, 8);
            let (first, last) = (changes[0], changes[REFINE as usize]);
            if last > first / 20.0 {
                failed.push(format!(
                    "{} seed {seed} at {:.4}: a jump ({changes:?})",
                    setting.name, step.to
                ));
            }
        }
    }
    failed
}
