//! What the sun's map was last drawn for (fn-246): the light, the view's
//! casters, the caster stride and whether the person casts (fn-310). None of
//! them depends on the camera, so a
//! frame that changes none of them reuses the map. The sun's wood is
//! surfaced into the buffers the camera's is written over in the same frame
//! (fn-258), so a map drawn again surfaces it again.
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
    figure: bool,
}

impl Key {
    /// `figure` is whether the person stands in the sun's map. Where it
    /// stands is the tree's to say, and a new tree forgets the map.
    pub(crate) fn new(light: Light, view: View, stride: u32, figure: bool) -> Self {
        Self {
            light,
            casters: Casters::of(view),
            stride,
            figure,
        }
    }
}

/// The key the map on the device was drawn for, where it is current, and
/// whether the last frame reused it.
#[derive(Debug, Default)]
pub(crate) struct Sun {
    drawn: Option<Key>,
    pub(crate) reused: bool,
}

impl Sun {
    /// Whether a frame under `key` draws the map, and the sun's wood with
    /// it, again.
    pub(crate) fn plan(&self, key: &Key) -> bool {
        self.drawn.as_ref() != Some(key)
    }

    /// The frame drew the map where `map` asked under `key`; `whole` is
    /// false where it could not surface the wood (its programs still
    /// compiling), and then the map it drew is not current.
    pub(crate) fn drew(&mut self, key: Key, map: bool, whole: bool) {
        self.reused = !map;
        self.drawn = whole.then_some(key);
    }

    /// A new tree, a released one or a failed frame: nothing is current.
    pub(crate) fn forget(&mut self) {
        self.drawn = None;
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
        crate::shadow::light(&row, &[])
    }

    /// Draws a frame under `key` the way the renderer does and returns
    /// whether it drew the map.
    fn frame(sun: &mut Sun, key: Key) -> bool {
        let map = sun.plan(&key);
        sun.drew(key, map, true);
        map
    }

    #[test]
    fn the_map_is_drawn_again_when_the_light_the_casters_the_stride_or_the_person_change() {
        let mut sun = Sun::default();
        let whole = Key::new(light(30.0), View::Whole, 4, true);
        assert!(frame(&mut sun, whole));
        assert!(!frame(&mut sun, whole));
        assert!(sun.reused);
        assert!(frame(&mut sun, Key::new(light(30.0), View::Bare, 4, true)));
        assert!(frame(&mut sun, whole));
        assert!(frame(&mut sun, Key::new(light(30.0), View::Whole, 9, true)));
        assert!(frame(
            &mut sun,
            Key::new(light(200.0), View::Whole, 9, true)
        ));
        assert!(frame(
            &mut sun,
            Key::new(light(200.0), View::Whole, 9, false)
        ));
        assert!(!frame(
            &mut sun,
            Key::new(light(200.0), View::Whole, 9, false)
        ));
        assert!(frame(
            &mut sun,
            Key::new(light(200.0), View::Whole, 9, true)
        ));
        sun.forget();
        assert!(sun.plan(&whole));
    }

    #[test]
    fn a_frame_without_its_wood_leaves_nothing_current() {
        let mut sun = Sun::default();
        let key = Key::new(light(30.0), View::Whole, 4, true);
        let map = sun.plan(&key);
        sun.drew(key, map, false);
        assert!(sun.plan(&key));
    }
}
