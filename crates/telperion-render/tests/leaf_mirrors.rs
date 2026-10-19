//! fn-217 E2: the supernatural group's two leaf mirrors - a metal on the
//! underside and on the margin band - act only where their reflectance is
//! above zero, and only on the face each names.
mod common;
use telperion_core::presets::Preset;
use telperion_core::supernatural::Supernatural;
use telperion_core::{math::Vec3, mesh, Family};
use telperion_render::{render, Camera, Level, Renderer, SceneRow, Still, View, STILL_FORMAT};

/// A young beech, its leaf drawn alone.
fn beech() -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (10.0, 1);
    f
}

/// The group's mirrors at `back` and `margin`, silver beneath.
fn mirrors(enabled: bool, back: f64, margin: f64) -> Supernatural {
    Supernatural {
        enabled,
        back_reflectance: back,
        back_red: 0.92,
        back_green: 0.94,
        back_blue: 0.97,
        margin_reflectance: margin,
        roughness: 0.25,
        ..Supernatural::NONE
    }
}

/// The leaf from in front or behind, the sun low behind the eye's side.
fn leaf(renderer: &mut Renderer, back: bool) -> Still {
    let side = if back { -1.0 } else { 1.0 };
    let camera = Camera {
        position: Vec3::new(0.0, 0.055, 0.3 * side),
        target: Vec3::new(0.0, 0.055, 0.0),
        field_of_view: 38.0,
        near: 0.001,
        far: 2.0,
    };
    renderer.set_scene(SceneRow {
        sun_azimuth: if back { 180.0 } else { 0.0 },
        sun_elevation: 20.0,
        ..Default::default()
    });
    render(renderer, &camera, 192, 192).unwrap()
}

fn changed(a: &Still, b: &Still) -> usize {
    a.rgba.iter().zip(&b.rgba).filter(|(x, y)| x != y).count()
}

#[test]
fn the_mirrors_are_neutral_off_and_at_zero_and_each_acts_on_its_own_face() {
    let Some(gpu) = common::gpu() else { return };
    let family = beech();
    let tree = mesh::build(&family).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Leaf);
    renderer.submit_at(&tree, Level::Forced(0)).unwrap();
    let mut with = |g: Option<Supernatural>, back: bool| {
        match g {
            None => renderer.set_material(family.material),
            Some(g) => renderer.set_family(&Family {
                supernatural: g,
                ..family.clone()
            }),
        }
        leaf(&mut renderer, back)
    };
    for back in [false, true] {
        let natural = with(None, back);
        // Off, every mirror set draws the natural leaf; on, at zero, too.
        let off = with(Some(mirrors(false, 0.8, 0.8)), back);
        assert_eq!(changed(&natural, &off), 0, "off, back={back}");
        let zero = with(Some(mirrors(true, 0.0, 0.0)), back);
        assert_eq!(changed(&natural, &zero), 0, "zero, back={back}");
        // The underside's mirror on the underside alone.
        let silver = changed(&natural, &with(Some(mirrors(true, 0.8, 0.0)), back));
        if back {
            assert!(silver > 100, "silver from behind: {silver}");
        } else {
            assert_eq!(silver, 0, "silver from in front");
        }
        // The rim's on either face.
        let rim = changed(&natural, &with(Some(mirrors(true, 0.0, 0.8)), back));
        assert!(rim > 20, "rim, back={back}: {rim}");
    }
}

/// Two stills side by side, `a` left of `b`.
fn beside(a: &Still, b: &Still) -> Still {
    let (w, h) = (a.width as usize, a.height as usize);
    let mut rgba = Vec::with_capacity(2 * w * h * 4);
    for row in 0..h {
        rgba.extend_from_slice(&a.rgba[row * w * 4..][..w * 4]);
        rgba.extend_from_slice(&b.rgba[row * w * 4..][..w * 4]);
    }
    Still {
        width: 2 * a.width,
        height: a.height,
        rgba,
        stats: a.stats,
    }
}

/// The R2 stills of a test tree (`STILLS=<dir> cargo test --profile ci -p
/// telperion-render --test leaf_mirrors -- --ignored`): the young beech, the
/// natural tree left and the group on right, silver beneath and a gold
/// rim, each whole and close.
#[test]
#[ignore]
fn stills_of_silver_beneath_and_a_gold_rim() {
    use telperion_render::{hero_pose, write_png, GROUND_REACH};
    let Some(gpu) = common::gpu() else { return };
    let dir = std::env::var("STILLS").expect("STILLS names the directory the stills go to");
    let mut natural = Preset::EuropeanBeech.parameters();
    (natural.age, natural.skeleton.seed) = (20.0, 1);
    let tree = mesh::build(&natural).unwrap();
    let mut silver = natural.clone();
    silver.supernatural = mirrors(true, 0.7, 0.0);
    silver.supernatural.roughness = 0.35;
    let mut gold = natural.clone();
    gold.supernatural = mirrors(true, 0.0, 0.9);
    let m = &mut gold.supernatural.material;
    (m.margin_red, m.margin_green, m.margin_blue) = (Some(0.6), Some(0.42), Some(0.03));
    let (w, h) = (640, 480);
    let whole = hero_pose(tree.bounds, f64::from(w) / f64::from(h), GROUND_REACH);
    let (lo, hi) = (tree.bounds.min, tree.bounds.max);
    let height = hi.y - lo.y;
    let toward = Vec3::new(whole.position.x, 0.0, whole.position.z).normalized();
    // Under the crown's edge looking up into it, and at a limb from 3 m.
    let under = Camera {
        position: toward * (0.35 * (hi.x - lo.x)) + Vec3::new(0.0, 1.4, 0.0),
        target: Vec3::new(0.0, 0.6 * height, 0.0),
        field_of_view: 50.0,
        near: 0.05,
        far: 200.0,
    };
    let limb_at = Vec3::new(0.0, 0.55 * height, 0.0) + Vec3::new(toward.z, 0.0, -toward.x);
    let limb = Camera {
        position: limb_at + toward * 2.5,
        target: limb_at,
        field_of_view: 38.0,
        near: 0.05,
        far: 200.0,
    };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit_at(&tree, Level::Chosen).unwrap();
    renderer.set_view(View::Whole);
    let shots = [
        ("silver-whole", &silver, &whole, (115.0, 35.0)),
        ("silver-under", &silver, &under, (295.0, 25.0)),
        ("gold-whole", &gold, &whole, (115.0, 35.0)),
        ("gold-limb", &gold, &limb, (115.0, 35.0)),
    ];
    for (name, legend, camera, (azimuth, elevation)) in shots {
        renderer.set_scene(SceneRow {
            sun_azimuth: azimuth,
            sun_elevation: elevation,
            ..SceneRow::default()
        });
        renderer.set_family(&natural);
        let left = render(&mut renderer, camera, w, h).unwrap();
        renderer.set_family(legend);
        let right = render(&mut renderer, camera, w, h).unwrap();
        println!("{name}: {} pixels apart", changed(&left, &right));
        let path = std::path::Path::new(&dir).join(format!("{name}.png"));
        write_png(&path, &beside(&left, &right)).unwrap();
    }
}
