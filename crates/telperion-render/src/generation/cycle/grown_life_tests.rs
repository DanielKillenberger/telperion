//! A stem whose lifespan passes 32 bits of ticks (2^20 a cycle) grows on
//! the device as on the CPU (the validator allows a million cycles); and
//! a request older than the device's accumulated time holds is refused
//! before any dispatch.
use super::build::Tables;
use super::build_grow::{grow_now as grow, Sizes};
use super::grown_tests::{device, grown, grown_from, held_to_cpu};
use telperion_core::presets::Preset;

#[test]
fn long_lived_stems_grow_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let family = Preset::EuropeanBeech.parameters();
    let mut differ = Vec::new();
    for life in [2048.0, 4096.0, 1_000_000.0] {
        for age in [1, 3] {
            let mut species = family.space.species.clone().expect("an engine species");
            species.states[0].lifespan = life;
            let g = grown_from(species, family.space.light, (age, 42));
            print!("stem of {life} cycles, age {age}: ");
            if !held_to_cpu(&gpu, &kit, &g, age).is_empty() {
                differ.push((life, age));
            }
        }
    }
    assert!(differ.is_empty(), "the device's trees are apart from the CPU's: {differ:?}");
}

/// A palm's stem that never dies nor moves on, grown to either side of
/// its lifespan: the age ends where the time left in it does.
#[test]
fn a_stem_ends_at_its_lifespan_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let family = Preset::DatePalm.parameters();
    let mut differ = Vec::new();
    for (life, ages) in [(1024, [1023, 1025]), (2047, [2046, 2047])] {
        for age in ages {
            let mut species = family.space.species.clone().expect("an engine species");
            (species.states[0].lifespan, species.states[0].viability, species.states[0].continuation) = (f64::from(life), 1.0, 0.0);
            let g = grown_from(species, family.space.light, (age, 42));
            print!("stem of {life} cycles, age {age}: ");
            if !held_to_cpu(&gpu, &kit, &g, age).is_empty() {
                differ.push((life, age));
            }
        }
    }
    assert!(differ.is_empty(), "the device's trees are apart from the CPU's: {differ:?}");
}

#[test]
fn a_tree_older_than_the_device_holds_is_refused() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let g = grown(Preset::EuropeanBeech, 3, 42);
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    let refused = grow(&gpu, &kit, &t, &g.tables, (Sizes::least(), super::build_grow::MOST_CYCLES));
    let error = refused.err().expect("refused").to_string();
    assert!(error.contains("2048 cycles"), "{error}");
}
