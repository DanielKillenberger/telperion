//! Lineage-keyed randomness (fn-192 R1, R2). Every draw is a hash of the
//! bud's path from the root and of what it decides, never a position in a
//! stream, so adding a branch anywhere moves no other draw. A draw that a
//! setting crosses makes or unmakes an element at vanishing size: its
//! presence grows in from zero as the setting moves on past the draw.
use crate::species::NodeLaw;

/// How far past a draw a setting must move before the element it made is
/// fully grown, as a share of the draw's room: the narrower of the run of
/// draws that make the element and the distance to certainty on the side
/// the setting comes from. At most twice this share of the elements a
/// setting makes are growing in, and none of a certain one.
pub const GROW_IN: f64 = 0.1;

/// The draws of one growth unit, under its unit key.
pub(crate) const VIABILITY: u64 = 0;
pub(crate) const ABORTION: u64 = 1;
/// Zone `z` of a growth unit is keyed `ZONE + z`.
pub(crate) const ZONE: u64 = 16;
/// The draws of one axis, under its lineage.
pub(crate) const RELAY: u64 = u64::MAX - 2;
pub(crate) const RELAY_BUD: u64 = u64::MAX - 1;
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

/// The presence of an element a setting has passed by `lead`: zero at the
/// crossing, whole once the setting is `GROW_IN` of the room past it. The
/// room is the narrower of the element's run of draws, `width`, and the
/// distance from the draw to certainty, `reach`. A draw with no room is
/// never crossed.
pub(crate) fn grow_in(lead: f64, reach: f64, width: f64) -> f64 {
    let room = reach.min(width);
    if room <= 0.0 {
        return 1.0;
    }
    (lead / (GROW_IN * room)).clamp(0.0, 1.0)
}

/// A bud's PA under `lateral` for the draw `u`, with its presence; none
/// for a bare bud. Each PA holds a run of `u` between its cumulative
/// bounds, and the bud grows in from whichever bound is nearer.
pub(crate) fn bud(lateral: &[f64], u: f64) -> Option<(usize, f64)> {
    let mut low = 0.0;
    for (pa, &p) in lateral.iter().enumerate() {
        let high = low + p;
        if p > 0.0 && u < high {
            let presence = grow_in(u - low, u, p).min(grow_in(high - u, 1.0 - u, p));
            return Some((pa, presence));
        }
        low = high;
    }
    None
}

/// A zone's node count under `law` for the draw `u`, by the inverse of the
/// law's distribution, and each node's presence into `presence`. A node of
/// the Poisson law grows in as the mean passes its draw.
pub(crate) fn nodes(law: NodeLaw, u: f64, cap: u32, presence: &mut Vec<f64>) {
    presence.clear();
    match law {
        NodeLaw::Uniform { min, max } => {
            let count = min + (u * f64::from(max - min + 1)) as u32;
            presence.resize(count.min(max) as usize, 1.0);
        }
        NodeLaw::Poisson { mean } => {
            // Node k stands while u lies above P(count <= k). That bound
            // falls by P(count = k) per unit of mean, so a node grows in
            // over the same share of the mean whatever its rank.
            let mut mass = (-mean).exp();
            let mut below = mass;
            let mut k = 0u32;
            while below <= u && k < cap {
                presence.push(grow_in(u - below, u, mass));
                k += 1;
                mass *= mean / f64::from(k);
                below += mass;
            }
        }
    }
}
