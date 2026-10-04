//! Lineage-keyed randomness (fn-192 R1, R2). Every draw is a hash of the
//! bud's path from the root and of what it decides, never a position in a
//! stream, so adding a branch anywhere moves no other draw. Each draw's lead
//! past the bound it was tested against, in log-odds, is what `presence.rs`
//! grows its element in by.
use crate::species::NodeLaw;

/// The draws of one growth unit, under its unit key.
pub(crate) const VIABILITY: u64 = 0;
pub(crate) const ABORTION: u64 = 1;
/// Zone `z` of a growth unit is keyed `ZONE + z`.
pub(crate) const ZONE: u64 = 16;
/// The draws of one axis, under its lineage, and its relay draw, under
/// the growth unit it stopped in.
pub(crate) const RELAY: u64 = u64::MAX - 2;
/// A lateral's roll about its parent, under its lineage.
pub(crate) const ROLL: u64 = u64::MAX - 4;
/// A lateral's share of vigour among its siblings, under its lineage.
pub(crate) const DOMINANCE: u64 = u64::MAX - 5;
/// A bud place's sleeping bud, under the place's key, and its waking
/// time, under the sleeping bud's.
pub(crate) const DORMANT: u64 = u64::MAX - 6;
pub(crate) const CONTINUATION: u64 = u64::MAX;

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

    /// The draw this key decides, uniform on [0, 1).
    pub fn unit(self) -> f64 {
        (mix(self.0 ^ 0xd1b5_4a32_d192_ed03) >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn logit(p: f64) -> f64 {
    (p / (1.0 - p)).ln()
}

/// How far, in log-odds, a draw `u` lies below the `bound` it must stay
/// under to make its element: none at the crossing, unbounded when the
/// element is certain.
pub(crate) fn below(u: f64, bound: f64) -> f64 {
    if bound >= 1.0 || u <= 0.0 {
        return f64::INFINITY;
    }
    logit(bound) - logit(u)
}

/// How far, in log-odds, a draw `u` lies above the `bound` it must reach.
pub(crate) fn above(u: f64, bound: f64) -> f64 {
    if bound <= 0.0 {
        return f64::INFINITY;
    }
    logit(u) - logit(bound)
}

/// A bud's PA under `lateral` for the draw `u`, with its lead past the
/// nearer bound of the PA's run of draws; none for a bare bud.
pub(crate) fn bud(lateral: &[f64], u: f64) -> Option<(usize, f64)> {
    let mut low = 0.0;
    for (pa, &p) in lateral.iter().enumerate() {
        let high = low + p;
        if p > 0.0 && u < high {
            return Some((pa, above(u, low).min(below(u, high))));
        }
        low = high;
    }
    None
}

/// A zone's node count under `law` for the draw `u`, by the inverse of the
/// law's distribution, and each node's lead into `leads`: a node of the
/// Poisson law is made as the mean passes its draw.
pub(crate) fn nodes(law: NodeLaw, u: f64, cap: u32, leads: &mut Vec<f64>) {
    leads.clear();
    match law {
        NodeLaw::Uniform { min, max } => {
            let count = min + (u * f64::from(max - min + 1)) as u32;
            leads.resize(count.min(max) as usize, f64::INFINITY);
        }
        NodeLaw::Poisson { mean } => {
            // Node k stands while u lies above P(count <= k).
            let mut mass = (-mean).exp();
            let mut cumulative = mass;
            let mut k = 0u32;
            while cumulative <= u && k < cap {
                leads.push(above(u, cumulative));
                k += 1;
                mass *= mean / f64::from(k);
                cumulative += mass;
            }
        }
    }
}
