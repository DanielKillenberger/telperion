//! The light lattice against closed forms (fn-197 step 1): a leaf slab's
//! exp(-k L) along every sky direction (Monsi and Saeki 1953), a sphere of
//! even leaf density along any ray, GreenLab's production on an even crown
//! under light from overhead (Letort et al., eq. 1), the sky's shares, and
//! the neutral and by-degree contracts.
use super::*;
use crate::transcendental::Transcendental;

const K: f64 = 0.5;

/// Leaf at `density` (m2 per m3) filling `inside`, sampled `fine` times
/// per cell along each axis over the box `low` to `high`.
fn filled(
    low: Vec3,
    high: Vec3,
    density: f64,
    fine: u32,
    inside: impl Fn(Vec3) -> bool,
) -> Vec<(Vec3, f64)> {
    let step = CELL / f64::from(fine);
    let count = |a: f64, b: f64| ((b - a) / step).round() as i64;
    let mut out = Vec::new();
    for i in 0..count(low.x, high.x) {
        for j in 0..count(low.y, high.y) {
            for k in 0..count(low.z, high.z) {
                let p = low + Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5) * step;
                if inside(p) {
                    out.push((p, density * step.powi(3)));
                }
            }
        }
    }
    out
}

fn one(toward: Vec3) -> Vec<(Vec3, f64)> {
    vec![(toward.unit().unwrap(), 1.0)]
}

fn field(leaves: &[(Vec3, f64)], reads: &[Vec3], directions: &[(Vec3, f64)]) -> Field {
    let points = leaves.iter().map(|l| l.0).chain(reads.iter().copied());
    let mut f =
        Field::spanning(points, super::field::NODES).expect("a test lattice inside its nodes");
    f.deposit_all(leaves);
    // The directions on four cores: each node sums them in their order.
    f.sweep(K, directions, 4);
    f
}

/// Every sky direction through a wide slab of even density: the optical
/// depth at a node is k L / sin e to rounding, the slab's top half a cell
/// above its top layer of nodes (the trapezoid's own half layer).
#[test]
fn a_leaf_slab_transmits_exp_of_minus_k_l_along_every_direction() {
    let density = 0.8;
    // Nodes from z = 0 to 4 m, 40 m across: deposits on the nodes.
    let mut leaves = Vec::new();
    for i in -40..=40 {
        for j in -40..=40 {
            for k in 0..=8 {
                let p = Vec3::new(f64::from(i), f64::from(j), f64::from(k)) * CELL;
                leaves.push((p, density * CELL.powi(3)));
            }
        }
    }
    let top = 8.5 * CELL;
    for (toward, _) in Light::default().directions() {
        let f = field(&leaves, &[], &one(toward));
        for k in 0..=8 {
            let at = Vec3::new(0.0, 0.0, f64::from(k) * CELL);
            let lai = density * (top - at.z);
            let expected = (-K * lai / toward.z).exp_fixed();
            let got = f.at(at);
            // Areas enter and light is read in f32 (fn-254), so the law
            // holds to a few units of f32's last place.
            assert!(
                (got - expected).abs() < 1e-6,
                "toward {toward:?}, depth {}: {got} against {expected}",
                top - at.z
            );
        }
    }
}

/// The distance from `p` to a sphere's surface along unit `toward`.
fn chord(p: Vec3, centre: Vec3, radius: f64, toward: Vec3) -> f64 {
    let q = p - centre;
    let b = q.dot(toward);
    -b + (b * b - q.dot(q) + radius * radius).sqrt()
}

