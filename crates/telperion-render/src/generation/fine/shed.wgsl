// A chain's keeping on the GPU (fn-209 F1 task 6): under realized
// mechanics the chain grower keeps every axis it grew, and each axis's
// shedding is the fade its whole subtree realizes (`fades.wgsl`). Part of
// `chain.wgsl`'s shader.

const SHED_FLAGS: u32 = 28u;
const KEPT: u32 = 1u;

fn kind_of(i: u32) -> u32 {
    return rd(i, KIND) & 0xffu;
}

fn keep_all() {
    for (var i = 0u; i < n_axes; i++) {
        wr(i, SHED_FLAGS, KEPT);
    }
}
