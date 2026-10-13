// fn-256 task 5 (COUNCIL.md round 4 step 3, the host's rulings of
// 2026-10-12): the arena compacted in place, a column at a time through
// one column of scratch. A table's rows are flagged kept, an exclusive
// scan places them, every column that holds a row's index is remapped
// through the scan, and each column's kept rows are moved into the
// scratch, which the host copies back over the column. The scan keeps
// the rows' order, so a compacted slot order is still the canonical
// order: a row's identity is its lineage key, never its slot.

struct Cfg {
    // The rows a pass covers, the column's stride, the column and the
    // table's first word in `src`.
    n: u32,
    stride: u32,
    col: u32,
    at: u32,
    // The row scan's first word in `rows`, the value scan's in `vals`, and
    // the rows the value scan covers.
    row_at: u32,
    val_at: u32,
    val_n: u32,
    // A remap's kind: an index (none where its row went) or a position.
    mode: u32,
    // A move's first word in `dst`.
    dst_at: u32,
    // A flag's fate column in `vals`, the fate a kept row is below, and
    // what it reads: the row's index, its column's value, or whether that
    // value is an index at all.
    fate_at: u32,
    below: u32,
    by: u32,
    row: u32,
    // A count's flag word, where its table held more rows than the pass
    // covered (the census's `Flag::Compaction`); none at zero.
    flag: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> src: array<u32>;
@group(0) @binding(2) var<storage, read_write> rows: array<u32>;
@group(0) @binding(3) var<storage, read> vals: array<u32>;
@group(0) @binding(4) var<storage, read_write> dst: array<u32>;

const NONE: u32 = 0xffffffffu;
const ID: u32 = 0u;
const BY_INDEX: u32 = 0u;
const BY_VALUE: u32 = 1u;
const RETIRED: u32 = 2u;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn word(r: u32) -> u32 {
    return cfg.at + cfg.col * cfg.stride + r;
}

// Each row's flag: kept where its fate (by its index or by the index its
// column holds) is below `below`, or kept where that column holds an index.
// The scan's last word none, for its total.
@compute @workgroup_size(64)
fn flag_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r > cfg.n {
        return;
    }
    var keep = false;
    if r < cfg.n {
        var v = r;
        if cfg.by != BY_INDEX {
            v = src[word(r)];
        }
        if cfg.by == BY_INDEX || cfg.by == BY_VALUE {
            keep = v < cfg.val_n && vals[cfg.fate_at + v] < cfg.below;
        } else {
            keep = v != NONE;
        }
    }
    rows[cfg.row_at + r] = u32(keep);
}

// Each row's index into a table the value scan compacts: its new index,
// none where its row went; or a position in it (a run's start or end).
@compute @workgroup_size(64)
fn remap_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    let v = src[word(r)];
    if cfg.mode == ID {
        var to = NONE;
        if v < cfg.val_n && vals[cfg.val_at + v + 1u] != vals[cfg.val_at + v] {
            to = vals[cfg.val_at + v];
        }
        src[word(r)] = to;
    } else {
        src[word(r)] = vals[cfg.val_at + min(v, cfg.val_n)];
    }
}

// Each kept row's word at its place in `dst`.
@compute @workgroup_size(64)
fn move_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    let to = rows[cfg.row_at + r];
    if rows[cfg.row_at + r + 1u] != to {
        dst[cfg.dst_at + to] = src[word(r)];
    }
}

// One word holding a count of the rows (the census's, a header's): the
// rows before it that were kept; flagged where it held more than the
// pass covered, which left them where they stood.
@compute @workgroup_size(1)
fn count_main() {
    let count = src[cfg.at];
    if cfg.flag != 0u && count > cfg.n {
        src[cfg.flag] = 1u;
    }
    src[cfg.at] = rows[cfg.row_at + min(count, cfg.n)];
}

// Each retired root's subtree's upkeep frozen into its row (as its bits'
// complement, none where unset) before the rows inside it go: its
// bearer's balance charges it on as it did (`Working::frozen`). `rows`
// holds the sums, `vals` retirement's columns.
@compute @workgroup_size(64)
fn freeze_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    if vals[cfg.fate_at + r] == RETIRED && src[word(r)] == 0u {
        src[word(r)] = ~rows[cfg.row_at + r];
    }
}
