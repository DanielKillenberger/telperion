//! fn-283 R1, R5: the frond axes the device hangs against the CPU's, by
//! lineage. An axis is matched by its place among its tree's axes, both
//! executors hanging them in the order the leaflets are placed (crowns in
//! turn, fronds newest first): counts exact, its bearer by key, its nodes'
//! points and radii within the bounds below, measured. A controlled
//! overflow of the room is refused by name.
use super::super::cycle::tree::{self, Kernels};
use super::super::io;
use super::boots::{frond_rows, most, Hanger};
use super::fold::CV;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;

/// The farthest apart an axis node stands (m) and its radii differ (m):
/// about twice the worst measured on the date palm at 4, 12 and 40 (a
/// point 1.15e-3 m apart at 40, a frond's heading 1.6e-4 apart in f32
/// carried to its 7 m rachis's tip; radii 1.9e-8).
const POSITION: f64 = 2.5e-3;
const RADIUS: f64 = 5e-8;
/// `boots.wgsl`'s flag of the clothing's records.
const BOOT: u32 = 0x4000_0000;

/// One frond's axis, as either executor hangs it: its bearer's key, and
/// each node's point and radii (its own, its start and its run's base).
#[derive(Debug, Clone)]
struct Axis {
    bearer: u32,
    nodes: Vec<[f64; 6]>,
}

fn family(age: Option<u32>) -> Family {
    let mut f = telperion_core::presets::by_identity("date-palm").unwrap();
    if let Some(age) = age {
        f.age = f64::from(age);
    }
    f
}

