//! What each view writes into the sun's map, on the hardware that draws it.
//! The map is read back rather than judged by eye: a texel still at the clear
//! depth is a texel nothing stood in front of, so the share of the map that is
//! not the clear is exactly what the pass drew into it.
mod common;

use telperion_core::pipeline::executor;

use telperion_core::{math::Vec3, mesh, presets::Preset};
use telperion_render::{
    figure_counts, figure_placement, hero_pose, render, Camera, Renderer, SceneRow, View,
    GROUND_REACH, STILL_FORMAT,
};

/// The share of the map carrying a caster's depth rather than the clear.
fn covered(depths: &[f32]) -> f64 {
    depths.iter().filter(|depth| **depth < 1.0).count() as f64 / depths.len() as f64
}

#[test]
fn the_sun_sees_the_wood_and_the_whole_crown_and_the_leaf_view_is_not_in_its_way() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");

    let mut share = |view: View, row: SceneRow| {
        renderer.set_scene(row);
        renderer.set_view(view);
        let bounds = renderer.bounds().expect("a tree is on stage");
        let camera = hero_pose(bounds, 1.0, GROUND_REACH);
        let still = render(&mut renderer, &camera, 128, 128).expect("the frame was drawn");
        assert!(still.has_subject(), "{view:?} came out as one flat colour");
        covered(&renderer.shadow_depths().expect("the map came back"))
    };
    let whole = share(View::Whole, SceneRow::default());
    let bare = share(View::Bare, SceneRow::default());
    let leaf = share(View::Leaf, SceneRow::default());

    assert!(whole > 0.0);
    assert!(bare > 0.0, "the wood cast nothing into the map");
    assert!(
        whole > bare,
        "the crown added nothing to the wood's own shadow: {whole} against {bare}"
    );
    assert_eq!(
        leaf, 0.0,
        "the leaf view wrote {leaf} of a map the sun does not draw for it"
    );
}

#[test]
fn every_preset_has_default_casters_fixed_under_orbit_and_row_changes() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    // The presets the engine grows (fn-198).
    for preset in [
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::EuropeanBeech,
        Preset::DatePalm,
    ] {
        let tree = mesh::build(&preset.parameters()).unwrap();
        renderer.set_scene(SceneRow::default());
        renderer.submit(&tree).unwrap();
        let camera = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
        // The sun's wood is surfaced each frame at its own texels' error
        // (fn-208), so it is counted after a frame, and the camera's orbit
        // does not move it.
        render(&mut renderer, &camera, 32, 32).unwrap();
        let counts = (renderer.caster_triangles(), renderer.caster_instances());
        assert!(counts.0 > 0 && counts.1 > 0, "{preset:?}: {counts:?}");
        assert_eq!(counts.1, (tree.foliage_instances() as u32).div_ceil(4));
        for turn in [0.0, 0.25] {
            let camera = telperion_render::orbit_pose(&camera, turn);
            render(&mut renderer, &camera, 32, 32).unwrap();
            assert_eq!(
                (renderer.caster_triangles(), renderer.caster_instances()),
                counts
            );
        }
        renderer.set_scene(SceneRow {
            caster_stride: 1.0,
            ..Default::default()
        });
        render(&mut renderer, &camera, 32, 32).unwrap();
        assert_eq!(renderer.caster_triangles(), counts.0);
        assert_eq!(renderer.caster_instances(), tree.foliage_instances() as u32);
        renderer.set_scene(SceneRow::default());
        render(&mut renderer, &camera, 32, 32).unwrap();
        assert_eq!(
            (renderer.caster_triangles(), renderer.caster_instances()),
            counts
        );
    }
}

