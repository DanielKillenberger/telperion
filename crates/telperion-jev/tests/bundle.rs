//! The bundle a round moves, and the contact sheet that judges it. Pure
//! arithmetic and pure rules: no dispatch, no render, no model.
mod fixture;

use serde_json::{json, Value};
use std::fs;
use telperion_jev::tuning::{
    actions::{Action, Dial},
    bundle,
    engine::Proposal,
    evaluation::{Comparison, Image, Trial},
    look::Priority,
    sheet::{self, Answer, Break, Grade, Movement, PriorityAnswer, Step},
};

fn dial(id: &str, path: &str, min: f64, max: f64, small: f64, integer: bool) -> Dial {
    Dial {
        id: id.into(),
        path: path.into(),
        meaning: "a row".into(),
        min,
        max,
        small,
        substantial: small * 2.,
        integer,
        group: Some(path.split('/').nth(1).unwrap().into()),
        score_visible: Some(true),
        meaning_basis: None,
        range_basis: None,
        source: None,
        preset_span: None,
        cap: None,
    }
}

fn beech() -> Value {
    telperion_core::params::metadata(
        &telperion_core::presets::Preset::from_id("european-beech")
            .unwrap()
            .parameters(),
    )
}

fn proposal(dial: &str, action: Action) -> Proposal {
    Proposal {
        dial: dial.into(),
        action,
        ledger: "jev:1".into(),
        direction_mass: Some(0.9),
        rule: None,
    }
}

#[test]
fn only_the_direction_of_each_supported_move_enters_the_bundle() {
    let proposals = vec![
        proposal("twig_hang", Action::SubstantialIncrease),
        proposal("rise_secondary", Action::SmallDecrease),
        proposal("twig_hang", Action::SmallDecrease),
        proposal("shoulder", Action::Hold),
    ];
    assert_eq!(
        bundle::directions(&proposals),
        vec![("twig_hang".to_string(), 1), ("rise_secondary".into(), -1)],
        "a dial keeps the first direction Jev gave it, and a hold is not a move"
    );
}

#[test]
fn a_strength_steps_every_dial_by_its_own_small_step() {
    let dials = vec![
        dial("twig_hang", "/skeleton/twigs/hang", 0., 3., 0.5, false),
        dial(
            "rise_secondary",
            "/skeleton/habit/riseSecondary",
            -1.,
            1.,
            0.2,
            false,
        ),
        dial(
            "limbs",
            "/skeleton/habit/lateralsPerStation",
            1.,
            4.,
            1.,
            true,
        ),
    ];
    let wanted = vec![
        ("twig_hang".to_string(), 1),
        ("rise_secondary".to_string(), -1),
        ("limbs".to_string(), 1),
    ];
    let wire = beech();
    let at = |strength: f64| {
        bundle::build(
            "european-beech",
            &wire,
            &dials,
            &wanted,
            strength,
            "base",
            "structure",
        )
        .unwrap()
    };

    // Half a step on each float row, and an integer row that cannot move half.
    let (half, _) = at(0.5);
    let hang = half.moves.iter().find(|m| m.dial == "twig_hang").unwrap();
    assert_eq!(
        (hang.from, hang.to, hang.direction.as_str()),
        (0., 0.25, "up")
    );
    let rise = half
        .moves
        .iter()
        .find(|m| m.dial == "rise_secondary")
        .unwrap();
    assert_eq!(rise.to, rise.from - 0.1);
    // Half of a whole-number step rounds away from zero, so it still moves.
    let limbs = half.moves.iter().find(|m| m.dial == "limbs").unwrap();
    assert_eq!(limbs.to, limbs.from + 1.);
    // Below half a step there is nothing to round up to, and the row drops.
    let (weak, _) = bundle::build(
        "european-beech",
        &wire,
        &dials,
        &wanted,
        0.4,
        "base",
        "structure",
    )
    .unwrap();
    assert!(
        weak.moves.iter().all(|m| m.dial != "limbs")
            && weak
                .dropped
                .iter()
                .any(|d| d.dial == "limbs" && d.reason.contains("rounds to no move")),
        "{weak:?}"
    );

    // A whole step moves every row, integers included.
    let (one, overlay) = at(1.0);
    assert_eq!(one.moves.len(), 3);
    assert_eq!(overlay["skeleton"]["twigs"]["hang"], json!(0.5));
    assert!(overlay["skeleton"]["habit"]["lateralsPerStation"].is_i64());

    // Four steps clip to the dial's own bound, and a row already at that
    // bound drops out rather than pretending to move.
    let (four, _) = at(4.0);
    let hang = four.moves.iter().find(|m| m.dial == "twig_hang").unwrap();
    assert_eq!(hang.to, 2.0);
    let at_bound = dial("shell_depth", "/shellDepth", 0., 1., 0.1, false);
    let (_, dropped) = bundle::build(
        "european-beech",
        &wire,
        &[at_bound],
        &[("shell_depth".to_string(), 1)],
        4.0,
        "base",
        "structure",
    )
    .map(|(b, o)| (o, b.dropped.clone()))
    .unwrap_or_else(|e| {
        (
            json!({}),
            vec![bundle::Dropped {
                dial: "shell_depth".into(),
                reason: e,
            }],
        )
    });
    assert!(
        dropped
            .iter()
            .any(|d| d.dial == "shell_depth" && d.reason.contains("bound")
                || d.reason.contains("dropped out")),
        "{dropped:?}"
    );
}

