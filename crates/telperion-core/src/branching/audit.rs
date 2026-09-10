//! Reproducible stage audit for retained fidelity counterexamples, and the
//! cross-preset proof that every trait and every field term moves every tree.
use super::*;
use crate::{bias::SupernaturalParams, presets::Preset};

/// Positions, parent links and solved radii: everything the scaffold decides.
fn skeleton_hash(tree: &Tree) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for n in tree.nodes.iter().skip(1) {
        for byte in [n.position.x, n.position.y, n.position.z, n.radius]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .chain(n.parent.unwrap().to_le_bytes())
        {
            hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
        }
    }
    hash
}
fn hashed(family: &crate::presets::Family) -> u64 {
    skeleton_hash(&generate(&family.skeleton, family.radii).unwrap().tree)
}
const PRESETS: [Preset; 3] = [
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
    Preset::Ordinary,
];

#[test]
#[ignore = "mature stage audit; run explicitly with --nocapture"]
fn retained_supports() {
    for (preset, seeds) in [
        (
            Preset::OregonWhiteOak,
            vec![1, 2, 3, 2666899686, 762807349, 1444323199],
        ),
        (
            Preset::NorwaySpruce,
            vec![1, 2, 3, 1982700925, 281313742, 2271779095, 4250668600],
        ),
    ] {
        for seed in seeds {
            let mut f = preset.parameters();
            f.skeleton.seed = seed;
            let p = &f.skeleton;
            let config = p.resolved_growth(0).unwrap();
            let bias = GrowthBias::new(p.envelope, seed, p.bias).unwrap();
            let mut tree = scaffold::generate(p, &config, &bias, &[]).unwrap();
            radius::solve(&mut tree, p.envelope, f.radii).unwrap();
            let structural = tree.clone();
            local::append(&mut tree, &config, p.twigs, seed, Some(&bias), p.habit).unwrap();
            let before = tree.clone();
            let removed = if p.habit.shedding_threshold > 0.0 {
                shed(&mut tree, p.envelope, p.habit.shedding_threshold).unwrap()
            } else {
                0
            };
            let mut descendants = vec![[0usize; 2]; structural.nodes.len()];
            let mut owner = vec![0; before.nodes.len()];
            for (i, entry) in owner.iter_mut().enumerate().take(structural.nodes.len()) {
                *entry = i;
            }
            let positions: std::collections::HashSet<_> = tree
                .nodes
                .iter()
                .map(|n| {
                    (
                        n.position.x.to_bits(),
                        n.position.y.to_bits(),
                        n.position.z.to_bits(),
                    )
                })
                .collect();
            for i in structural.nodes.len()..before.nodes.len() {
                let n = &before.nodes[i];
                owner[i] = owner[n.parent.unwrap() as usize];
                if n.kind == NodeKind::Twig {
                    let live = positions.contains(&(
                        n.position.x.to_bits(),
                        n.position.y.to_bits(),
                        n.position.z.to_bits(),
                    ));
                    descendants[owner[i]][usize::from(live)] += 1;
                }
            }
            let max_y = structural
                .nodes
                .iter()
                .map(|n| n.position.y)
                .fold(0.0, f64::max);
            let supports: Vec<_> = structural.nodes.iter().enumerate().skip(1).filter(|(_,n)| n.position.y > max_y * 0.75).map(|(i,n)| {
                let parent = n.parent.unwrap() as usize;
                serde_json::json!({"node":i,"parent":parent,"position":[n.position.x,n.position.y,n.position.z],"radius":n.radius,"handoff_eligible":n.radius < p.twigs.limb_radius*structural.nodes[0].radius,"lost_twigs":descendants[i][0],"retained_twigs":descendants[i][1]})
            }).collect();
            println!(
                "AUDIT {}",
                serde_json::json!({"preset":preset.profile_id(),"seed":seed,"structural":structural.nodes.len(),"local_before":before.nodes.len()-structural.nodes.len(),"shed":removed,"supports":supports})
            );
            assert!(!tree.diagnostics.node_capped);
        }
    }
}

