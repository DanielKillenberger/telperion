// An organ's inflorescence as one unit (fn-253): its florets, then its
// rachis, stand in one run of the placements on both executors, so a
// floret finds the rachis it hangs from as the first instance after it
// that carries the rachis's mark. The unit is thinned and enlarged as one,
// about the rachis's seat, so no floret is kept without its rachis. Read
// by selection and by the organ's draw, which bind `placements`.

// The most instances a unit's run holds: `bloom.rs`'s 64 florets, then
// its rachis.
const UNIT_RUN: u32 = 65u;

// The rachis instance `instance` belongs to: itself on a rachis, the first
// marked after it on a floret, or itself where its run holds none.
fn unit_rachis(instance: u32) -> u32 {
    let n = arrayLength(&placements) / LEAF_WORDS;
    let end = min(instance + UNIT_RUN, n);
    for (var j = instance; j < end; j++) {
        if (placements[j * LEAF_WORDS + 2u] & LEAF_WITHERED) != 0u {
            return j;
        }
    }
    return instance;
}

// The words of instance `i`.
fn unit_words(i: u32) -> vec3<u32> {
    let base = i * LEAF_WORDS;
    return vec3<u32>(placements[base], placements[base + 1u], placements[base + 2u]);
}

// Where the unit whose rachis is instance `r` is seated, against the box.
fn unit_seat(r: u32, box_min: vec3<f32>, box_extent: vec3<f32>) -> vec3<f32> {
    return leaf_position(unit_words(r), box_min, box_extent);
}