#[test]
fn a_bundle_is_the_same_attempt_when_its_dials_directions_strength_and_base_are() {
    let moves = vec![
        bundle::Move {
            dial: "a".into(),
            direction: "up".into(),
            from: 0.,
            to: 1.,
        },
        bundle::Move {
            dial: "b".into(),
            direction: "down".into(),
            from: 1.,
            to: 0.,
        },
    ];
    let reversed = vec![moves[1].clone(), moves[0].clone()];
    assert_eq!(
        bundle::id(&moves, 1., "base", "structure"),
        bundle::id(&reversed, 1., "base", "structure"),
        "the order Jev listed them in is not part of the attempt"
    );
    assert_ne!(
        bundle::id(&moves, 1., "base", "structure"),
        bundle::id(&moves, 2., "base", "structure")
    );
    assert_ne!(
        bundle::id(&moves, 1., "base", "structure"),
        bundle::id(&moves, 1., "another", "structure")
    );
    let mut other = moves.clone();
    other[0].direction = "down".into();
    assert_ne!(
        bundle::id(&moves, 1., "base", "structure"),
        bundle::id(&other, 1., "base", "structure")
    );
    // The values reached are not part of it: the same directions from the same
    // tree at the same strength are the same attempt.
    let mut moved = moves.clone();
    moved[0].to = 99.;
    assert_eq!(
        bundle::id(&moves, 1., "base", "structure"),
        bundle::id(&moved, 1., "base", "structure")
    );
}

#[test]
fn a_split_cuts_by_the_table_s_groups_and_falls_back_to_the_order_jev_gave() {
    let dials = vec![
        dial("twig_hang", "/skeleton/twigs/hang", 0., 3., 0.5, false),
        dial("twig_sag", "/skeleton/twigs/sag", 0., 1., 0.1, false),
        dial("leaf_size", "/canopy/size", 0., 2., 0.1, false),
    ];
    let moves: Vec<bundle::Move> = ["twig_hang", "twig_sag", "leaf_size"]
        .iter()
        .map(|d| bundle::Move {
            dial: (*d).into(),
            direction: "up".into(),
            from: 0.,
            to: 1.,
        })
        .collect();
    let (a, b) = bundle::split(&moves, &dials);
    assert_eq!(a.len(), 2, "the largest group goes together");
    assert_eq!(b.len(), 1);
    assert!(a.iter().all(|m| m.dial.starts_with("twig_")));

    let one_group: Vec<bundle::Move> = moves[..2].to_vec();
    let (a, b) = bundle::split(&one_group, &dials);
    assert_eq!((a.len(), b.len()), (1, 1), "one group halves by order");
    assert_eq!(a[0].dial, "twig_hang");
}