#[test]
#[ignore = "mature scaffold direction audit"]
fn scaffold_directions() {
    for seed in [1, 2, 3, 2666899686, 762807349, 1444323199] {
        let mut f = Preset::OregonWhiteOak.parameters();
        f.skeleton.seed = seed;
        let p = &f.skeleton;
        let config = p.resolved_growth(0).unwrap();
        let bias = GrowthBias::new(p.envelope, seed, p.bias).unwrap();
        let tree = scaffold::generate(p, &config, &bias, &[]).unwrap();
        let mut children = vec![0; tree.nodes.len()];
        for n in tree.nodes.iter().skip(1) {
            children[n.parent.unwrap() as usize] += 1;
        }
        let points:Vec<_> = tree.nodes.iter().enumerate().skip(1).map(|(i,n)|serde_json::json!({"id":i,"parent":n.parent,"tip":children[i]==0,"p":[n.position.x,n.position.y,n.position.z]})).collect();
        println!("OAK {}", serde_json::json!({"seed":seed,"nodes":points}));
    }
}

#[test]
fn shipped_scaffolds_are_reproducible() {
    // Re-recorded for the one builder; the seed and the rows, not the code
    // path, are what a tree is made of.
    for (preset, expected) in [
        (Preset::OregonWhiteOak, 14874354835152613499_u64),
        (Preset::NorwaySpruce, 11730507885382185461),
        (Preset::Ordinary, 12781088285770051220),
    ] {
        let family = preset.parameters();
        let hash = hashed(&family);
        assert_eq!(hash, hashed(&family), "{preset:?}: not reproducible");
        assert_eq!(hash, expected, "{preset:?} scaffold changed");
    }
}

#[test]
fn every_habit_trait_moves_every_shipped_preset() {
    for preset in PRESETS {
        let family = preset.parameters();
        let base = hashed(&family);
        let h = family.skeleton.habit;
        for (trait_name, habit) in [
            (
                "apical dominance",
                HabitParams {
                    apical_dominance: (h.apical_dominance - 0.05).abs(),
                    ..h
                },
            ),
            (
                "whorl strength",
                HabitParams {
                    whorl_strength: (h.whorl_strength - 0.05).abs(),
                    ..h
                },
            ),
            (
                "leader internode",
                HabitParams {
                    leader_internode: h.leader_internode * 0.9,
                    ..h
                },
            ),
            (
                "laterals per station",
                HabitParams {
                    laterals_per_station: h.laterals_per_station + 1,
                    ..h
                },
            ),
            (
                "lateral pitch",
                HabitParams {
                    lateral_pitch: h.lateral_pitch - 2.0,
                    ..h
                },
            ),
            (
                "lateral pitch variation",
                HabitParams {
                    pitch_variation: h.pitch_variation + 2.0,
                    ..h
                },
            ),
            (
                "primary rise per order",
                HabitParams {
                    rise_primary: h.rise_primary - 0.05,
                    ..h
                },
            ),
            (
                "secondary rise per order",
                HabitParams {
                    rise_secondary: h.rise_secondary - 0.05,
                    ..h
                },
            ),
            (
                "crookedness",
                HabitParams {
                    crookedness: h.crookedness + 2.0,
                    ..h
                },
            ),
            (
                "lateral spacing",
                HabitParams {
                    lateral_spacing: h.lateral_spacing * 0.9,
                    ..h
                },
            ),
            (
                "lateral length ratio",
                HabitParams {
                    lateral_length_ratio: h.lateral_length_ratio - 0.05,
                    ..h
                },
            ),
            (
                "lateral orders",
                HabitParams {
                    lateral_orders: h.lateral_orders - 1,
                    ..h
                },
            ),
            (
                "attractor weight",
                HabitParams {
                    attractor_weight: (h.attractor_weight - 0.05).abs(),
                    ..h
                },
            ),
            (
                "twig tip taper",
                HabitParams {
                    twig_tip_taper: h.twig_tip_taper * 0.8,
                    ..h
                },
            ),
            (
                "shedding threshold",
                HabitParams {
                    shedding_threshold: (h.shedding_threshold - 0.05).abs().max(0.05),
                    ..h
                },
            ),
        ] {
            let mut moved = family.clone();
            moved.skeleton.habit = habit;
            habit.validate().unwrap();
            assert_ne!(
                hashed(&moved),
                base,
                "{preset:?}: {trait_name} left the skeleton unmoved"
            );
        }
    }
}

