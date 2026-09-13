//! Earliest possible crown entry for a candidate whose geometry is fixed.
use super::*;
use crate::growth::GrowthTraits;

#[derive(Clone, Copy)]
pub(in crate::branching) struct Clock {
    pub month: u64,
    pub traits: GrowthTraits,
    pub envelope: Envelope,
}
impl Clock {
    pub fn next(self, point: Vec3, trunk_height: f64) -> u64 {
        let live = Envelope {
            height: self.envelope.height * self.traits.fraction(self.month),
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
        let mature = self.traits.mature_month();
        if height > self.envelope.height {
            return u64::MAX;
        }
        let mut lo = self.month + 1;
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
    let full = e.fullness.clamp(0.001, 0.999);
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
                .is_some_and(|(&month, _)| month <= clock.month)
            {
                self.queue.extend(self.sleeping.pop_first().unwrap().1);
            }
        }
        if planner.growing_envelope {
            self.identity_order(tree);
        }
    }
}

pub(super) fn below_reach(shoot: &Shoot, tree: &Tree, planner: &Planner<'_>) -> bool {
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
            month: 120,
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
            next > 121,
            "waiting candidate was scheduled for another monthly retry"
        );
        assert!(clock.envelope.height * clock.traits.fraction(next) >= point.y);
        assert!(clock.envelope.height * clock.traits.fraction(next - 1) < point.y);
    }
}

#[cfg(test)]
mod frontier_tests {
    use super::*;

    fn terminal() -> (Tree, Frontier) {
        let tree = Tree {
            nodes: vec![
                Node::root(),
                Node {
                    position: Vec3::new(0.0, 1.25, 0.0),
                    radius: 0.1,
                    start_radius: 0.1,
                    base_radius: 0.1,
                    parent: Some(0),
                    branch: 1,
                    kind: NodeKind::Branch,
                    ..Node::root()
                },
            ],
            crossover: 1,
            ..Tree::default()
        };
        let mut frontier = Frontier::default();
        frontier.queue.push_back(Shoot {
            flushed: 0,
            accepted: Vec::new(),
            at: 1,
            direction: Vec3::Y,
            normal: Vec3::X,
            phase: 0.0,
            radius: 0.1,
            length: 0.25,
            branch: Some(1),
            completed: 1,
            generation: 0,
            internodes: 1,
            key: 1,
            run: None,
            pendant: false,
            curtain_across: Vec3::X,
            pendant_floor: None,
        });
        (tree, frontier)
    }

    #[test]
    fn sleeping_terminal_receives_no_visits_and_wakes_at_crown_entry() {
        let traits = GrowthTraits::default();
        let envelope = Envelope {
            height: 2.0,
            crown_base: 0.0,
            ..Envelope::default()
        };
        let (mut tree, mut frontier) = terminal();
        for month in 120..=400 {
            let config = GrowthConfig {
                trunk_height: 0.0,
                shell: Some(Envelope {
                    height: envelope.height * traits.fraction(month),
                    ..envelope
                }),
                ..GrowthConfig::default()
            };
            frontier
                .advance(
                    &mut tree,
                    Planner {
                        clock: Some(Clock {
                            month,
                            traits,
                            envelope,
                        }),
                        widths: None,
                        growing_envelope: true,
                        planning: Some(envelope),
                        config: &config,
                        bias: None,
                        twigs: TwigParams {
                            laterals: 0,
                            ..TwigParams::default()
                        },
                        crookedness: 0.0,
                        seed: 7,
                    },
                    HabitParams::default(),
                    1,
                )
                .unwrap();
            if config
                .shell
                .unwrap()
                .contains(Vec3::new(0.0, 1.5, 0.0), 0.0)
            {
                assert_eq!(tree.nodes.len(), 3, "missed the first eligible birth");
                return;
            }
            assert_eq!(tree.nodes.len(), 2);
            if month > 120 {
                assert_eq!(
                    frontier.visited().count(),
                    0,
                    "retried a sleeping terminal in month {month}"
                );
            }
        }
        panic!("fixture did not reach the candidate");
    }

    #[test]
    fn a_shoot_beyond_all_reach_of_the_rising_trunk_is_not_replanned() {
        let traits = GrowthTraits::default();
        let envelope = Envelope {
            height: 2.0,
            crown_base: 0.0,
            ..Envelope::default()
        };
        let (mut tree, mut frontier) = terminal();
        for month in 120..=122 {
            let config = GrowthConfig {
                trunk_height: 2.0,
                shell: Some(Envelope {
                    height: envelope.height * traits.fraction(month),
                    ..envelope
                }),
                ..GrowthConfig::default()
            };
            frontier
                .advance(
                    &mut tree,
                    Planner {
                        clock: Some(Clock {
                            month,
                            traits,
                            envelope,
                        }),
                        widths: None,
                        growing_envelope: true,
                        planning: Some(envelope),
                        config: &config,
                        bias: None,
                        twigs: TwigParams {
                            laterals: 0,
                            ..TwigParams::default()
                        },
                        crookedness: 0.0,
                        seed: 7,
                    },
                    HabitParams::default(),
                    1,
                )
                .unwrap();
            assert_eq!(tree.nodes.len(), 2);
            assert_eq!(
                frontier.retries[0], 0,
                "replanned a shoot that cannot reach above the trunk"
            );
            assert!(frontier.queue.is_empty());
            assert_eq!(frontier.sleeping.values().map(Vec::len).sum::<usize>(), 1);
        }
    }
}

#[cfg(test)]
mod prediction_tests {
    use super::*;
    #[test]
    fn wake_bound_never_skips_an_eligible_slice_on_either_side_of_the_crown() {
        let traits = GrowthTraits::default();
        let mut delayed = 0;
        for shoulder in [0.5, 1.0, 2.2, 8.0] {
            for crown_base in [0.0, 0.16, 0.9] {
                for month in [12, 120, 240] {
                    let envelope = Envelope {
                        shoulder,
                        crown_base,
                        ..Envelope::default()
                    };
                    let clock = Clock {
                        month,
                        traits,
                        envelope,
                    };
                    let height = envelope.height * traits.fraction(month);
                    for y in [0.01, 0.2, 0.55, 0.99, 1.1] {
                        for radius in [0.01, 0.25, 0.75, 1.1] {
                            let point =
                                Vec3::new(height * envelope.spread * radius, height * y, 0.0);
                            let wake = clock.next(point, 0.0);
                            if let Some(first) = (month + 1..=traits.mature_month()).find(|&m| {
                                !rejected(
                                    &GrowthConfig {
                                        trunk_height: 0.0,
                                        shell: Some(Envelope {
                                            height: envelope.height * traits.fraction(m),
                                            ..envelope
                                        }),
                                        ..GrowthConfig::default()
                                    },
                                    point,
                                )
                            }) {
                                assert!(wake <= first, "wake {wake} missed {first}: {envelope:?}, month={month}, point={point:?}");
                                delayed += usize::from(wake > month + 1);
                            }
                        }
                    }
                }
            }
        }
        assert!(delayed > 0, "prediction degenerated into monthly retries");
    }
}
