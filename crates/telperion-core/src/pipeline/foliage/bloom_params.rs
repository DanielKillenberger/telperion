//! The flowers' value table (fn-216, option B): what an inflorescence is
//! where the engine's ages say a node flowers. Every row acts by degree and
//! none is read where no age flowers. A blossom, a corymb, a raceme and a
//! catkin differ only in these numbers; their colours are the material's.
use crate::catalogue::{input, Bounds, Site, FLOWERS};

crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct BloomParams in "/bloom" {
        /// Florets on one inflorescence. The fractional part is the last
        /// floret's size, so a walk across counts grows one in by degree.
        pub florets: f64 = "florets" "florets" Bounds::closed(1.0, 64.0) => [Expand] {
            wire: 251,
            check: input(Site::Bloom, 0, "bloom florets"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// Metres of the inflorescence's axis the florets stand along: zero
        /// is a cluster at the node, a few centimetres a raceme or catkin.
        pub rachis: f64 = "rachis" "m" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 252,
            check: input(Site::Bloom, 1, "bloom rachis"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// How the axis turns from the bud's side as it grows: 1 hangs
        /// straight down, -1 stands up as a candle, 0 holds the side.
        pub droop: f64 = "droop" "share" Bounds::closed(-1.0, 1.0) => [Expand] {
            wire: 253,
            check: input(Site::Bloom, 2, "bloom droop"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// Metres of stalk between the axis and each floret.
        pub pedicel: f64 = "pedicel" "m" Bounds::closed(0.0, 0.5) => [Expand] {
            wire: 254,
            check: input(Site::Bloom, 3, "bloom pedicel"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// The degrees a floret's stalk leaves the axis: small packs a
        /// spike, ninety spreads a raceme, more fans a cluster back.
        pub spread: f64 = "spread" "degrees" Bounds::closed(0.0, 180.0) => [Expand] {
            wire: 255,
            check: input(Site::Bloom, 4, "bloom spread"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// How much smaller the floret at the axis's tip is than the one at
        /// its base, as a share.
        pub taper: f64 = "taper" "share" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 256,
            check: input(Site::Bloom, 5, "bloom taper"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// Petals on a floret, turned evenly about it by the count. The
        /// fractional part is the last petal's size (host decision 4), so a
        /// walk across counts changes a flower by degree.
        pub petals: f64 = "petals" "petals" Bounds::closed(1.0, 12.0) => [Plan] {
            wire: 257,
            check: input(Site::Bloom, 6, "bloom petals"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// The degrees a petal opens from the floret's axis: a bud near
        /// zero, an open blossom near ninety.
        pub open: f64 = "open" "degrees" Bounds::closed(0.0, 180.0) => [Plan] {
            wire: 258,
            check: input(Site::Bloom, 7, "bloom open"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// A petal's length in metres.
        pub petal_length: f64 = "petalLength" "m" Bounds::closed(1e-4, 1.0) => [Plan] {
            wire: 259,
            check: input(Site::Bloom, 8, "bloom petal length"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// A petal's greatest width in metres.
        pub petal_width: f64 = "petalWidth" "m" Bounds::closed(1e-4, 1.0) => [Plan] {
            wire: 260,
            check: input(Site::Bloom, 9, "bloom petal width"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// Where a petal is widest, as a share from its base to its tip.
        pub petal_widest: f64 = "petalWidest" "share of the petal"
            Bounds::closed(0.05, 0.95) => [Plan] {
            wire: 261,
            check: input(Site::Bloom, 10, "bloom petal widest point"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// How far a petal's margins lift out of its plane, as a share of
        /// its half-width: a cupped blossom.
        pub petal_cup: f64 = "petalCup" "share of the half-width"
            Bounds::closed(-2.0, 2.0) => [Plan] {
            wire: 262,
            check: input(Site::Bloom, 11, "bloom petal cup"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// How far a floret's size varies from one to the next, as a share.
        pub scatter: f64 = "scatter" "share" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 263,
            check: input(Site::Bloom, 12, "bloom scatter"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// Where in the year the flowering window opens, as a share of the
        /// year from its start.
        pub opens: f64 = "opens" "share of the year" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 264,
            check: input(Site::Bloom, 13, "bloom opens"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// How long the window lasts, as a share of the year. A floret
        /// grows in and shrinks out inside it; zero flowers never.
        pub lasts: f64 = "lasts" "share of the year" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 265,
            check: input(Site::Bloom, 14, "bloom lasts"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
        /// The share of the window the florets' openings are spread over:
        /// zero opens every floret at once.
        pub stagger: f64 = "stagger" "share of the window" Bounds::closed(0.0, 1.0) => [Expand] {
            wire: 266,
            check: input(Site::Bloom, 15, "bloom stagger"),
            applies: "no age flowering",
            dial: FLOWERS,
        },
    }
}

impl Default for BloomParams {
    /// A small five-petalled blossom on a short stalk in spring: dormant
    /// until an age flowers.
    fn default() -> Self {
        Self {
            florets: 1.0,
            rachis: 0.0,
            droop: 0.0,
            pedicel: 0.01,
            spread: 60.0,
            taper: 0.0,
            petals: 5.0,
            open: 70.0,
            petal_length: 0.012,
            petal_width: 0.009,
            petal_widest: 0.6,
            petal_cup: 0.2,
            scatter: 0.1,
            opens: 0.3,
            lasts: 0.1,
            stagger: 0.5,
        }
    }
}
