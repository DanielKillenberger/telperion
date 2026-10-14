//! Lineage-keyed randomness (fn-192 R1, R2). Every draw is a hash of the
//! bud's path from the root and of what it decides, never a position in a
//! stream, so adding a branch anywhere moves no other draw. Each draw's lead
//! past the bound it was tested against, in log-odds, is what `presence.rs`
//! grows its element in by.
use crate::contract::{self, decide, presence};
use crate::species::NodeLaw;
use crate::transcendental::Transcendental;

pub mod keys;

pub(crate) use keys::{
    ABORTION, CONTINUATION, DOMINANCE, DORMANT, END, ENDED, FAILED, INSERTION, INTERNODE, MOVE,
    LAID, PLACED, RELAY, ROLL, SHED, TROPISM, VIABILITY, WANDER, ZONE,
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

/// A draw's uniform in f32: its top 24 bits, `(d >> 29) / 2^24`, exact
/// in [0, 1) on both executors (fn-254).
pub(crate) fn unit(d: Draw) -> f32 {
    (d.0 >> 29) as f32 * (1.0 / 16_777_216.0)
}

/// A draw's uniform as a signed one, `2u − 1`, exact.
pub(crate) fn signed(d: Draw) -> f32 {
    unit(d) * 2.0 - 1.0
}

/// The factor the internode of the node keyed `key` is scaled by at
/// `spread` (`Form::internode_spread`): 1 + spread (2u − 1), u the node's
/// internode draw, so its mean is one; exactly one without spread.
pub(crate) fn internode(key: u64, spread: f64) -> f32 {
    if spread <= 0.0 {
        return 1.0;
    }
    1.0 + spread as f32 * signed(Key(key).child(INTERNODE).draw())
}

/// The key of the node a relay joins its parent at (`keys::JOIN`), under
/// the key of the relay's first phytomer: the drawing's identity for it,
/// the same at any age and any cut (fn-220, fn-244). Not its lineage,
/// which a relay shares with the chain it carries on.
pub fn join_key(first: u64) -> u64 {
    Key(first).child(keys::JOIN).0
}

/// Which count of organs a draw is for (fn-277), under `keys::ORGANS`:
/// two rows of one bearer never draw alike.
pub mod organ {
    pub const CAULINE: u32 = 0;
    pub const FRONDS: u32 = 1;
    pub const SKIRT: u32 = 2;
    pub const BASES: u32 = 3;
    pub const LEAFLETS: u32 = 4;
    pub const SPINES: u32 = 5;
    pub const CLUSTER: u32 = 6;
}

/// The 32-bit finaliser both executors mix keys with (`tree::mix32`).
pub fn mix32(z: u32) -> u32 {
    let z = (z ^ (z >> 16)).wrapping_mul(0x21f0_aaad);
    let z = (z ^ (z >> 15)).wrapping_mul(0x735a_2d97);
    z ^ (z >> 15)
}

/// A count as both executors read it: its floor, and its fraction as a
/// share of 2^16 (as a retention's share is, fn-248).
pub fn split(count: f64) -> (u32, u32) {
    let floor = count.floor();
    (floor as u32, ((count - floor) * 65536.0).floor() as u32)
}

/// The two halves of `split` in one word: the floor above, the share
/// below; what a device table carries.
pub fn word(count: f64) -> u32 {
    let (floor, share) = split(count);
    floor << 16 | share
}

/// The whole organs the bearer keyed `key` bears of a count `word`s as
/// `word` (fn-277): its floor, and one more where the bearer's draw on
/// `row` falls under the share, so the expected count is the count and a
/// count moves the tree by degree. A whole count draws nothing.
pub fn drawn(word: u32, key: u32, row: u32) -> u32 {
    let (floor, share) = (word >> 16, word & 0xffff);
    let stream = mix32(key ^ keys::ORGANS as u32);
    let draw = mix32(stream.wrapping_add(row.wrapping_add(1).wrapping_mul(0x9e37_79b9))) >> 16;
    floor + u32::from(share > 0 && draw < share)
}

/// `drawn` of a count.
pub fn organs(count: f64, key: u32, row: u32) -> u32 {
    drawn(word(count), key, row)
}

/// The key of the placement `ordinal` of the bearer keyed `bearer`.
pub fn placement_key(bearer: u32, ordinal: u32) -> u32 {
    mix32(bearer.wrapping_add(ordinal.wrapping_add(1).wrapping_mul(0x9e37_79b9)))
}

/// An axis's lineage as the 32-bit key its organs draw under.
pub fn lineage_key(lineage: u64) -> u32 {
    lineage as u32 ^ (lineage >> 32) as u32
}

/// The whole leaves the cluster of the axis whose lineage is `lineage`
/// holds at its PA's `leaves` (`PaState::cluster_leaves`): none, it is no
/// short shoot.
pub fn cluster(leaves: f64, lineage: u64) -> u32 {
    organs(leaves, lineage_key(lineage), organ::CLUSTER)
}

/// `internode` as a real factor: what a node's grown wood is counted by
/// (the conversion's `SHORTEST` and the stations' count, fn-212), so the
/// wood is the length its internodes were laid at.
pub fn internode_factor(key: u64, spread: f64) -> f64 {
    f64::from(internode(key, spread))
}

/// The insertion angle, an amount in turns, of the axis whose lineage is
/// `lineage`, its PA's `angle` (radians) at `spread` (radians,
/// `Form::insertion_spread`): angle + spread (2u − 1), u the axis's
/// insertion draw, so its mean is the angle; exactly the angle without
/// spread. Each angle converted once, as the roll's.
pub(crate) fn insertion(lineage: u64, angle: f64, spread: f64) -> f32 {
    let angle = crate::geometry::amount(angle);
    if spread <= 0.0 {
        return angle;
    }
    let u = signed(Key(lineage).child(INSERTION).draw());
    angle + crate::geometry::amount(spread) * u
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

/// The factor the tropism of the axis whose lineage is `lineage` is scaled
/// by at `spread` (`Form::tropism_spread`): 1 + spread (2u − 1), u the
/// axis's tropism draw, so its mean is one; exactly one without spread.
pub(crate) fn tropism(lineage: u64, spread: f64) -> f32 {
    if spread <= 0.0 {
        return 1.0;
    }
    1.0 + spread as f32 * signed(Key(lineage).child(TROPISM).draw())
}

/// Whether the bud place keyed `place` stands at `share` (`Zone::share`):
/// a whole one always, the last a fraction makes by its draw (fn-231,
/// host decision 8), so its buds come in by count as the fraction grows.
pub(crate) fn placed(place: Key, share: f64) -> bool {
    share >= 1.0 || place.child(PLACED).draw().under(share)
}

/// The presence over `window` of the draw that made the bud place
/// keyed `place` stand at `share`: its lead under the share's threshold,
/// as `bud_presence` settles a bud's, so a place grows in from nothing as
/// the fraction passes its draw (fn-231); a whole place's is whole.
pub(crate) fn place_presence(place: Key, share: f64, window: f32) -> f32 {
    if share >= 1.0 {
        return presence::ONE;
    }
    let d = place.child(PLACED).draw();
    presence::of_under(threshold(share), d.0, window)
}

/// The nodes of a zone a unit a lifespan's fraction cut lays (fn-231,
/// host 2026-10-08): `share / of` of its present nodes, the sum of their
/// `presences`, so a node the law only starts to make adds a
/// vanishing share. In index order, each node whose presence the quota
/// still holds is laid whole, at its presence; the next, where its `LAID`
/// draw, under the zone's key `zone`, falls under the share of it left,
/// at its presence times that draw's lead over `window`, so it grows in
/// from nothing; the rest are not laid. The presences are summed in index
/// order and the share is a ratio of ticks, as the device takes them.
pub(crate) fn laid(presences: &mut Vec<f32>, cut: crate::schedule::Cut, zone: Key, window: f32) {
    let share = contract::ticks(cut.share) as f32 / contract::ticks(cut.of) as f32;
    let quota = share * presences.iter().sum::<f32>();
    let mut held = 0.0;
    for (j, &p) in presences.iter().enumerate() {
        if held + p <= quota {
            held += p;
            continue;
        }
        let left = quota - held;
        let last = (left > 0.0).then(|| {
            let d = zone.child(j as u64).child(LAID).draw();
            let t = contract::threshold(left / p);
            let lead = presence::of_under(t, d.0, window);
            (d.0 < t).then(|| presence::times(p, lead))
        });
        presences.truncate(j);
        presences.extend(last.flatten());
        return;
    }
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

/// The presence over `window` of the draw `d` that made a bud of
/// `pa` under `lateral`: its lead past the nearer bound of the PA's run of
/// draws, in log-odds on the bounds' thresholds (host decision 8), over
/// the window. The lesser of the two bounds' presences (a presence rises
/// with its lead). Taken only for a bud that is made.
pub(crate) fn bud_presence(lateral: &[f64], pa: usize, d: Draw, window: f32) -> f32 {
    use presence::{of_over, of_under, ONE};
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
/// law's distribution, and each node's lead into `leads`, in log-odds
/// (`contract.rs`; D18 reads the presences they give): a node of the
/// Poisson law is made as the mean passes its draw (FINE-CONTRACT.md D2).
/// A uniform node's lead is the log of the draw's ratio to its bound,
/// unbounded as the bound nears 0, where the node becomes a sure one, and
/// gentle as it nears 1.
#[cfg(test)]
pub(crate) fn nodes(law: NodeLaw, d: Draw, cap: u32, leads: &mut Vec<f32>) {
    leads.clear();
    walk(law, d, cap, |past| {
        leads.push(match past {
            Past::Uniform(bound) => decide::past(d.0, contract::ln_of(bound)),
            Past::Poisson(bound) => decide::above(d.0, contract::logit_of(bound)),
        })
    });
}

/// `nodes`, each node's presence over `window` (`decide::node_window`) in
/// place of its lead: the same nodes.
pub(crate) fn node_presences(law: NodeLaw, d: Draw, cap: u32, window: f32, out: &mut Vec<f32>) {
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

#[cfg(test)]
mod organ_tests {
    use super::{mix32, organ, organs};

    /// A whole count is its own count at every bearer, and a fraction
    /// gives each bearer the whole either side of it, the share past the
    /// floor bearing one more (fn-277).
    #[test]
    fn a_fraction_of_organs_is_split_at_each_bearer() {
        assert!((0..1000).all(|key| organs(3.0, key, organ::LEAFLETS) == 3));
        let drawn: Vec<u32> = (0..10_000).map(|key| organs(2.3, mix32(key), organ::LEAFLETS)).collect();
        assert!(drawn.iter().all(|&n| n == 2 || n == 3));
        let mean = drawn.iter().sum::<u32>() as f64 / drawn.len() as f64;
        assert!((mean - 2.3).abs() < 0.02, "{mean}");
        // Two rows of one bearer draw apart.
        let apart = (0..1000).filter(|&k| organs(0.5, k, organ::FRONDS) != organs(0.5, k, organ::SKIRT));
        assert!(apart.count() > 300);
    }
}
