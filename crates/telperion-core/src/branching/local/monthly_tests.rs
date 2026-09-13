use super::*;

#[test]
fn monthly_deferred_shoot_does_not_starve_a_younger_live_shoot() {
    let mut tree = Tree {
        nodes: vec![Node::root()],
        crossover: 3,
        ..Tree::default()
    };
    for y in [0.95, 0.5] {
        tree.nodes.push(Node {
            position: Vec3::new(0.0, y, 0.0),
            radius: 0.001,
            start_radius: 0.001,
            parent: Some(0),
            branch: tree.nodes.len() as u32,
            ..Node::root()
        });
    }
    tree.nodes[0].radius = 0.01;
    tree.nodes[0].start_radius = 0.01;
    let mut frontier = Frontier::default();
    for at in [1, 2] {
        frontier.queue.push_back(Shoot {
            flushed: 0,
            accepted: Vec::new(),
            at,
            direction: Vec3::Y,
            normal: Vec3::X,
            phase: 0.0,
            radius: 0.001,
            length: 0.25,
            branch: None,
            completed: 0,
            generation: 0,
            internodes: 1,
            key: at as u32,
            run: None,
            pendant: false,
            curtain_across: Vec3::X,
            pendant_floor: None,
        });
    }
    let config = GrowthConfig {
        trunk_height: 0.0,
        shell: Some(Envelope {
            height: 1.0,
            crown_base: 0.0,
            ..Envelope::default()
        }),
        ..GrowthConfig::default()
    };
    frontier
        .advance(
            &mut tree,
            Planner {
                clock: None,
                widths: None,
                growing_envelope: true,
                planning: None,
                config: &config,
                bias: None,
                twigs: TwigParams {
                    laterals: 0,
                    ..TwigParams::default()
                },
                crookedness: 0.0,
                seed: 7,
            },
            HabitParams::default(),
            1,
        )
        .unwrap();
    assert_eq!(
        tree.nodes.len(),
        4,
        "a waiting shoot must not consume the live shoot's unit"
    );
    assert_eq!(tree.nodes[3].parent, Some(2));
    assert!(
        frontier.queue.iter().any(|s| s.at == 1),
        "the waiting bud must remain available"
    );
}

#[test]
fn monthly_local_station_is_reconsidered_when_it_becomes_eligible() {
    let mut tree = Tree {
        nodes: vec![Node::root()],
        crossover: 3,
        ..Tree::default()
    };
    tree.nodes[0].radius = 1.0;
    for i in 1..3 {
        let mut n = Node {
            position: Vec3::new(0.0, i as f64, 0.0),
            radius: 0.5,
            parent: Some((i - 1) as u32),
            branch: i as u32,
            ..Node::root()
        };
        n.identity.birth = i as u64;
        tree.nodes.push(n);
    }
    let mut frontier = Frontier::default();
    let config = GrowthConfig {
        trunk_height: 0.0,
        ..GrowthConfig::default()
    };
    let twigs = TwigParams::default();
    frontier.seed(&tree, &config, twigs, HabitParams::default(), None);
    assert!(frontier.queue.iter().all(|s| s.at != 1));
    // The trunk thickens while this supporting branch keeps its radius: its
    // ratio now permits local laterals. An earlier refusal is not a bud birth.
    tree.nodes[0].radius = 10.0;
    frontier.seed(&tree, &config, twigs, HabitParams::default(), None);
    assert_eq!(
        frontier.queue.iter().filter(|s| s.at == 1).count(),
        1,
        "a temporarily ineligible station must remain able to bud"
    );
    frontier.seed(&tree, &config, twigs, HabitParams::default(), None);
    assert_eq!(
        frontier.queue.iter().filter(|s| s.at == 1).count(),
        1,
        "an eligible station must not be seeded twice"
    );
}

#[test]
fn monthly_run_keeps_stations_waiting_beyond_the_current_envelope() {
    let live = Envelope {
        height: 1.0,
        crown_base: 0.0,
        ..Envelope::default()
    };
    let config = GrowthConfig {
        trunk_height: 0.0,
        shell: Some(live),
        ..GrowthConfig::default()
    };
    let planner = Planner {
        clock: None,
        widths: None,
        growing_envelope: true,
        planning: Some(Envelope {
            height: 2.0,
            ..live
        }),
        config: &config,
        bias: None,
        twigs: TwigParams::default(),
        crookedness: 0.0,
        seed: 7,
    };
    let run = planner
        .run(Vec3::new(0.0, 0.5, 0.0), Vec3::Y, 1.0, 4, false, 11)
        .unwrap();
    assert_eq!(
        run.length, 1.0,
        "a temporary boundary must not permanently shorten the shoot"
    );
    assert_eq!(run.positions.last().unwrap().y, 1.5);
    assert!(
        run.positions.iter().any(|&p| rejected(&config, p)),
        "unreached stations must wait for expansion before they can be appended"
    );
}

#[test]
fn monthly_terminal_birth_does_not_retire_unborn_lateral_buds() {
    let mut tip = Node {
        position: Vec3::Y,
        radius: 0.5,
        parent: Some(0),
        branch: 1,
        ..Node::root()
    };
    tip.identity.birth = 1;
    let mut tree = Tree {
        nodes: vec![
            Node {
                radius: 1.0,
                ..Node::root()
            },
            tip,
        ],
        crossover: 2,
        ..Tree::default()
    };
    let config = GrowthConfig {
        trunk_height: 0.0,
        ..GrowthConfig::default()
    };
    let twigs = TwigParams::default();
    let mut frontier = Frontier::default();
    frontier.seed(&tree, &config, twigs, HabitParams::default(), None);
    assert_eq!(frontier.queue.len(), 1);
    tree.nodes[0].radius = 10.0;
    frontier.seed(&tree, &config, twigs, HabitParams::default(), None);
    assert_eq!(
        frontier.queue.len(),
        2,
        "terminal allocation must leave the lateral buds available"
    );
    assert_ne!(
        frontier.queue[1].flushed & 1,
        0,
        "the later allocation must not duplicate the terminal"
    );
    assert_eq!(
        frontier.queue[1].flushed & 0b110,
        0,
        "the laterals have not flushed yet"
    );
    frontier.seed(&tree, &config, twigs, HabitParams::default(), None);
    assert_eq!(frontier.queue.len(), 2, "each bud is allocated only once");
}
