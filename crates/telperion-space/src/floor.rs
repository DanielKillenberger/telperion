//! What fn-321's route rests on, measured: an axis's order, and the light's
//! cost on a tree cut at an order with the rest standing as density
//! (fn-323's manifest). A replay on a tree grown whole, for the measuring
//! code alone: no tree is grown through it.
use crate::light::{leaf_area, Field};
use crate::structure::{Axis, Origin, Vec3};
use crate::{Light, Result, Species};

/// Each axis's count of lateral buds from the seed: a continuation and a
/// relay keep their parent's.
pub fn orders(axes: &[Axis]) -> Vec<u32> {
    let mut out = Vec::with_capacity(axes.len());
    for a in axes {
        let order = match (a.origin.parent(), &a.origin) {
            (None, _) => 0,
            (Some(p), Origin::Lateral { .. }) => out[p] + 1,
            (Some(p), _) => out[p],
        };
        out.push(order);
    }
    out
}

/// Each phytomer's order, by the key a drawn node carries as its identity.
pub fn orders_by_key(axes: &[Axis]) -> std::collections::HashMap<u64, u32> {
    let order = orders(axes);
    let phytomers = axes.iter().zip(order).flat_map(|(a, o)| a.phytomers.iter().map(move |p| (p.key, o)));
    phytomers.collect()
}

/// What one replay read, and its seconds.
#[derive(Debug, Clone, Copy)]
pub struct Replay {
    pub seconds: f64,
    pub leaves: usize,
    pub tips: usize,
    pub nodes: usize,
}

/// A cut subtree's leaves: their area, and their area-weighted sum and sum
/// of squares, each from the origin.
#[derive(Clone, Copy, Default)]
struct Cut {
    area: f64,
    sum: [f64; 3],
    squares: f64,
}

/// The light of `year` on `axes`, finished, cut at order `k`, on one thread:
/// the kept axes' leaves plus 8 points for each subtree cut, read at the kept
/// axes' living tips. An axis stands from its birth, at its finished place.
pub fn light_replay(
    species: &Species,
    light: &Light,
    axes: &[Axis],
    (k, year): (u32, u32),
) -> Result<Replay> {
    let order = orders(axes);
    let mut leaves = Vec::new();
    let mut tips = Vec::new();
    let mut root = vec![usize::MAX; axes.len()];
    let mut cuts = std::collections::BTreeMap::new();
    for (i, a) in axes.iter().enumerate() {
        if a.birth > year {
            continue;
        }
        let area = species.states[a.pa].leaf_area;
        let bearing = a.phytomers.iter().filter(|p| p.cycle <= year);
        if order[i] <= k {
            leaves.extend(bearing.map(|p| (p.tip, leaf_area(p.scale, p.size, area))));
            let living = a.apex_end.is_none_or(|end| end > year);
            if let (true, Some(p)) = (living, a.phytomers.iter().rfind(|p| p.cycle <= year)) {
                tips.push(p.tip);
            }
            continue;
        }
        let parent = a.origin.parent().expect("a cut axis has a parent");
        root[i] = if order[parent] <= k { i } else { root[parent] };
        let cut: &mut Cut = cuts.entry(root[i]).or_default();
        for p in bearing {
            let w = leaf_area(p.scale, p.size, area);
            cut.area += w;
            let at = [p.tip.x, p.tip.y, p.tip.z];
            for c in 0..3 {
                cut.sum[c] += w * at[c];
                cut.squares += w * at[c] * at[c];
            }
        }
    }
    for cut in cuts.values().filter(|c| c.area > 0.0) {
        let mean = cut.sum.map(|s| s / cut.area);
        let spread = (cut.squares / cut.area - mean.iter().map(|m| m * m).sum::<f64>())
            .max(0.0)
            .sqrt()
            / 3f64.sqrt();
        for corner in 0..8u32 {
            let side = |bit: u32| if corner >> bit & 1 == 1 { spread } else { -spread };
            let p = Vec3::new(mean[0] + side(0), mean[1] + side(1), mean[2] + side(2));
            leaves.push((p, cut.area / 8.0));
        }
    }
    let started = crate::clock::seconds();
    let field = Field::quantised(light, &leaves, &tips, 1)?;
    let read: f64 = tips.iter().map(|t| field.at(*t)).sum();
    let seconds = crate::clock::seconds() - started;
    assert!(read.is_finite(), "the replay's light is a number");
    Ok(Replay {
        seconds,
        leaves: leaves.len(),
        tips: tips.len(),
        nodes: field.nodes(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grow_cycle, oak, Request};

    fn grown() -> (Species, Light, Vec<Axis>) {
        let species = oak();
        let light = Light::default();
        let request = Request {
            age: 12,
            seed: 7,
            budget: 20_000_000,
            light,
        };
        let axes = grow_cycle(&species, request).expect("a tree").structure.axes;
        (species, light, axes)
    }

    #[test]
    fn a_lateral_is_an_order_deeper_and_a_continuation_is_not() {
        let (_, _, axes) = grown();
        let order = orders(&axes);
        assert_eq!(order[0], 0);
        for (i, a) in axes.iter().enumerate().skip(1) {
            let parent = order[a.origin.parent().expect("a parent")];
            let deeper = matches!(a.origin, Origin::Lateral { .. });
            assert_eq!(order[i], parent + u32::from(deeper), "axis {i}");
        }
        assert!(order.iter().any(|&o| o > 1), "a tree of 12 years branches");
    }

    #[test]
    fn a_tighter_cut_reads_fewer_tips_and_deposits_fewer_leaves() {
        let (species, light, axes) = grown();
        let at = |k| light_replay(&species, &light, &axes, (k, 12)).expect("a replay");
        let (one, all) = (at(1), at(u32::MAX));
        assert!(one.tips < all.tips, "{one:?} against {all:?}");
        assert!(one.leaves < all.leaves, "{one:?} against {all:?}");
    }

    #[test]
    fn an_axis_not_yet_born_stands_in_no_year() {
        let (species, light, axes) = grown();
        let early = light_replay(&species, &light, &axes, (u32::MAX, 3)).expect("a replay");
        let late = light_replay(&species, &light, &axes, (u32::MAX, 12)).expect("a replay");
        assert!(early.leaves < late.leaves, "{early:?} against {late:?}");
    }
}
