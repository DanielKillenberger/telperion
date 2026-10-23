//! The person in the release film (fn-310 D5, D7, host rulings
//! 2026-10-23): one placement for both renders of a crossfaded frame, beside
//! the subject from each frame's view, and moving from frame to frame only
//! as the crown grows and the camera turns.
// The film command uses what a test does not.
#[allow(dead_code)]
#[path = "../examples/film/camera.rs"]
mod camera;
#[allow(dead_code)]
#[path = "../examples/film/path.rs"]
mod path;
#[allow(dead_code)]
#[path = "../examples/film/placing.rs"]
mod placing;

use telperion_core::{math::Vec3, surface::Bounds};
use telperion_render::{figure_placement_from, Placement};

/// The release path and its frames, each with its index in the film.
fn release() -> (path::Path, Vec<(usize, path::Frame)>) {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/film/release.json");
    let path = path::Path::read(&file).unwrap();
    let frames = path.frames().into_iter().enumerate().collect();
    (path, frames)
}

/// Every growth of the release path: its frames with their years.
fn growths(path: &path::Path, frames: &[(usize, path::Frame)]) -> Vec<Vec<placing::Year>> {
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

/// The frame camera's bearing at frame `i`.
fn view(path: &path::Path, i: usize) -> f64 {
    camera::azimuth(&path.camera, i as f64 / f64::from(path.fps))
}

/// A tree of `age` years as a crown grows: half a metre a year up and a
/// third of that out.
fn tree(age: f64) -> Bounds {
    let (h, w) = (0.5 * age, 0.17 * age);
    Bounds {
        min: Vec3::new(-w, 0.0, -w),
        max: Vec3::new(w, h, w),
    }
}

/// Where frame `i` stands the person beside a tree of `age` years.
fn stood(path: &path::Path, i: usize, age: f64) -> Placement {
    figure_placement_from(tree(age), view(path, i), path.sun.0).0
}

/// Each year drawn as the film draws it, and the placement of every
/// render: a crossfaded frame is drawn twice, once per year, and both
/// renders stand the person in one place.
#[test]
fn both_renders_of_a_crossfaded_frame_stand_the_person_in_one_place() {
    let (path, all) = release();
    for frames in growths(&path, &all) {
        let mut renders: std::collections::HashMap<usize, Vec<Placement>> = Default::default();
        for at in placing::built(&frames) {
            for (i, _) in placing::showing(&frames, at) {
                let frame = frames.iter().find(|f| f.0 == i).unwrap();
                let placement = placing::placed(frame, |y| Some(stood(&path, i, y))).unwrap();
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

/// From frame to frame the person moves no further than a year's growth
/// carries it out plus the arc a frame's turn of the camera carries it
/// round, and turns only with the camera: no jump within a growth.
#[test]
fn the_person_moves_across_a_growth_only_with_the_crown_and_the_camera() {
    let (path, all) = release();
    let turn = (path.camera.orbit / f64::from(path.fps)).to_radians();
    for frames in growths(&path, &all) {
        let years = placing::built(&frames);
        let placements: Vec<Placement> = (frames.iter())
            .map(|f| placing::placed(f, |y| Some(stood(&path, f.0, y))).unwrap())
            .collect();
        let step = (years.windows(2))
            .map(|y| (stood(&path, 0, y[1]).foot - stood(&path, 0, y[0]).foot).length())
            .fold(0.0, f64::max);
        let far = placements
            .iter()
            .map(|p| p.foot.length())
            .fold(0.0, f64::max);
        for (pair, f) in placements.windows(2).zip(&frames[1..]) {
            let moved = (pair[1].foot - pair[0].foot).length();
            assert!(
                moved <= step + far * turn + 1e-9,
                "frame {}: {moved} m",
                f.0
            );
            let turned = pair[1].facing - pair[0].facing;
            assert!(
                (turned - turn).abs() < 1e-9,
                "frame {}: turned {turned}",
                f.0
            );
        }
    }
}

/// Within any one segment of the film the person stays on one side of the
/// view (fn-310 D7, the motion check). Under D7 it changes sides at frame
/// 713 of segment 6, where the orbiting camera crosses the sun's azimuth
/// with the sun behind it; the remedy is the host's to choose.
#[test]
#[ignore = "fn-310 D7: the person changes sides at frame 713, segment 6; host ruling pending"]
fn the_person_never_changes_sides_within_a_segment() {
    let (path, frames) = release();
    let side = |i: usize| {
        let (foot, v) = (stood(&path, i, 20.0).foot, view(&path, i).to_radians());
        (foot.x * v.cos() - foot.z * v.sin()).signum()
    };
    let flips: Vec<(usize, usize)> = (frames.windows(2))
        .filter(|w| w[0].1.segment == w[1].1.segment && side(w[0].0) != side(w[1].0))
        .map(|w| (w[1].1.segment, w[1].0))
        .collect();
    assert!(
        flips.is_empty(),
        "sides change at (segment, frame) {flips:?}"
    );
}
