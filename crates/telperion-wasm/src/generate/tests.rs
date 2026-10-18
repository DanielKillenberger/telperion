//! The binding's tests: the diagnostics, the stages a field request runs,
//! and the field request's grid and limb order.
use super::*;
use telperion_core::tree::{Diagnostics, Node, Tree};

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
#[test]
fn surviving_diagnostics_match_radius_law_and_empty_state() {
    let mut tree = Tree {
        crossover: 2,
        ..Default::default()
    };
    for (i, parent) in [None, Some(0), Some(1), Some(1), Some(1), Some(1), Some(2)]
        .into_iter()
        .enumerate()
    {
        let mut node = Node::root();
        node.parent = parent;
        node.branch = i as u32;
        if i >= 2 {
            node.base_radius = [0.0025, 0.005, 0.02, 0.08, 0.0025][i - 2];
        }
        if i == 2 || i == 6 {
            node.kind = NodeKind::Twig;
        }
        tree.nodes.push(node);
    }
    let params = twigs::TwigParams {
        length_ratio: 0.5,
        ratio_power: 1.0,
        ..Default::default()
    };
    let (counts, handoffs, capped, twigs) = branch_diagnostics(&tree, params).unwrap();
    assert_eq!(handoffs, 4);
    assert_eq!(twigs, 2);
    assert_eq!(capped, 0);
    assert_eq!(counts.len(), params.generations as usize + 1);
    assert_eq!(counts, [1, 1, 0, 1, 0, 1, 0]);
    assert_eq!(
        branch_diagnostics(
            &tree,
            twigs::TwigParams {
                ratio_power: 0.0,
                ..params
            }
        )
        .unwrap()
        .2,
        3
    );
    tree.nodes.truncate(2);
    assert_eq!(
        branch_diagnostics(&tree, params).unwrap(),
        (vec![0; params.generations as usize + 1], 0, 0, 0)
    );
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
/// engine species (fn-198); a failure names the entry and the preset.
#[test]
fn every_shipped_preset_builds_through_the_main_entry() {
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
        let (_, meta) = generate(request)
            .unwrap_or_else(|e| panic!("entry telperion (main Wasm): preset {id}: {e}"));
        let leaves = meta["instances"].as_u64().unwrap_or(0);
        assert!(
            leaves > 0,
            "entry telperion (main Wasm): preset {id}: no leaves"
        );
    }
}
