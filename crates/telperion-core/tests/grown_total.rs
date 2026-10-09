//! fn-238 R2: the blend's size solve reads the grown phytomers as one
//! running total (`expected_grown`), on the first unit and the rest of the
//! axis as the wood is, not by summing the per-cycle count table. It is the
//! same expectation summed in another order, so it equals the table's sum
//! within the solve's own tolerance, 1e-12 of the log.
use telperion_core::presets::Preset;
use telperion_space::{blend, expected_counts, expected_grown, Species};

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
    let mut cases: Vec<(String, Species)> = presets
        .iter()
        .map(|&p| (format!("{p:?}"), species(p)))
        .collect();
    let (spruce, ordinary) = (species(Preset::NorwaySpruce), species(Preset::Ordinary));
    for t in [0.25, 0.5] {
        cases.push((
            format!("spruce to ordinary at {t}"),
            blend(&spruce, &ordinary, t).unwrap(),
        ));
    }
    for (name, s) in &cases {
        for age in [1, 2, 7, 40] {
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
