//! How long wood keeps its leaves (fn-248): a station keeps its leaf by
//! its wood's retention, clamp(leaf_years - age, 0, 1), on top of the
//! slender wood the canopy bears on. The conversion writes each node's
//! retention beside the tree, as `grown` is written; the expansion counts
//! and places by it. Empty on every tree whose ages all bear for ever.

/// A retention as a 16-bit share: `KEEP` keeps every station, 0 none, and
/// any other `q` the stations whose draw's top 16 bits fall below it.
pub const KEEP: u16 = 0xffff;

/// A node's wood as its leaves read it. A node draws one or more
/// phytomers, oldest first; where their retentions differ, the wood
/// before `older` metres of grown wood takes `keep`'s upper half and the
/// rest its lower half. The boundary stands where the last change falls:
/// a node whose phytomers hold three retentions, which takes three ages
/// straddling a fractional year merged into one node, gives its first
/// two the one before the boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Leafing {
    /// Metres of the node's grown wood before the boundary, scaled as
    /// `Tree::grown` is.
    pub older: f64,
    /// The retentions before and from the boundary, high and low half.
    pub keep: u32,
    /// Drawn within its year (fn-270 R5a): the retentions a year before,
    /// at the same boundary, which hold the stations drawn (a retention
    /// only falls with age, so every station kept now was kept then), and
    /// the share a leaf kept then and not now is drawn at, falling to
    /// nothing across the year. `keep` and 1 at the whole year.
    pub was: u32,
    pub fade: f32,
}

impl Leafing {
    /// Every station of the node keeps its leaf.
    pub const ALL: Self = Self {
        older: 0.0,
        keep: u32::MAX,
        was: u32::MAX,
        fade: 1.0,
    };
    /// The retention of wood `offset` along the node, in the units the
    /// boundary `older` is counted in by the caller.
    pub fn at(self, offset: f64, older: f64) -> u16 {
        half(self.keep, offset >= older)
    }

    /// The retention a year before of wood `offset` along the node.
    pub fn was_at(self, offset: f64, older: f64) -> u16 {
        half(self.was, offset >= older)
    }

    /// The share station `ordinal` of the run keyed `key`, on wood
    /// `offset` along the node, draws its leaf at: whole where it keeps
    /// it now, `fade` where it kept it only a year before; none where it
    /// keeps none.
    pub fn share(self, (offset, older): (f64, f64), key: u32, ordinal: u32) -> Option<f64> {
        if !keeps(self.was_at(offset, older), key, ordinal) {
            return None;
        }
        Some(match keeps(self.at(offset, older), key, ordinal) {
            true => 1.0,
            false => f64::from(self.fade),
        })
    }
}

/// A retention word's half: the low one from the boundary, else the high.
fn half(word: u32, from: bool) -> u16 {
    match from {
        true => word as u16,
        false => (word >> 16) as u16,
    }
}

/// A PA's `leaf_years` as the conversion and the device read it: the
/// first age that keeps less than all its stations, and the share that
/// age keeps. Never: no such age. A share is floored to 16 bits, so a
/// fraction within 2^-16 of a whole year keeps all.
pub fn retention(leaf_years: f64) -> (u32, u16) {
    if leaf_years.is_infinite() {
        return (u32::MAX, KEEP);
    }
    let floor = leaf_years.floor();
    // A fraction below one times 2^16 stays below 2^16 in f64.
    let share = ((leaf_years - floor) * 65536.0).floor();
    (floor as u32, share as u16)
}

/// The retention of a phytomer `age` years old under `retention`.
pub fn kept_share((floor, share): (u32, u16), age: u32) -> u16 {
    match age.cmp(&floor) {
        std::cmp::Ordering::Less => KEEP,
        std::cmp::Ordering::Equal => share,
        std::cmp::Ordering::Greater => 0,
    }
}

/// Whether station `ordinal` of the run whose first node's key is `key`
/// keeps its leaf on wood of retention `q`: a draw under the registry's
/// `RETAIN`, which consumes nothing of the leaves' own stream.
pub fn keeps(q: u16, key: u32, ordinal: u32) -> bool {
    if q == KEEP {
        return true;
    }
    let stream = super::mix32(key ^ telperion_space::lineage_keys::RETAIN as u32);
    let step = ordinal.wrapping_add(1).wrapping_mul(0x9e37_79b9);
    (super::mix32(stream.wrapping_add(step)) >> 16) < u32::from(q)
}
