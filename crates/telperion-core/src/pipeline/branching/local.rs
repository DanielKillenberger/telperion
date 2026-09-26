use super::*;
use crate::math::Transcendental;
use std::{f64::consts::TAU, rc::Rc};
#[derive(Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
struct Run {
    positions: Vec<Vec3>,
    fractions: Vec<f64>,
    length: f64,
}
#[derive(Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
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
mod pendant;
mod planner;
mod seed;
#[cfg(test)]
pub use pendant::in_band;
use pendant::Curtain;
pub(super) mod waiting;
pub(super) use planner::Planner;
use planner::{rejected, Axis};
#[derive(Clone, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Frontier {
    queue: std::collections::VecDeque<Shoot>,
    sleeping: std::collections::BTreeMap<u64, Vec<Shoot>>,
    seeded: std::collections::HashMap<u64, u16>,
    stations: seed::Stations,
    visited: Vec<usize>,
    ordered: bool,
    #[cfg(test)]
    #[cfg_attr(feature = "json", serde(skip))]
    pub(super) retries: [usize; 4],
    #[cfg(test)]
    #[cfg_attr(feature = "json", serde(skip))]
    order_visits: usize,
}
impl Frontier {
    pub(super) fn visited(&self) -> impl Iterator<Item = usize> + '_ {
        self.visited.iter().copied()
    }
    #[cfg(test)]
    pub(in crate::pipeline::branching) fn reverse_for_test(&mut self) {
        self.queue.make_contiguous().reverse();
        self.ordered = false;
    }
    pub(super) fn identity_order(&mut self, tree: &Tree) {
        if self.ordered {
            return;
        }
        self.ordered = true;
        #[cfg(test)]
        {
            self.order_visits += self.queue.len();
        }
        self.queue
            .make_contiguous()
            .sort_by_key(|s| (tree.nodes[s.at].identity.birth_order(), s.key));
    }
    pub(super) fn finished(&self) -> bool {
        self.queue.is_empty() && self.sleeping.is_empty()
    }
    pub(super) fn remove_dead(&mut self, tree: &Tree, dead: &[usize]) {
        self.stations.remove_dead(tree, dead);
        let living = |s: &Shoot| {
            tree.nodes[s.at].shoot.death_year.is_none()
                && s.branch
                    .is_none_or(|b| tree.nodes[b as usize].shoot.death_year.is_none())
        };
        self.queue.retain(living);
        self.sleeping.retain(|_, shoots| {
            shoots.retain(living);
            !shoots.is_empty()
        });
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
        self.sleeping.retain(|_, shoots| {
            shoots.retain_mut(remap);
            !shoots.is_empty()
        });
    }
}
#[cfg(test)]
mod append;
#[cfg(test)]
pub use append::append;

#[cfg(test)]
mod monthly_tests;