/// The real table, so the large-group rule is measured and not asserted.
fn table() -> Vec<Dial> {
    telperion_jev::tuning::table::authored()
}

#[test]
fn a_skeleton_row_takes_its_second_path_segment_as_its_family() {
    let table = table();
    for (path, want) in [
        ("/skeleton/envelope/taper", "envelope"),
        ("/skeleton/habit/riseSecondary", "habit"),
        ("/skeleton/twigs/hang", "twigs"),
        ("/skeleton/bias/strength", "bias"),
        ("/skeleton/growth/maxNodes", "growth"),
        // radii is four rows and shellDepth one, so neither is cut.
        ("/radii/trunk", "radii"),
        ("/shellDepth", "shellDepth"),
    ] {
        assert_eq!(bundle::family(path, &table), want, "{path}");
    }
}

/// The 91 material rows were one family, so six live rounds of a material
/// bundle could not be cut at all. A group of more than eight rows is told
/// apart by the leaf's own first word, qualified by its group so that
/// `element` and `material` do not share a `lobe`.
#[test]
fn a_group_too_large_to_cut_is_told_apart_by_the_leaf_s_first_word() {
    let table = table();
    for (path, want) in [
        ("/material/barkRed", "material.bark"),
        ("/material/barkGreen", "material.bark"),
        ("/material/plateCellScale", "material.plate"),
        ("/material/plateIdentity", "material.plate"),
        ("/material/lichenCoverage", "material.lichen"),
        ("/material/ridgeScale", "material.ridge"),
        ("/material/crestGreen", "material.crest"),
        ("/canopy/clumpSpan", "canopy.clump"),
        ("/canopy/size", "canopy.size"),
        ("/element/lobeDepth", "element.lobe"),
        ("/surface/flareHeight", "surface.flare"),
    ] {
        assert_eq!(bundle::family(path, &table), want, "{path}");
    }
    // A group of eight rows or fewer stays whole, whatever its leaves say.
    let small: Vec<Dial> = table
        .iter()
        .filter(|d| d.path.starts_with("/material/"))
        .take(8)
        .cloned()
        .collect();
    assert_eq!(bundle::family("/material/barkRed", &small), "material");

    // The live 91 material rows fall into families no cut can leave whole.
    let material = table
        .iter()
        .filter(|d| d.path.starts_with("/material/"))
        .map(|d| bundle::family(&d.path, &table))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(material.len(), 32, "{material:?}");
    for name in ["material.bark", "material.plate", "material.lichen"] {
        assert!(material.contains(name), "{material:?}");
    }
}

#[test]
fn a_worse_bundle_cuts_into_at_most_four_parts_with_the_smallest_merged() {
    let rows = [
        ("a1", "/skeleton/twigs/hang"),
        ("a2", "/skeleton/twigs/sag"),
        ("a3", "/skeleton/twigs/drop"),
        ("b1", "/skeleton/habit/riseSecondary"),
        ("b2", "/skeleton/habit/crookedness"),
        ("c1", "/skeleton/envelope/taper"),
        ("d1", "/canopy/size"),
        ("e1", "/material/bark/plates"),
        ("f1", "/surface/gloss"),
    ];
    let dials = rows
        .iter()
        .map(|(id, path)| dial(id, path, 0., 3., 0.1, false))
        .collect::<Vec<_>>();
    let moves = rows
        .iter()
        .map(|(id, _)| bundle::Move {
            dial: (*id).into(),
            direction: "up".into(),
            from: 0.,
            to: 1.,
        })
        .collect::<Vec<_>>();

    // Every group here is small, so each row keeps its own first segment.
    let parts = bundle::families(&moves, &dials);
    assert_eq!(parts.len(), 4, "{parts:?}");
    assert_eq!(
        parts.iter().map(|(_, m)| m.len()).sum::<usize>(),
        moves.len(),
        "the parts together are the bundle"
    );
    let names = parts.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>();
    assert_eq!(
        names,
        vec!["canopy+envelope", "habit", "material+surface", "twigs"],
        "the smallest families merge, by move count then name"
    );
    // Deterministic: the same bundle cuts the same way every time.
    assert_eq!(bundle::families(&moves, &dials), parts);
    // Four or fewer families are already the parts.
    let three = moves[..6].to_vec();
    assert_eq!(
        bundle::families(&three, &dials)
            .iter()
            .map(|(n, _)| n.clone())
            .collect::<Vec<_>>(),
        vec!["envelope", "habit", "twigs"]
    );
}

