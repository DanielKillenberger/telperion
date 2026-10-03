//! R1's minimal geometry: internodes of their PA's length, each lateral at
//! its PA's insertion angle to its parent, successive nodes turned by the
//! divergence.
use std::f64::consts::PI;
use telperion_space::{grow, NodeLaw, Origin, PaState, Request, Species, Vec3, Zone};

fn length(v: Vec3) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

#[test]
fn axes_take_their_lengths_and_angles_from_their_pa() {
    let state = |internode, insertion, lateral: &[f64]| PaState {
        lifespan: 5,
        next: None,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 2, max: 2 },
            buds: 2,
            lateral: lateral.to_vec(),
        }],
        shedding: None,
        internode,
        insertion,
        divergence: PI / 2.0,
    };
    let species = Species {
        states: vec![state(0.5, 0.0, &[0.0, 1.0]), state(0.3, 0.6, &[0.0, 0.0])],
    };
    let tree = grow(
        &species,
        Request {
            age: 5,
            seed: 3,
            budget: 1_000,
        },
    )
    .unwrap();
    let trunk = &tree.axes[0];
    assert_eq!(
        trunk.phytomers.last().unwrap().tip,
        Vec3::new(0.0, 0.0, 5.0)
    );
    let mut sides = Vec::new();
    for axis in tree.axes.iter().filter(|a| !a.phytomers.is_empty()) {
        let Origin::Lateral { node, .. } = axis.origin else {
            continue;
        };
        let tip = axis.phytomers.last().unwrap().tip;
        let run = tip - axis.base;
        assert!((length(run) - 0.3 * axis.phytomers.len() as f64).abs() < 1e-12);
        assert!(
            (run.z / length(run) - 0.6f64.cos()).abs() < 1e-12,
            "the insertion angle"
        );
        assert_eq!(axis.base, trunk.phytomers[node].tip);
        sides.push(((run.y.atan2(run.x).to_degrees().round() as i64) + 360) % 360);
    }
    sides.sort_unstable();
    sides.dedup();
    assert_eq!(
        sides,
        vec![0, 90, 180, 270],
        "opposite pairs turned a quarter each node"
    );
}
