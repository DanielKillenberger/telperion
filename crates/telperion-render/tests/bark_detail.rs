//! Bark changes shading, never the bytes that locate a silhouette.
mod common;
use telperion_core::{
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
    surface::SurfaceMesh,
};
use telperion_render::{render, Camera, Renderer, View, STILL_FORMAT};

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

#[test]
fn relief_moves_the_light_while_every_wood_mesh_byte_holds() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    // The cap bounds the fixture's work; enough annual growth must remain
    // for the trunk to carry the 0.16 m relief scale tested below.
    family.skeleton.growth.max_nodes = Some(40_000);
    let off = mesh::build(&family, Detail::Full).unwrap();
    family.material = MaterialParams {
        ridge_scale: 0.16,
        plate_scale: 0.12,
        roughness_detail: 0.3,
        ..family.material
    };
    let on = mesh::build(&family, Detail::Full).unwrap();
    println!(
        "R1 wood hash: off={}, on={}",
        hash(&off.wood),
        hash(&on.wood)
    );
    assert_eq!(hash(&off.wood), hash(&on.wood), "R1 wood mesh bytes");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&off).unwrap();
    renderer.set_view(View::Bare);
    let camera = Camera {
        position: Vec3::new(2.5, 2.0, 3.5),
        target: Vec3::new(0.0, 2.0, 0.0),
        field_of_view: 38.0,
        near: 0.01,
        far: 100.0,
    };
    let plain = render(&mut renderer, &camera, 160, 160).unwrap();
    renderer.set_material(family.material);
    let relief = render(&mut renderer, &camera, 160, 160).unwrap();
    let changed = plain
        .rgba
        .iter()
        .zip(&relief.rgba)
        .filter(|(a, b)| a != b)
        .count();
    assert!(changed > 100, "bark terms changed only {changed} channels");
}

#[test]
fn radius_storage_is_refused_before_upload_when_only_its_binding_limit_is_exceeded() {
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    let mut tree = mesh::build(&family, Detail::Full).unwrap();
    tree.foliage.instances.matrices.clear();
    let limit = (tree.wood.positions.len() / 3 * size_of::<f32>() - 4) as u64;
    let limits = wgpu::Limits {
        max_storage_buffer_binding_size: limit,
        ..Default::default()
    };
    let error = telperion_render::fits(&limits, &tree)
        .expect_err("radius storage passed the binding limit");
    assert!(error.to_string().contains("wood radii"), "{error}");
}
