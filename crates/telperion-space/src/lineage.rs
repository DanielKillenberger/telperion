//! Lineage-keyed randomness (fn-192 R1, R2). Every draw is a hash of the
//! bud's path from the root and of what it decides, never a position in a
//! stream, so adding a branch anywhere moves no other draw. Each draw's lead
//! past the bound it was tested against, in log-odds, is what `presence.rs`
//! grows its element in by.
use crate::fixed::{self, decide, mul, q32, ONE32};
use crate::species::NodeLaw;
use crate::transcendental::Transcendental;

pub mod keys;

pub(crate) use keys::{
    ABORTION, CONTINUATION, DOMINANCE, DORMANT, END, ENDED, FAILED, INSERTION, INTERNODE, MOVE,
    RELAY, ROLL, SHED, TROPISM, VIABILITY, WANDER, ZONE,
};

/// A hashed path from the root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Key(pub u64);

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

impl Key {
    /// The seed bud's lineage.
    pub fn root(seed: u64) -> Self {
        Self(mix(seed.wrapping_add(0x9e37_79b9_7f4a_7c15)))
    }

    /// The path one step further: `part` names the step among its siblings.
    pub fn child(self, part: u64) -> Self {
        let step = mix(part.wrapping_add(0x6a09_e667_f3bc_c909));
        Self(mix(self.0.rotate_left(23) ^ step))
    }

    /// The axis that carries a chain of ages on into `age`, keyed by that
    /// age's place on the reference axis from the lineage that began the
    /// chain, so an age passed through or grown in moves no other's key
    /// (host, 2026-10-05).
    pub fn onto(self, age: usize) -> Self {
        self.child(CONTINUATION).child(age as u64)
    }

    /// The draw this key decides: 53 bits, uniform on [0, 2^53).
    pub fn draw(self) -> Draw {
        Draw(mix(self.0 ^ 0xd1b5_4a32_d192_ed03) >> 11)
    }
}

/// A draw's 53 bits (fn-209, FINE-CONTRACT.md D1): every decision a draw
/// takes compares them, as an integer, with a probability's canonical
/// threshold, so two executors that hold the same bits and the same
/// threshold take the same decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Draw(pub u64);

/// 2^53: a probability's threshold where it is certain.
pub(crate) const ONE: u64 = 1 << 53;

/// A probability's canonical threshold: the draws under it are those
/// below ceil(p 2^53), which are exactly those whose real `d / 2^53` lies
/// below `p` (the scaling by 2^53 is exact), so the integer decision is
/// the real one to the bit. None for a probability of none or NaN.
pub(crate) fn threshold(p: f64) -> u64 {
    if p.is_nan() || p <= 0.0 {
        return 0;
    }
    if p >= 1.0 {
        return ONE;
    }
    (p * ONE as f64).ceil() as u64
}

impl Draw {
    /// The draw as a real, uniform on [0, 1): exact, 53 bits.
    #[cfg(test)]
    pub fn unit(self) -> f64 {
        self.0 as f64 / ONE as f64
    }

    /// Whether the draw falls under probability `p`: `draw < threshold(p)`.
    pub fn under(self, p: f64) -> bool {
        self.0 < threshold(p)
    }
}

/// A draw's unit as a signed Q32, `2u − 1`: `d >> 21` is `u` in Q32,
/// exact (CONTRACT-A.md 2.3, keyed draws).
fn signed(d: Draw) -> i64 {
    ((d.0 >> 21) as i64) * 2 - ONE32
}

/// The factor, Q32, the internode of the node keyed `key` is scaled by at
/// `spread` (`Form::internode_spread`): 1 + spread (2u − 1), u the node's
/// internode draw, so its mean is one; exactly one without spread.
/// Contract A: the spread converted once, one floor.
pub(crate) fn internode(key: u64, spread: f64) -> i64 {
    if spread <= 0.0 {
        return ONE32;
    }
    ONE32 + mul(q32(spread), signed(Key(key).child(INTERNODE).draw()))
}

/// `internode` as a real factor: what a node's grown wood is counted by
/// (the conversion's `SHORTEST` and the stations' count, fn-212), so the
/// wood is the length its internodes were laid at.
pub fn internode_factor(key: u64, spread: f64) -> f64 {
    internode(key, spread) as f64 / ONE32 as f64
}

/// The insertion angle, an amount (Q32 turns), of the axis whose lineage is
/// `lineage`, its PA's `angle` (radians) at `spread` (radians,
/// `Form::insertion_spread`): angle + spread (2u − 1), u the axis's
/// insertion draw, so its mean is the angle; exactly the angle without
/// spread. Contract A (2.4): each angle converted once, the offset an
/// amount by one floor, as the roll's.
pub(crate) fn insertion(lineage: u64, angle: f64, spread: f64) -> i64 {
    let angle = crate::geometry::amount(angle);
    if spread <= 0.0 {
        return angle;
    }
    let u = signed(Key(lineage).child(INSERTION).draw());
    angle + mul(crate::geometry::amount(spread), u)
}