/// A sphere of even density along overhead and every slanted direction:
/// exp(-k rho chord), within the lattice's discretisation (an optical
/// depth error under a quarter of k rho times two cells).
#[test]
fn a_leaf_sphere_transmits_exp_of_minus_k_rho_chord() {
    let (centre, radius, density) = (Vec3::new(0.3, -0.2, 10.0), 4.0, 0.6);
    let r = Vec3::new(radius, radius, radius);
    let leaves = filled(centre - r, centre + r, density, 4, |p| {
        (p - centre).length() < radius
    });
    let reads = [
        centre,
        centre + Vec3::new(1.2, 0.7, -1.5),
        centre + Vec3::new(-2.0, 0.4, 0.8),
        centre + Vec3::new(0.5, -2.5, -2.0),
    ];
    let tolerance = 0.25 * K * density * 2.0 * CELL;
    let mut worst = 0.0f64;
    for (toward, _) in Light::default().directions() {
        let f = field(&leaves, &reads, &one(toward));
        for &p in &reads {
            let expected = K * density * chord(p, centre, radius, toward);
            let got = -f.at(p).ln_fixed();
            worst = worst.max((got - expected).abs());
            assert!(
                (got - expected).abs() < tolerance,
                "toward {toward:?} at {p:?}: depth {got} against {expected}"
            );
        }
    }
    println!("worst optical depth error {worst:.4} (tolerance {tolerance:.4})");
}

/// GreenLab's production on a crown of even density under light from
/// overhead: the light the leaves gather, the sum of k times each leaf's
/// area times its light, is Sp (1 - exp(-k S / Sp)), Sp the crown's
/// projected area and S its leaf area (Letort et al., eq. 1, with PAR and
/// RUE 1). The lattice spreads the crown's rim over half a cell, so the
/// error falls as the crown widens: under 4 percent at 5 m across a
/// radius, under half that at 10.
#[test]
fn an_even_crown_gathers_greenlabs_production_from_overhead() {
    let error = |radius: f64| {
        let (low, high, density) = (5.0, 8.0, 1.0);
        let corner = Vec3::new(radius, radius, 0.0);
        let leaves = filled(
            Vec3::new(0.0, 0.0, low) - corner,
            Vec3::new(0.0, 0.0, high) + corner,
            density,
            4,
            |p| p.x.hypot_fixed(p.y) < radius,
        );
        let f = field(&leaves, &[], &one(Vec3::new(0.0, 0.0, 1.0)));
        let gathered: f64 = leaves.iter().map(|&(p, a)| K * a * f.at(p)).sum();
        let projected = std::f64::consts::PI * radius * radius;
        let area: f64 = leaves.iter().map(|l| l.1).sum();
        let expected = projected * (1.0 - (-K * area / projected).exp_fixed());
        (gathered / expected - 1.0).abs()
    };
    let (narrow, wide) = (error(5.0), error(10.0));
    println!("GreenLab production error: {narrow:.4} at 5 m, {wide:.4} at 10 m");
    assert!(narrow < 0.04, "{narrow} at 5 m");
    assert!(
        wide < narrow / 1.8,
        "{wide} at 10 m against {narrow} at 5 m"
    );
}

/// The sky's shares: each sums to 1; overhead only at 0; the standard
/// overcast sky's band shares at 0.5 and the uniform sky's at 1, each its
/// closed-form integral checked against a midpoint quadrature.
#[test]
fn the_skys_shares_follow_their_radiance_laws() {
    let quadrature = |radiance: fn(f64) -> f64, from: f64, to: f64| {
        let n = 20_000;
        let h = (to - from).to_radians() / f64::from(n);
        let total = |a: f64, b: f64, h: f64| {
            let m = ((b - a) / h).round() as u32;
            (0..m)
                .map(|i| {
                    let e = a + (f64::from(i) + 0.5) * h;
                    radiance(e) * e.sin_fixed() * e.cos_fixed() * h
                })
                .sum::<f64>()
        };
        total(from.to_radians(), to.to_radians(), h) / total(0.0, 90f64.to_radians(), h)
    };
    let laws: [(f64, fn(f64) -> f64); 2] = [(0.5, |e| 1.0 + 2.0 * e.sin_fixed()), (1.0, |_| 1.0)];
    for (sky, radiance) in laws {
        let d = Light { extinction: K, sky }.directions();
        let band = |from: usize, to: usize| d[from..to].iter().map(|x| x.1).sum::<f64>();
        let shares = [band(0, 1), band(1, 5), band(5, 9)];
        for (b, &share) in shares.iter().enumerate() {
            let expected = quadrature(radiance, BANDS[b + 1], BANDS[b]);
            assert!(
                (share - expected).abs() < 1e-6,
                "sky {sky} band {b}: {share} against {expected}"
            );
        }
    }
    for sky in [0.0, 0.2, 0.5, 0.8, 1.0] {
        let total: f64 = Light { extinction: K, sky }
            .directions()
            .iter()
            .map(|d| d.1)
            .sum();
        assert!(
            (total - 1.0).abs() < 1e-12,
            "sky {sky}: shares sum to {total}"
        );
    }
    let overhead = Light {
        extinction: K,
        sky: 0.0,
    }
    .directions();
    assert_eq!(overhead[0].1, 1.0);
    assert!(overhead[1..].iter().all(|d| d.1 == 0.0));
}

