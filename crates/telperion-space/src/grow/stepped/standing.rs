//! The coarse tree's run of the stepped executor (fn-214 task 2): the
//! structure above the cut grown through every cycle with each fine root
//! standing as its aggregate (`stands.rs`), then handed to the coarse
//! field's settle as grown, marked and assigned.
use super::stands::{Stands, Stood};
use super::{Counts, Executor};
use crate::closed_form::boundary::Rows;
use crate::contract::presence::{q, real, times};
use crate::error::{refuse, Result};
use crate::fine::Made;
use crate::grow::realized::Assigned;
use crate::grow::{start, Options, Request, Rules};
use crate::species::Species;
use std::sync::Arc;

/// The structure above the cut as grown, marked and assigned, its rules,
/// where each root stands on it (its placeholder and its bearer's node),
/// and each root as it stood.
pub(crate) struct Standing<'s> {
    pub rules: Rules<'s>,
    pub grown: Assigned,
    pub seats: Vec<(usize, usize)>,
    pub roots: Vec<Stood>,
}

/// Grows `species` above `cut` (the closed form's expected kept
/// phytomers, from `rows`) with every fine root standing as its
/// aggregate. Refuses a species whose structure above the cut reads
/// nothing of its roots before the tree's end: it has no crossing for a
/// root to stand in.
pub(crate) fn stand(
    species: &Species,
    request: Request,
    rows: Arc<Rows>,
    cut: f64,
) -> Result<Standing<'_>> {
    let (coarse, _) = start(species, request, Options::default())?;
    let counts: Counts = {
        let rows = rows.clone();
        Arc::new(move |pa, birth| rows.lateral(pa, birth).count)
    };
    let stands = Stands {
        rows,
        roots: Vec::new(),
    };
    let mut x = Executor::new(coarse, (counts, cut), request, stands);
    if x.reach(1) >= request.age {
        return refuse(
            "cut",
            "a standing root needs a structure above the cut that reads it each cycle",
        );
    }
    for cycle in 1..=request.age {
        x.cycle(cycle)?;
    }
    Ok(x.standing())
}

impl<'s> Executor<'s> {
    /// The structure above the cut marked and assigned, each root's
    /// placeholder made as its root was, as the stepped settle makes it.
    fn standing(self) -> Standing<'s> {
        let Executor {
            mut coarse,
            stands,
            request,
            ..
        } = self;
        coarse.mark();
        let mut roots = stands.roots;
        // A root that woke keeps what its bearer kept of it.
        let mut held = vec![None; coarse.axes.len()];
        for (k, r) in roots.iter().enumerate() {
            held[r.at] = Some(k);
        }
        for w in &coarse.woken {
            if let Some(k) = held[w.axis] {
                let kept = w.kept(&coarse.axes, &coarse.draws, &coarse.successor, request.age);
                let made = &mut roots[k].birth[1];
                *made = real(times(q(*made), q(kept)));
            }
        }
        for r in &roots {
            coarse.draws[r.at].birth = r.birth;
        }
        let seats = (roots.iter())
            .map(|r| (r.at, Made::of(&r.axis).node))
            .collect();
        let rules = coarse.rules.clone();
        Standing {
            rules,
            grown: coarse.assigned(),
            seats,
            roots,
        }
    }
}
