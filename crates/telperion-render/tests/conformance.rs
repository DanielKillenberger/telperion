//! One path for every tree. Each shipped family and a sample of parameter sets
//! nobody wrote by hand go up through the same submit and the same frame, a set
//! the generator refuses says which parameter it refused, and the renderer
//! library is checked for ever having heard of a family at all.
use std::path::Path;

use serde_json::{json, Value};
use telperion_core::{
    mesh::{self, Detail},
    params,
    rng::Rng,
};
use telperion_render::{
    hero_pose, render, FrameStats, RenderError, Renderer, Submitted, GROUND_REACH, STILL_FORMAT,
};

mod common;
use common::gpu;

/// Small enough that the sweep stays near a minute, whole enough that every
/// stage still runs. `tests/browser/integration.mjs` shrinks the same three
/// things for the browser rigs.
fn compact(family: &mut Value) {
    family["skeleton"]["envelope"]["height"] = json!(4.0);
    family["skeleton"]["attractors"] = json!(40);
    let habit = &mut family["skeleton"]["habit"];
    habit["leaderInternode"] = json!(0.6);
    habit["lateralSpacing"] = json!(0.5);
    habit["lateralsPerStation"] = json!(3);
    habit["lateralOrders"] = json!(2);
    // The traits that sit on their own bound in a shipped row are moved off it,
    // so a jitter of a quarter is still a tree rather than a refusal.
    habit["apicalDominance"] = json!(0.5);
    habit["whorlStrength"] = json!(0.4);
    habit["attractorWeight"] = json!(0.5);
    habit["twigTipTaper"] = json!(0.5);
    habit["sheddingThreshold"] = json!(0.4);
}

/// Scales every number in the family by a factor near one. A whole number
/// stays whole, and stays on its own side of zero: the schema reads counts as
/// counts and refuses a float where it wanted one. Casts here saturate, which
/// is what an uncapped count scaled past its own type should do.
fn jitter(value: &mut Value, rng: &mut Rng) {
    match value {
        Value::Object(map) => {
            for (_, field) in map.iter_mut() {
                jitter(field, rng);
            }
        }
        Value::Number(number) => {
            let factor = rng.range(0.8, 1.25);
            if let Some(count) = number.as_u64() {
                *value = json!((count as f64 * factor).round() as u64);
            } else if let Some(whole) = number.as_i64() {
                *value = json!((whole as f64 * factor).round() as i64);
            } else if let Some(real) = number.as_f64() {
                *value = json!(real * factor);
            }
        }
        _ => {}
    }
}

/// Renders whatever the generator will build from this set, at a size that
/// only has to prove the tree reached the pixels.
fn draws(
    renderer: &mut Renderer,
    value: &Value,
    size: u32,
) -> Result<(Submitted, FrameStats), telperion_core::Error> {
    let family = params::parse(value)?;
    let tree = mesh::build(&family, Detail::Full)?;
    let submitted = renderer
        .submit(&tree)
        .expect("a compact tree fits the device");
    let camera = hero_pose(tree.bounds, 1.0, GROUND_REACH);
    let still = render(renderer, &camera, size, size).expect("the frame was drawn");
    assert!(
        still.has_subject(),
        "the still is one flat colour: not even the room was drawn"
    );
    Ok((submitted, still.stats))
}

/// The room is always in the frame, so a still that is not flat proves nothing
/// about the tree. What the tree itself drew is in the frame statistics.
fn drew_the_tree(id: &str, submitted: &Submitted, stats: &FrameStats) {
    assert!(
        submitted.wood_triangles > 0 && submitted.foliage_instances > 0,
        "{id} generated {} wood triangles and {} placements: there is no tree to draw",
        submitted.wood_triangles,
        submitted.foliage_instances
    );
    assert!(
        stats.triangles > submitted.wood_triangles as u32,
        "{id} drew {} triangles, no more than its own wood: the crown never went up",
        stats.triangles
    );
    assert_eq!(
        stats.instances, submitted.foliage_instances as u32,
        "{id} drew a different crown from the one it submitted"
    );
}

