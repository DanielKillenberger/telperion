//! fn-255 R3a: the volumes on degenerate foliage, and the kernel against
//! its own definition.
use super::*;
use crate::light::Light;
use crate::transcendental::Transcendental;

const K: f64 = 0.5;
const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);
/// A leaf's own variance: a leaf of the oak's size (0.00181 m²) reaching
/// its own size.
const S2: f64 = 0.00181 / 7.0;

fn sky() -> Vec<(Vec3, f64)> {
    Light {
        extinction: K,
        sky: 0.5,
    }
    .directions()
}

/// Every leaf on the seed axis.
fn one_axis(leaves: &[(Vec3, f64)]) -> Volumes {
    let leaves: Vec<_> = leaves.iter().map(|&(p, a)| (p, a, 0, S2)).collect();
    Volumes::build(Candidate::default(), &leaves, &[None])
}

/// A leaf's kernel spreads its area over the plane its rays cross: the
/// vertical rays through it, summed over a fine grid, hold its area.
#[test]
fn a_kernel_holds_its_leaf_area() {
    let kernel = Moments::leaf(Vec3::new(0.3, -0.2, 1.0), 0.5, 0.04).kernel().unwrap();
    let step = 0.005;
    let mut sum = 0.0;
    for i in -200..200 {
        for j in -200..200 {
            let at = Vec3::new(0.3 + (f64::from(i) + 0.5) * step, -0.2 + (f64::from(j) + 0.5) * step, -5.0);
            sum += kernel.along(at, UP) * step * step;
        }
    }
    assert!((sum - 0.5).abs() < 1e-3, "{sum}");
}

/// Along any ray the closed form is the density's integral, here by a
/// fine midpoint rule, for a rotated ellipsoid and a ray starting inside.
#[test]
fn the_ray_integral_is_the_densitys() {
    let mut m = Moments::default();
    for (x, y, z) in [(0.0, 0.0, 0.0), (1.0, 0.5, 0.2), (-0.4, 0.3, 0.6), (0.2, -0.7, -0.1)] {
        m.add(&Moments::leaf(Vec3::new(x, y, z), 0.3, 0.02));
    }
    let kernel = m.kernel().unwrap();
    let toward = Vec3::new(0.3, -0.2, 0.9).unit().unwrap();
    let density = |p: Vec3| {
        let u = p - kernel.centre;
        let q = u.dot(apply(kernel.inverse, u));
        kernel.scale * (1.0 - q).max(0.0)
    };
    for start in [Vec3::new(0.1, 0.0, -2.0), kernel.centre, Vec3::new(0.2, 0.1, 0.3)] {
        let step = 1e-4;
        let numeric: f64 = (0..60_000)
            .map(|k| density(start + toward * ((f64::from(k) + 0.5) * step)) * step)
            .sum();
        let exact = kernel.along(start, toward);
        assert!((numeric - exact).abs() < 1e-6 * exact.max(1.0), "{numeric} {exact}");
    }
}

/// No leaf: the open sky.
#[test]
fn an_empty_snapshot_reads_the_open_sky() {
    let volumes = Volumes::build(Candidate::default(), &[], &[None, Some(0)]);
    let (light, visited) = volumes.read(Vec3::new(0.0, 0.0, 1.0), &sky(), K);
    assert!((light - 1.0).abs() < 1e-12, "{light}");
    assert_eq!(visited, 0);
}

/// One leaf, a line of them and a plane of them each shade a bud below
/// them by a finite amount, and the open sky beside them; at no
/// extinction nothing shades.
#[test]
fn a_point_a_line_and_a_plane_shade_finitely() {
    let point = vec![(Vec3::new(0.0, 0.0, 2.0), 0.00181)];
    let line: Vec<_> = (0..20).map(|k| (Vec3::new(f64::from(k) * 0.05 - 0.5, 0.0, 2.0), 0.00181)).collect();
    let plane: Vec<_> = (0..400)
        .map(|k| (Vec3::new(f64::from(k % 20) * 0.05 - 0.5, f64::from(k / 20) * 0.05 - 0.5, 2.0), 0.00181))
        .collect();
    let directions = sky();
    for leaves in [point, line, plane] {
        let volumes = one_axis(&leaves);
        let under = volumes.read(Vec3::new(0.0, 0.0, 1.9), &directions, K).0;
        assert!(under.is_finite() && under > 0.0 && under < 1.0, "{under}");
        let aside = volumes.read(Vec3::new(30.0, 0.0, 0.0), &directions, K).0;
        assert!((aside - 1.0).abs() < 1e-12, "{aside}");
        let neutral = volumes.read(Vec3::new(0.0, 0.0, 1.9), &directions, 0.0).0;
        assert!((neutral - 1.0).abs() < 1e-12, "{neutral}");
    }
}

