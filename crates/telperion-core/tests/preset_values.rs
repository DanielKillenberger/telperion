//! Presets are value files (fn-160): each shipped file is exactly what the
//! writer makes of its own family, an overlay written as a value file reads
//! back as the same family, and a row the catalogue refuses fails the build
//! with the file and line named.
use serde_json::json;
use std::path::Path;
use telperion_core::{params, presets::values, presets::Preset, Family};

fn same(a: &Family, b: &Family) -> bool {
    format!("{a:?}") == format!("{b:?}")
}

#[test]
fn every_shipped_value_file_is_what_the_writer_makes_of_its_family() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("presets");
    let mut seen = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let id = path.file_stem().unwrap().to_str().unwrap().to_string();
        let text = std::fs::read_to_string(&path).unwrap();
        let family = Preset::from_id(&id).unwrap().parameters();
        // The writer keeps every line and drops a row at its default, so a
        // file it leaves unchanged holds only rows off the default family.
        assert_eq!(values::write(&family, &text, "").unwrap(), text, "{id}");
        assert!(same(&values::read(&text).unwrap(), &family), "{id}");
        seen.push(id);
    }
    seen.sort();
    let mut shipped: Vec<_> = params::CATALOGUE
        .iter()
        .chain(params::IN_WORK)
        .map(|e| e.1)
        .collect();
    shipped.sort();
    assert_eq!(seen, shipped);
}

#[test]
fn an_overlay_written_as_a_value_file_builds_the_same_family() {
    let overlay = json!({
        "shellDepth": 0.3,
        "skeleton": {
            "attractors": 640,
            "habit": {"lateralPitch": 47.25, "stems": 3},
            "bias": {"supernatural": {"enabled": true, "writheAmplitude": 1e-5}},
            "growth": {"maxTurnPerStep": 35.0, "maxNodes": 90000},
        },
        "material": {"fissureRed": -0.0625},
    });
    for base in [Preset::Ordinary, Preset::DatePalm] {
        let family = params::overlay(&base.parameters(), &overlay).unwrap();
        let source = match base {
            Preset::Ordinary => String::new(),
            _ => std::fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("presets/date-palm.values"),
            )
            .unwrap(),
        };
        let text = values::write(&family, &source, "the overlay").unwrap();
        assert!(
            same(&values::read(&text).unwrap(), &family),
            "{base:?}\n{text}"
        );
    }
}

#[test]
fn the_writer_sets_moved_rows_in_place_and_adds_new_ones_under_the_note() {
    let source = "# Head.\n\n# The pitch.\n/skeleton/habit/lateralPitch = 30.0\n\
                  /skeleton/habit/crookedness = 3.0\n/shellDepth = 0.9\n";
    let mut family = values::read(source).unwrap();
    family.skeleton.habit.lateral_pitch = 31.5;
    family.skeleton.habit.crookedness = Family::default().skeleton.habit.crookedness;
    family.skeleton.twigs.laterals = 5;
    let text = values::write(&family, source, "Accepted.").unwrap();
    assert_eq!(
        text,
        "# Head.\n\n# The pitch.\n/skeleton/habit/lateralPitch = 31.5\n/shellDepth = 0.9\n\n\
         # Accepted.\n/skeleton/twigs/laterals = 5\n"
    );
}

#[test]
fn a_row_the_catalogue_refuses_is_named_by_its_line() {
    for (text, why) in [
        (
            "/age = 1\n/skeleton/habit/nope = 1.0",
            "2: /skeleton/habit/nope is not a catalogue row",
        ),
        ("/shellDepth = 1.5", "1: /shellDepth is off its bounds"),
        (
            "/canopy/clumpSystemOrder = 4294967296",
            "1: /canopy/clumpSystemOrder is off its bounds",
        ),
        (
            "/skeleton/habit/stems = 2.5",
            "1: /skeleton/habit/stems holds another kind of value",
        ),
        (
            "/skeleton/habit/stems = true",
            "1: /skeleton/habit/stems holds another kind of value",
        ),
        (
            "\n/shellDepth 1",
            "2: `/shellDepth 1` is not `<path> = <value>`",
        ),
    ] {
        assert_eq!(values::read(text).unwrap_err(), why, "{text}");
    }
}

/// The build's own refusals: the macro the build script emits for each
/// value file, given a row the catalogue does not hold and a row off its
/// bounds, fails to compile with the file and line in the message.
#[test]
fn a_value_file_the_catalogue_refuses_fails_the_build() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/preset_values/unknown.rs");
    cases.compile_fail("tests/preset_values/bounds.rs");
}
