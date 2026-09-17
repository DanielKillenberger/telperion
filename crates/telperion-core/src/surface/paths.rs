use super::{filled, reserved};
use crate::{math::Vec3, tree::Node, Error, Result};
pub(super) struct Run {
    pub start: usize,
    pub end: usize,
    pub trunk: bool,
}
pub(super) struct Paths {
    pub nodes: Vec<usize>,
    pub runs: Vec<Run>,
    /// Every node where a clump's stems part above the ground, and the run
    /// through it carries on into the straighter of them.
    pub forks: Vec<bool>,
}
pub(super) fn paths(nodes: &[Node]) -> Result<Paths> {
    let n = nodes.len();
    let mut result = Paths {
        nodes: reserved(
            n.checked_mul(2)
                .ok_or(Error::ResourceLimit("surface paths"))?,
        )?,
        runs: reserved(n)?,
        forks: filled(n, false)?,
    };
    if n < 2 {
        return Ok(result);
    }
    let mut stands = filled(n, 0)?;
    let mut first = filled(n, usize::MAX)?;
    let mut last = filled(n, usize::MAX)?;
    let mut next = filled(n, usize::MAX)?;
    let mut leaders = filled(n, usize::MAX)?;
    for i in 1..n {
        let at = stands[nodes[i].parent.unwrap() as usize];
        if nodes[i].position.distance(nodes[at].position) > 1e-9 {
            stands[i] = i;
            if first[at] == usize::MAX {
                first[at] = i;
            } else {
                next[last[at]] = i;
            }
            last[at] = i;
            if leaders[at] == usize::MAX || nodes[i].start_radius > nodes[leaders[at]].start_radius
            {
                leaders[at] = i;
            }
        } else {
            stands[i] = at;
        }
    }
    for at in (1..n).filter(|&at| stands[at] == at && nodes[at].stem) {
        let before = nodes[stands[nodes[at].parent.unwrap() as usize]].position;
        let child = |k: usize| Some(k).filter(|&k| k != usize::MAX);
        let stems = std::iter::successors(child(first[at]), |&k| child(next[k]))
            .filter(|&k| nodes[k].stem)
            .map(|k| (k, nodes[k].position));
        if let Some(leader) = straightest(before, nodes[at].position, stems) {
            leaders[at] = leader;
            result.forks[at] = true;
        }
    }
    let mut seeds = reserved(n)?;
    if leaders[0] != usize::MAX {
        seeds.push((0, leaders[0]));
    }
    let mut k = first[0];
    while k != usize::MAX {
        if k != leaders[0] {
            seeds.push((0, k));
        }
        k = next[k];
    }
    let mut s = 0;
    while s < seeds.len() {
        let (attach, mut at) = seeds[s];
        let start = result.nodes.len();
        result.nodes.extend([attach, at]);
        while leaders[at] != usize::MAX {
            let leader = leaders[at];
            let mut k = first[at];
            while k != usize::MAX {
                if k != leader {
                    seeds.push((at, k));
                }
                k = next[k];
            }
            result.nodes.push(leader);
            at = leader;
        }
        result.runs.push(Run {
            start,
            end: result.nodes.len(),
            // Every run that leaves the root is a trunk run: a buried root
            // sample, the flare its own height earns it, and no fork socket
            // where it meets the ground. A tree on one stem has exactly one,
            // which is the run that was the trunk before stems were a row.
            // A stem a clump parts from its first above the ground leaves
            // wood and not the ground, so it is swept from the socket of the
            // trunk below the fork like any run that forks off, unburied.
            trunk: attach == 0,
        });
        s += 1;
    }
    Ok(result)
}

/// Where two or more stems leave a node on a stem above the root, the run
/// carries on into the one that turns least from the wood below it, the
/// first of them on a tie, and every other leaves it as a socketed run. Any
/// fewer stems and the run follows its widest child, as everywhere else.
/// `before` is where the wood below the node starts, `at` the node, and
/// `stems` its stem children in child order with their positions. The
/// contact query asks the same of the same positions, so both choose one run.
pub(crate) fn straightest<K>(
    before: Vec3,
    at: Vec3,
    stems: impl IntoIterator<Item = (K, Vec3)>,
) -> Option<K> {
    let mut stems = stems.into_iter();
    let below = (at - before).normalized();
    let turn = |p: Vec3| below.dot((p - at).normalized());
    let (mut best, p) = stems.next()?;
    let mut most = turn(p);
    let mut parted = false;
    for (k, p) in stems {
        parted = true;
        let straightness = turn(p);
        if straightness > most {
            (best, most) = (k, straightness);
        }
    }
    parted.then_some(best)
}
