//! A lateral's kind under shade (fn-354 R7, R7-DESIGN.md as the host set
//! it on 12 Nov). A bud of a PA whose `shade_short` sums to e > 0, formed
//! at its bearer's light L, keeps its kind with probability L^e and
//! otherwise becomes PA j with probability `shade_short[j]` / e. Both
//! draws are keyed under the bud's place (`TURN`), and none is taken where
//! e is 0. Each adds its lead past its bound to the bud's presence, so the
//! kind changes by degree as the light or a row walks.
use super::{bud, bud_presence, threshold, Key, TURN};
use crate::contract::presence;
use crate::transcendental::Transcendental;

/// The chance a bud keeps its kind at `light` under `shade`.
fn keeps(shade: &[f64], light: f64) -> f64 {
    let e: f64 = shade.iter().sum();
    light.clamp(0.0, 1.0).powf_fixed(e)
}

/// Each kind's share of a turn: `shade` over its sum.
fn shares(shade: &[f64]) -> Vec<f64> {
    let e: f64 = shade.iter().sum();
    shade.iter().map(|x| x / e).collect()
}

/// The PA a bud of `pa` placed at `place` becomes at its bearer's
/// `light`: its own where it keeps its kind.
pub(crate) fn kind_under_shade(shade: &[f64], pa: usize, light: f64, place: Key) -> usize {
    let d = place.child(TURN).draw();
    if d.under(keeps(shade, light)) {
        return pa;
    }
    let shares = shares(shade);
    let pick = place.child(TURN).child(0).draw();
    // Rounding may leave the last share's top short of one draw.
    bud(&shares, pick).unwrap_or_else(|| shares.iter().rposition(|&s| s > 0.0).unwrap_or(pa))
}

/// The presence over `window` the turn of a bud of `pa` into `kind` adds:
/// the keeping draw's lead past L^e, and for a bud that turned, the
/// pick's past its kind's bounds.
pub(crate) fn shaded_presence(
    shade: &[f64],
    (pa, kind): (usize, usize),
    light: f64,
    place: Key,
    window: f32,
) -> f32 {
    let d = place.child(TURN).draw();
    let t = threshold(keeps(shade, light));
    if kind == pa {
        return presence::of_under(t, d.0, window);
    }
    let pick = place.child(TURN).child(0).draw();
    let picked = bud_presence(&shares(shade), kind, pick, window);
    presence::times(presence::of_over(t, d.0, window), picked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::{BRANCH, SHOOT, SHORT};

    /// The beech's branch in full light makes 85% of its laterals long,
    /// and at Lemoine's shade, 255 of 1,850, 40%, within four standard
    /// errors of the draws (R7-DESIGN.md).
    #[test]
    fn a_branchs_long_share_follows_its_light() {
        let species = crate::beech();
        let branch = &species.states[BRANCH];
        // Each zone's laterals weighed by its mean node count.
        let nodes = |z: usize| match branch.zones[z].nodes {
            crate::species::NodeLaw::Uniform { min, max } => (min + max) / 2.0,
            _ => unreachable!("the beech's zones are uniform"),
        };
        let n = 40_000u64;
        for (light, want) in [(1.0, 0.85), (255.0 / 1850.0, 0.40)] {
            let (mut long, mut all) = (0.0, 0.0);
            for (z, zone) in branch.zones.iter().enumerate() {
                for i in 0..n {
                    let place = Key::root(i).child(z as u64);
                    let Some(pa) = bud(&zone.lateral, place.draw()) else {
                        continue;
                    };
                    let shade = &species.states[pa].shade_short;
                    let kind = match species.states[pa].turns() {
                        true => kind_under_shade(shade, pa, light, place),
                        false => pa,
                    };
                    all += nodes(z);
                    if kind == SHOOT || kind == BRANCH {
                        long += nodes(z);
                    }
                    assert!(kind == pa || kind == SHORT, "a turn makes a short shoot");
                }
            }
            let share = long / all;
            let error = (want * (1.0 - want) / (all / 2.0)).sqrt();
            assert!(
                (share - want).abs() < 4.0 * error + 0.01,
                "{light}: {share}"
            );
        }
    }
}