#[test]
fn an_isolated_part_is_read_back_off_its_own_label() {
    assert_eq!(bundle::part_family("bundle@0.5/twigs"), Some("twigs"));
    assert_eq!(
        bundle::part_family("bundle@1/canopy+envelope"),
        Some("canopy+envelope")
    );
    assert_eq!(bundle::part_family("bundle@1"), None);
    assert_eq!(bundle::part_family("twig_hang"), None);
}

// --- the contact sheet ---

fn still(view: &str, body: &str) -> Image {
    let path = std::env::temp_dir().join(format!(
        "sheet-{}-{}.png",
        body,
        telperion_jev::ledger::new_entry_id()
    ));
    fs::write(&path, body.as_bytes()).unwrap();
    Image {
        path,
        sha256: telperion_jev::sha256_hex(body.as_bytes()),
        view: view.into(),
        seed: 1,
    }
}

fn priorities(ids: &[&str]) -> Vec<Priority> {
    ids.iter()
        .map(|id| Priority {
            id: (*id).into(),
            observation: format!("observation for {id}"),
        })
        .collect()
}

fn plan(keys: &[&str]) -> sheet::Plan {
    // The stills are named by position, never by what they are: a file path
    // travels to the adapter inside the request.
    let variants: Vec<(String, Image)> = keys[1..]
        .iter()
        .enumerate()
        .map(|(i, k)| ((*k).to_string(), still("whole", &format!("still{}", i + 1))))
        .collect();
    let current = still("whole", "still0");
    sheet::plan(
        "european-beech",
        "whole",
        1,
        &[still("whole", "reference")],
        (keys[0], &current),
        &variants,
        &priorities(&["g0", "g1"]),
        "owner notes",
    )
    .unwrap()
}

#[test]
fn the_sheet_never_says_which_render_the_loop_is_standing_on() {
    let plan = plan(&["current", "half", "one", "two"]);
    let wire = serde_json::to_string(&plan.request).unwrap();
    for word in ["current", "candidate", "baseline", "strength", "bundle"] {
        assert!(!wire.contains(word), "the sheet says {word}");
    }
    assert!(!wire.contains("\"order\"") && !wire.contains("\"current\""));
    assert_eq!(plan.request.renders.len(), 4);
    assert!(plan.order.contains(&"current".to_string()));
    assert_eq!(
        plan.order[plan.current.parse::<usize>().unwrap() - 1],
        "current",
        "the label code recorded is not where the current tree is"
    );
    // The order is decided by the whole set, so it is stable for that set.
    let again = sheet::order(&plan.order);
    assert_eq!(
        sheet::order(&["current".into(), "half".into(), "one".into(), "two".into()]).len(),
        4
    );
    assert_eq!(again.len(), 4);
}

fn steps(ranking: &[&str], grades: &[Grade]) -> Vec<Step> {
    ranking
        .windows(2)
        .zip(grades)
        .map(|(pair, grade)| Step {
            from: pair[0].into(),
            to: pair[1].into(),
            grade: *grade,
        })
        .collect()
}

