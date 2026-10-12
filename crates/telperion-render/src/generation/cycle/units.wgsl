// The cycle's units and wake on the device (DESIGN.md sections 2.1, 2.2
// and 10): a job a living apex in the live order, or a sleeping bud of
// the cycle in its order (`wake.wgsl`), run in rounds of small phases,
// each its own entry: the unit decided, its zones laid, their nodes
// counted or written, then the operations after it up to the next unit.
// Counted, a job's appends are scanned in job order into places;
// emitted, the same phases write each record at its place, so every
// index is the CPU's and none comes from an atomic.

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
    let t = select(vec4<u32>(0u), vec4<u32>(ls(T_UNITS), ls(T_UNITS + 1u), ls(T_UNITS + 2u), ls(T_UNITS + 3u)), after);
    let next = select(0u, ls(T_UNITS_B), after);
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

// The job of invocation `id`, none past the range's: its place in it.
fn job_of(id: vec3<u32>) -> u32 {
    let k = id.x + id.y * cfg.row;
    if cfg.cycle == 0u || cfg.base + k >= cfg.n {
        return NONE;
    }
    return k;
}

// Each job begun: a living apex loaded, or a sleeping bud aged, the axes
// it wakes on found and woken on the first.
@compute @workgroup_size(64)
fn init_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = job_of(id);
    if k == NONE {
        return;
    }
    open_job(k);
    take_job(k);
    let i = cfg.base + k;
    var o = op(OP_DONE, Apex(0u, 0u));
    if cfg.wake == 0u {
        o = op(OP_ADVANCE, Apex(inputs[2u * i], inputs[2u * i + 1u]));
        load(o.apex.axis);
    } else if i < ls(T_LIST) {
        let row = ls(cfg.list_at + i);
        let g = aged(sleeper(row, S_PA), (sleeper(row, S_SLEPT) << 20u) + sleeper(row, S_SLEEP));
        set_jw(J_ROW, row);
        set_jw(J_G, g.pa);
        set_jw(J_G + 1u, g.spent);
        set_jw(J_G + 2u, g.steps);
        if g.pa != NONE {
            carried(row, cfg.cycle);
            o.kind = select(OP_DONE, OP_WOKE, jw(J_CARRIED_N) > 0u && fault == 0u);
        }
    }
    keep_op(o);
    keep_job();
    woken(k);
}

// A sleeping bud woken on its next axis (`Shoot::woke`), as it begins a
// round: its first unit grows in it.
fn woken(k: u32) {
    if jw(J_OP) != OP_WOKE {
        return;
    }
    take_job(k);
    let j = jw(J_WOKE);
    let g = Aged(jw(J_G), jw(J_G + 1u), jw(J_G + 2u));
    let apex = woke(jw(J_ROW), g, jw64(J_CARRIED + 2u * j), cfg.cycle);
    set_jw(J_WOKE, j + 1u);
    keep_step(op(OP_ADVANCE, apex));
}

// Each waking bud a round after its first woken on its next axis: the
// wake's rounds' first phase (`init_main` wakes the first).
@compute @workgroup_size(64)
fn woke_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = job_of(id);
    if k == NONE {
        return;
    }
    job = k;
    woken(k);
}

// The unit of each job that grows one decided (`decide`), then what
// follows its decision in the CPU's order: its stop where it fails with
// no relay, the unit appended where it lives, or, where it dies and a
// relay takes over, appended at the share it lived and its abortion
// drawn. Each step reads only its own job, so one invocation runs them in
// order as entries of their own did (fn-256 task 6).
@compute @workgroup_size(64)
fn decide_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = job_of(id);
    if k == NONE {
        return;
    }
    job = k;
    if jw(J_OP) == OP_ADVANCE {
        take_job(k);
        var o = op_of();
        o.kind = decide(o.apex, cfg.cycle);
        keep_step(o);
    }
    if job_at_op(id, OP_FAILS) {
        keep_step(fails_now(op_of().apex, cfg.cycle));
    }
    if job_at_op(id, OP_UNIT) {
        keep_step(unit_lives(op_of().apex, cfg.cycle));
    }
    if job_at_op(id, OP_DIES) {
        keep_step(unit_dies(op_of().apex, cfg.cycle));
    }
    if job_at_op(id, OP_LIVED) {
        keep_step(unit_lived(op_of().apex, cfg.cycle));
    }
}

// The zone of a job's unit an invocation takes, its zone's place in the
// unit table, none where the job grows none or its unit has fewer.
fn zone_of(id: vec3<u32>) -> vec2<u32> {
    let at = id.x + id.y * cfg.row;
    let k = at / cfg.zones;
    if cfg.cycle == 0u || cfg.base + k >= cfg.n {
        return vec2<u32>(NONE);
    }
    job = k;
    if jw(J_OP) != OP_GROWN {
        return vec2<u32>(NONE);
    }
    let z = at % cfg.zones;
    let pa = fr(jw(J_FRAME), A_PA);
    if z >= table[2u * pa + 1u] {
        return vec2<u32>(NONE);
    }
    return vec2<u32>(z, table[2u * pa] + ZW * z);
}

