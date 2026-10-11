// The fine roots' blooms on the device (fn-229), in the stations' pass:
// each fine node's florets counted beside its runs and scanned with them
// (`florets`), then each bloom that keeps one written as the record the
// placement reads (`blooms`). It reads `stations.wgsl`'s bindings and
// `bloom.wgsl`'s florets.

// The phytomer key node `i` was drawn by, which its bloom draws by.
fn key(i: u32) -> vec2<u32> {
    return vec2<u32>(word(i, 27u), word(i, 28u));
}

// How grown node `i`'s bloom is: none off a flowering age.
fn grown_bloom(i: u32) -> f32 {
    let share = nodes[i * NW + 29u];
    if !(share > 0.0) {
        return 0.0;
    }
    return bloom_grown(share, key(i));
}

// Each fine node's florets, then its rachis where it keeps a floret and
// its rachis is drawn (fn-253): one inflorescence's instances in a run,
// and whether it keeps a bloom (fn-229).
@compute @workgroup_size(64)
fn florets(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !fine(i) {
        return;
    }
    var n = floret_count(cfg.flower, grown_bloom(i), key(i));
    if n > 0u && cfg.flower.c.x > 0.0 {
        n += 1u;
    }
    atomicStore(&owners[i * OW], n);
    atomicStore(&owners[i * OW + 1u], u32(n > 0u));
}

// Each bloom that keeps a floret as placement reads it (`place.wgsl`'s
// `floret_words` and `rachis_words`), at its record: its node and its
// bloom's growth, its parent and the node's radius, which its rachis
// tapers from (fn-253), and its first floret, count and key (fn-229).
// `segs` is the records; the florets' and records' totals are in `counts`.
@compute @workgroup_size(64)
fn blooms(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let first = atomicLoad(&owners[i * OW]);
    let record = atomicLoad(&owners[i * OW + 1u]);
    var end = atomicLoad(&counts[4u]);
    var next = atomicLoad(&counts[5u]);
    if i + 1u < cfg.n {
        end = atomicLoad(&owners[(i + 1u) * OW]);
        next = atomicLoad(&owners[(i + 1u) * OW + 1u]);
    }
    if next == record {
        return;
    }
    let at = record * GW;
    put4(at, vec4<f32>(pos(i), grown_bloom(i)));
    put4(at + 4u, vec4<f32>(pos(parent(i)), nodes[i * NW + 3u]));
    segs[at + 24u] = first;
    segs[at + 25u] = end - first;
    segs[at + 26u] = key(i).x;
    segs[at + 27u] = key(i).y;
}
