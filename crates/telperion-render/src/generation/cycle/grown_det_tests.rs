//! The device's grown state is the same in every word whatever runs it
//! (R2): over fresh device contexts, over the hierarchy's block caps and
//! over the ranges list dispatches are cut into. Built plain, the
//! default.
use super::build::Tables;
use super::build_grow::{grow_now as grow, Sizes};
use super::build_plan::Ranges;
use super::grown::Snapshot;
use super::grown_compare::exact;
use super::grown_tests::{device, grown, Grown};
use telperion_core::presets::Preset;

/// `g`'s tree on a fresh context from `sizes`.
fn fresh(g: &Grown, sizes: Sizes) -> Option<Snapshot> {
    let (gpu, kit) = device()?;
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    let age = g.request.age;
    let (b, ..) = grow(&gpu, &kit, &t, &g.tables, (sizes, age)).expect("a build");
    Some(b.snapshot(&gpu, age).expect("a readback"))
}

#[test]
fn the_grown_state_is_the_same_on_every_context_and_partition() {
    let _held = super::super::whole_tree();
    let mut differ = Vec::new();
    for (preset, name, age) in [
        (Preset::OregonWhiteOak, "oregon-white-oak", 9),
        (Preset::NorwaySpruce, "norway-spruce", 12),
    ] {
        let g = grown(preset, age, 42);
        let Some(first) = fresh(&g, Sizes::least()) else {
            return;
        };
        let mut runs = Vec::new();
        for k in 0..9 {
            runs.push((format!("context {}", k + 2), Sizes::least()));
        }
        for cap in [64, 128, 1024] {
            runs.push((
                format!("block cap {cap}"),
                Sizes {
                    block_cap: cap,
                    ..Sizes::least()
                },
            ));
        }
        let small = [
            Ranges {
                units: 64,
                draw: 128,
                deposit: 192,
            },
            Ranges {
                units: 1024,
                draw: 4096,
                deposit: 8192,
            },
        ];
        for ranges in small {
            let mut s = Sizes::least();
            s.bounds.ranges = ranges;
            runs.push((
                format!("{ranges:?}"),
                Sizes {
                    bounds: s.stated(),
                    ..s
                },
            ));
        }
        for (label, sizes) in runs {
            let again = fresh(&g, sizes).expect("a device");
            let a = exact(&again, &first);
            println!(
                "{name} {age}, {label}: {} fields, exact {}",
                a.fields.len(),
                a.exact()
            );
            if !a.exact() {
                println!("{}", a.report());
                differ.push(format!("{name} {label}"));
            }
        }
    }
    assert!(differ.is_empty(), "the grown state differs: {differ:?}");
}