#[test]
fn the_grade_over_the_current_tree_is_read_off_the_ranking_path() {
    let ranking = ["3", "2", "1", "4"];
    let labels: Vec<String> = ranking.iter().map(|s| (*s).to_string()).collect();
    let table = [
        // A render one clear step above the current tree.
        (
            vec![Grade::None, Grade::Clear, Grade::None],
            "2",
            Movement::Clear,
        ),
        // Two slight steps above it are still only slight.
        (
            vec![Grade::Slight, Grade::Slight, Grade::None],
            "3",
            Movement::Slight,
        ),
        // A clear step anywhere on the path makes it clear.
        (
            vec![Grade::Clear, Grade::Slight, Grade::None],
            "3",
            Movement::Clear,
        ),
        // Ranked above but indistinguishable is no improvement.
        (
            vec![Grade::None, Grade::None, Grade::None],
            "3",
            Movement::None,
        ),
        // Below the current tree, any visible step is worse.
        (
            vec![Grade::None, Grade::None, Grade::Slight],
            "4",
            Movement::Worse,
        ),
        (
            vec![Grade::None, Grade::None, Grade::None],
            "4",
            Movement::None,
        ),
    ];
    for (grades, render, want) in table {
        let got = sheet::grade(&labels, &steps(&ranking, &grades), "1", render);
        assert_eq!(got, want, "{render} under {grades:?}");
    }
    assert_eq!(
        sheet::grade(&labels, &steps(&ranking, &[Grade::Clear; 3]), "1", "1"),
        Movement::None,
        "the current tree is not better than itself"
    );
}

fn answer(ranking: &[&str], grades: &[Grade], breaks: Vec<Break>) -> Answer {
    overall_answer(ranking, grades, breaks, ranking)
}

/// The same answer with the overall believability ranking stated apart from
/// the per-priority one: a move can win a priority and lose the tree.
fn overall_answer(
    ranking: &[&str],
    grades: &[Grade],
    breaks: Vec<Break>,
    overall: &[&str],
) -> Answer {
    Answer {
        overall: overall.iter().map(|s| (*s).to_string()).collect(),
        wrong: vec![sheet::Wrong {
            render: overall[0].into(),
            text: "the outer branches are too thick".into(),
        }],
        priorities: ["g0", "g1"]
            .iter()
            .map(|id| PriorityAnswer {
                priority_id: (*id).into(),
                closest: ranking[0].into(),
                ranking: ranking.iter().map(|s| (*s).to_string()).collect(),
                steps: steps(ranking, grades),
            })
            .collect(),
        breaks,
        improved: "the outer foliage hangs".into(),
        missing: "the crown is still enclosed".into(),
    }
}

#[test]
fn a_sheet_answer_binds_only_when_it_ranks_every_render_once() {
    let plan = plan(&["current", "half", "one"]);
    let order: Vec<&str> = plan.order.iter().map(String::as_str).collect();
    let labels: Vec<String> = (0..3).map(sheet::Request::label).collect();
    let ranking: Vec<&str> = labels.iter().map(String::as_str).collect();
    let good = answer(&ranking, &[Grade::Clear, Grade::None], vec![]);
    let bound = sheet::bind(&plan, &good, "receipt".into(), "mock".into()).unwrap();
    assert_eq!(bound.renders.len(), 3);
    assert_eq!(bound.current, plan.current);
    assert!(bound.uncalibrated.contains("uncalibrated"));
    assert_eq!(bound.render(order[0]).unwrap().label, "1");

    let mut short = good.clone();
    short.priorities.truncate(1);
    assert!(sheet::bind(&plan, &short, "r".into(), "m".into()).is_err());
    let mut twice = good.clone();
    twice.priorities[1].priority_id = "g0".into();
    assert!(sheet::bind(&plan, &twice, "r".into(), "m".into()).is_err());
    let mut partial = good.clone();
    partial.priorities[0].ranking.truncate(2);
    assert!(sheet::bind(&plan, &partial, "r".into(), "m".into()).is_err());
    let mut stray = good.clone();
    stray.priorities[0].ranking[0] = "9".into();
    assert!(sheet::bind(&plan, &stray, "r".into(), "m".into()).is_err());
    let mut mismatched = good.clone();
    mismatched.priorities[0].steps[0].to = "3".into();
    assert!(sheet::bind(&plan, &mismatched, "r".into(), "m".into()).is_err());
    let mut unknown_break = good.clone();
    unknown_break.breaks = vec![Break {
        render: "9".into(),
        text: "the bole disappears".into(),
    }];
    assert!(sheet::bind(&plan, &unknown_break, "r".into(), "m".into())
        .unwrap_err()
        .contains("no render on the sheet"));
}

