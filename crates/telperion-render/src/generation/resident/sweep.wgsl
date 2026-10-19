// The tree's wood as curves on the device (fn-212 task 5, fn-256 task
// 3): F3's curve (`pipeline/surface/curve.rs`) over the nodes the cycle's
// dress drew, read in its records by id (fn-293). `unlinked`, `lead`,
// `ties` and `starts` find each node's leader and every run's first node as
// `paths.rs` does, the seed's run starting at the root's seam; `walk`
// sizes each run and finds its widest ring; the runs are sorted widest
// first (`bitonic`, as `rings.rs` ranks them); `block`, `blocks` and
// `place` give each its points and clusters; `emit` samples, frames and
// clusters each run (`samples.rs`, `frames.rs`, `curve.rs`) in the
// renderer's packed form (`pack.wgsl`, `packed_clusters`), and folds the
// curve's demand and bounds (`Curve::demand`, `CurveCluster::bounds`)
// while it holds each frame whole; `total` sums the clusters' sags. A
// retained leaf base's run that packs into its stem's lattice is swept on
// its cell's rings, its frames squared to the cell (`section.rs`; fn-266,
// DESIGN.md 12.8).

struct Cfg {
    n: u32,
    row: u32,
    j: u32,
    k: u32,
    // Where the drawn axes start, where the nodes' contacts start in the
    // table, and the drawn ids.
    axes: u32,
    contacts: u32,
    ids: u32,
    _a: u32,
    // The surface's rows: lobe depth, cos(π / segments), fork swell,
    // fork socket; socket containment, flare radius, flare falloff,
    // height; the lobes' twist (radians a metre) and depth, the lobe
    // factor of a ring's reach, and how deep a trunk's foot is buried.
    rows: array<vec4<f32>, 3>,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
// The drawn records by id, then the axes by slot (`fold.rs`'s `Dressed`).
@group(0) @binding(1) var<storage, read> drawn: array<f32>;
// The retained leaf bases' cells (`boots.wgsl`) in the curve's section
// layout, or one unread word.
@group(0) @binding(2) var<storage, read> cells: array<f32>;
@group(0) @binding(3) var<storage, read_write> links: array<atomic<u32>>;
// The runs, points and clusters counted; in `emit` and `total`, the
// curve's sums: its stretches, the bounds' corners as ordered bits, a
// pad, one partial sum of the sags a lane of `total`, then each
// cluster's sag.
@group(0) @binding(4) var<storage, read_write> counts: array<atomic<u32>>;
@group(0) @binding(5) var<storage, read_write> keys: array<vec2<u32>>;
// Each run's place (`TW` words), each block's totals, then from
// `cfg.contacts` each node's contact: its place along the run it is
// swept in (from 1; 0 where no run holds it) and that run's place.
@group(0) @binding(6) var<storage, read_write> table: array<u32>;
// The renderer's packed points and clusters (`POINT_WORDS`, `KW`).
@group(0) @binding(7) var<storage, read_write> points: array<u32>;
@group(0) @binding(8) var<storage, read_write> clusters: array<u32>;

const NONE: u32 = 0xffffffffu;
const CA: u32 = 12u;
const CV: u32 = 38u;
const MADE: u32 = 2u;
const FINE: u32 = 8u;
const SEAM: u32 = 16u;
// The record a base's run opens at where it is drawn as its cell
// (`boots.wgsl`), the cell's index in its word 29.
const CELL: u32 = 0x20000000u;
const SECTION: u32 = 28u;
// A node's links: the widest start radius of its children that are not
// codominant and of all, the first child of each so wide, whether a
// codominant child leaves it, the points of the run it starts; then, for
// each of the two, the least order key, lineage high and low word among
// the children so wide.
const LW: u32 = 12u;
// A run's place in the table: its first point and cluster, its points,
// its first node.
const TW: u32 = 4u;
const KW: u32 = 16u;
const CLUSTER: u32 = 32u;
const LEVELS: u32 = 6u;
// The curve's sums in `counts`: the bounds' corners, then the lanes'
// partial sums of the sags, then each cluster's.
const LO: u32 = 1u;
const HI: u32 = 4u;
const LANES: u32 = 256u;
const LANE0: u32 = 8u;
const SAG0: u32 = 264u;

fn ord(f: f32) -> u32 {
    let b = bitcast<u32>(f);
    if (b & 0x80000000u) != 0u {
        return ~b;
    }
    return b | 0x80000000u;
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

// Word `f` of node `i`'s drawn record (`dress.wgsl`'s `CV`).
fn rec(i: u32, f: u32) -> f32 {
    return drawn[i * CV + f];
}

fn word(i: u32, f: u32) -> u32 {
    return bitcast<u32>(rec(i, f));
}

// Word `f` of node `i`'s axis (`dress.wgsl`'s `CA`), at its record's slot.
fn axis(i: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[cfg.axes + word(i, 9u) * CA + f]);
}

fn pos(i: u32) -> vec3<f32> {
    return vec3<f32>(rec(i, 0u), rec(i, 1u), rec(i, 2u));
}

fn radius(i: u32) -> f32 {
    return rec(i, 3u);
}

fn start(i: u32) -> f32 {
    return rec(i, 4u);
}

fn codominant(i: u32) -> bool {
    return rec(i, 13u) > 0.0;
}

// Its flags where the dress made it a node; none where it did not.
fn flags(i: u32) -> u32 {
    let f = word(i, 6u);
    return select(0u, f, (f & MADE) != 0u);
}

fn parent(i: u32) -> u32 {
    return word(i, 16u);
}

// From its root's bearer, which every node of a run shares.
fn local(i: u32) -> f32 {
    return rec(i, 15u);
}

// Its bearer's distance from the root (its axis's), then its own.
fn along(i: u32) -> f32 {
    return bitcast<f32>(axis(i, 7u)) + local(i);
}

// A node the device draws a run of: one the dress drew, not the root
// (which the dress draws neither fine nor at a distance).
fn drawn_here(i: u32) -> bool {
    let f = flags(i);
    return (f & MADE) != 0u && (f & FINE) != 0u;
}

// Each node's links before any child is weighed: no width, no child,
// no order key.
@compute @workgroup_size(64)
fn unlinked(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    for (var f = 0u; f < LW; f++) {
        atomicStore(&links[i * LW + f], select(0u, NONE, f == 2u || f == 3u || f >= 6u));
    }
}

// A child that may carry its parent's run on: a drawn node, never the
// seed's first, off the root.
fn follows(i: u32) -> bool {
    return drawn_here(i) && (flags(i) & SEAM) == 0u;
}

@compute @workgroup_size(64)
fn lead(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !follows(i) {
        return;
    }
    let p = parent(i);
    let key = ord(start(i));
    if codominant(i) {
        atomicStore(&links[p * LW + 4u], 1u);
    } else {
        atomicMax(&links[p * LW], key);
    }
    atomicMax(&links[p * LW + 1u], key);
}

// The first order word: its axis's depth, then the lineage's high bits.
fn order(i: u32) -> u32 {
    return (min(axis(i, 8u), 63u) << 26u) | (axis(i, 10u) >> 6u);
}

// The order's words: the first, then the lineage's.
fn orders(i: u32) -> array<u32, 3> {
    return array<u32, 3>(order(i), axis(i, 10u), axis(i, 9u));
}

// Whether child `i` is as wide as the widest of its parent's that may
// carry the run on (`wide` 0) or of all (`wide` 1), and as early so far
// as `stage` words of the order: of those, the conversion's first.
fn tied(i: u32, wide: u32, stage: u32) -> bool {
    let p = parent(i);
    let key = ord(start(i));
    if wide == 0u && codominant(i) {
        return false;
    }
    if key != atomicLoad(&links[p * LW + wide]) {
        return false;
    }
    let at = p * LW + 6u + 3u * wide;
    let words = orders(i);
    for (var w = 0u; w < stage; w++) {
        if words[w] != atomicLoad(&links[at + w]) {
            return false;
        }
    }
    return true;
}

// Narrows each node's widest children to the conversion's first: by the
// order key, the lineage's words, then the id.
@compute @workgroup_size(64)
fn ties(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !follows(i) {
        return;
    }
    let p = parent(i);
    let words = orders(i);
    for (var wide = 0u; wide < 2u; wide++) {
        if !tied(i, wide, cfg.j) {
            continue;
        }
        if cfg.j < 3u {
            atomicMin(&links[p * LW + 6u + 3u * wide + cfg.j], words[cfg.j]);
        } else {
            atomicMin(&links[p * LW + 2u + wide], i);
        }
    }
}

// The child that carries `at`'s run on: the widest that is not
// codominant, or the widest where all are.
fn leader(at: u32) -> u32 {
    let l = atomicLoad(&links[at * LW + 2u]);
    if l != NONE {
        return l;
    }
    return atomicLoad(&links[at * LW + 3u]);
}

@compute @workgroup_size(64)
fn starts(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !drawn_here(i) {
        return;
    }
    if (flags(i) & SEAM) != 0u || leader(parent(i)) != i {
        let k = atomicAdd(&counts[0], 1u);
        keys[k] = vec2<u32>(0u, i);
    }
}

// The sweep's samples along one run, in turn (`samples.rs`): a run off
// the root is a trunk's (`trunk_run`), its foot buried and its first
// sample the root's own; any other sinks into its bearer's socket
// (`branch_run`).
struct Walk {
    attach: u32,
    at: u32,
    k: u32,
    fork: u32,
    pr: f32,
    trunk: bool,
}

// The root's id, which every trunk run stands on.
const ROOT: u32 = 0u;

// The samples a trunk's foot adds: the buried ring, where it has one.
fn foot(w: Walk) -> u32 {
    return select(0u, 1u, w.trunk && cfg.rows[2].w > 0.0);
}

fn flare(y: f32) -> f32 {
    let r = cfg.rows;
    return 1.0 + (r[1].y - 1.0) * exp(-max(y, 0.0) / (r[1].z * r[1].w));
}

fn begin(first: u32) -> Walk {
    let attach = parent(first);
    return Walk(attach, first, 0u, NONE, radius(attach), attach == ROOT);
}

struct Sample {
    p: vec3<f32>,
    r: f32,
    d: f32,
}

// The walk's next sample: its centre, radius and distance along.
fn sample(w: ptr<function, Walk>) -> Sample {
    let r = cfg.rows;
    let swell = r[0].z;
    if (*w).trunk && (*w).k <= foot(*w) {
        let a = (*w).attach;
        let p = pos(a);
        let s = radius(a) * flare(p.y);
        if (*w).k < foot(*w) {
            (*w).k += 1u;
            return Sample(p - vec3<f32>(0.0, r[2].w, 0.0), s, 0.0);
        }
        (*w).k += 1u;
        return Sample(p, s, along(a));
    }
    if (*w).k == 0u {
        (*w).k = 1u;
        let a = (*w).attach;
        let pr = (*w).pr;
        let away = normalize(pos((*w).at) - pos(a));
        let inscribed = pr * (1.0 - r[0].x) * r[0].y;
        let sink = min(r[0].w * pr, r[1].x * inscribed);
        let contained = sqrt(max(inscribed * inscribed - sink * sink, 0.0)) / (1.0 + r[0].x);
        let p = pos(a) - away * sink;
        return Sample(p, min(start((*w).at) * swell, contained) * flare(pos(a).y), along(a));
    }
    let i = (*w).at;
    let own = radius(i);
    var girth = own;
    let f = (*w).fork;
    if f != NONE {
        let trunk = radius(f);
        let t = (local(i) - local(f)) / max(2.0 * trunk, 1e-9);
        var held = 0.0;
        if t < 1.0 {
            held = (1.0 - t) * (1.0 - t) * (1.0 + 2.0 * t);
        }
        girth = own + max(trunk - own, 0.0) * held;
    }
    if atomicLoad(&links[i * LW + 4u]) != 0u {
        (*w).fork = i;
    }
    // A bearer's node stands at its own root's distance 0; a trunk has no
    // socket to swell from.
    var swelling = 1.0 + (swell - 1.0) * exp(-(local(i) - local((*w).attach)) / max((*w).pr, 1e-9));
    if (*w).trunk {
        swelling = 1.0;
    }
    (*w).at = leader(i);
    (*w).k += 1u;
    return Sample(pos(i), girth * swelling * flare(pos(i).y), along(i));
}

// The cell run `first` is drawn as, or none for a round run.
fn shape(first: u32) -> u32 {
    if (flags(first) & CELL) == 0u {
        return NONE;
    }
    return word(first, 29u);
}

fn cell3(c: u32, at: u32) -> vec3<f32> {
    let b = c * SECTION + at;
    return vec3<f32>(cells[b], cells[b + 1u], cells[b + 2u]);
}

// Cell `c`'s ring for sample `k` of `samples` (`Section::ring`): reach,
// rise and scale.
fn ring(c: u32, k: u32, samples: u32) -> vec3<f32> {
    return cell3(c, 17u + 3u * (k + 3u - clamp(samples, 2u, 3u)));
}

// How far a ring's vertices stand from its centre (`Section::extent`).
fn extent(c: u32, r: vec3<f32>) -> f32 {
    let b = c * SECTION + 12u;
    let a = vec2<f32>(cells[b] * r.x, cells[b + 1u]);
    let d = vec2<f32>(cells[b + 2u] * r.x, cells[b + 3u]);
    return sqrt(dot(a, a) + dot(d, d)) * r.z;
}

@compute @workgroup_size(64)
fn walk(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    let first = keys[k].y;
    var w = begin(first);
    var widest = 0.0;
    var count = 0u;
    // cap: one sample a node, the root's and a buried foot.
    let cap = cfg.ids + 2u;
    loop {
        if count >= cap {
            break;
        }
        let s = sample(&w);
        widest = max(widest, s.r);
        count += 1u;
        if w.at == NONE {
            break;
        }
    }
    let cell = shape(first);
    if cell != NONE {
        widest = 0.0;
        for (var k = 0u; k < count; k++) {
            widest = max(widest, extent(cell, ring(cell, k, count)));
        }
    }
    atomicStore(&links[first * LW + 5u], count);
    keys[k] = vec2<u32>(~ord(widest), first);
}

@compute @workgroup_size(64)
fn bitonic(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    let l = i ^ cfg.j;
    if i >= cfg.n || l <= i {
        return;
    }
    let a = keys[i];
    let b = keys[l];
    let greater = a.x > b.x || (a.x == b.x && a.y > b.y);
    let up = (i & cfg.k) == 0u;
    if greater == up {
        keys[i] = b;
        keys[l] = a;
    }
}

// The clusters of a run of `count` points.
fn clustered(count: u32) -> u32 {
    return (count - 1u + CLUSTER - 2u) / (CLUSTER - 1u);
}

var<workgroup> scan_p: array<u32, 256>;
var<workgroup> scan_c: array<u32, 256>;

// Each sorted run's points and clusters scanned within its block of 256;
// each block's totals after the table.
@compute @workgroup_size(256)
fn block(@builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wg: vec3<u32>) {
    let b = wg.x + wg.y * 65535u;
    let s = b * 256u + lid.x;
    var p = 0u;
    var c = 0u;
    if s < cfg.n {
        p = atomicLoad(&links[keys[s].y * LW + 5u]);
        c = clustered(p);
    }
    scan_p[lid.x] = p;
    scan_c[lid.x] = c;
    workgroupBarrier();
    for (var o = 1u; o < 256u; o = o * 2u) {
        var ap = 0u;
        var ac = 0u;
        if lid.x >= o {
            ap = scan_p[lid.x - o];
            ac = scan_c[lid.x - o];
        }
        workgroupBarrier();
        scan_p[lid.x] += ap;
        scan_c[lid.x] += ac;
        workgroupBarrier();
    }
    if s < cfg.n {
        table[s * TW] = scan_p[lid.x] - p;
        table[s * TW + 1u] = scan_c[lid.x] - c;
        table[s * TW + 2u] = p;
        table[s * TW + 3u] = keys[s].y;
    }
    if lid.x == 255u {
        table[cfg.n * TW + 2u * b] = scan_p[255];
        table[cfg.n * TW + 2u * b + 1u] = scan_c[255];
    }
}

// The blocks' totals scanned in turn, and the whole's into `counts`.
@compute @workgroup_size(1)
fn blocks() {
    let n = (cfg.n + 255u) / 256u;
    var p = 0u;
    var c = 0u;
    for (var b = 0u; b < n; b++) {
        let at = cfg.n * TW + 2u * b;
        let bp = table[at];
        let bc = table[at + 1u];
        table[at] = p;
        table[at + 1u] = c;
        p += bp;
        c += bc;
    }
    atomicStore(&counts[1], p);
    atomicStore(&counts[2], c);
}

@compute @workgroup_size(64)
fn place(@builtin(global_invocation_id) id: vec3<u32>) {
    let s = index(id);
    if s >= cfg.n {
        return;
    }
    let at = cfg.n * TW + 2u * (s / 256u);
    table[s * TW] += table[at];
    table[s * TW + 1u] += table[at + 1u];
}

fn rotate(v: vec3<f32>, was: vec3<f32>, now: vec3<f32>) -> vec3<f32> {
    var w = dot(was, now) + 1.0;
    var q = cross(was, now);
    if w < 1e-8 {
        w = 0.0;
        if abs(was.x) > abs(was.z) {
            q = vec3<f32>(-was.y, was.x, 0.0);
        } else {
            q = vec3<f32>(0.0, -was.z, was.y);
        }
    }
    let inv = 1.0 / sqrt(dot(q, q) + w * w);
    q = q * inv;
    w = w * inv;
    let t = cross(q, v) * 2.0;
    return vec3<f32>(
        v.x + w * t.x + q.y * t.z - q.z * t.y,
        v.y + w * t.y + q.z * t.x - q.x * t.z,
        v.z + w * t.z + q.x * t.y - q.y * t.x,
    );
}

fn perpendicular(v: vec3<f32>) -> vec3<f32> {
    let a = abs(v);
    var reference = vec3<f32>(0.0, 0.0, 1.0);
    if a.x <= a.y && a.x <= a.z {
        reference = vec3<f32>(1.0, 0.0, 0.0);
    } else if a.y <= a.z {
        reference = vec3<f32>(0.0, 1.0, 0.0);
    }
    return normalize(cross(reference, v));
}

// The unit step between two samples; `last` where they meet.
fn segment(a: Sample, b: Sample, last: vec3<f32>) -> vec3<f32> {
    let step = b.p - a.p;
    if dot(step, step) > 0.0 {
        return normalize(step);
    }
    return last;
}

// A stretch's sag (`plan.rs`, `sag`): from sample `a`, its tangent `ta`,
// to the next, `b` and `tb`.
fn sag(a: Sample, ta: vec3<f32>, b: Sample, tb: vec3<f32>) -> f32 {
    // The angle between the tangents, well conditioned where they nearly
    // agree (an f32 `acos` of a dot within an ulp of 1 is not).
    let turn = atan2(length(cross(ta, tb)), dot(ta, tb));
    let chord = b.p - a.p;
    let span = length(chord);
    let bent = max(
        length(chord - (ta * 2.0 + tb) * (span / 3.0)),
        length(chord - (ta + tb * 2.0) * (span / 3.0)),
    );
    let radius = max(a.r, b.r);
    let phase = cfg.rows[2].x * abs(b.d - a.d) * sqrt(cfg.rows[2].y);
    return max(span * turn, 6.0 * bent) + radius * phase * phase;
}

// What a run's points fold into as they are packed: the box round every
// ring, the open cluster's sum of root sags, and the point before.
struct Fold {
    lo: vec3<f32>,
    hi: vec3<f32>,
    root: f32,
    before: Sample,
    tangent: vec3<f32>,
}


// A run: its place in the table, its first point and its points.
struct Run {
    s: u32,
    at: u32,
    count: u32,
}

// Point `i` of `run` written in the renderer's packed form, and folded
// into `f` while its frame is whole: its ring's box (each ring's own
// extent) and the stretch to it; a cluster's sum of root sags is stored
// as its last stretch closes.
fn put(f: ptr<function, Fold>, run: Run, i: u32, x: Sample, normal: vec3<f32>, binormal: vec3<f32>) {
    let b = (run.at + i) * POINT_WORDS;
    points[b] = bitcast<u32>(x.p.x);
    points[b + 1u] = bitcast<u32>(x.p.y);
    points[b + 2u] = bitcast<u32>(x.p.z);
    points[b + 3u] = bitcast<u32>(x.r);
    points[b + 4u] = bitcast<u32>(x.d);
    let w = frame_words(normal, binormal);
    points[b + 5u] = w.x;
    points[b + 6u] = w.y;
    points[b + 7u] = w.z;
    let e = sqrt(normal * normal + binormal * binormal) * (x.r * cfg.rows[2].z);
    let tangent = cross(normal, binormal);
    if i == 0u {
        (*f).lo = x.p;
        (*f).hi = x.p;
    } else {
        (*f).root += sqrt(sag((*f).before, (*f).tangent, x, tangent));
        if i % (CLUSTER - 1u) == 0u || i + 1u == run.count {
            let c = table[run.s * TW + 1u] + (i - 1u) / (CLUSTER - 1u);
            atomicStore(&counts[SAG0 + c], bitcast<u32>((*f).root));
            (*f).root = 0.0;
        }
    }
    (*f).lo = min((*f).lo, x.p - e);
    (*f).hi = max((*f).hi, x.p + e);
    (*f).before = x;
    (*f).tangent = tangent;
}

// A swept run's stretches and box added to the curve's.
fn close(f: Fold, run: Run) {
    atomicAdd(&counts[0], run.count - 1u);
    for (var k = 0u; k < 3u; k++) {
        atomicMin(&counts[LO + k], ord(f.lo[k]));
        atomicMax(&counts[HI + k], ord(f.hi[k]));
    }
}

fn centre_of(p: u32) -> vec3<f32> {
    let b = p * POINT_WORDS;
    return bitcast<vec3<f32>>(vec3<u32>(points[b], points[b + 1u], points[b + 2u]));
}

fn radius_of(p: u32) -> f32 {
    return bitcast<f32>(points[p * POINT_WORDS + 3u]);
}

fn along_of(p: u32) -> f32 {
    return bitcast<f32>(points[p * POINT_WORDS + 4u]);
}

// Each sorted run swept: its samples and their transported frames
// (`frames.rs`), then its clusters (`curve.rs`).
@compute @workgroup_size(64)
fn emit(@builtin(global_invocation_id) id: vec3<u32>) {
    let s = index(id);
    if s >= cfg.n {
        return;
    }
    let first = table[s * TW + 3u];
    let run = Run(s, table[s * TW], table[s * TW + 2u]);
    var f: Fold;
    let cell = shape(first);
    if cell != NONE {
        shaped(&f, run, first, cell);
        return;
    }
    let count = run.count;
    var w = begin(first);
    var cur = sample(&w);
    var next = sample(&w);
    var seg_before = segment(cur, next, vec3<f32>(0.0, 1.0, 0.0));
    var seg = seg_before;
    var previous = seg;
    // Seeded from the bearer's wood's frame at the socket: its side, laid
    // across the first tangent (`frames.rs`, `seed`; fn-212, 2026-10-07).
    let a = parent(first);
    var normal = vec3<f32>(rec(a, 20u), rec(a, 21u), rec(a, 22u));
    for (var i = 0u; i < count; i++) {
        var t = seg;
        if i > 0u && i + 1u < count {
            let mean = seg_before + seg;
            t = seg;
            if dot(mean, mean) > 1e-9 {
                t = normalize(mean);
            }
        } else if i + 1u == count {
            t = seg_before;
        }
        if i > 0u {
            normal = rotate(normal, previous, t);
        }
        normal = normal + t * (-dot(normal, t));
        if dot(normal, normal) <= 1e-9 {
            normal = perpendicular(t);
        }
        normal = normalize(normal);
        put(&f, run, i, cur, normal, normalize(cross(t, normal)));
        previous = t;
        if i + 1u < count {
            cur = next;
            seg_before = seg;
            if i + 2u < count {
                next = sample(&w);
                seg = segment(cur, next, seg);
            }
        }
    }
    contacts(run, first);
    clusters_of(run, 0u);
    close(f, run);
}

// Each node's ring in its run, which its contacts read: past a trunk's
// buried foot (`rings.rs`, `record`).
fn contacts(run: Run, first: u32) {
    var node = first;
    for (var i = 1u + foot(begin(first)); i < run.count && node != NONE; i++) {
        table[cfg.contacts + 2u * node] = i;
        table[cfg.contacts + 2u * node + 1u] = run.s;
        node = leader(node);
    }
}

// A run drawn as cell `cell` (`section::reshape`, `section::square`):
// each ring centred on the radial at its reach and rise, its radius the
// ring's extent, its distance along the run's own, and every frame
// across the stem and along it.
fn shaped(f: ptr<function, Fold>, run: Run, first: u32, cell: u32) {
    var w = begin(first);
    let origin = cell3(cell, 0u);
    let axis = cell3(cell, 3u);
    let radial = cell3(cell, 6u);
    let across = cell3(cell, 9u);
    for (var i = 0u; i < run.count; i++) {
        let r = ring(cell, i, run.count);
        let d = sample(&w).d;
        put(f, run, i, Sample(origin + axis * r.y + radial * r.x, extent(cell, r), d), across, axis);
    }
    contacts(run, first);
    clusters_of(run, 4u | (cell << 8u));
    close(*f, run);
}

// The run's clusters in the renderer's packed form (`packed_clusters`):
// each its first point and points, its run's first point and points, its
// flags (the run's first, its last, and the run's shape: 4 and the cell's
// index from bit 8 for a shaped run, `form`), its sphere's centre and
// reach, its largest radius and its ring levels' errors.
fn clusters_of(run: Run, form: u32) {
    let r = cfg.rows;
    let twist = r[2].x;
    let depth = r[2].y;
    let lobe = r[2].z;
    let at = run.at;
    let count = run.count;
    var c = table[run.s * TW + 1u];
    var start = 0u;
    // cap: start moves on by CLUSTER each step, to count.
    loop {
        let end = min(start + CLUSTER - 1u, count - 1u);
        var lo = centre_of(at + start);
        var hi = lo;
        var largest = 0.0;
        for (var p = start; p <= end; p++) {
            lo = min(lo, centre_of(at + p));
            hi = max(hi, centre_of(at + p));
            largest = max(largest, radius_of(at + p));
        }
        let centre = (lo + hi) * 0.5;
        var reach = 0.0;
        for (var p = start; p <= end; p++) {
            reach = max(reach, distance(centre_of(at + p), centre) + radius_of(at + p) * lobe);
        }
        let b = c * KW;
        clusters[b] = at + start;
        clusters[b + 1u] = end - start + 1u;
        clusters[b + 2u] = at;
        clusters[b + 3u] = count;
        clusters[b + 4u] = u32(start == 0u) | (u32(end + 1u == count) << 1u) | form;
        clusters[b + 5u] = bitcast<u32>(centre.x);
        clusters[b + 6u] = bitcast<u32>(centre.y);
        clusters[b + 7u] = bitcast<u32>(centre.z);
        clusters[b + 8u] = bitcast<u32>(reach);
        clusters[b + 9u] = bitcast<u32>(largest);
        ladder(b + 10u, at + start, end - start, twist, depth);
        c += 1u;
        if end + 1u >= count {
            break;
        }
        start = end;
    }
}

// A cluster's ring levels' errors (`curve.rs`, `ladder`), from point
// `p0`, its last `last` points on.
fn ladder(out: u32, p0: u32, last: u32, twist: f32, depth: f32) {
    clusters[out] = 0u;
    var below = 0.0;
    for (var k = 1u; k < LEVELS; k++) {
        let step = 1u << k;
        var worst = 0.0;
        var a = 0u;
        // cap: a moves on by step each pass, to last.
        while a < last {
            let b = min(a + step, last);
            let pa = p0 + a;
            let pb = p0 + b;
            let da = along_of(pa);
            let length = max(along_of(pb) - da, 1e-12);
            for (var q = a + 1u; q < b; q++) {
                let p = p0 + q;
                let t = clamp((along_of(p) - da) / length, 0.0, 1.0);
                let line = centre_of(pa) + (centre_of(pb) - centre_of(pa)) * t;
                let ra = radius_of(pa);
                let rad = ra + (radius_of(pb) - ra) * t;
                let phase = twist * (along_of(pb) - da);
                let rp = radius_of(p);
                let lobes = rp * depth * min(phase * phase / 8.0, 2.0);
                worst = max(worst, distance(centre_of(p), line) + abs(rp - rad) + lobes);
            }
            a = b;
        }
        below = max(worst, below);
        clusters[out + k] = bitcast<u32>(below);
    }
}

// Each lane's partial sum of the clusters' sags, compensated.
@compute @workgroup_size(256)
fn total(@builtin(local_invocation_index) lane: u32) {
    var sum = 0.0;
    var lost = 0.0;
    for (var c = lane; c < cfg.n; c += LANES) {
        let y = bitcast<f32>(atomicLoad(&counts[SAG0 + c])) - lost;
        let t = sum + y;
        lost = (t - sum) - y;
        sum = t;
    }
    atomicStore(&counts[LANE0 + lane], bitcast<u32>(sum));
}
