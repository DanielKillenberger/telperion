//! fn-249 R2: the limb division against the bitwise long division it
//! replaced, on the boundary values and on random ones of every width.
use super::{Fault, I256};

/// The bitwise long division the limb division replaced (fn-245's profile:
/// 256 steps for every quotient), kept as its oracle.
fn bitwise(x: I256, d: i128) -> Result<I256, Fault> {
    if d == 0 {
        return Err(Fault::Zero);
    }
    let (hi, lo) = x.magnitude();
    let den = d.unsigned_abs();
    let (mut r, mut q) = (0u128, (0u128, 0u128));
    for i in (0..256).rev() {
        let bit = if i >= 128 { hi >> (i - 128) } else { lo >> i } & 1;
        r = (r << 1) | bit;
        if r >= den {
            r -= den;
            if i >= 128 {
                q.0 |= 1 << (i - 128);
            } else {
                q.1 |= 1 << i;
            }
        }
    }
    Ok(I256::signed(x.negative() != (d < 0), q))
}

/// Limbs that reach every branch: none, one, both halves' edges, all ones.
const EDGES: [u32; 6] = [0, 1, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0x0001_0000];

/// A xorshift stream, so the cases are the same on every run.
struct Stream(u64);

impl Stream {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A word of `n` 32-bit limbs, each an edge or random.
    fn word(&mut self, n: usize) -> u128 {
        (0..n).fold(0u128, |w, k| {
            let r = self.next();
            let limb = match r % 3 {
                0 => EDGES[(r >> 8) as usize % EDGES.len()],
                _ => (r >> 32) as u32,
            };
            w | (u128::from(limb) << (32 * k))
        })
    }
}

fn agree(x: I256, d: i128) {
    assert_eq!(x.quotient(d), bitwise(x, d), "{x:?} / {d}");
}

#[test]
fn the_limb_division_is_the_bitwise_one_at_the_boundaries() {
    let top = I256 {
        hi: i128::MIN,
        lo: 0,
    };
    let numerators = [
        I256::of(0),
        I256::of(1),
        I256::of(-1),
        I256::of(i128::MAX),
        I256::of(i128::MIN),
        I256 {
            hi: i128::MAX,
            lo: u128::MAX,
        },
        top,
        I256 { hi: -1, lo: 1 },
        // Knuth's add-back case in 32-bit limbs, over 2^95 + 1 below.
        I256 {
            hi: 0,
            lo: 0x7fff_ffff_8000_0000_0000_0000_0000_0000,
        },
    ];
    let mut divisors = vec![0, 1, -1, 2, 3, i128::MAX, i128::MIN, i128::MIN + 1];
    for shift in [31, 32, 33, 63, 64, 65, 95, 96, 97, 126] {
        divisors.extend([1i128 << shift, (1i128 << shift) - 1, (1i128 << shift) + 1]);
    }
    let negated: Vec<i128> = divisors.iter().map(|d| d.wrapping_neg()).collect();
    for x in numerators {
        for &d in divisors.iter().chain(&negated) {
            agree(x, d);
        }
    }
    assert_eq!(top.quotient(-1), Ok(top), "-2^255 / -1 wraps as it did");
    assert_eq!(top.quotient(0), Err(Fault::Zero));
}

#[test]
fn the_limb_division_is_the_bitwise_one_at_random() {
    let mut s = Stream(0x9e37_79b9_7f4a_7c15);
    for _ in 0..200_000 {
        let (nx, nd) = (s.next() as usize % 9, 1 + s.next() as usize % 4);
        let lo = s.word(nx.min(4));
        let hi = s.word(nx.saturating_sub(4)) as i128;
        let x = I256 { hi, lo };
        let d = s.word(nd) as i128;
        agree(x, d);
        agree(x, d.wrapping_neg());
    }
}
