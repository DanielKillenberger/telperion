//! The canopy's value table: every row declared once, with its catalogue entry.
use crate::catalogue::{bounded, input, tuned, value, Blend, Bounds, Dial, Growth, Site};

crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct CanopyParams in "/canopy" {
        /// Wood at or below this fraction of the root radius bears foliage of its
        /// own, beside whatever the twig layer marks. Zero leaves the twigs alone
        /// with it; without a twig layer it is what selects the terminal shoots.
        pub shoot_radius: f64 = "shootRadius" "share of root radius"
            Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 88,
            check: input(Site::Canopy, 0, "shoot radius"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            growth: Growth::Differs("measured against the root node, and capped by \
                `twig.bearingDiameter`"),
            note: "Above zero the leaf box uses the trunk radius.",
            dial: bounded("canopy_shoot_radius", "the share of the root radius at or below which \
                wood bears foliage of its own; at zero no wood beyond the twig layer bears \
                foliage of its own, and any rise starts clothing it",
                [0.005, 0.01]).span([0.0, 0.0375]),
        },
        /// Metres between leaves along a shoot, as a share of the tree's
        /// height. Raising it spreads the leaves further apart, so the crown
        /// carries fewer of them.
        pub spacing: f64 = "spacing" "share of height" Bounds::closed(0.001, 1e6) => [] {
            wire: 89,
            check: input(Site::Canopy, 1, "foliage spacing"),
            applies: "only a placement with no twig table reads it, which no production caller \
                passes",
            deprecated: true,
        },
        /// The degrees each successive leaf is turned around its shoot.
        /// Raising it turns the next leaf further round, so the leaves spiral
        /// differently.
        pub divergence: f64 = "divergence" "degrees" Bounds::closed(-1e9, 1e9) => [Expand] {
            wire: 90,
            check: input(Site::Canopy, 2, "divergence"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            note: "Past a phase-error bound, station preparation leaves the GPU for the CPU.",
            blend: Blend::Degrees,
            dial: bounded("foliage_divergence", "the degrees each successive leaf is turned \
                around its shoot", [15.0, 30.0]).span([116.262, 201.246]),
        },
        /// How many extra leaves are gathered at the end of a shoot that has
        /// no twig layer. Raising it packs a denser tuft at the tip.
        pub clump: u32 = "clump" "leaves" Bounds::closed(0.0, 64.0) => [] {
            wire: 91,
            check: input(Site::Canopy, 14, "foliage clump"),
            applies: "only a placement with no twig table reads it, which no production caller \
                passes",
            blend: Blend::Count,
            deprecated: true,
        },
        /// How far back from the tip that tuft is scattered, as a share of the
        /// shoot's length. Raising it spreads the tuft further down the shoot.
        pub clump_span: f64 = "clumpSpan" "share of the shoot"
            Bounds::closed(0.0, 1.0) => [] {
            wire: 92,
            check: input(Site::Canopy, 3, "clump span"),
            applies: "only a placement with no twig table reads it, which no production caller \
                passes",
            deprecated: true,
        },
        /// How far a leaf turns away from the trunk. Raising it points the
        /// leaves outward, away from the tree's axis.
        pub outward: f64 = "outward" "-" Bounds::closed(-1.0, 1.0) => [Expand] {
            wire: 93,
            check: input(Site::Canopy, 4, "outward"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            note: "Rosette fronds ignore it.",
            dial: bounded("leaf_outward", "how far a leaf turns away from the trunk", [0.25, 0.5]),
        },
        /// How far a leaf turns toward the sky. Raising it tips the leaves up.
        pub upward: f64 = "upward" "-" Bounds::closed(-1.0, 1.0) => [Expand] {
            wire: 94,
            check: input(Site::Canopy, 5, "upward"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            note: "Rosette fronds ignore it.",
            dial: bounded("leaf_upward", "how far a leaf turns toward the sky", [0.25, 0.5]),
        },
        /// Lean along the shoot, as a fraction of the radial off the wood.
        pub forward_lean: f64 = "forwardLean" "share of the radial"
            Bounds::closed(-1.0, 1.0) => [Expand] {
            wire: 95,
            check: input(Site::Canopy, 6, "forward lean"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            dial: bounded("leaf_forward_lean", "how far a leaf leans along its shoot", [0.25, 0.5]),
        },
        /// Further lean along the shoot on radials that face upward.
        pub lean_rise: f64 = "leanRise" "share of the radial"
            Bounds::closed(-2.0, 2.0) => [Expand] {
            wire: 96,
            check: input(Site::Canopy, 7, "lean rise"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            dial: bounded("leaf_lean_rise", "further lean along the shoot for leaves whose \
                radial faces up", [0.5, 1.0]),
        },
        /// The station sits on the shoot axis at 0 and on the wood's own contact
        /// surface at 1; the surface is built whenever it is positive.
        pub surface_contact: f64 = "surfaceContact" "share"
            Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 97,
            check: input(Site::Canopy, 8, "surface contact"),
            note: "Above zero leaves sit on the wood's rings, swept once for the wood and the \
                leaves or by the leaves alone.",
            dial: bounded("leaf_surface_contact", "how far out a leaf is seated, from the shoot \
                axis at nought to the wood's own skin at one", [0.15, 0.3]),
        },
        /// The degrees a leaf may be turned at random from where it was
        /// placed. Raising it leaves the crown less combed.
        pub scatter: f64 = "scatter" "degrees" Bounds::closed(0.0, 90.0) => [Plan, Expand] {
            wire: 98,
            check: input(Site::Canopy, 9, "scatter"),
            note: "Above zero a leaf draws four numbers instead of one, which moves every later \
                leaf on the stream.",
            blend: Blend::Degrees,
            dial: tuned("leaf_scatter", "the degrees a leaf may be turned at random from where \
                it was placed", [0.0, 90.0], [15.0, 30.0], "validated bound").span([0.0, 90.0]),
        },
        /// The size every leaf is drawn at, as a multiple of the element's own
        /// dimensions. Raising it enlarges every leaf.
        pub size: f64 = "size" "multiple of the element"
            Bounds::closed(0.0, 1000.0) => [Plan, Expand] {
            wire: 99,
            check: input(Site::Canopy, 10, "foliage size"),
            note: "At zero no leaf is placed.",
            dial: bounded("leaf_size", "the size every leaf is drawn at, as a multiple of the \
                element's own", [0.15, 0.3]).span([0.525, 1.575]),
        },
        /// How far leaf size varies leaf to leaf, as a share of that size.
        /// Raising it mixes larger and smaller leaves more widely.
        pub size_variation: f64 = "sizeVariation" "share"
            Bounds::closed(0.0, 0.9) => [Plan, Expand] {
            wire: 100,
            check: input(Site::Canopy, 11, "size variation"),
            dial: bounded("leaf_size_variation", "how far leaf size varies leaf to leaf",
                [0.15, 0.3]),
        },
        /// How far into each limb system the gap between it and its neighbours
        /// reaches, as a share of the way from their shared boundary to the
        /// system's centre: each limb system then keeps a rounded leaf mass of its
        /// own. Zero, the neutral, thins nothing.
        pub limb_clumping: f64 = "limbClumping" "share" Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 101,
            check: input(Site::Canopy, 12, "limb clumping"),
            note: "Above zero there is no leaf plan and leaves are thinned per limb system.",
            dial: bounded("limb_clumping", "how far the gap between neighbouring limb systems \
                reaches into each", [0.15, 0.3]),
        },
        /// How deep a lateral may be and still start a limb system of its own.
        /// Raising it parts the crown into more and smaller leaf masses; it
        /// does nothing until `limb_clumping` is above zero.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_clump_system_order"))]
        pub clump_system_order: u32 = "clumpSystemOrder" "order"
            Bounds::at_least(0.0) => [Plan, Expand] {
            wire: 102,
            note: "No upper bound is checked. The planned field assigns limb systems by it even \
                at `limbClumping` zero.",
            blend: Blend::Count,
            dial: tuned("clump_system_order", "how deep a lateral may be and still start a limb \
                system of its own",
                [1.0, 5.0], [1.0, 2.0], "capped").span([1.0, 5.0])
                .cap("capped both ways: the generator validates no closed range here (an \
                    unchecked whole number); the row stays at the preset span until the \
                    generator authors one"),
        },
        /// Nearest neighbours and cell crossings in the clumping approximation.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_clump_neighbours"))]
        pub clump_neighbours: u32 = "clumpNeighbours" "neighbours"
            crate::ranges::POSITIVE_COUNT.bounds() => [Expand] {
            wire: 103,
            check: value(Site::Clumping, 0, "clumpNeighbours"),
            applies: "`limbClumping` zero",
            blend: Blend::Count,
            dial: Dial::Excluded("An approximation budget: neighbours and cell crossings the \
                clumping search may spend (crates/telperion-core/src/foliage/placement.rs:85)."),
        },
        /// Fronds the rosette bears at the apex of each stem. At zero no rosette
        /// stands. A fraction is split by a keyed draw at each apex, which
        /// bears the whole count either side of it (fn-277).
        #[cfg_attr(feature = "json", serde(default))]
        pub rosette_fronds: f64 = "rosetteFronds" "fronds"
            Bounds::closed(0.0, crate::pipeline::foliage::MAX_FRONDS as f64) => [Grow, Plan, Expand] {
            wire: 104,
            check: input(Site::Rosette, 18, "rosette fronds"),
            note: "Above zero the apical twigs are cleared and fronds are planned and placed \
                beside whatever wood `cauline` clothes.",
            dial: bounded("rosette_fronds", "fronds in the crown at each stem apex; at zero no \
                rosette stands", [1.0, 4.0]),
        },
        /// The share of leaf-bearing wood that bears its leaves along it, cauline
        /// foliage as against a crown's alone: each run of leaf-bearing wood and
        /// each short shoot keeps its leaves by a keyed draw at this share. One,
        /// every one bears; at zero none does, as a palm bears its leaves only in
        /// its crown (fn-277).
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_cauline"))]
        pub cauline: f64 = "cauline" "share of leaf-bearing wood"
            Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 367,
            check: input(Site::Canopy, 15, "cauline share"),
            applies: "a crown of fronds (`rosetteFronds` above zero)",
            dial: bounded("cauline", "the share of leaf-bearing wood that bears its leaves \
                along it; at zero only a crown bears leaves", [0.05, 0.2]),
        },
        /// The degrees each successive frond is turned about the apex.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_rosette_divergence"))]
        pub rosette_divergence: f64 = "rosetteDivergence" "degrees"
            Bounds::closed(-1e9, 1e9) => [Grow, Plan, Expand] {
            wire: 105,
            check: input(Site::Rosette, 0, "rosette divergence"),
            applies: "no rosette and no leaf bases (`rosetteFronds` and `leafBases` zero)",
            note: "Also the leaf bases' spiral, rosette or none.",
            blend: Blend::Degrees,
            dial: tuned("rosette_divergence", "the degrees each successive frond is turned",
                [90.0, 180.0], [2.0, 8.0], "authored"),
        },
        /// Degrees from the axis the youngest frond stands: 0 upright, 90 level,
        /// 180 hanging.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_rosette_pitch"))]
        pub rosette_pitch: f64 = "rosettePitch" "degrees"
            Bounds::closed(0.0, 180.0) => [Plan, Expand] {
            wire: 106,
            check: input(Site::Rosette, 1, "rosette pitch"),
            applies: "`rosetteFronds` zero",
            dial: bounded("rosette_pitch", "degrees off the axis the youngest frond stands",
                [5.0, 15.0]),
        },
        /// How many degrees further than the youngest the oldest frond leans, so
        /// the crown opens from a spike to a skirt.
        #[cfg_attr(
            feature = "json",
            serde(default = "crate::ranges::default_rosette_pitch_spread")
        )]
        pub rosette_pitch_spread: f64 = "rosettePitchSpread" "degrees"
            Bounds::closed(0.0, 180.0) => [Plan, Expand] {
            wire: 107,
            check: input(Site::Rosette, 2, "rosette pitch spread"),
            applies: "`rosetteFronds` zero",
            dial: bounded("rosette_pitch_spread", "degrees further than that the oldest frond \
                leans", [5.0, 15.0]),
        },
        /// Metres below the apex the frond insertions are spread down the axis. At
        /// zero every frond leaves one point.
        #[cfg_attr(feature = "json", serde(default))]
        pub rosette_depth: f64 = "rosetteDepth" "m"
            Bounds::closed(0.0, 100.0) => [Grow, Plan, Expand] {
            wire: 108,
            check: input(Site::Rosette, 3, "rosette depth"),
            applies: "no rosette and no leaf bases (`rosetteFronds` and `leafBases` zero)",
            note: "Leaf bases start below it, rosette or none; the deepest frond sets the leaf \
                box's reach.",
            dial: tuned("rosette_depth", "metres the frond insertions spread down the axis",
                [0.0, 4.0], [0.05, 0.2], "authored"),
        },
        /// Leaflets one placement carries along its rachis. One is the single
        /// blade every family drew. A fraction is split by a keyed draw at each
        /// placement (fn-277).
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_leaflet_count"))]
        pub leaflet_count: f64 = "leafletCount" "leaflets"
            Bounds::closed(1.0, crate::pipeline::foliage::MAX_LEAFLETS as f64) => [Plan, Expand] {
            wire: 109,
            check: input(Site::Rosette, 20, "leaflet count"),
            applies: "`rachisLength` zero",
            note: "Leaflets group only above one and with `rachisLength` above zero.",
            blend: Blend::Log,
            dial: bounded("leaflet_count", "leaflets along one frond's rachis", [2.0, 8.0]),
        },
        /// Metres of rachis the leaflets are strung along. At zero the placement
        /// is one blade whatever the count says.
        #[cfg_attr(feature = "json", serde(default))]
        pub rachis_length: f64 = "rachisLength" "m" Bounds::closed(0.0, 1e3) => [Plan, Expand] {
            wire: 110,
            check: input(Site::Rosette, 4, "rachis length"),
            applies: "`leafletCount` one",
            dial: tuned("rachis_length", "metres of rachis the leaflets are strung along; at \
                zero one blade", [0.0, 8.0], [0.1, 0.4], "authored"),
        },
        /// The degrees a leaflet leaves its rachis.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_leaflet_pitch"))]
        pub leaflet_pitch: f64 = "leafletPitch" "degrees"
            Bounds::closed(0.0, 90.0) => [Plan, Expand] {
            wire: 111,
            check: input(Site::Rosette, 5, "leaflet pitch"),
            applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)",
            dial: bounded("leaflet_pitch", "the degrees a leaflet leaves its rachis", [3.0, 10.0]),
        },
        /// How far the rachis bends out of the straight line from its station, as
        /// a share of its length. Positive arches up, negative droops.
        #[cfg_attr(feature = "json", serde(default))]
        pub rachis_arch: f64 = "rachisArch" "share of the rachis"
            Bounds::closed(-1.0, 1.0) => [Plan, Expand] {
            wire: 112,
            check: input(Site::Rosette, 6, "rachis arch"),
            applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)",
            dial: bounded("rachis_arch", "how far the rachis bends out of its straight line",
                [0.05, 0.2]),
        },
        /// Whether a single leaflet closes the rachis's end, blended 0 to 1: the
        /// last leaflet turns from standing off the rachis to lying along it.
        #[cfg_attr(feature = "json", serde(default))]
        pub terminal_leaflet: f64 = "terminalLeaflet" "share"
            Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 113,
            check: input(Site::Rosette, 7, "terminal leaflet"),
            applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)",
            dial: bounded("terminal_leaflet", "how far a leaflet closes the rachis's end",
                [0.05, 0.2]),
        },
        /// Bases of shed fronds the stem keeps below its crown, clothing the
        /// trunk. At zero the trunk is bare and the bark is what it always was;
        /// any rise carries the crown's own spiral down it.
        #[cfg_attr(feature = "json", serde(default))]
        pub leaf_bases: f64 = "leafBases" "bases"
            Bounds::closed(0.0, crate::pipeline::branching::MAX_LEAF_BASES as f64) => [Grow] {
            wire: 114,
            check: input(Site::Rosette, 21, "leaf bases"),
            growth: Growth::Ignored,
            applies: "`leafBaseLength` zero",
            note: "Does not need a rosette: bases follow the rosette's spiral rows whether \
                fronds stand or not. A fraction is split by a keyed draw at each stem (fn-277).",
            dial: bounded("leaf_bases", "bases of shed fronds the stem keeps below its crown, \
                clothing the trunk; at zero the trunk is bare", [4.0, 16.0]),
        },
        /// Metres a retained base stands out from the bark. At zero no base is
        /// drawn whatever the count says.
        #[cfg_attr(feature = "json", serde(default))]
        pub leaf_base_length: f64 = "leafBaseLength" "m" Bounds::closed(0.0, 10.0) => [Grow] {
            wire: 115,
            check: input(Site::Rosette, 8, "leaf base length"),
            growth: Growth::Ignored,
            applies: "`leafBases` zero",
            dial: bounded("leaf_base_length", "metres a retained base stands out from the bark; \
                at zero no base is drawn", [0.05, 0.2]),
        },
        /// How thick a base is where it leaves the bark, as a share of the stem's
        /// own radius there. Raising it leaves a broader boot.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_leaf_base_radius"))]
        pub leaf_base_radius: f64 = "leafBaseRadius" "share of stem radius"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 116,
            check: input(Site::Rosette, 9, "leaf base radius"),
            growth: Growth::Ignored,
            applies: "no leaf base",
            dial: bounded("leaf_base_radius", "how thick a base is where it leaves the bark, as \
                a share of the stem's radius there", [0.05, 0.15]),
        },
        /// Degrees from the stem's axis a base points: 0 flat against the trunk,
        /// 90 square out of it, 180 turned back down.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_leaf_base_pitch"))]
        pub leaf_base_pitch: f64 = "leafBasePitch" "degrees" Bounds::closed(0.0, 180.0) => [Grow] {
            wire: 117,
            check: input(Site::Rosette, 10, "leaf base pitch"),
            growth: Growth::Ignored,
            applies: "no leaf base",
            note: "Clamped clear of the axis where `leafBaseWidth` is above zero.",
            blend: Blend::Degrees,
            dial: bounded("leaf_base_pitch", "degrees from the stem's axis a base points: 0 flat \
                against the trunk, 90 square out", [5.0, 15.0]),
        },
        /// How far the lowest and oldest base is worn back against the newest, in
        /// both its length and its girth. At zero every base stands full down the
        /// whole trunk, and any rise wears the foot away.
        #[cfg_attr(feature = "json", serde(default))]
        pub leaf_base_weathering: f64 = "leafBaseWeathering" "share"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 118,
            check: input(Site::Rosette, 11, "leaf base weathering"),
            growth: Growth::Ignored,
            applies: "no leaf base",
            dial: bounded("leaf_base_weathering", "how far the lowest and oldest base is worn \
                back against the newest; at zero none is worn", [0.05, 0.2]),
        },
        /// How broad a retained base is across the trunk, as a share of the cell
        /// the crown's spiral gives it on the bark: at 1 every base meets its
        /// neighbours edge to edge whatever the count and the trunk's girth, below
        /// it the bark shows between them and above it they crowd into each
        /// other. At zero the base is the round peg the radius row sizes and the
        /// lattice rows say nothing; any rise packs the bases into the lattice.
        #[cfg_attr(feature = "json", serde(default))]
        pub leaf_base_width: f64 = "leafBaseWidth" "share of the lattice cell"
            Bounds::closed(0.0, 2.0) => [Grow] {
            wire: 119,
            check: input(Site::Rosette, 12, "leaf base width"),
            growth: Growth::Ignored,
            applies: "no leaf base",
            note: "Above zero bases pack into a lattice of flat-faced boots.",
            dial: bounded("leaf_base_width", "how broad a retained base is across the trunk as a \
                share of the cell the crown's spiral gives it: 1 meets its neighbours edge to \
                edge; at zero the base is a round peg and any rise packs the bases into the \
                lattice", [0.05, 0.2]),
        },
        /// How flat-sided a lattice base is drawn: 0 the ellipse through its
        /// cell's corners, 1 the cell itself, a diamond with flat faces that meets
        /// each neighbour along a straight edge and is cut square at its outer
        /// end. At zero the section stays round, and at no width it reads nothing.
        #[cfg_attr(feature = "json", serde(default))]
        pub leaf_base_flatness: f64 = "leafBaseFlatness" "share"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 120,
            check: input(Site::Rosette, 13, "leaf base flatness"),
            growth: Growth::Ignored,
            applies: "no leaf base, or `leafBaseWidth` zero",
            dial: bounded("leaf_base_flatness", "how flat-sided a packed base is drawn: 1 the \
                diamond cell with flat faces, cut square at its end; at zero the section stays \
                round", [0.1, 0.25]),
        },
        /// Leaflets at a frond's base borne as spines rather than blades. At zero
        /// the frond carries blades all the way down; any rise hardens that many
        /// of them.
        #[cfg_attr(feature = "json", serde(default))]
        pub acanthophylls: f64 = "acanthophylls" "leaflets"
            Bounds::closed(0.0, crate::pipeline::foliage::MAX_LEAFLETS as f64) => [Plan, Expand] {
            wire: 121,
            check: input(Site::Rosette, 22, "acanthophylls"),
            applies: "`acanthophyllLength` zero, or no leaflet grouping (`leafletCount` one or \
                `rachisLength` zero)",
            note: "A fraction is split by a keyed draw at each placement (fn-277).",
            dial: bounded("acanthophylls", "leaflets at a frond's base borne as spines rather \
                than blades; at zero the frond carries blades", [1.0, 4.0]),
        },
        /// The share of a leaflet's own size a spine is drawn at. At zero no spine
        /// is drawn whatever the count says.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_acanthophyll_length"))]
        pub acanthophyll_length: f64 = "acanthophyllLength" "share of a leaflet"
            Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 122,
            check: input(Site::Rosette, 14, "acanthophyll length"),
            applies: "`acanthophylls` zero, or no leaflet grouping (`leafletCount` one or \
                `rachisLength` zero)",
            dial: bounded("acanthophyll_length", "the share of a leaflet's own size a spine is \
                drawn at; at zero no spine is drawn", [0.05, 0.15]),
        },
        /// The degrees a spine leaves the rachis, in place of the leaflet's own
        /// pitch.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_acanthophyll_pitch"))]
        pub acanthophyll_pitch: f64 = "acanthophyllPitch" "degrees"
            Bounds::closed(0.0, 90.0) => [Plan, Expand] {
            wire: 123,
            check: input(Site::Rosette, 15, "acanthophyll pitch"),
            applies: "no spine",
            blend: Blend::Degrees,
            dial: bounded("acanthophyll_pitch", "the degrees a spine leaves the rachis, in place \
                of the leaflet's own pitch", [3.0, 10.0]),
        },
        /// Dead fronds a rosette keeps below its living crown, continuing the
        /// crown's own spiral down the stem. At zero no frond is kept and the
        /// crown ends at its oldest living frond; any rise hangs that many.
        #[cfg_attr(feature = "json", serde(default))]
        pub skirt_fronds: f64 = "skirtFronds" "fronds"
            Bounds::closed(0.0, crate::pipeline::foliage::MAX_FRONDS as f64) => [Plan, Expand] {
            wire: 124,
            check: input(Site::Rosette, 19, "skirt fronds"),
            applies: "`skirtLength` zero, or `rosetteFronds` zero",
            note: "A fraction is split by a keyed draw at each apex (fn-277).",
            dial: bounded("skirt_fronds", "dead fronds a rosette keeps below its living crown; \
                at zero none is kept", [2.0, 6.0]),
        },
        /// Degrees from the axis a dead frond hangs: 0 upright, 90 level, 180
        /// collapsed straight down against the stem.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_skirt_pitch"))]
        pub skirt_pitch: f64 = "skirtPitch" "degrees" Bounds::closed(0.0, 180.0) => [Plan, Expand] {
            wire: 125,
            check: input(Site::Rosette, 16, "skirt pitch"),
            applies: "no dead frond",
            dial: bounded("skirt_pitch", "degrees from the axis a dead frond hangs: 90 level, \
                180 collapsed against the stem", [5.0, 15.0]),
        },
        /// A dead frond's length as a share of a living one's, rachis and
        /// leaflets alike. At zero no dead frond is drawn whatever the count says.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_skirt_length"))]
        pub skirt_length: f64 = "skirtLength" "share of a living frond"
            Bounds::closed(0.0, 1.0) => [Plan, Expand] {
            wire: 126,
            check: input(Site::Rosette, 17, "skirt length"),
            applies: "`skirtFronds` zero, or `rosetteFronds` zero",
            dial: bounded("skirt_length", "a dead frond's length as a share of a living one's; \
                at zero no dead frond is drawn", [0.05, 0.15]),
        },
        /// The years a living frond lives (fn-270 R5b): a rosette makes its
        /// living crown's count over that many years, its fronds aged by
        /// their place on the spiral, this year's emerging over its stem's
        /// elongation and unfolding by its expansion, the oldest passing
        /// into the skirt and out of it as new ones emerge above. At zero
        /// the crown makes none and stands as it is all year.
        #[cfg_attr(feature = "json", serde(default))]
        pub frond_life: f64 = "frondLife" "years"
            Bounds::closed(0.0, 100.0) => [Plan, Expand] {
            wire: 364,
            check: input(Site::Rosette, 23, "frond life"),
            applies: "`rosetteFronds` zero",
            dial: bounded("frond_life", "the years a living frond lives; at zero the crown stands \
                as it is all year", [3.0, 7.0]),
        },
        /// The years a rosette's crown takes to make a mature frond (fn-275
        /// R2, R2a): a frond is made at a share of its size set by the
        /// crown's age when it was made, along the logistic course from
        /// `frondEarlyShare` to whole over these years, and keeps it for its
        /// life; the stem the rosette crowns widens by the same course and
        /// waits at the ground until the crown is established. At zero every
        /// frond is made whole and the stem rises from its first year.
        #[cfg_attr(feature = "json", serde(default))]
        pub frond_establishment: f64 = "frondEstablishment" "years"
            Bounds::closed(0.0, 100.0) => [Grow, Plan, Expand] {
            wire: 365,
            check: input(Site::Rosette, 24, "frond establishment"),
            applies: "`rosetteFronds` zero",
            dial: bounded("frond_establishment", "the years a crown takes to make a mature \
                frond and its stem to widen before it rises; at zero every frond is made whole \
                and the stem rises from its first year", [1.0, 3.0]),
        },
        /// The share of a mature frond's size a crown's first frond is made
        /// at while it establishes, and of its stem's girth where the stem
        /// was first laid (fn-275 R2, R2a).
        #[cfg_attr(feature = "json", serde(default))]
        pub frond_early_share: f64 = "frondEarlyShare" "share of a mature frond"
            Bounds::closed(0.0, 1.0) => [Grow, Plan, Expand] {
            wire: 366,
            check: input(Site::Rosette, 25, "frond early share"),
            applies: "`frondEstablishment` zero",
            dial: bounded("frond_early_share", "the share of a mature frond's size a crown's \
                first frond is made at, and of its stem's girth where first laid", [0.05, 0.15]),
        },
        /// Hard total budget. Exceeding it returns an error, never partial foliage.
        #[cfg_attr(feature = "json", serde(with = "crate::ranges::portable_index"))]
        pub max_instances: usize = "maxInstances" "leaves"
            Bounds::closed(1.0, usize::MAX as f64) => [Plan, Expand] {
            wire: 127,
            check: input(Site::Canopy, 13, "foliage instance budget"),
            growth: Growth::Differs("`usize::MAX` turns on the sparse validation interval"),
            note: "Exceeding it is an error, never fewer leaves.",
            blend: Blend::Many,
            dial: Dial::Excluded("A hard resource cap; exceeding it is an error, never a \
                different tree (crates/telperion-core/src/foliage/placement.rs:88)."),
        },
    }
}
impl Default for CanopyParams {
    fn default() -> Self {
        Self {
            shoot_radius: 0.,
            spacing: 0.006,
            divergence: 137.508,
            clump: 5,
            clump_span: 0.3,
            outward: 0.6,
            upward: 0.35,
            forward_lean: 0.,
            lean_rise: 0.,
            surface_contact: 0.,
            scatter: 18.,
            size: 1.,
            size_variation: 0.35,
            // Neutral: every leaf the stations and the clusters place.
            limb_clumping: 0.,
            clump_system_order: crate::ranges::default_clump_system_order(),
            clump_neighbours: crate::ranges::default_clump_neighbours(),
            // Neutral: no rosette stands and no placement groups until a
            // table states a frond count and a rachis to string leaflets on.
            rosette_fronds: 0.0,
            cauline: 1.0,
            rosette_divergence: crate::ranges::default_rosette_divergence(),
            rosette_pitch: crate::ranges::default_rosette_pitch(),
            rosette_pitch_spread: crate::ranges::default_rosette_pitch_spread(),
            rosette_depth: 0.,
            leaflet_count: crate::ranges::default_leaflet_count(),
            rachis_length: 0.,
            leaflet_pitch: crate::ranges::default_leaflet_pitch(),
            rachis_arch: 0.,
            terminal_leaflet: 0.,
            // Neutral: the trunk carries no retained base and no frond bears a
            // spine until a table states a count and the reach to draw it at.
            leaf_bases: 0.0,
            leaf_base_length: 0.,
            leaf_base_radius: crate::ranges::default_leaf_base_radius(),
            leaf_base_pitch: crate::ranges::default_leaf_base_pitch(),
            leaf_base_weathering: 0.,
            leaf_base_width: 0.,
            leaf_base_flatness: 0.,
            acanthophylls: 0.0,
            acanthophyll_length: crate::ranges::default_acanthophyll_length(),
            acanthophyll_pitch: crate::ranges::default_acanthophyll_pitch(),
            // Neutral: no dead frond is kept below the living crown.
            skirt_fronds: 0.0,
            skirt_pitch: crate::ranges::default_skirt_pitch(),
            skirt_length: crate::ranges::default_skirt_length(),
            frond_life: 0.0,
            frond_establishment: 0.0,
            frond_early_share: 0.0,
            max_instances: usize::MAX,
        }
    }
}
