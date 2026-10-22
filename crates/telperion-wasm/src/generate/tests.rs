//! The binding's tests: the diagnostics, the stages a field request runs,
//! and the field request's grid and limb order.
use super::*;
use telperion_core::tree::Diagnostics;

#[test]
fn a_reduced_tree_reports_the_detail_it_kept() {
    let reduced = Diagnostics {
        twig_detail: Some(14),
        ..Default::default()
    };
    assert_eq!(twig_detail(reduced), Some(14));
    // Incomplete is not reduced, and a full build kept everything.
    let incomplete = Diagnostics {
        node_capped: true,
        ..reduced
    };
    assert_eq!(twig_detail(incomplete), None);
    assert_eq!(twig_detail(Diagnostics::default()), None);
}
/// A field request reads the plan and places no leaf, and the metadata
/// says which stages ran and names the grid, for every shipped preset the
/// pipeline grows (R1); the date palm's fronds are planned frond by frond
/// (fn-150). A preset with no engine species is refused by name (fn-198).
#[test]
fn a_field_request_reads_the_plan_and_names_its_stages_for_every_preset() {
    for &(_, id, _, _) in params::CATALOGUE.iter() {
        let request = json!({"family": id, "outputs": {"field": {"n": 24}}});
        if params::by_identity(id).unwrap().space.species.is_none() {
            let refused = generate(request).err();
            assert!(
                matches!(refused, Some(Error::InvalidInput("no engine species"))),
                "{id}"
            );
            continue;
        }
        let (out, meta) = generate(request).unwrap_or_else(|e| panic!("{id}: {e}"));
        let stages = &meta["stages"];
        assert_eq!(stages["plan"], json!(true), "{id}: {stages}");
        assert_eq!(stages["placement"], json!(false), "{id}: {stages}");
        assert_eq!(stages["contacts"], json!(false), "{id}: {stages}");
        assert_eq!(stages["surface"], json!(false), "{id}: {stages}");
        assert_eq!(meta["leavesPlaced"], json!(0), "{id}");
        assert!(meta["leavesPlanned"].as_u64().unwrap() > 0, "{id}");
        assert_eq!(meta["field"]["n"], json!(24), "{id}");
        assert_eq!(out.field.occupancy.len(), 24 * 24 * 24, "{id}");
        assert_eq!(out.field.wood_radii.len(), 24 * 24 * 24, "{id}");
    }
    let (_, meta) = generate(json!({
        "family": "oregon-white-oak", "outputs": {"foliage": true, "field": {"n": 24}}
    }))
    .unwrap();
    assert_eq!(meta["stages"]["plan"], json!(true));
    assert_eq!(meta["stages"]["placement"], json!(true));
    assert_eq!(meta["biologicalUnits"], meta["instances"]);
}

/// R10: no limb order and the family's own answer byte for byte alike; a
/// shallower order parts the oak's crown into fewer systems; a malformed
/// request is refused by name, and a grid side outside 3 to 256 as the
/// core refuses it.
#[test]
fn the_field_request_selects_the_limb_order_and_defaults_to_the_family() {
    let family = params::by_identity("oregon-white-oak").unwrap();
    let field = |field: Value| {
        generate(json!({"family": "oregon-white-oak", "outputs": {"field": field}}))
            .unwrap()
            .0
            .field
    };
    let default = field(json!({"n": 8}));
    let order = family.canopy.clump_system_order;
    let same = field(json!({"n": 8, "limbOrder": order}));
    assert_eq!(default.limbs, same.limbs);
    assert_eq!(default.leaves, same.leaves);
    let limbs = |f: &FieldCells| {
        let held = f.limbs.iter().filter(|&&l| l != u32::MAX);
        held.collect::<std::collections::HashSet<_>>().len()
    };
    // The engine's oak at 40 has no system past its family's order, so a
    // shallower order parts it into fewer (fn-198: 41 at order 0, 144 at 2).
    let coarser = field(json!({"n": 8, "limbOrder": 0}));
    assert!(
        limbs(&coarser) < limbs(&default),
        "{} against {}",
        limbs(&coarser),
        limbs(&default)
    );
    assert!(default.wood_radii.iter().any(|&r| r > 0.0));
    for bad in [
        json!(true),
        json!({}),
        json!({"limbOrder": 1}),
        json!({"n": 8, "limbOrder": -1}),
        json!({"n": 8.5}),
        json!({"n": 8, "x": 1}),
        json!(1),
    ] {
        let error =
            generate(json!({"family": "oregon-white-oak", "outputs": {"field": bad}})).err();
        assert!(
            matches!(error, Some(Error::InvalidInput("field request"))),
            "{bad}: {error:?}"
        );
    }
    let side = generate(json!({"family": "oregon-white-oak", "outputs": {"field": {"n": 2}}}));
    assert!(
        matches!(
            side.err(),
            Some(Error::InvalidValue {
                field: "field grid side",
                ..
            })
        ),
        "the side"
    );
    let (out, meta) =
        generate(json!({"family": "oregon-white-oak", "outputs": {"field": false}})).unwrap();
    assert!(out.field.occupancy.is_empty() && meta["stages"]["field"] == json!(false));
}

