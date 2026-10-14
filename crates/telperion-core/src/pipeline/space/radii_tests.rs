//! fn-281: the Oregon white oak at 80, seed 7, refused with "unsolved
//! radii" (fn-236 SHEET-4). Both trees take about 4 GB and 25 s, so they
//! are run by name: `cargo test --profile ci -p telperion-core radii_tests
//! -- --ignored --nocapture`.
use super::*;
use crate::pipeline::executor;
use crate::presets::Preset;

fn oak(age: f64, seed: u32) -> crate::Family {
    let mut family = Preset::OregonWhiteOak.parameters();
    family.age = age;
    family.skeleton.seed = seed;
    family
}

/// R1: the shipped path's tree, held to the check every later stage makes.
#[test]
#[ignore = "fn-281: red until R3; about 4 GB and 25 s"]
fn the_oak_at_80_seed_7_solves_every_radius() {
    let grown = executor::grow(&oak(80.0, 7)).unwrap();
    let x = grown.expansion().unwrap();
    x.tree().validate_solved().unwrap();
}

/// R2: every relay join that leaves no wood above it, the stopped module's
/// head, with the terms its girth was summed from (`girth::thicken_given`
/// sets `Join::above` to the f32 section at the join less the relay's
/// share there), and the node the conversion draws from that phytomer.
#[test]
#[ignore = "fn-281 R2 diagnostic; about 5 GB and 30 s"]
fn the_joins_the_oak_at_80_seed_7_draws_no_wood_above() {
    let family = oak(80.0, 7);
    let species = family.space.species.clone().unwrap();
    let request = Request {
        age: 80,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let tree = convert(&structure, (&family.space.trunk, &pa_rows(&species)));
    let by_identity: std::collections::HashMap<_, _> = (tree.nodes.iter().enumerate())
        .map(|(i, n)| (n.identity, i))
        .collect();
    let (mut none, mut thick, mut absorbed, mut drawn) = (0, 0, 0, 0);
    for (c, relay) in structure.axes.iter().enumerate() {
        let (Origin::Relay { parent, .. }, Some(j)) = (relay.origin, relay.joined()) else {
            continue;
        };
        if relay.phytomers.is_empty() || j.above > 0.0 {
            continue;
        }
        none += 1;
        let p = &structure.axes[parent];
        let state = &species.states[p.pa];
        let k = &p.phytomers[j.node];
        // Only a phytomer the conversion keeps (`FINEST`) is drawn.
        if k.radius < FINEST {
            continue;
        }
        thick += 1;
        // The phytomer's own pipe before its leaf term, ripened, and the
        // relay's share at the join, as f32 sections: whether the sum
        // `section` absorbs the own wood whole.
        let years = (structure.age.saturating_sub(k.cycle) + 1) as f64;
        let ripe = if state.form.ripening > 0.0 {
            (years / state.form.ripening).min(1.0)
        } else {
            1.0
        };
        let own = state.form.pipe * k.scale * ripe;
        let e = state.form.exponent;
        let at = (relay.phytomers[0].radius.powf(e) * f64::from(j.t)) as f32;
        let own_s = own.powf(e) as f32;
        let lost = (at + own_s) - at == 0.0;
        absorbed += usize::from(lost);
        let node = by_identity
            .get(&crate::tree::NodeIdentity::lineage(k.key()))
            .copied();
        let radius = node.map(|i| tree.nodes[i].radius);
        drawn += usize::from(radius == Some(0.0));
        println!(
            "relay {c} on {parent} (pa {}, leafGirth {}, secondary {}): join node {} of {} \
             t {} below {:e} above {:e}; phytomer radius {:e} scale {:e} cycle {}; \
             own section before leaf {own_s:e} against the relay's {at:e}: absorbed in f32 {lost}; \
             drawn as node {node:?} radius {radius:?}",
            p.pa,
            state.leaf_girth,
            state.form.secondary,
            j.node,
            p.phytomers.len(),
            j.t,
            j.below,
            j.above,
            k.radius,
            k.scale,
            k.cycle,
        );
    }
    println!(
        "{none} joins leave no wood above; {thick} on a phytomer the conversion keeps, \
         {absorbed} of them with the own wood absorbed in f32; {drawn} drawn at radius zero"
    );
    let zero: Vec<usize> = (0..tree.nodes.len())
        .filter(|&i| tree.nodes[i].radius <= 0.0)
        .collect();
    println!("converted nodes at radius zero: {zero:?}");
    // Wood the conversion draws thinner than it keeps (`FINEST`): only the
    // own draw above an inside join is clamped below the phytomer's radius.
    let fine = (tree.nodes.iter())
        .filter(|n| n.radius > 0.0 && n.radius < FINEST)
        .count();
    println!("converted nodes thinner than FINEST, above zero: {fine}");
    assert_eq!(none, 0);
}