/// Neutral: with no extinction every point reads exactly 1.
#[test]
fn no_extinction_is_light_everywhere_to_the_bit() {
    let leaves = filled(
        Vec3::new(-2.0, -2.0, 1.0),
        Vec3::new(2.0, 2.0, 4.0),
        3.0,
        2,
        |_| true,
    );
    let light = Light {
        extinction: 0.0,
        sky: 0.7,
    };
    let f = Field::new(&light, &leaves, &[]);
    assert!(leaves.iter().all(|&(p, _)| f.at(p) == 1.0));
}

/// A leaf moved across a cell's face, and a read moved across one, change
/// the light by degree: no step of 1 mm moves it by more than a small
/// multiple of the step.
#[test]
fn a_leaf_or_a_read_moving_across_a_cell_changes_the_light_by_degree() {
    let light = Light {
        extinction: 1.0,
        sky: 0.5,
    };
    let read = Vec3::new(0.1, 0.2, 1.0);
    let lit = |leaf: Vec3, at: Vec3| Field::new(&light, &[(leaf, 0.2)], &[at]).at(at);
    let (step, steps) = (1e-3, 2_000);
    let mut worst = 0.0f64;
    for i in 0..steps {
        let x = -0.5 + step * f64::from(i);
        let leaf = |x: f64| Vec3::new(x, 0.1, 2.3);
        worst = worst.max((lit(leaf(x + step), read) - lit(leaf(x), read)).abs());
        let at = |x: f64| Vec3::new(x, 0.15, 1.1);
        worst = worst.max((lit(leaf(0.0), at(x + step)) - lit(leaf(0.0), at(x))).abs());
    }
    assert!(
        worst < 50.0 * step,
        "a 1 mm step moved the light by {worst}"
    );
}

/// A fully shaded sample stays at nothing: the corners' f32 weights may
/// sum a place over one, and the light is never read below nothing or
/// above one (fn-254 review, its three points and a sweep of the cell).
#[test]
fn a_shaded_sample_is_never_below_nothing() {
    let corners: Vec<(Vec3, f64)> = (0..8)
        .map(|c| {
            let at = |a: usize| if (c >> a) & 1 == 1 { 0.5 } else { 0.0 };
            (Vec3::new(at(0), at(1), at(2)), 1e3)
        })
        .collect();
    let f = field(&corners, &[], &one(Vec3::new(0.0, 0.0, 1.0)));
    let given = [
        Vec3::new(0.4396865964, 0.02739467844, 0.1894081980),
        Vec3::new(0.01489860937, 0.10931898654, 0.25267764926),
        Vec3::new(
            0.2360713630914688,
            0.050350602716207504,
            0.21708591282367706,
        ),
    ];
    let swept = (0..4096).map(|k| {
        let u = |s: u32| f64::from((k * s) % 997) / 997.0 * 0.5;
        Vec3::new(u(13), u(31), u(71))
    });
    for at in given.into_iter().chain(swept) {
        let light = f.at(at);
        assert!((0.0..=1.0).contains(&light), "{at:?}: {light:e}");
    }
}

/// fn-294 R6: a tree whose leaves and tips span more lattice nodes than
/// the lattice may hold is refused by name before the lattice is
/// allocated, never an assert: the oak grown 8 years, its lattice held to
/// `field::SMALL` nodes in place of the platform's 2^28 (fn-349).
#[test]
fn a_lattice_past_its_nodes_is_refused_by_name() {
    let request = crate::Request {
        age: 8,
        seed: 1,
        light: Light {
            extinction: K,
            sky: 0.5,
        },
    };
    let oak = std::sync::Arc::new(crate::oak());
    let paused = crate::grow::cycle::Paused::begin(oak, request, AT_AGE, false);
    let mut paused = paused.unwrap().lattice_within(super::field::SMALL);
    let refused = paused.grow_to(request.age).unwrap_err();
    assert!(
        matches!(&refused, crate::Error::Refused { input, .. } if input == "light.lattice"),
        "{refused}"
    );
}

