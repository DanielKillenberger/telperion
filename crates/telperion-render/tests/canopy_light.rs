//! The canopy rows on real frames: a shell of leaves lit as one mass under
//! the canopy normal and falling into its own shade under the crown shade,
//! one leaf facing and facing away from the sun under the other three, and
//! the clay and bare views, which no canopy row may touch.
mod common;
use telperion_core::pipeline::executor;
use telperion_core::{
    foliage::{ElementParams, Instances},
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Foliage, TreeMesh},
    presets::Preset,
    surface::{Bounds, SurfaceMesh},
};
use telperion_render::{render, Camera, Renderer, SceneRow, Still, View, STILL_FORMAT};

const SIZE: u32 = 256;
const CENTRE: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const RADIUS: f64 = 2.0;

/// Every canopy term on at once, each well inside its rail.
fn canopy(row: MaterialParams) -> MaterialParams {
    MaterialParams {
        canopy_normal: 0.8,
        light_wrap: 0.5,
        diffuse_transmission: 1.0,
        leaf_sheen: 0.1,
        crown_shade: 0.5,
        ..row
    }
}

/// A leaf that transmits, so the diffuse share has something to carry.
fn translucent() -> MaterialParams {
    MaterialParams {
        transmission_strength: 0.6,
        ..Default::default()
    }
}

/// A seeded value in -1..1, by xorshift, so the shell is one shell.
fn next(state: &mut u32) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    f64::from(*state) / f64::from(u32::MAX) * 2.0 - 1.0
}

/// A seeded direction, uniform over the sphere.
fn unit(state: &mut u32) -> Vec3 {
    loop {
        let v = Vec3::new(next(state), next(state), next(state));
        if v.length_squared() > 0.01 && v.length_squared() <= 1.0 {
            return v.normalized();
        }
    }
}

/// A shell of leaves about one centre, each turned its own seeded way, and
/// no wood: the crown the canopy normal is measured against is this sphere.
fn shell() -> TreeMesh {
    let element = executor::element(ElementParams {
        cup: 0.0,
        curl: 0.0,
        ..Default::default()
    })
    .unwrap();
    let mut state = 0x9e37_79b9_u32;
    // Leaves a little inside the shell as well as on it, each a column-major
    // placement: its side, its axis and its face scaled up, then its seat.
    // The shell the fixture fills, as a box that holds it.
    let reach_box = RADIUS + 0.4;
    let mut instances = Instances::new(telperion_core::foliage::Reference::spanning(
        CENTRE - Vec3::new(reach_box, reach_box, reach_box),
        CENTRE + Vec3::new(reach_box, reach_box, reach_box),
    ));
    let matrices: Vec<[f32; 16]> = (0..6000)
        .map(|_| {
            let at = CENTRE + unit(&mut state) * (RADIUS * (1.0 - 0.2 * next(&mut state).abs()));
            let face = unit(&mut state);
            let side = face.perpendicular().normalized();
            let axis = face.cross(side);
            let [s, a, f] = [side, axis, face].map(|v| v * 1.6);
            [
                s.x, s.y, s.z, 0.0, a.x, a.y, a.z, 0.0, f.x, f.y, f.z, 0.0, at.x, at.y, at.z, 1.0,
            ]
            .map(|v| v as f32)
        })
        .collect();
    for m in &matrices {
        instances.push(m);
    }
    let reach = RADIUS + 0.2;
    TreeMesh {
        wood: SurfaceMesh::default(),
        foliage: Foliage { element, instances },
        bounds: Bounds {
            min: CENTRE - Vec3::new(reach, reach, reach),
            max: CENTRE + Vec3::new(reach, reach, reach),
        },
    }
}

fn draw(renderer: &mut Renderer, camera: &Camera, row: MaterialParams) -> Still {
    renderer.set_material(row);
    render(renderer, camera, SIZE, SIZE).unwrap()
}