/// The cosine of `insertion`'s angle in expectation over its draw, and its
/// square's: E cos(a + d) = cos a · sin s / s and E cos²(a + d) =
/// (1 + cos 2a · sin 2s / 2s) / 2 for d uniform on [−s, s] (fn-224). The
/// closed form reads both, exact in expectation; without spread, cos a and
/// its square, as it always read them.
pub(crate) fn turn(angle: f64, spread: f64) -> [f64; 2] {
    let cos = angle.cos_fixed();
    if spread <= 0.0 {
        return [cos, cos * cos];
    }
    let sinc = |x: f64| x.sin_fixed() / x;
    let square = (1.0 + (2.0 * angle).cos_fixed() * sinc(2.0 * spread)) / 2.0;
    [cos * sinc(spread), square]
}

/// The factor, Q32, the tropism of the axis whose lineage is `lineage` is
/// scaled by at `spread` (`Form::tropism_spread`): 1 + spread (2u − 1), u
/// the axis's tropism draw, so its mean is one; exactly one without
/// spread. Contract A: the spread converted once, one floor.
pub(crate) fn tropism(lineage: u64, spread: f64) -> i64 {
    if spread <= 0.0 {
        return ONE32;
    }
    ONE32 + mul(q32(spread), signed(Key(lineage).child(TROPISM).draw()))
}

/// A bud's PA under `lateral` for the draw `d`; none for a bare bud. The
/// bounds are the running sums of `lateral` in its order (FINE-CONTRACT.md
/// D3).
pub(crate) fn bud(lateral: &[f64], d: Draw) -> Option<usize> {
    let mut low = 0.0;
    for (pa, &p) in lateral.iter().enumerate() {
        let high = low + p;
        if p > 0.0 && d.under(high) {
            return Some(pa);
        }
        low = high;
    }
    None
}

/// The presence, Q32, over `window` of the draw `d` that made a bud of
/// `pa` under `lateral`: its lead past the nearer bound of the PA's run of
/// draws, in log-odds Q32 on the bounds' thresholds (host decision 8),
/// over the window. The lesser of the two bounds' presences (a presence
/// rises with its lead), each settled as `fixed::presence::of_under`
/// settles one. Taken only for a bud that is made: the logs it may take
/// are most of growth's arithmetic on one core.
pub(crate) fn bud_presence(lateral: &[f64], pa: usize, d: Draw, window: i64) -> u64 {
    use fixed::presence::{of_over, of_under, ONE};
    let (low, high) = bounds(lateral, pa);
    let over = if low <= 0.0 {
        ONE
    } else {
        of_over(threshold(low), d.0, window)
    };
    over.min(of_under(threshold(high), d.0, window))
}

/// The bounds of `pa`'s run of draws: the running sums of `lateral`.
fn bounds(lateral: &[f64], pa: usize) -> (f64, f64) {
    let low = lateral[..pa].iter().fold(0.0, |low, &p| low + p);
    (low, low + lateral[pa])
}

/// A zone's node count under `law` for the draw `d`, by the inverse of the
/// law's distribution, and each node's lead into `leads`, in Q32 log-odds
/// (`fixed.rs`; D18 reads the presences they give): a node of the
/// Poisson law is made as the mean passes its draw (FINE-CONTRACT.md D2).
/// A uniform node's lead is the log of the draw's ratio to its bound,
/// unbounded as the bound nears 0, where the node becomes a sure one, and
/// gentle as it nears 1.
#[cfg(test)]
pub(crate) fn nodes(law: NodeLaw, d: Draw, cap: u32, leads: &mut Vec<i64>) {
    leads.clear();
    walk(law, d, cap, |past| {
        leads.push(match past {
            Past::Uniform(bound) => decide::past(d, bound),
            Past::Poisson(bound) => decide::above(d, bound),
        })
    });
}

/// `nodes`, each node's presence over `window` (Q32, `decide::node_window`) in
/// place of its lead: the same nodes, their presences settled without
/// their logs where bounds of them settle them.
pub(crate) fn node_presences(law: NodeLaw, d: Draw, cap: u32, window: i64, out: &mut Vec<u64>) {
    out.clear();
    walk(law, d, cap, |past| {
        out.push(match past {
            Past::Uniform(bound) => decide::node_past(d, bound, window),
            Past::Poisson(bound) => decide::node_above(d, bound, window),
        })
    });
}

/// The bound a node's draw stands past: a uniform law's share of its span,
/// or a Poisson law's cumulative chance.
enum Past {
    Uniform(f64),
    Poisson(f64),
}

/// Each node `law` makes for the draw `d`, at most `cap`, in order.
fn walk(law: NodeLaw, d: Draw, cap: u32, mut node: impl FnMut(Past)) {
    match law {
        NodeLaw::Uniform { min, max } => {
            // Node j stands while u lies above its share of the span.
            let span = max - min + 1.0;
            for j in 0..(max.ceil() as u32).min(cap) {
                let bound = (f64::from(j) - min + 1.0) / span;
                if d.under(bound) {
                    break;
                }
                node(Past::Uniform(bound));
            }
        }
        NodeLaw::Poisson { mean } => {
            // Node k stands while u lies above P(count <= k).
            let mut mass = (-mean).exp_fixed();
            let mut cumulative = mass;
            let mut k = 0u32;
            while !d.under(cumulative) && k < cap {
                node(Past::Poisson(cumulative));
                k += 1;
                mass *= mean / f64::from(k);
                cumulative += mass;
            }
        }
    }
}
