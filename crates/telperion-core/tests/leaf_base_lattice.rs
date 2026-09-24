//! Retained leaf bases packed into the lattice of flat-faced boots a date
//! palm's trunk shows. fn-144.
//!
//! Measured on the built mesh: every base's bark ring and outer end are read
//! back out of the wood buffers, unrolled off a straight trunk, and set
//! against every other base. At full width and a flat section the bases tile
//! the bark: each covers its own cell, none crowds into another, and each
//! meets its neighbours edge to edge.
use telperion_core::{
    bias::BiasParams,
    foliage,
    math::Vec3,
    mesh::{self, Detail},
    presets::{Family, Preset},
    surface::SurfaceMesh,
};

/// The share of a cell's own size a gap or an overlap may reach before the
/// lattice stops reading as packed.
const TOLERANCE: f64 = 0.05;

fn palm(bases: u32) -> Family {
    let mut f = Preset::from_id("date-palm").unwrap().parameters();
    f.skeleton.seed = 1;
    // A straight trunk, so one axis unrolls every base.
    f.skeleton.habit.crookedness = 0.;
    f.skeleton.habit.attractor_weight = 0.;
    f.skeleton.bias = BiasParams::NONE;
    f.canopy.leaf_bases = bases;
    f.canopy.leaf_base_width = 1.;
    f.canopy.leaf_base_flatness = 1.;
    f.canopy.leaf_base_weathering = 0.;
    f
}

/// One ring unrolled off the trunk: its vertices as (radians round, metres
/// up), the angle and height of its centre, and its distance from the axis.
struct Ring {
    points: Vec<(f64, f64)>,
    angle: f64,
    height: f64,
    reach: f64,
}

fn wrap(a: f64) -> f64 {
    let t = std::f64::consts::TAU;
    (a + std::f64::consts::PI).rem_euclid(t) - std::f64::consts::PI
}

/// The bark ring and the outer end of every base the wood carries: every run
/// of two or three rings, read off its span of the index buffer. A base that
/// leaves the axis at a stem node has no ring inside the trunk.
fn bases(
    wood: &SurfaceMesh,
    segments: usize,
    unroll: impl Fn(Vec3) -> (f64, f64, f64),
) -> Vec<[Ring; 2]> {
    let vertex = |i: usize| {
        let p = &wood.positions[i * 3..i * 3 + 3];
        Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
    };
    let mut out = Vec::new();
    for run in &wood.run_table {
        let span = &wood.indices[run.first_index as usize..][..run.index_count as usize];
        let low = *span.iter().min().unwrap() as usize;
        let high = *span.iter().max().unwrap() as usize;
        let rings = (high + 1 - low - 2) / segments;
        if !(2..=3).contains(&rings) {
            continue;
        }
        let ring = |k: usize| {
            let raw: Vec<_> = (0..segments)
                .map(|j| unroll(vertex(low + k * segments + j)))
                .collect();
            let (s, c) = raw
                .iter()
                .fold((0., 0.), |(s, c), p| (s + p.0.sin(), c + p.0.cos()));
            let angle = s.atan2(c);
            // Unwrapped round the ring, so a cell wider than half the trunk
            // stays one outline.
            let mut round = wrap(raw[0].0 - angle);
            let mut points = Vec::with_capacity(segments);
            for (j, p) in raw.iter().enumerate() {
                if j > 0 {
                    round += wrap(p.0 - raw[j - 1].0);
                }
                points.push((round, p.1));
            }
            Ring {
                points,
                angle,
                height: raw.iter().map(|p| p.1).sum::<f64>() / segments as f64,
                reach: raw.iter().map(|p| p.2).sum::<f64>() / segments as f64,
            }
        };
        out.push([ring(rings - 2), ring(rings - 1)]);
    }
    out
}

/// A ring laid flat at a radius, shifted round by an angle.
fn laid(ring: &Ring, radius: f64, turn: f64) -> Vec<(f64, f64)> {
    ring.points
        .iter()
        .map(|&(a, h)| ((a + turn) * radius, h))
        .collect()
}

fn area(poly: &[(f64, f64)]) -> f64 {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        .abs()
        / 2.
}

/// The signed distance between two convex outlines along the axis that
/// parts them best: positive a gap, negative the depth one crowds the other.
fn apart(p: &[(f64, f64)], q: &[(f64, f64)]) -> f64 {
    let mut best = f64::NEG_INFINITY;
    for poly in [p, q] {
        for i in 0..poly.len() {
            let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
            let (x, y) = (b.1 - a.1, a.0 - b.0);
            let len = (x * x + y * y).sqrt();
            if len < 1e-9 {
                continue;
            }
            let along = |pt: &(f64, f64)| (pt.0 * x + pt.1 * y) / len;
            let span = |s: &[(f64, f64)]| {
                s.iter()
                    .map(along)
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), v| {
                        (l.min(v), h.max(v))
                    })
            };
            let (pl, ph) = span(p);
            let (ql, qh) = span(q);
            best = best.max((ql - ph).max(pl - qh));
        }
    }
    best
}

