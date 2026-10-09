//! A species' growth units as the device reads them (fn-209 F1, task 2):
//! `telperion_space::Units`' zones packed into the table and words every
//! fine-root shader binds (`zones.wgsl`).
use telperion_space::{fixed, Units};

pub(super) const NONE: u32 = u32::MAX;
/// A zone's words in the table (`zones.wgsl`).
const ZW: u32 = 14;

/// The tables as the kernel reads them (`unit.wgsl`).
pub(super) fn pack(units: &Units) -> (Vec<u32>, Vec<u64>) {
    let mut table = Vec::new();
    let mut words = Vec::new();
    let zones: usize = units.zones.iter().map(Vec::len).sum();
    let mut at = 2 * units.zones.len() as u32;
    for zones in &units.zones {
        table.extend([at, zones.len() as u32]);
        at += ZW * zones.len() as u32;
    }
    assert_eq!(at as usize, table.len() + ZW as usize * zones);
    for zone in units.zones.iter().flatten() {
        let law = words.len() as u32;
        for (t, b) in zone.thresholds.iter().zip(&zone.bounds) {
            words.extend([*t, *b]);
        }
        // Each node's bound's side of its lead (`zones.wgsl`, `node_lead`),
        // by the contract on the CPU (task 8).
        assert_eq!(zone.bounds.len(), zone.thresholds.len(), "a bound a node");
        words.extend(zone.bounds.iter().map(|&b| match (b, zone.poisson) {
            (0, _) => 0,
            (b, true) => fixed::logit(b) as u64,
            (b, false) => fixed::log2(b) as u64,
        }));
        let laterals = words.len() as u32;
        words.extend(zone.laterals.iter().map(|t| t.unwrap_or(u64::MAX)));
        let windows = words.len() as u32;
        words.extend(zone.windows.iter().map(|&w| w as u64));
        let dormant = words.len() as u32;
        words.extend(zone.dormant.iter().map(|t| t.unwrap_or(u64::MAX)));
        let wake = zone.wake.map_or(NONE, |(delay, per)| {
            words.extend([delay, per as u64]);
            words.len() as u32 - 2
        });
        let sleeps = words.len() as u32;
        words.extend(zone.sleeps.iter().map(|&s| u64::from(s)));
        let placed = words.len() as u32;
        words.extend(&zone.placed);
        // Each fractional place's threshold's side of its lead
        // (`zones.wgsl`, `place_q`), by the contract on the CPU (fn-241).
        words.extend(zone.placed.iter().map(|&t| match t < 1 << 53 {
            true => fixed::logit(t << 9) as u64,
            false => 0,
        }));
        let windows_asleep = words.len() as u32;
        for cycle in &zone.sleep_windows {
            words.extend(cycle.iter().map(|&w| w as u64));
        }
        table.extend([
            zone.zone as u32,
            u32::from(zone.poisson),
            zone.thresholds.len() as u32,
            law,
            zone.places as u32,
            laterals,
            zone.laterals.len() as u32,
            windows,
            dormant,
            zone.dormant.len() as u32,
            wake,
            sleeps,
            placed,
            windows_asleep,
        ]);
    }
    (table, words)
}
