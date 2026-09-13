//! Native CPU measurement using the same Instant clock as examples/measure.rs.
//! Test-only stage counters never affect simulation decisions.
use super::*;
use std::time::{Duration, Instant};

#[derive(Clone, Default)]
pub(super) struct Cost {
    pub stages: [Duration; 9],
    pub packing: Duration,
    pub finalizing: Duration,
    pub changes: Duration,
    pub packed_nodes: usize,
    pub identities: usize,
    pub storage: usize,
    pub pipes: usize,
    pub widths: usize,
    pub keyframe_pipes: usize,
    pub keyframe_widths: usize,
}
impl Cost {
    pub fn stamp(&mut self, slot: usize, clock: &mut Instant) {
        self.stages[slot] = clock.elapsed();
        *clock = Instant::now();
    }
}

#[test]
// Historical command name retained for the R10 evidence trail; slices are annual.
fn monthly_cost_report() {
    if std::env::var_os("FN11_MEASURE").is_none() {
        return;
    }
    use crate::presets::Preset;
    // The sparse years are the recorded session-12 comparison points. The final
    // envelope quantum is the dense case. These are evidence inputs, not rules.
    for (preset, sparse) in [(Preset::OregonWhiteOak, 66), (Preset::NorwaySpruce, 67)] {
        let mut family = preset.parameters();
        family.skeleton.seed = 7;
        let mature = family.growth.mature_slice();
        for sample in 0..3 {
            let clock = Instant::now();
            let envelope = Specimen::grow(&family.skeleton, family.radii).unwrap();
            let ms = clock.elapsed().as_secs_f64() * 1000.0;
            println!("R10 preset={preset:?} sample={sample} kind=envelope ms={ms:.6} nodes={} crossover={} bounds={:?}",
                envelope.tree().nodes.len(), envelope.tree().crossover, bounds(envelope.tree()));
            drop(envelope);
            family.age = mature as f64;
            let clock = Instant::now();
            let s = Specimen::build(&family).unwrap();
            let ms = clock.elapsed().as_secs_f64() * 1000.0;
            println!("R10 preset={preset:?} sample={sample} kind=mature ms={ms:.6} age={mature} nodes={} crossover={} bounds={:?}",
                s.tree().nodes.len(), s.tree().crossover, bounds(s.tree()));
            if preset == Preset::OregonWhiteOak {
                for (kind, age) in [("read_past", sparse), ("read_frontier", mature)] {
                    let clock = Instant::now();
                    let read = s.read_at_age(age as f64).unwrap();
                    let ms = clock.elapsed().as_secs_f64() * 1000.0;
                    println!("R10 preset={preset:?} sample={sample} kind={kind} ms={ms:.6} age={age} nodes={} placements={}",
                        read.tree.nodes.len(), read.placements.len());
                    drop(read);
                }
                let clock = Instant::now();
                let record = s
                    .changes_between((sparse - 1) as f64, sparse as f64)
                    .unwrap();
                let ms = clock.elapsed().as_secs_f64() * 1000.0;
                println!("R10 preset={preset:?} sample={sample} kind=record_past ms={ms:.6} from={} to={sparse} runs={} placements={}",
                    sparse - 1, record.born_runs.len() + record.resized_runs.len(),
                    record.born_placements.len() + record.moved_placements.len());
            }
        }
        for (kind, year) in [("sparse", sparse), ("dense", mature)] {
            for sample in 0..3 {
                family.age = (year - 1) as f64;
                let mut s = Specimen::build(&family).unwrap();
                let before = s.tree().nodes.len();
                let clock = Instant::now();
                let record = s.advance(1.0).unwrap();
                let ms = clock.elapsed().as_secs_f64() * 1000.0;
                let clock = Instant::now();
                let after = s.tree().nodes.len();
                let packed_ms = clock.elapsed().as_secs_f64() * 1000.0;
                let changed = s
                    .tree
                    .nodes
                    .iter()
                    .filter(|n| s.keyframes.changed(n.identity, year - 1, year))
                    .count();
                let internal = s.cost.stages.iter().sum::<Duration>().as_secs_f64() * 1000.0;
                println!("R10 preset={preset:?} sample={sample} kind={kind} ms={ms:.6} year={year} nodes_before={before} nodes_after={after} packed_read_ms={packed_ms:.6} changed={changed} internal_ms={internal:.6} finalizing_ms={:.6} record_ms={:.6} keyframe_pipes={} keyframe_widths={} runs={} placements={} stages_ms={:?}",
                    s.cost.finalizing.as_secs_f64() * 1000.0,
                    s.cost.changes.as_secs_f64() * 1000.0, s.cost.keyframe_pipes, s.cost.keyframe_widths,
                    record.born_runs.len() + record.resized_runs.len(),
                    record.born_placements.len() + record.moved_placements.len(),
                    s.cost.stages.map(|d| d.as_secs_f64() * 1000.0));
            }
        }
    }
    println!("Build timings are chronicle/wood generation, matching the historical envelope comparison; full foliage output is reported separately. Advance timing includes record construction, excludes caller record destruction. Stages: environment, scaffold, storage/identity, pipe-record, local-seed, local-growth, visited-vigour, shedding, annual-keyframes. record_ms is the stamp filter plus selected transforms. No snapshots exist; no round-trip timing claimed.");
}

fn bounds(tree: &Tree) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for n in &tree.nodes {
        for (i, v) in [n.position.x, n.position.y, n.position.z]
            .into_iter()
            .enumerate()
        {
            lo[i] = lo[i].min(v);
            hi[i] = hi[i].max(v);
        }
    }
    (lo, hi)
}
