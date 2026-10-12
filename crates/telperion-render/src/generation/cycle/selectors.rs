//! The contract probe's selectors (`probe.wgsl`'s `cfg.y`), in one
//! registry: each names the functions one probe case runs. The compiler
//! refuses a value given twice; `probe.wgsl`'s `P_*` constants mirror
//! `ALL`, and `mirror_tests.rs` checks the copy.

/// A probe selector; its discriminant is the value `cfg.y` carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Selector {
    ChildDraw = 0,
    Internode = 1,
    Angles = 2,
    Convert = 3,
    Threshold = 4,
    Logs = 5,
    Logit = 6,
    Survival = 7,
    Abortion = 8,
    Lived = 9,
    Moved = 10,
    Past = 11,
    Wake = 12,
    Lead = 13,
    Window = 14,
    Carry = 15,
    RelayNode = 16,
}

use Selector as S;

/// Every selector, in order: its WGSL name and the functions it runs.
pub(super) const ALL: [(Selector, &str, &str); 17] = [
    (S::ChildDraw, "P_CHILD_DRAW", "child, draw"),
    (S::Internode, "P_INTERNODE", "internode_draw"),
    (S::Angles, "P_ANGLES", "insertion_draw, tropism_draw"),
    (S::Convert, "P_CONVERT", "to_f32, floor64"),
    (S::Threshold, "P_THRESHOLD", "threshold, its floor"),
    (S::Logs, "P_LOGS", "neg_log2, prob"),
    (S::Logit, "P_LOGIT", "logit, logit53"),
    (S::Survival, "P_SURVIVAL", "survival_fine, scaled"),
    (S::Abortion, "P_ABORTION", "abortion_fine, lived_ticks"),
    (S::Lived, "P_LIVED", "lived_fine, draw_log2"),
    (S::Moved, "P_MOVED", "moved"),
    (S::Past, "P_PAST", "past, above"),
    (S::Wake, "P_WAKE", "wake_ticks"),
    (S::Lead, "P_LEAD", "lead"),
    (S::Window, "P_WINDOW", "wood_at, window, presence"),
    (S::Carry, "P_CARRY", "carry, fade"),
    (S::RelayNode, "P_RELAY_NODE", "relay_node"),
];

impl Selector {
    /// The functions it runs.
    pub(super) fn name(self) -> &'static str {
        ALL[self as usize].2
    }
}
