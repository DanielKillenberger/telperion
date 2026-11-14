//! fn-283 R7: the film's oak-to-palm walk at 40, seed 7 (fn-272 cut 4)
//! showed leaflets flying where a frond had no wood. With the rachis
//! drawn, every frond leaflet and spine base lies on drawn wood at every
//! walk share, the halo the film saw (t = 0.067 to 0.562) at least every
//! 0.05. Each frame is built as the film builds it (`blend::families`,
//! then the pipeline), and each base is measured against the drawn curve
//! (each ring's sphere, and between the rings of one run its tapered
//! capsule).
//!
//! The frond's leaves are told from the station-borne ones by a second
//! build at `cauline` 0, which grows the same wood and keeps the rosette's
//! leaves to the bit and places nothing on the twigs. The station-borne
//! compound leaves are a second design (host, 2026-10-19); their numbers
//! are printed for the owner's verdict, never held.
use std::collections::HashMap;
use telperion_core::{
    blend, foliage::Leaf, math::Vec3, mesh, presets::Preset, surface::Curve, Family,
};

/// What "on the wood" admits: a millimetre off the drawn curve.
const ON: f64 = 1e-3;
/// The walk's shares: its ends, the halo at least every 0.05 with the
/// film's measured edges and worst frame, and the palm's side.
const SHARES: [f64; 20] = [
    0.0, 0.025, 0.05, 0.067, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.5125, 0.55, 0.562,
    0.6, 0.75, 0.9, 1.0,
];
/// The halo's edges: every frame inside draws fronds.
const HALO: (f64, f64) = (0.067, 0.562);
/// The side of a cell of the wood's grid, and the most cells a search
/// reaches out: past it a base is reported as standing off by more.
const CELL: f64 = 0.5;
const REACH: i64 = 8;

fn end(preset: Preset) -> Family {
    let mut f = preset.parameters();
    (f.age, f.skeleton.seed) = (40.0, 7);
    f
}

type Cell = (i64, i64, i64);

fn cell(p: Vec3) -> Cell {
    let at = |v: f64| (v / CELL).floor() as i64;
    (at(p.x), at(p.y), at(p.z))
}

/// The drawn wood as pieces of one run, each a tapered capsule from a
/// ring to the next (a ring alone where a run has one), filed in every
/// cell its box meets.
struct Wood {
    pieces: Vec<[(Vec3, f64); 2]>,
    cells: HashMap<Cell, Vec<u32>>,
}

impl Wood {
    fn of(curve: &Curve) -> Wood {
        let mut wood = Wood {
            pieces: Vec::new(),
            cells: HashMap::new(),
        };
        for run in &curve.runs {
            let points = &curve.points[run.first as usize..(run.first + run.count) as usize];
            let ring = |i: usize| (points[i].centre, points[i].radius);
            for i in 0..points.len().saturating_sub(1).max(1) {
                wood.file([ring(i), ring((i + 1).min(points.len() - 1))]);
            }
        }
        wood
    }

    fn file(&mut self, piece: [(Vec3, f64); 2]) {
        let [(a, ra), (b, rb)] = piece;
        let r = Vec3::new(1., 1., 1.) * ra.max(rb);
        let least = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let most = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        let (lo, hi) = (cell(least - r), cell(most + r));
        let index = self.pieces.len() as u32;
        self.pieces.push(piece);
        for x in lo.0..=hi.0 {
            for y in lo.1..=hi.1 {
                for z in lo.2..=hi.2 {
                    self.cells.entry((x, y, z)).or_default().push(index);
                }
            }
        }
    }

