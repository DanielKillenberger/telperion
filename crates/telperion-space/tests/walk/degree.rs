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
pub const SEEDS: [u64; 3] = [1, 2, 3];
/// The levels a walk's steepest steps are split into eighths.
pub const REFINE: u32 = 6;

/// Every walk's steps at every seed, the walks in parallel.
pub fn walks(all: &[Setting]) -> Vec<Vec<(u64, Vec<Step>)>> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = all
            .iter()
            .map(|s| scope.spawn(move || SEEDS.iter().map(|&seed| (seed, walk(s, seed))).collect()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

/// Each walk's steepest step against the bound, and its three steepest
/// split finer: what fails.
pub fn by_degree(all: &[Setting]) -> Vec<String> {
    let mut failed = Vec::new();
    for (setting, seeds) in all.iter().zip(walks(all)) {
        let (seed, worst) = seeds
            .iter()
            .flat_map(|(seed, steps)| steps.iter().map(move |s| (*seed, s)))
            .max_by(|a, b| a.1.slope.total_cmp(&b.1.slope))
            .unwrap();
        println!(
            "{:36} {:7.2}  seed {seed} at {:.4}: {}",
            setting.name, worst.slope, worst.to, worst.what
        );
        if worst.slope > MULTIPLE {
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
