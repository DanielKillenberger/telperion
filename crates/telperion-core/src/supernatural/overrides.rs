//! The supernatural group's overrides: each stands in for the natural row
//! it mirrors while the group is on, and leaves it as it is while unset.
use crate::catalogue::{input, Bounds, Dial, Site};

/// An override has no value of its own to step: it is unset on every
/// natural species, and its natural row is the one tuned.
const OVERRIDE: Dial = Dial::Derived;

crate::catalogue::rows! {
    /// The leaf's colours the group overrides.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SupernaturalMaterial in "/supernatural/material" {
        /// `/material/leafFrontRed` while the group is on; unset, the natural row.
        pub leaf_front_red: Option<f64> = "leafFrontRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 281,
            check: input(Site::Supernatural, 0, "supernatural material leafFrontRed"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/leafFrontGreen` while the group is on; unset, the natural row.
        pub leaf_front_green: Option<f64> = "leafFrontGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 282,
            check: input(Site::Supernatural, 1, "supernatural material leafFrontGreen"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/leafFrontBlue` while the group is on; unset, the natural row.
        pub leaf_front_blue: Option<f64> = "leafFrontBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 283,
            check: input(Site::Supernatural, 2, "supernatural material leafFrontBlue"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/leafBackRed` while the group is on; unset, the natural row.
        pub leaf_back_red: Option<f64> = "leafBackRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 284,
            check: input(Site::Supernatural, 3, "supernatural material leafBackRed"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/leafBackGreen` while the group is on; unset, the natural row.
        pub leaf_back_green: Option<f64> = "leafBackGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 285,
            check: input(Site::Supernatural, 4, "supernatural material leafBackGreen"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/leafBackBlue` while the group is on; unset, the natural row.
        pub leaf_back_blue: Option<f64> = "leafBackBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 286,
            check: input(Site::Supernatural, 5, "supernatural material leafBackBlue"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/marginRed` while the group is on; unset, the natural row.
        pub margin_red: Option<f64> = "marginRed" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 287,
            check: input(Site::Supernatural, 6, "supernatural material marginRed"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/marginGreen` while the group is on; unset, the natural row.
        pub margin_green: Option<f64> = "marginGreen" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 288,
            check: input(Site::Supernatural, 7, "supernatural material marginGreen"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/material/marginBlue` while the group is on; unset, the natural row.
        pub margin_blue: Option<f64> = "marginBlue" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 289,
            check: input(Site::Supernatural, 8, "supernatural material marginBlue"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
    }
}

impl SupernaturalMaterial {
    /// Every override unset.
    pub const NONE: Self = Self {
        leaf_front_red: None,
        leaf_front_green: None,
        leaf_front_blue: None,
        leaf_back_red: None,
        leaf_back_green: None,
        leaf_back_blue: None,
        margin_red: None,
        margin_green: None,
        margin_blue: None,
    };
}

crate::catalogue::rows! {
    /// The flowers the group overrides.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct SupernaturalBloom in "/supernatural/bloom" {
        /// `/bloom/florets` while the group is on; unset, the natural row.
        pub florets: Option<f64> = "florets" "florets" Bounds::closed(1.0, 64.0) => [Expand] {
            wire: 290,
            check: input(Site::Supernatural, 0, "supernatural bloom florets"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/rachis` while the group is on; unset, the natural row.
        pub rachis: Option<f64> = "rachis" "m" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 291,
            check: input(Site::Supernatural, 1, "supernatural bloom rachis"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/droop` while the group is on; unset, the natural row.
        pub droop: Option<f64> = "droop" "share" Bounds::closed(-1.0, 1.0) => [Expand] {
            wire: 292,
            check: input(Site::Supernatural, 2, "supernatural bloom droop"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/pedicel` while the group is on; unset, the natural row.
        pub pedicel: Option<f64> = "pedicel" "m" Bounds::closed(0.0, 0.5) => [Expand] {
            wire: 293,
            check: input(Site::Supernatural, 3, "supernatural bloom pedicel"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/spread` while the group is on; unset, the natural row.
        pub spread: Option<f64> = "spread" "degrees" Bounds::closed(0.0, 180.0) => [Expand] {
            wire: 294,
            check: input(Site::Supernatural, 4, "supernatural bloom spread"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/taper` while the group is on; unset, the natural row.
        pub taper: Option<f64> = "taper" "share" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 295,
            check: input(Site::Supernatural, 5, "supernatural bloom taper"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/petals` while the group is on; unset, the natural row.
        pub petals: Option<f64> = "petals" "petals" Bounds::closed(1.0, 12.0) => [Plan] {
            wire: 296,
            check: input(Site::Supernatural, 6, "supernatural bloom petals"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/open` while the group is on; unset, the natural row.
        pub open: Option<f64> = "open" "degrees" Bounds::closed(0.0, 180.0) => [Plan] {
            wire: 297,
            check: input(Site::Supernatural, 7, "supernatural bloom open"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/petalLength` while the group is on; unset, the natural row.
        pub petal_length: Option<f64> = "petalLength" "m" Bounds::closed(1e-4, 1.0) => [Plan] {
            wire: 298,
            check: input(Site::Supernatural, 8, "supernatural bloom petalLength"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/petalWidth` while the group is on; unset, the natural row.
        pub petal_width: Option<f64> = "petalWidth" "m" Bounds::closed(1e-4, 1.0) => [Plan] {
            wire: 299,
            check: input(Site::Supernatural, 9, "supernatural bloom petalWidth"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/petalWidest` while the group is on; unset, the natural row.
        pub petal_widest: Option<f64> = "petalWidest" "share of the petal" Bounds::closed(0.05, 0.95) => [Plan] {
            wire: 300,
            check: input(Site::Supernatural, 10, "supernatural bloom petalWidest"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/petalCup` while the group is on; unset, the natural row.
        pub petal_cup: Option<f64> = "petalCup" "share of the half-width" Bounds::closed(-2.0, 2.0) => [Plan] {
            wire: 301,
            check: input(Site::Supernatural, 11, "supernatural bloom petalCup"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/petalCurl` while the group is on; unset, the natural row.
        pub petal_curl: Option<f64> = "petalCurl" "share of the length" Bounds::closed(-2.0, 2.0) => [Plan] {
            wire: 322,
            check: input(Site::Supernatural, 16, "supernatural bloom petalCurl"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/stamens` while the group is on; unset, the natural row.
        pub stamens: Option<f64> = "stamens" "stamens" Bounds::closed(0.0, 64.0) => [Plan] {
            wire: 330,
            check: input(Site::Supernatural, 17, "supernatural bloom stamens"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/stamenLength` while the group is on; unset, the natural row.
        pub stamen_length: Option<f64> = "stamenLength" "m" Bounds::closed(1e-4, 0.1) => [Plan] {
            wire: 331,
            check: input(Site::Supernatural, 18, "supernatural bloom stamenLength"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/stamenWidth` while the group is on; unset, the natural row.
        pub stamen_width: Option<f64> = "stamenWidth" "m" Bounds::closed(1e-5, 0.01) => [Plan] {
            wire: 332,
            check: input(Site::Supernatural, 19, "supernatural bloom stamenWidth"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/scatter` while the group is on; unset, the natural row.
        pub scatter: Option<f64> = "scatter" "share" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 302,
            check: input(Site::Supernatural, 12, "supernatural bloom scatter"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/opens` while the group is on; unset, the natural row.
        pub opens: Option<f64> = "opens" "share of the year" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 303,
            check: input(Site::Supernatural, 13, "supernatural bloom opens"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/lasts` while the group is on; unset, the natural row.
        pub lasts: Option<f64> = "lasts" "share of the year" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 304,
            check: input(Site::Supernatural, 14, "supernatural bloom lasts"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
        /// `/bloom/stagger` while the group is on; unset, the natural row.
        pub stagger: Option<f64> = "stagger" "share of the window" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 305,
            check: input(Site::Supernatural, 15, "supernatural bloom stagger"),
            applies: "`/supernatural/enabled` off",
            blend: crate::catalogue::Blend::Coupled,
            dial: OVERRIDE,
        },
    }
}

impl SupernaturalBloom {
    /// Every override unset.
    pub const NONE: Self = Self {
        florets: None,
        rachis: None,
        droop: None,
        pedicel: None,
        spread: None,
        taper: None,
        petals: None,
        open: None,
        petal_length: None,
        petal_width: None,
        petal_widest: None,
        petal_cup: None,
        petal_curl: None,
        stamens: None,
        stamen_length: None,
        stamen_width: None,
        scatter: None,
        opens: None,
        lasts: None,
        stagger: None,
    };
}
