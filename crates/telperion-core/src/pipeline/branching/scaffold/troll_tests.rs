//! Troll's two rows on a synthetic family, one station at a time: a
//! lateral's pitch follows its station's height in the crown, and a
//! first-order axis stops short of its room by a share it draws for itself.
use super::*;

/// The crown's base and top in the synthetic family.
const BASE: f64 = 4.0;
const TOP: f64 = 16.0;
/// The lateral pitch at the crown's base, and the degrees it moves to the top.
const PITCH: f64 = 70.0;
const BY_HEIGHT: f64 = -40.0;

fn envelope() -> Envelope {
    Envelope {
        height: TOP,
        crown_base: BASE / TOP,
        spread: 6.0,
        ..Default::default()
    }
}

fn habit(ragged_reach: f64) -> HabitParams {
    HabitParams {
        lateral_pitch: PITCH,
        pitch_variation: 0.0,
        pitch_by_height: BY_HEIGHT,
        ragged_reach,
        laterals_per_station: 6,
        attractor_weight: 0.0,
        ..HabitParams::default()
    }
}

/// Runs `probe` against a builder over the synthetic crown, with one node
/// on the axis at every height in `heights`.
fn with_builder<T>(habit: HabitParams, heights: &[f64], probe: impl Fn(&mut Builder) -> T) -> T {
    let e = envelope();
    let mut config = default_growth(e, 0, DEFAULT_STEP);
    config.trunk_height = BASE;
    let bias = GrowthBias::new(e, 1, BiasParams::NONE).unwrap();
    let mut tree = Tree::default();
    tree.nodes.push(Node::root());
    for &y in heights {
        tree.nodes.push(Node {
            position: Vec3::new(0.0, y, 0.0),
            ..Node::root()
        });
    }
    let mut builder = Builder {
        tree: &mut tree,
        envelope: e,
        planning: e,
        config: &config,
        bias: &bias,
        habit,
        points: &[],
        consumed: &mut [],
        year: 0,
        influence_sq: 0.0,
        kill_sq: 0.0,
        point_scale: 1.0,
        growing_envelope: false,
        paused: false,
        limbs: &mut Limbs::default(),
    };
    probe(&mut builder)
}

/// The degrees each lateral of a second-order station at `y` leaves an
/// upright parent by; a second-order axis takes its length from its parent,
/// so a station at the crown's very top still bears a lateral.
fn pitches(y: f64) -> Vec<f64> {
    pitches_while(y, false)
}

/// As `pitches`, on the growth path when `growing`: its planning shell sits
/// a twig's reach inside the crown.
fn pitches_while(y: f64, growing: bool) -> Vec<f64> {
    with_builder(habit(0.0), &[y], |b| {
        if growing {
            b.growing_envelope = true;
            b.planning.height -= 0.5;
        }
        let parent = Axis::new(1, Vec3::Y, 8.0, 1, 0x51ed_270b);
        let laterals = b.station(&parent, 1, Vec3::Y, 0);
        assert!(!laterals.is_empty(), "the station at {y} bore nothing");
        laterals
            .iter()
            .map(|a| a.heading.y.clamp(-1.0, 1.0).acos().to_degrees())
            .collect()
    })
}

fn close(found: &[f64], wanted: f64) -> bool {
    found.iter().all(|p| (p - wanted).abs() < 1e-9)
}

#[test]
fn a_station_at_the_crowns_base_leaves_at_the_base_pitch() {
    let found = pitches(BASE);
    assert!(close(&found, PITCH), "{found:?}");
}

#[test]
fn a_station_at_the_crowns_top_leaves_at_the_top_pitch() {
    let found = pitches(TOP);
    assert!(close(&found, PITCH + BY_HEIGHT), "{found:?}");
}

#[test]
fn the_stations_between_are_graded_by_height() {
    for share in [0.25, 0.5, 0.75] {
        let found = pitches(BASE + (TOP - BASE) * share);
        assert!(
            close(&found, PITCH + BY_HEIGHT * share),
            "{share}: {found:?}"
        );
    }
}

#[test]
fn the_growth_path_grades_the_pitch_over_the_same_crown() {
    for y in [BASE, 10.0, TOP] {
        assert_eq!(pitches_while(y, true), pitches(y), "at {y}");
    }
}

/// The share of its room each first-order axis of a few leader stations
/// stops short by, station by station and member by member.
fn shortfalls(ragged_reach: f64, key: u32) -> Vec<f64> {
    let heights = [6.0, 8.0, 10.0];
    with_builder(habit(ragged_reach), &heights, |b| {
        let leader = Axis::new(0, Vec3::Y, TOP, 0, key);
        let mut shares = Vec::new();
        for at in 1..=heights.len() {
            let position = b.tree.nodes[at].position;
            for axis in b.station(&leader, at, Vec3::Y, at) {
                shares.push(1.0 - axis.length / b.reach(position, axis.heading));
            }
        }
        assert!(shares.len() >= 12, "only {} axes found room", shares.len());
        shares
    })
}

#[test]
fn a_ragged_reach_stops_each_axis_short_within_the_row() {
    assert!(shortfalls(0.0, 7).iter().all(|&s| s == 0.0));
    let shares = shortfalls(0.4, 7);
    assert!(
        shares.iter().all(|&s| (-1e-12..=0.4 + 1e-12).contains(&s)),
        "{shares:?}"
    );
    assert!(shares.iter().any(|&s| s > 0.2), "{shares:?}");
}

#[test]
fn a_ragged_reach_differs_axis_to_axis() {
    let mut shares = shortfalls(0.4, 7);
    shares.sort_by(f64::total_cmp);
    assert!(
        shares.windows(2).all(|w| w[1] - w[0] > 1e-9),
        "two axes drew one share: {shares:?}"
    );
    assert_ne!(
        shortfalls(0.4, 8),
        shortfalls(0.4, 7),
        "another seed drew the same"
    );
}

#[test]
fn a_ragged_reach_draws_the_same_shares_from_the_same_seed() {
    assert_eq!(shortfalls(0.4, 7), shortfalls(0.4, 7));
}
