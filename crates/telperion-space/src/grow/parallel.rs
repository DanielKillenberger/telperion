//! A cycle's growth on every core (fn-210, lever 2). The living apexes
//! are cut, in order, into shares; each share grows its apexes' axes
//! where they stand in the tree, which no other share touches, and
//! numbers the axes it makes as if it came first. The shares are merged
//! in order, each moved past the axes the ones before it made, so the
//! tree is the one grown in turn, however many cores grew it.
use super::shoot::{Home, Line, Reads, Shoot, Spare};
use super::sketch::Sketch;
use super::{Apex, Grower, Pass, Stage};
use crate::cores::spread;
use crate::dormant::{Sleeper, Woken};
use crate::error::{Error, Result};
use crate::presence::Draws;
use crate::species::Species;
use crate::structure::{Axis, Origin};
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
    jobs: [u32; 2],
    /// Its lists to fill again: its apexes', emptied, and its own.
    spare: Spare,
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
            jobs: self.jobs,
            spare: Spare {
                next: Vec::new(),
                leads: self.leads,
                node_buds: self.node_buds,
                sleeping: self.sleeping,
            },
        }
    }

    /// Grows each of `apexes`' unit in `cycle`, its axis the home of the
    /// same place.
    fn grow(mut self, mut apexes: Vec<Apex>, cycle: u32) -> Result<Share> {
        let homes: Vec<usize> = apexes.iter().map(|a| a.axis).collect();
        for (k, apex) in apexes.drain(..).enumerate() {
            self.home = (k, apex.axis);
            self.job = 0;
            self.advance(apex, cycle)?;
            self.jobs[0] = self.jobs[0].max(self.job);
        }
        let mut share = self.finish(homes);
        share.spare.next = apexes;
        Ok(share)
    }
}

impl Grower<'_> {
    /// What the rough layout gives the apexes growing now.
    pub(super) fn reads(&self) -> Reads<'_> {
        reads(self.sketch.as_ref(), self.species)
    }

    /// Grows the `live` apexes' units in `cycle`, in shares in order, and
    /// merges them into the tree.
    pub(super) fn grow_live(
        &mut self,
        live: Vec<Apex>,
        cycle: u32,
        report: &mut dyn FnMut(Stage),
    ) -> Result<()> {
        let reads = reads(self.sketch.as_ref(), self.species);
        let homes = homes(
            (&mut self.axes, &mut self.draws),
            (&mut self.units, &mut self.successor),
            &live,
        );
        report(Stage::Pass(Pass::Homes));
        let (rules, tree) = (&self.rules, (&self.roots[..], &self.horizon[..]));
        let shoot = |homes| Shoot {
            homes,
            ..Shoot::new(rules, reads, tree)
        };
        let spare = std::mem::take(&mut self.spare);
        let at = (self.threads, self.share);
        let shares = grow_shares(live, homes, at, (&shoot, spare), cycle)?;
        report(Stage::Pass(Pass::Units));
        for share in shares {
            self.absorb(share);
        }
        report(Stage::Pass(Pass::Merge));
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
        if self.live.is_empty() {
            self.live = share.next;
        } else {
            self.live.extend(share.next);
        }
        for (wakes, sleeper) in share.asleep {
            self.asleep[wakes].push(sleeper);
        }
        self.woken.extend(share.woken);
        self.marked.extend(share.marked);
        self.rules.grown += share.grown;
        self.jobs = std::array::from_fn(|w| self.jobs[w].max(share.jobs[w]));
        self.spare = share.spare;
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
/// apex is its own axis, so they are apart. One walk up the four columns
/// picks them (a fine root makes this list each cycle it grows, so its
/// allocations count).
fn homes<'g>(
    (axes, draws): (&'g mut [Axis], &'g mut [Draws]),
    (units, successor): (&'g mut [f64], &'g mut [Vec<usize>]),
    live: &[Apex],
) -> Vec<Home<'g>> {
    let mut order: Vec<(usize, usize)> =
        live.iter().enumerate().map(|(k, a)| (a.axis, k)).collect();
    order.sort_unstable();
    let mut homes: Vec<Option<Home<'g>>> = (0..live.len()).map(|_| None).collect();
    let (mut rest, mut from) = ((axes, draws, units, successor), 0);
    for &(i, k) in &order {
        let (axes, draws, units, successor) = rest;
        let (axis, axes) = first(axes, i - from);
        let (draws, d) = first(draws, i - from);
        let (units, u) = first(units, i - from);
        let (successor, s) = first(successor, i - from);
        homes[k] = Some(Home {
            axis,
            draws,
            units,
            successor,
        });
        (rest, from) = ((axes, d, u, s), i + 1);
    }
    homes
        .into_iter()
        .map(|h| h.expect("every apex its own axis"))
        .collect()
}

/// `v[at]` and what follows it.
fn first<T>(v: &mut [T], at: usize) -> (&mut T, &mut [T]) {
    v[at..]
        .split_first_mut()
        .expect("an index within, increasing")
}

/// Grows `live`, each apex with its home, in shares of `share` apexes on
/// up to `threads` cores: the shares in order.
fn grow_shares<'g, 'a: 'g>(
    live: Vec<Apex>,
    homes: Vec<Home<'g>>,
    (threads, share): (usize, usize),
    (shoot, spare): (&(dyn Fn(Vec<Home<'g>>) -> Shoot<'g, 'a> + Sync), Spare),
    cycle: u32,
) -> Result<Vec<Share>> {
    let count = live.len().div_ceil(share);
    if threads <= 1 || count <= 1 {
        return Ok(vec![shoot(homes).spared(spare).grow(live, cycle)?]);
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
    spread(count, threads, &|k| {
        let (apexes, at) = work[k].lock().unwrap().take().expect("a share once");
        *done[k].lock().unwrap() = Some(shoot(at).grow(apexes, cycle));
    });
    done.into_iter()
        .map(|d| d.into_inner().unwrap().expect("every share grown"))
        .collect()
}
