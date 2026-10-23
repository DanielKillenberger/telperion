//! The person in the release film (fn-310 D7; host rulings on the film's
//! side and distance): square to the view on the camera's left in every
//! frame, at or past the distance its own frame asks, out of every crown's
//! shadow and from under every crown, and easing from frame to frame.
// The film command uses what a test does not.
#[allow(dead_code)]
#[path = "../examples/film/bounds.rs"]
mod bounds;
#[allow(dead_code)]
#[path = "../examples/film/camera.rs"]
mod camera;
#[allow(dead_code)]
#[path = "../examples/film/path.rs"]
mod path;
#[allow(dead_code)]
#[path = "../examples/film/placing.rs"]
mod placing;
#[allow(dead_code)]
#[path = "../examples/film/standing.rs"]
mod standing;

use standing::{smoothed, Shown, Standing, W};
use telperion_core::{math::Vec3, surface::Bounds};
use telperion_render::Placement;

/// The release path and its frames.
fn release() -> (path::Path, Vec<path::Frame>) {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/film/release.json");
    let path = path::Path::read(&file).unwrap();
    let frames = path.frames();
    (path, frames)
}

/// A tree of `age` years: 0.3 m a year up and two thirds of that across.
fn tree(age: f64) -> Bounds {
    let (h, w) = (0.3 * age, 0.1 * age);
    Bounds {
        min: Vec3::new(-w, 0.0, -w),
        max: Vec3::new(w, h, w),
    }
}

/// Every frame's crowns as the film fades its years, with each year's tree
/// `tree(age)` and a walk's tree that of its age.
fn modelled(path: &path::Path, frames: &[path::Frame]) -> Vec<bounds::Crowns> {
    let mut crowns = Vec::new();
    for (k, segment) in path.segments.iter().enumerate() {
        let of: Vec<(usize, path::Frame)> = (frames.iter().copied().enumerate())
            .filter(|f| f.1.segment == k)
            .collect();
        match segment.kind {
            path::Kind::Grow { from, to, .. } => {
                let years = placing::years_of(&of, (from, to), path.fade);
                crowns.extend(years.iter().map(|&(_, lo, hi, s)| (tree(lo), tree(hi), s)));
            }
            path::Kind::Walk { age, .. } => {
                crowns.extend(of.iter().map(|_| (tree(age), tree(age), 0.0)))
            }
        }
    }
    crowns
}

/// What every frame of the film shows the person beside.
fn shown(path: &path::Path, crowns: &[bounds::Crowns]) -> Vec<Shown> {
    (crowns.iter().enumerate())
        .map(|(i, &crowns)| Shown {
            view: camera::azimuth(&path.camera, i as f64 / f64::from(path.fps)),
            crowns,
        })
        .collect()
}

/// Every frame's placement as the rule asks: square to the view on the
/// camera's left at the trunk's depth, at or past its own frame's reach and
/// the distance out of the shadow, out of every crown's shadow it shows,
/// and out from under every crown it shows.
fn holds(shown: &[Shown], standing: &Standing, sun: (f64, f64)) {
    for (f, s) in shown.iter().enumerate() {
        let distance = standing.placements[f].foot.length();
        assert_eq!(standing.reach[f], s.reach(), "frame {f}");
        assert!(standing.needs[f] >= standing.reach[f], "frame {f}");
        assert!(
            distance >= standing.needs[f] - 1e-9,
            "frame {f}: nearer than it needs"
        );
        let placement = standing.placements[f];
        assert_eq!(placement.facing, s.view.to_radians(), "frame {f}");
        let (x, z) = s.view.to_radians().sin_cos();
        let (foot, toward) = (placement.foot, Vec3::new(x, 0.0, z));
        let depth = foot.dot(toward);
        assert!(
            depth.abs() < 1e-9,
            "frame {f}: {depth} m off the trunk's depth"
        );
        assert!(
            foot.x * z - foot.z * x < 0.0,
            "frame {f}: on the camera's right"
        );
        let (earlier, later, share) = s.crowns;
        let body = placement.body();
        let shows = if share > 0.0 {
            vec![earlier, later]
        } else {
            vec![earlier]
        };
        for crown in shows {
            let dark = body.iter().any(|&p| shaded(crown, p, sun));
            assert!(!dark, "frame {f}: in a crown's shadow");
            let (centre, half) = ((crown.min + crown.max) * 0.5, (crown.max - crown.min) * 0.5);
            let under = body.iter().any(|p| {
                let (x, z) = ((p.x - centre.x) / half.x, (p.z - centre.z) / half.z);
                x.hypot(z) <= 1.0
            });
            assert!(!under, "frame {f}: under the crown");
        }
    }
}

