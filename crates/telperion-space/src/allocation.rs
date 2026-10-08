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

use crate::fixed::feedback::total::Total;
use crate::fixed::feedback::wide::I256;
use crate::fixed::{exp2, log2, mul, q32, ONE32};

/// Contract A (CONTRACT-A.md 2.8): every input enters as its Q32, every
/// sum is exact, every product and quotient one truncation; every output
/// is the exact real of its Q32.
fn real32(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

/// `trunc(a b / d)` of Q32 values, Q32: one exact product, one division.
fn times_over(a: i64, b: i64, d: i64) -> i64 {
    ((i128::from(a) * i128::from(b)) / i128::from(d)) as i64
}

/// `trunc(a b c / (d 2^32))` of Q32 values, Q32.
fn thrice_over(a: i64, b: i64, c: i64, d: i64) -> i64 {
    let num = I256::mul(i128::from(a) * i128::from(b), i128::from(c));
    num.quotient(i128::from(d) << 32)
        .and_then(I256::narrow64)
        .expect("a vigour inside Q32")
}

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
    let own: Vec<i64> = own.iter().map(|&x| q32(x)).collect();
    let sub = subtotals_q(links, &own);
    // Each axis's children with light, in the order they stand on it.
    let mut kids: Vec<(usize, usize, usize)> = (0..n)
        .filter(|&i| sub[i] > 0)
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
    let mut incoming = vec![0i64; n];
    let mut bud = vec![0i64; n];
    for i in 0..n {
        if links[i].parent.is_none() {
            incoming[i] = base.map_or(sub[i], q32);
        }
        let (mut v, mut above) = (incoming[i], sub[i]);
        if v <= 0 || above <= 0 {
            continue;
        }
        let l = q32(lambda[i]);
        let mine = &kids[start[i]..start[i + 1]];
        let mut k = 0;
        while k < mine.len() && mine[k].1 != TIP {
            let node = mine[k].1;
            let end = k + mine[k..].iter().take_while(|c| c.1 == node).count();
            let lateral: i64 = mine[k..end].iter().map(|c| sub[c.2]).sum();
            let main = (above - lateral).max(0);
            let split = mul(l, main) + mul(ONE32 - l, lateral);
            if split > 0 {
                for c in &mine[k..end] {
                    incoming[c.2] = thrice_over(v, ONE32 - l, sub[c.2], split);
                }
                v = thrice_over(v, l, main, split);
            }
            above = main;
            k = end;
        }
        // At the tip the axis's own bud and what carries it on share what
        // is left by their light.
        if above > 0 {
            bud[i] = times_over(v, own[i], above);
            for c in &mine[k..] {
                incoming[c.2] = times_over(v, sub[c.2], above);
            }
        }
    }
    let reals = |v: Vec<i64>| v.into_iter().map(real32).collect();
    (reals(incoming), reals(bud))
}

/// Each axis's subtree's light, exact, child to parent.
fn subtotals_q(links: &[Link], own: &[i64]) -> Vec<i64> {
    let mut sub = own.to_vec();
    for i in (0..links.len()).rev() {
        if let Some(p) = links[i].parent {
            sub[p] = sub[p].checked_add(sub[i]).expect("a subtree's light");
        }
    }
    sub
}

/// Each axis's subtree's light, child to parent: what a fine root reports
/// its bearer (PHASES.md, phase (ii)).
pub(crate) fn subtotals(links: &[Link], own: &[f64]) -> Vec<f64> {
    let own: Vec<i64> = own.iter().map(|&x| q32(x)).collect();
    subtotals_q(links, &own).into_iter().map(real32).collect()
}

