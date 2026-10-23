//! Where the person stands in each frame of the film (fn-310 D7; host
//! ruling on the film's side, option (a)). The film is one shot, so the
//! person keeps to the camera's left throughout: on each frame at the
//! smallest angle round from the camera, at least 90 degrees, that keeps it
//! out of the shadow of every crown the frame shows, and at that bearing's
//! crown reach, year-faded in a growth. Both are smoothed across the whole
//! film: each frame takes the largest value within `2 W` frames, and then
//! the mean of those within `W`, so the person eases out and in, and never
//! stands nearer or less far round than its own frame asks.
use super::{bounds::Crowns, camera, path::Path};
use telperion_render::{figure_lit, figure_reach, Placement};

/// Half the smoothing window, in frames: one second at the film's 30.
pub const W: usize = 30;
/// The step of the angle search, in degrees.
const STEP: f64 = 1.0;
/// The farthest round from the camera the person may stand, in degrees.
pub const CAP: f64 = 180.0;

/// What a frame shows the person beside: the camera's bearing and the
/// frame's crowns.
#[derive(Debug, Clone, Copy)]
pub struct Shown {
    pub view: f64,
    pub crowns: Crowns,
}

impl Shown {
    /// The bearing `angle` degrees round to the camera's left.
    pub fn bearing(&self, angle: f64) -> f64 {
        self.view - angle
    }

    /// The crown's reach at `angle`, the later year's faded in.
    pub fn reach(&self, angle: f64) -> f64 {
        let (earlier, later, share) = self.crowns;
        let bearing = self.bearing(angle);
        let (a, b) = (figure_reach(earlier, bearing), figure_reach(later, bearing));
        a + (b - a) * share
    }

    /// The person at `angle` and `distance`, facing the camera.
    pub fn at(&self, angle: f64, distance: f64) -> Placement {
        Placement::at(self.bearing(angle), distance, self.view)
    }

    /// Whether the person at `placement` is out of every crown's shadow
    /// the frame shows.
    pub fn lit(&self, placement: Placement, sun: (f64, f64)) -> bool {
        let (earlier, later, share) = self.crowns;
        figure_lit(earlier, placement, sun) && (share == 0.0 || figure_lit(later, placement, sun))
    }

    /// The smallest angle, from 90 degrees in `STEP`s up to `CAP`, at which
    /// the person at its reach is lit; none where no angle serves.
    pub fn needs(&self, sun: (f64, f64)) -> Option<f64> {
        let steps = ((CAP - 90.0) / STEP).round() as u32;
        (0..=steps)
            .map(|k| 90.0 + STEP * f64::from(k))
            .find(|&angle| self.lit(self.at(angle, self.reach(angle)), sun))
    }
}

/// Each value raised to the largest within `2 W` frames, then the mean of
/// those within `W`: never below the value itself.
pub fn smoothed(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    let window = |f: usize, w: usize| f.saturating_sub(w)..(f + w + 1).min(n);
    let most: Vec<f64> = (0..n)
        .map(|f| {
            values[window(f, 2 * W)]
                .iter()
                .fold(f64::MIN, |m, v| m.max(*v))
        })
        .collect();
    (0..n)
        .map(|f| {
            let near = &most[window(f, W)];
            near.iter().sum::<f64>() / near.len() as f64
        })
        .collect()
}

/// Every frame's angle as it needs it and as smoothed, its reach at the
/// smoothed angle, its distance as smoothed, and where the person stands.
pub struct Standing {
    pub needs: Vec<f64>,
    pub angle: Vec<f64>,
    pub reach: Vec<f64>,
    pub distance: Vec<f64>,
    pub placements: Vec<Placement>,
}

/// Where the person stands in every frame, or the frames no angle serves.
pub fn stand(shown: &[Shown], sun: (f64, f64)) -> Result<Standing, Vec<usize>> {
    let needs: Vec<Option<f64>> = shown.iter().map(|s| s.needs(sun)).collect();
    let unserved: Vec<usize> = (0..shown.len()).filter(|&f| needs[f].is_none()).collect();
    if !unserved.is_empty() {
        return Err(unserved);
    }
    let needs: Vec<f64> = needs.into_iter().flatten().collect();
    let angle = smoothed(&needs);
    let reach: Vec<f64> = shown.iter().zip(&angle).map(|(s, &a)| s.reach(a)).collect();
    let distance = smoothed(&reach);
    let placements = (shown.iter().enumerate())
        .map(|(f, s)| s.at(angle[f], distance[f]))
        .collect();
    Ok(Standing {
        needs,
        angle,
        reach,
        distance,
        placements,
    })
}

/// Where the person stands in every frame of the film, its largest angle
/// and frame-to-frame move printed; an error naming the frames no angle to
/// the camera's left serves.
pub fn of_film(path: &Path, crowns: &[Crowns]) -> Result<Standing, String> {
    let shown: Vec<Shown> = (crowns.iter().enumerate())
        .map(|(i, &crowns)| Shown {
            view: camera::azimuth(&path.camera, i as f64 / f64::from(path.fps)),
            crowns,
        })
        .collect();
    let standing = stand(&shown, path.sun)
        .map_err(|frames| format!("no angle to the camera's left serves frames {frames:?}"))?;
    let moved = (standing.placements.windows(2))
        .map(|p| (p[1].foot - p[0].foot).length())
        .fold(0.0, f64::max);
    let widest = standing.angle.iter().fold(0.0, |m: f64, a| m.max(*a));
    println!("person: largest angle {widest:.1} degrees, largest move {moved:.3} m a frame");
    Ok(standing)
}
