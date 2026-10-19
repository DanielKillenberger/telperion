//! fn-266 R2 (fn-256 DESIGN.md 12.8): the retained leaf bases the device
//! hangs against the CPU's, by lineage. A base is matched by its stem's
//! apex key and its ordinal down the stem (the CPU's bases carry no keys
//! of their own, `leaf_bases.rs`' `hung`): counts exact, its bearer by
//! key, its nodes, spiral and cell within the bounds below, measured.
//! The swept runs of the packed bases are matched point for point. Two
//! negative controls hold the comparison to its word: a base missing and
//! a cell moved, each refused.
use super::super::cycle::tree::{self, Kernels};
use super::super::{io, Generator};
use super::boots::{Hanger, AW};
use super::fold::CV;
use crate::Renderer;
use std::collections::HashMap;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::surface::{CurvePoint, CLUSTER_WORDS, POINT_WORDS, SECTION_FLOATS};

/// The farthest apart a base's points stand (m), its unit vectors (the
/// spiral's radial, the cell's axis and across) and its cell's corners
/// (radians round, metres along) and ring shares: about twice the worst
/// measured on the date palm at 12 and at its default age, 40 (points
/// 9.0e-5 m apart at 40, swept and hung; directions 2.2e-4; corners
/// 3.9e-7; shares 6e-8, one float32 step of a share near one).
const POSITION: f64 = 2e-4;
const DIRECTION: f64 = 5e-4;
const CORNER: f64 = 1e-6;
const SHARE: f64 = 2e-7;

/// One base, as either executor hangs it.
#[derive(Debug, Clone)]
struct Base {
    /// Its stem's apex key and its ordinal down the stem.
    key: (u32, u32),
    bearer: u32,
    from: [f64; 3],
    tip: [f64; 3],
    radii: [f64; 6],
    /// The packed base's cell, in `packed_sections`' layout.
    cell: Option<[f64; SECTION_FLOATS]>,
}

/// How far apart the matched bases stood.
#[derive(Debug, Default)]
struct Apart {
    bases: usize,
    position: f64,
    direction: f64,
    corner: f64,
    share: f64,
}

fn most(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f64::max)
}

/// The device's bases against the CPU's: the first way they disagree.
fn compare(device: &[Base], cpu: &[Base]) -> Result<Apart, String> {
    if device.len() != cpu.len() {
        return Err(format!(
            "count: {} bases against {}",
            device.len(),
            cpu.len()
        ));
    }
    let theirs: HashMap<(u32, u32), &Base> = cpu.iter().map(|b| (b.key, b)).collect();
    let mut a = Apart {
        bases: device.len(),
        ..Apart::default()
    };
    for d in device {
        let c = theirs
            .get(&d.key)
            .ok_or(format!("base {:?}: none on the CPU", d.key))?;
        if d.bearer != c.bearer {
            return Err(format!(
                "base {:?}: borne on {} against {}",
                d.key, d.bearer, c.bearer
            ));
        }
        a.position = a
            .position
            .max(most(&d.from, &c.from))
            .max(most(&d.tip, &c.tip));
        a.position = a.position.max(most(&d.radii, &c.radii));
        match (&d.cell, &c.cell) {
            (Some(x), Some(y)) => {
                // Origin; axis, radial, across; corners; flatness; rings.
                a.position = a.position.max(most(&x[0..3], &y[0..3]));
                a.direction = a.direction.max(most(&x[3..12], &y[3..12]));
                a.corner = a.corner.max(most(&x[12..16], &y[12..16]));
                a.share = a.share.max(most(&x[16..17], &y[16..17]));
                for r in 0..3 {
                    let at = 17 + 3 * r;
                    a.position = a.position.max(most(&x[at..at + 2], &y[at..at + 2]));
                    a.share = a.share.max(most(&x[at + 2..at + 3], &y[at + 2..at + 3]));
                }
            }
            (None, None) => {}
            _ => return Err(format!("base {:?}: packed on one executor alone", d.key)),
        }
    }
    let bounds = [
        ("position", a.position, POSITION),
        ("cell direction", a.direction, DIRECTION),
        ("cell corner", a.corner, CORNER),
        ("cell share", a.share, SHARE),
    ];
    for (what, apart, bound) in bounds {
        if !(apart <= bound) {
            return Err(format!("{what}: {apart:e} apart, over {bound:e}"));
        }
    }
    Ok(a)
}

fn family(id: &str, age: Option<u32>) -> Family {
    let mut f = telperion_core::presets::by_identity(id).unwrap();
    if let Some(age) = age {
        f.age = f64::from(age);
    }
    f
}

