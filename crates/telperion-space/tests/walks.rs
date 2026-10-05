//! fn-192 R2 and R4: walking any setting at a fixed seed changes the tree
//! by degree. Every continuous setting of a three-PA tree that uses them
//! all is walked over its range in `STEPS` steps at three seeds, on its own
//! scale: log-odds for a probability, its unit otherwise.
mod walk;
use walk::{
    crossing, in_leaf, light_settings, made, refine, release_settings, settings, Setting, Step,
    STEPS,
};

/// The stated multiple: no step moves the total length, height, spread,
/// any branch's base or tip, or the wood made or unmade, by more than this
/// share of the tree per unit of the setting's scale. A draw grows in over
/// a log-odds window in proportion to the wood it decides, so no crossing
/// moves the tree faster than about its length per `SPAN` log-odds; linear
/// settings move it in proportion. The bound stands at the first round's 30.
const MULTIPLE: f64 = 30.0;
const SEEDS: [u64; 3] = [1, 2, 3];
/// The levels a walk's steepest steps are split into eighths.
const REFINE: u32 = 6;

/// Every walk's steps at every seed, the walks in parallel.
fn walks(all: &[Setting]) -> Vec<Vec<(u64, Vec<Step>)>> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = all
            .iter()
            .map(|s| {
                scope.spawn(move || {
                    SEEDS
                        .iter()
                        .map(|&seed| (seed, walk::walk(s, seed)))
                        .collect()
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

#[test]
fn every_setting_changes_the_tree_by_degree() {
    let failed = by_degree(&settings());
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-202 R3: a sleeping bud's release law walked moves its waking, and
/// with it the tree, by degree.
#[test]
fn the_release_law_changes_the_tree_by_degree() {
    let failed = by_degree(&release_settings());
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-197 step 3: light's settings walked change the tree by degree: shade
/// raising an apex's death hazard and shortening its units, its leaf area,
/// and the site's extinction and sky.
#[test]
fn light_changes_the_tree_by_degree() {
    let failed = by_degree(&light_settings());
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-197 decision 8: every setting walked in leaf, with light shading
/// and the full lay grown with the tree, changes the tree by degree.
/// Ignored: it did not finish in 50 minutes on 2026-10-05 (FRICTION.md);
/// the host decides its scope before it joins the gate.
#[test]
#[ignore = "runs past 50 minutes; scope pending the host (fn-197 FRICTION.md)"]
fn every_setting_in_leaf_changes_the_tree_by_degree() {
    let failed = by_degree(&in_leaf(settings()));
    assert!(failed.is_empty(), "{failed:#?}");
}

/// Each walk's steepest step against the bound, and its three steepest
/// split finer: what fails.
fn by_degree(all: &[Setting]) -> Vec<String> {
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

/// R2: a branch a setting makes enters at vanishing size, and one it
/// unmakes (death, abortion, a lost bud, shedding) has faded to nothing
/// when it goes: bisected to the crossing, its wood is none.
#[test]
fn branches_are_made_and_unmade_at_vanishing_size() {
    let all = settings();
    let mut crossed = 0;
    for (setting, seeds) in all.iter().zip(walks(&all)) {
        for (seed, steps) in &seeds {
            let Some(step) = steps.iter().max_by(|a, b| a.made.total_cmp(&b.made)) else {
                continue;
            };
            if step.made == 0.0 {
                continue;
            }
            let (a, b) = (
                setting.shape(step.from, *seed),
                setting.shape(step.to, *seed),
            );
            let (_, lineage) = made(&a, &b);
            let wood = crossing(setting, *seed, step.from, step.to, lineage.unwrap(), 48);
            assert!(
                wood < 1e-6,
                "{} seed {seed} at {:.4}: {wood} of the tree",
                setting.name,
                step.to
            );
            crossed += 1;
        }
    }
    assert!(crossed > 60, "{crossed} walks made or unmade branches");
}

/// The jump check sees a switch: a lifespan is a whole number of growth
/// units, so stepping it is a jump however finely the step is split.
#[test]
fn the_jump_check_sees_a_switch() {
    let switch = walk::setting("states[1].lifespan".into(), 0.0, 1.0, false, |s, v| {
        s.states[1].lifespan = if v < 0.5 { 5 } else { 6 }
    });
    let changes = refine(&switch, 1, 0.5 - 1.0 / f64::from(STEPS), 0.5, 3, 8);
    assert!(changes[3] > changes[0] / 2.0, "{changes:?}");
}
