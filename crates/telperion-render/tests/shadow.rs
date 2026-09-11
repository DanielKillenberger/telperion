//! What each view writes into the sun's map, on the hardware that draws it.
//! The map is read back rather than judged by eye: a texel still at the clear
//! depth is a texel nothing stood in front of, so the share of the map that is
//! not the clear is exactly what the pass drew into it.
mod common;

use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{hero_pose, render, Renderer, View, GROUND_REACH, STILL_FORMAT};

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

    let mut share = |view: View| {
        renderer.set_view(view);
        let bounds = renderer.bounds().expect("a tree is on stage");
        let camera = hero_pose(bounds, 1.0, GROUND_REACH);
        let still = render(&mut renderer, &camera, 128, 128).expect("the frame was drawn");
        assert!(still.has_subject(), "{view:?} came out as one flat colour");
        covered(&renderer.shadow_depths().expect("the map came back"))
    };
    let whole = share(View::Whole);
    let bare = share(View::Bare);
    let leaf = share(View::Leaf);

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
