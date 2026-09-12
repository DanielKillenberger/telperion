use super::tests::bytes;
use super::*;
use crate::{
    blend,
    growth::MAX_AGE,
    presets::{Family, Preset},
};

fn seedling(preset: Preset) -> (Family, Specimen) {
    let mut f = preset.parameters();
    f.age = 0.0;
    let s = Specimen::build(&f).unwrap();
    (f, s)
}

#[test]
fn monthly_growth_expands_the_envelope_and_retains_wood() {
    let (_, mut s) = seedling(Preset::OregonWhiteOak);
    let mut previous = Vec::new();
    let mut height = 0.0;
    let mut after_twigs = false;
    for _ in 0..360 {
        let crossover = s.tree().crossover;
        let had_twigs = s.tree().nodes.iter().any(|n| n.kind == NodeKind::Twig);
        s.advance(1.0 / 12.0).unwrap();
        s.tree().validate_solved().unwrap();
        assert!(s.envelope().height >= height);
        assert!(s.tree().nodes[..s.tree().crossover]
            .iter()
            .all(|n| n.kind == NodeKind::Structural));
        assert!(s.tree().nodes[s.tree().crossover..]
            .iter()
            .all(|n| n.kind != NodeKind::Structural));
        for (id, pos, parent, branch) in previous {
            let n = s.node(id).unwrap();
            assert_eq!(n.position, pos);
            assert_eq!(
                n.parent.map(|p| s.tree().nodes[p as usize].identity),
                parent
            );
            assert_eq!(s.tree().nodes[n.branch as usize].identity, branch);
        }
        previous = s
            .tree()
            .nodes
            .iter()
            .map(|n| {
                (
                    n.identity,
                    n.position,
                    n.parent.map(|p| s.tree().nodes[p as usize].identity),
                    s.tree().nodes[n.branch as usize].identity,
                )
            })
            .collect();
        after_twigs |= had_twigs && s.tree().crossover > crossover;
        height = s.envelope().height;
    }
    assert!(s.tree().crossover > 10, "monthly budget must append wood");
    assert!(
        after_twigs,
        "new structure must arrive after local wood exists"
    );
}

#[test]
fn monthly_build_and_irregular_fractional_replay_have_identical_bytes() {
    let presets = [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Telperion,
        Preset::Laurelin,
    ];
    let mut families: Vec<_> = presets.into_iter().map(|p| p.parameters()).collect();
    for pair in families.clone().windows(2) {
        families.push(blend::families(&pair[0], &pair[1], 0.37).unwrap());
    }
    for mut f in families {
        f.age = 12.25;
        let fresh = Specimen::build(&f).unwrap();
        f.age = 0.0;
        let mut replay = Specimen::build(&f).unwrap();
        // Repeated decimal pieces, each smaller than a month, cross many slice
        // boundaries. A loop that discards each call's remainder fails here.
        for _ in 0..24 {
            for part in [0.01, 0.02, 0.07, 0.1, 0.3] {
                replay.advance(part).unwrap();
            }
        }
        replay.advance(0.25).unwrap();
        assert_eq!(replay.age(), fresh.age());
        assert!(
            fresh.tree.nodes.len() > 1,
            "the test must grow, not compare empty trees"
        );
        assert!(
            bytes(&fresh.tree) == bytes(&replay.tree),
            "fresh/replay bytes differ"
        );
        // Equal remainder must continue across another call boundary as well.
        let mut whole = fresh;
        whole.advance(0.2).unwrap();
        replay.advance(0.03).unwrap();
        replay.advance(0.17).unwrap();
        assert!(bytes(&whole.tree) == bytes(&replay.tree));
    }
}

