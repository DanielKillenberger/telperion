//! fn-257: the site field's time and its grid's bytes, so two binaries can
//! be run alternately and compared (FIELD-TIME.md). Each field is read out
//! at seed 1 through `grow_field`, the call the package's field entry
//! makes, over the device's own box. `FIELD_PRESETS` names a comma list.
use super::super::{Clock, Generator};
use super::field_tests::family;

fn presets(or: &str) -> Vec<String> {
    let only = std::env::var("FIELD_PRESETS").unwrap_or_else(|_| or.into());
    only.split(',').map(str::to_string).collect()
}

fn generator() -> Option<Generator> {
    let gpu = super::super::test_gpu()?;
    Some(pollster::block_on(Generator::for_fields_async(gpu)).expect("a generator"))
}

/// Warm, per preset: the second of two read-outs at 36 cells a side, one
/// `FIELD_TIME <id> <ms>` line each.
#[test]
#[ignore = "a measurement"]
fn field_time() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    for id in presets("oregon-white-oak,silver-birch,norway-spruce") {
        let family = family(&id, 1);
        pollster::block_on(gen.grow_field(&family, 36, None)).expect("read out");
        let t = Clock::now();
        pollster::block_on(gen.grow_field(&family, 36, None)).expect("read out");
        println!("FIELD_TIME {id} {:.2}", t.elapsed_ms());
    }
}

/// Every preset's grid at 24, 36 and 48 cells a side, hashed to the bit
/// (FNV-1a over its frame and its answers' words): `FIELD_ANSWERS <id> <n>
/// <hash>`, the same lines from two binaries when their fields agree.
#[test]
#[ignore = "a measurement"]
fn field_answers() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let all: Vec<&str> = telperion_core::presets::CATALOGUE
        .iter()
        .map(|c| c.1)
        .collect();
    for id in presets(&all.join(",")) {
        let family = family(&id, 1);
        for n in [24, 36, 48] {
            let grid = pollster::block_on(gen.grow_field(&family, n, None)).expect("read out");
            let f = grid.frame;
            let mut words = vec![f.cell.to_bits(), f.origin.x.to_bits(), f.origin.y.to_bits()];
            words.push(f.origin.z.to_bits());
            for a in &grid.cells {
                let flags = u64::from(a.wood) | u64::from(a.foliage) << 1;
                let limb = u64::from(a.limb.unwrap_or(u32::MAX));
                words.extend([flags, a.wood_radius.to_bits(), a.leaves.to_bits(), limb]);
            }
            let mut hash = 0xcbf2_9ce4_8422_2325u64;
            for byte in words.iter().flat_map(|w| w.to_le_bytes()) {
                hash = (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
            }
            println!("FIELD_ANSWERS {id} {n} {hash:016x}");
        }
    }
}
