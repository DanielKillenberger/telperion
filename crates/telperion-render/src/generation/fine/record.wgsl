// A run of the chain grower recorded on the device (fn-209 F1, task 5;
// host decision 6): each chain of the run classified from its region. A
// chain that outgrew its region is queued for the next, larger one by its
// index in the wave; every other one gives its packed counts (axes,
// phytomers), its geometry scratch, and what its wave's laid buffer and
// next wave take from it (kept own axes, their phytomers, the buds they
// bear), which the scans turn into places. Nothing per chain leaves the
// device.

struct Cfg {
    n: u32,
    row: u32,
    first: u32,
    level: u32,
    axes_cap: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> status: array<vec4<u32>>;
@group(0) @binding(2) var<storage, read> axes: array<u32>;
@group(0) @binding(3) var<storage, read> list: array<u32>;
@group(0) @binding(4) var<storage, read_write> counts: array<u32>;
@group(0) @binding(5) var<storage, read_write> queue: array<u32>;
@group(0) @binding(6) var<storage, read_write> queued: array<atomic<u32>>;
@group(0) @binding(7) var<storage, read_write> runoff: array<u32>;

const AXW: u32 = 80u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const NPHYT: u32 = 24u;
const SHED_FLAGS: u32 = 28u;
const LATERAL: u32 = 1u;
// A chain's record in its run: packed axes and phytomers, scratch words
// (scanned into places), kept own axes, their phytomers, the buds they
// bear, whether its root axis was kept, its index in the wave, and its
// region's axes and phytomers.
const RW: u32 = 10u;
const CW: u32 = 3u;
// The geometry kernel's scratch (`geometry.wgsl`).
const SA: u32 = 66u;
const SP: u32 = 12u;

fn ax(c: u32, i: u32, f: u32) -> u32 {
    return axes[(c * cfg.axes_cap + i) * AXW + f];
}

// A region axis the chain holds as its own and keeps.
fn own_kept(c: u32, i: u32) -> bool {
    let own = i == 1u || (i > 1u && (ax(c, i, KIND) & 0xffu) != LATERAL);
    return own && (ax(c, i, SHED_FLAGS) & 1u) != 0u;
}

@compute @workgroup_size(64)
fn record(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n {
        return;
    }
    var wi = cfg.first + c;
    if cfg.level != 0u {
        wi = list[cfg.first + c];
    }
    let st = status[c];
    let r = c * RW;
    for (var f = 0u; f < RW; f++) {
        runoff[r + f] = 0u;
    }
    runoff[r + 7u] = wi;
    if st.z != 0u {
        queue[atomicAdd(&queued[0], 1u)] = wi;
        return;
    }
    runoff[r] = st.x;
    runoff[r + 1u] = st.y;
    runoff[r + 8u] = st.x;
    runoff[r + 9u] = st.y;
    let ok = st.x > 1u && own_kept(c, 1u);
    var own_axes = 0u;
    var own_phyts = 0u;
    var borne = 0u;
    if ok {
        for (var i = 1u; i < st.x; i++) {
            if own_kept(c, i) {
                own_axes += 1u;
                own_phyts += ax(c, i, NPHYT);
            } else if i > 1u && (ax(c, i, KIND) & 0xffu) == LATERAL && own_kept(c, ax(c, i, PARENT)) {
                borne += 1u;
            }
        }
        runoff[r + 2u] = st.x * SA + st.y * SP + 1u;
    }
    runoff[r + 3u] = own_axes;
    runoff[r + 4u] = own_phyts;
    runoff[r + 5u] = borne;
    runoff[r + 6u] = select(0u, 1u, ok);
    counts[wi * CW] = own_axes;
    counts[wi * CW + 1u] = own_phyts;
    counts[wi * CW + 2u] = borne;
}
