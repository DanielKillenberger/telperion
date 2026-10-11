//! The device's settle held to the CPU's (`grow_capture_settled_at`):
//! young trees of every shipped preset grown on the device from the seed
//! and settled there, exact in every word where the kernels are built
//! exact, and reported by lineage where they are built plain.
use super::build::{Build, Kit, Tables};
use super::columns::Cap;
use super::compare::Agreement;
use super::grown::Snapshot;
use super::grown_compare::{by_lineage, exact};
use super::grown_tests::{built, device, grown, Grown, YOUNG};
use super::settle_device::Kernels;
use super::settle_read::{beside_agree, cpu_settled, Beside};
use super::settle_run::{settle, Settled};
use super::Arith;
use crate::Gpu;
use telperion_space::SettleTo;

/// Each PA's settle constants (`grow_capture_settle_tables`).
pub(super) fn constants(g: &Grown) -> Vec<u32> {
    Cap(&telperion_space::grow_capture_settle_tables(&g.species)).u32s("settle_pa")
}

/// The device's tree grown and settled, read back.
pub(super) fn settled(gpu: &Gpu, (kit, k): (&Kit, &Kernels), g: &Grown) -> (Build, Settled, Snapshot, Beside) {
    let (b, _) = built(gpu, kit, g, g.request.age);
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let s = settle(gpu, (kit, k), &b, &t, &constants(g)).expect("a settle");
    let (snap, beside) = s.snapshot(gpu, &b.state).expect("a readback");
    (b, s, snap, beside)
}

/// The device's settle against the CPU's at `to`.
fn agreement(dev: (&Snapshot, &Beside), cpu: (&Snapshot, &Beside)) -> Agreement {
    let mut a = exact(dev.0, cpu.0);
    beside_agree(&mut a, dev.1, cpu.1);
    a
}

#[test]
fn young_trees_shed_and_order_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Exact) else {
        return;
    };
    let k = Kernels::new(&gpu, Arith::Exact);
    let mut differ = Vec::new();
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Shed).expect("the CPU's tree");
        let cpu = cpu_settled(&Cap(&capture));
        let (_, s, dev, beside) = settled(&gpu, (&kit, &k), &g);
        let a = agreement((&dev, &beside), (&cpu.0, &cpu.1));
        let t = &s.table;
        let depths = s.kept_starts.len() - 1;
        println!(
            "{name} {age}: {} kept axes in {depths} depths, {} units, {} phytomers; flags {}; {} dispatches\n{}",
            t.kept, t.units, t.phyts, s.flags, s.stamped.len(), a.report()
        );
        if !a.exact() {
            println!("{}", by_lineage(&dev, &cpu.0).report());
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "the device's settled trees are not the CPU's: {differ:?}");
}

/// Built plain, the same trees by lineage: reported, not bounded.
#[test]
fn young_trees_settled_plain_are_reported() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Plain) else {
        return;
    };
    let k = Kernels::new(&gpu, Arith::Plain);
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Shed).expect("the CPU's tree");
        let cpu = cpu_settled(&Cap(&capture));
        let (_, _, dev, beside) = settled(&gpu, (&kit, &k), &g);
        let a = agreement((&dev, &beside), (&cpu.0, &cpu.1));
        let parted: Vec<&str> = a.fields.iter().filter(|f| f.2 > 0).map(|f| f.0.as_str()).collect();
        println!("{name} {age}: fields that part {parted:?}\n{}", by_lineage(&dev, &cpu.0).report());
    }
}

/// A tree that grows no phytomer settles to the CPU's `Collapsed`: the
/// date palm with every zone's nodes none.
#[test]
fn a_tree_without_phytomers_collapses_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Exact) else {
        return;
    };
    let k = Kernels::new(&gpu, Arith::Exact);
    let mut g = grown(telperion_core::presets::Preset::DatePalm, 3, 42);
    for state in &mut g.species.states {
        for zone in &mut state.zones {
            zone.nodes = telperion_space::NodeLaw::Uniform { min: 0.0, max: 0.0 };
        }
    }
    g.tables = telperion_space::grow_capture_tables(&g.species, g.request).expect("tables");
    g.chain = telperion_space::ChainTables::cycle(&g.species, g.request.age).expect("chain tables");
    let capture = telperion_space::grow_capture_settled_at(&g.species, g.request, SettleTo::Shed).expect("the CPU's tree");
    let cpu = cpu_settled(&Cap(&capture));
    let (_, s, dev, beside) = settled(&gpu, (&kit, &k), &g);
    assert!(s.collapsed, "the device's tree collapsed");
    let a = agreement((&dev, &beside), (&cpu.0, &cpu.1));
    println!("{}", a.report());
    assert!(a.exact(), "the device's collapse is the CPU's");
}
