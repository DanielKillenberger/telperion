//! A cycle's growth on every core (fn-210, lever 2). The living apexes
//! are cut, in order, into shares; each share grows its apexes' axes
//! where they stand in the tree, which no other share touches, and
//! numbers the axes it makes as if it came first. The shares are merged
//! in order, each moved past the axes the ones before it made, so the
//! tree is the one grown in turn, however many cores grew it.
use super::shoot::{Home, Line, Reads, Shoot};
use super::sketch::Sketch;
use super::{Apex, Grower};
use crate::dormant::{Sleeper, Woken};
use crate::error::{Error, Result};
use crate::presence::Draws;
use crate::species::Species;
use crate::structure::{Axis, Origin};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// The living apexes in a share: enough work to pay for its merge.
pub(super) const SHARE: usize = 1024;

/// What a share of a cycle grew, let go of the tree.
pub(super) struct Share {
    /// The tree's count of axes when it began: the first index it made.
    base: usize,
    /// The axes it grew in the tree, whose successors may name axes it
    /// made.
    homes: Vec<usize>,
    made: Vec<Line>,
    next: Vec<Apex>,
    asleep: Vec<(usize, Sleeper)>,
    woken: Vec<Woken>,
    marked: Vec<u64>,
    grown: u64,
}

impl Shoot<'_, '_> {
    pub fn finish(self, homes: Vec<usize>) -> Share {
        Share {
            base: self.base,
            homes,
            made: self.made,
            next: self.next,
            asleep: self.asleep,
            woken: self.woken,
            marked: self.marked,
            grown: self.grown,
        }
    }

    /// Grows each of `apexes`' unit in `cycle`, its axis the home of the
    /// same place.
    fn grow(mut self, apexes: Vec<Apex>, cycle: u32) -> Result<Share> {
        let homes: Vec<usize> = apexes.iter().map(|a| a.axis).collect();
        for (k, apex) in apexes.into_iter().enumerate() {
            self.home = (k, apex.axis);
            self.advance(apex, cycle)?;
        }
        Ok(self.finish(homes))
    }
}

impl Grower<'_> {
    /// What the rough layout gives the apexes growing now.
    pub(super) fn reads(&self) -> Reads<'_> {
        reads(self.sketch.as_ref(), self.species)
    }

    /// Grows the `live` apexes' units in `cycle`, in shares in order, and
    /// merges them into the tree.
    pub(super) fn grow_live(&mut self, live: Vec<Apex>, cycle: u32) -> Result<()> {
        let reads = reads(self.sketch.as_ref(), self.species);
        let homes = homes(
            (&mut self.axes, &mut self.draws),
            (&mut self.units, &mut self.successor),
            &live,
        );
        let (rules, tree) = (&self.rules, (&self.roots[..], &self.horizon[..]));
        let shoot = |homes| Shoot {
            homes,
            ..Shoot::new(rules, reads, tree)
        };
        let shares = grow_shares(live, homes, (self.threads, self.share), &shoot, cycle)?;
        for share in shares {
            self.absorb(share);
        }
        if self.rules.grown > u64::from(self.rules.budget) {
            return Err(Error::Budget {
                limit: self.rules.budget,
            });
        }
        Ok(())
    }

    /// Merges a share into the tree, past the axes the shares before it
    /// made.
    pub(super) fn absorb(&mut self, mut share: Share) {
        let (base, offset) = (share.base, self.axes.len() - share.base);
        let moved = |i: &mut usize| {
            if *i >= base {
                *i += offset;
            }
        };
        if offset > 0 {
            share.shift(&moved);
            for &i in &share.homes {
                self.successor[i].iter_mut().for_each(moved);
            }
        }
        for line in share.made {
            self.axes.push(line.axis);
            self.draws.push(line.draws);
            self.units.push(line.units);
            self.successor.push(line.successor);
            self.roots.push(line.root);
            self.horizon.push(line.horizon);
        }
        self.live.extend(share.next);
        for (wakes, sleeper) in share.asleep {
            self.asleep[wakes].push(sleeper);
        }
        self.woken.extend(share.woken);
        self.marked.extend(share.marked);
        self.rules.grown += share.grown;
    }
}

