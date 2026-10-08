// Every lineage key, mirroring `telperion_space::lineage_keys` (its
// `ALL`), which is the source: a key is added there and copied here, and
// `fine/mirror_tests.rs` checks the copy. Each is the u64 as (low, high).

const KEY_VIABILITY = vec2<u32>(0x00000000u, 0x00000000u);
const KEY_ABORTION = vec2<u32>(0x00000001u, 0x00000000u);
const KEY_MOVE = vec2<u32>(0x00000002u, 0x00000000u);
const KEY_ZONE = vec2<u32>(0x00000010u, 0x00000000u);
const KEY_PLACE = vec2<u32>(0x00000000u, 0x00000000u);
const KEY_SHED = vec2<u32>(0xfffffffeu, 0xffffffffu);
const KEY_RELAY = vec2<u32>(0xfffffffdu, 0xffffffffu);
const KEY_ROLL = vec2<u32>(0xfffffffbu, 0xffffffffu);
const KEY_DOMINANCE = vec2<u32>(0xfffffffau, 0xffffffffu);
const KEY_DORMANT = vec2<u32>(0xfffffff9u, 0xffffffffu);
const KEY_ENDED = vec2<u32>(0xfffffff8u, 0xffffffffu);
const KEY_END = vec2<u32>(0xfffffff7u, 0xffffffffu);
const KEY_INTERNODE = vec2<u32>(0xfffffff6u, 0xffffffffu);
const KEY_FAILED = vec2<u32>(0xfffffff5u, 0xffffffffu);
const KEY_INSERTION = vec2<u32>(0xfffffff4u, 0xffffffffu);
const KEY_TROPISM = vec2<u32>(0xfffffff3u, 0xffffffffu);
const KEY_WANDER = vec2<u32>(0xfffffff2u, 0xffffffffu);
const KEY_BLOOM = vec2<u32>(0x0b100000u, 0x00000000u);
const KEY_BLOOM_ROLL = vec2<u32>(0x0b100001u, 0x00000000u);
const KEY_FLORET = vec2<u32>(0x0b100002u, 0x00000000u);
const KEY_FLORET_SIZE = vec2<u32>(0x00000000u, 0x00000000u);
const KEY_FLORET_OPENING = vec2<u32>(0x00000001u, 0x00000000u);
const KEY_CLUSTER = vec2<u32>(0x0c100000u, 0x00000000u);
const KEY_CONTINUATION = vec2<u32>(0xffffffffu, 0xffffffffu);
