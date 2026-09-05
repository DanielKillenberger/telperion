use telperion_core::{
    bias::{BiasParams, GrowthBias, SupernaturalParams},
    colonization::{colonize, GrowthConfig},
    envelope::Envelope,
    math::Vec3,
    rng::Rng,
};

#[test]
fn ordinary_and_giant_crowns() {
    for (height, seed, count) in [(24.0, 42, 500), (148.0, 1, 1600), (132.0, 2, 1600)] {
        let envelope = Envelope {
            height,
            ..Default::default()
        };
        let points = envelope.sample(count, &mut Rng::new(seed)).unwrap();
        let config = GrowthConfig {
            step_distance: height * 0.02,
            kill_distance: height * 0.04,
            influence_radius: height * 0.18,
            trunk_height: height * 0.3,
            shell: Some(envelope),
            ..Default::default()
        };
        let bias = GrowthBias::new(envelope, seed, BiasParams::default()).unwrap();
        let tree = colonize(&points, Vec3::ZERO, &config, Some(&bias)).unwrap();
        tree.validate().unwrap();
        assert!(tree.nodes.len() > 100);
        assert!(!tree.diagnostics.node_capped);
        assert_eq!(
            tree,
            colonize(&points, Vec3::ZERO, &config, Some(&bias)).unwrap()
        );
        for n in tree.nodes.iter().skip(1) {
            let parent = &tree.nodes[n.parent.unwrap() as usize];
            if parent.position.y >= config.trunk_height && envelope.contains(parent.position, 0.0) {
                assert!(envelope.contains(n.position, 0.0));
            }
        }
    }
}

#[test]
#[ignore = "requires pinned FN6 export; run crown_reference.mjs"]
fn compare_pinned_fn6_when_requested() {
    let directory = std::env::var("CROWN_REFERENCE_DIR")
        .expect("run crown_reference.mjs to export the pinned FN6 crown");
    for id in [
        "ordinary",
        "telperion",
        "laurelin",
        "empty",
        "capped",
        "envelope-crossing",
    ] {
        let source = std::fs::read_to_string(format!("{directory}/{id}.txt")).unwrap();
        let mut lines = source.lines();
        assert_eq!(lines.next(), Some(id));
        let values = |line: &str| {
            line.split_whitespace()
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
        };
        let e = values(lines.next().unwrap());
        let envelope = Envelope {
            height: e[0],
            crown_base: e[1],
            spread: e[2],
            fullness: e[3],
            shoulder: e[4],
        };
        let b = values(lines.next().unwrap());
        let bias = GrowthBias::new(
            envelope,
            b[0] as u32,
            BiasParams {
                gravitropism: b[1],
                lean: b[2],
                supernatural: SupernaturalParams {
                    enabled: true,
                    writhe_amplitude: b[3],
                    writhe_wavelength: b[4],
                    spiral_rate: b[5],
                },
            },
        )
        .unwrap();
        let c = values(lines.next().unwrap());
        let config = GrowthConfig {
            influence_radius: c[0],
            kill_distance: c[1],
            step_distance: c[2],
            trunk_height: c[3],
            max_nodes: c[4] as usize,
            max_turn_per_step: c[5],
            shell: Some(envelope),
        };
        let count: usize = lines.next().unwrap().parse().unwrap();
        let points: Vec<_> = (0..count)
            .map(|_| {
                let p = values(lines.next().unwrap());
                Vec3::new(p[0], p[1], p[2])
            })
            .collect();
        let reference_count: usize = lines.next().unwrap().parse().unwrap();
        let reference: Vec<_> = (0..reference_count)
            .map(|_| values(lines.next().unwrap()))
            .collect();
        assert!(lines.next().is_none());
        let tree = colonize(&points, Vec3::ZERO, &config, Some(&bias)).unwrap();
        tree.validate().unwrap();
        assert_eq!(tree.diagnostics.node_capped, id == "capped");
        let same_topology = tree.nodes.len() == reference.len()
            && tree
                .nodes
                .iter()
                .zip(&reference)
                .all(|(n, r)| n.parent.map_or(-1, |p| p as i64) == r[0] as i64);
        let max_delta = if same_topology {
            tree.nodes
                .iter()
                .zip(&reference)
                .map(|(n, r)| n.position.distance(Vec3::new(r[1], r[2], r[3])))
                .fold(0.0, f64::max)
        } else {
            f64::NAN
        };
        println!("{id}: FN6={reference_count}, Rust={}, same_topology={same_topology}, max_position_delta={max_delta:e}, capped={}",tree.nodes.len(),tree.diagnostics.node_capped);
        for n in tree.nodes.iter().skip(1) {
            let parent = &tree.nodes[n.parent.unwrap() as usize];
            if parent.position.y >= config.trunk_height && envelope.contains(parent.position, 0.0) {
                assert!(envelope.contains(n.position, 0.0));
            }
        }
    }
}
