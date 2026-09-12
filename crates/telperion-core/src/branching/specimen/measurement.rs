//! Native CPU measurement using the same Instant clock as examples/measure.rs.
//! Test-only stage counters never affect simulation decisions.
use super::*;
use std::time::{Duration, Instant};

#[derive(Clone, Default)]
pub(super) struct Cost {
    pub stages: [Duration; 9],
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
fn monthly_cost_report() {
    if std::env::var_os("FN11_MEASURE").is_none() {
        return;
    }
    use crate::presets::Preset;
    for preset in [Preset::OregonWhiteOak, Preset::NorwaySpruce] {
        let mut family = preset.parameters();
        family.skeleton.seed = 7;
        let mature = family.growth.mature_month();
        for sample in 0..3 {
            let clock = Instant::now();
            let envelope = Specimen::grow(&family.skeleton, family.radii).unwrap();
            println!(
                "{preset:?} sample={sample} envelope_ms={:.6} nodes={} crossover={}",
                clock.elapsed().as_secs_f64() * 1000.0,
                envelope.tree.nodes.len(),
                envelope.tree.crossover
            );
            family.age = mature as f64 / 12.0;
            let clock = Instant::now();
            let s = Specimen::build(&family).unwrap();
            println!(
                "{preset:?} sample={sample} mature_ms={:.6} month={mature} nodes={} crossover={}",
                clock.elapsed().as_secs_f64() * 1000.0,
                s.tree.nodes.len(),
                s.tree.crossover
            );
        }
        family.age = 0.0;
        let mut s = Specimen::build(&family).unwrap();
        let mut small = None;
        let mut large = None;
        // Diff identity/radius tuples outside the timed advance. Measure every
        // slice so sparse work on large trees cannot be hidden by an average.
        for month in 1..=mature {
            let first_birth = s.next_identity;
            let before: Vec<_> = s
                .tree
                .nodes
                .iter()
                .map(|n| (n.identity, (n.radius, n.start_radius, n.base_radius)))
                .collect();
            let clock = Instant::now();
            s.advance(1.0 / 12.0).unwrap();
            let ms = clock.elapsed().as_secs_f64() * 1000.0;
            let born = s
                .tree
                .nodes
                .iter()
                .filter(|n| n.identity.birth_order() >= first_birth)
                .count();
            let changed = born
                + before
                    .iter()
                    .filter(|(id, previous)| {
                        s.node(*id)
                            .is_ok_and(|n| (n.radius, n.start_radius, n.base_radius) != *previous)
                    })
                    .count();
            let line = format!("{preset:?} month={month} active={} nodes_before={} nodes_after={} born={born} changed_or_born={changed} slice_ms={ms:.6} stages_ms={:?} identity_visits={} storage_moved={} pipe_visits={} width_visits={} crown_samples={}",
                family.growth.budget(month)>0, before.len(), s.tree.nodes.len(),
                s.cost.stages.map(|d| d.as_secs_f64()*1000.0), s.cost.identities,
                s.cost.storage, s.cost.pipes, s.cost.widths,
                s.timeline.as_ref().unwrap().crown.evaluated);
            if [241, 1201, mature].contains(&month) {
                println!("AGE {line}");
            }
            if before.len() > 50_000 && family.growth.budget(month) > 0 && changed > 0 {
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
    }
    println!("Stages: environment, scaffold, storage/identity, pipes+local-widths, local-seed/order, local-growth, identify+visited-vigour, shedding, final-widths. Snapshot unavailable; no round-trip time claimed.");
}
