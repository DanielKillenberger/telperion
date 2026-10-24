// fn-256 task 2 (SETTLE-PLAN.md): the settle's words both its modules
// read: work's columns (`settle_cols.wgsl`'s sections), each PA's
// constants and `atom`'s tallies.

// Work's grown axes' columns: phytomers' and units' starts (scanned),
// depths and ancestors (two each, by turns), the children's starts are
// `atom`'s, the children by index, the living, the woken bud's birth
// presence, the end's presence, vigour, the living apex's presence,
// lived and most present apex carried up, fade, kept, base scale, base
// radius as grown, the share above, the disused pipe's radius, share
// and flag, identity, final index, settled node, and the depth lists.
const W_PSTART: u32 = 0u;
const W_USTART: u32 = 1u;
const W_DEPTH: u32 = 2u;
const W_ANC: u32 = 4u;
const W_KIDS: u32 = 6u;
const W_LIVING: u32 = 7u;
const W_ALIVE: u32 = 8u;
const W_QB1: u32 = 9u;
const W_END: u32 = 10u;
const W_VIG: u32 = 11u;
const W_ALIVEQ: u32 = 12u;
const W_LIVED: u32 = 13u;
const W_SALIVE: u32 = 14u;
const W_FADE: u32 = 15u;
const W_KEPT: u32 = 16u;
const W_BASE: u32 = 17u;
const W_RADIUS: u32 = 18u;
const W_ABOVE: u32 = 19u;
const W_DR: u32 = 20u;
const W_DS: u32 = 21u;
const W_DON: u32 = 22u;
const W_ID: u32 = 23u;
const W_FINAL: u32 = 25u;
const W_NODE: u32 = 26u;
const W_DLIST: u32 = 27u;
const W_COLS: u32 = 28u;
// The grown phytomers' (an axis's in order from its start): the arena's
// row, the scale assigned, its unit's record, and the leaf sums' and
// thickening's scratch.
const WP_ROW: u32 = 0u;
const WP_SCALE: u32 = 1u;
const WP_UNIT: u32 = 2u;
const WP_AT: u32 = 3u;
// The grown units' (an axis's in order from its start): the arena's
// record, its presence and the presences summed to it.
const WU_ROW: u32 = 0u;
const WU_PRES: u32 = 1u;
const WU_PSUM: u32 = 2u;
// The final axes': the grown axis, phytomers' and units' starts
// (scanned), each parent's children by grown index, last first, the
// first child's final index, a depth's scan, and the kept children.
const WF_GROWN: u32 = 0u;
const WF_PSTART: u32 = 1u;
const WF_USTART: u32 = 2u;
const WF_KBI: u32 = 3u;
const WF_KSTART: u32 = 4u;
const WF_SCAN: u32 = 5u;
const WF_KC: u32 = 6u;
// The lists: each living apex's axis and spent ticks, each woken bud's
// axis, bearer, next unit, cycle and share.
const WL_LIVE: u32 = 0u;
const WL_SPENT: u32 = 1u;
const WL_WOKEN: u32 = 2u;

// Each PA's constants from word `K_AT` (`grow_capture_settle_tables`),
// sixteen words a PA.
const K_AT: u32 = 64u;
const K_LIFE: u32 = 0u;
const K_GO: u32 = 1u;
const K_RELAY_ENDED: u32 = 2u;
const K_DELAY: u32 = 3u;
const K_RETAINED: u32 = 4u;
const K_CHI: u32 = 5u;
const K_RELAY_AT: u32 = 6u;
const K_FLAGS: u32 = 7u;
const K_PIPE: u32 = 8u;
const K_RIPENING: u32 = 9u;
const K_EXPONENT: u32 = 10u;
const K_INV_E: u32 = 11u;
const K_SECONDARY: u32 = 12u;
const K_DOMINANCE: u32 = 13u;
const K_SAGLOG: u32 = 14u;
const KF_SHEDS: u32 = 1u;
const KF_RETAINS: u32 = 2u;
const KF_SAGS: u32 = 8u;
const KF_DOMINANCE: u32 = 16u;

// `atom`'s words: the tallies, each depth's axes counted (then their
// starts) and filled, the kept axes per depth, each axis's children
// counted (then their starts), and from `cfg.kfill` filled. The leaf
// term's sums and mean from `T_LEAF`.
const DMAX: u32 = 4096u;
const T_FAULT: u32 = 0u;
const T_DEEPEST: u32 = 1u;
const T_KEPT: u32 = 2u;
const T_KEPT_PHYTS: u32 = 3u;
const T_KEPT_UNITS: u32 = 4u;
// The place's faults (`settle_state.wgsl`'s `FAULT_`).
const T_PLACE: u32 = 5u;
// The lay's first error (its grown index times four and its code) and
// whether any phytomer has size.
const T_ERR: u32 = 6u;
const T_SCALED: u32 = 7u;
const T_LEAF: u32 = 8u;
const T_DCOUNT: u32 = 64u;
const T_DFILL: u32 = T_DCOUNT + DMAX + 64u;
const T_KDCOUNT: u32 = T_DFILL + DMAX;
// Each kept depth's longest axis's phytomers, which bounds the lay's
// dispatches a depth (`settle_place.rs`).
const T_KDLONG: u32 = T_KDCOUNT + DMAX;
const T_KCOUNT: u32 = T_KDLONG + DMAX;
// Faults: an axis's units or phytomers not as counted, a phytomer no
// unit assigned, a depth past `DMAX`, two kept siblings of one key.
const FAULT_LIST: u32 = 1u;
const FAULT_UNASSIGNED: u32 = 2u;
const FAULT_DEEP: u32 = 4u;
const FAULT_TWINS: u32 = 8u;
