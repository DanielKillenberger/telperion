//! The binding's tests: the diagnostics, the stages a field request runs,
//! and the field request's limb order.
use super::*;
use telperion_core::tree::{Node, Tree};
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

/// A field request places no leaf where the family has a plan, and the
/// metadata says which stages ran, for every shipped preset (R1). The
/// beech, with short shoots and limb clumping, keeps the placed path; the
/// date palm's fronds are planned as ribbons (fn-150).
#[test]
fn a_field_request_reads_the_plan_and_names_its_stages_for_every_preset() {
    for &(_, id, _, _) in params::CATALOGUE.iter() {
        let (_, meta) = generate(json!({"family": id, "outputs": {"field": true}}))
            .unwrap_or_else(|e| panic!("{id}: {e}"));
        let stages = &meta["stages"];
        let planned = id != "european-beech";
        assert_eq!(stages["plan"], json!(planned), "{id}: {stages}");
        assert_eq!(stages["placement"], json!(!planned), "{id}: {stages}");
        assert_eq!(stages["cull"], json!(!planned), "{id}: {stages}");
        assert_eq!(stages["contacts"], json!(false), "{id}: {stages}");
        assert_eq!(stages["surface"], json!(false), "{id}: {stages}");
        assert_eq!(
            stages["fieldSource"],
            json!(if planned { "plan" } else { "placed" }),
            "{id}: {stages}"
        );
        if planned {
            assert_eq!(meta["leavesPlaced"], json!(0), "{id}");
            assert!(meta["leavesPlanned"].as_u64().unwrap() > 0, "{id}");
            assert_eq!(meta["biologicalUnits"], Value::Null, "{id}");
        } else {
            // The placed path grows its field from the leaves it placed.
            assert!(meta["leavesPlaced"].as_u64().unwrap() > 0, "{id}");
        }
    }
    let (_, meta) = generate(json!({
        "family": "oregon-white-oak", "outputs": {"foliage": true, "field": true}
    }))
    .unwrap();
    assert_eq!(meta["stages"]["plan"], json!(true));
    assert_eq!(meta["stages"]["placement"], json!(true));
    assert_eq!(meta["stages"]["fieldSource"], json!("plan"));
    assert_eq!(meta["biologicalUnits"], meta["instances"]);
}

/// R10: `"field": true` and the family's own order answer byte for byte
/// alike; a deeper order parts the oak's crown into more systems; a
/// malformed order is refused by name.
#[test]
fn the_field_request_selects_the_limb_order_and_defaults_to_the_family() {
    let family = params::by_identity("oregon-white-oak").unwrap();
    let answers = |field: Value| {
        let (out, _) =
            generate(json!({"family": "oregon-white-oak", "outputs": {"field": field}})).unwrap();
        let f = out.field.unwrap();
        let b = f.bounds().unwrap();
        let mut answers = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                for k in 0..8 {
                    let at = |lo: f64, hi: f64, n: usize| lo + (hi - lo) * (n as f64 + 0.5) / 8.0;
                    let c = Vec3::new(
                        at(b.min.x, b.max.x, i),
                        at(b.min.y, b.max.y, j),
                        at(b.min.z, b.max.z, k),
                    );
                    answers.push(f.query(c, (b.max.y - b.min.y) / 16.0).unwrap());
                }
            }
        }
        answers
    };
    let default = answers(json!(true));
    let order = family.canopy.clump_system_order;
    assert_eq!(default, answers(json!({"limbOrder": order})));
    let limbs = |a: &[telperion_core::field::Occupancy]| {
        a.iter()
            .filter_map(|o| o.limb)
            .collect::<std::collections::HashSet<_>>()
            .len()
    };
    let finer = answers(json!({"limbOrder": order + 3}));
    assert!(
        limbs(&finer) > limbs(&default),
        "{} against {}",
        limbs(&finer),
        limbs(&default)
    );
    assert!(default.iter().any(|o| o.wood && o.wood_radius > 0.0));
    for bad in [
        json!({}),
        json!({"limbOrder": -1}),
        json!({"limbOrder": 1.5}),
        json!({"limbOrder": 1, "x": 1}),
        json!(1),
    ] {
        let error =
            generate(json!({"family": "oregon-white-oak", "outputs": {"field": bad}})).err();
        assert!(
            matches!(error, Some(Error::InvalidInput("field limb order"))),
            "{bad}: {error:?}"
        );
    }
    let (out, meta) =
        generate(json!({"family": "oregon-white-oak", "outputs": {"field": false}})).unwrap();
    assert!(out.field.is_none() && meta["stages"]["field"] == json!(false));
}
