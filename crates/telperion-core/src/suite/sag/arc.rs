//! The arc one hanging shoot takes: how far it runs, what stops it, and
//! how far toward straight down it has turned by each step of the way.
use super::*;

#[test]
fn a_hanging_shoot_turns_toward_the_ground_and_never_back() {
    // Along one shoot the angle to straight down never grows, no step turns
    // more than the step before it, and nothing that left its limb hanging
    // ever comes back up past the horizon.
    const PENDULOUS: f64 = 0.3;
    for sag in [1.0, 0.5] {
        let runs = whole_runs(&hanging(sag, PENDULOUS), PENDULOUS);
        assert!(
            !runs.is_empty(),
            "seed {SEED} sag {sag}: no hanging run long enough to read an arc on"
        );
        for run in &runs {
            for k in 1..run.len() {
                let (before, after) = (&run[k - 1], &run[k]);
                assert!(
                    after.angle <= before.angle + 1e-9,
                    "seed {SEED} sag {sag}: step {k} turned back up, \
                     {:.4} rad to {:.4} rad",
                    before.angle,
                    after.angle
                );
                assert!(
                    after.angle < FRAC_PI_2,
                    "seed {SEED} sag {sag}: step {k} points upward"
                );
                if k > 1 {
                    let (last, this) =
                        (run[k - 2].angle - before.angle, before.angle - after.angle);
                    assert!(
                        this <= last + 1e-9,
                        "seed {SEED} sag {sag}: step {k} turned {this:.5} rad \
                         against the {last:.5} rad of the step before it"
                    );
                }
            }
        }
    }
}

#[test]
fn a_shoot_of_its_full_pendulous_length_turns_the_stated_fraction() {
    // The row is a promise about the end of a full run: a shoot that has run
    // its whole pendulous length carries `1 - sag` of the angle it departed
    // with. The departure itself is fn-37's droop and is read back off the
    // first step rather than assumed.
    const PENDULOUS: f64 = 0.3;
    for sag in [1.0, 0.5] {
        let whole = whole_runs(&hanging(sag, PENDULOUS), PENDULOUS);
        assert!(
            !whole.is_empty(),
            "seed {SEED} sag {sag}: no shoot ran its whole pendulous length"
        );
        for run in &whole {
            let first = &run[0];
            let departure = first.angle / remaining(sag, first.along, PENDULOUS);
            let end = run.last().expect("a run has a step").angle;
            let wanted = departure * (1.0 - sag);
            assert!(
                (end - wanted).abs() < 1e-6,
                "seed {SEED} sag {sag}: a shoot that departed at {:.4} rad \
                 ended at {end:.4} rad, not the {wanted:.4} rad the row states",
                departure
            );
        }
    }
}

#[test]
fn a_sag_carries_a_shoot_past_what_holds_a_stiff_one_back() {
    // Two of the three things that end a shoot the branch law grows are not
    // what ends a strand hanging by its own weight: the length its own wood
    // would hold out, and the tip of the limb it hangs under. Under a sag the
    // run is the pendulous length the table states and the floor is the
    // crown's own base. The third, the shell, still holds: what the envelope
    // contains is the tree, and a curtain is no exception to that.
    const PENDULOUS: f64 = 2.5;
    let base = hanging(1.0, PENDULOUS).skeleton.envelope.crown_base
        * hanging(1.0, PENDULOUS).skeleton.envelope.height;
    let mut whole = Vec::new();
    let mut fall = Vec::new();
    let mut lowest = Vec::new();
    for sag in [0.0, 1.0] {
        let runs = hanging_runs(&hanging(sag, PENDULOUS));
        whole.push(
            runs.iter()
                .filter(|r| (r.last().expect("a run has a step").along - PENDULOUS).abs() < 1e-6)
                .count() as f64
                / runs.len() as f64,
        );
        fall.push(
            runs.iter()
                .map(|r| r[0].lowest - r.last().expect("a run has a step").lowest)
                .sum::<f64>()
                / runs.len() as f64,
        );
        lowest.push(
            runs.iter()
                .map(|r| r.last().expect("a run has a step").lowest)
                .fold(f64::INFINITY, f64::min),
        );
    }
    // The population is every descending run in the crown, and the ones that
    // establish the descent carry no curtain and so no sag; what the row has
    // to show is that it carries a large share of them out to a length the
    // branch law reached on none of them.
    assert!(
        whole[0] < 0.01,
        "the branch law let {:.0}% of its shoots run a whole pendulous length \
         with no sag on them",
        whole[0] * 100.0
    );
    assert!(
        whole[1] > 0.2,
        "only {:.0}% of the descending runs at full sag ran their whole \
         pendulous length",
        whole[1] * 100.0
    );
    // And it falls: the limb's own tip no longer catches it, so a shoot
    // descends further before anything stops it, while the crown's base still
    // does. The deepest point of the curtain is the shell's business and is
    // the same either way.
    assert!(
        fall[1] > fall[0] * 1.5,
        "a shoot fell {:.2} m under a full sag against {:.2} m dry",
        fall[1],
        fall[0]
    );
    assert!(
        lowest[1] >= base - 1e-6,
        "the curtain ended {:.2} m below the crown's own base of {base:.2} m",
        base - lowest[1]
    );
}

#[test]
fn the_lower_half_of_a_shoot_at_full_sag_hangs_all_but_straight_down() {
    // What the owner asks of a weeping tree: not that the tip of a shoot points
    // down, but that its whole lower half does.
    const PENDULOUS: f64 = 2.5;
    let runs = whole_runs(&hanging(1.0, PENDULOUS), PENDULOUS);
    assert!(
        !runs.is_empty(),
        "seed {SEED}: no shoot ran its whole length"
    );
    let steep = runs
        .iter()
        .filter(|r| {
            let half = r.last().expect("a run has a step").along / 2.0;
            r.iter()
                .filter(|s| s.along >= half)
                .all(|s| s.angle.to_degrees() <= 15.0)
        })
        .count();
    assert!(
        steep * 10 >= runs.len() * 9,
        "only {steep} of {} shoots hang their lower half within 15 degrees \
         of straight down",
        runs.len()
    );
}