#[test]
fn monthly_pause_and_submonth_remainder_do_not_grow_early() {
    let (_, mut s) = seedling(Preset::Ordinary);
    let before = bytes(&s.tree);
    s.advance(0.0).unwrap();
    s.advance(0.04).unwrap();
    assert_eq!(s.age(), 0.04);
    assert_eq!(before, bytes(&s.tree));
    s.advance(0.04).unwrap();
    assert_eq!(before, bytes(&s.tree));
    s.advance(0.01).unwrap();
    assert!(s.envelope().height > 0.0);
    assert_eq!(s.timeline.as_ref().unwrap().age.month, 1);
    assert_eq!(s.timeline.as_ref().unwrap().age.remainder, 80_000_000);
}

#[test]
fn monthly_time_errors_name_values_and_preserve_the_specimen() {
    let (mut f, mut s) = seedling(Preset::Ordinary);
    s.advance(2.0).unwrap();
    let before = bytes(&s.tree);
    for value in [-0.1, f64::NAN, f64::INFINITY, MAX_AGE + 1.0] {
        let message = s.advance(value).unwrap_err().to_string();
        assert!(message.contains("advance"), "{message}");
        assert!(message.contains(&value.to_string()), "{message}");
        assert_eq!(s.age(), 2.0);
        assert_eq!(bytes(&s.tree), before);
        f.age = value;
        let message = match Specimen::build(&f) {
            Ok(_) => panic!("accepted {value}"),
            Err(e) => e.to_string(),
        };
        assert!(
            message.contains("age") && message.contains(&value.to_string()),
            "{message}"
        );
    }
    let message = s.advance(MAX_AGE).unwrap_err().to_string();
    assert!(message.contains("age") && message.contains("1000002"));
    s.advance(1.0).unwrap();
    assert_eq!(s.age(), 3.0);
}

#[test]
fn monthly_node_cap_rolls_back_a_partial_slice_and_resumes_after_raise() {
    let (_, mut s) = seedling(Preset::OregonWhiteOak);
    s.advance(10.0).unwrap();
    // Find a month with multiple births so the test actually stops mid-slice.
    let mut full = s.clone();
    loop {
        full.advance(1.0 / 12.0).unwrap();
        if full.tree.nodes.len() > s.tree.nodes.len() + 1 {
            break;
        }
        s = full.clone();
        assert!(s.age() < 30.0);
    }
    let old = bytes(&s.tree);
    let old_age = s.age();
    s.set_node_ceiling(full.tree.nodes.len() - 1).unwrap();
    s.advance(1.0 / 12.0).unwrap();
    assert!(s.tree.diagnostics.node_capped);
    assert_eq!(s.age(), old_age);
    let mut without_diagnostic = s.tree.clone();
    without_diagnostic.diagnostics.node_capped = false;
    assert_eq!(bytes(&without_diagnostic), old);
    assert_eq!(
        s.advance(1.0),
        Err(Error::ResourceLimit("node ceiling reached"))
    );
    s.set_node_ceiling(NODE_CEILING).unwrap();
    s.advance(1.0 / 12.0).unwrap();
    assert!(
        bytes(&s.tree) == bytes(&full.tree),
        "rollback must restore streams, identities and spent attractors"
    );
}

#[test]
fn monthly_frontier_storage_permutations_do_not_change_birth_order() {
    let (_, mut a) = seedling(Preset::OregonWhiteOak);
    a.advance(20.0).unwrap();
    let mut b = a.clone();
    b.local.reverse_for_test();
    b.scaffold.reverse_for_test();
    a.advance(1.0).unwrap();
    b.advance(1.0).unwrap();
    assert!(bytes(&a.tree) == bytes(&b.tree));
}

