//! fn-192 R2 and R4: walking any setting at a fixed seed changes the tree
//! by degree. Every continuous setting of a three-PA tree that uses them
//! all is walked over its range in `STEPS` steps at three seeds, on its own
//! scale: log-odds for a probability, its unit otherwise. `WALK=<substring>`
//! walks only the settings whose name holds it (`walk::degree::named`).
mod walk;
use std::sync::OnceLock;
use walk::degree::{
    bound, by_degree, judge, named, vanishing, walks, Walks, MULTIPLE, REFINE, ROUGH_LAYOUT,
    TICK_RESOLUTION,
};
use walk::{
    crossing, in_leaf, light_settings, made, refine, release_settings, setting, settings, spread,
    Setting, STEPS,
};

/// Every setting with its walks, made once: two tests read the same walks,
/// and a second making of them was a third of this binary's time.
fn every_walk() -> &'static (Vec<Setting>, Walks) {
    static ALL: OnceLock<(Vec<Setting>, Walks)> = OnceLock::new();
    ALL.get_or_init(|| {
        let all = named(settings());
        let walked = walks(&all);
        (all, walked)
    })
}

#[test]
fn every_setting_changes_the_tree_by_degree() {
    let (all, walked) = every_walk();
    let failed = judge(all, walked);
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
    let failed: Vec<String> = by_degree(all)
        .into_iter()
        .filter(|f| f.contains("a jump"))
        .collect();
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-202 R3: a sleeping bud's release law walked moves its waking, and
/// with it the tree, by degree.
#[test]
fn the_release_law_changes_the_tree_by_degree() {
    let failed = by_degree(release_settings());
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-197 step 3: light's settings walked change the tree by degree: shade
/// raising an apex's death hazard and shortening its units, its leaf area,
/// and the site's extinction and sky.
#[test]
fn light_changes_the_tree_by_degree() {
    let failed = by_degree(light_settings());
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-226: the rough layout's exception names two light walks that exist,
/// and every other light walk stays at the stated bound.
#[test]
fn only_the_named_light_walks_take_their_own_bound() {
    let names: Vec<String> = light_settings().into_iter().map(|s| s.name).collect();
    for (walk, _) in ROUGH_LAYOUT {
        assert!(names.iter().any(|n| n == walk), "{walk} is walked");
    }
    let wider = names.iter().filter(|n| bound(n) != MULTIPLE).count();
    assert_eq!(wider, ROUGH_LAYOUT.len());
}

/// fn-231: the tick-resolution exception names lifespan walks that exist.
#[test]
fn only_the_named_lifespan_walks_take_the_tick_bound() {
    let names: Vec<String> = walk::settings().into_iter().map(|s| s.name).collect();
    for (walk, _) in TICK_RESOLUTION {
        assert!(names.iter().any(|n| n == walk), "{walk} is walked");
    }
    let wider = names
        .iter()
        .filter(|n| vanishing(n) != walk::degree::VANISHING);
    assert_eq!(wider.count(), TICK_RESOLUTION.len());
}

/// fn-197 decision 12: a sample of structural settings walked in leaf,
/// with light shading, changes the tree by degree: a limb's sag, a limb's yearly survival (a lifespan is
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
    let failed = by_degree(in_leaf(sample));
    assert!(failed.is_empty(), "{failed:#?}");
}

/// fn-197 decision 8: every setting walked in leaf, with light shading,
/// changes the tree by degree. A
/// slow suite outside the gate (host decision 12): it ran past 50 minutes
/// on 2026-10-05. Run it with
/// `cargo test --profile ci -p telperion-space --test walks -- --ignored every_setting_in_leaf`.
#[test]
#[ignore = "slow suite, outside the gate: see its doc comment"]
fn every_setting_in_leaf_changes_the_tree_by_degree() {
    let failed = by_degree(in_leaf(settings()));
    assert!(failed.is_empty(), "{failed:#?}");
}

/// R2: a branch a setting makes enters at vanishing size, and one it
/// unmakes (death, abortion, a lost bud, shedding) has faded to nothing
/// when it goes: bisected to the crossing, its wood is none, a lifespan's
/// at one tick's resolution (`TICK_RESOLUTION`).
#[test]
fn branches_are_made_and_unmade_at_vanishing_size() {
    let (all, walked) = every_walk();
    // Each walk's step that made the most wood, bisected to its crossing,
    // every crossing at once.
    let mut made_at = Vec::new();
    for (setting, seeds) in all.iter().zip(walked) {
        for (seed, steps) in seeds {
            let Some(step) = steps.iter().max_by(|a, b| a.made.total_cmp(&b.made)) else {
                continue;
            };
            if step.made > 0.0 {
                made_at.push((setting, *seed, step));
            }
        }
    }
    let woods = spread(made_at.len(), |k| {
        let (setting, seed, step) = made_at[k];
        let (a, b) = (setting.shape(step.from, seed), setting.shape(step.to, seed));
        let (_, lineage) = made(&a, &b);
        crossing(setting, seed, step.from, step.to, lineage.unwrap(), 48)
    });
    for (&(setting, seed, step), wood) in made_at.iter().zip(woods) {
        assert!(
            wood < vanishing(&setting.name),
            "{} seed {seed} at {:.4}: {wood} of the tree",
            setting.name,
            step.to
        );
    }
    let crossed = made_at.len();
    let filtered = std::env::var_os("WALK").is_some();
    assert!(
        filtered || crossed > 60,
        "{crossed} walks made or unmade branches"
    );
}

/// The jump check sees a switch: a lifespan is a whole number of growth
/// units, so stepping it is a jump however finely the step is split.
#[test]
fn the_jump_check_sees_a_switch() {
    let switch = walk::setting("states[1].lifespan".into(), 0.0, 1.0, false, |s, v| {
        s.states[1].lifespan = if v < 0.5 { 5.0 } else { 6.0 }
    });
    let refined = refine(&switch, 1, 0.5 - 1.0 / f64::from(STEPS), 0.5, 3, 8);
    let changes = &refined.changes;
    assert!(changes[3] > changes[0] / 2.0, "{changes:?}");
    // At the walks' own depth the refinement ends within f32 places of the
    // switch, and the floor, taken outside the last sub-step, still does
    // not hide it (fn-254 review).
    let deep = refine(&switch, 1, 0.5 - 1.0 / f64::from(STEPS), 0.5, REFINE, 8);
    let (jumps, floor) = deep.jumps(&switch, 1, 20.0);
    assert!(
        jumps,
        "a switch stands above the noise floor: {:?}, {floor:?}",
        deep.changes
    );
}

/// A stem that all but stopped grows on as itself and as the relay it
/// would have had (host decision 11), so raising its abortion only ever
/// takes the stem's wood away, never dips it and gives it back: a stop
/// near its bound fades nothing its relay carries on.
#[test]
fn a_stop_near_its_bound_fades_nothing_its_relay_carries() {
    // Every seed's every tree at once.
    let all = spread(4 * 201, |n| {
        let (seed, k) = (n as u64 / 201 + 1, n as u32 % 201);
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
            .sum::<f64>()
    });
    for (seed, lengths) in (1..=4).zip(all.chunks(201)) {
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

/// A twig's shedding delay between whole cycles, walked in leaf on
/// sagging limbs: a delay crossing a whole cycle moves the tree by
/// degree (Codex on fn-206's port).
#[test]
fn the_shedding_delay_changes_the_tree_by_degree_in_leaf() {
    let shedding = setting("states[2].shedding".into(), 0.0, 3.0, false, |s, v| {
        s.states[1].form.sag = 3e-5;
        s.states[2].shedding = v
    });
    let failed = by_degree(in_leaf(vec![shedding]));
    assert!(failed.is_empty(), "{failed:#?}");
}