fn words(gpu: &crate::Gpu, b: &wgpu::Buffer, n: usize) -> Vec<u32> {
    let bytes = io::read(gpu, b, (n.max(4) * 4) as u64).unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

/// `f` drawn with room for `room` records a stem, and its bases and axes
/// hung: the bases hung, the drawn records and the clothing's wave, or
/// the refusal.
fn hung(
    gpu: &crate::Gpu,
    (k, hanger): (&Kernels, &Hanger),
    f: &Family,
    room: u32,
) -> crate::Result<(u32, Vec<u32>, (u32, u32))> {
    let growth = executor::growth(f).unwrap();
    let bases = executor::bases(f).unwrap();
    let crown = frond_rows(f, growth.year(1.0)).unwrap();
    let crown = crown.expect("the palm stands a crown");
    let (mut dressed, _) = pollster::block_on(tree::drawn(gpu, k, (&growth, room)))?;
    let rows = (bases.as_ref(), Some(&crown));
    let hung = pollster::block_on(hanger.hang(gpu, &mut dressed, rows))?;
    let wave = dressed.waves.last().unwrap();
    let all = words(gpu, &dressed.records, dressed.ids as usize * CV);
    Ok((hung.bases, all, (wave.base, wave.phyts)))
}

/// The axes the device hangs on `f`: the clothing's records past its
/// bases', run by run (their first record's id), with their bearers' keys.
fn device(gpu: &crate::Gpu, tools: (&Kernels, &Hanger), f: &Family) -> Vec<Axis> {
    let (bases, all, (base, phyts)) = hung(gpu, tools, f, most(f).unwrap()).unwrap();
    let record = |id: u32| &all[id as usize * CV..][..CV];
    let f32s = |r: &[u32], at: usize| f64::from(f32::from_bits(r[at]));
    let mut out: Vec<(u32, Axis)> = Vec::new();
    for id in base + 2 * bases..base + phyts {
        let r = record(id);
        if r[6] & BOOT == 0 {
            continue;
        }
        let node = [0, 1, 2, 3, 4, 5].map(|w| f32s(r, w));
        match out.last_mut() {
            Some((run, axis)) if *run == r[23] => axis.nodes.push(node),
            _ => {
                let bearer = record(r[16])[24];
                out.push((r[23], Axis { bearer, nodes: vec![node] }));
            }
        }
    }
    out.into_iter().map(|(_, a)| a).collect()
}

/// The axes the CPU hangs on `f`: its clothing's runs past its bases'
/// (two nodes each; an axis has three or more), with their bearers' keys.
fn cpu(f: &Family) -> Vec<Axis> {
    let x = executor::grow(f).unwrap().expansion().unwrap();
    let tree = x.tree();
    let first = tree.clothing.expect("the palm is clothed");
    let mut out: Vec<Axis> = Vec::new();
    for (i, n) in tree.nodes.iter().enumerate().skip(first) {
        let p = n.position;
        let node = [p.x, p.y, p.z, n.radius, n.start_radius, n.base_radius];
        match n.branch as usize == i {
            true => out.push(Axis {
                bearer: tree.key(n.parent.unwrap() as usize),
                nodes: vec![node],
            }),
            false => out.last_mut().unwrap().nodes.push(node),
        }
    }
    out.retain(|a| a.nodes.len() > 2);
    out
}

/// The device's axes against the CPU's: the most apart a point and a
/// radius stood, or the first way they disagree.
fn compare(device: &[Axis], cpu: &[Axis]) -> Result<(f64, f64), String> {
    if device.len() != cpu.len() {
        return Err(format!("count: {} axes against {}", device.len(), cpu.len()));
    }
    let (mut position, mut radius) = (0.0f64, 0.0f64);
    for (k, (d, c)) in device.iter().zip(cpu).enumerate() {
        if d.bearer != c.bearer {
            return Err(format!("axis {k}: borne on {} against {}", d.bearer, c.bearer));
        }
        if d.nodes.len() != c.nodes.len() {
            let (m, n) = (d.nodes.len(), c.nodes.len());
            return Err(format!("axis {k}: {m} nodes against {n}"));
        }
        for (a, b) in d.nodes.iter().zip(&c.nodes) {
            let apart = |w: std::ops::Range<usize>| w.map(|w| (a[w] - b[w]).abs()).fold(0.0, f64::max);
            position = position.max(apart(0..3));
            radius = radius.max(apart(3..6));
        }
    }
    match position <= POSITION && radius <= RADIUS {
        true => Ok((position, radius)),
        false => Err(format!("apart: point {position:e}, radius {radius:e}")),
    }
}

/// The date palm young, at 12 and at its default age, 40: every frond
/// axis the CPU hangs, the device hangs, on the same bearer, its nodes
/// within the bounds.
#[test]
fn every_frond_axis_is_hung_as_the_cpu_hangs_it() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, hanger) = (Kernels::new(&gpu), Hanger::new(&gpu));
    for age in [Some(4), Some(12), None] {
        let f = family(age);
        let (mine, theirs) = (device(&gpu, (&k, &hanger), &f), cpu(&f));
        let apart = compare(&mine, &theirs).unwrap_or_else(|e| panic!("palm at {}: {e}", f.age));
        let nodes: usize = mine.iter().map(|a| a.nodes.len()).sum();
        println!("palm at {}: {} axes, {nodes} nodes, apart {apart:?}", f.age, mine.len());
        assert!(!mine.is_empty(), "no axis hung");
        // The control: an axis missing.
        let missing = compare(&mine[1..], &theirs).expect_err("a missing axis passed");
        assert!(missing.starts_with("count"), "{missing}");
    }
}

/// A room that holds the palm's bases and not its axes is refused by the
/// axes' name, never truncated.
#[test]
fn axes_past_the_room_are_refused_by_name() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, hanger) = (Kernels::new(&gpu), Hanger::new(&gpu));
    let f = family(None);
    let bases = f.canopy.leaf_bases.ceil() as u32;
    let refused = hung(&gpu, (&k, &hanger), &f, 2 * bases).err().expect("the axes fit");
    assert!(refused.to_string().contains("rachis nodes"), "{refused}");
}

