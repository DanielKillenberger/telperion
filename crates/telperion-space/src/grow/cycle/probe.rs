//! R3's frozen comparison (DESIGN.md section 4 (a)): every candidate read
//! on the same leaves and the same buds as the lattice, every cycle, the
//! lattice driving growth, so foliage, buds and allocation are the same
//! for both.
use super::{lit, Grower};
use crate::light::volumes::Candidate;
use crate::structure::Vec3;

/// Buds kept a cycle for the error's distribution, and those read
/// exhaustively as well (every group's kernel, no aggregate).
const SAMPLE: usize = 4096;
const EXHAUSTIVE: usize = 16;

/// Each candidate's measures, and the seconds the lattice took.
pub struct Probe {
    pub lattice_seconds: f64,
    pub measures: Vec<Measure>,
}

/// A candidate against the lattice over a run.
pub struct Measure {
    pub candidate: Candidate,
    /// Buds read, the sums of its light less the lattice's and of that
    /// difference's size, and its largest size.
    pub buds: u64,
    pub sum: f64,
    pub sum_abs: f64,
    pub max_abs: f64,
    /// Nodes visited, summed over every bud and at most.
    pub visited: u64,
    pub visited_max: u32,
    /// A sample of buds: the lattice's light, the candidate's and the
    /// nodes it visited.
    pub sample: Vec<[f32; 3]>,
    /// Fewer buds: the lattice's, the candidate's and the exhaustive read.
    pub exhaustive: Vec<[f32; 3]>,
    /// Seconds building and reading.
    pub build: f64,
    pub read: f64,
    /// Each cycle's buds, mean absolute difference, mean and largest
    /// visits, and the nodes the volumes held.
    pub cycles: Vec<[f64; 5]>,
}

impl Probe {
    pub(super) fn new(candidates: &[Candidate]) -> Self {
        let measures = candidates
            .iter()
            .map(|&candidate| Measure {
                candidate,
                buds: 0,
                sum: 0.0,
                sum_abs: 0.0,
                max_abs: 0.0,
                visited: 0,
                visited_max: 0,
                sample: Vec::new(),
                exhaustive: Vec::new(),
                build: 0.0,
                read: 0.0,
                cycles: Vec::new(),
            })
            .collect();
        Self {
            lattice_seconds: 0.0,
            measures,
        }
    }

    /// Every candidate read at `tips` beside the lattice's `lattice`.
    pub(super) fn measure(&mut self, g: &Grower, tips: &[Vec3], lattice: &[f64], started: f64) {
        self.lattice_seconds += crate::clock::seconds() - started;
        let light = g.sketch.as_ref().expect("sketching").light;
        let (directions, k, threads) = (lit(&light), light.extinction, g.threads);
        for m in &mut self.measures {
            let begun = crate::clock::seconds();
            let volumes = g.volumes(m.candidate);
            let built = crate::clock::seconds();
            let mut read = vec![(0.0, 0u32); tips.len()];
            crate::cores::fill(&mut read, 1 << 8, threads, &|i| volumes.read(tips[i], &directions, k));
            m.build += built - begun;
            m.read += crate::clock::seconds() - built;
            let mut cycle = [tips.len() as f64, 0.0, 0.0, 0.0, volumes.len() as f64];
            let stride = tips.len().div_ceil(SAMPLE).max(1);
            for (i, (&l, &(v, visited))) in lattice.iter().zip(&read).enumerate() {
                let d = v - l;
                (m.sum, m.sum_abs, m.max_abs) = (m.sum + d, m.sum_abs + d.abs(), m.max_abs.max(d.abs()));
                m.visited += u64::from(visited);
                m.visited_max = m.visited_max.max(visited);
                cycle[1] += d.abs();
                cycle[2] += f64::from(visited);
                cycle[3] = cycle[3].max(f64::from(visited));
                if i % stride == 0 {
                    m.sample.push([l as f32, v as f32, visited as f32]);
                }
            }
            m.buds += tips.len() as u64;
            if !tips.is_empty() {
                cycle[1] /= cycle[0];
                cycle[2] /= cycle[0];
            }
            m.cycles.push(cycle);
            let stride = tips.len().div_ceil(EXHAUSTIVE).max(1);
            for i in (0..tips.len()).step_by(stride) {
                let all = volumes.exhaustive(tips[i], &directions, k);
                m.exhaustive.push([lattice[i] as f32, read[i].0 as f32, all as f32]);
            }
        }
    }
}
