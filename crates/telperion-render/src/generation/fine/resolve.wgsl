// A chain's axes and phytomers as the geometry passes read them
// (`geometry.wgsl`, `settle.wgsl`), and how far an axis is pulled up.
// Part of the geometry kernel's shader.

// The chain's own kept axes, numbered in order among every own axis,
// kept or not, as realized growth placed them in the wave's laid buffer,
// and their phytomers in order along each, by the region's indices.
fn own_axes(na: u32, np: u32) {
    var n = 0u;
    var phytomers = 0u;
    for (var i = 0u; i < na; i++) {
        let own = i == 1u || (i > 1u && kind_of(i) != LATERAL);
        ss(i, S_OWN, NONE);
        if !own {
            continue;
        }
        if (ax(i, SHED_FLAGS) & 1u) != 0u {
            ss(i, S_OWN, n);
            ss(i, S_START, phytomers);
            ss(i, S_COUNT, 0u);
        }
        phytomers += ax(i, NPHYT);
        n += 1u;
    }
    for (var p = 0u; p < np; p++) {
        let i = ph(p, 3u);
        if s(i, S_OWN) == NONE {
            continue;
        }
        let k = s(i, S_COUNT);
        scratch[sp + (s(i, S_START) + k) * SP + P_ORDER] = p;
        ss(i, S_COUNT, k + 1u);
    }
}

// How far an axis is pulled towards the vertical at its base
// (`geometry::bend`).
fn bend_of(i: u32, f: Pa) -> f32 {
    let years = f32(cfg.age - min(ax(i, BIRTH), cfg.age)) - f32(ax(i, SLEEP)) * (1.0 / 1048576.0);
    let k = kind_of(i);
    let b = axf(i, BLEND);
    let erected = gone(f.erection * years);
    if k == CONTINUED {
        return 0.0;
    }
    if k == RELAYED {
        return 1.0 - (1.0 - f.straightening * b) * (1.0 - b * erected);
    }
    return 1.0 - (1.0 - f.straightening) * (1.0 - erected);
}
