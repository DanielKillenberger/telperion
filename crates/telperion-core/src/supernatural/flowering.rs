//! The supernatural group's flowering: the share of each age's nodes that
//! flower, overridden by age as the natural `/space/ages/<k>/flowering`
//! rows state it.
use crate::catalogue::{input, Bounds, Site};

/// An override has no value of its own to step: it is unset on every
/// natural species, and its natural row is the one tuned.
const OVERRIDE: crate::catalogue::Dial = crate::catalogue::Dial::Derived;

crate::catalogue::rows! {
    /// The share of each age's nodes that flower, which the group overrides.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SupernaturalFlowering in "/supernatural/flowering" {
        /// `/space/ages/0/flowering` while the group is on; unset, the natural
        /// share.
        pub age0: Option<f64> = "age0" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 306,
            check: input(Site::Supernatural, 0, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/1/flowering` while the group is on; unset, the natural
        /// share.
        pub age1: Option<f64> = "age1" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 307,
            check: input(Site::Supernatural, 1, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/2/flowering` while the group is on; unset, the natural
        /// share.
        pub age2: Option<f64> = "age2" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 308,
            check: input(Site::Supernatural, 2, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/3/flowering` while the group is on; unset, the natural
        /// share.
        pub age3: Option<f64> = "age3" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 309,
            check: input(Site::Supernatural, 3, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/4/flowering` while the group is on; unset, the natural
        /// share.
        pub age4: Option<f64> = "age4" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 310,
            check: input(Site::Supernatural, 4, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/5/flowering` while the group is on; unset, the natural
        /// share.
        pub age5: Option<f64> = "age5" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 311,
            check: input(Site::Supernatural, 5, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/6/flowering` while the group is on; unset, the natural
        /// share.
        pub age6: Option<f64> = "age6" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 312,
            check: input(Site::Supernatural, 6, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/7/flowering` while the group is on; unset, the natural
        /// share.
        pub age7: Option<f64> = "age7" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 313,
            check: input(Site::Supernatural, 7, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/8/flowering` while the group is on; unset, the natural
        /// share.
        pub age8: Option<f64> = "age8" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 314,
            check: input(Site::Supernatural, 8, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/9/flowering` while the group is on; unset, the natural
        /// share.
        pub age9: Option<f64> = "age9" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 315,
            check: input(Site::Supernatural, 9, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/10/flowering` while the group is on; unset, the natural
        /// share.
        pub age10: Option<f64> = "age10" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 316,
            check: input(Site::Supernatural, 10, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/11/flowering` while the group is on; unset, the natural
        /// share.
        pub age11: Option<f64> = "age11" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 317,
            check: input(Site::Supernatural, 11, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/12/flowering` while the group is on; unset, the natural
        /// share.
        pub age12: Option<f64> = "age12" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 318,
            check: input(Site::Supernatural, 12, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/13/flowering` while the group is on; unset, the natural
        /// share.
        pub age13: Option<f64> = "age13" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 319,
            check: input(Site::Supernatural, 13, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/14/flowering` while the group is on; unset, the natural
        /// share.
        pub age14: Option<f64> = "age14" "share of the age's nodes" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 320,
            check: input(Site::Supernatural, 14, "supernatural flowering"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
    }
}

impl SupernaturalFlowering {
    /// The ages the engine's reference axis holds, one override each.
    pub const AGES: usize = telperion_space::AGES;
    /// Every override unset.
    pub const NONE: Self = Self {
        age0: None,
        age1: None,
        age2: None,
        age3: None,
        age4: None,
        age5: None,
        age6: None,
        age7: None,
        age8: None,
        age9: None,
        age10: None,
        age11: None,
        age12: None,
        age13: None,
        age14: None,
    };

    /// Age `k`'s override.
    pub fn get(&self, k: usize) -> Option<f64> {
        (Self::ROWS[k].get)(self).number()
    }

    /// Sets age `k`'s override.
    pub fn set(&mut self, k: usize, share: f64) {
        (Self::ROWS[k].set)(self).put(share);
    }

    /// Each age and its override.
    pub fn shares(&self) -> impl Iterator<Item = (usize, Option<f64>)> + '_ {
        (0..Self::AGES).map(|k| (k, self.get(k)))
    }
}