/// The frond curve evaluated case by case: a case's point and length, its
/// heading and arch, its lift and girth, and its share.
const CURVE: &str = "
@group(0) @binding(0) var<uniform> cfg: vec4<u32>;
@group(0) @binding(1) var<storage, read> cases: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> out: array<vec4<f32>>;
@compute @workgroup_size(1)
fn curve(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    let a = cases[i * 4u];
    let b = cases[i * 4u + 1u];
    let c = cases[i * 4u + 2u];
    let r = rachis(a.xyz, b.xyz, c.xyz, a.w, b.w, c.w, cases[i * 4u + 3u].x);
    out[i * 2u] = vec4<f32>(r.at, 0.0);
    out[i * 2u + 1u] = vec4<f32>(r.run, 0.0);
}
";

/// fn-301 R2 (host, 2026-10-21): a frond meeting the ground straight down
/// rests along its frame's lift, +lift with no arch and the arch's way
/// with one, as far along the ground as its parabola runs past contact:
/// the cases and expectations the CPU's `frond_contact` suite holds its
/// curve to, here within f32.
#[test]
fn a_frond_straight_down_rests_on_the_device_as_on_the_cpu() {
    use telperion_core::math::Vec3;
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    const FINEST: f64 = 2e-4;
    let (down, lift) = (Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    // Base height, arch, the way it lies along the lift, contact, and the
    // parabola's length from contact to its tip.
    let arc = |a: f64, t: f64| {
        let x = 2.0 * a * t;
        match a == 0.0 {
            true => 5.0 * t,
            false => 5.0 / (4.0 * a) * (x * (1.0 + x * x).sqrt() + x.asinh()),
        }
    };
    let cases = [
        (2.0, 0.0, 1.0, (2.0 - FINEST) / 5.0),
        (1e-4, -0.5, -1.0, 0.0),
        (1e-4, 0.5, 1.0, 0.0),
    ];
    let words: Vec<f32> = (cases.iter())
        .flat_map(|&(base, arch, _, _)| {
            [
                [0.0, base, 0.0, 5.0],
                [down.x, down.y, down.z, arch],
                [lift.x, lift.y, lift.z, 0.0],
                [1.0, 0.0, 0.0, 0.0],
            ]
        })
        .flatten()
        .map(|v| v as f32)
        .collect();
    let n = cases.len() as u32;
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let cfg = io::buffer(&gpu, "cfg", 16, wgpu::BufferUsages::UNIFORM, &[0; 16]).unwrap();
    let input = io::buffer(
        &gpu,
        "cases",
        4 * words.len() as u64,
        usage,
        bytemuck::cast_slice(&words),
    );
    let output = io::buffer(&gpu, "curve", 32 * u64::from(n), usage, &[]).unwrap();
    let input = input.unwrap();
    let source = format!("{}\n{CURVE}", super::super::sources::rachis());
    let pass = io::Pass::new(&gpu, source, &[true, false], &["curve"]);
    let bind = pass.bind(&gpu, &[&cfg, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, n);
    gpu.submit([e.finish()]);
    let read = io::read(&gpu, &output, 32 * u64::from(n)).unwrap();
    let read: &[f32] = bytemuck::cast_slice(&read);
    for (k, &(base, arch, way, c)) in cases.iter().enumerate() {
        let v = |i: usize| Vec3::new(read[i].into(), read[i + 1].into(), read[i + 2].into());
        let (at, run) = (v(k * 8), v(k * 8 + 4));
        let toward = lift * way;
        let lies = (at - Vec3::new(0.0, base, 0.0)).dot(toward);
        let expected = arc(arch, 1.0) - arc(arch, c);
        let case = format!("base {base}, arch {arch}");
        assert!(run.distance(toward) < 1e-6, "{case}: runs {run:?}");
        assert!((at.y - FINEST).abs() < 1e-9, "{case}: at {}", at.y);
        assert!(
            (lies - expected).abs() < 1e-5,
            "{case}: lies {lies} against {expected}"
        );
    }
}
