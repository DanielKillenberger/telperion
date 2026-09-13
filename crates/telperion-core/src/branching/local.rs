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
    pendant: bool,
    curtain_across: Vec3,
    pendant_floor: Option<f64>,
}
mod advance;
mod planner;
mod seed;
pub(super) mod waiting;
use planner::rejected;
pub(super) use planner::Planner;
#[derive(Clone, Default)]
pub(super) struct Frontier {
    queue: std::collections::VecDeque<Shoot>,
    sleeping: std::collections::BTreeMap<u64, Vec<Shoot>>,
    seeded: std::collections::HashMap<u64, u16>,
    stations: seed::Stations,
    visited: Vec<usize>,
    ordered: bool,
    #[cfg(test)]
    pub(super) retries: [usize; 4],
    #[cfg(test)]
    order_visits: usize,
}
impl Frontier {
    pub(super) fn visited(&self) -> impl Iterator<Item = usize> + '_ {
        self.visited.iter().copied()
    }
    #[cfg(test)]
    pub(in crate::branching) fn reverse_for_test(&mut self) {
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
mod append;
pub use append::append;

#[cfg(test)]
mod monthly_tests;
