//! fn-309 R3 (owner, 2026-10-22, on the film's cut 5: "i thought i still
//! saw bare/flying leaves in the palm"): every leaf of an oak-to-palm walk
//! frame classed by the drawn element it stands on. Each frame is built as
//! the film builds it (`blend::families`, then the pipeline), and each leaf
//! base is measured against the drawn curve (each ring's sphere, and
//! between the rings of one run its tapered capsule):
//!
//! - (a) on wood at least a pixel wide, (b) on wood under a pixel;
//! - (c) on a drawn rachis, the wood a second build at `rachisWidth` 0
//!   does not draw, reported as at least and under a pixel;
//! - (d) a station- or cluster-borne leaf, told from the fronds' by a
//!   third build at `cauline` 0, standing more than `SEAT` off drawn wood;
//! - (e) any leaf more than `SEAT` off every drawn element.
//!
//! A pixel is the film's at the stem of frame 526 (26.8 mm at 28.03 m,
//! the host's diagnosis), scaled by `FLY_DISTANCE`; the pixel split is
//! reported, never held. (d) and (e) are held at zero on every frame.
//!
//! `cargo test --profile ci -p telperion-core --test walk_flying --
//! --ignored --nocapture --test-threads 1`. `FLY_FRAMES` replaces the
//! frames: `from,to,age,seed,t` each, separated by `;`.
use std::collections::{HashMap, HashSet};
use telperion_core::{
    blend, foliage::Leaf, math::Vec3, mesh, presets::by_identity, surface::Curve, Family,
};

/// What "on" an element admits, as the diagnosis took it.
const SEAT: f64 = 0.1;
/// The film's camera at frame 526: its distance and a pixel at the stem.
const DISTANCE: f64 = 28.03;
const PIXEL: f64 = 26.8e-3;
/// The side of a cell of the wood's grid, and the most cells a search
/// reaches out: past it a base is reported as standing off by more.
const CELL: f64 = 0.5;
const REACH: i64 = 8;

/// A walk frame: its ends, their age and seed, and its share.
type Walk = (String, String, f64, u32, f64);

/// The film's frames 506, 516, 526 and 536 (seed 7), and fn-308's
/// oak-to-palm grid (seeds 1, 7 and 42), all at 40.
fn frames() -> Vec<Walk> {
    let walk = |seed, t| ("oregon-white-oak".into(), "date-palm".into(), 40.0, seed, t);
    let Ok(raw) = std::env::var("FLY_FRAMES") else {
        let film = [0.129, 0.227, 0.34, 0.463].map(|t| walk(7, t));
        let grid = [1, 7, 42].map(|s| [0.25, 0.5, 0.75].map(|t| walk(s, t)));
        return film.into_iter().chain(grid.into_iter().flatten()).collect();
    };
    let frame = |f: &str| -> Walk {
        let v: Vec<&str> = f.split(',').map(str::trim).collect();
        let number = |i: usize| v[i].parse::<f64>().expect("a number");
        (
            v[0].into(),
            v[1].into(),
            number(2),
            number(3) as u32,
            number(4),
        )
    };
    raw.split(';')
        .filter(|f| !f.trim().is_empty())
        .map(frame)
        .collect()
}

fn end(id: &str, age: f64, seed: u32) -> Family {
    let mut f = by_identity(id).expect("a shipped preset");
    (f.age, f.skeleton.seed) = (age, seed);
    f
}

type Cell = (i64, i64, i64);

fn cell(p: Vec3) -> Cell {
    let at = |v: f64| (v / CELL).floor() as i64;
    (at(p.x), at(p.y), at(p.z))
}

/// Drawn pieces, each a tapered capsule from a ring to the next of one run
/// (a ring alone where a run has one), filed in every cell its box meets.
#[derive(Default)]
struct Wood {
    pieces: Vec<[(Vec3, f64); 2]>,
    cells: HashMap<Cell, Vec<u32>>,
}

