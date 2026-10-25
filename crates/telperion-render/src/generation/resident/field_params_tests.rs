//! fn-317 R6: the device field takes a tree's parameters. Every shipped
//! preset, at seeds 1 and 5678, grows from its parameters as the binding
//! reads them (`params::parse` of the wire) to the same answers byte for
//! byte as from the preset the catalogue names, on a grid of 24.
use super::field_tests::{device, family, generator, references};
use telperion_core::field::Grid;
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

/// fn-317 R4c, a measurement: every preset's named field at seed 1 on 24
/// and 36 cells, on the CPU and on the device, written as JSON to the
/// directory `FIELD_DUMP` names, for the voxelizer to read in Node
/// (`.flow/evidence/fn-317-*/cubes.test.ts`).
#[test]
#[ignore = "a measurement: writes both executors' named fields"]
fn every_preset_s_named_fields_are_written_for_the_voxelizer() {
    let _held = super::super::whole_tree();
    let (Some(gen), Ok(dir)) = (generator(), std::env::var("FIELD_DUMP")) else {
        return;
    };
    let json = |g: &Grid| {
        let c = &g.cells;
        serde_json::json!({
            "n": g.frame.n, "cell": g.frame.cell,
            "origin": [g.frame.origin.x, g.frame.origin.y, g.frame.origin.z],
            "flags": c.iter().map(|a| u8::from(a.wood) | u8::from(a.foliage) << 1 | u8::from(a.flowers > 0.0) << 2).collect::<Vec<_>>(),
            "woodRadius": c.iter().map(|a| a.wood_radius as f32).collect::<Vec<_>>(),
            "leaves": c.iter().map(|a| a.leaves as f32).collect::<Vec<_>>(),
            "limbs": c.iter().map(|a| a.limb.unwrap_or(u32::MAX)).collect::<Vec<_>>(),
            "flowers": c.iter().map(|a| a.flowers as f32).collect::<Vec<_>>(),
        })
    };
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        let f = family(id, 1);
        for mut cpu in references(&f, &[24, 36], None) {
            let mut dev = device(&gen, &f, &cpu, None);
            cpu.name_limbs();
            dev.name_limbs();
            let out = serde_json::json!({"cpu": json(&cpu), "device": json(&dev)});
            let path = format!("{dir}/{id}-{}.json", cpu.frame.n);
            std::fs::write(&path, out.to_string()).expect("written");
        }
    }
}
