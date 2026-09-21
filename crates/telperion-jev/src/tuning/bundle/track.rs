//! Which dials move together, and where their sheet is taken.
//!
//! The 91 material rows were left out of every run while the five numbers
//! chose the move, because the numbers cannot see bark. The reviewer chooses
//! now, so they come back - but on a sheet of their own at the close trunk
//! view, because a bark change and a branching change on one whole-tree still
//! cannot be told apart.
use crate::tuning::actions::Dial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    pub name: String,
    /// The dial-table groups this track owns. A dial belongs to the first
    /// track listing its group; a dial whose group no track lists belongs to
    /// the first track.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// The view this track's sheet is always taken at. `None` keeps the rule
    /// a single-track round uses: a view a tuning priority named and some
    /// variant changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    /// Views to capture for this track's variants beyond the numeric
    /// reference views, so its sheet has a still to show at all.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_views: Vec<String>,
}

/// The one track a config that declares none is running: every dial, the
/// view rule the round has always used, nothing extra to capture.
pub fn implicit() -> Track {
    Track {
        name: String::new(),
        groups: vec![],
        view: None,
        extra_views: vec![],
    }
}

/// The tracks a round runs, in order.
pub fn all(declared: &[Track]) -> Vec<Track> {
    if declared.is_empty() {
        vec![implicit()]
    } else {
        declared.to_vec()
    }
}

/// Which track owns a dial: the first listing its group, else the first.
fn owner(tracks: &[Track], dials: &[Dial], id: &str) -> usize {
    dials
        .iter()
        .find(|d| d.id == id)
        .and_then(|d| d.group.as_deref())
        .and_then(|group| {
            tracks
                .iter()
                .position(|t| t.groups.iter().any(|g| g == group))
        })
        .unwrap_or(0)
}

/// The moves each track owns, in track order. A track with none is still
/// listed, so the round can say it was skipped.
pub fn assign(tracks: &[Track], dials: &[Dial], wanted: &[(String, i8)]) -> Vec<Vec<(String, i8)>> {
    let mut out = vec![vec![]; tracks.len().max(1)];
    for (id, sign) in wanted {
        out[owner(tracks, dials, id)].push((id.clone(), *sign));
    }
    out
}

/// Names are unique and present, and a fixed view is one the run assesses.
pub fn verify(tracks: &[Track], required: &[crate::tuning::state::Cell]) -> Result<(), String> {
    let mut seen = vec![];
    for track in tracks {
        if track.name.trim().is_empty() || seen.contains(&track.name) {
            return Err("a track has no name, or two tracks share one".into());
        }
        seen.push(track.name.clone());
        for view in track.view.iter().chain(track.extra_views.iter()) {
            if !required.iter().any(|c| &c.view == view) {
                return Err(format!(
                    "track {} names view {view}, which the run does not assess",
                    track.name
                ));
            }
        }
    }
    Ok(())
}
