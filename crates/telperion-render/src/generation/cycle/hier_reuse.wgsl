// fn-256 task 10 (COUNCIL.md round 5, step 10; DESIGN.md section 16):
// the upward sums take only the axes whose own part changed, and their
// ancestors. Each laid axis's own part is held against what the sum last
// took of it; one that changed, or that it never took, is marked with
// its ancestors up to the first already marked, each with its block, by
// the cycle's number. A sweep enters only the blocks marked and takes
// only the axes marked, each from its own value then every child's in
// order (`totals`, `sub`): a child it does not take keeps the words it
// was last given, which are the words it would be given again. Nothing
// is subtracted or added back. The downward passes take every axis.

// What the sums last took of each axis: its wood's upkeep as an f32 (or
// its frozen upkeep), its own lit presence, and the bits below; then the
// cycle each sum last marked an axis, and a block, to take.
const S_HAD_WOOD: u32 = 19u;
const S_HAD_OWN: u32 = 20u;
const S_HAD: u32 = 21u;
const S_TAKE_TOTALS: u32 = 22u;
const S_TAKE_SUB: u32 = 23u;
const S_BLOCK_TOTALS: u32 = 24u;
const S_BLOCK_SUB: u32 = 25u;

// `S_HAD`'s bits: lit and frozen when the totals took it, whether they
// have, growing when the subtrees' sum took it, whether it has; then
// what changed this cycle, for the count.
const HAD_LIT: u32 = 1u;
const HAD_FROZEN: u32 = 2u;
const HAD_TOTALS: u32 = 4u;
const HAD_GROWING: u32 = 8u;
const HAD_SUB: u32 = 16u;
const NEW_TOTALS: u32 = 256u;
const NEW_LIGHT: u32 = 512u;
const NEW_WOOD: u32 = 1024u;
const NEW_FROZEN: u32 = 2048u;
const NEW_SUB: u32 = 4096u;
const NEW_OWN: u32 = 8192u;
const TOTALS_BITS: u32 = 3847u;
const SUB_BITS: u32 = 12312u;

// The count's words in the tally (`census::Reused`'s first thirteen).
const Y_REUSE: u32 = 96u;
const REUSE_WORDS: u32 = 13u;

// Axis `i` and its ancestors up to the first already marked, each marked
// for the sum whose stamps stand in column `take`, its block in `block`.
fn take_up(i: u32, take: u32, block: u32) {
    var a = i;
    // The tree's depth at most: no walk passes the laid axes.
    for (var steps = 0u; steps < cfg.n && a < cfg.n && sums[at(take, a)] != cfg.cycle; steps++) {
        sums[at(take, a)] = cfg.cycle;
        let b = shape[at(H_BLOCK, a)];
        if b < cfg.stride {
            sums[at(block, b)] = cfg.cycle;
        }
        a = tree[at(T_PARENT, a)];
    }
}

// Whether the sum whose stamps stand in `take` takes axis or block `i`.
fn taken(take: u32, i: u32) -> bool {
    return sums[at(take, i)] == cfg.cycle;
}

// Each laid axis's own part of the totals (`totals`) against what they
// last took: its frozen upkeep, or its wood's and, lit now or then, its
// light. The PA's upkeep is the same every cycle (`Tables::rows_at`).
@compute @workgroup_size(64)
fn totals_mark_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if cfg.cycle == 0u || i >= cfg.n {
        return;
    }
    let had = sums[at(S_HAD, i)];
    let frozen = tree[at(T_FROZEN, i)];
    var wood = frozen;
    var flags = HAD_TOTALS | HAD_FROZEN;
    if frozen == NONE {
        wood = bitcast<u32>(single(wide(T_WOOD, i)));
        flags = HAD_TOTALS | select(0u, HAD_LIT, sums[at(S_LIT, i)] != 0u);
    }
    var changed = 0u;
    if (had & HAD_TOTALS) == 0u {
        changed = NEW_TOTALS;
    } else {
        if ((had ^ flags) & HAD_FROZEN) != 0u {
            changed |= NEW_FROZEN;
        }
        if frozen == NONE && ((had | flags) & HAD_LIT) != 0u {
            changed |= NEW_LIGHT;
        }
        if sums[at(S_HAD_WOOD, i)] != wood {
            changed |= NEW_WOOD;
        }
    }
    sums[at(S_HAD_WOOD, i)] = wood;
    sums[at(S_HAD, i)] = (had & SUB_BITS) | flags | changed;
    if changed != 0u {
        take_up(i, S_TAKE_TOTALS, S_BLOCK_TOTALS);
    }
}

// Each laid axis's own part of its subtree's lit presence (`sub`)
// against what that sum last took: its own lit presence, and whether it
// grows.
@compute @workgroup_size(64)
fn sub_mark_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if cfg.cycle == 0u || i >= cfg.n {
        return;
    }
    let had = sums[at(S_HAD, i)];
    let own = sums[at(S_OWN, i)];
    let growing = select(0u, HAD_GROWING, sums[at(S_GROWING, i)] != 0u);
    var changed = 0u;
    if (had & HAD_SUB) == 0u {
        changed = NEW_SUB;
    } else if sums[at(S_HAD_OWN, i)] != own || (had & HAD_GROWING) != growing {
        changed = NEW_OWN;
    }
    sums[at(S_HAD_OWN, i)] = own;
    sums[at(S_HAD, i)] = (had & TOTALS_BITS) | HAD_SUB | growing | changed;
    if changed != 0u {
        take_up(i, S_TAKE_SUB, S_BLOCK_SUB);
    }
}

var<workgroup> w_reused: array<atomic<u32>, REUSE_WORDS>;

// A measurement's count (`Tables::counted`): the laid axes by what
// changed in their own part of each upward sum, those each took, the
// blocks each entered (a block's number is under the laid axes), the lit
// axes, the growing buds and those whose own lit presence held.
@compute @workgroup_size(64)
fn reused_main(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    if lid < REUSE_WORDS {
        atomicStore(&w_reused[lid], 0u);
    }
    workgroupBarrier();
    let i = index(id);
    if cfg.cycle != 0u && i < cfg.n {
        let had = sums[at(S_HAD, i)];
        var bits = array<u32, 4>(NEW_TOTALS, NEW_LIGHT, NEW_WOOD, NEW_FROZEN);
        for (var k = 0u; k < 4u; k++) {
            if (had & bits[k]) != 0u {
                atomicAdd(&w_reused[k], 1u);
            }
        }
        var held = array<bool, 9>(
            taken(S_TAKE_TOTALS, i),
            taken(S_BLOCK_TOTALS, i),
            (had & NEW_SUB) != 0u,
            (had & NEW_OWN) != 0u,
            taken(S_TAKE_SUB, i),
            taken(S_BLOCK_SUB, i),
            (had & HAD_LIT) != 0u,
            (had & HAD_GROWING) != 0u,
            (had & (HAD_GROWING | NEW_SUB | NEW_OWN)) == HAD_GROWING,
        );
        for (var k = 0u; k < 9u; k++) {
            if held[k] {
                atomicAdd(&w_reused[4u + k], 1u);
            }
        }
    }
    workgroupBarrier();
    if lid < REUSE_WORDS {
        atomicAdd(&tally[Y_REUSE + lid], atomicLoad(&w_reused[lid]));
    }
}