#[test]
fn every_bias_term_reaches_every_shipped_preset() {
    for preset in PRESETS {
        let family = preset.parameters();
        let base = hashed(&family);
        let b = family.skeleton.bias;
        for (term, bias) in [
            (
                "gravitropism",
                BiasParams {
                    gravitropism: b.gravitropism + 0.1,
                    ..b
                },
            ),
            (
                "lean",
                BiasParams {
                    lean: b.lean + 0.03,
                    ..b
                },
            ),
        ] {
            let mut moved = family.clone();
            moved.skeleton.bias = bias;
            assert_ne!(hashed(&moved), base, "{preset:?}: {term} left no mark");
        }
        // The supernatural terms are inert while the field is off and its
        // amplitude is zero, so the field is switched on to read them.
        let field = SupernaturalParams {
            enabled: true,
            writhe_amplitude: 0.05,
            writhe_wavelength: 0.45,
            spiral_rate: 0.6,
        };
        let mut lit = family.clone();
        lit.skeleton.bias.supernatural = field;
        let on = hashed(&lit);
        assert_ne!(on, base, "{preset:?}: the supernatural field never landed");
        for (term, effects) in [
            (
                "writhe amplitude",
                SupernaturalParams {
                    writhe_amplitude: 0.09,
                    ..field
                },
            ),
            (
                "writhe wavelength",
                SupernaturalParams {
                    writhe_wavelength: 0.7,
                    ..field
                },
            ),
            (
                "spiral rate",
                SupernaturalParams {
                    spiral_rate: 1.4,
                    ..field
                },
            ),
        ] {
            let mut moved = lit.clone();
            moved.skeleton.bias.supernatural = effects;
            assert_ne!(hashed(&moved), on, "{preset:?}: {term} left no mark");
        }
    }
}

#[test]
fn trait_extremes_are_trees() {
    let base = Preset::Ordinary.parameters();
    for (trait_name, habit) in [
        (
            "apical dominance 0",
            HabitParams {
                apical_dominance: 0.0,
                ..base.skeleton.habit
            },
        ),
        (
            "apical dominance 1",
            HabitParams {
                apical_dominance: 1.0,
                ..base.skeleton.habit
            },
        ),
        (
            "whorl strength 0",
            HabitParams {
                whorl_strength: 0.0,
                ..base.skeleton.habit
            },
        ),
        (
            "whorl strength 1",
            HabitParams {
                whorl_strength: 1.0,
                ..base.skeleton.habit
            },
        ),
        (
            "rise per order -1",
            HabitParams {
                rise_primary: -1.0,
                rise_secondary: -1.0,
                ..base.skeleton.habit
            },
        ),
        (
            "rise per order +1",
            HabitParams {
                rise_primary: 1.0,
                rise_secondary: 1.0,
                ..base.skeleton.habit
            },
        ),
        (
            "attractor weight 0",
            HabitParams {
                attractor_weight: 0.0,
                ..base.skeleton.habit
            },
        ),
        (
            "attractor weight 1",
            HabitParams {
                attractor_weight: 1.0,
                ..base.skeleton.habit
            },
        ),
    ] {
        let mut family = base.clone();
        family.skeleton.habit = habit;
        let report = generate(&family.skeleton, family.radii)
            .unwrap_or_else(|e| panic!("{trait_name}: {e:?}"));
        report.tree.validate_solved().unwrap();
        assert!(
            report.tree.nodes.iter().all(|n| n.position.is_finite()),
            "{trait_name}: non-finite position"
        );
        assert!(report.tree.nodes.len() > 1, "{trait_name}: no tree");
    }
}

#[test]
#[ignore = "calibration read-out"]
fn shape_report() {
    for preset in [
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Ordinary,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let family = preset.parameters();
        let start = std::time::Instant::now();
        let report = generate(&family.skeleton, family.radii).unwrap();
        let t = &report.tree;
        let mut twig_length = 0.0;
        let mut children = vec![0usize; t.nodes.len()];
        for n in t.nodes.iter().skip(1) {
            children[n.parent.unwrap() as usize] += 1;
            if n.kind == NodeKind::Twig {
                twig_length += n
                    .position
                    .distance(t.nodes[n.parent.unwrap() as usize].position);
            }
        }
        let origins = t.nodes[..t.crossover]
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(i, n)| {
                children[*i] == 0
                    || n.radius < family.skeleton.twigs.limb_radius * t.nodes[0].radius
            })
            .count();
        let top = t.nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
        let width = t
            .nodes
            .iter()
            .map(|n| n.position.x.hypot(n.position.z))
            .fold(0.0, f64::max)
            * 2.0;
        println!(
            "{:?}: nodes {} crossover {} origins {origins} twig_m {twig_length:.0} top {top:.2} width {width:.2} shed {} capped {} in {:.1}ms",
            preset,
            t.nodes.len(),
            t.crossover,
            report.shed,
            t.diagnostics.node_capped,
            start.elapsed().as_secs_f64() * 1000.0
        );
    }
}
