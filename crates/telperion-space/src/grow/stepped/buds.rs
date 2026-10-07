//! Roots handed over at zero cycles (PHASES.md, decision 2 as amended on
//! fn-211 task 12). A handover is a root with whatever it has grown so
//! far. Where nothing crosses between a root and the structure above
//! before the tree's end, its next synchronisation point is the end: the
//! structure above hands its bud over as it is made (`Hands`, as fine
//! growth's does) and never grows it, and at the end each root is grown
//! from its bud, marked, assigned and shed on one core. Its growth is
//! counted cycle by cycle for the growth digest, as an item's run is.
use super::schedule::Ran;
use super::{Executor, Keep};
use crate::error::Result;
use crate::fine::{Made, Root};
use crate::grow::alone::start;
use crate::grow::partition::Digest;
use crate::grow::realized::Hands;
use crate::grow::{Grower, Rules};
use std::sync::Arc;
use std::time::Instant;

/// The buds in a piece of the work spread over the cores, as fine
/// growth takes its roots (`fine::Plan`'s chunk).

/// A root grown from its bud: where it stands, the root settled through
/// phase 1, and its run.
pub(crate) struct Finished {
    pub at: usize,
    pub node: usize,
    pub root: Root,
}

impl Executor<'_> {
    /// Where nothing crosses before the tree's end, the structure above
    /// hands each root over as its bud.
    pub(super) fn hand_buds(&mut self) {
        let (age, cut) = (self.request.age, self.cut);
        if self.reach(1) < age {
            return;
        }
        let counts = self.counts.clone();
        let pas = self.coarse.species.states.len();
        let hands = Hands::new(pas, age, |pa, birth| counts(pa, birth) <= cut);
        self.coarse.rules.hands = Some(Arc::new(hands));
    }

    /// Each bud handed over grown to the tree's end on one core, marked,
    /// assigned and shed: the structure above marked first, as its
    /// buds' kept shares read it.
    pub(super) fn grow_buds(&mut self) -> Result<()> {
        if self.coarse.rules.hands.is_none() {
            return Ok(());
        }
        self.coarse.mark();
        self.marked = true;
        let handed = self.coarse.handed();
        let rules = Rules {
            alone: false,
            hands: None,
            grown: 0,
            ..self.coarse.rules.clone()
        };
        let (species, age, keep) = (self.coarse.species, self.request.age, self.keep);
        let threads = self.coarse.threads;
        // Spread over the cores as fine growth spreads its roots.
        let mut handed = handed;
        let plan = crate::fine::Plan {
            threads,
            ..crate::fine::Plan::default()
        };
        let done = crate::fine::each(
            &mut handed,
            plan,
            threads,
            &|_, h| -> Result<(Finished, Ran)> {
                let started = Instant::now();
                let (mut g, from, _) = start(rules.clone(), 0, Some(&h.bud))?;
                // Its state at the end of the cycle it was made in, then each
                // cycle it grows.
                let mut ran = Ran {
                    from: from - 1,
                    ..Ran::default()
                };
                let count = |g: &Grower, ran: &mut Ran| {
                    if keep != Keep::Tree {
                        let mut part: Digest = g.digest(2);
                        part.counts[0] -= 2;
                        ran.parts.push(part);
                    }
                };
                count(&g, &mut ran);
                for cycle in from..=age {
                    if g.idle(cycle) {
                        ran.died = Some(cycle - 1);
                        break;
                    }
                    g.step(cycle, &mut |_| {})?;
                    count(&g, &mut ran);
                }
                g.mark();
                let root = Root::shed(species, &rules, h, g.assigned(), started);
                let node = Made::of(&h.bud.axis).node;
                Ok((
                    Finished {
                        at: h.at,
                        node,
                        root,
                    },
                    ran,
                ))
            },
        );
        for d in done {
            let (finished, ran) = d?;
            self.record(ran);
            self.finished.push(finished);
        }
        Ok(())
    }
}
