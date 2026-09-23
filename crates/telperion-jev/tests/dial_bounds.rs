//! A dial's walls are the generator's validated bounds; the span the shipped
//! presets occupy rides on the row as a hint and never limits a move.
//!
//! Offline: no model is called, nothing is grown past its first frontier,
//! nothing is rendered. Every claim is answered by the generator's own
//! validators, in process.
use serde_json::{json, Value};
use telperion_core::branching::Specimen;
use telperion_core::foliage::{build_element, place, Reference, TwigPlacement};
use telperion_core::params;
use telperion_core::presets::Preset;
use telperion_core::tree::{Node, Tree};
use telperion_core::Family;
use telperion_jev::tuning::actions::{candidate, Action, Dial};

const TABLE: &str = include_str!("../data/dials.json");

fn table() -> Vec<Dial> {
    serde_json::from_str(TABLE).expect("data/dials.json is a dial table")
}

/// The rows that once took their range from the preset span.
fn audited() -> Vec<Dial> {
    table()
        .into_iter()
        .filter(|d| d.preset_span.is_some())
        .collect()
}

fn families() -> Vec<&'static str> {
    params::CATALOGUE
        .iter()
        .chain(params::IN_WORK)
        .map(|entry| entry.1)
        .collect()
}

fn row(dial: &str) -> Dial {
    table()
        .into_iter()
        .find(|d| d.id == dial)
        .unwrap_or_else(|| panic!("no row {dial}"))
}

/// The family with one row restated, read back through the wire.
fn with(family: &str, dial: &Dial, value: f64) -> Result<Family, String> {
    let base = Preset::from_id(family)
        .expect("catalogue identity")
        .parameters();
    let mut wire = params::metadata(&base);
    *wire.pointer_mut(&dial.path).expect("row on the wire") = if dial.integer {
        json!(value as i64)
    } else {
        json!(value)
    };
    params::parse(&wire).map_err(|e| format!("wire: {e}"))
}

/// A solved tree of one node: the canopy's rails are checked against it and
/// nothing is placed, since a tree without an axis bears no leaf.
fn bare_tree() -> Tree {
    let root = Node {
        radius: 0.1,
        start_radius: 0.1,
        base_radius: 0.1,
        ..Node::root()
    };
    Tree {
        nodes: vec![root],
        crossover: 1,
        ..Tree::default()
    }
}

/// The generator's own validators for the row's group, and nothing else.
fn accepts(family: &str, dial: &Dial, value: f64) -> Result<(), String> {
    let f = with(family, dial, value)?;
    let checked = match dial.group.as_deref() {
        Some("skeleton") => {
            let mut skeleton = f.skeleton.clone();
            if dial.path == "/skeleton/attractors" {
                // The count is checked before any is scattered; at zero
                // weight none is, so a million costs nothing here.
                skeleton.habit.attractor_weight = 0.0;
            }
            Specimen::new(&skeleton, f.radii).map(drop)
        }
        Some("radii") => f.radii.resolved().map(drop),
        Some("surface") => f.surface.validate(),
        Some("element") => build_element(f.element).map(drop),
        Some("canopy") => {
            let twig = f.skeleton.twigs.resolved().map_err(|e| e.to_string())?.twig;
            let placement = TwigPlacement {
                internode_length: twig.internode_length,
                stations_per_internode: twig.stations_per_internode,
            };
            let reference = Reference::of(&f).map_err(|e| e.to_string())?;
            place(
                &bare_tree(),
                f.skeleton.envelope,
                f.skeleton.seed,
                f.canopy,
                Some(placement),
                reference,
            )
            .map(drop)
        }
        // The wire is the material row's consumer: `parse` above judged it.
        Some("material") => Ok(()),
        other => panic!("{}: no validator for group {other:?}", dial.id),
    };
    checked.map_err(|e| e.to_string())
}

