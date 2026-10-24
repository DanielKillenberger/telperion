// fn-275 R2a (`pipeline/space/establish.rs`): a rosette's stem
// establishing with its crown in the dress, appended to `dress.wgsl`. A
// stem phytomer whose tip was laid `h` up reached it at the crown's age
// `h / rise`; it is narrowed to the girth of an organ made then, never
// below the thinnest wood drawn, and lowered by the rise its stem waited
// on the crown; an axis off the stem is carried by its bearing stem
// node's lowering.

// The logistic course's integral from 0 (`unfold::integral`).
fn soft(t: f32) -> f32 {
    return log(1.0 + exp(10.0 * (t - 0.5))) / 10.0;
}

fn integral(x0: f32) -> f32 {
    let x = clamp(x0, 0.0, 1.0);
    let lo = 1.0 / (1.0 + exp(5.0));
    let hi = 1.0 / (1.0 + exp(-5.0));
    return (soft(x) - soft(0.0) - lo * x) / (hi - lo);
}

// Whether the stem establishes: rows stated and a rise to clock it by.
fn establishing() -> bool {
    return cfg.est_years > 0.0 && cfg.est_rise > 0.0;
}

// The metres a stem tip laid `h` up is lowered by: the rise it waited on
// the crown (`foliage::waited`).
fn lowered(h: f32) -> f32 {
    let x = clamp(h / cfg.est_rise / cfg.est_years, 0.0, 1.0);
    return cfg.est_rise * cfg.est_years * (x - integral(x));
}

// Axis `a`'s `k`th phytomer's lowering: its own on a stem, its bearing
// node's off it, none where the stem establishes by no rows.
fn lowering(a: u32, k: u32) -> f32 {
    if !establishing() {
        return 0.0;
    }
    if trunk(a) {
        return lowered(np3(NP_TIP, ph(a, k)).z);
    }
    return bearing(a);
}

// What axis `a` stands on is lowered by, off a stem: the stem node it
// stands on, through every axis between; a stem axis's base, its
// bearer's node.
fn bearing(a: u32) -> f32 {
    if !establishing() {
        return 0.0;
    }
    var x = a;
    // cap: one step an order; no tree is deeper than its axes.
    for (var d = 0u; d < 64u; d++) {
        let kind = kind(x);
        if kind == SEED {
            return 0.0;
        }
        let p = na(NA_PARENT, x);
        var node = na(NA_NODE, x);
        if kind == CONTINUED {
            node = count(p) - 1u;
        }
        if trunk(p) {
            return lowered(np3(NP_TIP, ph(p, node)).z);
        }
        x = p;
    }
    return 0.0;
}

// Axis `a`'s `k`th phytomer's radius `r` as its stem establishes
// (`foliage::establishment`), never below the thinnest wood drawn.
fn established(a: u32, k: u32, r: f32) -> f32 {
    if !establishing() || !trunk(a) {
        return r;
    }
    let t = np3(NP_TIP, ph(a, k)).z / cfg.est_rise;
    let share = 1.0 - (1.0 - cfg.est_early) * (1.0 - course(t / cfg.est_years));
    return max(r * share, min(r, FINEST));
}
