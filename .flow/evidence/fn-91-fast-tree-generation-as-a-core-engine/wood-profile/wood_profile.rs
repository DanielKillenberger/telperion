//! Scratch-only diagnostic: no foliage expansion and no renderer/device.
use serde_json::json;
use std::{hint::black_box, time::Instant};
use telperion_core::{branching, presets::Preset, surface};
#[path = "../../telperion-render/src/wood/radius.rs"]
mod radius;

fn digest(mesh: &surface::SurfaceMesh) -> String {
    let mut hash = 14695981039346656037_u64;
    let mut put = |bytes: &[u8]| {
        for &byte in bytes { hash = (hash ^ byte as u64).wrapping_mul(1099511628211); }
    };
    // Length-prefix every array; fixed little-endian IEEE float bits, u32 indices,
    // u64 counts; bounds presence byte then six f64s; ordered run records.
    for values in [&mesh.positions, &mesh.normals, &mesh.coords] {
        put(&(values.len() as u64).to_le_bytes());
        for v in values { put(&v.to_le_bytes()); }
    }
    put(&(mesh.indices.len() as u64).to_le_bytes());
    for v in &mesh.indices { put(&v.to_le_bytes()); }
    put(&[u8::from(mesh.bounds.is_some())]);
    if let Some(b) = mesh.bounds {
        for v in [b.min.x,b.min.y,b.min.z,b.max.x,b.max.y,b.max.z] { put(&v.to_le_bytes()); }
    }
    for v in [mesh.runs, mesh.dropped, mesh.run_table.len()] { put(&(v as u64).to_le_bytes()); }
    for r in &mesh.run_table {
        put(&r.first_index.to_le_bytes()); put(&r.index_count.to_le_bytes()); put(&r.largest_radius.to_le_bytes());
    }
    format!("{hash:016x}")
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = args.get(1).ok_or("preset required")?;
    let seed: u32 = args.get(2).ok_or("seed required")?.parse()?;
    let mut f = Preset::from_id(preset).ok_or("unknown preset")?.parameters();
    f.skeleton.seed = seed;
    let start = Instant::now();
    let report = branching::generate(&f.skeleton, f.radii)?;
    assert!(report.tree.diagnostics.complete());
    println!("{}", json!({"event":"fixture","preset":preset,"seed":seed,"nodes":report.tree.nodes.len(),"growth_ms":start.elapsed().as_secs_f64()*1000.,"parameters":telperion_core::params::metadata(&f)}));
    for sample in 0..4 {
        let start = Instant::now();
        let wood = surface::build(&report.tree, f.skeleton.envelope.height, &f.surface)?;
        let wood_ms = start.elapsed().as_secs_f64()*1000.;
        let stages: Vec<f64> = Vec::new(); // replaced only in instrumented scratch
        let start = Instant::now();
        let radii = black_box(radius::radii(black_box(&wood)));
        let radius_ms = start.elapsed().as_secs_f64()*1000.;
        let hash = digest(&wood);
        println!("{}", json!({"event":"sample","preset":preset,"seed":seed,"sample":sample,"wood_ms":wood_ms,"radius_ms":radius_ms,"stages_ms":stages,"all_surface_fnv1a64":hash,"vertices":wood.positions.len()/3,"triangles":wood.indices.len()/3,"runs":wood.runs,"dropped":wood.dropped,"radius_bytes":radii.len()*4,"surface_capacity_bytes":(wood.positions.capacity()+wood.normals.capacity()+wood.coords.capacity())*4+wood.indices.capacity()*4+wood.run_table.capacity()*std::mem::size_of::<surface::SurfaceRun>()}));
        black_box(radii);
        black_box(wood);
    }
    Ok(())
}
