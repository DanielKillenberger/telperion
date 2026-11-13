//! One axis's wood as allotted, beside the wood as laid (fn-364, host): a
//! span laid too short to draw only for its stretch keeps the kind and the
//! leaf-bearing length it would have had laid whole and hands that length
//! to the node it merges into (`Tree::handed`); a node left without a
//! drawn child only by the stretch of what follows it is not terminal for
//! leaves (`Tree::followed`). Wood that would merge laid whole merges as
//! it always has.
use super::join::Wood;
use super::SHORTEST;
use crate::pipeline::foliage::plan::micrometres;
use crate::tree::Handed;
use telperion_space::{Axis, Join};

/// The retentions of the wood merged into the next node, now and a year
/// before (`unfolded`'s `wood` and `was`).
pub(super) type Kept = (Option<(f64, u16, u16)>, Option<(u16, u16)>);

pub(super) struct Hand {
    /// The last place of the axis drawn with every stretch at 1.
    last: Option<usize>,
    /// Leaf-bearing wood allotted since the last span drawn laid whole.
    group: f64,
    /// Whether a stretch shortened any of that wood.
    shortened: bool,
    /// The retentions held before that wood merged in.
    before: Kept,
    /// The radius of the last span handed on since a node was drawn: the
    /// wood the next node starts from, as it would laid whole.
    from: Option<f64>,
    /// Whether any span of the axis was handed on: a node drawn after one
    /// is not its axis's first, as it would not be laid whole.
    pub gave: bool,
    /// Spans handed on, to the next node drawn.
    pub held: Vec<Handed>,
}

impl Hand {
    /// Axis `axis`'s hand, its PA's internode and spread, its wood thinner
    /// than `least` undrawn, `joins` the relays that join it.
    pub fn of(
        axis: &Axis,
        (internode, spread): (f64, f64),
        least: f64,
        joins: &[(usize, Join)],
    ) -> Self {
        let mut group = 0.0;
        let mut last = None;
        for (place, p) in axis.phytomers.iter().enumerate() {
            if p.radius < least {
                break;
            }
            let factor = telperion_space::internode_factor(p.key(), spread);
            group += Wood::of(p, internode, factor).bearing;
            if micrometres(group) >= SHORTEST {
                (last, group) = (Some(place), 0.0);
            }
            let inside = joins.iter().filter(|(_, j)| j.node == place && j.t > 0.0);
            if inside.map(|(_, j)| j.above).fold(p.radius, f64::min) < least {
                break;
            }
        }
        Self {
            last,
            group: 0.0,
            shortened: false,
            before: (None, None),
            from: None,
            gave: false,
            held: Vec::new(),
        }
    }

    /// The retentions as they stand before a draw at `kept`.
    pub fn open(&mut self, kept: Kept) {
        if self.group == 0.0 {
            self.before = kept;
        }
    }

    /// A draw's wood `piece`: whether its group would be drawn laid whole,
    /// and whether a stretch shortened any of it.
    pub fn add(&mut self, piece: Wood) -> (bool, bool) {
        self.group += piece.bearing;
        self.shortened |= piece.laid < piece.bearing;
        (micrometres(self.group) >= SHORTEST, self.shortened)
    }

    /// The radius the next node drawn starts from, where a span handed on
    /// stands before it.
    pub fn from(&self) -> Option<f64> {
        self.from
    }

    /// The group ending at `place`, of radius `radius` and retention
    /// `(q, qw)`, merged only for its stretch: handed on, its wood and its
    /// retentions taken back from what the next node draws.
    pub fn give(
        &mut self,
        place: usize,
        radius: f64,
        (q, qw): (u16, u16),
        own: (&mut f64, &mut Kept),
    ) {
        let pack = |r: u16| u32::from(r) << 16 | u32::from(r);
        self.held.push(Handed {
            node: 0,
            length: self.group,
            radius,
            terminal: self.last.is_none_or(|l| place >= l),
            keep: pack(q),
            was: pack(qw),
        });
        *own.0 -= self.group;
        *own.1 = self.before;
        (self.group, self.shortened) = (0.0, false);
        (self.from, self.gave) = (Some(radius), true);
    }

    /// A node drawn at `place`, numbered `node`: whether wood drawn laid
    /// whole follows it, and the spans handed to it.
    pub fn drawn(&mut self, place: usize, node: u32) -> (bool, impl Iterator<Item = Handed> + '_) {
        (self.group, self.shortened, self.from) = (0.0, false, None);
        let followed = self.last.is_some_and(|l| l > place);
        (
            followed,
            self.held.drain(..).map(move |h| Handed { node, ..h }),
        )
    }
}
