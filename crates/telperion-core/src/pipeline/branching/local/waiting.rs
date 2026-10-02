//! Earliest possible crown entry for a candidate whose geometry is fixed.
use super::*;
use crate::growth::GrowthTraits;

#[derive(Clone, Copy)]
pub(in crate::pipeline::branching) struct Clock {
    pub slice: u64,
    pub traits: GrowthTraits,
    pub envelope: Envelope,
}
impl Clock {
    pub fn next(self, point: Vec3, trunk_height: f64) -> u64 {
        let live = Envelope {
            height: self.envelope.height * self.traits.fraction(self.slice),
            ..self.envelope
        };
        let radial = point.x.hypot_fixed(point.z);
        if point.y < trunk_height || (radial > 0.0 && point.y <= live.height * live.crown_base) {
            return u64::MAX;
        }
        let mut height = point.y.max(radial / live.spread.max(1e-300));
        // A supporting tangent separates an outside point from every smaller
        // homothetic convex crown. It gives a conservative first possible height,
        // even when the moving lower crown will eventually pass above the point.
        if live.shoulder >= 1.0 && point.y < live.height {
            if let Some(bound) = tangent_height(live, point.y, radial) {
                height = height.max(bound);
            }
        }
        let mature = self.traits.mature_slice();
        if height > self.envelope.height {
            return u64::MAX;
        }
        let mut lo = self.slice + 1;
        let mut hi = mature;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.envelope.height * self.traits.fraction(mid) >= height {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

fn tangent_height(e: Envelope, y: f64, radial: f64) -> Option<f64> {
    let t = (y / e.height - e.crown_base) / (1.0 - e.crown_base);
    let full = e.fullness;
    let (p, width, sign) = if t < full {
        (1.0 - t / full, full, 1.0)
    } else {
        ((t - full) / (1.0 - full), 1.0 - full, -1.0)
    };
    if !(0.0..1.0).contains(&p) {
        return None;
    }
    let slope = sign * e.spread / ((1.0 - e.crown_base) * width)
        * p.powf_fixed(e.shoulder - 1.0)
        * (1.0 - p.powf_fixed(e.shoulder)).powf_fixed(1.0 / e.shoulder - 1.0);
    let radius = e.radius_at(y);
    let support = radius - slope * y;
    if !slope.is_finite() || support <= 1e-9 || radial <= radius {
        return None;
    }
    let height = e.height * (radial - slope * y) / support;
    // Round toward an earlier visit. This margin affects scheduling only; the
    // existing exact crown predicate still decides every birth.
    Some(height - height.abs() * 1e-10 - 1e-10)
}

impl Frontier {
    pub(super) fn wake(&mut self, planner: &Planner<'_>, tree: &Tree) {
        if let Some(clock) = planner.clock {
            while self
                .sleeping
                .first_key_value()
                .is_some_and(|(&slice, _)| slice <= clock.slice)
            {
                self.queue.extend(self.sleeping.pop_first().unwrap().1);
                self.ordered = false;
            }
        }
        if planner.growing_envelope {
            self.identity_order(tree);
        }
    }
}

pub(super) fn below_reach(shoot: &Shoot, tree: &Tree, planner: &Planner<'_>) -> bool {
    // A curtain that drops may fall below the trunk boundary, into its band.
    if shoot.curtain.drops(planner.twigs) {
        return false;
    }
    // Every candidate is within the retained run length or the fixed twig
    // length. Thickening can change subdivision and bud fate, but neither can
    // bridge this gap to a trunk boundary which only rises. Include rounding
    // room so borderline geometry is left to the exact birth predicate.
    let y = tree.nodes[shoot.at].position.y;
    let reach = shoot.length.max(planner.twigs.twig.length);
    y + reach + (y.abs() + reach) * 1e-10 + 1e-10 < planner.config.trunk_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_candidate_waits_until_the_crown_can_reach_it() {
        let clock = Clock {
            slice: 10,
            traits: GrowthTraits::default(),
            envelope: Envelope {
                height: 2.0,
                crown_base: 0.0,
                ..Envelope::default()
            },
        };
        let point = Vec3::new(0.0, 1.5, 0.0);
        let next = clock.next(point, 0.0);
        assert!(
            next > 11,
            "waiting candidate was scheduled for another annual retry"
        );
        assert!(clock.envelope.height * clock.traits.fraction(next) >= point.y);
        assert!(clock.envelope.height * clock.traits.fraction(next - 1) < point.y);
    }
}