/// Each bud's size from its vigour per presence against the mean of the
/// buds of its own order (host decisions 14 and 23): (r / r̄_g)^ψ, r =
/// vigour / presence, r̄_g the presence-weighted mean over the buds of its
/// group `group` (its PA). Normalised exactly, so each group's
/// presence-weighted mean size is 1 at every ψ to Q32's last place, and
/// a group of one bud is whole to the bit: light moves growth within an
/// order, never from one order to another. Buds of no presence are whole
/// and weigh nothing.
pub(crate) fn sizes(vigour: &[f64], presence: &[f64], psi: &[f64], group: &[usize]) -> Vec<f64> {
    let groups = group.iter().copied().max().map_or(0, |g| g + 1);
    let sums = size_sums(vigour, presence, psi, group, groups);
    let sums: Vec<[i128; 2]> = sums.iter().map(|s| factors(s)).collect();
    sized(vigour, presence, psi, group, &sums)
}

/// A group's exact `[Σp, Σp(v/p)^ψ]` in W.
pub(crate) fn factors(s: &[Total; 2]) -> [i128; 2] {
    s.map(|t| t.get().expect("a group's sums inside W"))
}

/// A bud's (v/p)^ψ, Q32, of Q32 inputs: `2^(ψ (log2 v - log2 p))`; whole
/// where it has no presence or ψ is none; none where it has no vigour.
pub(crate) fn drawn(vigour: i64, presence: i64, psi: i64) -> i64 {
    if presence <= 0 || psi == 0 {
        return ONE32;
    }
    if vigour <= 0 {
        return if psi > 0 { 0 } else { crate::fixed::INF };
    }
    exp2(mul(psi, log2(vigour as u64) - log2(presence as u64)))
}

/// Each of `groups` groups' Σp and Σp(v/p)^ψ, exact (`Total`), so a
/// partition's partials merge to the tree's sums (fn-211.4).
pub(crate) fn size_sums(
    vigour: &[f64],
    presence: &[f64],
    psi: &[f64],
    group: &[usize],
    groups: usize,
) -> Vec<[Total; 2]> {
    let mut sums = vec![[Total::default(); 2]; groups];
    for i in 0..vigour.len() {
        let [total, weighted] = &mut sums[group[i]];
        let p = q32(presence[i]);
        total.add(i128::from(p) << 32);
        weighted.add(i128::from(drawn(q32(vigour[i]), p, q32(psi[i]))) * i128::from(p));
    }
    sums
}

/// A bud's size against its group's `[Σp, Σp(v/p)^ψ]` (W): `drawn Σp /
/// Σp drawn`, one product, one truncation; whole where it has no presence
/// or the weighted sum is none.
pub(crate) fn size_of(vigour: i64, presence: i64, psi: i64, [total, weighted]: [i128; 2]) -> i64 {
    if presence <= 0 || weighted <= 0 {
        return ONE32;
    }
    I256::mul(i128::from(drawn(vigour, presence, psi)), total)
        .quotient(weighted)
        .and_then(I256::narrow64)
        .expect("a size inside Q32")
}

/// A bud's size against a mean its group is given rather than summed
/// (fn-226 R4): `drawn / mean`, one truncation; whole where it has no
/// presence or the mean is none.
pub(crate) fn size_against(vigour: i64, presence: i64, psi: i64, mean: i64) -> i64 {
    if presence <= 0 || mean <= 0 {
        return ONE32;
    }
    let drawn = i128::from(drawn(vigour, presence, psi));
    ((drawn << 32) / i128::from(mean)).min(i128::from(crate::fixed::INF)) as i64
}

/// Each bud's size against its group's sums.
pub(crate) fn sized(
    vigour: &[f64],
    presence: &[f64],
    psi: &[f64],
    group: &[usize],
    sums: &[[i128; 2]],
) -> Vec<f64> {
    (0..vigour.len())
        .map(|i| {
            let q = |x: &[f64]| q32(x[i]);
            real32(size_of(q(vigour), q(presence), q(psi), sums[group[i]]))
        })
        .collect()
}

/// A real as W, the nearest (`fixed::sum`'s rule): exact for a Q32 real.
pub(crate) fn w_of(x: f64) -> i128 {
    (x * 18_446_744_073_709_551_616.0).round_ties_even() as i128
}

#[cfg(test)]
mod tests;
