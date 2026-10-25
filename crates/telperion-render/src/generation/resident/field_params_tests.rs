//! fn-317 R6: the device field takes a tree's parameters. Every shipped
//! preset, at seeds 1 and 5678, grows from its parameters as the binding
//! reads them (`params::parse` of the wire) to the same answers byte for
//! byte as from the preset the catalogue names, on a grid of 24.
use super::field_tests::{family, generator};
use telperion_core::params;

#[test]
fn every_preset_grows_its_field_from_its_parameters_as_from_its_id() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        for seed in [1, 5678] {
            let named = family(id, seed);
            let wire = params::metadata(&named);
            let parsed = params::parse(&wire).unwrap_or_else(|e| panic!("{id}: {e}"));
            let grow = |f| pollster::block_on(gen.grow_field(f, 24, None));
            match (grow(&named), grow(&parsed)) {
                (Ok(a), Ok(b)) => assert!(a == b, "{id} {seed}: the parameters grew another field"),
                (Err(a), Err(b)) => assert_eq!(a.to_string(), b.to_string(), "{id} {seed}"),
                (a, b) => panic!("{id} {seed}: {:?} against {:?}", a.err(), b.err()),
            }
        }
    }
}
