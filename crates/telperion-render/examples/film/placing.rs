//! A growth's years and the person's place in each frame (fn-310 D5, host
//! ruling 2026-10-23): a frame shows its earlier year with the later faded
//! over it, and both renders stand the person at one placement, the earlier
//! year's moved toward the later's by the frame's fade share. So the person
//! walks out as the crown grows, with no second person in a crossfade.
use super::path::{faded, Frame};
use telperion_render::Placement;

/// A growth frame: its index in the film, its earlier year, its later year
/// and the later's share.
pub type Year = (usize, f64, f64, f64);

/// A growth's age `along` its segment.
pub fn age_at(from: f64, to: f64, along: f64) -> f64 {
    (from + (to - from) * along).max(1.0)
}

/// Each frame of a growth from `from` to `to` with its years and share.
pub fn years_of(frames: &[(usize, Frame)], (from, to): (f64, f64), fade: f64) -> Vec<Year> {
    frames
        .iter()
        .map(|&(i, f)| {
            let a = age_at(from, to, f.along);
            (i, a.floor(), a.ceil(), faded(a - a.floor(), fade))
        })
        .collect()
}

/// The whole years the frames draw, rising: each frame's earlier year, and
/// its later where the later has begun to fade in.
pub fn built(frames: &[Year]) -> Vec<f64> {
    let mut years: Vec<f64> = frames
        .iter()
        .flat_map(|&(_, lo, hi, share)| if share > 0.0 { vec![lo, hi] } else { vec![lo] })
        .collect();
    years.sort_by(f64::total_cmp);
    years.dedup();
    years
}

/// The frames that draw year `at`, each with the later year's share.
pub fn showing(frames: &[Year], at: f64) -> Vec<(usize, f64)> {
    frames
        .iter()
        .filter(|&&(_, lo, hi, share)| lo == at || (share > 0.0 && hi == at))
        .map(|&(i, _, _, share)| (i, share))
        .collect()
}

/// Where the person stands in `frame`, given each year's own placement
/// (`stood`); none while a year it shows has not been built.
pub fn placed(frame: &Year, stood: impl Fn(f64) -> Option<Placement>) -> Option<Placement> {
    let &(_, lo, hi, share) = frame;
    let earlier = stood(lo)?;
    match share > 0.0 {
        true => Some(earlier.lerp(stood(hi)?, share)),
        false => Some(earlier),
    }
}
