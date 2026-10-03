//! fn-192 R2 and R4: walking any setting at a fixed seed changes the tree
//! by degree. Every continuous setting of a three-PA tree that uses them
//! all is walked over its range in `STEPS` steps at three seeds, on its own
//! scale: log-odds for a probability, its unit otherwise.
mod walk;
use telperion_space::GROW_IN;
use walk::{crossing, made, refine, settings, Setting, Step, STEPS};

/// The stated multiple: no step moves the total length, height, spread,
/// any branch's base or tip, or the wood made or unmade, by more than this
/// share of the tree per unit of the setting's scale. A draw that decides
/// the whole crown grows it in over `GROW_IN` of a unit; a whole tree over
/// that is 1 / GROW_IN, and the measures of a branch on the crown's top
/// move with it, so three times that.
const MULTIPLE: f64 = 3.0 / GROW_IN;
const SEEDS: [u64; 3] = [1, 2, 3];

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
    let all = settings();
    let mut failed = Vec::new();
    for (setting, seeds) in all.iter().zip(walks(&all)) {
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
        let mut steps: Vec<(u64, &Step)> = seeds
            .iter()
            .flat_map(|(seed, steps)| steps.iter().map(move |s| (*seed, s)))
            .collect();
        steps.sort_by(|a, b| b.1.slope.total_cmp(&a.1.slope));
        for (seed, step) in steps.iter().take(3).filter(|(_, s)| s.slope > 0.0) {
            let changes = refine(setting, *seed, step.from, step.to, 3, 8);
            let (first, last) = (changes[0], changes[3]);
            if last > first / 20.0 {
                failed.push(format!(
                    "{} seed {seed} at {:.4}: a jump ({changes:?})",
                    setting.name, step.to
                ));
            }
        }
    }
    assert!(failed.is_empty(), "{failed:#?}");
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
