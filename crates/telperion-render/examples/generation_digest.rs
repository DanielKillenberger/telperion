//! Digests of what every shipped preset builds, one JSON line a preset and
//! seed: the pipeline's artifacts for a full request and for the field alone,
//! the mesh, the GPU executor's output at both deliveries with its metrics
//! less their timings, and the growth path's mesh at a young age. Two runs of
//! the same generator print the same lines; a refactor that keeps the bytes
//! keeps every digest. `cargo run --release -p telperion-render --example
//! generation_digest [preset ...]`; `DIGEST_SEEDS` overrides seeds 1 and 7.
use serde_json::{json, Value};
use std::fmt::{Debug, Write};
use telperion_core::{
    field::{FieldSnapshot, IndexSnapshot},
    mesh,
    pipeline::{self, Request},
    presets::Preset,
    specimen::SpecimenView,
    Family,
};
use telperion_render::{
    generation::{Delivery, Generator, Metrics, Prepared},
    Gpu, Renderer, STILL_FORMAT,
};

const PRESETS: [&str; 8] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "date-palm",
    "telperion",
    "laurelin",
];

/// FNV-1a over the value's `Debug` text, which prints every float exactly.
struct Fnv(u64);
impl Write for Fnv {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for b in s.bytes() {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
        Ok(())
    }
}
fn digest(v: &impl Debug) -> String {
    let mut h = Fnv(0xcbf2_9ce4_8422_2325);
    write!(h, "{v:?}").expect("hashing never fails");
    format!("{:016x}", h.0)
}
fn index(i: &IndexSnapshot) -> (&Vec<f64>, &Vec<u32>, u32) {
    (&i.bounds, &i.topology, i.node_count)
}
fn snapshot(s: &FieldSnapshot) -> String {
    digest(&(
        &s.wood,
        index(&s.wood_index),
        index(&s.leaves),
        &s.plan,
        &s.plan_stations,
        index(&s.plan_index),
        &s.plan_sides,
    ))
}
fn full(f: &Family) -> Result<Value, Box<dyn std::error::Error>> {
    let request = Request {
        wood: true,
        leaves: true,
        field: Some(None),
        structure: true,
        ..Request::default()
    };
    let built = pipeline::build(f, request)?;
    let o = &built.outputs;
    let structure = o.structure.as_ref().map(|s| (&s.nodes, &s.topology));
    Ok(json!({
        "tree": digest(&built.skeleton.tree),
        "shed": built.skeleton.shed,
        "element": digest(&o.element),
        "plan": digest(&o.plan),
        "wood": digest(&o.wood),
        "leaves": digest(&o.leaves),
        "field": o.field.as_ref().map(|f| f.snapshot().map(|s| snapshot(&s))).transpose()?,
        "structure": digest(&structure),
    }))
}
fn field_only(f: &Family) -> Result<Value, Box<dyn std::error::Error>> {
    let request = Request {
        field: Some(None),
        ..Request::default()
    };
    let o = pipeline::build(f, request)?.outputs;
    let field = o.field.as_ref().map(|f| f.snapshot().map(|s| snapshot(&s)));
    Ok(json!({"field": field.transpose()?, "leaves": digest(&o.leaves), "plan": digest(&o.plan)}))
}
/// The metrics a run reports, less their timings.
fn metrics(m: &Metrics) -> String {
    digest(&(
        (m.position_cpu_bytes, m.position_gpu_peak_bytes),
        (m.position_retained_metadata_bytes, m.position_fallback),
        (m.gpu_positions, m.wood_prepared_cpu_bytes),
        (m.wood_metadata_cpu_bytes, m.wood_gpu_peak_bytes),
        (m.wood_backend, m.wood_fallback, m.base_cpu_bytes),
        (m.wood_cpu_bytes, m.retained_gpu_bytes, m.input_instances),
        (
            m.instances,
            m.descriptor_cpu_bytes,
            m.shared_contact_cpu_bytes,
        ),
        (m.shared_prepare_cpu_bytes, m.shared_metadata_cpu_bytes),
        m.gpu_compute_peak_bytes,
    ))
}
fn prepared(g: &Generator, p: &Prepared) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(json!({
        "backend": format!("{:?}", p.backend),
        "mesh": p.cpu_mesh().map(digest),
        "instances": digest(&g.read_instances(p)?),
        "count": p.count(),
        "wood": [p.wood_vertices(), p.wood_triangles()],
        "bounds": digest(&p.bounds()),
        "metrics": metrics(&p.metrics),
    }))
}
fn growth(f: &Family) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = f.clone();
    f.age = 6.0;
    Ok(digest(&SpecimenView::build(&f)?.mesh()?))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let named: Vec<String> = std::env::args().skip(1).collect();
    let presets: Vec<&str> = if named.is_empty() {
        PRESETS.to_vec()
    } else {
        named.iter().map(String::as_str).collect()
    };
    let seeds: Vec<u32> = std::env::var("DIGEST_SEEDS")
        .unwrap_or_else(|_| "1,7".into())
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let renderer = Renderer::new(pollster::block_on(Gpu::request(None))?, STILL_FORMAT);
    let resident = Generator::new(&renderer)?;
    let owned = Generator::for_cpu_output(pollster::block_on(Gpu::request(None))?)?;
    for id in presets {
        for &seed in &seeds {
            let mut f = Preset::from_id(id).ok_or("unknown preset")?.parameters();
            f.skeleton.seed = seed;
            let line = json!({
                "preset": id,
                "seed": seed,
                "pipeline": full(&f)?,
                "fieldOnly": field_only(&f)?,
                "mesh": digest(&mesh::build(&f)?),
                "gpuCpu": prepared(&owned, &owned.prepare(&f, Delivery::Cpu)?)?,
                "gpuResident": prepared(&resident, &resident.prepare(&f, Delivery::Resident)?)?,
                "growth": growth(&f)?,
            });
            println!("{line}");
        }
    }
    Ok(())
}
