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

/// R4: where the nodes drawn thinner than `FINEST` come from, by what
/// set their radius below the phytomer's own: a relay's join drawn at its
/// `Join::below`, a head clamped to a join's `above`, or a head `aside`
/// scaled down to `FORK_FROM` of its carrier.
#[test]
#[ignore = "fn-281 R4 count; about 5 GB and 30 s"]
fn where_the_oak_at_80_seed_7_draws_wood_thinner_than_finest() {
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
    // Each phytomer's radius and the least `above` of the joins inside its
    // span, by its node identity.
    let mut grown = std::collections::HashMap::new();
    for axis in &structure.axes {
        for p in &axis.phytomers {
            grown.insert(NodeIdentity::lineage(p.key()), (p.radius, f64::INFINITY));
        }
    }
    for relay in &structure.axes {
        let (Origin::Relay { parent, .. }, Some(j)) = (relay.origin, relay.joined()) else {
            continue;
        };
        let inside = j.t > 0.0 && j.t < join::ONE && !relay.phytomers.is_empty();
        if let (true, Some(p)) = (inside, structure.axes[parent].phytomers.get(j.node)) {
            let held = grown.get_mut(&NodeIdentity::lineage(p.key())).unwrap();
            held.1 = held.1.min(j.above);
        }
    }
    let (mut joins, mut clamped, mut scaled, mut other) = (0, 0, 0, 0);
    for n in tree.nodes.iter().skip(1).filter(|n| n.radius < FINEST) {
        match grown.get(&n.identity) {
            None => joins += 1,
            Some(&(_, above)) if n.radius == above => clamped += 1,
            Some(&(r, _)) if n.radius < r && n.shoot.bud_fate == BudFate::Lateral => scaled += 1,
            Some(_) => other += 1,
        }
    }
    println!(
        "nodes thinner than FINEST: {} ({joins} joins, {clamped} clamped to a join's above, \
         {scaled} heads scaled aside, {other} other)",
        joins + clamped + scaled + other
    );
    assert_eq!(clamped, 0, "a head drawn thinner than FINEST above a join");
}
