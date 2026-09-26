use telperion_core::{branching, presets::Preset, tree::NodeKind};

#[test]
fn spreading_crown_retains_generated_interior_subdivisions() {
    for seed in [2, 3, 2666899686] {
        let mut family = Preset::OregonWhiteOak.parameters();
        family.skeleton.seed = seed;
        let report = branching::generate(&family.skeleton, family.radii).unwrap();
        assert_eq!(report.shed, 0, "seed {seed}: grown crown subdivisions lost");
        assert!(report.tree.diagnostics.complete());
        let interior = report
            .tree
            .nodes
            .iter()
            .filter(|n| {
                n.kind == NodeKind::Twig
                    && n.position.y < family.skeleton.envelope.height * 0.75
                    && n.position.x.hypot(n.position.z)
                        < family.skeleton.envelope.radius_at(n.position.y) * 0.4
            })
            .count();
        assert!(interior > 20, "seed {seed}: only {interior} interior twigs");
    }
}

#[test]
fn spruce_bearing_shoots_follow_the_secondary_span() {
    for seed in [1, 2, 3, 4250668600] {
        let mut family = Preset::NorwaySpruce.parameters();
        family.skeleton.seed = seed;
        let report = branching::generate(&family.skeleton, family.radii).unwrap();
        let tree = &report.tree;
        let mut children = vec![Vec::new(); tree.nodes.len()];
        for (i, n) in tree.nodes.iter().enumerate().skip(1) {
            children[n.parent.unwrap() as usize].push(i);
        }
        let mut owner: Vec<Option<(usize, f64)>> = vec![None; tree.nodes.len()];
        let mut spans = Vec::new();
        for (i, n) in tree.nodes.iter().enumerate().skip(1) {
            let p = n.parent.unwrap() as usize;
            if owner[i].is_none() {
                // A node the walk below already placed keeps its own distance.
                owner[i] = owner[p];
            }
            if i < tree.crossover
                && owner[i].is_none()
                && (n.position - tree.nodes[p].position).normalized().y < -0.5
            {
                // Walk the hanging axis itself, recording how far along it each
                // node sits: the wood is judged along the axis it clothes, not
                // against the height the axis happens to lose.
                let (mut end, mut along) = (i, 0.0);
                let mut lowest = tree.nodes[i].position.y;
                let mut walk = vec![(i, 0.0)];
                while let Some(&next) = children[end].iter().find(|&&c| c < tree.crossover) {
                    along += tree.nodes[next].position.distance(tree.nodes[end].position);
                    lowest = lowest.min(tree.nodes[next].position.y);
                    walk.push((next, along));
                    end = next;
                }
                let span = spans.len();
                spans.push((along, lowest));
                for (node, distance) in walk {
                    owner[node] = Some((span, distance));
                }
            }
        }
        let mut lengths = [0.0; 4];
        for (i, n) in tree.nodes.iter().enumerate().skip(tree.crossover) {
            let Some((span, along)) = owner[i] else {
                continue;
            };
            let (length, lowest) = spans[span];
            if length < 0.3 {
                continue;
            }
            if n.kind != NodeKind::Twig
                && n.radius.max(n.start_radius) > tree.nodes[0].radius * family.canopy.shoot_radius
            {
                continue;
            }
            let parent = &tree.nodes[n.parent.unwrap() as usize];
            let bin = if (n.position.y + parent.position.y) * 0.5 < lowest {
                3
            } else {
                ((along / length).clamp(0.0, 1.0) * 3.0).floor().min(2.0) as usize
            };
            lengths[bin] += n.position.distance(parent.position);
        }
        let total: f64 = lengths.iter().sum();
        println!(
            "seed {seed}: nodes={} bearing thirds/below={lengths:?}",
            tree.nodes.len()
        );
        assert!(total > 10.0);
        assert!(
            lengths[3] / total < 0.25,
            "seed {seed}: bearing wood below the secondary's own reach {lengths:?}"
        );
        for length in &lengths[..3] {
            assert!(
                *length / total > 0.1,
                "seed {seed}: unclothed third {lengths:?}"
            );
        }
    }
}

#[test]
fn oak_infill_reaches_the_retained_seed_two_window() {
    use telperion_core::math::Vec3;
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 2;
    let report = branching::generate(&family.skeleton, family.radii).unwrap();
    let camera = Vec3::new(25.454365371536586, 20.23331671361438, 42.31588348801908);
    let target = Vec3::new(-0.4632261710395369, 8.528597952450966, 0.5133164838640489);
    let forward = (target - camera).normalized();
    let right = forward.cross(Vec3::Y).normalized();
    let up = right.cross(forward);
    let tangent = 19.0_f64.to_radians().tan();
    let ray = (forward
        + right * ((553.0 / 960.0 * 2.0 - 1.0) * tangent * 4.0 / 3.0)
        + up * ((1.0 - 372.0 / 720.0 * 2.0) * tangent))
        .normalized();
    let near = report
        .tree
        .nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Twig && (n.position - camera).cross(ray).length() < 0.5)
        .count();
    println!("oak2 retained window: {near} twig endpoints within 0.5m ray");
    assert!(
        near > 10,
        "retained seed2 window has only {near} nearby endpoints"
    );
}
