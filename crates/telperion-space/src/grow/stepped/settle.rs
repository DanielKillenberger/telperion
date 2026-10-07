//! The stepped executor's settle (fn-211 task 12, host decision (b) on
//! TASK11.md): realized mechanics' four phases over the roots as their
//! own growers grew them. Each root's grower is marked, assigned and shed
//! alone (phase 1); the coarse grower's structure above the cut settles
//! with them through `fine::settle_grown`, whose exchange carries the
//! tree as grown where any PA keeps it. Every pass is the whole tree's
//! own and every sum it takes across the cut is the whole tree's, so the
//! tree is the whole tree's to the bit.
use super::{Executor, ITEMS};
use crate::error::Result;
use crate::fine::{assemble, settle_grown, Above, Clock, Cpu, Made, Plan, Root, Stages};
use crate::fixed::presence::{q, real, times};
use crate::grow::realized::Assigned;
use crate::presence::assign;
use crate::structure::Structure;
use std::time::Instant;

impl Executor<'_> {
    /// The tree settled and laid, each root settled by its own grower.
    pub(super) fn settle(self) -> Result<Structure> {
        let Executor {
            mut coarse,
            items,
            counts,
            request,
            ..
        } = self;
        let (species, age, threads) = (coarse.species, request.age, coarse.threads);
        coarse.mark();
        // A root that woke keeps what its bearer kept of it, as the
        // whole tree's mark sizes it.
        let mut held = vec![None; coarse.axes.len()];
        for (k, item) in items.iter().enumerate() {
            held[item.at] = Some(k);
        }
        let mut items = items;
        for w in &coarse.woken {
            if let Some(k) = held[w.axis] {
                let kept = w.kept(&coarse.axes, &coarse.draws, &coarse.successor, age);
                let made = &mut items[k].g.draws[1].birth[1];
                *made = real(times(q(*made), q(kept)));
            }
        }
        // Each stand-in made as its root was, so its bearer's fade carries
        // the root's living up by the root's own vigour.
        for item in &items {
            coarse.draws[item.at].birth = item.g.draws[1].birth;
        }
        let seats: Vec<(usize, usize)> = items
            .iter()
            .map(|i| (i.at, Made::of(&i.g.axes[1]).node))
            .collect();
        let grew: u64 = items.iter().map(|i| i.g.rules.grown).sum();
        let roots = crate::cores::pinned(items, ITEMS, threads, &|_, item| {
            let started = Instant::now();
            let mut g = item.g;
            g.mark();
            assign(&mut g.axes, &g.draws, species, 1);
            let made = Made::of(&g.axes[1]);
            let axes = std::mem::take(&mut g.axes);
            let assigned = Assigned {
                axes,
                grown: g.rules.grown,
            };
            Root::shed_grown(species, age, made, assigned, started)
        });
        let living = roots
            .iter()
            .map(|r| (r.living.lived, r.living.alive))
            .collect();
        let rules = coarse.rules.clone();
        let grown: Assigned = coarse.assigned();
        let plan = Plan {
            threads,
            ..Plan::default()
        };
        let mut cpu = Cpu {
            species,
            rules: None,
            plan,
            threads,
            roots,
        };
        let settled = settle_grown(
            species,
            (request, &rules, plan, threads),
            Above { grown, seats },
            (living, grew),
            &mut cpu,
            None,
            (Clock::default(), Stages::default()),
        )?;
        Ok(assemble(settled, cpu.roots, &*counts)?.structure)
    }
}
