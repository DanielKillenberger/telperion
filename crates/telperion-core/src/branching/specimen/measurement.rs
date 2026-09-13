//! Native CPU measurement using the same Instant clock as examples/measure.rs.
//! Test-only stage counters never affect simulation decisions.
use super::*;
use std::time::{Duration, Instant};

#[derive(Clone, Default)]
pub(super) struct Cost {
    pub stages: [Duration; 9],
    pub packing: Duration,
    pub finalizing: Duration,
    pub packed_nodes: usize,
    pub identities: usize,
    pub storage: usize,
    pub pipes: usize,
    pub widths: usize,
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
    for preset in [Preset::OregonWhiteOak, Preset::NorwaySpruce] {
        let mut family = preset.parameters();
        family.skeleton.seed = 7;
        let mature = family.growth.mature_slice();
        for sample in 0..3 {
            let clock = Instant::now();
            let envelope = Specimen::grow(&family.skeleton, family.radii).unwrap();
            println!(
                "{preset:?} sample={sample} envelope_ms={:.6} nodes={} crossover={} bounds={:?}",
                clock.elapsed().as_secs_f64() * 1000.0,
                envelope.tree.nodes.len(),
                envelope.tree.crossover,
                bounds(&envelope.tree)
            );
            family.age = mature as f64;
            let clock = Instant::now();
            let s = Specimen::build(&family).unwrap();
            let build_ms = clock.elapsed().as_secs_f64() * 1000.0;
            let clock = Instant::now();
            s.tree();
            let read_ms = clock.elapsed().as_secs_f64() * 1000.0;
            println!(
                "{preset:?} sample={sample} mature_ms={build_ms:.6} consumer_read_ms={read_ms:.6} slice={mature} nodes={} crossover={} bounds={:?}",
                s.tree.nodes.len(),
                s.tree.crossover,
                bounds(&s.tree)
            );
        }
        family.age = 0.0;
        let mut s = Specimen::build(&family).unwrap();
        let mut small = None;
        let mut large = None;
        // Diff identity/radius tuples outside the timed advance. Measure every
        // slice so sparse work on large trees cannot be hidden by an average.
        for slice in 1..=mature {
            let first_birth = s.next_identity;
            let before: Vec<_> = s
                .tree
                .nodes
                .iter()
                .map(|n| (n.identity, (n.radius, n.start_radius, n.base_radius)))
                .collect();
            let clock = Instant::now();
            s.advance(1.0).unwrap();
            let ms = clock.elapsed().as_secs_f64() * 1000.0;
            let born = s
                .tree
                .nodes
                .iter()
                .filter(|n| n.identity.birth_order() >= first_birth)
                .count();
            let read_clock = Instant::now();
            s.tree();
            let read_ms = read_clock.elapsed().as_secs_f64() * 1000.0;
            let changed = born
                + before
                    .iter()
                    .filter(|(id, previous)| {
                        s.node(*id)
                            .is_ok_and(|n| (n.radius, n.start_radius, n.base_radius) != *previous)
                    })
                    .count();
            let line = format!("{preset:?} slice={slice} active={} nodes_before={} nodes_after={} born={born} changed_or_born={changed} advance_ms={ms:.6} internal_slice_ms={:.6} read_ms={read_ms:.6} finalizing_ms={:.6} packing_ms={:.6} packed_nodes={} stages_ms={:?} identity_visits={} storage_moved={} pipe_visits={} width_visits={} crown_samples={} local_attempts={:?}",
                family.growth.budget(slice)>0, before.len(), s.tree.nodes.len(),
                s.cost.stages.iter().sum::<Duration>().as_secs_f64()*1000.0,
                s.cost.finalizing.as_secs_f64()*1000.0,
                s.cost.packing.as_secs_f64()*1000.0, s.cost.packed_nodes,
                s.cost.stages.map(|d| d.as_secs_f64()*1000.0), s.cost.identities,
                s.cost.storage, s.cost.pipes, s.cost.widths,
                s.timeline.as_ref().unwrap().crown.evaluated, s.local.retries);
            if [21, 101, mature].contains(&slice) {
                println!("AGE {line}");
            }
            if before.len() > 50_000 && family.growth.budget(slice) > 0 && changed > 0 {
                if small.as_ref().is_none_or(|(n, _)| changed < *n) {
                    small = Some((changed, line.clone()));
                }
                if large.as_ref().is_none_or(|(n, _)| changed > *n) {
                    large = Some((changed, line));
                }
            }
        }
        println!("SPARSE {}", small.unwrap().1);
        println!("DENSE {}", large.unwrap().1);
        let clock = Instant::now();
        s.advance(crate::growth::MAX_AGE - s.age()).unwrap();
        println!(
            "{preset:?} saturated_advance_ms={:.6}",
            clock.elapsed().as_secs_f64() * 1000.0
        );
        // Fresh foliage reads are separate consumer costs. Recording the count
        // also exposes the consequence of lifetime expiry as births approach zero.
        for age in [20.0, 100.0, mature as f64] {
            family.age = age;
            let specimen = Specimen::build(&family).unwrap();
            let clock = Instant::now();
            let leaves = specimen.placements().unwrap();
            println!(
                "FOLIAGE {preset:?} age={age} placements={} read_ms={:.6}",
                leaves.len(),
                clock.elapsed().as_secs_f64() * 1000.0
            );
        }
    }
    println!("Stages: environment, scaffold, storage/identity, pipe-record, local-seed+width-queries, local-growth/order+width-queries, identify+visited-vigour, shedding, post-shed-record. Output widths finalize once per advance (finalizing_ms). Lazy consumer packing is timed separately (read_ms); storage_moved counts insertion moves inside the slice. Snapshot unavailable; no round-trip time claimed.");
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
