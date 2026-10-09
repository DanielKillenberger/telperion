//! What the sun's side of a frame was last drawn for (fn-246). Its wood,
//! surfaced for the map, depends on the tree and the light; the map itself
//! also on the view's casters and the caster stride. Neither depends on the
//! camera, so a frame that changes none of them reuses both, and a frame
//! that changes only the casters redraws the map over the same wood.
use crate::{shadow::Light, View};

/// What stands between the sun and the ground in a view: nothing in the
/// leaf view, the wood alone when it is bare, the wood and the crown in
/// every view that draws the crown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Casters {
    None,
    Wood,
    Crown,
}

impl Casters {
    fn of(view: View) -> Self {
        match view {
            View::Leaf => Self::None,
            View::Bare => Self::Wood,
            View::Whole | View::Clay => Self::Crown,
        }
    }
}

/// Everything the map is drawn from beside the tree, whose change forgets
/// both outright.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Key {
    light: Light,
    casters: Casters,
    stride: u32,
}

impl Key {
    pub(crate) fn new(light: Light, view: View, stride: u32) -> Self {
        Self {
            light,
            casters: Casters::of(view),
            stride,
        }
    }
}

/// What a frame has to draw of the sun's side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Plan {
    /// Surface the wood for the sun again.
    pub(crate) surface: bool,
    /// Draw the map again.
    pub(crate) map: bool,
}

/// The light the sun's wood was surfaced for and the key the map on the
/// device was drawn for, each where it is current, and whether the last
/// frame reused the map.
#[derive(Debug, Default)]
pub(crate) struct Sun {
    surfaced: Option<Light>,
    drawn: Option<Key>,
    pub(crate) reused: bool,
}

impl Sun {
    /// What a frame under `key` draws. New wood is drawn into a new map.
    pub(crate) fn plan(&self, key: &Key) -> Plan {
        let surface = self.surfaced != Some(key.light);
        Plan {
            surface,
            map: surface || self.drawn.as_ref() != Some(key),
        }
    }

    /// The frame drew what `plan` asked under `key`; `whole` is false where
    /// it could not surface the wood (its programs still compiling), and
    /// then nothing it drew is current.
    pub(crate) fn drew(&mut self, key: Key, plan: Plan, whole: bool) {
        self.reused = !plan.map;
        if !whole {
            self.forget();
            return;
        }
        self.surfaced = Some(key.light);
        self.drawn = Some(key);
    }

    /// A new tree, a released one or a failed frame: nothing is current.
    pub(crate) fn forget(&mut self) {
        (self.surfaced, self.drawn) = (None, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn light(azimuth: f64) -> Light {
        let row = crate::SceneRow {
            sun_azimuth: azimuth,
            ..crate::SceneRow::default()
        };
        crate::shadow::light(&row, None)
    }

    /// Draws a frame under `key` the way the renderer does and returns the
    /// plan it followed.
    fn frame(sun: &mut Sun, key: Key) -> Plan {
        let plan = sun.plan(&key);
        sun.drew(key, plan, true);
        plan
    }

    #[test]
    fn the_casters_redraw_the_map_over_the_same_wood_and_the_light_redraws_both() {
        let mut sun = Sun::default();
        let (whole, both) = (Key::new(light(30.0), View::Whole, 4), Plan { surface: true, map: true });
        assert_eq!(frame(&mut sun, whole), both);
        let none = Plan { surface: false, map: false };
        assert_eq!(frame(&mut sun, whole), none);
        assert!(sun.reused);
        let map = Plan { surface: false, map: true };
        assert_eq!(frame(&mut sun, Key::new(light(30.0), View::Bare, 4)), map);
        assert_eq!(frame(&mut sun, whole), map);
        assert_eq!(frame(&mut sun, Key::new(light(30.0), View::Whole, 9)), map);
        assert_eq!(frame(&mut sun, Key::new(light(200.0), View::Whole, 9)), both);
        sun.forget();
        assert_eq!(sun.plan(&whole), both);
    }

    #[test]
    fn a_frame_without_its_wood_leaves_nothing_current() {
        let mut sun = Sun::default();
        let key = Key::new(light(30.0), View::Whole, 4);
        let plan = sun.plan(&key);
        sun.drew(key, plan, false);
        assert_eq!(sun.plan(&key), Plan { surface: true, map: true });
    }
}
