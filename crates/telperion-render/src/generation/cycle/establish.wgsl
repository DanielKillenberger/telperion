// fn-275 R2a (`pipeline/space/establish.rs`): a rosette's stem
// establishing with its crown in the dress, appended to `dress.wgsl`. A
// stem phytomer laid at the crown's age `t` is narrowed to the girth of an
// organ made then and lowered by the rise its stem waited on the crown;
// an axis off the stem is carried by its bearing stem node's lowering.

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

// The crown's age axis `a`'s `k`th phytomer was laid at: evenly over the
// year among those its axis laid in its cycle (`Establishing::laid`).
fn laid(a: u32, k: u32) -> f32 {
    let c = np(NP_CYCLE, ph(a, k));
    var j = 0u;
    // cap: the axis's phytomers.
    while j < k && np(NP_CYCLE, ph(a, k - j - 1u)) == c {
        j++;
    }
    var n = j;
    // cap: the axis's phytomers.
    while k + n - j < count(a) && np(NP_CYCLE, ph(a, k + n - j)) == c {
        n++;
    }
    return f32(c) - 1.0 + f32(j + 1u) / f32(n);
}

// The metres a stem phytomer laid at `t` is lowered by: the rise it
// waited on the crown (`foliage::waited`).
fn lowered(t: f32) -> f32 {
    let x = clamp(t / cfg.est_years, 0.0, 1.0);
    return cfg.est_rise * cfg.est_years * (x - integral(x));
}

// Axis `a`'s `k`th phytomer's lowering: its own on a stem, its bearing
// node's off it, none where the stem establishes by no rows.
fn lowering(a: u32, k: u32) -> f32 {
    if cfg.est_years <= 0.0 {
        return 0.0;
    }
    if trunk(a) {
        return lowered(laid(a, k));
    }
    return bearing(a);
}

// What axis `a` stands on is lowered by, off a stem: the stem node it
// stands on, through every axis between.
fn bearing(a: u32) -> f32 {
    if cfg.est_years <= 0.0 {
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
            return lowered(laid(p, node));
        }
        x = p;
    }
    return 0.0;
}

// The girth of a stem phytomer as its crown establishes
// (`foliage::establishment`).
fn established(a: u32, k: u32) -> f32 {
    if cfg.est_years <= 0.0 || !trunk(a) {
        return 1.0;
    }
    return 1.0 - (1.0 - cfg.est_early) * (1.0 - course(laid(a, k) / cfg.est_years));
}