#[test]
fn the_round_keeps_the_clearest_improvement_that_breaks_nothing() {
    let plan = plan(&["current", "half", "one", "two"]);
    let current = plan.current.parse::<usize>().unwrap() - 1;
    // Rank the current tree last so every variant is above it.
    let mut ranking: Vec<String> = (0..4)
        .map(sheet::Request::label)
        .filter(|l| l != &plan.current)
        .collect();
    ranking.push(plan.current.clone());
    let as_str: Vec<&str> = ranking.iter().map(String::as_str).collect();
    let keys: Vec<String> = plan.order.clone();
    let shown: Vec<(String, f64)> = keys
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != current)
        .map(|(i, key)| (key.clone(), [0.5, 1.0, 2.0, 4.0][i]))
        .collect();

    // The top two are clear, the third only slight: the clear one with the
    // smaller strength wins.
    let bound = sheet::bind(
        &plan,
        &answer(&as_str, &[Grade::None, Grade::Clear, Grade::Slight], vec![]),
        "r".into(),
        "m".into(),
    )
    .unwrap();
    let winner = sheet::adopt(&bound, &shown).unwrap();
    let winner_render = bound.render(&winner).unwrap();
    assert!(winner_render.clear() && winner_render.adoptable());
    let smaller = shown
        .iter()
        .find(|(k, _)| k == &winner)
        .map(|(_, s)| *s)
        .unwrap();
    assert!(
        shown
            .iter()
            .filter(|(k, _)| bound.render(k).is_some_and(|r| r.clear()))
            .all(|(_, s)| *s >= smaller),
        "a clear variant with a smaller strength was passed over"
    );

    // A break on the best variant takes it out and leaves it for a split.
    let breaks = vec![Break {
        render: as_str[0].into(),
        text: "the bole is bare".into(),
    }];
    let with_break = sheet::bind(
        &plan,
        &answer(&as_str, &[Grade::None, Grade::Clear, Grade::Slight], breaks),
        "r".into(),
        "m".into(),
    )
    .unwrap();
    let broken_key = &plan.order[as_str[0].parse::<usize>().unwrap() - 1];
    assert!(!with_break.render(broken_key).unwrap().adoptable());
    assert!(with_break.render(broken_key).unwrap().better_with_breaks());
    assert_ne!(sheet::adopt(&with_break, &shown).as_ref(), Some(broken_key));
    assert_eq!(
        sheet::to_split(&with_break, &shown).map(|(key, _)| key),
        Some(broken_key.clone())
    );

    // Nothing above the current tree is nothing to keep.
    let flat = sheet::bind(
        &plan,
        &answer(&as_str, &[Grade::None, Grade::None, Grade::None], vec![]),
        "r".into(),
        "m".into(),
    )
    .unwrap();
    assert_eq!(sheet::adopt(&flat, &shown), None);
    assert_eq!(sheet::to_split(&flat, &shown), None);
}

// --- which variants the sheet is worth showing ---

