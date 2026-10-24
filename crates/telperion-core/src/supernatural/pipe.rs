//! The supernatural group's pipes (fn-248): the girth each age's
//! phytomer adds to the wood that bears it, overridden by age as the
//! natural `/space/ages/<k>/form/pipe` rows state it.
use crate::catalogue::{input, Bounds, Site};

/// An override has no value of its own to step: it is unset on every
/// natural species, and its natural row is the one tuned.
const OVERRIDE: crate::catalogue::Dial = crate::catalogue::Dial::Derived;

crate::catalogue::rows! {
    /// Each age's pipe a phytomer adds, which the group overrides.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SupernaturalPipe in "/supernatural/pipe" {
        /// `/space/ages/0/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age0: Option<f64> = "age0" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 348,
            check: input(Site::Supernatural, 0, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/1/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age1: Option<f64> = "age1" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 349,
            check: input(Site::Supernatural, 1, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/2/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age2: Option<f64> = "age2" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 350,
            check: input(Site::Supernatural, 2, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/3/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age3: Option<f64> = "age3" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 351,
            check: input(Site::Supernatural, 3, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/4/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age4: Option<f64> = "age4" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 352,
            check: input(Site::Supernatural, 4, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/5/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age5: Option<f64> = "age5" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 353,
            check: input(Site::Supernatural, 5, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/6/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age6: Option<f64> = "age6" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 354,
            check: input(Site::Supernatural, 6, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/7/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age7: Option<f64> = "age7" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 355,
            check: input(Site::Supernatural, 7, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/8/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age8: Option<f64> = "age8" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 356,
            check: input(Site::Supernatural, 8, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/9/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age9: Option<f64> = "age9" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 357,
            check: input(Site::Supernatural, 9, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/10/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age10: Option<f64> = "age10" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 358,
            check: input(Site::Supernatural, 10, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/11/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age11: Option<f64> = "age11" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 359,
            check: input(Site::Supernatural, 11, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/12/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age12: Option<f64> = "age12" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 360,
            check: input(Site::Supernatural, 12, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/13/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age13: Option<f64> = "age13" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 361,
            check: input(Site::Supernatural, 13, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/space/ages/14/form/pipe` while the group is on; unset, the natural
        /// pipe a phytomer adds.
        pub age14: Option<f64> = "age14" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 362,
            check: input(Site::Supernatural, 14, "supernatural pipe"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
    }
}

impl SupernaturalPipe {
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
