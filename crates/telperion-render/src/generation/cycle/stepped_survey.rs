//! fn-302 R7: what each stepped limit needed against what the cycle
//! recorded, over every census a device build on this thread reads. A
//! stepped limit is projected from the census two cycles back; its rise
//! is how far a cycle's need passed that census's, its headroom what the
//! cycle recorded less what it needed (below 0 where the build passed it
//! and began again, `Growing::grow_to`).
use super::build_grow::retries;
use super::build_plan::Bounds;
use super::build_size::passes;
use super::census as c;
use super::gate::REC;
use super::plan_units::rounds_used;
use std::cell::RefCell;

thread_local! {
    /// Each census read on this thread and its cycle, in order.
    static TAP: RefCell<Vec<(u32, Vec<u32>)>> = const { RefCell::new(Vec::new()) };
}

/// The census of `cycle`, as a build read it.
pub(super) fn tap(cycle: u32, read: &[u32]) {
    TAP.with_borrow_mut(|t| t.push((cycle, read.to_vec())));
}

/// The stepped limits, in `Limits::words`' order but the levels' and the
/// layout's, which no build begins again for.
const NAMES: [&str; 8] = [
    "units ranges",
    "draw ranges",
    "deposit ranges",
    "sweeps",
    "hierarchy rounds",
    "placing passes",
    "living units rounds",
    "wake units rounds",
];

/// Each stepped limit's word in the census's recorded limits.
const WORDS: [usize; 8] = [0, 1, 2, 3, 4, 5, 9, 10];

/// What the census's cycle needed of each stepped limit: the counts its
/// flag holds to it (`plan_units`, `plan_lay`, `plan_hier`).
fn needs(r: &[u32]) -> [u32; 8] {
    let leaves = r[c::PHYTS].saturating_sub(r[c::PHYTS0]);
    let ranges = Bounds::default().ranges;
    [
        r[c::LIVE].div_ceil(ranges.units),
        r[c::GREW].div_ceil(ranges.draw),
        leaves.div_ceil(ranges.deposit),
        r[c::SWEPT] / super::build_light::LAYERS,
        r[c::GENS],
        passes(r),
        rounds_used(r, 0),
        rounds_used(r, 1),
    ]
}

/// The most and the least of a measure, and where each stood.
#[derive(Default)]
struct Extreme {
    most: Option<(i64, String)>,
    least: Option<(i64, String)>,
}

impl Extreme {
    fn add(&mut self, v: i64, at: impl Fn() -> String) {
        if self.most.as_ref().is_none_or(|m| v > m.0) {
            self.most = Some((v, at()));
        }
        if self.least.as_ref().is_none_or(|l| v < l.0) {
            self.least = Some((v, at()));
        }
    }
}

/// The survey over every build taken.
#[derive(Default)]
pub(in crate::generation) struct Survey {
    rise: [Extreme; 8],
    headroom: [Extreme; 8],
    builds: usize,
    censuses: usize,
    /// Each build that began again, and how often.
    pub retried: Vec<(String, u32)>,
}

impl Survey {
    /// The censuses tapped since the last build, as `name`'s: one
    /// attempt after another, each starting again at its first cycle.
    pub(in crate::generation) fn take(&mut self, name: &str) {
        let tapped = TAP.take();
        let again = retries();
        if again > 0 {
            self.retried.push((name.to_string(), again));
        }
        self.builds += 1;
        let mut attempts: Vec<Vec<(u32, Vec<u32>)>> = Vec::new();
        for (cycle, read) in tapped {
            match attempts.last_mut() {
                Some(a) if a.last().is_some_and(|l| l.0 < cycle) => a.push((cycle, read)),
                _ => attempts.push(vec![(cycle, read)]),
            }
        }
        for a in &attempts {
            self.attempt(name, a);
        }
    }

    fn attempt(&mut self, name: &str, a: &[(u32, Vec<u32>)]) {
        let need: Vec<[u32; 8]> = a.iter().map(|(_, r)| needs(r)).collect();
        self.censuses += a.len();
        for (k, (cycle, r)) in a.iter().enumerate() {
            for l in 0..8 {
                let at = || format!("{name} cycle {cycle}");
                let recorded = i64::from(r[REC + WORDS[l]]);
                // The first two cycles record the bounds, read nothing.
                if *cycle > 2 {
                    self.headroom[l].add(recorded - i64::from(need[k][l]), at);
                }
                if k >= 2 && a[k - 2].0 + 2 == *cycle {
                    let rise = i64::from(need[k][l]) - i64::from(need[k - 2][l]);
                    self.rise[l].add(rise, at);
                }
            }
        }
    }

    /// One line a stepped limit, then the builds that began again.
    pub(in crate::generation) fn report(&self) -> String {
        let mut out = format!(
            "{} builds, {} censuses, {} began again\n",
            self.builds,
            self.censuses,
            self.retried.len()
        );
        let shown = |e: &Option<(i64, String)>| {
            e.as_ref()
                .map_or("none".to_string(), |(v, at)| format!("{v} ({at})"))
        };
        let limits = NAMES.iter().zip(self.rise.iter().zip(&self.headroom));
        for (name, (rise, room)) in limits {
            out += &format!(
                "{name}: most rise {}, least headroom {}\n",
                shown(&rise.most),
                shown(&room.least)
            );
        }
        for (name, n) in &self.retried {
            out += &format!("began again {n}: {name}\n");
        }
        out
    }
}
