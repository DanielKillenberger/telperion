//! Whole-tree allocation (fn-197, host decision 11): the extended
//! Borchert–Honda model as Pałubicki et al. (2009, 4.2) describe it. A
//! basipetal pass sums the light each axis's subtree collects; an acropetal
//! pass splits the base's vigour at every branching point between the
//! continuing axis and its laterals, v_main = v λ Q_m / (λ Q_m + (1 - λ)
//! Q_l), each lateral the rest by its Q, and gives each bud its share. The
//! whole tree's vigour is conserved. At λ = 0.5 every split is in
//! proportion to light, so each bud's vigour is its own light's share of
//! the tree's: the split is unbiased. Every sum is of presence-weighted
//! light, and no bud is excluded by a threshold.

/// Where an axis stands on its bearer: its parent's index and the node it
/// grows from, `TIP` where it carries the parent on (none for the root).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Link {
    pub parent: Option<usize>,
    pub node: usize,
}

/// A link's node where it carries its parent on: after every node.
pub(crate) const TIP: usize = usize::MAX;

use crate::contract::{exp2, log2, INF};

/// Each axis's bud's vigour, from the base's vigour, the light its own bud
/// collects (`own`, 0 where it has none) and its apical control λ at its
/// branching points. Parents precede children. The base's vigour is the
/// tree's light, α = 1: a bud's size reads its vigour per presence against
/// the tree's mean (`sizes`), in which any α cancels, so α is no setting
/// (host decision 15).
#[cfg(test)]
pub(crate) fn vigours(links: &[Link], own: &[f64], lambda: &[f64]) -> Vec<f64> {
    vigours_from(links, own, lambda, None)
}

/// `vigours`, the base's vigour `base` where given (fn-211.4: a fine
/// subtree's root's recorded share), its light where not.
pub(crate) fn vigours_from(
    links: &[Link],
    own: &[f64],
    lambda: &[f64],
    base: Option<f64>,
) -> Vec<f64> {
    split(links, own, lambda, base).1
}

/// Each axis's incoming vigour, and its own bud's: what `vigours_from`
/// gives, with the vigour each subtree receives at its base (fn-211.4:
/// what a fine root's bearer hands it).
pub(crate) fn split(
    links: &[Link],
    own: &[f64],
    lambda: &[f64],
    base: Option<f64>,
) -> (Vec<f64>, Vec<f64>) {
    let n = links.len();
    let own: Vec<f32> = own.iter().map(|&x| x as f32).collect();
    let sub = subtotals_f(links, &own);
    // Each axis's children with light, in the order they stand on it.
    let mut kids: Vec<(usize, usize, usize)> = (0..n)
        .filter(|&i| sub[i] > 0.0)
        .filter_map(|i| links[i].parent.map(|p| (p, links[i].node, i)))
        .collect();
    kids.sort_unstable();
    let mut start = vec![kids.len(); n + 1];
    for (k, &(p, _, _)) in kids.iter().enumerate().rev() {
        start[p] = k;
    }
    for p in (0..n).rev() {
        start[p] = start[p].min(start[p + 1]);
    }
    let mut incoming = vec![0.0f32; n];
    let mut bud = vec![0.0f32; n];
    for i in 0..n {
        if links[i].parent.is_none() {
            incoming[i] = base.map_or(sub[i], |b| b as f32);
        }
        let mut v = incoming[i];
        if v <= 0.0 || sub[i] <= 0.0 {
            continue;
        }
        let l = lambda[i] as f32;
        let mine = &kids[start[i]..start[i + 1]];
        // What stands above each child on the axis, summed from the tip
        // down, never the total less what branched off: in f32 a faint
        // continuation would round away against a bright lateral (fn-254).
        let mut rest = vec![own[i]; mine.len() + 1];
        for j in (0..mine.len()).rev() {
            rest[j] = rest[j + 1] + sub[mine[j].2];
        }
        let mut k = 0;
        while k < mine.len() && mine[k].1 != TIP {
            let node = mine[k].1;
            let end = k + mine[k..].iter().take_while(|c| c.1 == node).count();
            let lateral: f32 = mine[k..end].iter().map(|c| sub[c.2]).sum();
            let main = rest[end];
            let split = l * main + (1.0 - l) * lateral;
            if split > 0.0 {
                for c in &mine[k..end] {
                    incoming[c.2] = v * (1.0 - l) * sub[c.2] / split;
                }
                v = v * l * main / split;
            }
            k = end;
        }
        let above = rest[k];
        // At the tip the axis's own bud and what carries it on share what
        // is left by their light.
        if above > 0.0 {
            bud[i] = v * own[i] / above;
            for c in &mine[k..] {
                incoming[c.2] = v * sub[c.2] / above;
            }
        }
    }
    let reals = |v: Vec<f32>| v.into_iter().map(f64::from).collect();
    (reals(incoming), reals(bud))
}

