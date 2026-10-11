// fn-256 task 2 (SETTLE-PLAN.md S5): sag's turn (`sag::turn`) and the
// frames a lever is carried between (`sagged::Frame`), in the CPU's f32
// operations, its logs and amounts in turns. Pure: it reads no buffer.

// `amount(HANG)`, and `LN_2`, as the CPU's f32s.
const HANG_AMOUNT: f32 = 0x1.8723a2p-6f;
const LN2: f32 = 0x1.62e430p-1f;

fn log2_q(x: f32) -> f32 {
    if x <= 0.0 {
        return -INF;
    }
    return log2c(x);
}

// A lever's unit and the log2 of its length; `-INF` for none.
struct UnitW {
    u: vec3<f32>,
    l: f32,
}

// The dot product and scaling with gradual underflow (`settle_sub.wgsl`,
// the plain operations where built plain): a lever at a fine tip squares
// below the least normal.
fn sub_dot(a: vec3<f32>, b: vec3<f32>) -> f32 {
    return sub_add(sub_add(sub_mul(a.x, b.x), sub_mul(a.y, b.y)), sub_mul(a.z, b.z));
}

fn sub_scaled(v: vec3<f32>, s: f32) -> vec3<f32> {
    return vec3<f32>(sub_mul(v.x, s), sub_mul(v.y, s), sub_mul(v.z, s));
}

fn sub_sum(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(sub_add(a.x, b.x), sub_add(a.y, b.y), sub_add(a.z, b.z));
}

fn unit_w(v: vec3<f32>) -> UnitW {
    let l = sub_sqrt(sub_dot(v, v));
    if !(l > 0.0) {
        return UnitW(vec3<f32>(0.0), -INF);
    }
    return UnitW(sub_scaled(v, qdiv(1.0, l)), log2c(l));
}

fn torque(lever: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(-lever.y, lever.x, 0.0);
}

// The share of a lever the ground leaves the wood before it (`held`).
fn held(height: f32, chord: vec3<f32>) -> f32 {
    let low = height + chord.z;
    if low >= 0.0 {
        return 1.0;
    }
    if height <= 0.0 {
        return 0.0;
    }
    let free = qdiv(height, height - low);
    return m(free, free);
}

// `2 atan2(sin(fall / 2) e^-k, cos(fall / 2))`, `x = k log2 e` (`settle`).
fn settle_fall(fall: f32, x: f32) -> f32 {
    let cs = qsincos(m(fall, 0.5));
    let c = max(cs.x, 0.0);
    let y = m(cs.y, exp2c(-x));
    if y == 0.0 && c <= 0.0 {
        return 0.0;
    }
    return m(qatan2(y, c), 2.0);
}

// `ln(tan(fall / 2) / tan(high / 2))` (`tangent_ratio`).
fn tangent_ratio(fall: f32, high: f32) -> f32 {
    let f = qsincos(m(fall, 0.5));
    let h = qsincos(m(high, 0.5));
    let a = m(max(f.y, 0.0), max(h.x, 0.0));
    let b = m(max(f.x, 0.0), max(h.y, 0.0));
    if a <= 0.0 {
        return -INF;
    }
    if b <= 0.0 {
        return INF;
    }
    return m(log2c(a) - log2c(b), LN2);
}

// `fall` settled over `s` metres (`turn`'s `settled`).
fn settled(fall: f32, s: f32, c: f32, reach: f32) -> f32 {
    if s <= 0.0 {
        return fall;
    }
    let k = exp2c((c + reach) + log2_q(s));
    return settle_fall(fall, m(k, QLOG2E));
}

// The beam's turn under `lever`, its moment capped at `cap`
// (`sag::turn`), `saglog` the PA's sag's log2.
fn sag_turn(lever: vec3<f32>, cap: f32, saglog: f32, r: f32, left0: f32) -> f32 {
    let uw = unit_w(lever);
    let reach = uw.l;
    if reach <= -INF || cap <= -INF || r <= 0.0 || left0 <= 0.0 {
        return 0.0;
    }
    let c = saglog - m(4.0, log2_q(r));
    let start = qatan2(hypotf(uw.u.x, uw.u.y), -uw.u.z);
    let low = qbetween(0.0, qdiv(asinf(clamp(exp2c(min(cap - reach, 0.0)), -1.0, 1.0)), QTAU));
    let high = 0.5 - low;
    var fall = start;
    var left = left0;
    if fall > high {
        let ln = tangent_ratio(fall, high);
        var s = INF;
        if ln < INF {
            s = m(ln, exp2c(-(c + reach)));
        }
        if s >= left {
            return start - settled(fall, left, c, reach);
        }
        fall = high;
        left = left - s;
    }
    if fall > low {
        let rate = c + cap;
        let s = m(m(fall - low, QTAU), exp2c(-rate));
        if s >= left {
            return start - (fall - qdiv(exp2c(rate + log2_q(left)), QTAU));
        }
        fall = low;
        left = left - s;
    }
    return start - settled(fall, left, c, reach);
}

// A frame (`sagged::Frame`): heading, its side made square and unit, and
// their normal; `ok` none where the side has no length.
struct SagFrame {
    h: vec3<f32>,
    s: vec3<f32>,
    n: vec3<f32>,
    ok: bool,
}

fn sag_frame(heading: vec3<f32>, side: vec3<f32>) -> SagFrame {
    let u = qunit(side - mv(heading, qdot(side, heading)));
    return SagFrame(heading, u.xyz, qcross(heading, u.xyz), u.w > 0.0);
}

// `v`'s parts in frame `a`, carried into frame `b`, summed from none.
fn frame_carry(a: SagFrame, b: SagFrame, v: vec3<f32>) -> vec3<f32> {
    let p = vec3<f32>(sub_dot(v, a.h), sub_dot(v, a.s), sub_dot(v, a.n));
    let zero = vec3<f32>(apart(0.0));
    return sub_sum(sub_sum(sub_sum(zero, sub_scaled(b.h, p.x)), sub_scaled(b.s, p.y)), sub_scaled(b.n, p.z));
}

