//! When the page draws a frame (fn-246): only when something in the picture
//! changed - the camera, the tree, the view, the canvas's size or the
//! scene - and never while the last frame's work is on the device. A change
//! asked for while a frame is in flight waits for the next attempt rather
//! than being lost, and is let go only when a frame that shows it is
//! submitted whole.
#[derive(Debug)]
pub(crate) struct Redraw {
    pending: bool,
}

impl Default for Redraw {
    /// A new canvas has drawn nothing yet.
    fn default() -> Self {
        Self { pending: true }
    }
}

impl Redraw {
    /// Something in the picture changed.
    pub(crate) fn ask(&mut self) {
        self.pending = true;
    }

    /// Whether this attempt draws: something is pending, or the last frame
    /// failed and has to be drawn again, and nothing is in flight.
    pub(crate) fn due(&self, in_flight: bool, failed: bool) -> bool {
        !in_flight && (self.pending || failed)
    }

    /// A frame was submitted. It settles what was pending unless it could
    /// not draw all of it (the wood's programs still compiling), in which
    /// case the next attempt draws again.
    pub(crate) fn submitted(&mut self, whole: bool) {
        self.pending = !whole;
    }
}

#[cfg(test)]
mod tests {
    use super::Redraw;

    /// Draws when due and reports whether it did, the way `frame` does.
    fn attempt(redraw: &mut Redraw, in_flight: bool, whole: bool) -> bool {
        let due = redraw.due(in_flight, false);
        if due {
            redraw.submitted(whole);
        }
        due
    }

    #[test]
    fn an_idle_canvas_draws_once_and_then_nothing() {
        let mut redraw = Redraw::default();
        assert!(attempt(&mut redraw, false, true), "the first frame was not drawn");
        for _ in 0..3 {
            assert!(!attempt(&mut redraw, false, true), "an unchanged picture was redrawn");
        }
    }

    #[test]
    fn a_resize_or_a_ground_only_scene_edit_draws_one_frame() {
        let mut redraw = Redraw::default();
        attempt(&mut redraw, false, true);
        for _change in ["resize", "ground colour"] {
            redraw.ask();
            assert!(attempt(&mut redraw, false, true));
            assert!(!attempt(&mut redraw, false, true));
        }
    }

    #[test]
    fn a_camera_move_during_a_frame_in_flight_is_drawn_after_it() {
        let mut redraw = Redraw::default();
        attempt(&mut redraw, false, true);
        redraw.ask();
        assert!(!attempt(&mut redraw, true, true), "drew over a frame in flight");
        assert!(!attempt(&mut redraw, true, true));
        assert!(attempt(&mut redraw, false, true), "the move was lost");
        assert!(!attempt(&mut redraw, false, true));
    }

    #[test]
    fn a_partial_or_failed_frame_is_drawn_again() {
        let mut redraw = Redraw::default();
        assert!(attempt(&mut redraw, false, false));
        assert!(attempt(&mut redraw, false, true), "a frame without its wood settled");
        assert!(!redraw.due(false, false));
        assert!(redraw.due(false, true), "a failed frame was not drawn again");
        assert!(!redraw.due(true, true));
    }
}