fn variant(key: &str, label: &str, strength: f64, images: Vec<Image>) -> Trial {
    let mut trial = Trial {
        key: key.into(),
        identity: "progress-fixture".into(),
        seed: 1,
        round: 1,
        label: label.into(),
        overrides: json!({}),
        ledger: None,
        feasible: true,
        reason: None,
        measurement: json!({}),
        comparisons: vec![Comparison {
            reference: "whole".into(),
            reference_weight: 1.,
            metric_weights: [1.; 5],
            target: [1.; 5],
            observed: [None; 5],
            images,
        }],
        score: Some(0.2),
        seconds: 0.,
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
        adopted_over: vec![],
        bundle: None,
        parent_bundle: None,
        sheet: None,
        vetoed: None,
        adopted: false,
        step: None,
    };
    trial.bundle = Some(bundle::Bundle {
        strength,
        moves: vec![bundle::Move {
            dial: "twig_hang".into(),
            direction: "up".into(),
            from: 0.,
            to: strength,
        }],
        dropped: vec![],
        id: format!("bundle-{strength}"),
    });
    trial
}

fn gap() -> telperion_jev::tuning::priority::Gap {
    telperion_jev::tuning::priority::Gap {
        id: "owner-crown".into(),
        observation: "Crown shape and foliage organization".into(),
        evidence_ids: vec!["render-0".into()],
        views: vec!["whole".into()],
        track: None,
    }
}

#[test]
fn a_variant_that_draws_the_current_tree_or_a_twin_never_reaches_the_sheet() {
    let view = |body: &str| vec![still("whole", body)];
    let current = variant("current", "baseline", 0., view("current"));
    let state = fixture::progress_run(vec![
        current.clone(),
        // Byte for byte the current tree: nothing to ask about.
        variant("half", "bundle@0.5", 0.5, view("current")),
        variant("one", "bundle@1", 1., view("one")),
        // The same picture as the variant before it, at a larger strength.
        variant("two", "bundle@2", 2., view("one")),
        variant("four", "bundle@4", 4., view("four")),
    ]);
    let look = sheet::look(
        &state,
        "european-beech",
        &[still("whole", "reference")],
        0,
        &[1, 2, 3, 4],
        &[gap()],
        None,
    )
    .unwrap();

    assert_eq!(look.shown, vec![2, 4], "{:?}", look.not_shown);
    assert_eq!(look.not_shown.len(), 2);
    assert!(look.not_shown[0].inert && look.not_shown[0].trial == 1);
    assert!(
        !look.not_shown[1].inert && look.not_shown[1].reason.contains("bundle@1"),
        "the larger strength is the one dropped: {:?}",
        look.not_shown[1]
    );
    let plan = look.plan.unwrap();
    assert_eq!(
        plan.request.renders.len(),
        3,
        "the current tree and two variants"
    );
    assert_eq!(
        plan.order[plan.current.parse::<usize>().unwrap() - 1],
        "current"
    );

    // Nothing that differs is a sheet nobody is paid for.
    let flat = fixture::progress_run(vec![
        current,
        variant("twin", "bundle@1", 1., view("current")),
    ]);
    let look = sheet::look(
        &flat,
        "european-beech",
        &[still("whole", "reference")],
        0,
        &[1],
        &[gap()],
        None,
    )
    .unwrap();
    assert!(look.plan.is_none() && look.shown.is_empty() && look.not_shown[0].inert);
}