/// Leaf area is conserved through every merge: the root's aggregate is
/// the leaves' sum however they are grouped, and its moments the sum of
/// its parts'.
#[test]
fn leaf_area_is_conserved_through_merges() {
    let leaves: Vec<_> = (0..30)
        .map(|k| (Vec3::new(f64::from(k).sin_fixed(), f64::from(k).cos_fixed(), 0.1 * f64::from(k)), 0.01 * f64::from(k + 1)))
        .collect();
    let mut whole = Moments::default();
    let (mut a, mut b) = (Moments::default(), Moments::default());
    for (k, &(p, area)) in leaves.iter().enumerate() {
        let leaf = Moments::leaf(p, area, S2);
        whole.add(&leaf);
        if k % 3 == 0 { &mut a } else { &mut b }.add(&leaf);
    }
    a.add(&b);
    let sum: f64 = leaves.iter().map(|l| l.1).sum();
    assert!((whole.m0 - sum).abs() < 1e-12 && (a.m0 - sum).abs() < 1e-12);
    for (x, y) in whole.m2.iter().zip(&a.m2) {
        assert!((x - y).abs() < 1e-12 * x.abs().max(1.0));
    }
}

/// A subtree's shade goes to nothing as its presence does: the light's
/// shortfall scales with its leaves' area down to none.
#[test]
fn a_fading_subtree_shades_less_by_degree() {
    let at = Vec3::new(0.05, 0.0, 1.0);
    let shortfall = |presence: f64| {
        let leaves: Vec<_> = (0..10)
            .map(|k| (Vec3::new(f64::from(k) * 0.04, 0.0, 1.3), 0.00181 * presence, 1, S2))
            .collect();
        let volumes = Volumes::build(Candidate::default(), &leaves, &[None, Some(0)]);
        1.0 - volumes.read(at, &sky(), K).0
    };
    let (whole, small, tiny) = (shortfall(1.0), shortfall(1e-3), shortfall(1e-9));
    assert!(whole > 0.0);
    assert!(small < 2e-3 * whole && small > 0.0, "{small} {whole}");
    assert!(tiny < 1e-8 * whole && tiny >= 0.0, "{tiny}");
    assert!((shortfall(0.0)).abs() < 1e-15);
}

/// Continuity across a parent's refinement: a bud walked from inside a
/// subtree to far beyond it, across the interval where its aggregate
/// takes over from its children, reads a light that never steps.
#[test]
fn the_light_is_continuous_across_refinement() {
    // A bearer (axis 0) with leaves, two children spread apart above it.
    let mut leaves = Vec::new();
    for k in 0..8 {
        let x = f64::from(k) * 0.1;
        leaves.push((Vec3::new(x, 0.0, 3.0), 0.002, 0, S2));
        leaves.push((Vec3::new(-1.0 + x, 0.4, 3.6), 0.002, 1, S2));
        leaves.push((Vec3::new(1.0 + x, -0.4, 3.4), 0.002, 2, S2));
    }
    let volumes = Volumes::build(Candidate::default(), &leaves, &[None, Some(0), Some(0)]);
    let directions = sky();
    let (mut last, mut biggest, mut opened, mut whole): (Option<f64>, f64, bool, bool) = (None, 0.0, false, false);
    // Straight down from under the leaves, until far below them.
    for k in 0..=25_000 {
        let at = Vec3::new(0.35, 0.0, 2.5 - f64::from(k) * 0.001);
        let (light, visited) = volumes.read(at, &directions, K);
        opened |= visited > 1;
        whole |= visited == 1 && light < 1.0;
        if let Some(l) = last {
            biggest = biggest.max((light - l).abs());
        }
        last = Some(light);
    }
    assert!(opened && whole, "the walk crosses the refinement");
    assert!(biggest < 2e-4, "a step of {biggest}");
}
