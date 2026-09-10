//! The numeric outline of one foliage element: how wide it is at each station
//! along its axis, and what its transverse section looks like there. Both are
//! continuous in the element's own traits, so the lobed blade and the
//! four-sided needle are two rows of one table and every point between them is
//! an element in its own right.
use super::element::ElementParams;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The margin at a sinus keeps this fraction of the local envelope at full
/// lobe depth: the midrib the retired five-lobe oak table carried. It is why
/// depth 1 cuts a blade into lobes rather than into nothing, and why a fully
/// lobed section is still a section.
const MIDRIB: f64 = 0.17;

/// How the cut is distributed along a lobe period. A plain cosine spends as
/// much of the period cutting as it does at full width, which draws a chevron
/// between two notches as wide as the lobe itself; raising it broadens the
/// crest and narrows the notch without moving either. Six is measured, not
/// chosen: it puts 60.2% of the period within a quarter of the crest and 17.2%
/// within a quarter of the sinus floor, where the retired five-lobe oak table
/// sat at 60.2% and 16.9% by the same two contours.
const LOBE_BROADNESS: i32 = 6;

/// Half the transverse extent at `t` along the axis, in metres. The envelope
/// is the widest point with its base fullness and tip sharpness; the lobes cut
/// sinuses into it, `lobe_count` crests along the margin at (2k+1)/2n with a
/// sinus between each pair and one at either end, each cutting `lobe_depth` of
/// the way to the midrib.
pub(super) fn half_width(p: &ElementParams, t: f64) -> f64 {
    let envelope = if t <= p.widest_at {
        (FRAC_PI_2 * t / p.widest_at).sin().powf(p.base_fullness)
    } else {
        (FRAC_PI_2 * (t - p.widest_at) / (1. - p.widest_at))
            .cos()
            .powf(p.tip_sharpness)
    };
    // No lobes, no sinuses: an entire margin, whatever the depth says.
    let sinus = if p.lobe_count == 0 {
        0.
    } else {
        (0.5 + 0.5 * (TAU * p.lobe_count as f64 * t).cos()).powi(LOBE_BROADNESS)
    };
    p.width / 2. * envelope * (1. - sinus * p.lobe_depth * (1. - MIDRIB))
}

/// One vertex of the transverse section, as (x, z) in metres, at `u` across
/// the section from -1 to 1 with `half` the local half-width.
///
/// At roundness 0 the section is the flat strip a blade carries, cupped by
/// `cup`. At 1 it is the four-sided shaft a needle carries: the same strip
/// rolled into a closed loop whose two edges meet at the top, so the section
/// spans four sides and no vertex is spent on a seam that is not there.
/// Between them it is the roll itself, which is what lets a blade become a
/// needle with no switch frame. The loop runs clockwise in (x, z) from the
/// seam, which is the strip's own direction at either edge, so the wound face
/// points outward all the way round and forward at roundness 0.
pub(super) fn section(p: &ElementParams, half: f64, u: f64) -> (f64, f64) {
    let flat = (u * half, p.cup * half * u * u);
    let (sin, cos) = (-FRAC_PI_2 - PI * u).sin_cos();
    // The four-sided section is the unit ball of |x| + |z|, sampled by angle.
    let reach = half / (sin.abs() + cos.abs());
    let round = (reach * cos, reach * sin);
    let r = p.section_roundness;
    (
        (1. - r) * flat.0 + r * round.0,
        (1. - r) * flat.1 + r * round.1,
    )
}
