//! Depth beyond the shaded normal. The row is off by default, and with it off
//! the wood the core hands up and the frame the device draws are what they
//! were; with it on, the picture changes by a measured amount.
mod common;
use telperion_core::{
    material::MaterialParams, math::Vec3, mesh, presets::Preset, surface::SurfaceMesh,
};
use telperion_render::{render, Camera, Renderer, Still, View, STILL_FORMAT};

const SIZE: (u32, u32) = (640, 400);

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

fn difference(a: &Still, b: &Still) -> (f64, u32) {
    let mut sum = 0.0;
    let mut worst = 0;
    for (x, y) in a.rgba.iter().zip(&b.rgba) {
        let d = x.abs_diff(*y);
        sum += f64::from(d);
        worst = worst.max(d);
    }
    (sum / a.rgba.len() as f64, u32::from(worst))
}

#[test]
fn the_depth_row_moves_no_wood_and_changes_the_picture() {
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let off = MaterialParams {
        depth_strength: 0.0,
        ..family.material
    };
    assert!(
        family.material.depth_strength > 0.0,
        "the oak's own row is what is being measured"
    );
    assert_eq!(
        MaterialParams::default().depth_strength,
        0.0,
        "the row must arrive off, so an older document renders as it did"
    );
    // Depth is a shading term and moves no vertex: the core builds one wood,
    // and the row cannot reach it. The fn-24 identity pins hash this mesh.
    let row = family.material;
    let lit = {
        let mut without = family.clone();
        without.material = off;
        mesh::build(&without).unwrap()
    };
    let tree = mesh::build(&family).unwrap();
    assert_eq!(
        hash(&tree.wood),
        hash(&lit.wood),
        "the depth row moved the wood the core hands up"
    );
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    renderer.set_material(off);
    let plain = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    renderer.set_material(row);
    let deep = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    let again = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    assert_eq!(deep.rgba, again.rgba, "the identical draw changed");
    renderer.set_material(off);
    let back = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    assert_eq!(
        plain.rgba, back.rgba,
        "the row at zero did not return the frame it started from"
    );
    let (mean, worst) = difference(&plain, &deep);
    eprintln!(
        "depth row at {}: mean {mean:.4}/255, worst {worst}/255 over the trunk frame",
        row.depth_strength
    );
    assert!(
        mean > 0.2,
        "the depth row changed almost nothing: mean {mean}/255"
    );
}
