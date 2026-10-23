//! Where the person stands in each frame of the film (fn-310 D7; host
//! rulings on the film's side and distance, 2026-10-23). The film is one
//! shot, so the person keeps to the camera's left throughout, square to the
//! view at the trunk's depth. Its distance on each frame is the crown's
//! reach and the margin, year-faded in a growth, or further out where the
//! figure would stand in the shadow of a crown the frame shows: the nearest
//! distance out from there, in `STEP`s, at which the whole figure is lit.
//! That distance is smoothed across the whole film: each frame takes the
//! largest within `2 W` frames, then the mean of those within `W`, so the
//! person eases out and in and never stands nearer than its own frame asks.
//! A crown's shadow is convex, so a figure lit at one distance out from the
//! trunk is lit at every distance beyond it.
use super::{bounds::Crowns, camera, path::Path};
use telperion_render::{figure_lit, figure_reach, Placement};

/// Half the smoothing window, in frames: one second at the film's 30.
pub const W: usize = 30;
/// The step of the search out of the shadow, in metres.
const STEP: f64 = 0.02;
/// How far beyond the reach the search goes before it gives up, in metres:
/// a sun above the horizon throws a shadow far shorter.
const SEARCH: f64 = 100.0;

/// What a frame shows the person beside: the camera's bearing and the
/// frame's crowns.
#[derive(Debug, Clone, Copy)]
pub struct Shown {
    pub view: f64,
    pub crowns: Crowns,
}

impl Shown {
    /// The bearing square to the view on the camera's left.
    pub fn bearing(&self) -> f64 {
        self.view - 90.0
    }

    /// The crown's reach and the margin on that bearing, the later year's
    /// faded in.
    pub fn reach(&self) -> f64 {
        let (earlier, later, share) = self.crowns;
        let (a, b) = (
            figure_reach(earlier, self.bearing()),
            figure_reach(later, self.bearing()),
        );
        a + (b - a) * share
    }

    /// The person `distance` out on the bearing, facing the camera.
    pub fn at(&self, distance: f64) -> Placement {
        Placement::at(self.bearing(), distance, self.view)
    }

    /// Whether the person at `placement` is out of every crown's shadow
    /// the frame shows.
    pub fn lit(&self, placement: Placement, sun: (f64, f64)) -> bool {
        let (earlier, later, share) = self.crowns;
        figure_lit(earlier, placement, sun) && (share == 0.0 || figure_lit(later, placement, sun))
    }

    /// The nearest distance, from the reach out in `STEP`s, at which the
    /// whole figure is lit; none within `SEARCH`.
    pub fn needs(&self, sun: (f64, f64)) -> Option<f64> {
        let reach = self.reach();
        let steps = (SEARCH / STEP) as u32;
        (0..=steps)
            .map(|k| reach + STEP * f64::from(k))
            .find(|&distance| self.lit(self.at(distance), sun))
    }
}

/// Each value raised to the largest within `2 W` frames, then the mean of
/// those within `W`, and never below the value itself, which the mean of
/// equal values can round to.
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
            (near.iter().sum::<f64>() / near.len() as f64).max(values[f])
        })
        .collect()
}

/// Every frame's reach, the distance it needs, and where the person
/// stands: at that distance as smoothed.
pub struct Standing {
    pub reach: Vec<f64>,
    pub needs: Vec<f64>,
    pub placements: Vec<Placement>,
}

/// Where the person stands in every frame, or the frames no distance
/// within `SEARCH` serves.
pub fn stand(shown: &[Shown], sun: (f64, f64)) -> Result<Standing, Vec<usize>> {
    let needs: Vec<Option<f64>> = shown.iter().map(|s| s.needs(sun)).collect();
    let unserved: Vec<usize> = (0..shown.len()).filter(|&f| needs[f].is_none()).collect();
    if !unserved.is_empty() {
        return Err(unserved);
    }
    let needs: Vec<f64> = needs.into_iter().flatten().collect();
    let distance = smoothed(&needs);
    let placements = (shown.iter().zip(&distance))
        .map(|(s, &d)| s.at(d))
        .collect();
    Ok(Standing {
        reach: shown.iter().map(Shown::reach).collect(),
        needs,
        placements,
    })
}

/// Where the person stands in every frame of the film, its largest step
/// out of the shadow and frame-to-frame move printed; an error naming the
/// frames no distance serves.
pub fn of_film(path: &Path, crowns: &[Crowns]) -> Result<Standing, String> {
    let shown: Vec<Shown> = (crowns.iter().enumerate())
        .map(|(i, &crowns)| Shown {
            view: camera::azimuth(&path.camera, i as f64 / f64::from(path.fps)),
            crowns,
        })
        .collect();
    let standing = stand(&shown, path.sun)
        .map_err(|frames| format!("no distance on the camera's left serves frames {frames:?}"))?;
    let moved = (standing.placements.windows(2))
        .map(|p| (p[1].foot - p[0].foot).length())
        .fold(0.0, f64::max);
    let out = (standing.needs.iter().zip(&standing.reach))
        .map(|(n, r)| n - r)
        .fold(0.0, f64::max);
    println!("person: out of the shadow by up to {out:.2} m, largest move {moved:.3} m a frame");
    Ok(standing)
}
