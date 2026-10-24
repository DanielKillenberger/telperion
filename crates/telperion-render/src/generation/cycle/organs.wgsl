// Counts of organs between two whole numbers (fn-277,
// `telperion_space::drawn`): a count's word is its floor above and its
// fraction as a share of 2^16 below; a bearer keyed `key` bears the floor,
// and one more where its draw on `row` falls under the share.
const ORGAN_CAULINE: u32 = 0u;
const ORGAN_FRONDS: u32 = 1u;
const ORGAN_SKIRT: u32 = 2u;
const ORGAN_BASES: u32 = 3u;
const ORGAN_LEAFLETS: u32 = 4u;
const ORGAN_SPINES: u32 = 5u;
const ORGAN_CLUSTER: u32 = 6u;

// The 32-bit finaliser (`telperion_space::mix32`).
fn organ_mix(v: u32) -> u32 {
    var z = v;
    z = (z ^ (z >> 16u)) * 0x21f0aaadu;
    z = (z ^ (z >> 15u)) * 0x735a2d97u;
    return z ^ (z >> 15u);
}

// A count's word (`telperion_space::word`): its floor above, its
// fraction as a share of 2^16 below.
fn count_word(count: f32) -> u32 {
    let floor_ = floor(count);
    return (u32(floor_) << 16u) | u32(floor((count - floor_) * 65536.0));
}

fn organs(word: u32, key: u32, row: u32) -> u32 {
    let share = word & 0xffffu;
    let stream = organ_mix(key ^ KEY_ORGANS.x);
    let draw = organ_mix(stream + (row + 1u) * 0x9e3779b9u) >> 16u;
    return (word >> 16u) + select(0u, 1u, share > 0u && draw < share);
}

// The key of placement `ordinal` of the bearer keyed `bearer`
// (`telperion_space::placement_key`).
fn placement_key(bearer: u32, ordinal: u32) -> u32 {
    return organ_mix(bearer + (ordinal + 1u) * 0x9e3779b9u);
}