const AT_AGE: crate::grow::cycle::Retiring = crate::grow::cycle::Retiring::AtAge;

/// fn-351 speed round 2: the quantised deposit cut into parts for four
/// cores is the deposit in turn, node for node, and a node only the parts
/// together fill past its u32 is refused as it is in turn.
#[test]
fn the_deposit_in_parts_is_the_deposit_in_turn() {
    let light = Light {
        extinction: K,
        sky: 0.5,
    };
    let most = super::field::NODES;
    let leaves: Vec<(Vec3, f64)> = (0..40_000)
        .map(|k| {
            let t = f64::from(k);
            let at = Vec3::new(
                (t * 0.37).sin() * 2.0,
                (t * 0.71).cos() * 2.0,
                2.0 + (t * 0.13).sin(),
            );
            (at, 0.0018 * (1.0 + (t * 0.3).cos()))
        })
        .collect();
    let quanta = |threads: usize| {
        let field = Field::quantised_with(&light, &leaves, &[], (threads, most));
        field.unwrap().1
    };
    let (turn, parts) = (quanta(1), quanta(4));
    assert!(turn.iter().any(|&q| q > 0), "leaf deposited");
    assert_eq!(turn, parts);
    // 16,384 leaves of 0.3 m² on one node: 5.2e9 quanta, a quarter of
    // them under a u32.
    let heavy = vec![(Vec3::new(0.0, 0.0, 1.0), 0.3); 1 << 14];
    for threads in [1, 4] {
        let refused = Field::quantised_with(&light, &heavy, &[], (threads, most));
        let refused = refused.err().expect("refused");
        assert!(
            format!("{refused}").starts_with("light.deposit"),
            "{threads}: {refused}"
        );
    }
}

/// fn-351 speed round 2: the deposit's count of quanta is `round`'s, cast
/// as it was, at every edge and over a sweep of deposits; and a leaf's
/// corners, tested against the box once, are `corners`' to the bit.
#[test]
fn the_deposits_count_and_corners_are_the_ones_it_took() {
    use super::field::{rounded, Grid, NODES, QUANTUM};
    let counted = |x: f64| (x.round() <= f64::from(u32::MAX)).then(|| x.round() as u32);
    let edges = [
        0.0,
        -0.0,
        0.25,
        0.499_999_999_999_999_94,
        0.5,
        0.500_000_000_000_000_1,
        1.5,
        2.5,
        -0.3,
        -0.5,
        -7.0,
        4_294_967_294.5,
        4_294_967_295.0,
        4_294_967_295.499_999,
        4_294_967_295.5,
        4_294_967_296.0,
        1e300,
        f64::INFINITY,
        f64::NAN,
    ];
    for x in edges {
        assert_eq!(rounded(x), counted(x), "{x:e}");
    }
    let mut state = 1u64;
    for _ in 0..200_000 {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let area = (state >> 40) as f32 / 1e5;
        let weight = ((state >> 16) & 0xff_ffff) as f32 / 16_777_216.0;
        let x = f64::from(area) * f64::from(weight) / QUANTUM;
        assert_eq!(rounded(x), counted(x), "{x:e}");
    }
    let points = [
        Vec3::new(0.3, -1.2, 2.7),
        Vec3::new(-0.0, 0.0, 0.5),
        Vec3::new(-1e-9, 1.999_999, 3.25),
        Vec3::new(1.0, 1.0, 1.0),
    ];
    let grid = Grid::spanning(points.iter().copied(), NODES).unwrap();
    for at in points {
        for (took, takes) in grid.corners(at).iter().zip(&grid.inside(at)) {
            assert_eq!(took.0, Some(takes.0), "{at:?}");
            assert_eq!(took.1.to_bits(), takes.1.to_bits(), "{at:?}");
        }
    }
}
