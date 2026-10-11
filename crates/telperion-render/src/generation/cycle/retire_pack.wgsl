// fn-256 task 2 (DESIGN.md section 6, compaction at a move): the
// phytomers' arena compacted into the moved build's. Each row flagged
// kept (its axis open or kept) or, where a PA keeps shed pipes or leaf
// girth, recorded (its axis retired or inside a retired subtree); a
// stable scan of each places it; the kept rows' columns move into the
// new arena at their places, the recorded rows' grown columns (key,
// axis, cycle, node presence) into the records after those already held.

struct Cfg {
    // The old arena's stride and the new's, `ret`'s stride, the scans'
    // stride, the records' stride and the records already held, whether
    // rows are recorded, the census's phytomers word, a dispatch row.
    ostride: u32,
    nstride: u32,
    rstride: u32,
    sstride: u32,
    rec_cap: u32,
    rec_base: u32,
    keeps: u32,
    phyts_word: u32,
    row: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> census: array<u32>;
@group(0) @binding(2) var<storage, read> ogrown: array<u32>;
@group(0) @binding(3) var<storage, read> olaid: array<u32>;
@group(0) @binding(4) var<storage, read> ret: array<u32>;
@group(0) @binding(5) var<storage, read_write> ngrown: array<u32>;
@group(0) @binding(6) var<storage, read_write> nlaid: array<u32>;
@group(0) @binding(7) var<storage, read_write> recs: array<u32>;
@group(0) @binding(8) var<storage, read_write> scan: array<u32>;

const GROWN: u32 = 5u;
const LAID: u32 = 14u;
const P_AXIS: u32 = 2u;
const R_FATE: u32 = 1u;
const RETIRED: u32 = 2u;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

// Each row's flags for the two scans: kept, recorded; none past the
// arena's rows, the scans' last rows none.
@compute @workgroup_size(64)
fn flag_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r > cfg.ostride {
        return;
    }
    var kept = 0u;
    var recorded = 0u;
    if r < census[cfg.phyts_word] {
        let retired = ret[R_FATE * cfg.rstride + ogrown[P_AXIS * cfg.ostride + r]] >= RETIRED;
        kept = u32(!retired);
        recorded = u32(retired && cfg.keeps != 0u);
    }
    scan[r] = kept;
    scan[cfg.sstride + r] = recorded;
}

// Each row at its place: kept into the new arena, recorded into the
// records.
@compute @workgroup_size(64)
fn scatter_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= census[cfg.phyts_word] || r >= cfg.ostride {
        return;
    }
    let to = scan[r];
    let rec = scan[cfg.sstride + r];
    let kept = scan[r + 1u] != to;
    let recorded = scan[cfg.sstride + r + 1u] != rec;
    if kept {
        for (var c = 0u; c < GROWN; c++) {
            ngrown[c * cfg.nstride + to] = ogrown[c * cfg.ostride + r];
        }
        for (var c = 0u; c < LAID; c++) {
            nlaid[c * cfg.nstride + to] = olaid[c * cfg.ostride + r];
        }
    } else if recorded {
        for (var c = 0u; c < GROWN; c++) {
            recs[c * cfg.rec_cap + cfg.rec_base + rec] = ogrown[c * cfg.ostride + r];
        }
    }
}
