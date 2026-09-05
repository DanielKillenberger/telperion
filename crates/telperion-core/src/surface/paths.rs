use super::{filled, reserved};
use crate::{tree::Node, Error, Result};
pub(super) struct Run {
    pub start: usize,
    pub end: usize,
    pub trunk: bool,
}
pub(super) struct Paths {
    pub nodes: Vec<usize>,
    pub runs: Vec<Run>,
}
pub(super) fn paths(nodes: &[Node]) -> Result<Paths> {
    let n = nodes.len();
    let mut result = Paths {
        nodes: reserved(
            n.checked_mul(2)
                .ok_or(Error::ResourceLimit("surface paths"))?,
        )?,
        runs: reserved(n)?,
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
            trunk: s == 0,
        });
        s += 1;
    }
    Ok(result)
}
