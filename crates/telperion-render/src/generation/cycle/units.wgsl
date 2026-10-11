// The cycle's units and wake on the device (DESIGN.md sections 2.1 and
// 2.2), one entry for both and both modes, so the driver compiles `run`
// once: one invocation per living apex in the live order, running
// `grow_live`'s advance of it, or per sleeping bud of the cycle in its
// order (`wake.wgsl`). Counted, the appends are scanned in that order
// into places; emitted, the same body writes each record at its place,
// so every index is the CPU's and none comes from an atomic.

fn index(id: vec3<u32>) -> u32 {
    return cfg.base + id.x + id.y * cfg.row;
}

// The apex's counts, its fault in the second record.
fn put_counts(i: u32) {
    let a = 4u * i;
    counts[a] = n_axes;
    counts[a + 1u] = n_phyts;
    counts[a + 2u] = n_units;
    counts[a + 3u] = n_sleep;
    counts[cfg.b_at + a] = n_next;
    counts[cfg.b_at + a + 1u] = n_woken;
    counts[cfg.b_at + a + 2u] = fault;
    counts[cfg.b_at + a + 3u] = 0u;
}

// Where record `i`'s appends go: past the tree's records before the
// cycle (and, in the wake, past the units'), at its scanned places; a
// count lays them past the tree's, where no index it reads stands.
fn bases(i: u32, emit: bool, after: bool) {
    let a = 4u * i;
    let t = select(vec4<u32>(0u), vec4<u32>(lists[T_UNITS], lists[T_UNITS + 1u], lists[T_UNITS + 2u], lists[T_UNITS + 3u]), after);
    let next = select(0u, lists[T_UNITS_B], after);
    base_axes = cfg.axes_before + t.x;
    base_phyts = cfg.phyts_before + t.y;
    base_units = cfg.units_before + t.z;
    base_sleep = cfg.sleep_before + t.w;
    base_next = next;
    base_woken = cfg.woken_before;
    if emit {
        base_axes += counts[a];
        base_phyts += counts[a + 1u];
        base_units += counts[a + 2u];
        base_sleep += counts[a + 3u];
        base_next += counts[cfg.b_at + a];
        base_woken += counts[cfg.b_at + a + 1u];
    }
}

@compute @workgroup_size(64)
fn cycle_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let emit = cfg.emit != 0u;
    let waking = cfg.wake != 0u;
    begin(emit);
    bases(i, emit, waking);
    var jobs = 1u;
    var row = 0u;
    var g = Aged(NONE, 0u, 0u);
    if waking {
        jobs = 0u;
        if i < lists[T_LIST] {
            row = lists[cfg.list_at + i];
            g = aged(sleeper(row, S_PA), (sleeper(row, S_SLEPT) << 20u) + sleeper(row, S_SLEEP));
        }
        if g.pa != NONE {
            carried(row, cfg.cycle);
            jobs = carried_n;
        }
    }
    for (var j = 0u; j < jobs && fault == 0u; j++) {
        var apex: Apex;
        if waking {
            apex = woke(row, g, carried_at[j], cfg.cycle);
        } else {
            apex = Apex(inputs[2u * i], inputs[2u * i + 1u]);
            load(apex.axis);
        }
        run(apex, cfg.cycle);
        // What one job made is never read by the next.
        store_all();
        n_frames = 0u;
    }
    flush_next();
    if !emit {
        put_counts(i);
    }
}

// The grower before its first cycle (`Grower::seeded`): the seed's bud of
// the first age it does not pass through, its draws' birth that age's
// reach, as the living apex the cycle before the first leaves.
@compute @workgroup_size(1)
fn seed_main() {
    let root = mix(add64(vec2<u32>(cfg.seed_lo, cfg.seed_hi), vec2<u32>(0x7f4a7c15u, 0x9e3779b9u)));
    let pa = lived_pa(0u);
    if pa == NONE {
        lists[T_COLLAPSED] = 1u;
        return;
    }
    let lineage = onto(root, pa);
    let reach0 = real64(reach(0u, 0u));
    var words = array<vec2<u32>, 7>(lineage, root, vec2<u32>(0u, 0xfff00000u), vec2<u32>(0u), ONE64, reach0, ONE64);
    var at = array<u32, 7>(A_LINEAGE, A_ROOT, A_HORIZON, A_WHORL, A_BLEND, A_B0, A_B1);
    for (var f = 1u; f < AXES_COLUMNS; f++) {
        set_ax(0u, f, 0u);
    }
    for (var k = 0u; k < 7u; k++) {
        set_ax(0u, at[k], words[k].x);
        set_ax(0u, at[k] + 1u, words[k].y);
    }
    set_ax(0u, A_PA, pa);
    var none = array<u32, 5>(A_PARENT, A_SUCC0, A_SUCC1, A_TAIL, A_HEAD);
    for (var k = 0u; k < 5u; k++) {
        set_ax(0u, none[k], NONE);
    }
    set_next(0u, 0u, 0u);
}
