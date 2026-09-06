//! Reproducible stage audit for retained fidelity counterexamples.
use super::*;
use crate::presets::Preset;
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
            let mut tree = habit::generate(p, &config, &bias).unwrap();
            radius::solve(&mut tree, p.envelope, f.radii).unwrap();
            let structural = tree.clone();
            local::append_with_habit(&mut tree, &config, p.twigs, seed, Some(&bias), p.habit)
                .unwrap();
            let before = tree.clone();
            let removed = shed_with_upper_supports(
                &mut tree,
                p.envelope,
                0.45,
                matches!(p.habit, BranchHabit::Spreading(_)),
            )
            .unwrap();
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
            if preset == Preset::OregonWhiteOak {
                for (i, n) in structural.nodes.iter().enumerate() {
                    if n.position.y >= max_y * 0.75 {
                        assert_eq!(descendants[i][0], 0, "upper support {i} lost twigs");
                    }
                }
            }
        }
    }
}

#[test]
fn upper_retention_does_not_disable_lower_shedding() {
    let mut tree = Tree::default();
    for (i, (y, parent)) in [
        (0.0, None),
        (6.0, Some(0)),
        (12.0, Some(1)),
        (6.2, Some(1)),
        (12.2, Some(2)),
    ]
    .into_iter()
    .enumerate()
    {
        tree.nodes.push(Node {
            position: Vec3::new(0.0, y, 0.0),
            parent,
            branch: i as u32,
            kind: if i < 3 {
                NodeKind::Structural
            } else {
                NodeKind::Twig
            },
            ..Node::root()
        });
    }
    tree.crossover = 3;
    let envelope = Envelope {
        height: 24.0,
        crown_base: 0.0,
        spread: 0.55,
        ..Envelope::default()
    };
    assert_eq!(
        shed_with_upper_supports(&mut tree, envelope, 0.0, true).unwrap(),
        1
    );
    assert_eq!(tree.nodes.len(), 4);
    assert_eq!(tree.nodes[3].position.y, 12.2);
    assert_eq!(tree.nodes[3].parent, Some(2));
    tree.validate().unwrap();
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
        let tree = habit::generate(p, &config, &bias).unwrap();
        let mut children = vec![0; tree.nodes.len()];
        for n in tree.nodes.iter().skip(1) {
            children[n.parent.unwrap() as usize] += 1;
        }
        let points:Vec<_> = tree.nodes.iter().enumerate().skip(1).map(|(i,n)|serde_json::json!({"id":i,"parent":n.parent,"tip":children[i]==0,"p":[n.position.x,n.position.y,n.position.z]})).collect();
        println!("OAK {}", serde_json::json!({"seed":seed,"nodes":points}));
    }
}

#[test]
fn accepted_oak_scaffolds_are_unchanged() {
    // Pass-3 structural positions and parent links, before pass-4 spacing repair.
    for (seed, expected) in [
        (1, 8709238459600713761),
        (762807349, 3941552297145586417),
        (1444323199, 4459427015154599464),
    ] {
        let mut f = Preset::OregonWhiteOak.parameters();
        f.skeleton.seed = seed;
        let p = &f.skeleton;
        let config = p.resolved_growth(0).unwrap();
        let bias = GrowthBias::new(p.envelope, seed, p.bias).unwrap();
        let tree = habit::generate(p, &config, &bias).unwrap();
        let mut hash = 14695981039346656037_u64;
        for n in tree.nodes.iter().skip(1) {
            for byte in [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
                .chain(n.parent.unwrap().to_le_bytes())
            {
                hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
            }
        }
        assert_eq!(hash, expected, "accepted oak {seed} scaffold changed");
    }
}