/// The mean of the picture's channels over a band of columns across its
/// middle rows, in 0..255.
fn band(still: &Still, columns: std::ops::Range<u32>) -> f64 {
    let (mut sum, mut count) = (0.0, 0.0);
    for y in SIZE * 3 / 8..SIZE * 5 / 8 {
        for x in columns.clone() {
            let at = ((y * SIZE + x) * 4) as usize;
            sum += still.rgba[at..at + 3]
                .iter()
                .map(|&c| f64::from(c))
                .sum::<f64>()
                / 3.0;
            count += 1.0;
        }
    }
    sum / count
}

#[test]
fn the_canopy_normal_lights_the_sunward_side_of_the_mass_over_the_far_side() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&shell()).unwrap();
    renderer.set_view(View::Whole);
    renderer.set_figure(false);
    // The sun low at +x, which is the picture's right from an eye at +z.
    renderer.set_scene(SceneRow {
        sun_azimuth: 90.0,
        sun_elevation: 10.0,
        ..Default::default()
    });
    let camera = Camera {
        position: CENTRE + Vec3::new(0.0, 0.0, 8.0),
        target: CENTRE,
        field_of_view: 36.0,
        near: 0.1,
        far: 100.0,
    };
    let contrast = |still: &Still| {
        band(still, SIZE * 5 / 8..SIZE * 3 / 4) - band(still, SIZE / 4..SIZE * 3 / 8)
    };
    let cards = draw(&mut renderer, &camera, MaterialParams::default());
    let mass = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            canopy_normal: 1.0,
            ..Default::default()
        },
    );
    let (card, bent) = (contrast(&cards), contrast(&mass));
    assert!(
        bent > card + 8.0,
        "the sunward side led the far side by {card:.1} as cards and {bent:.1} as a mass"
    );
    assert!(
        band(&mass, SIZE * 5 / 8..SIZE * 3 / 4) > band(&cards, SIZE * 5 / 8..SIZE * 3 / 4),
        "the sunward side did not brighten"
    );
}

/// The mean of the picture's channels over a band of rows across its middle
/// columns, in 0..255.
fn rows(still: &Still, rows: std::ops::Range<u32>) -> f64 {
    let (mut sum, mut count) = (0.0, 0.0);
    for y in rows {
        for x in SIZE * 3 / 8..SIZE * 5 / 8 {
            let at = ((y * SIZE + x) * 4) as usize;
            sum += still.rgba[at..at + 3]
                .iter()
                .map(|&c| f64::from(c))
                .sum::<f64>()
                / 3.0;
            count += 1.0;
        }
    }
    sum / count
}

#[test]
fn the_crown_shade_darkens_the_underside_of_the_mass_more_than_its_top() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&shell()).unwrap();
    renderer.set_view(View::Whole);
    renderer.set_figure(false);
    // Overcast: a sun on the horizon behind the eye lights nothing of note,
    // so the sky carries the crown and the shade is the sky's.
    renderer.set_scene(SceneRow {
        sun_azimuth: 180.0,
        sun_elevation: 0.0,
        sun_red: 0.0,
        sun_green: 0.0,
        sun_blue: 0.0,
        ..Default::default()
    });
    let camera = Camera {
        position: CENTRE + Vec3::new(0.0, 0.0, 8.0),
        target: CENTRE,
        field_of_view: 36.0,
        near: 0.1,
        far: 100.0,
    };
    let row = MaterialParams {
        canopy_normal: 0.5,
        ..translucent()
    };
    let open = draw(&mut renderer, &camera, row);
    let shaded = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            crown_shade: 0.5,
            ..row
        },
    );
    let (top, bottom) = (SIZE / 4..SIZE * 3 / 8, SIZE * 5 / 8..SIZE * 3 / 4);
    let kept = |band: std::ops::Range<u32>| rows(&shaded, band.clone()) / rows(&open, band);
    assert!(
        kept(bottom.clone()) < 0.9,
        "the underside kept {:.2}",
        kept(bottom.clone())
    );
    assert!(
        kept(top.clone()) > kept(bottom.clone()) + 0.1,
        "the top kept {:.2} and the underside {:.2}",
        kept(top),
        kept(bottom)
    );
}