// Each zone's nodes laid, then their laterals counted and their sleeping
// buds: what a zone reads of its own alone.
@compute @workgroup_size(64)
fn zones_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let z = zone_of(id);
    if z.x == NONE {
        return;
    }
    zone_laid(z.x, z.y, jw64(J_UKEY), cfg.cycle, jw(J_GROW), jw(J_CUT));
    for (var k = 0u; k < 2u; k++) {
        take_job(job);
        pending = false;
        zone_nodes(z.x, z.y, jw(J_FRAME), cfg.cycle, 7u * k);
    }
}

// Emitted, each zone's records written from the places the zones before
// it left, in three phases a zone's own writes order: its nodes ordered
// along the unit, then its phytomers and sleeping buds (`written_main`);
// its laterals (`sprouted_main`); its laterals' presences (their PA's
// draw's, then times their place's) and its sleeping buds'
// (`placed_main`). The laterals are a phase of their own: fused with the
// phytomers, each dispatch took three times as long (fn-256 task 11).
@compute @workgroup_size(64)
fn written_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let z = zone_of(id);
    if z.x == NONE {
        return;
    }
    order_zone(z.x, z.y);
    let whats = array<u32, 2>(3u, 2u);
    for (var k = 0u; k < 2u; k++) {
        take_job(job);
        pending = false;
        zone_nodes(z.x, z.y, jw(J_FRAME), cfg.cycle, whats[k]);
    }
}

@compute @workgroup_size(64)
fn sprouted_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let z = zone_of(id);
    if z.x == NONE {
        return;
    }
    take_job(job);
    pending = false;
    zone_nodes(z.x, z.y, jw(J_FRAME), cfg.cycle, 1u);
}

@compute @workgroup_size(64)
fn placed_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let z = zone_of(id);
    if z.x == NONE {
        return;
    }
    let whats = array<u32, 2>(4u, 5u);
    for (var k = 0u; k < 2u; k++) {
        take_job(job);
        pending = false;
        zone_nodes(z.x, z.y, jw(J_FRAME), cfg.cycle, whats[k]);
    }
    take_job(job);
    sleepers_placed(z.x, z.y, jw(J_FRAME), cfg.cycle);
}

// A job whose operation is `kind`, taken up: its operation, none for
// another job or past the range.
fn job_at_op(id: vec3<u32>, kind: u32) -> bool {
    let k = job_of(id);
    if k == NONE {
        return false;
    }
    job = k;
    if jw(J_OP) != kind {
        return false;
    }
    take_job(k);
    if fault != 0u && kind != OP_NONE {
        keep_step(op(OP_NONE, Apex(jw(J_AXIS), jw(J_SPENT))));
        return false;
    }
    return true;
}

fn keep_step(o: Op) {
    keep_op(o);
    keep_job();
}

// What follows each grown unit, a job each (DESIGN.md section 10's
// `after`), in a round's order: its zones' records taken in, then its
// failure's relay, its abortion or its move (`Shoot::move_on`), the
// continuation a move made sprouted and carried on, a stop
// (`Shoot::stop`), a relay made (`Shoot::relay_bud`), and a job with
// nothing left to do now: the relay a move left once its continuation's
// growth is done, or its growth's end, after which a waking bud wakes on
// its next axis.
@compute @workgroup_size(64)
fn after_main(@builtin(global_invocation_id) id: vec3<u32>) {
    if job_at_op(id, OP_GROWN) {
        let s = jw(J_FRAME);
        for (var z = 0u; z < table[2u * fr(s, A_PA) + 1u]; z++) {
            n_phyts += zw(z, Z_N);
            n_axes += zw(z, Z_LAT);
            n_next += zw(z, Z_LAT);
            n_sleep += zw(z, Z_SLEEP);
            fault |= zw(z, Z_FAULT);
            set_fr(s, A_PHYTS, fr(s, A_PHYTS) + zw(z, Z_N));
        }
        keep_step(grown_on(op_of(), cfg.cycle));
    }
    if job_at_op(id, OP_MOVE) {
        keep_step(move_on(op_of(), cfg.cycle));
    }
    if job_at_op(id, OP_MOVED) {
        keep_step(moved_on(op_of(), cfg.cycle));
    }
    if job_at_op(id, OP_STOP) {
        keep_step(stop(op_of(), cfg.cycle));
    }
    if job_at_op(id, OP_RELAY) {
        keep_step(relay(op_of(), cfg.cycle));
    }
    if job_at_op(id, OP_NONE) {
        ended();
    }
}

fn ended() {
    if fault == 0u && jw(J_DEPTH) > 0u {
        keep_step(relay_after(pop_after(), cfg.cycle));
        return;
    }
    // What one job made is never read by the next.
    store_all();
    n_frames = 0u;
    var o = op_of();
    let more = cfg.wake != 0u && jw(J_WOKE) < jw(J_CARRIED_N) && fault == 0u;
    o.kind = select(OP_DONE, OP_WOKE, more);
    keep_step(o);
}

// Each job closed: its last living apex written, its counts kept where
// counted, and a job with operations left past the cycle's last round
// marked.
@compute @workgroup_size(64)
fn close_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = job_of(id);
    if k == NONE {
        return;
    }
    take_job(k);
    if jw(J_OP) != OP_DONE {
        put_ls(T_ROUNDS, 1u);
    }
    flush_next();
    if !emitting {
        put_counts(cfg.base + k);
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
        put_ls(T_COLLAPSED, 1u);
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