#[test]
fn every_shipped_family_renders_through_the_one_path() {
    let Some(gpu) = gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for &(_, id, _, _) in params::CATALOGUE {
        let family = params::by_identity(id).expect("the catalogue names a family it has");
        let tree = mesh::build(&family, Detail::Full).expect("the core built the tree");
        let submitted = renderer.submit(&tree).expect("the tree fits the device");
        let camera = hero_pose(tree.bounds, 1.0, GROUND_REACH);
        let still = render(&mut renderer, &camera, 128, 128).expect("the frame was drawn");
        assert!(
            still.has_subject(),
            "{id} came out as flat background: nothing of it was drawn"
        );
        drew_the_tree(id, &submitted, &still.stats);
    }
}

#[test]
fn parameter_sets_nobody_wrote_by_hand_render_the_same_way() {
    let Some(gpu) = gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let mut rng = Rng::new(20_260_908);
    let (mut rendered, mut refused, mut attempts) = (0, 0, 0);
    while rendered < 20 && attempts < 60 {
        let &(_, id, _, _) = &params::CATALOGUE[attempts % params::CATALOGUE.len()];
        let mut value = params::metadata(&params::by_identity(id).expect("a shipped family"));
        compact(&mut value);
        jitter(&mut value, &mut rng);
        attempts += 1;
        match draws(&mut renderer, &value, 96) {
            // A set the generator will not have is refused, never drawn blank.
            Err(_) => refused += 1,
            Ok((submitted, stats)) => {
                drew_the_tree(&format!("set {attempts} from {id}"), &submitted, &stats);
                rendered += 1;
            }
        }
    }
    println!("{rendered} sets rendered, {refused} refused, {attempts} tried");
    assert!(
        rendered >= 20,
        "only {rendered} of {attempts} sets rendered ({refused} refused by the generator)"
    );
}

#[test]
fn a_set_the_generator_will_not_have_says_which_parameter() {
    // The renderer carries the generator's own words out, so a panel or a
    // command line can show the parameter that was wrong.
    let ordinary = || params::metadata(&params::by_identity("ordinary").expect("a family"));
    for (parameter, value) in [
        ("envelope", {
            let mut value = ordinary();
            value["skeleton"]["envelope"]["height"] = json!(-4.0);
            value
        }),
        ("leaf length", {
            let mut value = ordinary();
            value["element"]["length"] = json!(-0.1);
            value
        }),
    ] {
        let family = params::parse(&value).expect("the schema takes it; the generator judges it");
        let error = RenderError::from(
            mesh::build(&family, Detail::Full).expect_err("an impossible tree was built anyway"),
        );
        assert!(
            error.to_string().contains(parameter),
            "the refusal does not name {parameter}: {error}"
        );
    }
}

#[test]
fn the_renderer_library_never_names_a_family() {
    // The crate itself is called telperion, so a preset id is no test; what
    // must never appear is the table of families. Only the headless entry
    // point resolves an id, and it is not part of the library.
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut read = 0;
    for entry in std::fs::read_dir(&source).expect("the renderer has a source directory") {
        let path = entry.expect("a source entry").path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a source file");
        assert!(
            !text.contains("presets"),
            "{} knows the family table",
            path.display()
        );
        read += 1;
    }
    assert!(
        read >= 5,
        "only {read} source files were read: check the path"
    );
}

#[test]
fn the_selection_path_names_no_family_and_no_anatomy() {
    // Selection reads sections, deviations and a matrix. A species id or an
    // anatomy's own word appearing here would mean a branch that the level
    // rule exists precisely to avoid.
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let files = [
        source.join("select.rs"),
        source.join("foliage.rs"),
        source.join("shaders/select.wgsl"),
        source.join("shaders/foliage.wgsl"),
    ];
    let anatomies = ["blade", "needle", "lobe", "petiole", "conifer", "broadleaf"];
    for path in files {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("{} is missing", path.display()))
            .to_lowercase();
        for &(_, id, name, _) in params::CATALOGUE {
            for word in [id, name] {
                // One family shares its name with the crate, and every file
                // here imports the crate; that one name proves nothing.
                if word.to_lowercase().contains("telperion") {
                    continue;
                }
                assert!(
                    !text.contains(&word.to_lowercase()),
                    "{} names the family \"{word}\"",
                    path.display()
                );
            }
        }
        for anatomy in anatomies {
            assert!(
                !text.contains(anatomy),
                "{} names the anatomy \"{anatomy}\"",
                path.display()
            );
        }
    }
}
