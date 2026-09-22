//! What the generator can express, declared once and owned by no preset.
//!
//! The species pipeline's capability gate compares a species' required names
//! against [`EXPRESSED`]. A name the field cannot draw is simply absent from
//! it and waits in [`UNEXPRESSED`] for the spec that implements it; that spec
//! moves the line from one list to the other, in one line, with a test that
//! failed before it. A required name in neither list is unrecognised, which is
//! a different report than missing: nobody has said what it means.
//!
//! [`derived`] is the other question, and not this one. It reads a registered
//! preset's own value table and says what that table produces, which answers
//! whether a shipped species draws what it claims. It never answers what the
//! generator can express.

use crate::presets::Preset;

/// One capability name and the one line that says what the name means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability {
    pub name: &'static str,
    pub meaning: &'static str,
}

/// The vocabulary: what the generator expresses today. The first six names
/// are the frozen fn-19 protocol's, the ones [`derived`] reads off a value
/// table; then what the shipped beech and birch require and draw, and last the
/// pinnate grouping and the frond crown fn-109 gave the canopy, and the two
/// trunk organs fn-110 gave it.
pub const EXPRESSED: &[Capability] = &[
    Capability {
        name: "woody-axes",
        meaning: "wood along every axis the skeleton grows, the stem down to the finest twig",
    },
    Capability {
        name: "lobed-blade",
        meaning: "a blade whose margin is cut into lobes rather than entire",
    },
    Capability {
        name: "four-sided-needle",
        meaning: "a needle whose section is rolled past halfway towards square",
    },
    Capability {
        name: "alternate-petiole",
        meaning: "a blade that leans off its own petiole, held clear of the wood it is borne on",
    },
    Capability {
        name: "radial-peg",
        meaning: "a needle pegged into the wood all round its shoot, without a petiole",
    },
    Capability {
        name: "tiered-secondary",
        meaning: "secondary axes that hang below their parent, the whorl read as a tier",
    },
    Capability {
        name: "entire-blade",
        meaning: "a blade whose margin is uncut, the beech's and the birch's leaf",
    },
    Capability {
        name: "pendulous-laterals",
        meaning: "laterals that hang from where they are borne, the birch's fall",
    },
    Capability {
        name: "pinnate-compound",
        meaning: "one leaf divided into leaflets along a rachis and borne as a single placement",
    },
    Capability {
        name: "apical-rosette",
        meaning: "leaves borne only as a crown at the apex of an axis that bears none below it",
    },
    Capability {
        name: "pinnate-frond",
        meaning: "a palm's frond: the same pinnate grouping the ash needs, at a frond's scale",
    },
    Capability {
        name: "acanthophyll",
        meaning: "a leaflet hardened into a spine, borne at the base of a frond",
    },
    Capability {
        name: "persistent-leaf-base",
        meaning: "the sheathing base of a shed leaf kept on the axis, clothing the trunk",
    },
];

/// Names an assessment may use that the generator cannot express yet. Each is
/// a gap with a spec of its own, and none of them is a capability the
/// generator has: a required name here is missing, not satisfied. The first is
/// the ash's remaining unmet name and the second the date palm's fruiting
/// cluster.
pub const UNEXPRESSED: &[Capability] = &[
    Capability {
        name: "opposite-attachment",
        meaning: "leaves borne as opposite pairs at one station, not scattered one to a node",
    },
    Capability {
        name: "infructescence",
        meaning: "a branched fruiting cluster borne from an axis, distinct from the foliage",
    },
];

/// The names [`derived`] can read off a value table: the frozen fn-19
/// protocol's six, the three fn-109 added and the two fn-110 added, each with
/// a threshold on a shipped value. A name outside this set has no threshold, so a table that
/// does not produce it has said nothing about it either way.
pub const DERIVABLE: &[&str] = &[
    "woody-axes",
    "lobed-blade",
    "four-sided-needle",
    "alternate-petiole",
    "radial-peg",
    "tiered-secondary",
    "pinnate-compound",
    "apical-rosette",
    "pinnate-frond",
    "persistent-leaf-base",
    "acanthophyll",
];

/// What the vocabulary says about one required name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Support {
    /// The generator expresses it.
    Expressed,
    /// A name the vocabulary carries and the generator cannot express yet.
    Absent,
    /// A name the vocabulary does not carry at all.
    Unrecognised,
}

/// The vocabulary's word on one required name.
pub fn support(name: &str) -> Support {
    if EXPRESSED.iter().any(|entry| entry.name == name) {
        Support::Expressed
    } else if UNEXPRESSED.iter().any(|entry| entry.name == name) {
        Support::Absent
    } else {
        Support::Unrecognised
    }
}

