use super::*;
use crate::envelope::queries;
use crate::math::Transcendental;
/// What one axis asks the planner for: where it starts and the direction it
/// leaves with, how far it runs and in how many internodes, whether its wood
/// bears leaves rather than branching, the key its crookedness is drawn from,
/// and the curtain that bends it if it hangs.
#[derive(Clone, Copy)]
pub(super) struct Axis {
    pub start: Vec3,
    pub first: Vec3,
    pub length: f64,
    pub internodes: usize,
    pub bearing: bool,
    pub key: u32,
    pub curtain: Curtain,
}
pub(in crate::pipeline::branching) struct Planner<'a> {
    pub(in crate::pipeline::branching) config: &'a GrowthConfig,
    pub(in crate::pipeline::branching) bias: Option<&'a GrowthBias>,
    pub(in crate::pipeline::branching) twigs: TwigParams,
    pub(in crate::pipeline::branching) crookedness: f64,
    pub(in crate::pipeline::branching) seed: u32,
}
impl Planner<'_> {
    pub(super) fn width(&self, tree: &Tree, i: usize) -> [f64; 3] {
        let n = &tree.nodes[i];
        [n.radius, n.start_radius, n.base_radius]
    }
    pub(super) fn heading(&self, at: Vec3, from: Vec3, wanted: Vec3, distance: f64) -> Vec3 {
        let c = self.config;
        let wanted = self
            .bias
            .map_or(wanted.normalized(), |b| b.apply(at, wanted));
        colonization::limit_turn(
            Some(from),
            wanted,
            c.max_turn_per_step.to_radians() * (distance / c.step_distance).min(1.0),
        )
    }
    pub(super) fn run(&self, axis: Axis) -> Option<Rc<Run>> {
        let Axis {
            start,
            first,
            length,
            internodes,
            bearing,
            key,
            curtain,
        } = axis;
        // An axis runs to the length its own rules give it, or to its
        // curtain's floor: the crown's outline is no wall to the twig layer.
        queries::planned_axis();
        let count = internodes.max(if bearing {
            1
        } else {
            self.twigs.laterals as usize + 1
        });
        let mut stations: Vec<_> = (1..=count).map(|k| k as f64 / count as f64).collect();
        if !bearing {
            for j in 0..self.twigs.laterals {
                let station = ((j + 1) as f64 * count as f64 / (self.twigs.laterals + 1) as f64)
                    .round()
                    .max(1.0) as usize;
                stations[station - 1] = (j + 1) as f64 / (self.twigs.laterals + 1) as f64;
            }
        }
        // A run's full length is only a hint: a run may stop at its curtain's
        // floor, so a reservation the allocator refuses is skipped.
        let (mut points, mut along) = (Vec::new(), Vec::new());
        let _ = points.try_reserve(count.saturating_add(1));
        let _ = along.try_reserve(count.saturating_add(1));
        points.push(start);
        along.push(0.0);
        // The course is where the branch law holds the shoot; the heading is
        // where it goes once its own weight has bent the course toward the
        // ground. Without a curtain to hang from they are one vector.
        let mut course = first;
        let phase = Rng::new(self.seed ^ key).range(0.0, TAU);
        let normal = first.perpendicular();
        let binormal = first.cross(normal);
        for k in 0..count {
            let stride = length * (stations[k] - if k == 0 { 0.0 } else { stations[k - 1] });
            let at = *points.last().unwrap();
            let travelled = *along.last().unwrap() + stride;
            let wanted = if self.crookedness == 0.0 {
                course
            } else {
                let angle = stations[k] * TAU * 2.0 + phase;
                first
                    + (normal * angle.sin_fixed() + binormal * (angle * 0.7).cos_fixed())
                        * self.crookedness.to_radians()
            };
            course = self.heading(at, course, wanted, stride);
            let heading = curtain.sagged(course, travelled, self.twigs, key ^ self.seed);
            if let Some(kept) = curtain.crossing(at.y, (at + heading * stride).y) {
                let kept = kept * stride;
                if kept > 1e-9 {
                    points.push(at + heading * kept);
                    along.push(along.last().unwrap() + kept)
                }
                break;
            }
            let end = at + heading * stride;
            points.push(end);
            along.push(along.last().unwrap() + stride);
        }
        let mut actual = *along.last().unwrap();
        if actual < length - 1e-9 {
            actual = (actual - self.twigs.twig.length).max(0.0)
        }
        if actual <= 1e-9 {
            return None;
        }
        while along.len() > 2 && along[along.len() - 2] >= actual {
            along.pop();
            points.pop();
        }
        let last = along.len() - 1;
        points[last] = points[last - 1].lerp(
            points[last],
            (actual - along[last - 1]) / (along[last] - along[last - 1]),
        );
        along[last] = actual;
        if actual < length - 1e-9 && !bearing {
            let count = ((internodes as f64 * actual / length).ceil() as usize)
                .max(self.twigs.laterals as usize + 1);
            let mut distances = along.clone();
            distances.extend((1..=count).map(|k| actual * k as f64 / count as f64));
            distances.sort_by(f64::total_cmp);
            distances.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
            let mut resampled = Vec::with_capacity(distances.len());
            resampled.push(start);
            let mut edge = 1;
            for &d in distances.iter().skip(1) {
                while edge < along.len() - 1 && along[edge] < d {
                    edge += 1;
                }
                resampled.push(points[edge - 1].lerp(
                    points[edge],
                    ((d - along[edge - 1]) / (along[edge] - along[edge - 1])).clamp(0.0, 1.0),
                ));
            }
            points = resampled;
            along = distances;
        }
        Some(Rc::new(Run {
            positions: points.into_iter().skip(1).collect(),
            fractions: along.into_iter().skip(1).map(|d| d / actual).collect(),
            length: actual,
        }))
    }
}
