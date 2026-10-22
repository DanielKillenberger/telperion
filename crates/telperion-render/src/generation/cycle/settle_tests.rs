//! The device's settle held to the CPU's (`grow_capture_settled_at`):
//! young trees of every shipped preset grown on the device from the seed
//! and settled there, to the ruled standard (`ruled.rs`, DESIGN.md
//! section 9).
use super::build::{Build, Kit, Tables};
use super::columns::Cap;
use super::compare::Agreement;
use super::grown::Snapshot;
use super::grown_compare::by_lineage;
use super::grown_tests::{built, device, grown, Grown, YOUNG};
use super::settle_device::Kernels;
use super::settle_read::{beside_agree, cpu_settled, Beside};
use super::settle_run::{settle_now as settle, Settled};
use crate::Gpu;
use telperion_space::SettleTo;

/// Each PA's settle constants (`grow_capture_settle_tables`).
pub(super) fn constants(g: &Grown) -> Vec<u32> {
    Cap(&telperion_space::grow_capture_settle_tables(&g.species)).u32s("settle_pa")
}

/// The device's tree grown and settled as far as `to`, read back.
pub(super) fn settled(
    gpu: &Gpu,
    (kit, k): (&Kit, &Kernels),
    g: &Grown,
    to: SettleTo,
) -> (Build, Settled, Snapshot, Beside) {
    let (b, _) = built(gpu, kit, g, g.request.age);
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let s = settle(gpu, (kit, k), (&b, &t), (&constants(g), to)).expect("a settle");
    let (snap, beside) = s.snapshot(gpu, &b.state, to).expect("a readback");
    (b, s, snap, beside)
}

/// The young trees of every preset settled as far as `to` on the device,
/// held to the CPU's to the ruled standard.
fn young_trees_settle_as_the_cpu(to: SettleTo) {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let mut apart = Vec::new();
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, to)
            .expect("the CPU's tree");
        let cpu = cpu_settled(&Cap(&capture), to);
        let (_, s, dev, beside) = settled(&gpu, (&kit, &k), &g, to);
        let found = held((&dev, &beside), (&cpu.0, &cpu.1));
        let (t, depths) = (&s.table, s.kept_starts.len() - 1);
        let joins = beside.joined.chunks(5).filter(|j| j[0] == 1).count();
        println!(
            "{name} {age} to {to:?}: {} kept axes in {depths} depths, {} units, {} phytomers, {joins} joins; flags {}; {} dispatches",
            t.kept, t.units, t.phyts, s.flags, s.stamped.len()
        );
        apart.extend(found.into_iter().map(|f| format!("{name}: {f}")));
    }
    assert!(
        apart.is_empty(),
        "the device's settled trees are apart from the CPU's: {apart:#?}"
    );
}

/// What breaks the ruled standard between the device's settle and the
/// CPU's: the tables by lineage, and the sides beside them.
pub(super) fn held(dev: (&Snapshot, &Beside), cpu: (&Snapshot, &Beside)) -> Vec<String> {
    let by = by_lineage(dev.0, cpu.0);
    println!("{}", by.report());
    let mut found = super::ruled::findings(&by);
    let mut a = Agreement::default();
    beside_agree(&mut a, dev.1, cpu.1);
    found.extend(
        a.fields
            .iter()
            .filter(|f| f.2 > 0)
            .map(|f| format!("{}: {}", f.0, f.3)),
    );
    found
}

#[test]
fn young_trees_shed_and_order_on_the_device_as_on_the_cpu() {
    young_trees_settle_as_the_cpu(SettleTo::Shed);
}

#[test]
fn young_trees_scale_and_thicken_on_the_device_as_on_the_cpu() {
    young_trees_settle_as_the_cpu(SettleTo::Thick);
}

#[test]
#[ignore = "a probe"]
fn young_trees_are_placed_on_the_device_as_on_the_cpu() {
    young_trees_settle_as_the_cpu(SettleTo::Placed);
}

#[test]
fn young_trees_are_laid_on_the_device_as_on_the_cpu() {
    young_trees_settle_as_the_cpu(SettleTo::Laid);
}

/// A tree that grows no phytomer settles to the CPU's `Collapsed`: the
/// date palm with every zone's nodes none.
#[test]
fn a_tree_without_phytomers_collapses_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let mut g = grown(telperion_core::presets::Preset::DatePalm, 3, 42);
    for state in &mut g.species.states {
        for zone in &mut state.zones {
            zone.nodes = telperion_space::NodeLaw::Uniform { min: 0.0, max: 0.0 };
        }
    }
    g.tables = telperion_space::grow_capture_tables(&g.species, g.request).expect("tables");
    g.chain = telperion_space::ChainTables::cycle(&g.species, g.request.age).expect("chain tables");
    let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Shed)
        .expect("the CPU's tree");
    let cpu = cpu_settled(&Cap(&capture), SettleTo::Shed);
    let (_, s, dev, beside) = settled(&gpu, (&kit, &k), &g, SettleTo::Shed);
    assert_eq!(s.error[0], 1, "the device's tree collapsed");
    let found = held((&dev, &beside), (&cpu.0, &cpu.1));
    assert!(
        found.is_empty(),
        "the device's collapse is the CPU's: {found:?}"
    );
}

