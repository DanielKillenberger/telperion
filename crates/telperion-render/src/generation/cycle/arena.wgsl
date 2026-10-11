// fn-256 task 2: the one cycle's state on the device (DESIGN.md section
// 1), the columns `units.wgsl` and `wake.wgsl` read and append to. Every
// column is one word an axis (or a unit, phytomer, sleeping bud, living
// apex, woken bud), its rows `cap` apart in its buffer; an index on the
// device is the CPU's. Times are ticks of 2^-20 cycle, reals their bits.

struct Cfg {
    n: u32,
    row: u32,
    cycle: u32,
    age: u32,
    // Whether the kernel writes (emit) or only counts, and which phase
    // it runs in: the living apexes' units (0) or the wake (1).
    emit: u32,
    wake: u32,
    lights: u32,
    zero: u32,
    axes_cap: u32,
    units_cap: u32,
    phyts_cap: u32,
    sleep_cap: u32,
    next_cap: u32,
    woken_cap: u32,
    list_cap: u32,
    pencils: u32,
    phyt_at: u32,
    sleep_at: u32,
    next_at: u32,
    woken_at: u32,
    list_at: u32,
    axes_before: u32,
    units_before: u32,
    phyts_before: u32,
    sleep_before: u32,
    pen_at: u32,
    rows: u32,
    whorls: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
    pad3: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> table: array<u32>;
@group(0) @binding(2) var<storage, read> words: array<vec2<u32>>;
@group(0) @binding(3) var<storage, read> inputs: array<u32>;
@group(0) @binding(4) var<storage, read_write> axes: array<u32>;
@group(0) @binding(5) var<storage, read_write> arena: array<u32>;
@group(0) @binding(6) var<storage, read_write> lists: array<u32>;
@group(0) @binding(7) var<storage, read_write> counts_a: array<u32>;
@group(0) @binding(8) var<storage, read_write> counts_b: array<u32>;

const TICKS: u32 = 0x100000u;

// An axis's columns, the same words a frame holds (`frame.wgsl`); word 0
// of a frame is its index.
const A_PA: u32 = 1u;
const A_BIRTH: u32 = 2u;
// Kind (0 seed, 1 lateral, 2 continuation, 3 relay), slot << 8, woken << 16.
const A_ORIGIN: u32 = 3u;
const A_PARENT: u32 = 4u;
const A_NODE: u32 = 5u;
const A_LINEAGE: u32 = 6u;
const A_ROOT: u32 = 8u;
const A_HORIZON: u32 = 10u;
const A_WHORL: u32 = 12u;
const A_BLEND: u32 = 14u;
// The cycle its apex ended in, plus one; none: 0.
const A_END: u32 = 16u;
const A_PHYTS: u32 = 17u;
// Its time spent in its PA (`Grower::units`).
const A_SPENT: u32 = 18u;
const A_B0: u32 = 19u;
const A_B1: u32 = 21u;
const A_FLAGS: u32 = 23u;
const A_BELOW: u32 = 24u;
const A_OUTLIVED: u32 = 25u;
const A_ENDED: u32 = 26u;
const A_AGED: u32 = 27u;
const A_SLEEP: u32 = 28u;
const A_UNITS: u32 = 29u;
const A_GROWN: u32 = 30u;
const A_SUCC0: u32 = 31u;
const A_SUCC1: u32 = 32u;
// Its last unit's record, none: NONE.
const A_TAIL: u32 = 33u;
const AW: u32 = 34u;

// The flags word (`presence::Draws`).
const F_FAILED: u32 = 1u;
const F_ENDED: u32 = 2u;
const F_REST: u32 = 4u;
const F_CARRIED: u32 = 8u;
const F_WOKE: u32 = 16u;
const F_OUTLIVED: u32 = 32u;

// A unit's columns: its axis, survival and persistence (f32), share and
// size (ticks), its shade's size and light (f64), the units before and
// after it on its axis, and its phytomers.
const U_AXIS: u32 = 0u;
const U_SURVIVE: u32 = 1u;
const U_PERSIST: u32 = 2u;
const U_SHARE: u32 = 3u;
const U_SIZE: u32 = 4u;
const U_SHADE: u32 = 5u;
const U_PREV: u32 = 9u;
const U_NEXT: u32 = 10u;
const U_FIRST: u32 = 11u;
const U_COUNT: u32 = 12u;

// A phytomer's columns: its key, axis, cycle and node presence.
const P_KEY: u32 = 0u;
const P_AXIS: u32 = 2u;
const P_CYCLE: u32 = 3u;
const P_NODE: u32 = 4u;

// A sleeping bud's columns (`dormant::Sleeper`): the cycle it wakes in,
// then its fields, and its unit's record.
const S_WAKE: u32 = 0u;
const S_PARENT: u32 = 1u;
const S_NODE: u32 = 2u;
const S_UNIT: u32 = 3u;
const S_SLOT: u32 = 4u;
const S_PA: u32 = 5u;
const S_OWN: u32 = 6u;
const S_SLEEP: u32 = 7u;
const S_SLEPT: u32 = 8u;
const S_WHORL: u32 = 9u;
const S_KEY: u32 = 11u;
const S_REC: u32 = 13u;

// A woken bud's columns (`dormant::Woken`).
const W_AXIS: u32 = 0u;
const W_BEARER: u32 = 1u;
const W_NEXT: u32 = 2u;
const W_CYCLE: u32 = 3u;
const W_SHARE: u32 = 4u;

// The tally's words at the head of `lists`: the units' totals (axes,
// phytomers, units, sleeping buds; living apexes, woken), the cycle's
// sleeping buds, then the wake's.
const T_UNITS: u32 = 0u;
const T_UNITS_B: u32 = 4u;
const T_LIST: u32 = 8u;
const T_WAKE: u32 = 12u;
const T_WAKE_B: u32 = 16u;

fn ax(i: u32, f: u32) -> u32 {
    return axes[f * cfg.axes_cap + i];
}

fn set_ax(i: u32, f: u32, v: u32) {
    axes[f * cfg.axes_cap + i] = v;
}

fn unit(r: u32, f: u32) -> u32 {
    return arena[f * cfg.units_cap + r];
}

fn set_unit(r: u32, f: u32, v: u32) {
    arena[f * cfg.units_cap + r] = v;
}

fn set_phyt(p: u32, f: u32, v: u32) {
    arena[cfg.phyt_at + f * cfg.phyts_cap + p] = v;
}

fn sleeper(s: u32, f: u32) -> u32 {
    return lists[cfg.sleep_at + f * cfg.sleep_cap + s];
}

fn set_sleeper(s: u32, f: u32, v: u32) {
    lists[cfg.sleep_at + f * cfg.sleep_cap + s] = v;
}

fn set_next(k: u32, axis: u32, spent: u32) {
    lists[cfg.next_at + k] = axis;
    lists[cfg.next_at + cfg.next_cap + k] = spent;
}

fn set_woken(k: u32, f: u32, v: u32) {
    lists[cfg.woken_at + f * cfg.woken_cap + k] = v;
}

// Axis `i`'s light as the f32 survival reads (`light as f32`), its
// subtrees' survival as a presence (`q(kept)`), and its size and light as
// a unit's shade holds them; whole for an axis laid by no pencil.
fn light32(i: u32) -> f32 {
    return select(1.0, bitcast<f32>(inputs[cfg.pen_at + i]), i < cfg.pencils);
}

fn kept32(i: u32) -> f32 {
    return select(1.0, bitcast<f32>(inputs[cfg.pen_at + cfg.pencils + i]), i < cfg.pencils);
}

const ONE64 = vec2<u32>(0u, 0x3ff00000u);

fn pen64(i: u32, k: u32) -> vec2<u32> {
    if i >= cfg.pencils {
        return ONE64;
    }
    let at = cfg.pen_at + (2u + 2u * k) * cfg.pencils + i;
    return vec2<u32>(inputs[at], inputs[at + cfg.pencils]);
}

// A product's rounding kept apart from the sum it feeds: the driver
// fuses a multiply into an add by the code's shape, which the CPU never
// does (SPIKE.md), and cannot see through a word it does not know.
fn kept_apart(x: f32) -> f32 {
    return bitcast<f32>(bitcast<u32>(x) ^ cfg.zero);
}

// PA `pa`'s row (`ChainTables::cycle`'s `PaRow` with the capture's
// tables): word `k`.
fn prow(pa: u32, k: u32) -> vec2<u32> {
    return words[table[cfg.rows + pa] + k];
}

fn prow_f(pa: u32, k: u32) -> f32 {
    return bitcast<f32>(prow(pa, k).x);
}

const R_LIFE: u32 = 0u;
const R_VIABILITY: u32 = 1u;
const R_SHADE: u32 = 2u;
// The relay thresholds of a stop by abortion, end and failure.
const R_RELAY: u32 = 3u;
const R_ABORT: u32 = 6u;
const R_ABORT_L: u32 = 7u;
const R_RISE: u32 = 8u;
// Flags: aborts 1, rises 2, sheds 4; then the PA a bud grows in and the
// one it moves on to.
const R_FLAGS: u32 = 9u;
const R_LIVED: u32 = 10u;
const R_GO: u32 = 11u;
const R_GO_LOGIT: u32 = 12u;
const R_MOVED: u32 = 13u;
const R_DELAY: u32 = 14u;
const R_STAKE1: u32 = 15u;
const R_REACHES: u32 = 16u;
const R_WOODS: u32 = 17u;
const R_WINDOWS: u32 = 18u;

fn life(pa: u32) -> u32 {
    return prow(pa, R_LIFE).x;
}

fn lived_pa(pa: u32) -> u32 {
    return prow(pa, R_LIVED).x;
}

fn successor_pa(pa: u32) -> u32 {
    return prow(pa, R_LIVED).y;
}

// The reach of stage `k` from a bud of `pa` (`PaRow::reaches`).
fn reach(pa: u32, k: u32) -> f32 {
    return bitcast<f32>(words[prow(pa, R_REACHES).x + k].x);
}

fn log_wood_at(pa: u32, cycle: u32) -> f32 {
    return bitcast<f32>(words[prow(pa, R_WOODS).x + cycle].x);
}

// The window of a draw of `pa` from `cycle` at a whole stake (0), a
// failed unit's (1) or an abortion's (2) (`Windows::window`, no share
// used), as the CPU took it.
fn window_none(pa: u32, cycle: u32, stake: u32) -> f32 {
    return bitcast<f32>(words[prow(pa, R_WINDOWS).x + stake * (cfg.age + 2u) + cycle].x);
}

// The same `used` ticks into `cycle`, at a whole stake, or at the move's
// (`decide::moved`): the wood between the cycle's and the next's logs.
fn window_used(pa: u32, cycle: u32, used: u32, rate: f32, floor: f32) -> f32 {
    let wood = wood_at(log_wood_at(pa, cycle), log_wood_at(pa, cycle + 1u), w(used, 0u));
    return window(wood, rate, floor, 2.0);
}

fn window_whole(pa: u32, cycle: u32, used: u32) -> f32 {
    let s = prow(pa, R_STAKE1);
    return window_used(pa, cycle, used, bitcast<f32>(s.x), bitcast<f32>(s.y));
}
