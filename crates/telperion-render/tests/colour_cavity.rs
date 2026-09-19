//! Colour and contact shade the existing wood; crown depth hides only sky.
mod common;
use telperion_core::{
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail, TreeMesh},
    presets::Preset,
    surface::SurfaceMesh,
};
use telperion_render::{
    hero_pose, render, Camera, Level, Renderer, SceneRow, Still, View, GROUND_REACH, STILL_FORMAT,
};

fn tree() -> TreeMesh {
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(400);
    mesh::build(&family, Detail::Full).unwrap()
}

fn hash(mesh: &SurfaceMesh) -> u64 {
    mesh.positions
        .iter()
        .chain(&mesh.normals)
        .chain(&mesh.coords)
        .flat_map(|v| v.to_le_bytes())
        .chain(mesh.indices.iter().flat_map(|v| v.to_le_bytes()))
        .fold(14695981039346656037, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(1099511628211)
        })
}

fn trunk() -> Camera {
    Camera {
        position: Vec3::new(2.5, 2.0, 3.5),
        target: Vec3::new(0.0, 2.0, 0.0),
        field_of_view: 38.0,
        near: 0.01,
        far: 100.0,
    }
}

fn luminance(frame: &Still) -> f64 {
    frame
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|p| {
            // Monotonic comparisons use the same display transform in both frames.
            0.2126 * f64::from(p[0]) + 0.7152 * f64::from(p[1]) + 0.0722 * f64::from(p[2])
        })
        .sum()
}

fn draw(renderer: &mut Renderer, camera: &Camera, row: MaterialParams) -> Still {
    renderer.set_material(row);
    render(renderer, camera, 192, 192).unwrap()
}

#[test]
fn fissures_crests_and_mottle_change_colour_without_changing_wood() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(400);
    let off = MaterialParams {
        ridge_scale: 0.08,
        plate_scale: 0.16,
        ..Default::default()
    };
    family.material = off;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    let plain = draw(&mut renderer, &trunk(), off);
    for (name, row, direction) in [
        (
            "fissure",
            MaterialParams {
                fissure_red: -0.12,
                fissure_green: -0.08,
                fissure_blue: 0.005,
                fissure_strength: 0.8,
                ..off
            },
            -1.0,
        ),
        (
            "crest",
            MaterialParams {
                crest_red: 0.15,
                crest_green: 0.12,
                crest_blue: 0.08,
                crest_strength: 0.8,
                ..off
            },
            1.0,
        ),
        (
            "mottle",
            MaterialParams {
                bark_mottle_scale: 0.7,
                bark_mottle_strength: 0.8,
                ..off
            },
            0.0,
        ),
    ] {
        family.material = row;
        let on_tree = mesh::build(&family, Detail::Full).unwrap();
        assert_eq!(
            hash(&tree.wood),
            hash(&on_tree.wood),
            "{name} changed wood bytes"
        );
        let on = draw(&mut renderer, &trunk(), row);
        let changed = plain
            .rgba
            .iter()
            .zip(&on.rgba)
            .filter(|(a, b)| a != b)
            .count();
        assert!(changed > 100, "{name}: {changed} channels");
        if direction != 0.0 {
            // Unchanged pixels cancel: this also tests the changed trunk pixels' mean.
            assert!(
                direction * (luminance(&on) - luminance(&plain)) > 100.0,
                "{name} luminance"
            );
        }
        assert_eq!(
            on.rgba,
            draw(&mut renderer, &trunk(), row).rgba,
            "{name} redraw"
        );
    }
}

#[test]
fn cavity_cuts_sun_and_sky_and_ground_contact_needs_no_relief() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree()).unwrap();
    renderer.set_view(View::Bare);
    for sun in [true, false] {
        renderer.set_scene(if sun {
            SceneRow::default()
        } else {
            SceneRow {
                sun_red: 0.0,
                sun_green: 0.0,
                sun_blue: 0.0,
                ..Default::default()
            }
        });
        for relief in [true, false] {
            let off = MaterialParams {
                ridge_scale: if relief { 0.08 } else { 0.0 },
                plate_scale: 0.16,
                ..Default::default()
            };
            let camera = if relief {
                trunk()
            } else {
                Camera {
                    position: Vec3::new(2.5, 0.3, 3.5),
                    target: Vec3::new(0.0, 0.3, 0.0),
                    ..trunk()
                }
            };
            let plain = draw(&mut renderer, &camera, off);
            let dark = draw(
                &mut renderer,
                &camera,
                MaterialParams {
                    cavity_strength: 0.8,
                    ..off
                },
            );
            assert!(
                luminance(&dark) < luminance(&plain) - 100.0,
                "sun={sun}, relief={relief}"
            );
        }
    }
}