/// fn-301 R2: a step straight down at the ground rests along its
/// phytomer's side on the device as on the CPU: the walk's own `rested`
/// (`lay_walk.wgsl`), run alone, gives `lay::rested`'s words for the
/// CPU's case (`a_step_straight_down_rests_along_its_side`).
#[test]
fn a_step_straight_down_rests_along_its_side_on_the_device() {
    use super::super::io;
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let from = super::LAY_WALK
        .find("fn rested(")
        .expect("the walk's rested");
    let to = from + super::LAY_WALK[from..].find("\n}\n").expect("its end") + 3;
    let probe = "
@group(0) @binding(0) var<uniform> unused: vec4<u32>;
@group(0) @binding(1) var<storage, read_write> out: array<f32>;
@compute @workgroup_size(1)
fn probe_main() {
    let p = rested(vec3<f32>(0.25, 0.5, 1.0), vec3<f32>(0.0, 0.0, -1.0), vec3<f32>(0.0, -1.0, 0.0), 0.25, 1.25);
    out[0] = p.x;
    out[1] = p.y;
    out[2] = p.z;
}
";
    let shader = format!(
        "{}\n{}\n{}\n{}\n{probe}",
        super::ARITH,
        super::ROOT,
        super::Q,
        &super::LAY_WALK[from..to]
    );
    let pass = io::Pass::new(&gpu, shader, &[false], &["probe_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let out = io::buffer(&gpu, "rested out", 12, usage, &[]).unwrap();
    let unused = io::buffer(
        &gpu,
        "rested cfg",
        16,
        wgpu::BufferUsages::UNIFORM,
        &[0; 16],
    )
    .unwrap();
    let bind = pass.bind(&gpu, &[&unused, &out]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, 1);
    gpu.submit([e.finish()]);
    let raw = io::read(&gpu, &out, 12).unwrap();
    assert_eq!(bytemuck::cast_slice::<u8, f32>(&raw), [0.25, -0.5, 0.25]);
}

/// fn-303: the device's `span_share` reads a length at a span's summed end
/// as the whole span, as `stand::span_share` does, on the CPU test's
/// scales (`geometry/tests.rs`' `SPANS`), whose division alone reads the
/// tip 1 − δ short; the tip summed on the device by `summed`'s fold.
#[test]
fn a_length_at_a_span_end_is_the_whole_span_on_the_device() {
    use super::super::io;
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let from = super::LAY_STAND
        .find("fn span_share(")
        .expect("the stand's span_share");
    let to = from + super::LAY_STAND[from..].find("\n}\n").expect("its end") + 3;
    let spans: [f32; 3] = [0.3, 0.3, 0.044714];
    let word = |v: f32| format!("bitcast<f32>({}u)", v.to_bits());
    let (a, b, c) = (word(spans[0]), word(spans[1]), word(spans[2]));
    let probe = format!(
        "
@group(0) @binding(0) var<uniform> unused: vec4<u32>;
@group(0) @binding(1) var<storage, read_write> out: array<f32>;
@compute @workgroup_size(1)
fn probe_main() {{
    // Zero read at run time, so the sums are the device's, not the compiler's.
    let zero = bitcast<f32>(unused.x);
    let before = zero + {a} + {b};
    let tip = -zero + {a} + {b} + {c};
    let short = bitcast<f32>(bitcast<u32>(tip) - 1u);
    out[0] = tip;
    out[1] = span_share(tip, before, {c});
    out[2] = span_share(before, {a}, {b});
    out[3] = span_share(short, before, {c});
}}
"
    );
    let shader = format!(
        "{}\n{}\n{}\n{}\n{probe}",
        super::ARITH,
        super::ROOT,
        super::Q,
        &super::LAY_STAND[from..to]
    );
    let pass = io::Pass::new(&gpu, shader, &[false], &["probe_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let out = io::buffer(&gpu, "span share out", 16, usage, &[]).unwrap();
    let unused = io::buffer(
        &gpu,
        "span share cfg",
        16,
        wgpu::BufferUsages::UNIFORM,
        &[0; 16],
    )
    .unwrap();
    let bind = pass.bind(&gpu, &[&unused, &out]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, 1);
    gpu.submit([e.finish()]);
    let raw = io::read(&gpu, &out, 16).unwrap();
    let got = bytemuck::cast_slice::<u8, f32>(&raw);
    // The CPU's own sums and share, as `stand` takes them.
    let before = 0.0f32 + spans[0] + spans[1];
    let tip: f32 = spans.iter().sum();
    let short = f32::from_bits(tip.to_bits() - 1);
    assert!(
        (tip - before) / spans[2] < 1.0,
        "the spans must round below 1"
    );
    assert_eq!(got[0].to_bits(), tip.to_bits(), "the device's fold");
    assert_eq!((got[1], got[2]), (1.0, 1.0), "a span's end is its whole");
    let cpu = ((short - before) / spans[2]).clamp(0.0, 1.0);
    assert!(cpu < 1.0);
    assert_eq!(got[3].to_bits(), cpu.to_bits(), "a length short of the end");
}
