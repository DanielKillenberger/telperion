//! fn-253 R4: every drawn floret hangs from wood through its stalk, on
//! both executors. Each executor's organ words are decoded as the organ's
//! vertex stage reads them (`foliage.wgsl`'s `pedicel_vertex` and
//! `rachis_vertex`): a floret's pedicel seat is its base a pedicel back
//! along its axis, and must lie inside its own rachis's tube, laid from
//! the rachis instance by `bloom.rs`'s axis, which every rachis level runs
//! through ring by ring (`stalk.rs`); its own rachis is the first
//! marked instance after it in its run, as selection and the draw find it
//! (`unit.wgsl`); each rachis must stand on a shoot tip of the tree. With
//! the rachis at zero, the seat is the tip itself.
//! The trees carry non-unit floret sizes (taper, scatter) and, at a
//! stagger of one, florets the season has grown only part way or dropped.
use crate::generation::{io, Generator};
use crate::{
    select::{frame, Level, Select},
    Camera,
};
use crate::{Gpu, Renderer};
use std::collections::HashMap;
use telperion_core::foliage::{BloomParams, Leaf, Reference, WITHERED};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::tree::Tree;

/// `bloom.rs`'s axis segments.
const SEGMENTS: usize = 8;

/// One executor's florets against its rachises, as drawn.
#[derive(Debug, Default)]
struct Hung {
    florets: usize,
    rachises: usize,
    /// Florets drawn below their full size.
    partial: usize,
    /// The farthest a pedicel's seat stands outside its rachis's tube, and
    /// a rachis's seat from a shoot tip, in metres (zero or under: inside).
    outside: f64,
    seat: f64,
    /// Florets with no rachis within reach.
    loose: usize,
}

fn column(m: &[f32; 16], c: usize) -> Vec3 {
    Vec3::new(m[c * 4].into(), m[c * 4 + 1].into(), m[c * 4 + 2].into())
}

/// The rachis standing at `seat` along `heading`: its points, as the
/// vertex stage lays them.
fn axis(seat: Vec3, heading: Vec3, p: &BloomParams) -> [Vec3; SEGMENTS + 1] {
    let toward = if p.droop >= 0.0 { -Vec3::Y } else { Vec3::Y };
    let mut points = [seat; SEGMENTS + 1];
    for k in 0..SEGMENTS {
        let rest = 1.0 - (k as f64 + 0.5) / SEGMENTS as f64;
        let turned = p.droop.abs() * (1.0 - rest.powi(3));
        let t = heading.lerp(toward, turned);
        let t = if t.length_squared() > 1e-24 {
            t.normalized()
        } else {
            toward
        };
        points[k + 1] = points[k] + t * (p.rachis / SEGMENTS as f64);
    }
    points
}

/// How far `q` stands outside the tube through `points`, which narrows from
/// `radius` to `end`.
fn outside(q: Vec3, points: &[Vec3; SEGMENTS + 1], radius: f64, end: f64) -> f64 {
    (0..SEGMENTS)
        .map(|k| {
            let (a, b) = (points[k], points[k + 1]);
            let d = b - a;
            let t = ((q - a).dot(d) / d.length_squared().max(1e-30)).clamp(0.0, 1.0);
            let along = (k as f64 + t) / SEGMENTS as f64;
            q.distance(a + d * t) - (radius + (end - radius) * along)
        })
        .fold(f64::INFINITY, f64::min)
}

