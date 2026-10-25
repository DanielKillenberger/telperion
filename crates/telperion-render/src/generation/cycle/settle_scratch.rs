//! fn-256 task 2 (SETTLE-PLAN.md risk 4): the build's growth-only
//! buffers let go before the settle takes its own. Once the last cycle
//! has grown, the settle reads only the state and the layout's form:
//! the hierarchy, the sums, the tree view, the light and the layout's
//! lists and flags are dead, and destroyed at once (their bind groups
//! may hold them, so dropping alone would not free them).
use super::build::Build;
use crate::buffer::Owned;

impl Build {
    /// Destroys every buffer only growth reads: the bytes freed.
    pub(super) fn shed_growth(&self) -> u64 {
        let (h, m, l) = (&self.hier, &self.sums, &self.light);
        let dead: [&Owned; 20] = [
            &h.bucket,
            &h.sums,
            &m.sums,
            &m.live,
            &m.norm,
            &m.tally,
            &m.rows,
            &m.scan_sums,
            &self.tree,
            &l.points,
            &l.reads,
            &l.state,
            &l.grid,
            &l.quanta,
            &l.field,
            &l.depths,
            &l.start,
            &self.layout.lists,
            &self.layout.counts,
            &self.inputs,
        ];
        let rest = [
            &self.counts,
            &self.flags,
            &self.spare,
            &self.scan_tally,
            &self.scan_sums,
        ];
        // The hierarchy a build that grows on reads in its next cycle.
        let hier = [&h.shape, &h.counts, &h.lists[0], &h.lists[1], &h.tally];
        let hier = hier.into_iter().filter(|_| !self.goes_on);
        let mut freed = 0;
        for b in dead.into_iter().chain(rest).chain(hier) {
            freed += b.size();
            b.destroy();
        }
        freed
    }
}
