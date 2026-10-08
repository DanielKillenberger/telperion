//! fn-217 task 3: the floret's form on stills (`STILLS=<dir> cargo test
//! --profile ci -p telperion-render --test bloom_form -- --ignored`). A
//! young beech flowers on its outer ages; each shot stands outside the crown
//! at one inflorescence on the camera's side.
mod common;
use telperion_core::foliage::BloomParams;
use telperion_core::presets::Preset;
use telperion_core::{math::Vec3, Family};
use telperion_render::{
    hero_pose, render, write_png, Camera, Level, Renderer, SceneRow, View, GROUND_REACH,
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

/// Laurelin's racemes on existing rows (DESIGN.md 7.5): narrow-opened
/// petals, widest at the mouth and cupped, hanging on a long axis.
fn horn(f: &mut Family, open: f64, cup: f64) {
    f.bloom = BloomParams {
        florets: 14.0,
        rachis: 0.16,
        droop: 1.0,
        pedicel: 0.012,
        spread: 70.0,
        taper: 0.35,
        petals: 5.0,
        open,
        petal_length: 0.024,
        petal_width: 0.016,
        petal_widest: 0.85,
        petal_cup: cup,
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

/// One still of `family` from `camera` at `w` x `w`, written to `dir`.
fn shoot(renderer: &mut Renderer, family: &Family, camera: &Camera, w: u32, path: String) {
    let tree = telperion_core::mesh::build(family).unwrap();
    renderer.submit_at(&tree, Level::Chosen).unwrap();
    renderer.set_family(family);
    renderer.set_view(View::Whole);
    renderer.set_scene(SceneRow {
        sun_azimuth: 115.0,
        sun_elevation: 40.0,
        ..SceneRow::default()
    });
    let still = render(renderer, camera, w, w).unwrap();
    write_png(std::path::Path::new(&path), &still).unwrap();
}

#[test]
#[ignore]
fn stills_of_the_floret_s_form() {
    let Some(gpu) = common::gpu() else { return };
    let dir = std::env::var("STILLS").expect("STILLS names the directory the stills go to");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    // The horn on existing rows: two openings and cups of one raceme, a
    // sparse flowering so one raceme stands clear, whole and close.
    for (open, cup) in [(18.0, 0.6), (30.0, 1.4)] {
        let mut f = flowering(20.0, 0.3);
        horn(&mut f, open, cup);
        // The leaves shrunk to specks, so no blade stands between the eye
        // and the raceme it looks at.
        (f.element.length, f.element.connector_length) = (0.003, 0.0005);
        for (shot, distance, drop) in [("raceme", 0.35, 0.0), ("floret", 0.1, 0.0)] {
            let camera = at_a_bloom(&f, distance, drop);
            let path = format!("{dir}/horn-{shot}-open{open}-cup{cup}.png");
            shoot(&mut renderer, &f, &camera, 512, path);
        }
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
