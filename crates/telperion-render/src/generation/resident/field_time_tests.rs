//! fn-257 task 11: the site field's time and its answers' bytes, so two
//! binaries can be run alternately and compared (FIELD-TIME.md). Each
//! field is grown at seed 1 through `grow_field` and asked through
//! `query_field`, the calls the package's `growField` and `query` make,
//! over a grid on the field's own box. `FIELD_PRESETS` names a comma list.
use super::super::{Clock, Generator};
use super::field::DeviceField;
use super::field_tests::grid;
use telperion_core::foliage::Bounds;
use telperion_core::math::Vec3;

pub(super) fn presets(or: &str) -> Vec<String> {
    let only = std::env::var("FIELD_PRESETS").unwrap_or_else(|_| or.into());
    only.split(',').map(str::to_string).collect()
}

pub(super) fn grown(gen: &Generator, id: &str) -> DeviceField {
    let mut family = telperion_core::presets::by_identity(id).expect("a preset");
    family.skeleton.seed = 1;
    pollster::block_on(gen.grow_field(&family, None)).expect("grown")
}

/// A grid of `n` cells a side over the field's box, four numbers a cell.
pub(super) fn cells(field: &DeviceField, n: u32) -> Vec<f32> {
    let b = field.bounds.map(f64::from);
    let bounds = Bounds {
        min: Vec3::new(b[0], b[1], b[2]),
        max: Vec3::new(b[3], b[4], b[5]),
    };
    let cell = |(c, h): (Vec3, f64)| [c.x as f32, c.y as f32, c.z as f32, h as f32];
    grid(&bounds, n).into_iter().flat_map(cell).collect()
}

/// Warm, per preset: the second of two growths and a 36-cell query of
/// it, one `FIELD_TIME <id> <grow ms> <query ms>` line each.
#[test]
#[ignore = "a measurement"]
fn field_time() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu)).expect("a generator");
    for id in presets("oregon-white-oak,silver-birch,norway-spruce") {
        drop(grown(&gen, &id));
        let t = Clock::now();
        let field = grown(&gen, &id);
        let grow = t.elapsed_ms();
        let asked = cells(&field, 36);
        let t = Clock::now();
        pollster::block_on(gen.query_field(&field, &asked)).expect("answered");
        println!("FIELD_TIME {id} {grow:.2} {:.2}", t.elapsed_ms());
    }
}

/// Every preset's box and answers at 24, 36 and 48 cells a side, hashed
/// to the bit (FNV-1a over their words): `FIELD_ANSWERS <id> <n> <box>
/// <hash>`, the same lines from two binaries when their fields agree.
#[test]
#[ignore = "a measurement"]
fn field_answers() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let gen = pollster::block_on(Generator::for_fields_async(gpu)).expect("a generator");
    let all: Vec<&str> = telperion_core::presets::CATALOGUE
        .iter()
        .map(|c| c.1)
        .collect();
    for id in presets(&all.join(",")) {
        let field = grown(&gen, &id);
        let corners: Vec<u32> = field.bounds.iter().map(|v| v.to_bits()).collect();
        for n in [24, 36, 48] {
            let answers = pollster::block_on(gen.query_field(&field, &cells(&field, n)));
            let mut hash = 0xcbf2_9ce4_8422_2325u64;
            for a in answers.expect("answered") {
                let flags = u32::from(a.wood) | u32::from(a.foliage) << 1;
                let limb = a.limb.unwrap_or(u32::MAX);
                for w in [flags, a.wood_radius.to_bits(), a.leaves.to_bits(), limb] {
                    for byte in w.to_le_bytes() {
                        hash = (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
                    }
                }
            }
            println!("FIELD_ANSWERS {id} {n} {corners:08x?} {hash:016x}");
        }
    }
}