#[test]
fn a_variant_the_reviewer_finds_less_believable_overall_is_not_adopted() {
    let plan = plan(&["current", "half", "one"]);
    let labels: Vec<String> = (0..3).map(sheet::Request::label).collect();
    // Every variant beats the current tree on both priorities.
    let mut ranking: Vec<String> = labels
        .iter()
        .filter(|l| *l != &plan.current)
        .cloned()
        .collect();
    ranking.push(plan.current.clone());
    let as_str: Vec<&str> = ranking.iter().map(String::as_str).collect();
    let shown: Vec<(String, f64)> = plan
        .order
        .iter()
        .enumerate()
        .filter(|(i, _)| sheet::Request::label(*i) != plan.current)
        .map(|(i, key)| (key.clone(), [0.5, 1.0, 2.0][i]))
        .collect();
    let grades = [Grade::Clear, Grade::Clear];

    // The reviewer puts the current tree first overall: a step that wins the
    // priorities and loses the tree is not a step forward.
    let mut overall = vec![plan.current.as_str()];
    overall.extend(as_str.iter().take(2));
    let bound = sheet::bind(
        &plan,
        &overall_answer(&as_str, &grades, vec![], &overall),
        "r".into(),
        "m".into(),
    )
    .unwrap();
    assert!(bound.renders.iter().filter(|r| r.below_current).count() == 2);
    assert_eq!(sheet::adopt(&bound, &shown), None);
    assert_eq!(sheet::to_split(&bound, &shown), None);

    // With the same grades and the current tree last overall, it is adopted,
    // and what the reviewer said looks wrong travels with the verdict.
    let mut overall = as_str.iter().take(2).copied().collect::<Vec<_>>();
    overall.push(&plan.current);
    let bound = sheet::bind(
        &plan,
        &overall_answer(&as_str, &grades, vec![], &overall),
        "r".into(),
        "m".into(),
    )
    .unwrap();
    let kept = sheet::adopt(&bound, &shown).expect("nothing was adopted");
    assert_eq!(bound.render(&kept).unwrap().overall, 1);
    assert_eq!(
        bound.render(&kept).unwrap().wrong,
        vec!["the outer branches are too thick".to_string()]
    );

    // The overall ranking binds as strictly as the others.
    let mut short = overall_answer(&as_str, &grades, vec![], &overall);
    short.overall.truncate(2);
    assert!(sheet::bind(&plan, &short, "r".into(), "m".into())
        .unwrap_err()
        .contains("overall ranking"));
    let mut three = overall_answer(&as_str, &grades, vec![], &overall);
    three.wrong = (0..3)
        .map(|i| sheet::Wrong {
            render: overall[0].into(),
            text: format!("wrong {i}"),
        })
        .collect();
    assert!(sheet::bind(&plan, &three, "r".into(), "m".into())
        .unwrap_err()
        .contains("more than two"));
    let mut stray = overall_answer(&as_str, &grades, vec![], &overall);
    stray.wrong[0].render = "9".into();
    assert!(sheet::bind(&plan, &stray, "r".into(), "m".into()).is_err());
}

#[test]
fn a_track_judged_at_one_view_is_judged_on_that_view_alone() {
    // A material move changes the bark and nothing else: on the whole-tree
    // still the variant is the current tree byte for byte.
    let whole = still("whole", "same tree");
    let current = variant(
        "current",
        "baseline",
        0.,
        vec![whole.clone(), still("B-BASE", "current bark")],
    );
    let material = variant(
        "bark",
        "bundle@1",
        1.,
        vec![whole, still("B-BASE", "rougher bark")],
    );
    let state = fixture::progress_run(vec![current, material]);

    // Judged on what the evaluation renders - the numeric reference views -
    // it is inert, and no sheet would be bought for any material move.
    let rendered = fixture::progress_run(vec![
        variant("current", "baseline", 0., vec![still("whole", "same tree")]),
        variant("bark", "bundle@1", 1., vec![still("whole", "same tree")]),
    ]);
    let anywhere = sheet::look(
        &rendered,
        "european-beech",
        &[still("whole", "reference")],
        0,
        &[1],
        &[gap()],
        None,
    )
    .unwrap();
    assert!(anywhere.plan.is_none() && anywhere.not_shown[0].inert);

    // Judged at the track's own view, it is a question worth asking.
    let at_view = sheet::look(
        &state,
        "european-beech",
        &[still("whole", "reference"), still("B-BASE", "reference")],
        0,
        &[1],
        &[gap()],
        Some("B-BASE"),
    )
    .unwrap();
    assert_eq!(at_view.shown, vec![1], "{:?}", at_view.not_shown);
    let plan = at_view.plan.expect("no sheet for the material track");
    assert_eq!(plan.request.view, "B-BASE");
    assert_eq!(plan.request.renders.len(), 2);
    assert!(plan.request.references.iter().all(|i| i.view == "B-BASE"));
}