// Select visible material independently of occlusion by brightening just that
// material. A central disk crops the crown; sky, ground and the other material
// do not change in the marker frame and cannot dilute its luminance sum.
fn crown_sums(plain: &Still, dark: &Still, marker: &Still) -> (usize, f64, f64) {
    let luma = |p: &[u8; 4]| {
        0.2126 * f64::from(p[0]) + 0.7152 * f64::from(p[1]) + 0.0722 * f64::from(p[2])
    };
    plain
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(dark.rgba.as_chunks::<4>().0)
        .zip(marker.rgba.as_chunks::<4>().0)
        .enumerate()
        .filter(|(i, ((p, _), m))| {
            let x = (i % 512) as f64 + 0.5 - 256.0;
            let y = (i / 512) as f64 + 0.5 - 256.0;
            x * x + y * y < 192.0_f64.powi(2) && luma(m) > luma(p) + 32.0
        })
        .fold((0, 0.0, 0.0), |(count, p, d), (_, ((plain, dark), _))| {
            (count + 1, p + luma(plain), d + luma(dark))
        })
}

#[test]
fn crown_occlusion_darkens_wood_and_leaves_but_not_a_single_leaf_or_clay() {
    // The 400-node fixture stops before leaf-bearing shoots exist: no leaves
    // means no crown ellipsoid and exactly zero depth in Whole AND Bare.
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = None;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let placements = &tree.foliage.instances;
    assert!(
        placements.len() > 1000,
        "fixture must have a developed crown"
    );
    let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = -min;
    for index in 0..placements.len() {
        let p = placements.position(index);
        min = Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
        max = Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
    }
    let centre = (min + max) * 0.5;
    let radii = (max - min) * 0.5;
    let depth = |p: Vec3| {
        let d = p - centre;
        1.0 - Vec3::new(d.x / radii.x, d.y / radii.y, d.z / radii.z).length()
    };
    assert!(
        (0..placements.len())
            .filter(|&index| depth(placements.position(index)) > 0.2)
            .count()
            > 1000,
        "fixture needs interior leaves"
    );
    assert!(
        tree.wood
            .positions
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|p| {
                depth(Vec3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]))) > 0.2
            })
            .count()
            > 1000,
        "Bare needs wood inside the same crown ellipsoid"
    );
    let crown_camera = Camera {
        target: centre,
        position: centre
            + Vec3::new(1.0, 0.1, -1.0).normalized() * (2.0 * radii.x.max(radii.y).max(radii.z)),
        ..trunk()
    };
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit_at(&tree, Level::Chosen).unwrap();
    // Isolate the sky contribution; neither direct sun nor transmission can
    // mask the term under test. The ground hemisphere remains present.
    renderer.set_scene(SceneRow {
        sun_red: 0.0,
        sun_green: 0.0,
        sun_blue: 0.0,
        ..Default::default()
    });
    let off = MaterialParams::default();
    let on = MaterialParams {
        sky_occlusion_strength: 0.9,
        ..off
    };
    for view in [View::Whole, View::Bare, View::Leaf, View::Clay] {
        renderer.set_view(view);
        let camera = if matches!(view, View::Whole | View::Bare) {
            crown_camera
        } else {
            hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH)
        };
        renderer.set_material(off);
        let plain = render(&mut renderer, &camera, 512, 512).unwrap();
        renderer.set_material(if view == View::Clay {
            Preset::OregonWhiteOak.parameters().material
        } else {
            on
        });
        let dark = render(&mut renderer, &camera, 512, 512).unwrap();
        let full_plain = luminance(&plain);
        let full_dark = luminance(&dark);
        eprintln!("{view:?} full luminance: plain={full_plain}, dark={full_dark}");
        if matches!(view, View::Whole | View::Bare) {
            for leaves in [false, true] {
                if leaves && view == View::Bare {
                    continue;
                }
                let marker = if leaves {
                    MaterialParams {
                        leaf_front_red: 1.0,
                        leaf_front_green: 1.0,
                        leaf_front_blue: 1.0,
                        leaf_back_red: 1.0,
                        leaf_back_green: 1.0,
                        leaf_back_blue: 1.0,
                        ..off
                    }
                } else {
                    MaterialParams {
                        bark_red: 1.0,
                        bark_green: 1.0,
                        bark_blue: 1.0,
                        ..off
                    }
                };
                renderer.set_material(marker);
                let marker = render(&mut renderer, &camera, 512, 512).unwrap();
                let (pixels, plain_sum, dark_sum) = crown_sums(&plain, &dark, &marker);
                eprintln!(
                    "{view:?} leaves={leaves}: pixels={pixels}, plain={plain_sum}, dark={dark_sum}"
                );
                assert!(
                    pixels > 100,
                    "{view:?} leaves={leaves}: only {pixels} visible crown pixels"
                );
                // Require a measurable relative loss on the actual subject,
                // independent of image size and the surrounding sky's brightness.
                assert!(dark_sum < plain_sum * 0.99,
                    "{view:?} leaves={leaves}: pixels={pixels}, plain={plain_sum}, dark={dark_sum}; full plain={full_plain}, dark={full_dark}");
            }
        } else {
            assert_eq!(dark.rgba, plain.rgba, "{view:?}");
        }
    }
}
