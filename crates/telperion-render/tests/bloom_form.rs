//! fn-217 task 3: the floret's form on stills (`STILLS=<dir> cargo test
//! --profile ci -p telperion-render --test bloom_form -- --ignored`). A
//! young beech flowers on its outer ages; each shot stands outside the crown
//! at one inflorescence on the camera's side.
mod common;
use telperion_core::foliage::BloomParams;
use telperion_core::presets::Preset;
use telperion_core::{math::Vec3, Family};
use telperion_render::{
    hero_pose, render, write_png, Camera, Level, Renderer, SceneRow, Still, View, GROUND_REACH,
    STILL_FORMAT,
};

/// A beech of `age` whose outer ages flower at `share`.
fn flowering(age: f64, share: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (age, 1);
    let species = f.space.species.as_mut().unwrap();
    for k in [12, 14] {
        species.states[k].flowering = share;
    }
    f
}

/// Laurelin's racemes (DESIGN.md 9.2): small narrow-opened florets packed
/// close along a hanging axis, each petal curled out at `curl` into a
/// horn's mouth.
fn horn(f: &mut Family, curl: f64) {
    f.bloom = BloomParams {
        florets: 24.0,
        rachis: 0.16,
        droop: 1.0,
        pedicel: 0.008,
        spread: 25.0,
        taper: 0.35,
        petals: 5.0,
        open: 20.0,
        petal_length: 0.012,
        petal_width: 0.008,
        petal_widest: 0.85,
        petal_cup: 0.6,
        petal_curl: curl,
        scatter: 0.1,
        ..f.bloom
    };
    let m = &mut f.material;
    (m.bloom_front_red, m.bloom_front_green, m.bloom_front_blue) = (0.13, 0.075, 0.008);
    (m.bloom_back_red, m.bloom_back_green, m.bloom_back_blue) = (0.11, 0.06, 0.006);
}

/// A cherry's corymb (fn-216's beech blossom): white five-petal florets on
/// pendent stalks.
fn blossom(f: &mut Family) {
    f.bloom = BloomParams {
        florets: 4.0,
        rachis: 0.0,
        droop: 0.6,
        pedicel: 0.03,
        spread: 60.0,
        petals: 5.0,
        open: 75.0,
        petal_length: 0.014,
        petal_width: 0.012,
        petal_widest: 0.62,
        petal_cup: 0.35,
        scatter: 0.12,
        ..f.bloom
    };
    let m = &mut f.material;
    (m.bloom_front_red, m.bloom_front_green, m.bloom_front_blue) = (0.1, 0.097, 0.09);
    (m.bloom_back_red, m.bloom_back_green, m.bloom_back_blue) = (0.085, 0.082, 0.076);
}

/// A camera `distance` out from the outermost whole floret on the hero camera's
/// side, half-way up the crown, looking at the point `drop` below it.
fn at_a_bloom(family: &Family, distance: f64, drop: f64) -> Camera {
    let mesh = telperion_core::mesh::build(family).unwrap();
    let bounds = mesh.bounds;
    let hero = hero_pose(bounds, 1.0, GROUND_REACH);
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z).normalized();
    let height = bounds.max.y - bounds.min.y;
    let organ = mesh.organ.expect("the tree flowers");
    let floret = organ
        .instances
        .leaves
        .iter()
        // A floret grown in whole, not one opening or fading.
        .filter(|l| organ.instances.reference.scale(**l) > 0.6)
        .map(|l| organ.instances.reference.position(*l))
        .filter(|p| (0.45..0.8).contains(&(p.y / height)))
        .max_by(|a, b| a.dot(toward).total_cmp(&b.dot(toward)))
        .expect("a floret half-way up the crown");
    let target = floret - Vec3::Y * drop;
    Camera {
        position: target + toward * distance + Vec3::Y * (0.2 * distance),
        target,
        field_of_view: 38.0,
        near: 0.005,
        far: 500.0,
    }
}

