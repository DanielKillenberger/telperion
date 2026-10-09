//! What the sun's side of a frame was last drawn for (fn-246): its wood
//! surfaced for the map and the map itself depend on the tree, the light,
//! the view's casters and the caster stride, never on the camera, so a
//! frame that changes none of them reuses both.
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

/// Everything the sun's side is drawn from beside the tree, whose change
/// forgets it outright.
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

/// The key the map now on the device was drawn for, if it is current, and
/// whether the last frame reused it.
#[derive(Debug, Default)]
pub(crate) struct Sun {
    drawn: Option<Key>,
    pub(crate) reused: bool,
}

impl Sun {
    /// Whether a frame under `key` may reuse what is on the device.
    pub(crate) fn current(&self, key: &Key) -> bool {
        self.drawn.as_ref() == Some(key)
    }

    /// The frame drew the sun's side under `key`; `None` where it could not
    /// draw it whole (the wood's programs still compiling).
    pub(crate) fn drawn(&mut self, key: Option<Key>) {
        self.drawn = key;
    }

    /// A new tree, a released one or a failed frame: nothing is current.
    pub(crate) fn forget(&mut self) {
        self.drawn = None;
    }
}