/// Enlargement can fill gaps between overlapping leaves, so union coverage is
/// not monotone on an arbitrary crown. Here four separated cards have known
/// coverage: the first is unit size and the other three are twice its size.
/// Keeping the first and doubling it cannot exceed the full set's area.
#[test]
fn full_casters_cover_at_least_the_default_on_separated_surfaces() {
    use telperion_core::foliage::ElementParams;
    let Some(gpu) = common::gpu() else { return };
    let mut tree = mesh::build(&common::ordinary()).unwrap();
    tree.foliage.element = executor::element(ElementParams {
        card: true,
        width: 1.0,
        length: 1.0,
        ..Default::default()
    })
    .unwrap();
    // A box the four casters stand inside, so where each one stands is read
    // back off its words as it was written.
    tree.foliage.instances =
        telperion_core::foliage::Instances::new(telperion_core::foliage::Reference::spanning(
            telperion_core::math::Vec3::new(-8.0, -8.0, -8.0),
            telperion_core::math::Vec3::new(8.0, 8.0, 8.0),
        ));
    for m in (0..4)
        .map(|i| {
            let scale = if i == 0 { 1.0 } else { 2.0 };
            [
                scale,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                scale,
                0.0,
                0.0,
                -scale,
                0.0,
                0.0,
                -6.0 + i as f32 * 4.0,
                25.0,
                0.0,
                1.0,
            ]
        })
        .collect::<Vec<[f32; 16]>>()
    {
        tree.foliage.instances.push(&m);
    }
    tree.bounds.max.y = tree.bounds.max.y.max(28.0);
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    let camera = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
    let mut share = |view, row| {
        renderer.set_view(view);
        renderer.set_scene(row);
        render(&mut renderer, &camera, 32, 32).unwrap();
        covered(&renderer.shadow_depths().unwrap())
    };
    let whole = share(View::Whole, SceneRow::default());
    let all_foliage = share(
        View::Whole,
        SceneRow {
            caster_stride: 1.0,
            ..Default::default()
        },
    );
    assert!(
        all_foliage >= whole,
        "full foliage {all_foliage} < default {whole}"
    );
    assert!(whole > 0.0);
    renderer.set_scene(SceneRow {
        caster_stride: 64.0,
        ..Default::default()
    });
    assert_eq!(renderer.caster_instances(), 0);
    render(&mut renderer, &camera, 32, 32).unwrap();
    renderer.set_scene(SceneRow {
        caster_stride: 1.0,
        ..Default::default()
    });
    assert_eq!(renderer.caster_instances(), 4);

    // No wood, no leaves the stride keeps and no person: empty draws must
    // leave a clear map.
    tree.curve = Default::default();
    renderer.set_figure(false);
    renderer.set_scene(SceneRow {
        caster_stride: 64.0,
        ..Default::default()
    });
    renderer.submit(&tree).unwrap();
    render(&mut renderer, &camera, 32, 32).unwrap();
    assert_eq!(renderer.caster_triangles(), 0);
    assert_eq!(renderer.caster_instances(), 0);
    assert_eq!(covered(&renderer.shadow_depths().unwrap()), 0.0);
}

/// The person casts into the sun's map wherever the room draws it, whole,
/// and nowhere the figure is off or the leaf view stands alone (fn-310).
#[test]
fn the_person_is_a_caster_only_while_the_room_draws_it() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    let camera = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
    let person = figure_counts().1;
    for (view, figure, casters) in [
        (View::Whole, true, person),
        (View::Bare, true, person),
        (View::Whole, false, 0),
        (View::Bare, false, 0),
        (View::Leaf, true, 0),
    ] {
        renderer.set_view(view);
        renderer.set_figure(figure);
        render(&mut renderer, &camera, 32, 32).unwrap();
        assert_eq!(
            renderer.figure_casters(),
            casters,
            "{view:?} with the figure {figure}"
        );
    }
}

/// The person's shadow on the floor, under a sun standing behind it from the
/// tree (fn-310 task 5): a floor point in it is darker than with the figure
/// off, and the tree's own casters write the map exactly as they did.
#[test]
fn the_persons_shadow_darkens_the_floor_and_leaves_the_trees_alone() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    let foot = figure_placement(tree.bounds, SceneRow::default().sun_azimuth)
        .0
        .foot;
    let row = SceneRow {
        sun_azimuth: foot.x.atan2(foot.z).to_degrees().rem_euclid(360.0),
        sun_elevation: 30.0,
        ..SceneRow::default()
    };
    renderer.set_scene(row);
    // Where the sun throws the person's chest, seen from above it.
    let (azimuth, elevation) = (row.sun_azimuth.to_radians(), row.sun_elevation.to_radians());
    let along = 1.2 / elevation.tan();
    let point = Vec3::new(
        foot.x - along * azimuth.sin(),
        0.0,
        foot.z - along * azimuth.cos(),
    );
    let camera = Camera {
        position: Vec3::new(point.x + 0.3, 4.0, point.z + 0.3),
        target: point,
        field_of_view: 20.0,
        near: 0.1,
        far: 100.0,
    };
    let mut floor = |figure: bool| {
        renderer.set_figure(figure);
        let still = render(&mut renderer, &camera, 32, 32).unwrap();
        let mut sum = 0.0;
        for y in 14..18 {
            for x in 14..18 {
                let i = (y * 32 + x) * 4;
                sum += still.rgba[i..i + 3]
                    .iter()
                    .map(|c| f64::from(*c))
                    .sum::<f64>();
            }
        }
        (sum, renderer.shadow_depths().unwrap())
    };
    let (open, alone) = floor(false);
    let (shaded, with) = floor(true);
    assert!(
        shaded < 0.85 * open,
        "the person's shadow left the floor at {shaded} against {open}"
    );
    let changed = alone.iter().zip(&with).filter(|(a, w)| a != w).count();
    assert!(changed > 0, "the person wrote nothing into the map");
    assert!(
        alone.iter().zip(&with).all(|(a, w)| w <= a),
        "the person's map is not the tree's with the person in front"
    );
    assert!(
        (changed as f64) < 0.01 * alone.len() as f64,
        "{changed} texels moved: the tree's own map was fitted again"
    );
}
