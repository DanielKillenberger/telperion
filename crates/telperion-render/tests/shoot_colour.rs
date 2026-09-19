//! Young wood takes the row's shoot colour and gives it up to the bark's as it
//! thickens. A row with no shoot radius has no young wood and draws the frame
//! it always drew, whatever shoot colour it states.
mod common;
use telperion_core::{
    foliage::{Element, Instances},
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail, Foliage, TreeMesh},
    presets::Preset,
    surface::{Bounds, SurfaceMesh, SurfaceRun},
};
use telperion_render::{render, Camera, Renderer, Still, View, STILL_FORMAT};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 160;
/// The ramp: a horizontal cone along x at this height, its radius rising
/// linearly from the thin end to the thick one.
const LENGTH: f64 = 3.0;
const THIN: f64 = 0.04;
const THICK: f64 = 0.28;
const AXIS: f64 = 2.0;
const SHOOT: f64 = 0.1;

fn ramp() -> TreeMesh {
    let (rings, segments) = (151_usize, 24_usize);
    let mut wood = SurfaceMesh {
        positions: Vec::new(),
        normals: Vec::new(),
        coords: Vec::new(),
        indices: Vec::new(),
        bounds: None,
        runs: 1,
        run_table: Vec::new(),
        dropped: 0,
    };
    for ring in 0..rings {
        let along = LENGTH * ring as f64 / (rings - 1) as f64;
        let radius = THIN + (THICK - THIN) * along / LENGTH;
        for k in 0..segments {
            let angle = std::f64::consts::TAU * k as f64 / segments as f64;
            let (y, z) = (angle.cos(), angle.sin());
            let x = along - LENGTH / 2.0;
            wood.positions
                .extend([x, AXIS + radius * y, radius * z].map(|v| v as f32));
            wood.normals.extend([0.0, y as f32, z as f32]);
            wood.coords.extend([along as f32, angle as f32]);
        }
    }
    for ring in 0..rings - 1 {
        let (lower, upper) = ((ring * segments) as u32, ((ring + 1) * segments) as u32);
        for k in 0..segments as u32 {
            let next = (k + 1) % segments as u32;
            wood.indices.extend([
                lower + k,
                upper + k,
                lower + next,
                lower + next,
                upper + k,
                upper + next,
            ]);
        }
    }
    wood.run_table.push(SurfaceRun {
        first_index: 0,
        index_count: wood.indices.len() as u32,
        largest_radius: THICK,
    });
    TreeMesh {
        wood,
        foliage: Foliage {
            element: Element::default(),
            instances: Instances::default(),
        },
        bounds: Bounds {
            min: Vec3::new(-LENGTH / 2.0, AXIS - THICK, -THICK),
            max: Vec3::new(LENGTH / 2.0, AXIS + THICK, THICK),
        },
    }
}

fn side() -> Camera {
    Camera {
        position: Vec3::new(0.0, AXIS, 3.2),
        target: Vec3::new(0.0, AXIS, 0.0),
        field_of_view: 30.0,
        near: 0.01,
        far: 100.0,
    }
}

fn draw(renderer: &mut Renderer, camera: &Camera, row: MaterialParams) -> Still {
    renderer.set_material(row);
    render(renderer, camera, WIDTH, HEIGHT).unwrap()
}

/// A white bark and a dark red-brown shoot, far apart in every channel.
fn birchlike() -> MaterialParams {
    MaterialParams {
        bark_red: 0.8,
        bark_green: 0.8,
        bark_blue: 0.75,
        shoot_red: 0.1,
        shoot_green: 0.06,
        shoot_blue: 0.045,
        shoot_radius: SHOOT,
        ..Default::default()
    }
}

/// The picture's column at a world x on the cone's axis, and the radius the
/// wood there carries. Only columns clear of both ends are read.
fn columns() -> impl Iterator<Item = (usize, f64)> {
    let camera = side();
    let half = camera.position.z * (camera.field_of_view.to_radians() / 2.0).tan();
    let per_metre = f64::from(HEIGHT) / (2.0 * half);
    let span = (LENGTH / 2.0 - 0.1) * per_metre;
    let centre = f64::from(WIDTH) / 2.0;
    ((centre - span) as usize..(centre + span) as usize).map(move |column| {
        let x = (column as f64 + 0.5 - centre) / per_metre;
        (column, THIN + (THICK - THIN) * (x + LENGTH / 2.0) / LENGTH)
    })
}