/// `words`, packed against `r`, hung from `tree`'s tips by `p`.
fn hung(words: &[Leaf], r: Reference, tree: &Tree, p: &BloomParams) -> Hung {
    let mut out = Hung::default();
    let cell = (p.rachis + p.pedicel).max(1e-3);
    let key = |v: Vec3| {
        let c = v * (1.0 / cell);
        (c.x.floor() as i64, c.y.floor() as i64, c.z.floor() as i64)
    };
    let near = |grid: &HashMap<_, Vec<usize>>, v: Vec3| -> Vec<usize> {
        let (x, y, z) = key(v);
        let mut found = Vec::new();
        for d in -1..=1 {
            for e in -1..=1 {
                for f in -1..=1 {
                    found.extend(grid.get(&(x + d, y + e, z + f)).into_iter().flatten());
                }
            }
        }
        found
    };
    // The tips that bear, by cell.
    let mut tips: HashMap<_, Vec<usize>> = HashMap::new();
    for b in &tree.blooms {
        let i = b.node as usize;
        tips.entry(key(tree.nodes[i].position)).or_default().push(i);
    }
    let end = 0.5 * p.stamen_width;
    // Each floret waits for the rachis that closes its run.
    let mut run: Vec<Vec3> = Vec::new();
    for w in words {
        let m = r.unpack(*w);
        if w[2] & WITHERED == 0 {
            let along = column(&m, 1);
            out.partial += usize::from(along.length() < 0.99);
            out.florets += 1;
            let seat = column(&m, 3) - along.normalized() * p.pedicel;
            if p.rachis > 0.0 {
                run.push(seat);
                continue;
            }
            let gap = (near(&tips, seat).iter())
                .map(|&i| tree.nodes[i].position.distance(seat) - tree.nodes[i].radius)
                .fold(f64::INFINITY, f64::min);
            match gap.is_finite() {
                true => out.outside = out.outside.max(gap),
                false => out.loose += 1,
            }
            continue;
        }
        let seat = column(&m, 3);
        let heading = column(&m, 1);
        let tip = (near(&tips, seat).iter())
            .map(|&i| tree.nodes[i].position.distance(seat))
            .fold(f64::INFINITY, f64::min);
        out.seat = out.seat.max(tip);
        out.rachises += 1;
        let points = axis(seat, heading.normalized(), p);
        for q in run.drain(..) {
            out.outside = out.outside.max(outside(q, &points, heading.length(), end));
        }
    }
    out.loose += run.len();
    out
}

/// Both executors' organ words for `family`, each with the box it packed
/// them against (`bloom_box` of its own tree's leaves): the device's whole
/// tree grown on the cycle (fn-256), then the CPU whole tree's, with the
/// CPU tree and the element.
struct Organs {
    device: (Vec<Leaf>, Reference),
    cpu: (Vec<Leaf>, Reference),
    tree: Tree,
    bloom: BloomParams,
    element: telperion_core::foliage::Element,
}

fn organs(gpu: &Gpu, family: &Family) -> Organs {
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = gen.prepare(family).unwrap();
    let organ = prepared.organ.as_ref().expect("the tree flowers");
    let reference = (prepared.mesh.organ.as_ref().unwrap().instances).reference;
    let device = pollster::block_on(io::read_leaves_async(
        gpu,
        organ.leaves.buffer(),
        organ.count,
    ));
    let device = device.unwrap();
    drop(prepared);
    let x = executor::grow(family).unwrap().expansion().unwrap();
    let (bloom, _, _) = x.bloom();
    let cpu = x.organ().unwrap().expect("the CPU tree flowers");
    let cpu_words = (cpu.instances.leaves, cpu.instances.reference);
    Organs {
        device: (device, reference),
        cpu: cpu_words,
        tree: x.tree().clone(),
        bloom,
        element: cpu.element,
    }
}

/// `family` built on the device and as the CPU whole tree: each one's
/// organ hung from the CPU tree's tips, and the coarser of the two boxes.
fn both(gpu: &Gpu, family: &Family) -> (Hung, Hung, Reference) {
    let o = organs(gpu, family);
    let p = &o.bloom;
    let ((dw, dr), (cw, cr)) = (&o.device, &o.cpu);
    let coarser = if dr.step().length() >= cr.step().length() {
        *dr
    } else {
        *cr
    };
    (
        hung(dw, *dr, &o.tree, p),
        hung(cw, *cr, &o.tree, p),
        coarser,
    )
}

/// Laurelin young, her florets opening one after another, at her own rows
/// and with no rachis.
#[test]
fn every_drawn_floret_hangs_from_wood_through_its_stalk() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for rachis in [None, Some(0.0)] {
        let mut family = telperion_core::presets::Preset::Laurelin.parameters();
        (family.age, family.skeleton.seed) = (30.0, 1);
        family.supernatural.bloom.stagger = Some(1.0);
        if let Some(r) = rachis {
            family.supernatural.bloom.rachis = Some(r);
        }
        let (device, cpu, r) = both(&gpu, &family);
        // Two codes of quantisation at each end of a stalk.
        let codes = 2.0 * r.step().length();
        println!("rachis {rachis:?}: device {device:?}, cpu {cpu:?}, codes {codes}");
        for (name, h) in [("device", &device), ("cpu", &cpu)] {
            assert!(h.florets > 1000 && h.partial > 100, "{name}: {h:?}");
            assert_eq!(h.loose, 0, "{name}: a floret in the air");
            assert!(
                h.outside <= codes,
                "{name}: a stalk off its seat by {}",
                h.outside
            );
            assert!(
                h.seat <= codes,
                "{name}: a rachis off its tip by {}",
                h.seat
            );
        }
        assert_eq!(
            (device.florets, device.rachises),
            (cpu.florets, cpu.rachises)
        );
        if rachis.is_some() {
            assert_eq!(cpu.rachises, 0, "no rachis drawn at zero");
        } else {
            assert!(cpu.rachises > 100);
        }
    }
}

