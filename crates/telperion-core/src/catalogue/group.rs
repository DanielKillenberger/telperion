//! What a control does to the tree, the panel's groups (fn-284 R3): every
//! catalogue row and every engine setting sits in one, beside its meaning.
//! A row is placed by its own path first, then by the longest prefix that
//! holds it; an engine setting by its name.

/// A group of controls, in the order the panel shows them: the most
/// telling first, the engine's internals last and closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Size,
    Form,
    Branching,
    Leaves,
    Flowers,
    Bark,
    Supernatural,
    Advanced,
}

impl Group {
    pub const ALL: [Group; 8] = [
        Group::Size,
        Group::Form,
        Group::Branching,
        Group::Leaves,
        Group::Flowers,
        Group::Bark,
        Group::Supernatural,
        Group::Advanced,
    ];

    /// The group's heading on the panel.
    pub const fn title(self) -> &'static str {
        match self {
            Group::Size => "size and age",
            Group::Form => "trunk and form",
            Group::Branching => "branching",
            Group::Leaves => "leaves and fronds",
            Group::Flowers => "flowers and fruit",
            Group::Bark => "bark and colour",
            Group::Supernatural => "supernatural",
            Group::Advanced => "advanced",
        }
    }
}

use Group::{Advanced, Bark, Branching, Flowers, Form, Leaves, Size, Supernatural};

/// Catalogue rows by path or path prefix (ending in `/`).
const ROWS: &[(&str, Group)] = &[
    ("/age", Size),
    ("/skeleton/seed", Advanced),
    ("/radii/trunkRadius", Advanced),
    ("/skeleton/twigs/twig/diameter", Advanced),
    ("/skeleton/twigs/", Leaves),
    ("/canopy/maxInstances", Advanced),
    ("/canopy/clumpNeighbours", Advanced),
    ("/canopy/clumpSystemOrder", Advanced),
    ("/canopy/leafBase", Form),
    ("/canopy/", Leaves),
    ("/element/axialSegments", Advanced),
    ("/element/crossSegments", Advanced),
    ("/element/", Leaves),
    ("/surface/radialSegments", Advanced),
    ("/surface/lobes", Bark),
    ("/surface/lobeDepth", Bark),
    ("/surface/twistRate", Bark),
    ("/surface/", Form),
    ("/material/bloom", Flowers),
    ("/material/", Bark),
    ("/bloom/", Flowers),
    ("/supernatural/", Supernatural),
];

/// Engine settings by name.
const SETTINGS: &[(&str, Group)] = &[
    ("lifespan", Size),
    ("continuation", Size),
    ("viability", Size),
    ("internode", Size),
    ("elongation", Size),
    ("form.pipe", Form),
    ("form.exponent", Form),
    ("form.ripening", Form),
    ("form.secondary", Form),
    ("form.tropism", Form),
    ("form.elevation", Form),
    ("form.wander", Form),
    ("form.bend_length", Form),
    ("form.sag", Form),
    ("form.internode_spread", Form),
    ("form.tropism_spread", Form),
    ("erection", Form),
    ("straightening", Form),
    ("leaf_girth", Form),
    ("retained", Form),
    ("insertion", Branching),
    ("divergence", Branching),
    ("form.plane", Branching),
    ("form.roll", Branching),
    ("form.dominance", Branching),
    ("form.insertion_spread", Branching),
    ("apical_control", Branching),
    ("readiness", Branching),
    ("rhythm", Branching),
    ("abortion", Branching),
    ("abortion_rise", Branching),
    ("relay", Branching),
    ("relay_ended", Branching),
    ("relay_failed", Branching),
    ("relay_at", Branching),
    ("epitony", Branching),
    ("takeover", Branching),
    ("shedding", Branching),
    ("shade_hazard", Branching),
    ("ground_hazard", Branching),
    ("shade_size", Branching),
    ("upkeep", Branching),
    ("balance_hazard", Branching),
    ("tolerance", Branching),
    ("zone.nodes.min", Branching),
    ("zone.nodes.max", Branching),
    ("zone.buds", Branching),
    ("zone.lateral", Branching),
    ("zone.dormant", Branching),
    ("zone.delay", Branching),
    ("zone.rate", Branching),
    ("leaf_area", Leaves),
    ("cluster_leaves", Leaves),
    ("cluster_spread", Leaves),
    ("leaf_years", Leaves),
    ("expansion", Leaves),
    ("leaf_life", Leaves),
    ("budburst", Leaves),
    ("senescence", Leaves),
    ("colouring", Leaves),
    ("leaf_fall", Leaves),
    ("flowering", Flowers),
];

/// The group of the catalogue row at `path`.
pub fn of_row(path: &str) -> Option<Group> {
    let exact = ROWS.iter().find(|(p, _)| *p == path);
    let prefix = (ROWS.iter())
        .filter(|(p, _)| path.starts_with(p))
        .max_by_key(|(p, _)| p.len());
    exact.or(prefix).map(|&(_, g)| g)
}

/// The group of the engine setting `name`.
pub fn of_setting(name: &str) -> Option<Group> {
    SETTINGS.iter().find(|(n, _)| *n == name).map(|&(_, g)| g)
}
