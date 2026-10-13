// Fate and births: each surviving apex's unit survival (D5, `advance`):
// its viability raised by shade, `viability^(light^-φ)`, the threshold
// over its share of the cycle, and its draw against it; then the axes the
// apexes bear placed in their order, an exclusive scan of their counts.
//
// P.a[0]: apexes, axes before, scan block count.

@group(0) @binding(1) var<storage, read_write> keys: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read_write> shares: array<vec2<u32>>;
@group(0) @binding(3) var<storage, read_write> light: array<f32>;
@group(0) @binding(4) var<storage, read_write> apa: array<u32>;
@group(0) @binding(5) var<storage, read_write> tab: array<f32>;
@group(0) @binding(6) var<storage, read_write> thresholds: array<vec2<u32>>;
@group(0) @binding(7) var<storage, read_write> fate_draws: array<vec2<u32>>;
@group(0) @binding(8) var<storage, read_write> survives: array<u32>;
@group(0) @binding(9) var<storage, read_write> births: array<u32>;
@group(0) @binding(10) var<storage, read_write> first: array<u32>;
@group(0) @binding(11) var<storage, read_write> block_sums: array<u32>;

@compute @workgroup_size(256)
fn fate(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    let k = 8u * apa[i];
    let viability = tab[k + 6u];
    let phi = tab[k + 7u];
    let l = light[i];
    var p = viability;
    if phi > 0.0 && l < 1.0 {
        p = exp2c(log2c(viability) * exp2c(-phi * log2c(l)));
    }
    let share = shares[i];
    var t: vec2<u32>;
    if share.x == 0xffffffffu && share.y == 0xffffffffu {
        t = threshold(p);
    } else {
        t = survival_fine(neg_log2(p), share);
    }
    let d = draw(child(keys[i], KEY_VIABILITY));
    thresholds[i] = t;
    fate_draws[i] = d;
    survives[i] = u32(lt64(d, t));
}

var<workgroup> part: array<u32, 256>;

// Each block of 256 counts scanned in place, exclusive, its total kept.
@compute @workgroup_size(256)
fn scan_blocks(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let i = wid.x * WG + lid;
    var v = 0u;
    if i < P.a[0].x {
        v = births[i];
    }
    part[lid] = v;
    workgroupBarrier();
    for (var s = 1u; s < WG; s <<= 1u) {
        var add = 0u;
        if lid >= s {
            add = part[lid - s];
        }
        workgroupBarrier();
        part[lid] += add;
        workgroupBarrier();
    }
    if i < P.a[0].x {
        first[i] = part[lid] - v;
    }
    if lid == WG - 1u {
        block_sums[wid.x] = part[lid];
    }
}

// The blocks' totals scanned, exclusive, by one workgroup.
@compute @workgroup_size(256)
fn scan_sums(@builtin(local_invocation_index) lid: u32) {
    var carry = 0u;
    let blocks = P.a[0].z;
    for (var base = 0u; base < blocks; base += WG) {
        let i = base + lid;
        var v = 0u;
        if i < blocks {
            v = block_sums[i];
        }
        part[lid] = v;
        workgroupBarrier();
        for (var s = 1u; s < WG; s <<= 1u) {
            var add = 0u;
            if lid >= s {
                add = part[lid - s];
            }
            workgroupBarrier();
            part[lid] += add;
            workgroupBarrier();
        }
        if i < blocks {
            block_sums[i] = carry + part[lid] - v;
        }
        carry += part[WG - 1u];
        workgroupBarrier();
    }
}

@compute @workgroup_size(256)
fn scan_add(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let i = wid.x * WG + lid;
    if i >= P.a[0].x {
        return;
    }
    first[i] = P.a[0].y + block_sums[wid.x] + first[i];
}