#[test]
fn no_row_takes_a_wall_from_the_preset_span() {
    let rows = audited();
    assert_eq!(
        rows.len(),
        75,
        "the audit covers the former preset-span rows"
    );
    for dial in &rows {
        let basis = dial.range_basis.as_deref();
        assert!(
            matches!(basis, Some("validated bound" | "capped")),
            "{}: range basis {basis:?}",
            dial.id
        );
        assert_eq!(
            dial.cap.is_some(),
            basis == Some("capped"),
            "{}: a capped row, and only a capped row, says why",
            dial.id
        );
        let [low, high] = dial.preset_span.unwrap();
        assert!(
            dial.min <= low && high <= dial.max,
            "{}: [{}, {}] is narrower than the preset span",
            dial.id,
            dial.min,
            dial.max
        );
    }
}

/// A row widened past what the generator validates fails here: each wall the
/// audit moved off the preset span is a value the generator accepts on every
/// family. A wall still at the span is the one the table always had.
#[test]
fn every_widened_wall_is_a_value_the_generator_accepts() {
    let mut refused = Vec::new();
    for dial in audited() {
        let [low, high] = dial.preset_span.unwrap();
        let walls = [(dial.min, low), (dial.max, high)];
        for family in families() {
            for (wall, _) in walls.iter().filter(|(wall, span)| wall != span) {
                if let Err(e) = accepts(family, &dial, *wall) {
                    refused.push(format!("{family}: {} at {wall}: {e}", dial.id));
                }
            }
        }
    }
    assert!(
        refused.is_empty(),
        "walls the generator refuses: {refused:#?}"
    );
}

/// The check above has teeth: one step past a validated wall is refused.
#[test]
fn a_value_past_a_validated_wall_is_refused() {
    for (dial, past) in [
        ("leaf_length", 1e3 + 1.),
        ("leaf_size", 1000.5),
        ("twig_length", 1e6 + 1.),
        ("radial_segments", 65.),
        ("attractors", 1_000_001.),
        ("material_lenticel_density", 401.),
    ] {
        let dial = row(dial);
        assert!(past > dial.max, "{}", dial.id);
        assert!(
            accepts("ordinary", &dial, past).is_err(),
            "{}: the generator accepted {past}, past its wall {}",
            dial.id,
            dial.max
        );
    }
}

/// The span is a hint: a move from its edge outward is offered and drawn.
#[test]
fn a_move_past_the_preset_span_is_accepted() {
    let dial = row("leaf_length");
    let [_, high] = dial.preset_span.unwrap();
    let family = "date-palm";
    let base = Preset::from_id(family).unwrap().parameters();
    let mut wire = params::metadata(&base);
    *wire.pointer_mut(&dial.path).unwrap() = json!(high);
    let moved = candidate(family, &wire, &dial, Action::SubstantialIncrease)
        .expect("a move past the span, inside the wall");
    let to = moved.pointer(&dial.path).and_then(Value::as_f64).unwrap();
    assert!(to > high, "the move went nowhere: {to}");
    accepts(family, &dial, to).expect("the generator draws it");
}

/// The palm's measured values sit inside the rows the loop tunes it on.
/// A1 and F1 (fn-80 extract): leaflets 20 to 40 cm (spec R3), a trunk up to
/// half a metre across on a palm of 24.4 m (F1, 80 ft) to 35 m (A1).
#[test]
fn the_date_palm_can_reach_its_measured_proportions() {
    let palm = Preset::from_id("date-palm").unwrap().parameters();
    let reach = |id: &str, value: f64| {
        let dial = row(id);
        assert!(
            (dial.min..=dial.max).contains(&value),
            "{id}: {value} lies outside [{}, {}]",
            dial.min,
            dial.max
        );
        accepts("date-palm", &dial, value).unwrap_or_else(|e| panic!("{id}: {e}"));
    };
    for leaflet in [0.2, 0.4] {
        // A drawn leaflet is the element's length times the canopy's size.
        reach("leaf_length", leaflet / palm.canopy.size);
        reach("leaf_size", leaflet / palm.element.length);
    }
    for height in [24.4, 35.0] {
        reach("trunk_radius", 0.25 / height);
    }
    // No source measures a leaflet's width (unknown); the row's wall is now
    // the generator's, not the broadleaf span's 0.11175.
    assert_eq!(row("leaf_width").max, 1e3);
}
