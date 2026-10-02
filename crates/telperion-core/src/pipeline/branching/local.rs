use super::*;
use crate::math::Transcendental;
use std::{f64::consts::TAU, rc::Rc};
#[derive(Clone)]
struct Run {
    positions: Vec<Vec3>,
    fractions: Vec<f64>,
    length: f64,
}
#[derive(Clone)]
struct Shoot {
    flushed: u16,
    accepted: Vec<Vec3>,
    at: usize,
    direction: Vec3,
    normal: Vec3,
    phase: f64,
    radius: f64,
    length: f64,
    branch: Option<u32>,
    completed: usize,
    generation: usize,
    internodes: usize,
    key: u32,
    run: Option<Rc<Run>>,
    curtain: pendant::Curtain,
}
mod advance;
pub(super) mod detail;
mod pendant;
mod planner;
mod seed;
#[cfg(test)]
pub use pendant::in_band;
use pendant::Curtain;
pub(super) use planner::Planner;
use planner::Axis;
#[derive(Clone, Default)]
pub(super) struct Frontier {
    queue: std::collections::VecDeque<Shoot>,
    seeded: std::collections::HashMap<u64, u16>,
    stations: seed::Stations,
    /// The detail a direct build at its node budget kept; none grows every
    /// order the rows allow. Chosen per build and never stored.
    pub(super) detail: Option<detail::Detail>,
    /// Whether every stem apex bears a rosette, and so no twig: its stations
    /// are never seeded. Chosen per build and never stored.
    pub(super) crowned: bool,
    #[cfg(test)]
    pub(super) retries: [usize; 4],
}
impl Frontier {
    pub(super) fn finished(&self) -> bool {
        self.queue.is_empty()
    }
    pub(super) fn remap(&mut self, index: &[Option<u32>]) {
        self.stations.remap(index);
        let remap = |s: &mut Shoot| {
            let Some(at) = index[s.at] else { return false };
            s.at = at as usize;
            if let Some(branch) = s.branch {
                let Some(branch) = index[branch as usize] else {
                    return false;
                };
                s.branch = Some(branch);
            }
            true
        };
        self.queue.retain_mut(remap);
    }
}
#[cfg(test)]
mod append;
#[cfg(test)]
pub use append::append;

