//! The contract probe's selectors (`probe.wgsl`'s `cfg.y`), in one
//! registry: each names the functions one probe case runs. The compiler
//! refuses a value given twice; `probe.wgsl`'s `P_*` constants mirror
//! `ALL`, and `mirror_tests.rs` checks the copy.

/// A probe selector; its discriminant is the value `cfg.y` carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Selector {
    ChildDraw = 0,
    Log2 = 1,
    Exp2 = 2,
    LnRatio = 3,
    MulPresence = 4,
    Survival = 5,
    Abortion = 6,
    Moved = 7,
    Past = 8,
    Wake = 9,
    RelayNode = 10,
    And = 11,
    UnderOver = 12,
    Times = 13,
    Carry = 14,
    Lead = 15,
    Internode = 16,
    Angles = 17,
    FineSurvival = 18,
    FineAbortion = 19,
    Q32Float = 20,
}

use Selector as S;

/// Every selector, in order: its WGSL name and the functions it runs.
pub(super) const ALL: [(Selector, &str, &str); 21] = [
    (S::ChildDraw, "P_CHILD_DRAW", "child, draw"),
    (S::Log2, "P_LOG2", "log2, exp2_neg"),
    (S::Exp2, "P_EXP2", "exp2, neg_log2"),
    (S::LnRatio, "P_LN_RATIO", "ln_ratio, exp"),
    (S::MulPresence, "P_MUL_PRESENCE", "mul, presence"),
    (S::Survival, "P_SURVIVAL", "survival_q, scaled"),
    (S::Abortion, "P_ABORTION", "abortion_q, lived_q"),
    (S::Moved, "P_MOVED", "moved_q"),
    (S::Past, "P_PAST", "past_q, above_q"),
    (S::Wake, "P_WAKE", "wake_q, kept"),
    (S::RelayNode, "P_RELAY_NODE", "relay_node_q"),
    (S::And, "P_AND", "and"),
    (S::UnderOver, "P_UNDER_OVER", "under_q, over_q"),
    (S::Times, "P_TIMES", "times_q, window_q, wood_at_q"),
    (S::Carry, "P_CARRY", "carry_q, fade_q"),
    (S::Lead, "P_LEAD", "lead_q"),
    (S::Internode, "P_INTERNODE", "internode_draw"),
    (S::Angles, "P_ANGLES", "insertion_draw, tropism_draw"),
    (
        S::FineSurvival,
        "P_FINE_SURVIVAL",
        "survival_fine_q, lived_fine_q",
    ),
    (S::FineAbortion, "P_FINE_ABORTION", "abortion_fine_q"),
    (S::Q32Float, "P_Q32_FLOAT", "q32_f"),
];

impl Selector {
    /// The functions it runs.
    pub(super) fn name(self) -> &'static str {
        ALL[self as usize].2
    }
}
