//! A subtree fades once (fn-359): a line whose living apex stands small
//! is drawn at that apex's size, however many axes carry it on.
use super::*;
use crate::chain::passed;
use crate::structure::{Phytomer, Structure, Vec3};

/// A trunk that never sheds and a limb age that sheds 2 cycles idle.
fn species() -> Species {
    let mut limb = passed();
    limb.shedding = 2.0;
    Species {
        states: vec![passed(), limb],
    }
}

fn axis(origin: Origin, pa: usize, birth: u32, alive: f64) -> Axis {
    let phytomer = Phytomer {
        cycle: birth + 1,
        tip: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        radius: 0.0,
        scale: 1.0,
        key: 0,
        rank: 0.0,
        size: 1.0,
        stretch: 1.0,
        grown: 1.0,
        section: 0.0,
        sapwood: 0.0,
    };
    Axis {
        lineage: u64::from(birth),
        pa,
        birth,
        origin,
        vigour: 1.0,
        blend: 1.0,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: vec![phytomer; 2],
        units: vec![1.0, 1.0],
        alive,
        rank: 0.0,
        sleep: 0.0,
        phase: 0.0,
        joined: None,
    }
}

/// A trunk bearing a limb lateral born 2 and three relays carrying it on,
/// each born as the one before stopped; the last relay's apex lives at
/// `apex`. Nothing has grown since cycle 10.
fn line(apex: f64) -> Vec<Axis> {
    let lateral = Origin::Lateral {
        parent: 0,
        node: 1,
        slot: 0,
        whorl: 1.0,
        woken: false,
    };
    let relay = |parent| Origin::Relay { parent, node: 1 };
    vec![
        axis(Origin::Seed, 0, 0, 1.0),
        axis(lateral, 1, 2, 0.0),
        axis(relay(1), 1, 4, 0.0),
        axis(relay(2), 1, 6, 0.0),
        axis(relay(3), 1, 8, apex),
    ]
}

/// Each kept axis's share as drawn, which its girth reads.
fn shares(shed: &Shed) -> Vec<f64> {
    shed.applied.clone()
}

/// Each kept axis's base scale, settled at `age`.
fn settled(axes: Vec<Axis>, age: u32) -> (Vec<f64>, Shed) {
    let species = species();
    let shed = shed(axes, &species, age);
    let mut tree = Structure {
        age,
        pas: species.states.len(),
        axes: shed.axes.clone(),
    };
    (crate::geometry::scale(&mut tree, &species, 1), shed)
}

/// R2: the line's only living apex stands at an eighth relative to each
/// of its axes, idle past the delay. Each axis fades to an eighth, and the
/// last relay is drawn at the lateral's eighth, not at its fourth power.
#[test]
fn a_line_idle_past_its_delay_fades_once_to_its_living_apex() {
    let (base, shed) = settled(line(0.125), 20);
    assert_eq!(shed.axes.len(), 5, "nothing shed");
    assert_eq!(base[1], 0.125, "the lateral at an eighth of its unfaded scale");
    assert_eq!(base[4], base[1], "the last relay at the lateral's scale");
    let share = shares(&shed);
    assert_eq!(share[1], 0.125);
    assert_eq!(share[4], share[1], "the last relay's girth at the lateral's share");
}

/// R2: where every presence is whole, nothing kept fades: every share is
/// one and the shed tree is the tree grown, to the bit.
#[test]
fn whole_presences_fade_nothing_kept() {
    for age in [10, 12, 30] {
        let grown = line(1.0);
        let (base, shed) = settled(grown.clone(), age);
        assert_eq!(shed.axes, grown, "at {age}: the tree as grown");
        assert!(shares(&shed).iter().all(|&s| s == 1.0), "at {age}");
        assert!(base.iter().all(|&b| b == 1.0), "at {age}");
    }
}