/// Entry coverage (fn-151 R2): every shipped preset builds its wood and its
/// leaves through the main binding, or is refused by name where it has no
/// engine species (fn-198); a failure names the entry and the preset. A
/// preset that grows no flowers hands empty organ slots and a null organ
/// reference, and only Telperion and Laurelin flower (fn-294 R1, read here
/// so no preset is built twice; host, fn-304 ruling 3).
#[test]
fn every_shipped_preset_builds_through_the_main_entry() {
    let mut flowering = Vec::new();
    for &(_, id, _, _) in params::CATALOGUE.iter() {
        let request = json!({"family": id, "outputs": {"surface": true, "foliage": true}});
        if params::by_identity(id).unwrap().space.species.is_none() {
            let refused = generate(request).err();
            assert!(
                matches!(refused, Some(Error::InvalidInput("no engine species"))),
                "{id}"
            );
            continue;
        }
        let (out, meta) = generate(request)
            .unwrap_or_else(|e| panic!("entry telperion (main Wasm): preset {id}: {e}"));
        let leaves = meta["instances"].as_u64().unwrap_or(0);
        assert!(
            leaves > 0,
            "entry telperion (main Wasm): preset {id}: no leaves"
        );
        chunks_read_whole(&out, &meta, id);
        if meta["organReference"] == Value::Null {
            assert!(
                [27, 28, 29].iter().all(|&n| crate::slot(&out, n).1 == 0),
                "{id}: organ slots without an organ"
            );
        } else {
            flowering.push(id);
        }
    }
    assert_eq!(flowering, FLOWERING);
}

/// The shipped presets the pipeline grows flowers for.
const FLOWERING: [&str; 2] = ["telperion", "laurelin"];

/// The metadata counts the wood's chunks and the accessor hands each one's
/// positions, oct normals and indices whole, and nothing past the last.
fn chunks_read_whole(out: &Output, meta: &Value, id: &str) {
    let count = meta["surfaceChunks"].as_u64().unwrap() as u32;
    assert!(count > 0, "{id}: no wood chunk");
    for chunk in 0..count {
        let [positions, normals, indices] = [0, 1, 2].map(|f| crate::chunk_field(out, chunk, f).1);
        let vertices = positions / 3;
        assert!(vertices <= telperion_core::surface::CHUNK_VERTICES, "{id}");
        assert_eq!(normals, vertices * 2, "{id}");
        assert!(indices > 0 && indices % 3 == 0, "{id}");
    }
    assert_eq!(crate::chunk_field(out, count, 0).1, 0, "{id}");
    assert_eq!(crate::chunk_field(out, 0, 3).1, 0, "{id}");
}

/// The flowers cross the binding (fn-294 R1): where the pipeline grows an
/// organ for a shipped preset, its element, its florets and their reference
/// reach the output slots and the metadata whole; where no foliage is asked
/// for, the slots are empty and the reference is null. The presets that grow
/// none are read by `every_shipped_preset_builds_through_the_main_entry`.
#[test]
fn flowers_cross_the_binding_where_the_pipeline_grows_them() {
    let words = telperion_core::foliage::WORDS;
    for id in FLOWERING {
        let f = params::by_identity(id).unwrap();
        let request = pipeline::Request {
            leaves: true,
            ..Default::default()
        };
        let organ = pipeline::build(&f, request).unwrap().outputs.organ;
        let (out, meta) = generate(json!({"family": id, "outputs": {"foliage": true}})).unwrap();
        let len = |n| crate::slot(&out, n).1;
        let organ = organ.unwrap_or_else(|| panic!("{id}: the pipeline grows no organ"));
        let florets = organ.instances.leaves.len();
        assert!(florets > 0, "{id}: an organ with no floret");
        assert_eq!(len(27), organ.element.positions.len() * 3, "{id}");
        assert_eq!(len(28), organ.element.indices.len(), "{id}");
        assert_eq!(len(29), florets * words, "{id}");
        let r = organ.instances.reference;
        let expected = json!({"min": [r.min.x, r.min.y, r.min.z],
            "extent": [r.extent.x, r.extent.y, r.extent.z]});
        assert_eq!(meta["organReference"], expected, "{id}");
        let (bare, meta) = generate(json!({"family": id, "outputs": {"surface": true}})).unwrap();
        assert!(
            [27, 28, 29].iter().all(|&n| crate::slot(&bare, n).1 == 0),
            "{id}"
        );
        assert_eq!(meta["organReference"], Value::Null, "{id}");
    }
}

/// A flowering window of no length flowers never (`/bloom/lasts` zero,
/// natural and supernatural): the binding hands no organ, not an empty one.
#[test]
fn a_window_of_no_length_crosses_no_organ() {
    let mut family = params::metadata(&params::by_identity("laurelin").unwrap());
    family["bloom"]["lasts"] = json!(0.0);
    family["supernatural"]["bloom"]["lasts"] = json!(0.0);
    let (out, meta) = generate(json!({"family": family, "outputs": {"foliage": true}})).unwrap();
    assert!([27, 28, 29].iter().all(|&n| crate::slot(&out, n).1 == 0));
    assert_eq!(meta["organReference"], Value::Null);
}