    /// How far `p` stands off the drawn wood, zero inside; infinite past
    /// the search's reach.
    fn off(&self, p: Vec3) -> f64 {
        let (x, y, z) = cell(p);
        let mut least = f64::INFINITY;
        for k in 0..=REACH {
            for dx in -k..=k {
                for dy in -k..=k {
                    for dz in -k..=k {
                        if dx.abs().max(dy.abs()).max(dz.abs()) != k {
                            continue;
                        }
                        let filed = self.cells.get(&(x + dx, y + dy, z + dz));
                        for &i in filed.into_iter().flatten() {
                            least = least.min(off_piece(self.pieces[i as usize], p));
                        }
                    }
                }
            }
            // A piece not yet seen lies wholly outside these shells.
            if least <= k as f64 * CELL {
                return least.max(0.);
            }
        }
        least.max(0.)
    }
}

/// How far `p` stands off one tapered capsule: the nearest point of its
/// axis and the radius interpolated there, and either end's sphere.
fn off_piece([(a, ra), (b, rb)]: [(Vec3, f64); 2], p: Vec3) -> f64 {
    let run = b - a;
    let t = match run.length_squared() {
        l if l > 0. => ((p - a).dot(run) / l).clamp(0., 1.),
        _ => 0.,
    };
    let axis = p.distance(a + run * t) - (ra + (rb - ra) * t);
    axis.min(p.distance(a) - ra).min(p.distance(b) - rb)
}

/// One frame's leaves split by source and measured against its wood.
struct Frame {
    fronds: Vec<f64>,
    stations: Vec<f64>,
}

/// The frame at `t`: the film's build (`mesh::build`), and the same
/// tree at `cauline` 0, whose wood is the frame's and whose leaves are the
/// fronds' alone.
fn frame(a: &Family, b: &Family, t: f64) -> Frame {
    let f = blend::families(a, b, t).unwrap();
    let mesh = mesh::build(&f).unwrap();
    let mut g = f.clone();
    g.canopy.cauline = 0.0;
    let rosette = mesh::build(&g).unwrap();
    assert!(rosette.curve == mesh.curve, "the same wood at {t}");
    let mut left: HashMap<Leaf, usize> = HashMap::new();
    for &leaf in &rosette.foliage.instances.leaves {
        *left.entry(leaf).or_default() += 1;
    }
    let wood = Wood::of(&mesh.curve);
    let mut out = Frame {
        fronds: Vec::new(),
        stations: Vec::new(),
    };
    let instances = &mesh.foliage.instances;
    for (i, leaf) in instances.leaves.iter().enumerate() {
        let off = wood.off(instances.position(i));
        match left.get_mut(leaf).filter(|n| **n > 0) {
            Some(n) => {
                *n -= 1;
                out.fronds.push(off);
            }
            None => out.stations.push(off),
        }
    }
    assert!(
        left.values().all(|&n| n == 0),
        "a frond leaf at cauline 0 is missing from the frame at {t}"
    );
    out
}

/// The most, the median and the count past half a metre, sorted in place.
fn summary(v: &mut [f64]) -> String {
    v.sort_by(f64::total_cmp);
    let far = v.iter().filter(|&&d| d > 0.5).count();
    match (v.last(), v.get(v.len() / 2)) {
        (Some(most), Some(median)) => {
            format!("{}\t{most:.2e}\t{median:.2e}\t{far}", v.len())
        }
        _ => "0\t-\t-\t0".into(),
    }
}

#[test]
fn every_frond_leaflet_on_the_walk_lies_on_drawn_wood() {
    let (a, b) = (end(Preset::OregonWhiteOak), end(Preset::DatePalm));
    println!("t\tfrond leaves\tmax m\tmedian m\t>0.5 m\tstation leaves\tmax m\tmedian m\t>0.5 m");
    let mut off = Vec::new();
    for t in SHARES {
        let Frame {
            mut fronds,
            mut stations,
        } = frame(&a, &b, t);
        if (HALO.0..=HALO.1).contains(&t) {
            assert!(!fronds.is_empty(), "no frond drawn at {t}");
        }
        let far = fronds.iter().filter(|&&d| d >= ON);
        off.extend(far.map(|d| format!("{t}: {d:.3e} m")));
        println!("{t}\t{}\t{}", summary(&mut fronds), summary(&mut stations));
    }
    assert!(off.is_empty(), "frond leaflets off the wood: {off:#?}");
}