/// Each axis's subtree's light, child to parent, each child added to its
/// parent last first.
fn subtotals_f(links: &[Link], own: &[f32]) -> Vec<f32> {
    let mut sub = own.to_vec();
    for i in (0..links.len()).rev() {
        if let Some(p) = links[i].parent {
            sub[p] += sub[i];
        }
    }
    sub
}

/// Each axis's subtree's light, child to parent: what a fine root reports
/// its bearer (PHASES.md, phase (ii)).
pub(crate) fn subtotals(links: &[Link], own: &[f64]) -> Vec<f64> {
    let own: Vec<f32> = own.iter().map(|&x| x as f32).collect();
    subtotals_f(links, &own).into_iter().map(f64::from).collect()
}

/// Each bud's size from its vigour per presence against the mean of the
/// buds of its own order (host decisions 14 and 23): (r / r̄_g)^ψ, r =
/// vigour / presence, r̄_g the presence-weighted mean over the buds of its
/// group `group` (its PA). Normalised, so each group's presence-weighted
/// mean size is 1 at every ψ, and a group of one bud is whole: light moves
/// growth within an order, never from one order to another. Buds of no
/// presence are whole and weigh nothing.
pub(crate) fn sizes(vigour: &[f64], presence: &[f64], psi: &[f64], group: &[usize]) -> Vec<f64> {
    let groups = group.iter().copied().max().map_or(0, |g| g + 1);
    let sums = size_sums(vigour, presence, psi, group, groups);
    sized(vigour, presence, psi, group, &sums)
}

/// A bud's (v/p)^ψ: `2^(ψ (log2 v - log2 p))`; whole where it has no
/// presence or ψ is none; none where it has no vigour.
pub(crate) fn drawn(vigour: f32, presence: f32, psi: f32) -> f32 {
    if presence <= 0.0 || psi == 0.0 {
        return 1.0;
    }
    if vigour <= 0.0 {
        return if psi > 0.0 { 0.0 } else { INF };
    }
    exp2(psi * (log2(vigour) - log2(presence)))
}

/// Each of `groups` groups' Σp and Σp(v/p)^ψ, summed in the buds' order,
/// so a partition's partials merge in its order (fn-211.4).
pub(crate) fn size_sums(
    vigour: &[f64],
    presence: &[f64],
    psi: &[f64],
    group: &[usize],
    groups: usize,
) -> Vec<[f32; 2]> {
    let mut sums = vec![[0.0f32; 2]; groups];
    for i in 0..vigour.len() {
        let [total, weighted] = &mut sums[group[i]];
        let p = presence[i] as f32;
        *total += p;
        *weighted += drawn(vigour[i] as f32, p, psi[i] as f32) * p;
    }
    sums
}

/// A bud's size against its group's `[Σp, Σp(v/p)^ψ]`: `drawn Σp / Σp
/// drawn`; whole where it has no presence or the weighted sum is none.
pub(crate) fn size_of(vigour: f32, presence: f32, psi: f32, [total, weighted]: [f32; 2]) -> f32 {
    if presence <= 0.0 || weighted <= 0.0 {
        return 1.0;
    }
    drawn(vigour, presence, psi) * total / weighted
}

/// Each bud's size against its group's sums.
pub(crate) fn sized(
    vigour: &[f64],
    presence: &[f64],
    psi: &[f64],
    group: &[usize],
    sums: &[[f32; 2]],
) -> Vec<f64> {
    (0..vigour.len())
        .map(|i| {
            let q = |x: &[f64]| x[i] as f32;
            f64::from(size_of(q(vigour), q(presence), q(psi), sums[group[i]]))
        })
        .collect()
}

#[cfg(test)]
mod tests;
