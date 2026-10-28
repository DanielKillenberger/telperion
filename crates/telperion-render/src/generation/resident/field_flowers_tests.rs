//! fn-319 R5: a flowering tree's florets boxed and counted on the device,
//! none read back. A seat outside the grid is refused as the CPU refuses
//! it, a flowering tree with no seats answers as one without flowers, and
//! the grid and its answers at the largest side are held to the device's
//! limits on both sides of them.
use super::super::Generator;
use super::field::words;
use super::field_tests::{family, generator, references};
use crate::{Gpu, RenderError};
use telperion_core::field::Frame;
use telperion_core::foliage::BloomParams;
use telperion_core::math::Vec3;
use telperion_core::presets::{Family, Preset};

/// The beech young and small, flowering on its two finest ages at
/// `share` (the core's `suite/flowers.rs`).
fn blossoming(share: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    f.age = 15.0;
    let species = f.space.species.as_mut().expect("an engine species");
    for age in [12, 14] {
        species.states[age].flowering = share;
    }
    f.bloom = BloomParams {
        florets: 4.0,
        pedicel: 0.025,
        droop: 0.6,
        petals: 5.0,
        ..BloomParams::default()
    };
    f
}

/// A grid that holds none of the tree's florets is refused by name, as
/// the CPU's `florets_into` refuses it, never read out without them.
#[test]
fn a_floret_outside_the_field_is_refused_by_name() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let far = Frame {
        n: 3,
        cell: 1e-3,
        origin: Vec3::new(1e3, 1e3, 1e3),
    };
    let flowering = blossoming(0.4);
    let ask = gen.grow_field_in(&flowering, far, None);
    let refused = pollster::block_on(ask).expect_err("refused");
    assert!(
        refused.to_string().contains("a floret outside the field"),
        "{refused}"
    );
}

/// A tree that flowers but keeps no seat, as the CPU counts it, answers
/// to the bit as the same tree without flowers: its box and every count.
#[test]
fn a_flowering_tree_with_no_seats_answers_as_one_without_flowers() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let seatless = blossoming(1e-6);
    let [cpu] = <[_; 1]>::try_from(references(&seatless, &[24], None))
        .ok()
        .unwrap();
    assert!(cpu.cells.iter().all(|c| c.flowers == 0.0), "a seat hangs");
    let grow = |f: &Family| pollster::block_on(gen.grow_field(f, 24, None)).expect("read out");
    assert!(grow(&seatless) == grow(&blossoming(0.0)), "the grid moved");
}

/// A device whose buffers stop at exactly the largest side's need, or one
/// word short of it: `within` counts the cells' fifth word.
fn limited(bytes: u64) -> Option<Generator> {
    let limits = |available: &wgpu::Limits| wgpu::Limits {
        max_buffer_size: bytes,
        max_storage_buffer_binding_size: bytes,
        ..available.clone()
    };
    let gpu = match pollster::block_on(Gpu::request_within(None, limits)) {
        Ok(gpu) => gpu,
        Err(error) => {
            println!("skipped: {error}");
            return None;
        }
    };
    Some(pollster::block_on(Generator::for_fields_async(gpu)).expect("a generator"))
}

/// At 256 cells a side the grid, five words a cell beside its three, is
/// the largest buffer, its answers next: a device that holds it grows the
/// field, one a word short refuses it by name before any work.
#[test]
fn the_largest_side_grows_or_refuses_as_the_device_s_limits_say() {
    let _held = super::super::whole_tree();
    let (grid, answers) = words(256u64.pow(3));
    assert_eq!((grid, answers), (2 + 8 * (1 << 24), 5 * (1 << 24) + 4));
    let need = 4 * grid.max(answers);
    let palm = family("date-palm", 1);
    let Some(short) = limited(need - 4) else {
        return;
    };
    match pollster::block_on(short.grow_field(&palm, 256, None)) {
        Err(RenderError::Oversize { buffer, bytes, .. }) => {
            assert_eq!((buffer, bytes), ("field grid", need));
        }
        other => panic!("not refused by its limit: {:?}", other.map(|g| g.frame)),
    }
    drop(short);
    let held = limited(need).expect("the same adapter");
    let grown = pollster::block_on(held.grow_field(&palm, 256, None)).expect("read out");
    assert_eq!(grown.cells.len(), 1 << 24);
}