fn pixel(still: &Still, column: usize) -> [i32; 3] {
    let at = ((HEIGHT as usize / 2) * WIDTH as usize + column) * 4;
    [0, 1, 2].map(|c| i32::from(still.rgba[at + c]))
}

#[test]
fn a_row_with_no_shoot_radius_draws_the_bark_whatever_shoot_colour_it_states() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(400);
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    let camera = telperion_render::hero_pose(tree.bounds, 1.0, telperion_render::GROUND_REACH);
    for view in [View::Whole, View::Bare] {
        renderer.set_view(view);
        let neutral = draw(&mut renderer, &camera, family.material);
        let stated = MaterialParams {
            shoot_red: 1.0,
            shoot_green: 0.0,
            shoot_blue: 1.0,
            ..family.material
        };
        let drawn = draw(&mut renderer, &camera, stated);
        assert!(neutral.rgba == drawn.rgba, "{view:?}: an inert row moved");
    }
}

#[test]
fn young_wood_takes_the_shoot_colour_and_gives_it_up_continuously_by_twice_its_radius() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&ramp()).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_figure(false);
    let camera = side();
    let row = birchlike();
    let blended = draw(&mut renderer, &camera, row);
    let bark = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            shoot_radius: 0.0,
            ..row
        },
    );
    let young = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            bark_red: row.shoot_red,
            bark_green: row.shoot_green,
            bark_blue: row.shoot_blue,
            shoot_radius: 0.0,
            ..row
        },
    );
    let (mut previous, mut crossed, mut read) = (None::<f64>, false, 0);
    for (column, radius) in columns() {
        let (b, n, y) = (
            pixel(&blended, column),
            pixel(&bark, column),
            pixel(&young, column),
        );
        let apart = (0..3).all(|c| (y[c] - n[c]).abs() > 1);
        if radius < SHOOT * 0.97 {
            assert!(
                (0..3).all(|c| (b[c] - y[c]).abs() <= 1),
                "r {radius}: {b:?} is not young {y:?}"
            );
        }
        if radius > 2.0 * SHOOT * 1.03 {
            assert!(
                (0..3).all(|c| (b[c] - n[c]).abs() <= 1),
                "r {radius}: {b:?} is not bark {n:?}"
            );
        }
        // How far toward the shoot colour this column is, on the channel with
        // the widest gap: 1 young, 0 bark, never a step between neighbours.
        let c = (0..3).max_by_key(|&c| (y[c] - n[c]).abs()).unwrap();
        if !apart {
            continue;
        }
        let youth = f64::from(b[c] - n[c]) / f64::from(y[c] - n[c]);
        crossed |= (0.3..0.7).contains(&youth);
        if let Some(before) = previous {
            assert!(
                youth <= before + 0.03,
                "r {radius}: the blend turned back ({before} to {youth})"
            );
            assert!(
                before - youth <= 0.06,
                "r {radius}: a step from {before} to {youth}"
            );
        }
        previous = Some(youth);
        read += 1;
    }
    assert!(read > 200, "only {read} columns were read");
    assert!(crossed, "no column is between the two colours");
}

#[test]
fn mottle_still_acts_on_young_wood() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&ramp()).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_figure(false);
    let camera = side();
    let plain = draw(&mut renderer, &camera, birchlike());
    let mottled = draw(
        &mut renderer,
        &camera,
        MaterialParams {
            bark_mottle_scale: 0.05,
            bark_mottle_strength: 0.8,
            ..birchlike()
        },
    );
    let moved = columns()
        .filter(|&(_, radius)| radius < SHOOT * 0.97)
        .filter(|&(column, _)| pixel(&plain, column) != pixel(&mottled, column))
        .count();
    assert!(moved > 10, "mottle moved only {moved} young columns");
}