/// R1 at one count and width: the widest gap to an edge neighbour and the
/// deepest crowding, each a share of the cell's size, the thinnest cover of a
/// cell, and how many rings were read in the middle rows.
fn lattice(count: u32, width: f64) -> (f64, f64, f64, usize) {
    let mut f = palm(count);
    f.canopy.leaf_base_width = width;
    let tree = mesh::grow(&f).unwrap();
    let wood = mesh::build(&f, Detail::Full).unwrap().wood;
    let rosette = &foliage::rosettes(&tree)[0];
    let axis = rosette.axis;
    let (normal, binormal) = foliage::frame(axis);
    let origin = tree.nodes[0].position;
    let unroll = |v: Vec3| {
        let r = v - origin;
        let h = r.dot(axis);
        let q = r - axis * h;
        (q.dot(binormal).atan2(q.dot(normal)), h, q.length())
    };
    let segments = f.surface.radial_segments.max(f.surface.lobes * 4) as usize;
    let found = bases(&wood, segments, unroll);
    assert_eq!(
        found.len(),
        count as usize,
        "every base is a run of its own"
    );
    let (mut gap, mut crowd, mut cover, mut checked) = (0f64, 0f64, f64::INFINITY, 0);
    for level in 0..2 {
        let rings: Vec<&Ring> = found.iter().map(|b| &b[level]).collect();
        let (low, high) = rings
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), r| {
                (l.min(r.height), h.max(r.height))
            });
        let pitch = (high - low) / f64::from(count - 1);
        for (i, own) in rings.iter().enumerate() {
            let radius = own.reach;
            let cell = std::f64::consts::TAU * radius * pitch;
            let size = cell.sqrt();
            let mine = laid(own, radius, 0.);
            // A base at either end of the trunk has neighbours on one side
            // only; its cover and its touches are read in the middle rows.
            let inner = own.height - low > 2. * size && high - own.height > 2. * size;
            if inner {
                cover = cover.min(area(&mine) / cell);
            }
            let mut near = Vec::new();
            for (j, other) in rings.iter().enumerate() {
                if i == j || (other.height - own.height).abs() > 3. * size {
                    continue;
                }
                // The bark closes on itself: a neighbour is met a turn either
                // way as well.
                for lap in [-1., 0., 1.] {
                    let turn = wrap(other.angle - own.angle) + lap * std::f64::consts::TAU;
                    let d = apart(&mine, &laid(other, radius, turn)) / size;
                    crowd = crowd.max(-d);
                    near.push(d.abs());
                }
            }
            if inner {
                checked += 1;
                // Four edge neighbours in a tiling, each met along an edge:
                // the fourth nearest is the widest gap a base leaves.
                near.sort_by(f64::total_cmp);
                gap = gap.max(near[3]);
            }
        }
    }
    (gap, crowd, cover, checked)
}

/// R1: at width 1 and a flat section the bases tile the bark and their outer
/// ends alike, at 32, 96 and 256 bases.
#[test]
fn bases_at_full_width_tile_the_trunk_edge_to_edge() {
    for count in [32, 96, 256] {
        let (gap, crowd, cover, checked) = lattice(count, 1.);
        eprintln!(
            "{count} bases: {checked} rings read, widest gap {gap:.1e} and deepest \
             crowding {crowd:.1e} of a cell, thinnest cover {cover:.5}"
        );
        assert!(checked > 0, "{count} bases: no inner base to measure");
        assert!(
            crowd <= TOLERANCE,
            "{count} bases: a base crowds its neighbour by {crowd:.3} of a cell"
        );
        assert!(
            cover >= 1. - TOLERANCE,
            "{count} bases: a base covers {cover:.3} of its cell, the bark shows between"
        );
        assert!(
            gap <= TOLERANCE,
            "{count} bases: a base stands {gap:.3} of a cell off a neighbour"
        );
    }
}

/// R1's errors: the measure fails a lattice a tenth too narrow on its gaps
/// and a tenth too broad on its crowding.
#[test]
fn a_lattice_off_full_width_is_caught() {
    let (gap, _, cover, _) = lattice(96, 0.9);
    assert!(
        gap > TOLERANCE && cover < 1. - TOLERANCE,
        "gap {gap}, cover {cover}"
    );
    let (_, crowd, _, _) = lattice(96, 1.1);
    assert!(crowd > TOLERANCE, "crowding {crowd}");
}