/// What selection drew of one executor's organ from one camera.
#[derive(Debug, Default)]
struct Drawn {
    /// Inflorescences kept and dropped whole.
    kept: usize,
    dropped: usize,
    /// Florets drawn at a floret level, and at the card.
    florets: usize,
    carded: usize,
    /// Drawn florets whose rachis was not drawn.
    stranded: usize,
    /// The farthest a drawn floret's pedicel seat stands outside the
    /// rachis level its rachis was drawn at, in pixels, past two codes,
    /// once the unit is enlarged about its seat as the draw enlarges it.
    off: f64,
}

/// `words` as selection draws them from `camera` in a `size`d frame: each
/// drawn floret checked against its run's rachis at the level chosen for
/// it, its ribbon's chords or its tube (`stalk.rs`).
fn drawn(
    gpu: &Gpu,
    o: &Organs,
    (words, r): &(Vec<Leaf>, Reference),
    camera: &Camera,
    size: (u32, u32),
) -> Drawn {
    let r = *r;
    let element = crate::card::carded(&o.element).into_owned();
    let sphere = frame::sphere(&element);
    let stems = crate::card::rachis_levels(&element);
    let mut instances = telperion_core::foliage::Instances::new(r);
    instances.leaves = words.to_vec();
    let foliage = telperion_core::mesh::Foliage {
        element: element.clone(),
        instances,
    };
    let mut selection = Select::new(gpu);
    selection.submit(gpu, &foliage, &element, Level::Chosen);
    let p = &o.bloom;
    selection.set_stalk(p.rachis as f32, p.pedicel as f32, p.droop as f32);
    let mut encoder = gpu.encoder(&Default::default());
    selection.dispatch(gpu, &mut encoder, camera, size, true, None);
    gpu.submit([encoder.finish()]);
    let chosen = selection.chosen(gpu);
    let ppm = frame::pixels_per_metre(f64::from(size.1), camera.field_of_view);
    let forward = (camera.target - camera.position).normalized();
    let codes = 2.0 * r.step().length();
    let end = 0.5 * p.stamen_width;
    let mut out = Drawn::default();
    let mut run: Vec<usize> = Vec::new();
    for (i, w) in words.iter().enumerate() {
        if w[2] & WITHERED == 0 {
            run.push(i);
            continue;
        }
        let rachis = chosen[i];
        let drawn: Vec<usize> = run.drain(..).filter(|&f| chosen[f].is_some()).collect();
        match rachis {
            Some(_) => out.kept += 1,
            None if drawn.is_empty() => out.dropped += 1,
            None => out.stranded += drawn.len(),
        }
        let Some(level) = rachis else { continue };
        let m = r.unpack(*w);
        let (seat, heading) = (column(&m, 3), column(&m, 1));
        let full = axis(seat, heading.normalized(), p);
        // The chosen level's rings: every fourth, second, or every one.
        let stride = [4, 2, 1, 1][(level as usize).min(3)];
        let mut points = [seat; SEGMENTS + 1];
        for (k, q) in points.iter_mut().enumerate() {
            *q = full[(k / stride) * stride];
            if k % stride != 0 {
                let (a, b) = (
                    full[(k / stride) * stride],
                    full[((k / stride) + 1) * stride],
                );
                *q = a + (b - a) * ((k % stride) as f64 / stride as f64);
            }
        }
        let depth = (seat - camera.position).dot(forward).max(camera.near);
        // The unit's kept share and the enlargement the draw gives it about
        // its seat (`select.wgsl`, `foliage.wgsl`): every length from the
        // seat, a gap among them, grows by its inverse root.
        let reach = p.rachis + p.pedicel + f64::from(sphere[3]);
        let kept = (reach * ppm / depth).clamp(0.0, 1.0).powi(2);
        let grown = 1.0 / kept.max(1e-4).sqrt();
        for f in drawn {
            if chosen[f] == Some(stems as u32) {
                out.carded += 1;
                continue;
            }
            out.florets += 1;
            let fm = r.unpack(words[f]);
            let along = column(&fm, 1);
            let q = column(&fm, 3) - along.normalized() * p.pedicel;
            let gap = outside(q, &points, heading.length(), end) - codes;
            out.off = out.off.max(gap * grown * ppm / depth);
        }
    }
    out
}