/// One leaf at the origin, seen square from +z, under a sun `azimuth`
/// degrees round from in front of it, low over the horizon.
fn one_leaf(azimuth: f64) -> (Renderer, Camera) {
    let gpu = common::gpu().expect("checked by the caller");
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    let mut tree = mesh::build(&family).unwrap();
    tree.foliage.element = executor::element(ElementParams {
        cup: 0.0,
        curl: 0.0,
        ..Default::default()
    })
    .unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Leaf);
    renderer.submit(&tree).unwrap();
    renderer.set_scene(SceneRow {
        sun_azimuth: azimuth,
        sun_elevation: 10.0,
        ..Default::default()
    });
    let camera = Camera {
        position: Vec3::new(0.0, 0.055, 0.3),
        target: Vec3::new(0.0, 0.055, 0.0),
        field_of_view: 38.0,
        near: 0.001,
        far: 2.0,
    };
    (renderer, camera)
}

fn brightness(still: &Still) -> f64 {
    band(still, SIZE * 3 / 8..SIZE * 5 / 8)
}

#[test]
fn each_leaf_term_lights_a_leaf_the_way_its_sun_stands() {
    if common::gpu().is_none() {
        return;
    }
    let off = translucent();
    let gain = |azimuth: f64, row: MaterialParams| {
        let (mut renderer, camera) = one_leaf(azimuth);
        let plain = brightness(&draw(&mut renderer, &camera, off));
        brightness(&draw(&mut renderer, &camera, row)) - plain
    };
    // Facing the sun (in front of the face) and facing away from it (behind
    // it, well off the eye's line so the forward share stays small).
    let (facing, away) = (20.0, 125.0);
    let diffuse = MaterialParams {
        diffuse_transmission: 1.0,
        ..off
    };
    assert!(
        gain(away, diffuse) > gain(facing, diffuse) + 2.0,
        "diffuse transmission lit the leaf facing the sun as much as the one facing away"
    );
    assert!(
        gain(away, diffuse) > 2.0,
        "no diffuse transmission reached the leaf"
    );
    let wrap = MaterialParams {
        light_wrap: 1.0,
        ..off
    };
    assert!(
        gain(100.0, wrap) > 1.0,
        "the wrap did not light a face just past the terminator"
    );
    let sheen = MaterialParams {
        leaf_sheen: 0.3,
        ..off
    };
    for azimuth in [facing, away] {
        assert!(gain(azimuth, sheen) > 1.0, "no sheen at {azimuth}");
    }
    // One leaf on its own has no crown to be part of.
    let bend = MaterialParams {
        canopy_normal: 1.0,
        ..off
    };
    let (mut renderer, camera) = one_leaf(away);
    assert_eq!(
        draw(&mut renderer, &camera, off).rgba,
        draw(&mut renderer, &camera, bend).rgba,
        "the canopy normal moved a leaf that stands alone"
    );
}

#[test]
fn no_canopy_row_touches_the_clay_room_or_the_wood() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let row = translucent();
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(400);
    let wood = mesh::build(&family).unwrap();
    let pose = |tree: &TreeMesh| {
        telperion_render::hero_pose(tree.bounds, 1.0, telperion_render::GROUND_REACH)
    };
    for (tree, view) in [(shell(), View::Clay), (wood, View::Bare)] {
        renderer.submit(&tree).unwrap();
        let camera = pose(&tree);
        renderer.set_view(view);
        let neutral = draw(&mut renderer, &camera, row);
        let on = draw(&mut renderer, &camera, canopy(row));
        assert!(neutral.rgba == on.rgba, "{view:?}: a canopy row moved it");
    }
    let tree = shell();
    renderer.submit(&tree).unwrap();
    let camera = pose(&tree);
    renderer.set_view(View::Whole);
    let neutral = draw(&mut renderer, &camera, row);
    let on = draw(&mut renderer, &camera, canopy(row));
    let moved = neutral
        .rgba
        .iter()
        .zip(&on.rgba)
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        moved > 1000,
        "the canopy rows moved only {moved} channels of the crown"
    );
}
