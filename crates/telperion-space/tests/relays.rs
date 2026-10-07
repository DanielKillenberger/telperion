//! fn-199: relays change the tree by degree. A relay always taking over a
//! stop (Troll's modules) makes four walked settings jump on the base the
//! Codex re-review of fn-193 read: survival (R1), the relay's place along
//! its parent (R2), a bent lateral's straightening (R3) and the parent's
//! girth where the relay joins it (R4). Each is walked here at a fixed
//! seed and held to the walk tests' bound.
mod walk;
use telperion_space::{NodeLaw, Species, Structure};
use walk::degree::{by_degree, REFINE, SEEDS};
use walk::{setting, Setting, STEPS};

/// Every stop always relayed: aborted, ended and failed.
fn always_relayed(s: &mut Species, pa: usize) {
    let state = &mut s.states[pa];
    (state.relay, state.relay_ended, state.relay_failed) = (1.0, 1.0, 1.0);
}

fn assert_by_degree(walked: Setting) {
    let failed = by_degree(&[walked]);
    assert!(failed.is_empty(), "{failed:#?}");
}

/// R1: with every stop relayed, a limb's yearly survival crossing a draw
/// moves its growth by degree, not by the whole unit a failure loses.
#[test]
fn survival_with_every_stop_relayed_changes_the_tree_by_degree() {
    assert_by_degree(setting(
        "states[1].viability, relay 1".into(),
        0.6,
        1.0 - walk::EDGE,
        true,
        |s, v| {
            always_relayed(s, 1);
            s.states[1].viability = v;
        },
    ));
}

/// R2: a relay stands at its share of its parent's last growth unit by
/// the unit's scaled length, so a node growing in from nothing beneath it
/// moves it by nothing (fn-206, b172c2d4).
#[test]
fn a_node_growing_in_moves_no_relay() {
    assert_by_degree(setting(
        "states[0].zones[1].nodes.max, relay 1".into(),
        2.0,
        4.0,
        false,
        |s, v| {
            let trunk = &mut s.states[0];
            trunk.relay = 1.0;
            trunk.relay_at = 0.5;
            trunk.abortion = 0.3;
            trunk.zones[1].nodes = NodeLaw::Uniform { min: 2.0, max: v };
        },
    ));
}

/// R3: on a bent lateral, a relay that has barely left its parent keeps
/// the parent's remaining straightening, so a limb's module-ending chance
/// crossing a draw bends it by degree.
#[test]
fn a_relay_barely_off_a_bent_limb_changes_the_tree_by_degree() {
    assert_by_degree(setting(
        "states[1].abortion, relay 1, straightening 1".into(),
        walk::EDGE,
        0.5,
        true,
        |s, v| {
            always_relayed(s, 1);
            let limb = &mut s.states[1];
            limb.relay_at = 0.5;
            limb.epitony = 0.6;
            limb.straightening = 1.0;
            limb.abortion = v;
        },
    ));
}

/// Each phytomer's radius by axis and node: the walk keeps the topology.
fn radii(tree: &Structure) -> Vec<Vec<f64>> {
    let radii = |a: &telperion_space::Axis| a.phytomers.iter().map(|p| p.radius).collect();
    tree.axes.iter().map(radii).collect()
}

/// The largest change of any radius between two trees of one topology, as
/// a share of the larger tree's thickest.
fn girth_change(a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
    assert_eq!(a.len(), b.len(), "the walk changed the topology");
    let thickest = a.iter().chain(b).flatten().fold(0.0, |m: f64, &r| m.max(r));
    let moved = a.iter().zip(b).flat_map(|(x, y)| {
        assert_eq!(x.len(), y.len(), "the walk changed the topology");
        x.iter().zip(y).map(|(r, s)| (r - s).abs())
    });
    moved.fold(0.0, f64::max) / thickest
}

/// R4: a relay's girth enters its parent spread over the span it stands
/// in, so walking where it stands (`relay_at`) moves the parent's radius
/// by degree: the steepest step, split eighth by eighth six times, shrinks
/// twentyfold, as a slope does and a jump does not.
#[test]
fn a_relay_moving_along_its_parent_thickens_it_by_degree() {
    let at = |x: f64, seed: u64| {
        let mut s = walk::species();
        let trunk = &mut s.states[0];
        (trunk.relay, trunk.abortion, trunk.relay_at) = (1.0, 0.4, x);
        radii(&walk::tree(&s, seed).unwrap())
    };
    let mut failed = Vec::new();
    for seed in SEEDS {
        let x = |i: u32| f64::from(i) / f64::from(STEPS);
        let trees: Vec<_> = (0..=STEPS).map(|i| at(x(i), seed)).collect();
        let (i, _) = (0..STEPS)
            .map(|i| (i, girth_change(&trees[i as usize], &trees[i as usize + 1])))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap();
        let (mut from, mut to) = (x(i), x(i + 1));
        let first = girth_change(&at(from, seed), &at(to, seed));
        let mut last = first;
        for _ in 0..REFINE {
            let part = |k: u32| from + (to - from) * f64::from(k) / 8.0;
            let trees: Vec<_> = (0..=8).map(|k| at(part(k), seed)).collect();
            let (k, worst) = (0..8)
                .map(|k| (k, girth_change(&trees[k as usize], &trees[k as usize + 1])))
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();
            (from, to, last) = (part(k), part(k + 1), worst);
        }
        if first > 0.0 && last > first / 20.0 {
            failed.push(format!("seed {seed} at {to:.6}: {first:.4} to {last:.4}"));
        }
    }
    assert!(failed.is_empty(), "{failed:#?}");
}
