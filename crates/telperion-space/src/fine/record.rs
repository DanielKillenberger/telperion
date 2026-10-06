//! The canonical fine-root record (fn-209 F1, Astra's change 1;
//! FINE-CONTRACT.md, "The record"): everything the expansion grows a fine
//! root from, and nothing else. The structure above the cut writes it
//! once, from its bud and its laid bearer; no field is read back.
//!
//! - **The bud** as its bearer made it (P2's `Bud`): its first axis's
//!   lineage, PA, birth, origin and vigour as made; its birth draws'
//!   presences; the lineage its chain of ages began from (`root`); its
//!   horizon, the most past its subtree's last growth that it or a lateral
//!   or relay it stands on keeps it, which is the ancestors' termination
//!   as shedding reads it; and a woken bud's dormant and relay history
//!   (the sleeper, the time it aged asleep, the presence that carried its
//!   bearer to its waking, its cycle, and the share of its size its
//!   bearer kept).
//! - **The ancestors' survival**: where its bearer was kept on its borne
//!   buds' chances alone, the chance its bearer's other buds had all died
//!   (P2's shedding context, as `-log2` in Q32), on which its own keeping is conditioned.
//! - **The attachment**: its bearer's PA and the bearer's laid frame at
//!   the node, before bending and as laid: the node's phytomer (its tip,
//!   frame, rank, scale and size, which the root's base scale is read
//!   from as its bearer reads it) and the point below it.
//! - **The committed boundary** its bearer read (option 4): its base
//!   section handed to the bearer at the bearer's exponent, the retained
//!   pipes' share included (the retained-pipe transfer), its mass and that
//!   mass's first moment along its heading, and its base radius. The
//!   expansion fits the root's girth to that radius; its bearer never
//!   reads what the root realizes.
//! - **The predictor**: its expected kept phytomers, which put it below
//!   the cut.
//! - **Its placement** (host decision 3): whether its conservative extent
//!   bound from its attachment can reach the ground's landing zone, which
//!   keeps it on the CPU reference executor. Work placement only: the
//!   botany is the same on either executor.
//!
//! Light is named and not carried: a species any bud of which reads light
//! needs DESIGN-LIGHT.md's history-dependent aggregate state, which is
//! F2b's (fn-211), and is refused (`grow/alone.rs`, `rules`).
use crate::grow::alone::Bud;
use crate::grow::Record;
use crate::structure::{Axis, Origin, Phytomer, Vec3};

/// The bearer's laid frame at the bud's node: its phytomer there and the
/// point below it (the phytomer before it, or the bearer's base).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Frame {
    pub below: Vec3,
    pub at: Phytomer,
}

/// Where a fine root stands: its bearer's index above the cut, its PA,
/// lineage and birth, the node, and the frame before bending and as laid.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Attachment {
    pub bearer: usize,
    pub pa: usize,
    pub lineage: u64,
    pub birth: u32,
    pub node: usize,
    pub unbent: Frame,
    pub bent: Frame,
}

/// What a fine root's bearer committed to (option 4).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Committed {
    /// Its base section handed to its bearer at the bearer's exponent,
    /// the bearer's retained share of its pipes included.
    pub section: f64,
    /// Its mass, and that mass's moment along its heading.
    pub mass: f64,
    pub moment: f64,
    /// Its base radius, which its girth is fitted to.
    pub radius: f64,
}

/// A fine root: a bud whose expected kept subtree is at most the cut.
#[derive(Debug, Clone)]
pub struct FineRoot {
    pub(crate) bud: Bud,
    pub(crate) context: Option<i64>,
    pub(crate) record: Record,
    pub(crate) attachment: Attachment,
    pub committed: Committed,
    /// Its expected kept phytomers.
    pub predicted: f64,
    /// Whether its wood can come near the ground (host decision 3): then
    /// it is grown on the CPU reference executor, never on the GPU.
    pub grounded: bool,
}

impl FineRoot {
    /// Whether its bud slept and woke: its dormant history rides with it.
    pub fn woken(&self) -> bool {
        self.bud.woken()
    }
}

impl Attachment {
    /// The bearer as the root's tree holds it: axis 0, the node's phytomer
    /// alone on it standing on the point below; before bending and as
    /// laid.
    pub fn stubs(&self) -> (Axis, Axis) {
        let stub = |frame: &Frame| Axis {
            lineage: self.lineage,
            pa: self.pa,
            birth: self.birth,
            origin: Origin::Seed,
            vigour: 1.0,
            blend: 1.0,
            apex_end: None,
            base: frame.below,
            heading: Vec3::default(),
            side: Vec3::default(),
            phytomers: vec![frame.at],
            units: Vec::new(),
            alive: 0.0,
            rank: 0.0,
            sleep: 0.0,
        };
        (stub(&self.unbent), stub(&self.bent))
    }
}

impl Frame {
    /// The frame of laid `bearer` at `node`.
    pub fn of(bearer: &Axis, node: usize) -> Self {
        let below = node
            .checked_sub(1)
            .map_or(bearer.base, |n| bearer.phytomers[n].tip);
        Self {
            below,
            at: bearer.phytomers[node],
        }
    }
}
