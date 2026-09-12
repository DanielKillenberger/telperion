//! What each view writes into the sun's map, on the hardware that draws it.
//! The map is read back rather than judged by eye: a texel still at the clear
//! depth is a texel nothing stood in front of, so the share of the map that is
//! not the clear is exactly what the pass drew into it.
mod common;

use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{hero_pose, render, Renderer, SceneRow, View, GROUND_REACH, STILL_FORMAT};

/// The share of the map carrying a caster's depth rather than the clear.
fn covered(depths: &[f32]) -> f64 {
    depths.iter().filter(|depth| **depth < 1.0).count() as f64 / depths.len() as f64
}

#[test]
fn the_sun_sees_the_wood_and_the_whole_crown_and_the_leaf_view_is_not_in_its_way() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&Preset::Ordinary.parameters(), Detail::Full).expect("the tree grew");
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
    for preset in [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let tree = mesh::build(&preset.parameters(), Detail::Full).unwrap();
        renderer.set_scene(SceneRow::default());
        renderer.submit(&tree).unwrap();
        let counts = (renderer.caster_triangles(), renderer.caster_instances());
        assert!(counts.0 > 0 && counts.1 > 0, "{preset:?}: {counts:?}");
        assert_eq!(counts.1, (tree.foliage_instances() as u32).div_ceil(4));
        let camera = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
        for turn in [0.0, 0.25] {
            let camera = telperion_render::orbit_pose(&camera, turn);
            render(&mut renderer, &camera, 32, 32).unwrap();
            assert_eq!(
                (renderer.caster_triangles(), renderer.caster_instances()),
                counts
            );
        }
        renderer.set_scene(SceneRow {
            caster_texels: 0.0,
            caster_stride: 1.0,
            ..Default::default()
        });
        assert_eq!(
            renderer.caster_triangles(),
            tree.wood.indices.len() as u32 / 3
        );
        assert_eq!(renderer.caster_instances(), tree.foliage_instances() as u32);
        renderer.set_scene(SceneRow::default());
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
    use telperion_core::foliage::{build_element, ElementParams};
    let Some(gpu) = common::gpu() else { return };
    let mut tree = mesh::build(&Preset::Ordinary.parameters(), Detail::Full).unwrap();
    tree.foliage.element = build_element(ElementParams {
        card: true,
        width: 1.0,
        length: 1.0,
        ..Default::default()
    })
    .unwrap();
    tree.foliage.instances.matrices = (0..4)
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
        .collect();
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
    let all_wood = share(
        View::Whole,
        SceneRow {
            caster_texels: 0.0,
            ..Default::default()
        },
    );
    let all_foliage = share(
        View::Whole,
        SceneRow {
            caster_stride: 1.0,
            ..Default::default()
        },
    );
    assert!(all_wood >= whole, "full wood {all_wood} < default {whole}");
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

    // A real, uniformly shrunken wood surface under this wide fit has no run
    // above an in-range threshold. Empty draws must leave a clear map.
    for radius in &mut tree.wood.run_table {
        radius.largest_radius *= 1e-4;
    }
    for coordinate in &mut tree.wood.positions {
        *coordinate *= 1e-4;
    }
    if let Some(bounds) = &mut tree.wood.bounds {
        bounds.min = bounds.min * 1e-4;
        bounds.max = bounds.max * 1e-4;
    }
    renderer.set_scene(SceneRow {
        caster_texels: 8.0,
        caster_stride: 64.0,
        ..Default::default()
    });
    renderer.submit(&tree).unwrap();
    assert_eq!(renderer.caster_triangles(), 0);
    assert_eq!(renderer.caster_instances(), 0);
    render(&mut renderer, &camera, 32, 32).unwrap();
    assert_eq!(covered(&renderer.shadow_depths().unwrap()), 0.0);
}