#[test]
fn monthly_growth_traits_and_age_are_numeric_family_dimensions() {
    let mut a = Preset::Ordinary.parameters();
    let mut b = a.clone();
    a.age = 5.0;
    b.age = 15.0;
    a.growth.rate = 0.04;
    b.growth.rate = 0.12;
    a.growth.shape = 1.0;
    b.growth.shape = 3.0;
    let mid = blend::families(&a, &b, 0.5).unwrap();
    assert_eq!(mid.age, 10.0);
    assert_eq!(mid.growth.rate, 0.08);
    assert_eq!(mid.growth.shape, 2.0);
    let young = Specimen::build(&a).unwrap();
    let older = Specimen::build(&b).unwrap();
    assert!(young.envelope().height < older.envelope().height);
    for (rate, shape, field) in [(0.0, 2.0, "rate"), (0.1, 0.0, "shape")] {
        a.growth.rate = rate;
        a.growth.shape = shape;
        let error = match Specimen::build(&a) {
            Ok(_) => panic!("invalid traits accepted"),
            Err(e) => e,
        };
        assert!(error.to_string().contains(field));
    }
}

#[test]
fn monthly_saturated_age_jumps_without_visiting_the_intervening_months() {
    let (mut f, _) = seedling(Preset::NorwaySpruce);
    let mature = f.growth.mature_month();
    f.age = mature as f64 / 12.0;
    let start = std::time::Instant::now();
    let mut s = Specimen::build(&f).unwrap();
    let build = start.elapsed();
    let before = bytes(&s.tree);
    let start = std::time::Instant::now();
    s.advance(MAX_AGE - f.age).unwrap();
    let jump = start.elapsed();
    assert!(
        jump < build / 10 + std::time::Duration::from_millis(5),
        "saturated jump {jump:?}; build {build:?}"
    );
    assert_eq!(s.age(), MAX_AGE);
    assert_eq!(bytes(&s.tree), before);
    f.age = MAX_AGE;
    let start = std::time::Instant::now();
    let old = Specimen::build(&f).unwrap();
    let past = start.elapsed();
    assert!(
        past < build * 3 + std::time::Duration::from_millis(50),
        "past {past:?}; mature {build:?}"
    );
    assert!(bytes(&old.tree) == before);
}

#[test]
fn monthly_zero_ceiling_keeps_an_empty_seedling_and_recovers() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 1.0;
    f.skeleton.growth.max_nodes = Some(0);
    let mut s = Specimen::build(&f).unwrap();
    assert!(s.tree.nodes.is_empty());
    assert!(s.tree.diagnostics.node_capped);
    assert_eq!(s.age(), 0.0);
    assert_eq!(
        s.advance(1.0),
        Err(Error::ResourceLimit("node ceiling reached"))
    );
    s.set_node_ceiling(NODE_CEILING).unwrap();
    s.advance(1.0).unwrap();
    f.skeleton.growth.max_nodes = None;
    let fresh = Specimen::build(&f).unwrap();
    assert_eq!(bytes(&s.tree), bytes(&fresh.tree));
}

#[test]
fn monthly_cached_local_runs_obey_the_current_crown_boundary() {
    let (_, mut s) = seedling(Preset::OregonWhiteOak);
    for _ in 0..360 {
        let first_birth = s.next_identity;
        s.advance(1.0 / 12.0).unwrap();
        for node in &s.tree().nodes[s.tree().crossover..] {
            if node.identity.birth_order() >= first_birth {
                assert!(
                    s.envelope().contains(node.position, 1e-9),
                    "local birth {} outside current crown at age {}: {:?}",
                    node.identity.birth_order(),
                    s.age(),
                    node.position
                );
            }
        }
    }
}

#[test]
fn monthly_blends_preserve_valid_age_and_rate_boundaries() {
    for rate in [0.001, 10.0] {
        let mut a = Preset::Ordinary.parameters();
        a.age = MAX_AGE;
        a.growth.rate = rate;
        for t in [0.009, 0.059, 0.063, 0.37] {
            let blended = blend::families(&a, &a, t).unwrap();
            assert_eq!(blended.age, a.age, "equal endpoint age drifted at {t}");
            assert_eq!(
                blended.growth.rate, rate,
                "equal endpoint rate drifted at {t}"
            );
            assert!(blended.growth.validate().is_ok());
            assert!(crate::growth::Age::from_years(blended.age).is_ok());
        }
    }
}
