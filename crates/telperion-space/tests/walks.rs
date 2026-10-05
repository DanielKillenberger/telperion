//! fn-192 R2 and R4: walking any setting at a fixed seed changes the tree
//! by degree. Every continuous setting of a three-PA tree that uses them
//! all is walked over its range in `STEPS` steps at three seeds, on its own
//! scale: log-odds for a probability, its unit otherwise.
mod walk;
use walk::{
    crossing, in_leaf, light_settings, made, refine, release_settings, setting, settings, Setting,
    Step, STEPS,
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

/// fn-206 R3: a walk from one species to the next, every setting between
/// theirs on the shared reference axis, changes the tree by degree: no
/// jump. The share of the way is not a setting, so the bound per unit of a
/// setting does not hold it: the whole tree turns from one species into
/// the next over it, its laterals spun round the stem as the divergence
/// walks (steepest about 300 per unit, in a branch's place). The strips
/// judge the look.
#[test]
fn a_walk_between_species_changes_the_tree_by_degree() {
    use telperion_space::{beech, blend, oak, palm, spruce};
    type Point = fn() -> telperion_space::Species;
    let pairs: [(&str, Point, Point); 3] = [
        ("beech to spruce", beech, spruce),
        ("spruce to oak", spruce, oak),
        ("oak to palm", oak, palm),
    ];
    let all: Vec<Setting> = pairs
        .into_iter()
        .map(|(name, a, b)| {
            setting(name.into(), 0.0, 1.0, false, move |s, t| {
                *s = blend(&a(), &b(), t).unwrap()
            })
        })
        .collect();
    let failed: Vec<String> = by_degree(&all)
        .into_iter()
        .filter(|f| f.contains("a jump"))
        .collect();
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

/// fn-197 decision 12: a sample of structural settings walked in leaf,
/// with light shading and the full lay grown with the tree, changes the
/// tree by degree: a limb's sag, a limb's yearly survival (a lifespan is
/// a whole number of units, never walked) and a sleeping-bud probability.
#[test]
fn a_sample_of_settings_in_leaf_changes_the_tree_by_degree() {
    let names = [
        "states[1].form.sag",
        "states[1].viability",
        "sleeping: states[1].zones[0].dormant[2]",
    ];
    let sample: Vec<Setting> = settings()
        .into_iter()
        .filter(|s| names.contains(&s.name.as_str()))
        .collect();
    assert_eq!(sample.len(), names.len());
    let failed = by_degree(&in_leaf(sample));
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-197 decision 8: every setting walked in leaf, with light shading
/// and the full lay grown with the tree, changes the tree by degree. A
/// slow suite outside the gate (host decision 12): it ran past 50 minutes
/// on 2026-10-05. Run it with
/// `cargo test --profile ci -p telperion-space --test walks -- --ignored every_setting_in_leaf`.
#[test]
#[ignore = "slow suite, outside the gate: see its doc comment"]
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
        s.states[1].lifespan = if v < 0.5 { 5.0 } else { 6.0 }
    });
    let changes = refine(&switch, 1, 0.5 - 1.0 / f64::from(STEPS), 0.5, 3, 8);
    assert!(changes[3] > changes[0] / 2.0, "{changes:?}");
}

/// A stem that all but stopped grows on as itself and as the relay it
/// would have had (host decision 11), so raising its abortion only ever
/// takes the stem's wood away, never dips it and gives it back: a stop
/// near its bound fades nothing its relay carries on.
#[test]
fn a_stop_near_its_bound_fades_nothing_its_relay_carries() {
    for seed in 1..=4 {
        let lengths: Vec<f64> = (0..=200)
            .map(|k| {
                let mut species = walk::species();
                let stem = &mut species.states[0];
                stem.abortion = 0.1 + 0.3 * f64::from(k) / 200.0;
                stem.relay = 0.5;
                let tree = walk::tree(&species, seed).unwrap();
                tree.axes
                    .iter()
                    .filter(|a| a.pa == 0)
                    .flat_map(|a| &a.phytomers)
                    .map(|p| p.scale)
                    .sum()
            })
            .collect();
        for i in 1..lengths.len() - 1 {
            let rim = |side: &[f64]| side.iter().copied().fold(0.0, f64::max);
            let rim = rim(&lengths[..i]).min(rim(&lengths[i + 1..]));
            assert!(
                lengths[i] >= rim * (1.0 - 1e-9),
                "seed {seed}: the stem dips to {} between {rim}s",
                lengths[i]
            );
        }
    }
}

/// fn-199 R2: a relay stands at its share of its parent's last growth
/// unit by the unit's scaled length, so a node growing in from nothing
/// beneath it moves it by nothing (fn-206, b172c2d4).
#[test]
fn a_node_growing_in_moves_no_relay() {
    let nodes = walk::setting(
        "states[0].zones[1].nodes.max, relay 1".into(),
        2.0,
        4.0,
        false,
        |s, v| {
            let trunk = &mut s.states[0];
            trunk.relay = 1.0;
            trunk.relay_at = 0.5;
            trunk.abortion = 0.3;
            trunk.zones[1].nodes = telperion_space::NodeLaw::Uniform { min: 2.0, max: v };
        },
    );
    let failed = by_degree(&[nodes]);
    assert!(failed.is_empty(), "{failed:#?}");
}

/// Host decision 12: a stop's outcome is one distribution, carrying on,
/// relaying or dying, mixed as shares. Neither the beech's leader (it
/// aborts and always relays) nor the spruce's (it never aborts) dies, so
/// the tree halfway between keeps its leader to age 60: an apex still
/// growing at whole size at the top of the tree.
#[test]
fn a_midpoint_between_two_undying_leaders_keeps_its_leader() {
    use telperion_space::{beech, blend, grow, spruce, Origin, Request};
    let species = blend(&beech(), &spruce(), 0.5).unwrap();
    let request = Request {
        age: 60,
        seed: 1,
        budget: 60_000_000,
        light: telperion_space::Light::NEUTRAL,
    };
    let tree = grow(&species, request).unwrap();
    // The leader: from the seed, on through the most vigorous of each
    // axis's continuations and relays.
    let mut at = 0;
    loop {
        let next = tree
            .axes
            .iter()
            .enumerate()
            .filter(|(_, a)| match a.origin {
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => parent == at,
                _ => false,
            })
            .max_by(|a, b| a.1.vigour.total_cmp(&b.1.vigour));
        match next {
            Some((j, _)) => at = j,
            None => break,
        }
    }
    let leader = &tree.axes[at];
    let tip = leader.phytomers.last().expect("the leader grew");
    let top = tree
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .map(|p| p.tip.z)
        .fold(0.0, f64::max);
    assert_eq!(leader.apex_end, None, "the leader stopped");
    assert!(tip.scale > 0.99, "the leader faded to {}", tip.scale);
    assert!(
        tip.tip.z > 0.95 * top,
        "the leader at {} of {top}",
        tip.tip.z
    );
}
