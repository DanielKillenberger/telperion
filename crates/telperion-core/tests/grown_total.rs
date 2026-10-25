//! fn-238 R2: the blend's size solve reads the grown phytomers as one
//! running total (`expected_grown`), on the first unit and the rest of the
//! axis as the wood is, not by summing the per-cycle count table. It is the
//! same expectation summed in another order, so it equals the table's sum
//! within the solve's own tolerance, 1e-12 of the log.
use telperion_core::presets::Preset;
use telperion_space::{blend, expected_axes, expected_counts, expected_grown, Species};

fn species(preset: Preset) -> Species {
    let mut s = preset.parameters().space.species.expect("an engine preset");
    for st in &mut s.states {
        (
            st.shade_hazard,
            st.shade_size,
            st.balance_hazard,
            st.leaf_girth,
        ) = (0.0, 0.0, 0.0, 0.0);
    }
    s
}

fn summed(species: &Species, age: u32) -> f64 {
    let counts = expected_counts(species, age).unwrap();
    (0..species.states.len()).map(|pa| counts.total(pa)).sum()
}

/// The four branching presets, the corners and the ordinary tree, and the
/// engine's raw walk between the spruce and the ordinary tree (whose rows
/// carry every corner's features at once), from a first year to 40.
#[test]
fn the_running_total_is_the_count_tables_sum() {
    let presets = [
        Preset::EuropeanBeech,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Ordinary,
    ];
    let (spruce, ordinary) = (species(Preset::NorwaySpruce), species(Preset::Ordinary));
    for age in [1, 2, 7, 40] {
        // The walk's species is the one a tree of that age reads (fn-308).
        let mut cases: Vec<(String, Species)> = presets
            .iter()
            .map(|&p| (format!("{p:?}"), species(p)))
            .collect();
        for t in [0.25, 0.5] {
            cases.push((
                format!("spruce to ordinary at {t}"),
                blend(&spruce, &ordinary, t, age).unwrap(),
            ));
        }
        for (name, s) in &cases {
            let (total, sum) = (expected_grown(s, age).unwrap(), summed(s, age));
            let off = (total.ln() - sum.ln()).abs();
            assert!(
                off <= 1e-12,
                "{name} at {age}: {total} against {sum}, {off:e}"
            );
        }
    }
}

/// The age is refused as the count table refuses it.
#[test]
fn a_tree_of_no_cycles_is_refused() {
    assert!(expected_grown(&species(Preset::EuropeanBeech), 0).is_err());
}

/// fn-256 task 6: the expected axes (the device's axis tables' hint). An
/// axis that sprouts no bud, relays none and carries on none is the tree's
/// one axis at any age; a species' buds add to it as it ages.
#[test]
fn the_expected_axes_count_the_buds_that_live() {
    let mut lone = species(Preset::EuropeanBeech);
    for st in &mut lone.states {
        // An age it passes through moves on into the next it lives in.
        if st.lifespan > 0.0 {
            st.continuation = 0.0;
        }
        (st.relay, st.relay_ended, st.relay_failed) = (0.0, 0.0, 0.0);
        for z in &mut st.zones {
            z.lateral
                .iter_mut()
                .chain(z.dormant.iter_mut())
                .for_each(|p| *p = 0.0);
        }
    }
    for age in [1, 7, 40] {
        assert_eq!(expected_axes(&lone, age).unwrap(), 1.0, "at {age}");
    }
    let beech = species(Preset::EuropeanBeech);
    let (young, old) = (
        expected_axes(&beech, 7).unwrap(),
        expected_axes(&beech, 40).unwrap(),
    );
    assert!(1.0 < young && young < old, "{young} then {old}");
    assert!(expected_axes(&beech, 0).is_err());
}
