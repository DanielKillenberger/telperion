//! The supernatural group's leaf size (fn-248): the canopy's `size` row
//! overridden while the group is on, so a legendary tree's larger crown
//! of ordinary twigs reaches a beech's leaf area index with fewer, larger
//! leaves.
use crate::catalogue::{input, Bounds, Dial, Site};

/// An override has no value of its own to step: it is unset on every
/// natural species, and its natural row is the one tuned.
const OVERRIDE: Dial = Dial::Derived;

crate::catalogue::rows! {
    /// The canopy rows the group overrides.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SupernaturalCanopy in "/supernatural/canopy" {
        /// `/canopy/size` while the group is on; unset, the natural row.
        pub size: Option<f64> = "size" "multiple of the element"
            Bounds::closed(0.0, 1000.0) => [Plan, Expand] {
            wire: 363,
            check: input(Site::Supernatural, 0, "supernatural canopy size"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
    }
}

impl SupernaturalCanopy {
    /// Every override unset.
    pub const NONE: Self = Self { size: None };
}