/// fn-253 R4 (review): every floret selection draws hangs from its
/// rachis as drawn, on both executors, with thinning active: from the hero
/// camera and from six times as far in a small frame, where thinning keeps a share of the
/// inflorescences and drops the rest whole; each drawn floret's rachis is
/// drawn, and its pedicel's seat stands within half a pixel of the level
/// its rachis was given.
#[test]
fn every_drawn_floret_hangs_from_its_drawn_rachis() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut family = telperion_core::presets::Preset::Laurelin.parameters();
    (family.age, family.skeleton.seed) = (30.0, 1);
    family.supernatural.bloom.stagger = Some(1.0);
    let o = organs(&gpu, &family);
    let corner = |pick: fn(f64, f64) -> f64, from: f64| {
        let v = o.tree.nodes.iter().map(|n| n.position);
        v.fold(Vec3::new(from, from, from), |a, b| {
            Vec3::new(pick(a.x, b.x), pick(a.y, b.y), pick(a.z, b.z))
        })
    };
    let (lo, hi) = (corner(f64::min, f64::MAX), corner(f64::max, f64::MIN));
    let size = (960, 720);
    let bounds = telperion_core::surface::Bounds { min: lo, max: hi };
    let hero = crate::hero_pose(bounds, 960.0 / 720.0, crate::GROUND_REACH);
    let far = Camera {
        position: hero.target + (hero.position - hero.target) * 6.0,
        far: 10.0 * hero.far,
        ..hero.clone()
    };
    // Far, a 96-line frame puts a whole raceme under thinning's 2 px.
    let small = (96, 72);
    for (view, camera, size) in [("hero", &hero, size), ("far", &far, small)] {
        for (name, words) in [("device", &o.device), ("cpu", &o.cpu)] {
            let d = drawn(&gpu, &o, words, camera, size);
            println!("{view} {name}: {d:?}");
            assert_eq!(
                d.stranded, 0,
                "{view} {name}: a floret drawn without its rachis"
            );
            assert!(
                d.off <= 0.5,
                "{view} {name}: a pedicel {} px off its drawn rachis",
                d.off
            );
            assert!(d.kept > 0, "{view} {name}: {d:?}");
            // Thinned, a unit is under 2 px across, so each floret in it,
            // enlarged, is under 4 px and drawn as its card, which has no
            // pedicel (host): far, every drawn floret is a card; near,
            // pedicels are checked.
            match view {
                "far" => assert_eq!(d.florets, 0, "{view} {name}: {d:?}"),
                _ => assert!(d.florets > 1000, "{view} {name}: {d:?}"),
            }
        }
    }
    let d = drawn(&gpu, &o, &o.cpu, &far, small);
    assert!(d.dropped > 0, "thinning dropped no inflorescence: {d:?}");
}

/// fn-253 R1's probe, on the cycle: a shipped preset's florets hung from
/// its tips on both executors (`cargo test --profile ci -p
/// telperion-render preset_florets -- --ignored --nocapture`, under the
/// GPU lock, `PROBE=laurelin:30:1`). Reported, not held.
#[test]
#[ignore = "a probe: one shipped preset on the device"]
fn preset_florets() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("PROBE").unwrap_or_else(|_| "laurelin:30:1".into());
    let parts: Vec<&str> = only.split(':').collect();
    let mut family = telperion_core::presets::by_identity(parts[0]).unwrap();
    family.age = parts[1].parse().unwrap();
    family.skeleton.seed = parts[2].parse().unwrap();
    let (device, cpu, _) = both(&gpu, &family);
    println!("{only}: device {device:?}, cpu {cpu:?}");
}
