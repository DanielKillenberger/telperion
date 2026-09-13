// The field is evaluated on a cylinder embedded in a plane: its arc coordinate
// is radius * angle, and its local metric is metres around the run. Embedding
// the circle lets cells cross the angular wrap without rounding a ridge count
// or introducing a seam as the radius tapers.
fn bark_hash(p: vec2<f32>) -> f32 {
    var q = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    q += dot(q, q.yzx + 33.33);
    return fract((q.x + q.y) * q.z);
}

fn bark_noise(t: f32) -> f32 {
    let cell = floor(t);
    let f = fract(t);
    return mix(bark_hash(vec2(cell, 7.0)), bark_hash(vec2(cell + 1.0, 7.0)),
        f * f * (3.0 - 2.0 * f));
}

// The nearest two jittered sites cut staggered plates, rather than laying a
// second sinusoid over the ridges. Each circumferential cell has its own cuts.
fn bark_cross_cut(along: f32, seed: f32) -> f32 {
    let cell = floor(along);
    var first = 10.0;
    var second = 10.0;
    for (var i = -1; i <= 1; i++) {
        let site = cell + f32(i);
        let distance = abs(along - site - 0.2 - 0.6 * bark_hash(vec2(site, seed)));
        second = min(second, max(first, distance));
        first = min(first, distance);
    }
    return smoothstep(0.025, 0.20, second - first);
}

fn bark_field(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32) -> f32 {
    if (ridge_scale <= 0.0) { return 0.0; }
    // Young wood is smooth. Diameter must span several ridge widths before
    // relief develops; there is no species switch or extra maturity trait.
    let maturity = smoothstep(2.0, 5.0, 2.0 * radius / ridge_scale);
    let longitudinal = max(plate_scale, ridge_scale * 3.0);
    let drift = 0.7 * (bark_noise(along / (longitudinal * 2.0)) - 0.5);
    let phase = drift * ridge_scale / max(radius, ridge_scale);
    let rotated = vec2(circle.x * cos(phase) - circle.y * sin(phase),
        circle.x * sin(phase) + circle.y * cos(phase));
    let arc = rotated * radius / ridge_scale;
    // Local phase wander keeps adjacent ridges from bending in lockstep.
    // These continuous cylinder projections are periodic at the angle wrap.
    let wander = vec2(
        bark_noise(along / longitudinal + dot(arc, vec2(0.53, 0.71))),
        bark_noise(along / (longitudinal * 0.73) + dot(arc, vec2(-0.61, 0.43))));
    let p = arc + 0.85 * (wander - vec2(0.5));
    let cell = floor(p);
    var first = 10.0;
    var second = 10.0;
    var seed = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let id = cell + vec2(f32(x), f32(y));
            let random = vec2(bark_hash(id), bark_hash(id + vec2(19.1, 3.7)));
            let offset = id + 0.2 + 0.6 * random - p;
            let distance = dot(offset, offset);
            second = min(second, max(first, distance));
            if (distance < first) {
                first = distance;
                seed = bark_hash(id + vec2(41.3, 11.9));
            }
        }
    }
    let ridge = 1.0 - exp(-4.0 * max(second - first, 0.0));
    let cross_wander = 0.4 * bark_noise(dot(arc, vec2(1.13, -0.79))
        + along / (longitudinal * 2.0));
    let plate = bark_cross_cut(along / max(plate_scale, 0.0001) + cross_wander,
        seed * 91.0);
    let amplitude = 0.65 + 0.55 * bark_noise(along / longitudinal + seed * 37.0);
    return ridge_scale * 0.055 * maturity * ridge * amplitude
        * select(1.0, plate, plate_scale > 0.0);
}
