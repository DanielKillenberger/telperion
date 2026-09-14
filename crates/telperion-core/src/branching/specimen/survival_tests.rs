use super::*;
use crate::presets::Preset;

fn mature_crown(preset: Preset, minimum: usize) {
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    assert_eq!(family.skeleton.habit.shedding_threshold, 0.45);
    let s = Specimen::build(&family).unwrap();
    let nodes = s.tree().nodes.len();
    eprintln!(
        "SURVIVAL {preset:?} age={} threshold={} nodes={nodes} shed={}",
        s.age(),
        family.skeleton.habit.shedding_threshold,
        s.shed
    );
    assert!(
        nodes >= minimum,
        "{preset:?} mature crown collapsed to {nodes} nodes"
    );
}

#[test]
fn ordinary_retains_its_mature_crown_with_authored_shedding() {
    mature_crown(Preset::Ordinary, 10_000);
}

#[test]
fn telperion_retains_its_mature_crown_with_authored_shedding() {
    mature_crown(Preset::Telperion, 50_000);
}

#[test]
fn laurelin_retains_its_mature_crown_with_authored_shedding() {
    mature_crown(Preset::Laurelin, 50_000);
}

#[test]
fn mature_light_survives_while_interior_shoots_die_as_stamps() {
    for preset in [Preset::Ordinary, Preset::Telperion, Preset::Laurelin] {
        let mut family = preset.parameters();
        family.age = 0.0;
        assert_eq!(family.skeleton.habit.shedding_threshold, 0.45);
        let mut s = Specimen::build(&family).unwrap();
        let e = family.skeleton.envelope;
        let y = e.height * (e.crown_base + (1.0 - e.crown_base) * e.fullness);
        for x in [0.0, e.radius_at(y)] {
            let i = s.tree.nodes.len();
            s.tree.nodes.push(Node {
                position: Vec3::new(x, y, 0.0),
                parent: Some(0),
                branch: i as u32,
                kind: NodeKind::Branch,
                radius: 0.001,
                start_radius: 0.001,
                base_radius: 0.001,
                ..Node::root()
            });
        }
        s.identify();
        s.timeline.as_mut().unwrap().envelope = e;
        let dark = s.tree.nodes[1].identity;
        let lit = s.tree.nodes[2].identity;
        let year = family.growth.mature_age() as u64;
        for slice in year..year + 3 {
            let roots = s.environment(slice);
            s.stamp_deaths(&roots, slice);
        }
        assert!(
            s.tree.nodes[1].shoot.death_year.is_some(),
            "{preset:?}: shaded shoot never shed"
        );
        assert!(
            s.tree.nodes[2].shoot.death_year.is_none(),
            "{preset:?}: lit shoot died by age alone"
        );
        eprintln!(
            "DEATH {preset:?} dark={dark:?} stamp={:?} lit={lit:?} survives=true",
            s.tree.nodes[1].shoot.death_year
        );
    }
}
