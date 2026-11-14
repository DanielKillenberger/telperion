//! The supernatural group's internodes (fn-248): each age's internode
//! length, overridden by age as the natural `/space/ages/<k>/internode`
//! rows state it, so the Two Trees grow large through their coarse ages
//! while their fine ages stay the natural species'.
use crate::catalogue::{input, Bounds, Site};

/// An override has no value of its own to step: it is unset on every
/// natural species, and its natural row is the one tuned.
const OVERRIDE: crate::catalogue::Dial = crate::catalogue::Dial::Derived;

crate::catalogue::rows! {
    /// Each age's internode length, which the group overrides.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SupernaturalInternode in "/supernatural/internode" {
        /// `/space/ages/0/internode` while the group is on; unset, the natural
        /// internode length.
        pub age0: Option<f64> = "age0" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 333,
            check: input(Site::Supernatural, 0, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/1/internode` while the group is on; unset, the natural
        /// internode length.
        pub age1: Option<f64> = "age1" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 334,
            check: input(Site::Supernatural, 1, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/2/internode` while the group is on; unset, the natural
        /// internode length.
        pub age2: Option<f64> = "age2" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 335,
            check: input(Site::Supernatural, 2, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/3/internode` while the group is on; unset, the natural
        /// internode length.
        pub age3: Option<f64> = "age3" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 336,
            check: input(Site::Supernatural, 3, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/4/internode` while the group is on; unset, the natural
        /// internode length.
        pub age4: Option<f64> = "age4" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 337,
            check: input(Site::Supernatural, 4, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/5/internode` while the group is on; unset, the natural
        /// internode length.
        pub age5: Option<f64> = "age5" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 338,
            check: input(Site::Supernatural, 5, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/6/internode` while the group is on; unset, the natural
        /// internode length.
        pub age6: Option<f64> = "age6" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 339,
            check: input(Site::Supernatural, 6, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/7/internode` while the group is on; unset, the natural
        /// internode length.
        pub age7: Option<f64> = "age7" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 340,
            check: input(Site::Supernatural, 7, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/8/internode` while the group is on; unset, the natural
        /// internode length.
        pub age8: Option<f64> = "age8" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 341,
            check: input(Site::Supernatural, 8, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/9/internode` while the group is on; unset, the natural
        /// internode length.
        pub age9: Option<f64> = "age9" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 342,
            check: input(Site::Supernatural, 9, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/10/internode` while the group is on; unset, the natural
        /// internode length.
        pub age10: Option<f64> = "age10" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 343,
            check: input(Site::Supernatural, 10, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/11/internode` while the group is on; unset, the natural
        /// internode length.
        pub age11: Option<f64> = "age11" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 344,
            check: input(Site::Supernatural, 11, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/12/internode` while the group is on; unset, the natural
        /// internode length.
        pub age12: Option<f64> = "age12" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 345,
            check: input(Site::Supernatural, 12, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/13/internode` while the group is on; unset, the natural
        /// internode length.
        pub age13: Option<f64> = "age13" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 346,
            check: input(Site::Supernatural, 13, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/14/internode` while the group is on; unset, the natural
        /// internode length.
        pub age14: Option<f64> = "age14" "m" Bounds::closed(1e-6, 100.0) => [Grow] {
            wire: 347,
            check: input(Site::Supernatural, 14, "supernatural internode"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
    }
}

impl SupernaturalInternode {
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
    pub fn set(&mut self, k: usize, value: f64) {
        (Self::ROWS[k].set)(self).put(value);
    }

    /// Each age and its override.
    pub fn values(&self) -> impl Iterator<Item = (usize, Option<f64>)> + '_ {
        (0..telperion_space::AGES).map(|k| (k, self.get(k)))
    }
}
