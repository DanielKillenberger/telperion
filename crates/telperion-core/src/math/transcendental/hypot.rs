//! The pinned libm's hypot, step for step, with the one square root it takes
//! done by the hardware. The pinned libm builds without its `arch` feature,
//! so its own square root is a software loop, and every containment test
//! against the shell pays for it. Square root is correctly rounded under
//! IEEE 754, in software and in hardware alike, so the two agree to the bit;
//! the test holds this against the pinned libm itself.

/// Splits `x * x` into a head and a tail that sum to it exactly.
fn square(x: f64) -> (f64, f64) {
    const SPLIT: f64 = 134_217_729.0; // 2^27 + 1
    let c = x * SPLIT;
    let head = x - c + c;
    let tail = x - head;
    let hi = x * x;
    (hi, head * head - hi + 2.0 * head * tail + tail * tail)
}

pub(super) fn hypot(x: f64, y: f64) -> f64 {
    let (mut big, mut small) = (x.to_bits() & !(1 << 63), y.to_bits() & !(1 << 63));
    if big < small {
        std::mem::swap(&mut big, &mut small);
    }
    let (eb, es) = ((big >> 52) as i64, (small >> 52) as i64);
    let (mut x, mut y) = (f64::from_bits(big), f64::from_bits(small));
    if es == 0x7ff {
        return y;
    }
    if eb == 0x7ff || small == 0 {
        return x;
    }
    if eb - es > 64 {
        return x + y;
    }
    // Scale by 2^±700 so the squares neither overflow nor lose their tails.
    let (up, down) = (
        f64::from_bits(0x6bb0000000000000),
        f64::from_bits(0x1430000000000000),
    );
    let mut z = 1.0;
    if eb > 0x3ff + 510 {
        (z, x, y) = (up, x * down, y * down);
    } else if es < 0x3ff - 450 {
        (z, x, y) = (down, x * up, y * up);
    }
    let (hx, lx) = square(x);
    let (hy, ly) = square(y);
    z * (ly + lx + hy + hx).sqrt()
}

#[cfg(test)]
mod tests {
    use super::hypot;
    use crate::rng::Rng;

    #[test]
    fn matches_the_pinned_libm_to_the_bit() {
        let edges = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            f64::MIN_POSITIVE,
            5e-324,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            1e-160,
            1e160,
            1e300,
            1e-300,
            3.0,
            4.0,
        ];
        let mut cases: Vec<(f64, f64)> = edges
            .iter()
            .flat_map(|&a| edges.iter().map(move |&b| (a, b)))
            .collect();
        let mut rng = Rng::new(0x5eed);
        for _ in 0..200_000 {
            let scale = 10f64.powi((rng.next_u32() % 40) as i32 - 20);
            cases.push((rng.range(-1.0, 1.0) * scale, rng.range(-1.0, 1.0) * scale));
        }
        for _ in 0..50_000 {
            let bits =
                |r: &mut Rng| f64::from_bits(((r.next_u32() as u64) << 32) | r.next_u32() as u64);
            cases.push((bits(&mut rng), bits(&mut rng)));
        }
        // Either side of the exponent-gap shortcut and the scaling thresholds.
        for e in [
            -1100, -1022, -600, -512, -66, -65, -64, -63, 510, 511, 512, 600, 1023,
        ] {
            let small = 2f64.powi(e);
            cases.extend([
                (1.0, small),
                (small, 1.0),
                (1.5, small * 1.5),
                (small, small),
            ]);
        }
        for (x, y) in cases {
            let (ours, theirs) = (hypot(x, y), libm::hypot(x, y));
            assert_eq!(
                ours.to_bits(),
                theirs.to_bits(),
                "hypot({x:e}, {y:e}): {ours:e} against libm's {theirs:e}"
            );
        }
    }
}
