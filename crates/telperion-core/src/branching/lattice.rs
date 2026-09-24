//! The cell a spiral gives each of its members on a stem.
//!
//! Members set `divergence` apart round the stem and `spacing` apart along it
//! stand on a lattice once the stem is unrolled: the one spanned by a full turn
//! of the bark and by the step from one member to the next. Its two shortest
//! vectors are the parastichies the eye reads as the crossing spirals, and the
//! parallelogram they span, centred on a member, is that member's cell: the
//! cells tile the bark, each meeting its four parastichy neighbours along an
//! edge. A cell is stated as two half-diagonals in (radians round, metres
//! along), so it keeps its angle on bark of any radius.

/// The most reduction steps a lattice takes. Each step shortens the longer
/// vector, so a lattice of any real stem is reduced long before this.
const STEPS: usize = 64;

type Pair = [f64; 2];

fn dot(a: Pair, b: Pair) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

fn less(a: Pair, b: Pair, m: f64) -> Pair {
    [a[0] - b[0] * m, a[1] - b[1] * m]
}

/// The two shortest vectors of the lattice spanned by `a` and `b`, shorter
/// first: Lagrange's reduction, which subtracts the nearest whole multiple of
/// the shorter from the longer until the longer is no longer shortened.
fn reduced(mut a: Pair, mut b: Pair) -> (Pair, Pair) {
    if dot(a, a) > dot(b, b) {
        std::mem::swap(&mut a, &mut b);
    }
    for _ in 0..STEPS {
        b = less(b, a, (dot(a, b) / dot(a, a)).round());
        if dot(b, b) >= dot(a, a) {
            break;
        }
        std::mem::swap(&mut a, &mut b);
    }
    (a, b)
}

/// The cell of a member on bark of radius `girth`, drawn at `width` of its
/// full size: two half-diagonals in (radians round, metres along), turned so
/// the first leads the second counterclockwise seen from outside the stem.
pub(super) fn cell(girth: f64, spacing: f64, divergence: f64, width: f64) -> [Pair; 2] {
    let turn = std::f64::consts::TAU;
    let step =
        (divergence.to_radians() + std::f64::consts::PI).rem_euclid(turn) - std::f64::consts::PI;
    // Unrolled in metres, so the reduction weighs round and along alike; the
    // next member stands one step round and one spacing further down.
    let (a, b) = reduced([turn * girth, 0.], [step * girth, -spacing]);
    let half = |p: Pair, q: Pair, sign: f64| {
        [
            (p[0] + q[0] * sign) / 2. / girth * width,
            (p[1] + q[1] * sign) / 2. * width,
        ]
    };
    let (first, second) = (half(a, b, 1.), half(a, b, -1.));
    if first[0] * second[1] - first[1] * second[0] >= 0. {
        [first, second]
    } else {
        [second, first]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Transcendental;

    /// The unit bark direction a member at `turn` radians stands on.
    fn round(turn: f64) -> Pair {
        let (sin, cos) = turn.sin_cos_fixed();
        [cos, sin]
    }

    /// A cell covers exactly the bark one member is owed, whatever the count:
    /// the parallelogram's area is the turn's circumference times a spacing.
    #[test]
    fn a_full_cell_covers_the_bark_one_member_is_owed() {
        for (girth, spacing) in [(0.3, 0.3), (0.3, 0.04), (0.5, 0.012), (0.2, 1.2)] {
            let [a, b] = cell(girth, spacing, 137.5, 1.);
            // Half-diagonals span half the parallelogram: area = 2 |a x b|.
            let area = 2. * (a[0] * b[1] - a[1] * b[0]) * girth;
            let owed = std::f64::consts::TAU * girth * spacing;
            assert!(
                (area - owed).abs() < 1e-9 * owed.max(1.),
                "girth {girth}, spacing {spacing}: cell {area}, owed {owed}"
            );
        }
    }

    /// The reduced vectors are the lattice's shortest: no member stands nearer
    /// its neighbour than the shorter of them.
    #[test]
    fn the_reduced_vectors_are_the_nearest_neighbours() {
        let (girth, spacing, divergence) = (0.3_f64, 0.05, 137.5_f64);
        let (a, _) = reduced(
            [std::f64::consts::TAU * girth, 0.],
            [
                (divergence.to_radians() - std::f64::consts::TAU) * girth,
                -spacing,
            ],
        );
        let shortest = dot(a, a).sqrt();
        for k in 1..400 {
            let at = round(f64::from(k) * divergence.to_radians());
            let chord = at[1].atan2(at[0]).abs() * girth;
            let apart = chord.hypot(f64::from(k) * spacing);
            assert!(
                apart + 1e-12 >= shortest,
                "member {k} stands {apart} off, nearer than {shortest}"
            );
        }
    }
}
