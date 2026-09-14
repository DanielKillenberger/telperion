//! Colour by structure: a weathered face against a fresh furrow, a side
//! turned from the sun, and a plate that keeps its own value and cast. Each
//! one alone changes the picture; all of them at zero leave it exactly as it
//! was before the rows existed, and none of them moves a vertex.
mod common;
use telperion_core::{
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
    surface::SurfaceMesh,
};
use telperion_render::{render, Camera, Renderer, Still, View, STILL_FORMAT};

const SIZE: (u32, u32) = (384, 320);

fn trunk() -> Camera {
    Camera {
        position: Vec3::new(1.7307636095778745, 2.2967023330704928, -1.7307636095778745),
        target: Vec3::new(0.0, 2.0, 0.0),
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    }
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

fn changed(a: &Still, b: &Still) -> f64 {
    a.rgba
        .iter()
        .zip(&b.rgba)
        .map(|(x, y)| f64::from(x.abs_diff(*y)))
        .sum::<f64>()
        / a.rgba.len() as f64
}

/// The oak's row with every addition this spec makes put back to nothing.
fn before() -> MaterialParams {
    MaterialParams {
        plate_cell_scale: 0.0,
        plate_elongation: 0.0,
        plate_dome: 0.0,
        plate_edge_lift: 0.0,
        plate_identity: 0.0,
        weathering_strength: 0.0,
        weathering_red: 0.0,
        weathering_green: 0.0,
        weathering_blue: 0.0,
        orientation_strength: 0.0,
        orientation_red: 0.0,
        orientation_green: 0.0,
        orientation_blue: 0.0,
        directional_occlusion: 0.0,
        depth_strength: 0.0,
        ..Preset::OregonWhiteOak.parameters().material
    }
}

#[test]
fn each_structure_colour_row_changes_the_picture_and_moves_no_wood() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let plain = before();
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_material(plain);
    let bare = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    // A weathered face and a fresh furrow need a structure to tell apart, so
    // each row below is measured over the oak's own plate network.
    let network = MaterialParams {
        plate_cell_scale: family.material.plate_cell_scale,
        plate_elongation: family.material.plate_elongation,
        plate_dome: family.material.plate_dome,
        plate_edge_lift: family.material.plate_edge_lift,
        ..plain
    };
    renderer.set_material(network);
    let without = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    for (name, row) in [
        (
            "weathering",
            MaterialParams {
                weathering_strength: 0.9,
                weathering_red: 0.12,
                weathering_green: 0.115,
                weathering_blue: 0.10,
                ..network
            },
        ),
        (
            "orientation",
            MaterialParams {
                orientation_strength: 0.9,
                orientation_red: -0.04,
                orientation_green: 0.06,
                orientation_blue: -0.03,
                ..network
            },
        ),
        (
            "plate identity",
            MaterialParams {
                plate_identity: 0.8,
                ..network
            },
        ),
    ] {
        let mut with = family.clone();
        with.material = row;
        assert_eq!(
            hash(&tree.wood),
            hash(&mesh::build(&with, Detail::Full).unwrap().wood),
            "{name} moved the wood the core hands up"
        );
        renderer.set_material(row);
        let on = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
        let moved = changed(&without, &on);
        eprintln!("{name}: mean {moved:.4}/255 over the trunk frame");
        assert!(moved > 0.2, "{name} changed almost nothing: {moved}/255");
    }
    // Everything this spec added, back to nothing, is the frame it started
    // from - byte for byte, not within a tolerance.
    renderer.set_material(plain);
    let again = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    assert_eq!(
        bare.rgba, again.rgba,
        "the additions at zero did not return the frame they started from"
    );
    // And the oak's shipped row is not that frame: the spec did something.
    renderer.set_material(family.material);
    let shipped = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    let moved = changed(&bare, &shipped);
    eprintln!("the oak's shipped row against the same row before fn-32: mean {moved:.4}/255");
    assert!(moved > 1.0, "the shipped oak barely moved: {moved}/255");
}
