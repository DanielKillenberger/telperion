//! What of a submitted tree the next frame draws. The same three the harness
//! offers, so a way of looking at a tree survives the change of renderer.

/// Whole is the tree as it stands; bare strips the crown so the surface can be
/// judged with nothing over it; leaf isolates one element at the scale it was
/// generated at, placed at the origin, so its own shape can be read. Clay is
/// the whole tree in the neutral room instead of outdoors: no sun, no
/// material, no tone map, so geometry can be inspected with nothing over it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    #[default]
    Whole,
    Bare,
    Leaf,
    Clay,
}

impl View {
    /// The names a caller may pass, in the order a usage line wants them.
    pub const NAMES: [&'static str; 4] = ["whole", "bare", "leaf", "clay"];

    /// The view of that name, or nothing. An unknown name is the caller's to
    /// report; the renderer never guesses one.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "whole" => Some(Self::Whole),
            "bare" => Some(Self::Bare),
            "leaf" => Some(Self::Leaf),
            "clay" => Some(Self::Clay),
            _ => None,
        }
    }

    /// Whether the frame chooses a level for every leaf of the crown. The
    /// whole tree and the clay room both draw it; the bare view has no crown
    /// and the leaf view is one instance at one known level.
    pub fn selects(self) -> bool {
        matches!(self, Self::Whole | Self::Clay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_the_usage_line_offers_resolves_and_nothing_else_does() {
        for name in View::NAMES {
            assert!(
                View::from_id(name).is_some(),
                "{name} is offered but unknown"
            );
        }
        assert_eq!(View::NAMES.len(), 4, "a view was added without a name");
        for unknown in ["", "Whole", "foliage-detail", "wood"] {
            assert_eq!(View::from_id(unknown), None, "{unknown:?} was guessed at");
        }
    }
}
