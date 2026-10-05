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

/// Each axis's bud's vigour, from the base's vigour, the light its own bud
/// collects (`own`, 0 where it has none) and its apical control λ at its
/// branching points. Parents precede children. The base's vigour is the
/// tree's light, α = 1: a bud's size reads its vigour against a uniformly
/// lit tree's, in which α cancels (STEP3C.md).
pub(crate) fn vigours(links: &[Link], own: &[f64], lambda: &[f64]) -> Vec<f64> {
    let n = links.len();
    let mut sub = own.to_vec();
    for i in (0..n).rev() {
        if let Some(p) = links[i].parent {
            sub[p] += sub[i];
        }
    }
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
    let mut incoming = vec![0.0; n];
    let mut bud = vec![0.0; n];
    for i in 0..n {
        if links[i].parent.is_none() {
            incoming[i] = sub[i];
        }
        let (mut v, mut above) = (incoming[i], sub[i]);
        if v <= 0.0 || above <= 0.0 {
            continue;
        }
        let mine = &kids[start[i]..start[i + 1]];
        let mut k = 0;
        while k < mine.len() && mine[k].1 != TIP {
            let node = mine[k].1;
            let end = k + mine[k..].iter().take_while(|c| c.1 == node).count();
            let lateral: f64 = mine[k..end].iter().map(|c| sub[c.2]).sum();
            let main = (above - lateral).max(0.0);
            let l = lambda[i];
            let split = l * main + (1.0 - l) * lateral;
            if split > 0.0 {
                for c in &mine[k..end] {
                    incoming[c.2] = v * (1.0 - l) * sub[c.2] / split;
                }
                v *= l * main / split;
            }
            above = main;
            k = end;
        }
        // At the tip the axis's own bud and what carries it on share what
        // is left by their light.
        if above > 0.0 {
            bud[i] = v * own[i] / above;
            for c in &mine[k..] {
                incoming[c.2] = v * sub[c.2] / above;
            }
        }
    }
    bud
}

#[cfg(test)]
mod tests;
