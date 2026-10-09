//! Every lineage key, in one registry. A key is the step a draw takes
//! from the key it is drawn under; two keys under one parent with one
//! value would draw the same number. Keys named after the reserved top
//! of the range (`u64::MAX - n`) are unique across every parent, so a new
//! one takes the next free `n` here and nowhere else. The GPU's mirror is
//! `telperion-render`'s `fine/lineage.wgsl`, checked against `ALL`.

/// What a key is drawn under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Under {
    /// An axis's lineage.
    Lineage,
    /// A growth unit's key, or the key its decisions are drawn under.
    Unit,
    /// A node's key.
    Node,
    /// A bud place's key, or its sleeping bud's.
    Place,
    /// A floret's key: its bloom's `FLORET` draw and its index (fn-216).
    Floret,
}

/// The draws of one growth unit, under its unit key.
pub const VIABILITY: u64 = 0;
pub const ABORTION: u64 = 1;
/// An apex that has spent its lifespan moving on, under its last unit.
pub const MOVE: u64 = 2;
/// Zone `z` of a growth unit is keyed `ZONE + z`, for `z` below `ZONES`.
pub const ZONE: u64 = 16;
/// A node's bud place `slot` is keyed `PLACE + slot`, for `slot` below
/// `MAX_BUDS`, under its node's key; the node's order is its first
/// place's draw.
pub const PLACE: u64 = 0;
/// A lateral's yearly shedding on its carbon balance, under its lineage
/// and then the cycle.
pub const SHED: u64 = u64::MAX - 1;
/// An axis's continuation into an age, under the lineage that began its
/// chain of ages (`Key::onto`), and its relay draw, under the growth unit
/// it stopped in.
pub const RELAY: u64 = u64::MAX - 2;
/// A lateral's roll about its parent, under its lineage.
pub const ROLL: u64 = u64::MAX - 4;
/// A lateral's share of vigour among its siblings, under its lineage.
pub const DOMINANCE: u64 = u64::MAX - 5;
/// A bud place's sleeping bud, under the place's key, and its waking
/// time, under the sleeping bud's.
pub const DORMANT: u64 = u64::MAX - 6;
/// The stop decisions of a relay of an age that ended, by its count.
pub const ENDED: u64 = u64::MAX - 7;
/// An age's end: whether it moves on, and its relay if it does not.
pub const END: u64 = u64::MAX - 8;
/// An internode's length, under its node's key (fn-222).
pub const INTERNODE: u64 = u64::MAX - 9;
/// The decisions of a failed unit's relay's first unit, which grows the
/// rest of the cycle the unit failed in (fn-199 R1).
pub const FAILED: u64 = u64::MAX - 10;
/// An axis's insertion angle, under its lineage (fn-224).
pub const INSERTION: u64 = u64::MAX - 11;
/// An axis's tropism, under its lineage (fn-224).
pub const TROPISM: u64 = u64::MAX - 12;
/// A node's wander, under its node's key: apart from its places, so its
/// wander and its placement are drawn apart (host, 2026-10-07).
pub const WANDER: u64 = u64::MAX - 13;
/// Whether a bud place a fraction of a node's buds makes stands, under
/// the place's key (fn-231): a making draw at that fraction's chance.
pub const PLACED: u64 = u64::MAX - 14;
/// Whether the node a unit cut by a lifespan's fraction lays at a
/// fraction stands, under the node's key (fn-231).
pub const LAID: u64 = u64::MAX - 15;
/// Whether a bloom flowers, under its node's key (fn-216); clear of the
/// node's places, so a flower and a bud never draw alike.
pub const BLOOM: u64 = 0x0b10_0000;
/// A bloom's inflorescence's roll about its bud's side, under its node's
/// key (fn-216).
pub const BLOOM_ROLL: u64 = 0x0b10_0001;
/// A bloom's florets, keyed `FLORET` and then the floret's index under its
/// node's key (fn-216).
pub const FLORET: u64 = 0x0b10_0002;
/// A floret's size scatter and its opening in the season, under its
/// floret's key (fn-216).
pub const FLORET_SIZE: u64 = 0;
pub const FLORET_OPENING: u64 = 1;
/// A short shoot's cluster, under its tip node's key (fn-223); its
/// leaves draw by their index in it. Clear of the node's places and
/// blooms.
pub const CLUSTER: u64 = 0x0c10_0000;
/// A coarse tree's stand-in answering foliage in a cell of the field,
/// under the fine root's lineage, then the cell (fn-214).
pub const STAND: u64 = u64::MAX - 16;
/// A relay's join on its parent, the node the drawing inserts where it
/// stands, under the key of the relay's first phytomer (fn-244).
pub const JOIN: u64 = u64::MAX - 17;
/// An axis carried on into an age, under its chain's lineage.
pub const CONTINUATION: u64 = u64::MAX;

