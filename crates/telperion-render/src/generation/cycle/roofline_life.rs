//! fn-256 task 7: what each cycle's phases visit, from the CPU's grower
//! (the device's reference, agreement held by lineage): the axes held,
//! those whose subtree grew, the apexes that grew and the phytomers laid,
//! each also counted on laterals that lived under 3, 5 and 10 years. A
//! lateral's life is the cycles its apex grew in, first to last, across
//! its continuations and relays; a lateral whose apex still lives at the
//! tree's age is never short-lived.
use super::columns::{Cap, NONE};
use telperion_space::{grow_capture_grown, grow_capture_retired, Request, Species};

/// The lives the shares are taken under.
pub(super) const UNDER: [u32; 3] = [3, 5, 10];

/// One cycle's visits, each `[all, under 3, under 5, under 10]`.
#[derive(Debug, Default, Clone)]
pub(super) struct Visits {
    pub held: [u64; 4],
    /// Held axes whose subtree grew in the cycle (its own apex included).
    pub changed: u64,
    /// Held axes whose own apex had stopped by the cycle's start.
    pub stopped: u64,
    pub grew: [u64; 4],
    /// Apexes living at the cycle's start: made before it, stopped in it
    /// or later.
    pub live: [u64; 4],
    pub laid: [u64; 4],
}

/// The tree's lives and each cycle's visits.
pub(super) struct Lives {
    pub phytomers: [u64; 4],
    pub laterals: [u64; 4],
    pub cycles: Vec<Visits>,
}

/// What `species` grown by `request` visits, its axes let go at the
/// compactions made after the cycles in `compacted`; `let_go` whether a
/// compaction lets a retired subtree's axes go (no PA keeps records).
pub(super) fn lives(species: &Species, request: Request, compacted: &[u32], let_go: bool) -> Lives {
    let grown = grow_capture_grown(species, request).expect("the reference");
    let g = Cap(&grown);
    let (birth, origin, parent, end) = (g.u32s("g_birth"), g.u32s("g_origin"), g.u32s("g_parent"), g.u32s("g_apex_end"));
    let (p_axis, p_cycle) = (g.u32s("g_phyt_axis"), g.u32s("g_phyt_cycle"));
    let n = birth.len();
    // An axis's row is made at its birth, a woken bud's at its waking.
    let mut made = birth.clone();
    for (a, c) in g.u32s("g_woken_axis").into_iter().zip(g.u32s("g_woken_cycle")) {
        made[a as usize] = c;
    }
    // Each axis's lateral: itself, or the one it carries on or relays.
    let mut branch = vec![0usize; n];
    for a in 0..n {
        branch[a] = if origin[a] & 0xff >= 2 { branch[parent[a] as usize] } else { a };
    }
    let (mut first, mut last, mut alive) = (vec![u32::MAX; n], vec![0u32; n], vec![false; n]);
    for (&a, &c) in p_axis.iter().zip(&p_cycle) {
        let b = branch[a as usize];
        (first[b], last[b]) = (first[b].min(c), last[b].max(c));
    }
    (0..n).filter(|&a| end[a] == 0).for_each(|a| alive[branch[a]] = true);
    // Each axis's lateral's class: 0 none, else the first life it is under.
    let class: Vec<usize> = (0..n)
        .map(|a| {
            let b = branch[a];
            let lateral = origin[b] & 0xff == 1 && !alive[b] && first[b] != u32::MAX;
            let life = last[b].saturating_sub(first[b]) + 1;
            UNDER.iter().position(|&u| lateral && life < u).map_or(0, |k| k + 1)
        })
        .collect();
    let count = |into: &mut [u64; 4], k: usize| {
        into[0] += 1;
        if k > 0 {
            (k..4).for_each(|j| into[j] += 1);
        }
    };
    let mut l = Lives { phytomers: [0; 4], laterals: [0; 4], cycles: vec![Visits::default(); request.age as usize + 1] };
    for (&a, &c) in p_axis.iter().zip(&p_cycle) {
        count(&mut l.phytomers, class[a as usize]);
        count(&mut l.cycles[c as usize].laid, class[a as usize]);
    }
    (0..n).filter(|&a| origin[a] & 0xff == 1 && first[a] != u32::MAX).for_each(|a| count(&mut l.laterals, class[a]));
    let gone = if let_go { retired_at(species, request, &grown, compacted, &parent) } else { vec![u32::MAX; n] };
    // Each axis that grew in a cycle, by the cycles it grew in.
    let mut grew: Vec<Vec<u32>> = vec![Vec::new(); request.age as usize + 1];
    for (&a, &c) in p_axis.iter().zip(&p_cycle) {
        if grew[c as usize].last() != Some(&a) {
            grew[c as usize].push(a);
        }
    }
    let mut mark = vec![0u32; n];
    for c in 1..=request.age {
        let v = &mut l.cycles[c as usize];
        for &a in &grew[c as usize] {
            count(&mut v.grew, class[a as usize]);
            let mut x = a;
            while x != NONE && mark[x as usize] != c {
                mark[x as usize] = c;
                x = parent[x as usize];
            }
        }
        for a in (0..n).filter(|&a| made[a] <= c && c < gone[a]) {
            count(&mut v.held, class[a]);
            v.changed += u64::from(mark[a] == c);
            v.stopped += u64::from(end[a] != 0 && end[a] - 1 < c);
            if made[a] < c && (end[a] == 0 || c < end[a]) {
                count(&mut v.live, class[a]);
            }
        }
    }
    l
}

/// The first cycle each axis is no longer held: the cycle after the first
/// compaction at or after the verdict of a retired subtree it is inside.
fn retired_at(species: &Species, request: Request, grown: &telperion_space::Capture, compacted: &[u32], parent: &[u32]) -> Vec<u32> {
    let retired = grow_capture_retired(species, request).expect("the retirement");
    let r = Cap(&retired);
    assert_eq!(r.u32s("r_lineage"), Cap(grown).u64s("g_lineage").iter().flat_map(|&k| [k as u32, (k >> 32) as u32]).collect::<Vec<_>>(), "the same axes");
    let n = parent.len();
    let mut root = vec![u32::MAX; n];
    for w in r.u32s("r_log").chunks(5) {
        if w[2] == 2 {
            root[w[0] as usize] = w[1];
        }
    }
    // Inside a retired subtree from its root's verdict, the root's own row
    // kept (its living stands in its bearer's fade): parents come first.
    let mut inside = vec![u32::MAX; n];
    for a in 0..n {
        if parent[a] != NONE {
            let p = parent[a] as usize;
            inside[a] = inside[p].min(root[p]);
        }
    }
    inside
        .into_iter()
        .map(|v| compacted.iter().find(|&&k| k >= v).map_or(u32::MAX, |k| k + 1))
        .collect()
}