impl Wood {
    /// The curve's pieces split by `rachis`, which says whether a ring is
    /// a rachis's: a piece with such a ring is the rachis's.
    fn split(curve: &Curve, rachis: impl Fn(Vec3) -> bool) -> (Wood, Wood) {
        let (mut wood, mut axes) = (Wood::default(), Wood::default());
        for run in &curve.runs {
            let points = &curve.points[run.first as usize..(run.first + run.count) as usize];
            let ring = |i: usize| (points[i].centre, points[i].radius);
            for i in 0..points.len().saturating_sub(1).max(1) {
                let piece = [ring(i), ring((i + 1).min(points.len() - 1))];
                match rachis(piece[0].0) || rachis(piece[1].0) {
                    true => axes.file(piece),
                    false => wood.file(piece),
                }
            }
        }
        (wood, axes)
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

    /// How far `p` stands off these pieces, zero inside, and the radius
    /// where it comes nearest; infinite past the search's reach.
    fn off(&self, p: Vec3) -> (f64, f64) {
        let (x, y, z) = cell(p);
        let mut least = (f64::INFINITY, 0.);
        for k in 0..=REACH {
            for dx in -k..=k {
                for dy in -k..=k {
                    for dz in -k..=k {
                        if dx.abs().max(dy.abs()).max(dz.abs()) != k {
                            continue;
                        }
                        let filed = self.cells.get(&(x + dx, y + dy, z + dz));
                        for &i in filed.into_iter().flatten() {
                            let near = off_piece(self.pieces[i as usize], p);
                            if near.0 < least.0 {
                                least = near;
                            }
                        }
                    }
                }
            }
            // A piece not yet seen lies wholly outside these shells.
            if least.0 <= k as f64 * CELL {
                break;
            }
        }
        (least.0.max(0.), least.1)
    }
}

/// How far `p` stands off one tapered capsule and its radius there: the
/// nearest point of its axis with the radius interpolated there, or
/// either end's sphere where it is nearer.
fn off_piece([(a, ra), (b, rb)]: [(Vec3, f64); 2], p: Vec3) -> (f64, f64) {
    let run = b - a;
    let t = match run.length_squared() {
        l if l > 0. => ((p - a).dot(run) / l).clamp(0., 1.),
        _ => 0.,
    };
    let r = ra + (rb - ra) * t;
    [
        (p.distance(a + run * t) - r, r),
        (p.distance(a) - ra, ra),
        (p.distance(b) - rb, rb),
    ]
    .into_iter()
    .min_by(|x, y| x.0.total_cmp(&y.0))
    .expect("three candidates")
}

/// One frame's counts: (a), (b), (c) at least and under a pixel, (d), (e),
/// of how many leaves, and the farthest a (d) or (e) leaf stands off.
#[derive(Default)]
struct Classes {
    a: usize,
    b: usize,
    c: [usize; 2],
    d: usize,
    e: usize,
    leaves: usize,
    farthest: f64,
}

fn bits(p: Vec3) -> [u64; 3] {
    [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]
}

/// The frame `(from, to, age, seed, t)` classed with a pixel of `pixel`
/// metres: the film's build, the same tree at `rachisWidth` 0 for the
/// wood without its rachises, and at `cauline` 0 for the fronds' leaves.
fn classes((from, to, age, seed, t): &Walk, pixel: f64) -> Classes {
    let (a, b) = (end(from, *age, *seed), end(to, *age, *seed));
    let f = blend::families(&a, &b, *t).expect("the frame blends");
    let built = mesh::build(&f).expect("the frame builds");
    let variant = |edit: fn(&mut Family)| {
        let mut g = f.clone();
        edit(&mut g);
        mesh::build(&g).expect("the variant builds")
    };
    let bare = variant(|g| g.canopy.rachis_width = 0.0);
    let rosette = variant(|g| g.canopy.cauline = 0.0);
    assert!(rosette.curve == built.curve, "the same wood at {t}");
    let woody: HashSet<_> = bare.curve.points.iter().map(|p| bits(p.centre)).collect();
    let (wood, axes) = Wood::split(&built.curve, |c| !woody.contains(&bits(c)));
    let mut fronds: HashMap<Leaf, usize> = HashMap::new();
    for &leaf in &rosette.foliage.instances.leaves {
        *fronds.entry(leaf).or_default() += 1;
    }
    let instances = &built.foliage.instances;
    let mut out = Classes {
        leaves: instances.len(),
        ..Classes::default()
    };
    for (i, leaf) in instances.leaves.iter().enumerate() {
        let p = instances.position(i);
        let (w, r) = (wood.off(p), axes.off(p));
        let frond = match fronds.get_mut(leaf).filter(|n| **n > 0) {
            Some(n) => {
                *n -= 1;
                true
            }
            None => false,
        };
        if !frond && w.0 > SEAT {
            out.d += 1;
            out.farthest = out.farthest.max(w.0.min(r.0));
        }
        match (w.0.min(r.0) > SEAT, r.0 < w.0) {
            (true, _) => {
                out.e += 1;
                out.farthest = out.farthest.max(w.0.min(r.0));
            }
            (false, true) => out.c[usize::from(2. * r.1 < pixel)] += 1,
            (false, false) if 2. * w.1 >= pixel => out.a += 1,
            (false, false) => out.b += 1,
        }
    }
    assert!(
        fronds.values().all(|&n| n == 0),
        "a frond leaf at cauline 0 is missing from the frame at {t}"
    );
    out
}

#[test]
#[ignore = "a measurement: fn-309 R3's classes on heavy walk frames"]
fn no_leaf_on_the_walk_stands_off_drawn_structure() {
    let distance = std::env::var("FLY_DISTANCE")
        .ok()
        .and_then(|d| d.parse::<f64>().ok())
        .unwrap_or(DISTANCE);
    let pixel = PIXEL * distance / DISTANCE;
    println!(
        "seat {SEAT} m, a pixel {:.1} mm at {distance} m",
        pixel * 1e3
    );
    println!("from\tto\tage\tseed\tt\tleaves\ta\tb\tc>=1px\tc<1px\td\te\tfarthest m");
    let mut flying = Vec::new();
    for walk in frames() {
        let c = classes(&walk, pixel);
        let (from, to, age, seed, t) = &walk;
        println!(
            "{from}\t{to}\t{age}\t{seed}\t{t}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.2}",
            c.leaves, c.a, c.b, c.c[0], c.c[1], c.d, c.e, c.farthest
        );
        if c.d + c.e > 0 {
            flying.push(format!(
                "{from}>{to} {age} seed {seed} t {t}: d {} e {}",
                c.d, c.e
            ));
        }
    }
    assert!(flying.is_empty(), "leaves off drawn structure: {flying:#?}");
}