fn words(gpu: &crate::Gpu, b: &wgpu::Buffer, n: usize) -> Vec<u32> {
    let bytes = io::read(gpu, b, (n.max(4) * 4) as u64).unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

/// The bases the device hangs on `f`'s stems.
fn device(gpu: &crate::Gpu, k: &Kernels, hanger: &Hanger, f: &Family) -> Vec<Base> {
    let growth = executor::growth(f).unwrap();
    let rows = executor::bases(f)
        .unwrap()
        .expect("a family that hangs bases");
    let bases = super::boots::most(f).unwrap();
    let (mut dressed, _) = pollster::block_on(tree::drawn(gpu, k, (&growth, bases))).unwrap();
    let hung = pollster::block_on(hanger.hang(gpu, &mut dressed, Some(&rows))).unwrap();
    let seed = &dressed.waves[1];
    let all = words(gpu, &dressed.records, dressed.ids as usize * CV);
    let record = |id: u32| &all[id as usize * CV..][..CV];
    let key_of = |id: u32| record(id)[24];
    let stems = words(
        gpu,
        hung.stems.as_ref().unwrap(),
        2 + seed.phyts as usize * AW as usize,
    );
    // Each stem's slot: its apex, its first base and its bases (fn-277).
    let slot = |t: usize, f: usize| stems[2 + t * AW as usize + f] as usize;
    let stem_of = |b: usize| (0..stems[0] as usize).rfind(|&t| slot(t, 1) <= b).unwrap();
    let boots = dressed.waves.last().unwrap().base;
    let n = hung.bases as usize;
    let cells = hung
        .cells
        .as_ref()
        .map(|c| words(gpu, c, n * SECTION_FLOATS));
    let f = |w: u32| f64::from(f32::from_bits(w));
    (0..n)
        .map(|b| {
            let t = stem_of(b);
            let apex = slot(t, 0) as u32;
            let first = record(boots + 2 * b as u32);
            let second = record(boots + 2 * b as u32 + 1);
            let cell = cells.as_ref().map(|c| {
                let mut out = [0.0; SECTION_FLOATS];
                for (o, &w) in out
                    .iter_mut()
                    .zip(&c[b * SECTION_FLOATS..][..SECTION_FLOATS])
                {
                    *o = f(w);
                }
                out
            });
            Base {
                key: (record(seed.base + apex)[24], (b - slot(t, 1)) as u32),
                bearer: key_of(first[16]),
                from: [f(first[0]), f(first[1]), f(first[2])],
                tip: [f(second[0]), f(second[1]), f(second[2])],
                radii: [
                    first[3], first[4], first[5], second[3], second[4], second[5],
                ]
                .map(f),
                cell,
            }
        })
        .collect()
}

/// The bases the CPU hangs on `f`'s stems: the last nodes of its expanded
/// tree, two a base, in its clothed apices' order, and its last sections.
fn cpu(f: &Family) -> Vec<Base> {
    let p = executor::bases(f).unwrap().unwrap();
    let x = executor::grow(f).unwrap().expansion().unwrap();
    let tree = x.tree();
    // The stems clothed: longer than the crown's depth (`leaf_bases`).
    let clothed: Vec<usize> = (tree.stem_apices().into_iter())
        .filter(|&apex| {
            let (mut at, mut run) = (apex, 0.0);
            while let Some(q) = tree.nodes[at]
                .parent
                .map(|q| q as usize)
                .filter(|&q| tree.nodes[q].stem)
            {
                run += tree.nodes[at].position.distance(tree.nodes[q].position);
                at = q;
            }
            run - p.rosette_depth > 0.0
        })
        .collect();
    let per = p.leaf_bases as usize;
    let n = clothed.len() * per;
    let start = tree.nodes.len() - 2 * n;
    let packed = tree
        .sections
        .len()
        .checked_sub(n)
        .filter(|_| p.leaf_base_width > 0.0);
    (0..n)
        .map(|b| {
            let (first, second) = (&tree.nodes[start + 2 * b], &tree.nodes[start + 2 * b + 1]);
            let v = |v: telperion_core::math::Vec3| [v.x, v.y, v.z];
            let cell = packed.map(|at| {
                let s = &tree.sections[at + b];
                let mut out = [0.0; SECTION_FLOATS];
                for (k, w) in [s.origin, s.axis, s.radial, s.across].iter().enumerate() {
                    out[3 * k..3 * k + 3].copy_from_slice(&v(*w));
                }
                out[12..16].copy_from_slice(&[
                    s.corners[0][0],
                    s.corners[0][1],
                    s.corners[1][0],
                    s.corners[1][1],
                ]);
                out[16] = s.flatness;
                for (k, r) in s.rings.iter().enumerate() {
                    out[17 + 3 * k..20 + 3 * k].copy_from_slice(&[r.reach, r.rise, r.scale]);
                }
                out
            });
            Base {
                key: (tree.key(clothed[b / per]), (b % per) as u32),
                bearer: tree.key(first.parent.unwrap() as usize),
                from: v(first.position),
                tip: v(second.position),
                radii: [
                    first.radius,
                    first.start_radius,
                    first.base_radius,
                    second.radius,
                    second.start_radius,
                    second.base_radius,
                ],
                cell,
            }
        })
        .collect()
}

/// The swept runs of the packed bases on the device's curve against the
/// CPU curve's, matched by their first point: how many on each, and the
/// most apart a matched point's centre (m) and radius (m) stood.
fn swept(gpu: &crate::Gpu, f: &Family) -> ((usize, usize), f64) {
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let kit = gen.kit();
    let built =
        pollster::block_on(super::build::build(gpu, &gen, &kit.kernels, &kit.tools, f)).unwrap();
    let (points, clusters, p, k, _) = built.prepared.wood.as_ref().unwrap().blocks();
    let points = words(gpu, points, p as usize * POINT_WORDS);
    let clusters = words(gpu, clusters, k as usize * CLUSTER_WORDS);
    let point = |i: u32| {
        CurvePoint::unpack(
            points[i as usize * POINT_WORDS..][..POINT_WORDS]
                .try_into()
                .unwrap(),
        )
    };
    let mut mine: Vec<(u32, u32)> = Vec::new();
    for c in clusters.chunks(CLUSTER_WORDS).filter(|c| c[4] & 4 != 0) {
        if mine.last() != Some(&(c[2], c[3])) {
            mine.push((c[2], c[3]));
        }
    }
    let theirs = executor::grow(f)
        .unwrap()
        .expansion()
        .unwrap()
        .curve()
        .unwrap();
    let shaped: Vec<_> = theirs.runs.iter().filter(|r| r.section.is_some()).collect();
    let mut apart: f64 = 0.0;
    for &(first, count) in &mine {
        let at = point(first + 1).centre;
        let near = shaped.iter().min_by(|a, b| {
            let d = |r: &&&telperion_core::surface::CurveRun| {
                theirs.points[r.first as usize + 1].centre.distance(at)
            };
            d(a).total_cmp(&d(b))
        });
        let want = theirs.run_points(near.expect("a shaped run on the CPU"));
        assert_eq!(want.len(), count as usize, "a base's samples");
        for (i, c) in want.iter().enumerate() {
            let m = point(first + i as u32);
            apart = apart
                .max(m.centre.distance(c.centre))
                .max((m.radius - c.radius).abs());
        }
    }
    ((mine.len(), shaped.len()), apart)
}

/// The date palm at 12 and at its default age: every base the CPU hangs,
/// the device hangs, and draws on its cell where it packs.
#[test]
fn every_leaf_base_is_hung_as_the_cpu_hangs_it() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, hanger) = (Kernels::new(&gpu), Hanger::new(&gpu));
    for age in [Some(12), None] {
        let f = family("date-palm", age);
        let (mine, theirs) = (device(&gpu, &k, &hanger, &f), cpu(&f));
        let apart =
            compare(&mine, &theirs).unwrap_or_else(|e| panic!("date palm at {}: {e}", f.age));
        println!("date palm at {}: {apart:?}", f.age);
        assert!(apart.bases > 0, "no base hung");
        let (runs, swept) = swept(&gpu, &f);
        println!(
            "date palm at {}: shaped runs {runs:?}, apart {swept:e}",
            f.age
        );
        assert_eq!(runs.0, runs.1, "shaped runs");
        assert!(swept <= POSITION, "swept {swept:e}");
        // The controls: a base missing, and one cell's corner moved.
        let short = &mine[1..];
        let missing = compare(short, &theirs).expect_err("a missing base passed");
        assert!(missing.starts_with("count"), "{missing}");
        let mut moved = mine.clone();
        moved[0].cell.as_mut().expect("the palm's bases pack")[12] += 1e-2;
        let perturbed = compare(&moved, &theirs).expect_err("a moved cell passed");
        assert!(perturbed.starts_with("cell corner"), "{perturbed}");
    }
}
