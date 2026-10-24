// The counts of organs the stations and the boots stage's fronds read
// (`stations.rs`'s `organ_rows`, fn-277), in their stage's `cfg.organs`.

// Word `i` of the counts of organs.
fn organ_word(i: u32) -> u32 {
    return cfg.organs[i / 4u][i % 4u];
}

// Whether any placement may carry more than one leaflet.
fn grouped() -> bool {
    return organ_word(3u) > 0x10000u && bitcast<f32>(organ_word(5u)) > 0.0;
}

// The leaves placement `k` of the bearer keyed `key` carries: its
// leaflets, one where it draws one or there is no rachis to string them
// on (`organs::leaflets`).
fn leaves_at(key: u32, k: u32) -> u32 {
    if !grouped() {
        return 1u;
    }
    let n = organs(organ_word(3u), placement_key(key, k), ORGAN_LEAFLETS);
    return select(1u, n, n > 1u);
}
