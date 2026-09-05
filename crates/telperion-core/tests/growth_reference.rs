use std::{fs, path::Path};
use telperion_core::{branching::generate, presets::Preset, tree::NodeKind};
fn floats(dir: &Path, id: &str, suffix: &str) -> Vec<f64> {
    let b = fs::read(dir.join(format!("{id}-{suffix}.bin"))).unwrap();
    assert_eq!(b.len() % 8, 0);
    b.as_chunks::<8>()
        .0
        .iter()
        .map(|b| f64::from_le_bytes(*b))
        .collect()
}
fn ints(dir: &Path, id: &str, suffix: &str) -> Vec<i32> {
    let b = fs::read(dir.join(format!("{id}-{suffix}.bin"))).unwrap();
    assert_eq!(b.len() % 4, 0);
    b.as_chunks::<4>()
        .0
        .iter()
        .map(|b| i32::from_le_bytes(*b))
        .collect()
}
#[test]
#[ignore = "requires pinned FN6 export: GROWTH_REFERENCE_DIR=<directory> cargo test --test growth_reference -- --ignored --nocapture"]
fn complete_fn6_comparison() {
    let directory = std::env::var("GROWTH_REFERENCE_DIR").expect("pinned FN6 export directory");
    let dir = Path::new(&directory);
    for (id, preset) in [
        ("ordinary", Preset::Ordinary),
        ("telperion", Preset::Telperion),
        ("laurelin", Preset::Laurelin),
        ("empty", Preset::Ordinary),
        ("capped", Preset::Ordinary),
        ("degenerate", Preset::Ordinary),
        ("envelope-crossing", Preset::Ordinary),
    ] {
        let mut p = preset.parameters();
        if preset == Preset::Ordinary {
            p.skeleton.step = 0.02
        }
        if id == "empty" {
            p.skeleton.attractors = 0
        }
        if id == "degenerate" {
            p.skeleton.envelope.spread = 0.0;
            p.skeleton.attractors = 0;
        }
        if id == "envelope-crossing" {
            p.skeleton.envelope.crown_base = 0.8;
            p.skeleton.attractors = 64;
        }
        if id == "capped" {
            p.skeleton.growth.max_nodes = Some(20)
        }
        let metadata = fs::read_to_string(dir.join(format!("{id}-growth-report.txt")))
            .expect("run tests/migration/growth-reference.mjs");
        let mut words = metadata.split_whitespace();
        assert_eq!(
            words.next(),
            Some("fdafb099b1495519de75a6b9a66d37f7d07e47bd")
        );
        let expected: Vec<usize> = words.map(|s| s.parse().unwrap()).collect();
        let report = generate(&p.skeleton, p.radii).unwrap();
        assert_eq!(
            expected,
            [
                report.tree.nodes.len(),
                report.tree.crossover,
                report.shed,
                usize::from(report.tree.diagnostics.node_capped),
                usize::from(report.tree.diagnostics.level_capped)
            ]
        );
        let tree = report.tree;
        let parents = ints(dir, id, "parents");
        let positions = floats(dir, id, "positions");
        let radius = floats(dir, id, "radius");
        let start = floats(dir, id, "start-radius");
        let branch = ints(dir, id, "branch-id");
        let base = floats(dir, id, "base-radius");
        let ends = floats(dir, id, "end-radius");
        let twigs = fs::read(dir.join(format!("{id}-twig.bin"))).unwrap();
        let same = tree.nodes.len() == parents.len()
            && tree
                .nodes
                .iter()
                .zip(&parents)
                .all(|(n, &p)| n.parent.map_or(-1, |p| p as i32) == p);
        println!(
            "{id}: native={} reference={} crossover={} shed={} topology={same}",
            tree.nodes.len(),
            parents.len(),
            tree.crossover,
            report.shed
        );
        assert!(same, "{id} topology differs; diagnose explicitly");
        assert_eq!(tree.crossover, parents.len() - branch.len());
        let mut max_position = 0.0_f64;
        let mut max_radius = 0.0_f64;
        for (i, n) in tree.nodes.iter().enumerate() {
            for (j, v) in [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .enumerate()
            {
                max_position = max_position.max((v - positions[3 * i + j]).abs())
            }
            max_radius = max_radius
                .max((n.radius - radius[i]).abs())
                .max((n.start_radius - start[i]).abs());
            if i >= tree.crossover {
                let j = i - tree.crossover;
                assert_eq!(n.branch as i32, branch[j]);
                assert_eq!(n.kind == NodeKind::Twig, twigs[j] == 1);
                max_radius = max_radius
                    .max((n.base_radius - base[j]).abs())
                    .max((n.radius - ends[j]).abs());
            }
        }
        println!("{id}: max_position={max_position:e} max_radius={max_radius:e}");
        assert!(max_position < 1e-5);
        assert!(max_radius < 1e-7);
    }
}