/// The vocabulary's version: a digest over every declared name and the list it
/// sits in, so the version changes exactly when the list changes and a name
/// that moves from unexpressed to expressed changes it too. A round of the
/// capability assessment records the version it assessed against and the gate
/// records the version it compared against, so the two can be compared. The
/// meanings are prose and are not digested; the names are what an assessment
/// can use.
pub fn version() -> String {
    digest(EXPRESSED, UNEXPRESSED)
}

fn digest(expressed: &[Capability], unexpressed: &[Capability]) -> String {
    // FNV-1a over the declared names, written out so the digest is the same
    // number on every toolchain and stays comparable across runs.
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut eat = |bytes: &[u8]| {
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    };
    for (list, tag) in [(expressed, b"e"), (unexpressed, b"u")] {
        for entry in list {
            eat(tag);
            eat(entry.name.as_bytes());
            eat(b"\n");
        }
    }
    format!("v{hash:016x}")
}

/// The vocabulary as an assessment round reads it: the version the round has
/// to record, then every declared name with the one line beside it and the
/// list it sits in. `geometry_benchmark --vocabulary` prints this, so a round
/// reads what the generator expresses from the generator rather than from a
/// file it has to find, and records a version it did not have to compute.
#[cfg(feature = "json")]
pub fn vocabulary() -> serde_json::Value {
    let entries = |list: &[Capability]| -> Vec<serde_json::Value> {
        list.iter()
            .map(|entry| serde_json::json!({"name": entry.name, "meaning": entry.meaning}))
            .collect()
    };
    serde_json::json!({
        "vocabulary_version": version(),
        "expressed": entries(EXPRESSED),
        "unexpressed": entries(UNEXPRESSED),
    })
}

/// The capabilities a preset's own value table produces, as thresholds on the
/// values it ships. This is a check over a registered preset, never a reading
/// of what the generator can express.
pub fn derived(preset: Preset) -> Vec<&'static str> {
    let f = preset.parameters();
    let mut produced = vec!["woody-axes"];
    // A lobed margin and a section rolled past halfway are what these two
    // names have always meant.
    if f.element.lobe_count > 0 && f.element.lobe_depth > 0.0 {
        produced.push("lobed-blade");
    }
    if f.element.section_roundness >= 0.5 {
        produced.push("four-sided-needle");
    }
    // A blade that leans off its own petiole and a needle pegged into the wood
    // are what these two attachments have always meant.
    if f.canopy.forward_lean > 0.0 && f.canopy.surface_contact < 0.5 {
        produced.push("alternate-petiole");
    }
    if f.canopy.surface_contact >= 0.5 {
        produced.push("radial-peg");
    }
    // A family whose deeper axes hang is what this name has always meant.
    if f.skeleton.habit.rise_secondary < 0.0 {
        produced.push("tiered-secondary");
    }
    // A placement that carries leaflets along a rachis is the compound leaf; a
    // crown of them borne only at an apex is the palm's frond.
    let pinnate = f.canopy.leaflet_count > 1 && f.canopy.rachis_length > 0.0;
    let rosette = f.canopy.rosette_fronds > 0;
    if pinnate {
        produced.push("pinnate-compound");
    }
    if rosette {
        produced.push("apical-rosette");
    }
    if pinnate && rosette {
        produced.push("pinnate-frond");
    }
    // The trunk organs are the crown's own history and the frond's own
    // leaflets: a base keeps the spiral the rosette turns in, and a spine is a
    // leaflet hardened, so neither name is produced without the crown.
    if f.canopy.leaf_bases > 0 && f.canopy.leaf_base_length > 0.0 && rosette {
        produced.push("persistent-leaf-base");
    }
    if f.canopy.acanthophylls > 0 && f.canopy.acanthophyll_length > 0.0 && pinnate {
        produced.push("acanthophyll");
    }
    produced
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: Capability = Capability {
        name: "woody-axes",
        meaning: "the one name every family has",
    };
    const ANOTHER: Capability = Capability {
        name: "apical-rosette",
        meaning: "a name the generator cannot draw",
    };

    #[test]
    fn the_version_changes_when_a_name_is_added() {
        assert_ne!(digest(&[ONE], &[]), digest(&[ONE, ANOTHER], &[]));
    }

    #[test]
    fn the_version_changes_when_a_name_moves_into_the_vocabulary() {
        assert_ne!(digest(&[ONE], &[ANOTHER]), digest(&[ONE, ANOTHER], &[]));
    }

    #[test]
    fn the_version_ignores_the_meanings_and_reads_only_the_names() {
        let reworded = Capability {
            name: ONE.name,
            meaning: "the same name, said again in other words",
        };
        assert_eq!(digest(&[ONE], &[]), digest(&[reworded], &[]));
    }
}
