//! A build's resident state moved into larger tables on the queue
//! (DESIGN.md section 5): the state's and the hierarchy's columns copied
//! column by column to their new strides, every other buffer the cycle
//! writes before it reads. No synchronisation: the copies follow the
//! cycles already submitted.
use super::build::Build;
use super::hier_build::{COUNTS, LISTS, SHAPE};
use super::state::{AXES, PENCILS, PHYTS, SLEEPERS, TALLY, UNITS, WOKEN};
use crate::buffer::Owned;

/// `columns` columns of `from`, `a` rows apart from word `at_a`, into
/// `into`, `b` rows apart from word `at_b`, each `a` rows long.
fn columns(e: &mut wgpu::CommandEncoder, (from, into): (&Owned, &Owned), columns: u64, (a, b): (u64, u64), (at_a, at_b): (u64, u64)) {
    for k in 0..columns {
        e.copy_buffer_to_buffer(from, 4 * (at_a + k * a), into, 4 * (at_b + k * b), 4 * a);
    }
}

impl Build {
    /// This build's state, hierarchy and census recorded into `into`'s.
    pub(super) fn move_into(&self, e: &mut wgpu::CommandEncoder, into: &Build, census: (&Owned, &Owned)) {
        let (o, n) = (self.state.caps, into.state.caps);
        let (s, t) = (&self.state, &into.state);
        let w = u64::from;
        columns(e, (&s.axes, &t.axes), AXES, (w(o.axes), w(n.axes)), (0, 0));
        columns(e, (&s.pencils, &t.pencils), PENCILS, (w(o.axes), w(n.axes)), (0, 0));
        columns(e, (&s.arena, &t.arena), UNITS, (w(o.units), w(n.units)), (0, 0));
        columns(e, (&s.arena, &t.arena), PHYTS, (w(o.phyts), w(n.phyts)), (w(o.phyt_at()), w(n.phyt_at())));
        e.copy_buffer_to_buffer(&s.lists, 0, &t.lists, 0, 4 * TALLY);
        let lists = [
            (SLEEPERS, o.sleepers, n.sleepers, o.sleep_at(), n.sleep_at()),
            (2, o.next, n.next, o.next_at(), n.next_at()),
            (WOKEN, o.woken, n.woken, o.woken_at(), n.woken_at()),
        ];
        for (cols, a, b, at_a, at_b) in lists {
            columns(e, (&s.lists, &t.lists), cols, (w(a), w(b)), (w(at_a), w(at_b)));
        }
        let (h, g) = (&self.hier, &into.hier);
        let (hs, gs) = (w(h.stride), w(g.stride));
        columns(e, (&h.shape, &g.shape), SHAPE, (hs, gs), (0, 0));
        columns(e, (&h.counts, &g.counts), COUNTS, (hs, gs), (0, 0));
        for k in 0..2 {
            columns(e, (&h.lists[k], &g.lists[k]), LISTS, (hs + 64, gs + 64), (0, 0));
        }
        e.copy_buffer_to_buffer(&h.tally, 0, &g.tally, 0, h.tally.size());
        e.copy_buffer_to_buffer(census.0, 0, census.1, 0, census.0.size());
    }
}