/// One still of `family` from `camera` at `w` x `w`.
fn still(renderer: &mut Renderer, family: &Family, camera: &Camera, w: u32) -> Still {
    let tree = telperion_core::mesh::build(family).unwrap();
    renderer.submit_at(&tree, Level::Chosen).unwrap();
    renderer.set_family(family);
    renderer.set_view(View::Whole);
    renderer.set_scene(SceneRow {
        sun_azimuth: 115.0,
        sun_elevation: 40.0,
        ..SceneRow::default()
    });
    render(renderer, camera, w, w).unwrap()
}

/// One still of `family` from `camera` at `w` x `w`, written to `path`.
fn shoot(renderer: &mut Renderer, family: &Family, camera: &Camera, w: u32, path: String) {
    let still = still(renderer, family, camera, w);
    write_png(std::path::Path::new(&path), &still).unwrap();
}

fn changed(a: &Still, b: &Still) -> usize {
    a.rgba.iter().zip(&b.rgba).filter(|(x, y)| x != y).count()
}

/// The eye and the stamens are dormant at zero, to the pixel, and each
/// changes the floret where it is set (fn-217 F1, F2).
#[test]
fn the_eye_and_the_stamens_act_only_where_set() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let mut f = flowering(20.0, 0.6);
    blossom(&mut f);
    let camera = at_a_bloom(&f, 0.1, 0.0);
    let plain = still(&mut renderer, &f, &camera, 256);
    // An eye colour with no share, and stamens' size with no count.
    let mut dormant = f.clone();
    let m = &mut dormant.material;
    (m.bloom_eye_red, m.bloom_eye_green, m.bloom_eye_blue) = (0.3, 0.0, 0.0);
    (dormant.bloom.stamen_length, dormant.bloom.stamen_width) = (0.02, 0.002);
    assert_eq!(
        changed(&plain, &still(&mut renderer, &dormant, &camera, 256)),
        0
    );
    let mut eye = dormant.clone();
    eye.material.bloom_eye_share = 0.4;
    let tinted = changed(&plain, &still(&mut renderer, &eye, &camera, 256));
    assert!(tinted > 500, "the eye moved {tinted} values");
    let mut stamens = f.clone();
    stamens.bloom.stamens = 20.0;
    let ringed = changed(&plain, &still(&mut renderer, &stamens, &camera, 256));
    assert!(ringed > 200, "the stamens moved {ringed} values");
}

#[test]
#[ignore]
fn stills_of_the_floret_s_form() {
    let Some(gpu) = common::gpu() else { return };
    let dir = std::env::var("STILLS").expect("STILLS names the directory the stills go to");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    // The petal's curl on one floret, straight and flared, the leaves
    // shrunk to specks so no blade stands in front of it.
    for curl in [0.0, 0.6] {
        let mut f = flowering(20.0, 1.0);
        horn(&mut f, curl);
        (f.element.length, f.element.connector_length) = (0.003, 0.0005);
        let camera = at_a_bloom(&f, 0.06, 0.0);
        shoot(
            &mut renderer,
            &f,
            &camera,
            512,
            format!("{dir}/horn-floret-curl{curl}.png"),
        );
    }
    // The raceme among its leaves, close and from the middle distance.
    let mut f = flowering(20.0, 1.0);
    horn(&mut f, 0.6);
    for (shot, distance) in [("close", 0.35), ("middle", 1.5)] {
        let camera = at_a_bloom(&f, distance, 0.08);
        shoot(
            &mut renderer,
            &f,
            &camera,
            512,
            format!("{dir}/horn-raceme-{shot}.png"),
        );
    }
    let mut f = flowering(20.0, 0.6);
    blossom(&mut f);
    let camera = at_a_bloom(&f, 0.22, 0.0);
    shoot(
        &mut renderer,
        &f,
        &camera,
        512,
        format!("{dir}/blossom-close.png"),
    );
}