/// Whether the ray from `from` towards the sun passes through the crown the
/// bounds inscribe.
fn shaded(bounds: Bounds, from: Vec3, (azimuth, elevation): (f64, f64)) -> bool {
    let (centre, half) = (
        (bounds.min + bounds.max) * 0.5,
        (bounds.max - bounds.min) * 0.5,
    );
    let (a, e) = (azimuth.to_radians(), elevation.to_radians());
    let to_sun = Vec3::new(e.cos() * a.sin(), e.sin(), e.cos() * a.cos());
    let scale = |v: Vec3| Vec3::new(v.x / half.x, v.y / half.y, v.z / half.z);
    let (o, d) = (scale(from - centre), scale(to_sun));
    let (a, b, c) = (d.dot(d), 2.0 * o.dot(d), o.dot(o) - 1.0);
    let root = b * b - 4.0 * a * c;
    root >= 0.0 && (-b + root.sqrt()) / (2.0 * a) > 0.0
}

/// The largest frame-to-frame move of the foot, in metres, and its frame.
fn largest_move(placements: &[Placement]) -> (f64, usize) {
    (placements.windows(2).enumerate())
        .map(|(f, p)| ((p[1].foot - p[0].foot).length(), f + 1))
        .fold((0.0, 0), |m, v| if v.0 > m.0 { v } else { m })
}

#[test]
fn smoothing_never_falls_below_a_value_and_eases_a_step() {
    let step: Vec<f64> = (0..300).map(|f| if f < 150 { 1.0 } else { 2.0 }).collect();
    let eased = smoothed(&step);
    assert!(eased.iter().zip(&step).all(|(e, v)| e >= v));
    let rise = eased.windows(2).map(|p| p[1] - p[0]).fold(0.0, f64::max);
    assert!((rise - 1.0 / (2 * W + 1) as f64).abs() < 1e-12, "{rise}");
    assert_eq!(smoothed(&[3.0]), [3.0]);
}

/// The rule over the release path with modelled crowns, always run.
#[test]
fn the_person_keeps_to_the_cameras_left_lit_and_easing_across_the_film() {
    let (path, frames) = release();
    let shown = shown(&path, &modelled(&path, &frames));
    let standing = standing::stand(&shown, path.sun).unwrap();
    holds(&shown, &standing, path.sun);
    let asked = standing
        .needs
        .iter()
        .zip(&standing.reach)
        .any(|(n, r)| n > r);
    assert!(asked, "the shadow never asks the person further out");
    let (moved, f) = largest_move(&standing.placements);
    assert!(moved < 0.1, "frame {f}: {moved} m");
}

/// The largest frame-to-frame move of the person over the film's real
/// crowns, in metres (fn-310, host ruling on the film's distance): 0.183
/// measured at frame 830, where Laurelin's crown grows out faster than any
/// smoothing of a growing reach can hide.
const BOUND: f64 = 0.19;

/// The same over the film's real crowns, from the bounds pass the film
/// keeps (`film ... --bounds`). Run by name (`--ignored`) once the pass
/// exists.
#[test]
#[ignore = "reads the film's bounds pass from raw/; run after `film --bounds`"]
fn the_person_keeps_to_the_cameras_left_beside_the_films_real_crowns() {
    let (path, frames) = release();
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/film-bounds.tsv",
    );
    let crowns = bounds::read(&file, frames.len()).unwrap();
    let shown = shown(&path, &crowns);
    let standing = standing::stand(&shown, path.sun).unwrap();
    holds(&shown, &standing, path.sun);
    let (moved, f) = largest_move(&standing.placements);
    println!("largest move {moved:.4} m at frame {f}");
    assert!(moved <= BOUND, "frame {f}: {moved} m");
}
