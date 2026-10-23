//! The person in the release film's growths (fn-310 D5, host ruling
//! 2026-10-23): one placement for both renders of a crossfaded frame, and no
//! frame moving it further than one year's growth does.
// The film command uses what a test does not.
#[allow(dead_code)]
#[path = "../examples/film/path.rs"]
mod path;
#[allow(dead_code)]
#[path = "../examples/film/placing.rs"]
mod placing;

use telperion_core::{math::Vec3, surface::Bounds};
use telperion_render::{figure_placement, Placement};

/// Every growth of the release path: its frames with their years.
fn growths() -> Vec<Vec<placing::Year>> {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/film/release.json");
    let path = path::Path::read(&file).unwrap();
    let frames: Vec<(usize, path::Frame)> = path.frames().into_iter().enumerate().collect();
    let growths: Vec<_> = (path.segments.iter().enumerate())
        .filter_map(|(k, s)| match s.kind {
            path::Kind::Grow { from, to, .. } => {
                let of: Vec<_> = frames
                    .iter()
                    .copied()
                    .filter(|f| f.1.segment == k)
                    .collect();
                Some(placing::years_of(&of, (from, to), path.fade))
            }
            path::Kind::Walk { .. } => None,
        })
        .collect();
    assert_eq!(growths.len(), 3, "the release path grows three times");
    growths
}

/// A tree of `age` years as a crown grows: half a metre a year up and a
/// third of that out, so the person stands cut by the frame beside the
/// seedlings and whole beside the rest.
fn stood(age: f64) -> Placement {
    let (h, w) = (0.5 * age, 0.17 * age);
    figure_placement(Bounds {
        min: Vec3::new(-w, 0.0, -w),
        max: Vec3::new(w, h, w),
    })
    .0
}

/// Each year drawn as the film draws it, and the placement of every
/// render: a crossfaded frame is drawn twice, once per year, and both
/// renders stand the person in one place.
#[test]
fn both_renders_of_a_crossfaded_frame_stand_the_person_in_one_place() {
    for frames in growths() {
        let mut renders: std::collections::HashMap<usize, Vec<Placement>> = Default::default();
        for at in placing::built(&frames) {
            for (i, _) in placing::showing(&frames, at) {
                let frame = frames.iter().find(|f| f.0 == i).unwrap();
                let placement = placing::placed(frame, |y| Some(stood(y))).unwrap();
                renders.entry(i).or_default().push(placement);
            }
        }
        let mut crossfaded = 0;
        for &(i, _, _, share) in &frames {
            let drawn = &renders[&i];
            assert_eq!(drawn.len(), if share > 0.0 { 2 } else { 1 }, "frame {i}");
            assert!(drawn.iter().all(|p| *p == drawn[0]), "frame {i}: {drawn:?}");
            crossfaded += usize::from(share > 0.0);
        }
        assert!(crossfaded > 0, "a growth with no crossfade proves nothing");
    }
}

/// From frame to frame the person never moves further than it does between
/// two whole years, nor turns: it walks out with the crown, with no jump.
#[test]
fn the_person_moves_across_a_growth_by_no_more_than_a_years_step() {
    for frames in growths() {
        let years = placing::built(&frames);
        let step = years
            .windows(2)
            .map(|y| (stood(y[1]).foot - stood(y[0]).foot).length())
            .fold(0.0, f64::max);
        let placements: Vec<Placement> = frames
            .iter()
            .map(|f| placing::placed(f, |y| Some(stood(y))).unwrap())
            .collect();
        let moved = placements
            .windows(2)
            .map(|p| (p[1].foot - p[0].foot).length())
            .fold(0.0, f64::max);
        assert!(moved > 0.0, "the person never moved");
        assert!(
            moved <= step + 1e-9,
            "a frame moved {moved} m, a year {step} m"
        );
        assert!(placements.iter().all(|p| p.facing == placements[0].facing));
    }
}
