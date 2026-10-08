//! When each item steps (host decision on fn-211 task 12, PHASES.md):
//! from the cycle it is due to its next synchronisation point, the next
//! cycle at which anything crosses between it and the structure above.
//! The order of evaluation, not what is built: the growth digest each
//! cycle is rebuilt from the counts each item recorded as it ran.
use super::Executor;
use crate::grow::partition::Digest;

/// What an item's run gave: its growth digest's part each cycle, and the
/// cycle it died in, if it did.
#[derive(Default)]
pub(super) struct Ran {
    /// The item, the cycles it ran from and is next due, and the
    /// phytomers it grew.
    pub item: usize,
    pub from: u32,
    pub next: u32,
    pub grown: u64,
    pub parts: Vec<Digest>,
    pub died: Option<u32>,
}

impl Executor<'_> {
    /// The last cycle an item stepping in `cycle` runs to before anything
    /// crosses between it and the structure above (host decision on task
    /// 12, PHASES.md): its light and layout at a cycle's end, and
    /// the balance or allotment reading its wood at a cycle's start.
    pub(super) fn reach(&self, cycle: u32) -> u32 {
        let c = &self.coarse;
        let before = c.balances() || c.allots();
        let after = |k: u32| c.sketch.is_some() && k < self.request.age;
        let mut last = cycle;
        while last < self.request.age && !after(last) && !before {
            last += 1;
        }
        last
    }

    /// An item's run from `cycle` kept: its growth digest's part each
    /// cycle it ran, and, where it died, that part for every cycle after.
    pub(super) fn record(&mut self, ran: Ran) {
        let age = self.request.age as usize;
        if self.grown_parts.len() < age + 2 {
            self.grown_parts.resize(age + 2, Digest::default());
            self.burials.resize(age + 2, Digest::default());
        }
        for (k, part) in ran.parts.iter().enumerate() {
            let c = ran.from as usize + k;
            self.grown_parts[c] = self.grown_parts[c].and(*part);
        }
        if let (Some(d), Some(last)) = (ran.died, ran.parts.last()) {
            self.burials[d as usize + 1] = self.burials[d as usize + 1].and(*last);
        }
    }

    /// Phase `k`'s hash over every grower: the item's stub and root are
    /// counted once, as the coarse grower's placeholder.
    pub(super) fn mark(&mut self, cycle: u32, k: usize) {
        if self.keep == super::Keep::Tree {
            return;
        }
        let mut d = self.coarse.digest(k);
        let c = cycle as usize;
        if self.phases.len() <= c {
            self.phases.resize(c + 1, [0; 4]);
            self.coarse_grown.resize(c + 1, None);
        }
        if k == 2 {
            // The items' part is known once each has run (`grown`).
            self.coarse_grown[c] = Some(d);
        } else {
            for item in self.items.iter().filter(|i| i.next != u32::MAX) {
                let mut part = item.g.digest(k);
                part.counts[0] -= 2;
                d = d.and(part);
            }
        }
        if k != 2 {
            self.phases[c][k] = d.finish(k);
        }
        if self.keep == super::Keep::States {
            if self.states.len() <= c {
                self.states.resize(c + 1, [0; 4]);
            }
            self.states[c][k] = self.state();
        }
    }

    /// Each cycle's growth digest, once every item has run: the coarse
    /// grower's, the dead items' and each item's as it ran.
    pub(super) fn grown(&mut self) {
        let mut buried = Digest::default();
        for (c, coarse) in self.coarse_grown.iter().enumerate() {
            if let Some(burial) = self.burials.get(c) {
                buried = buried.and(*burial);
            }
            if let Some(d) = coarse {
                let ran = self.grown_parts.get(c).copied().unwrap_or_default();
                self.phases[c][2] = d.and(buried).and(ran).finish(2);
            }
        }
    }
}