impl Share {
    /// Names every axis it made where the tree will hold it.
    fn shift(&mut self, moved: &impl Fn(&mut usize)) {
        for line in &mut self.made {
            match &mut line.axis.origin {
                Origin::Seed => {}
                Origin::Lateral { parent, .. }
                | Origin::Continuation { parent }
                | Origin::Relay { parent, .. } => moved(parent),
            }
            line.successor.iter_mut().for_each(moved);
        }
        self.next.iter_mut().for_each(|a| moved(&mut a.axis));
        for (_, sleeper) in &mut self.asleep {
            moved(&mut sleeper.parent);
        }
        for woken in &mut self.woken {
            moved(&mut woken.axis);
            moved(&mut woken.bearer);
        }
    }
}

fn reads<'g>(sketch: Option<&'g Sketch>, species: &Species) -> Reads<'g> {
    Reads {
        pencils: sketch.map_or(&[], |s| &s.pencils[..]),
        sketching: sketch.is_some(),
        lights: species.states.iter().any(|s| s.leaf_girth > 0.0),
    }
}

/// The living apexes' axes where they stand, in the apexes' order: each
/// apex is its own axis, so they are apart.
fn homes<'g>(
    (axes, draws): (&'g mut [Axis], &'g mut [Draws]),
    (units, successor): (&'g mut [f64], &'g mut [Vec<usize>]),
    live: &[Apex],
) -> Vec<Home<'g>> {
    let mut order: Vec<(usize, usize)> =
        live.iter().enumerate().map(|(k, a)| (a.axis, k)).collect();
    order.sort_unstable();
    let mut picked: Vec<Option<Home<'g>>> = (0..live.len()).map(|_| None).collect();
    let (mut axes, mut draws, mut units, mut successor) = (axes, draws, units, successor);
    let mut at = 0;
    for &(i, k) in &order {
        let skip = i - at;
        let (axis, rest) = std::mem::take(&mut axes)[skip..]
            .split_first_mut()
            .expect("living");
        axes = rest;
        let (draw, rest) = std::mem::take(&mut draws)[skip..]
            .split_first_mut()
            .expect("living");
        draws = rest;
        let (unit, rest) = std::mem::take(&mut units)[skip..]
            .split_first_mut()
            .expect("living");
        units = rest;
        let (next, rest) = std::mem::take(&mut successor)[skip..]
            .split_first_mut()
            .expect("living");
        successor = rest;
        picked[k] = Some(Home {
            axis,
            draws: draw,
            units: unit,
            successor: next,
        });
        at = i + 1;
    }
    picked
        .into_iter()
        .map(|h| h.expect("every apex its own axis"))
        .collect()
}

/// Grows `live`, each apex with its home, in shares of `share` apexes on
/// up to `threads` cores: the shares in order.
fn grow_shares<'g, 'a: 'g>(
    live: Vec<Apex>,
    homes: Vec<Home<'g>>,
    (threads, share): (usize, usize),
    shoot: &(dyn Fn(Vec<Home<'g>>) -> Shoot<'g, 'a> + Sync),
    cycle: u32,
) -> Result<Vec<Share>> {
    let count = live.len().div_ceil(share);
    if threads <= 1 || count <= 1 {
        return Ok(vec![shoot(homes).grow(live, cycle)?]);
    }
    let (mut live, mut homes) = (live.into_iter(), homes.into_iter());
    let work: Vec<_> = (0..count)
        .map(|_| {
            let apexes: Vec<Apex> = live.by_ref().take(share).collect();
            let at: Vec<Home<'g>> = homes.by_ref().take(share).collect();
            Mutex::new(Some((apexes, at)))
        })
        .collect();
    let done: Vec<Mutex<Option<Result<Share>>>> = (0..count).map(|_| Mutex::new(None)).collect();
    let ticket = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..threads.min(count) {
            scope.spawn(|| loop {
                let k = ticket.fetch_add(1, Ordering::Relaxed);
                if k >= count {
                    break;
                }
                let (apexes, at) = work[k].lock().unwrap().take().expect("a share once");
                *done[k].lock().unwrap() = Some(shoot(at).grow(apexes, cycle));
            });
        }
    });
    done.into_iter()
        .map(|d| d.into_inner().unwrap().expect("every share grown"))
        .collect()
}
