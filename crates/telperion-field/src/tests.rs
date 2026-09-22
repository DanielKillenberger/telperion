//! The slim ABI against its contract: the refusals, the handle, a batch that
//! answers whole or not at all, and the same answers for the same request.
use super::*;

fn write_species(id: &str) {
    assert_eq!(species_alloc(id.len() as u32), 0);
    let dst = species_ptr().cast_mut();
    unsafe { std::ptr::copy_nonoverlapping(id.as_ptr(), dst, id.len()) };
}
fn error() -> String {
    let bytes = unsafe { std::slice::from_raw_parts(error_ptr(), error_len()) };
    String::from_utf8(bytes.to_vec()).unwrap()
}
fn bounds() -> Vec<f64> {
    unsafe { std::slice::from_raw_parts(bounds_ptr(), bounds_len()) }.to_vec()
}
/// A cubic grid of `n` cells a side over the standing tree's bounds.
fn write_grid(n: usize) {
    let b = bounds();
    let step = (0..3).map(|a| b[a + 3] - b[a]).fold(0_f64, f64::max) / n as f64;
    assert_eq!(query_alloc((n * n * n) as u32), 0);
    let cells = unsafe { std::slice::from_raw_parts_mut(query_ptr().cast_mut(), n * n * n * 4) };
    let mut i = 0;
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let at = |a: usize, c: usize| b[a] + (c as f64 + 0.5) * step;
                cells[i..i + 4].copy_from_slice(&[at(0, x), at(1, y), at(2, z), step / 2.]);
                i += 4;
            }
        }
    }
}
fn answers() -> (Vec<u8>, Vec<f32>, Vec<f32>, Vec<u32>) {
    let slice =
        |slot: u32| unsafe { std::slice::from_raw_parts(answer_ptr(slot), answer_len(slot)) };
    let f32s = |slot: u32| {
        let count = answer_len(slot);
        unsafe { std::slice::from_raw_parts(answer_ptr(slot).cast::<f32>(), count) }.to_vec()
    };
    let limbs = unsafe { std::slice::from_raw_parts(answer_ptr(3).cast::<u32>(), answer_len(3)) };
    (slice(0).to_vec(), f32s(1), f32s(2), limbs.to_vec())
}

#[test]
fn an_unknown_or_in_work_species_is_refused_with_the_core_error() {
    for id in ["no-such-tree", "european-beech", ""] {
        write_species(id);
        assert_eq!(grow(1, FAMILY_ORDER), 1, "{id}");
        assert_eq!(error(), "invalid input: preset identity");
        assert_eq!(bounds_len(), 0);
    }
    assert_eq!(species_alloc(SPECIES_LIMIT + 1), 1);
    assert_eq!(species_alloc(0), 0);
    assert_eq!(grow(1, FAMILY_ORDER), 1);
    assert_eq!(error(), "invalid input: preset identity");
}

#[test]
fn a_query_needs_a_standing_tree_at_its_revision() {
    release();
    assert_eq!(query_alloc(1), 0);
    assert_eq!(query(revision()), 1);
    assert_eq!(error(), "invalid input: no tree grown");
    write_species("ordinary");
    assert_eq!(grow(1, FAMILY_ORDER), 0, "{}", error());
    let handle = revision();
    assert_eq!(bounds_len(), 6);
    release();
    assert_eq!(bounds_len(), 0);
    assert_eq!(query_alloc(1), 0);
    assert_eq!(query(handle), 1);
    assert_eq!(error(), "invalid input: stale field handle");
}

#[test]
fn a_batch_with_one_bad_cell_answers_nothing() {
    write_species("ordinary");
    assert_eq!(grow(1, FAMILY_ORDER), 0, "{}", error());
    let b = bounds();
    assert_eq!(query_alloc(2), 0);
    let cells = unsafe { std::slice::from_raw_parts_mut(query_ptr().cast_mut(), 8) };
    cells.copy_from_slice(&[b[0], b[1], b[2], 0.5, b[3], b[4], b[5], -0.1]);
    assert_eq!(query(revision()), 1);
    assert_eq!(error(), "invalid input: field query");
    assert_eq!((0..4).map(|s| answer_len(s)).sum::<usize>(), 0);
    cells[7] = f64::NAN;
    assert_eq!(query(revision()), 1);
    assert_eq!((0..4).map(|s| answer_len(s)).sum::<usize>(), 0);
}

#[test]
fn the_same_species_and_seed_answer_identically_and_the_order_names_limbs() {
    write_species("ordinary");
    assert_eq!(grow(7, FAMILY_ORDER), 0, "{}", error());
    write_grid(8);
    assert_eq!(query(revision()), 0, "{}", error());
    let first = answers();
    assert_eq!(first.0.len(), 512);
    assert!(first.0.iter().any(|f| f & 1 != 0), "wood somewhere");
    assert!(first.0.iter().any(|f| f & 2 != 0), "foliage somewhere");
    write_species("ordinary");
    assert_eq!(grow(7, FAMILY_ORDER), 0);
    write_grid(8);
    assert_eq!(query(revision()), 0);
    assert_eq!(answers(), first, "a rebuild answers the same bytes");
    write_species("ordinary");
    assert_eq!(grow(7, 0), 0);
    write_grid(8);
    assert_eq!(query(revision()), 0);
    let coarse = answers();
    assert_eq!(coarse.0, first.0, "the order changes no flag");
    assert_ne!(coarse.3, first.3, "order 0 parts the crown differently");
    write_species("ordinary");
    assert_eq!(grow(8, FAMILY_ORDER), 0);
    write_grid(8);
    assert_eq!(query(revision()), 0);
    assert_ne!(answers().0, first.0, "another seed is another tree");
}