/// Every key: its name, value and what it is drawn under. `ZONE` and
/// `PLACE` stand for their spans.
pub const ALL: [(&str, u64, Under); 28] = [
    ("VIABILITY", VIABILITY, Under::Unit),
    ("ABORTION", ABORTION, Under::Unit),
    ("MOVE", MOVE, Under::Unit),
    ("ZONE", ZONE, Under::Unit),
    ("PLACE", PLACE, Under::Node),
    ("SHED", SHED, Under::Lineage),
    ("RELAY", RELAY, Under::Unit),
    ("ROLL", ROLL, Under::Lineage),
    ("DOMINANCE", DOMINANCE, Under::Lineage),
    ("DORMANT", DORMANT, Under::Place),
    ("ENDED", ENDED, Under::Unit),
    ("END", END, Under::Lineage),
    ("INTERNODE", INTERNODE, Under::Node),
    ("FAILED", FAILED, Under::Unit),
    ("INSERTION", INSERTION, Under::Lineage),
    ("TROPISM", TROPISM, Under::Lineage),
    ("WANDER", WANDER, Under::Node),
    ("PLACED", PLACED, Under::Place),
    ("LAID", LAID, Under::Node),
    ("BLOOM", BLOOM, Under::Node),
    ("BLOOM_ROLL", BLOOM_ROLL, Under::Node),
    ("FLORET", FLORET, Under::Node),
    ("FLORET_SIZE", FLORET_SIZE, Under::Floret),
    ("FLORET_OPENING", FLORET_OPENING, Under::Floret),
    ("CLUSTER", CLUSTER, Under::Node),
    ("STAND", STAND, Under::Lineage),
    ("JOIN", JOIN, Under::Node),
    ("CONTINUATION", CONTINUATION, Under::Lineage),
];

/// The values key `name` takes: `ZONE`'s and `PLACE`'s spans, every
/// other key's one.
pub fn span(name: &str, value: u64) -> std::ops::Range<u64> {
    let n = match name {
        "ZONE" => crate::species::ZONES as u64,
        "PLACE" => u64::from(crate::species::MAX_BUDS),
        _ => 1,
    };
    value..value + n
}

/// Where the reserved top of the range starts.
pub const TOP: u64 = u64::MAX - 1023;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_under_one_parent_are_distinct() {
        for (i, a) in ALL.iter().enumerate() {
            for b in &ALL[i + 1..] {
                let (x, y) = (span(a.0, a.1), span(b.0, b.1));
                let overlap = x.start < y.end && y.start < x.end;
                assert!(a.2 != b.2 || !overlap, "{} and {} meet", a.0, b.0);
            }
        }
    }

    #[test]
    fn reserved_keys_are_distinct_across_parents() {
        for (i, a) in ALL.iter().enumerate() {
            for b in &ALL[i + 1..] {
                let reserved = a.1 >= TOP && b.1 >= TOP;
                assert!(!reserved || a.1 != b.1, "{} and {} share a value", a.0, b.0);
            }
            assert!(
                a.1 >= TOP || span(a.0, a.1).end < TOP,
                "{} reaches the top",
                a.0
            );
        }
    }
}
