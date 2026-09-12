use super::*;
pub(super) fn rejected(config: &GrowthConfig, p: Vec3) -> bool {
    p.y < config.trunk_height
        || config
            .shell
            .is_some_and(|s| p.y > s.height || p.x.hypot(p.z) > s.radius_at(p.y))
}
pub(in crate::branching) struct Planner<'a> {
    pub(in crate::branching) config: &'a GrowthConfig,
    pub(in crate::branching) bias: Option<&'a GrowthBias>,
    pub(in crate::branching) twigs: TwigParams,
    pub(in crate::branching) crookedness: f64,
    pub(in crate::branching) seed: u32,
}
impl Planner<'_> {
    pub(super) fn heading(&self, at: Vec3, from: Vec3, wanted: Vec3, distance: f64) -> Vec3 {
        let c = self.config;
        let wanted = self.bias.map_or(wanted.normalized(), |b| {
            b.apply(at, wanted, c.step_distance)
        });
        colonization::limit_turn(
            Some(from),
            wanted,
            c.max_turn_per_step.to_radians() * (distance / c.step_distance).min(1.0),
        )
    }
    pub(super) fn run(
        &self,
        start: Vec3,
        first: Vec3,
        length: f64,
        internodes: usize,
        bearing: bool,
        key: u32,
    ) -> Option<Rc<Run>> {
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
        let mut points = vec![start];
        let mut along = vec![0.0];
        let mut heading = first;
        let phase = Rng::new(self.seed ^ key).range(0.0, TAU);
        let normal = first.perpendicular();
        let binormal = first.cross(normal);
        for k in 0..count {
            let stride = length * (stations[k] - if k == 0 { 0.0 } else { stations[k - 1] });
            let at = *points.last().unwrap();
            let wanted = if self.crookedness == 0.0 {
                heading
            } else {
                let angle = stations[k] * TAU * 2.0 + phase;
                first
                    + (normal * angle.sin() + binormal * (angle * 0.7).cos())
                        * self.crookedness.to_radians()
            };
            heading = self.heading(at, heading, wanted, stride);
            let end = at + heading * stride;
            if rejected(self.config, end) {
                let mut low = 0.0;
                let mut high = stride;
                for _ in 0..40 {
                    let mid = (low + high) / 2.0;
                    if rejected(self.config, at + heading * mid) {
                        high = mid
                    } else {
                        low = mid
                    }
                }
                if low > 1e-9 {
                    points.push(at + heading * low);
                    along.push(along.last().unwrap() + low)
                }
                break;
            }
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
