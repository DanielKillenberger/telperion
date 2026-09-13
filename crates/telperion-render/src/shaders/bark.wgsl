// A circle embedding preserves the metre arc metric and crosses the angular
// wrap continuously. Sites partition only the circumference, into columns;
// their slow axial drift is independent of the shorter scale breaks.
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

// Independent axial/circumferential noise avoids diagonal waves in the cuts.
fn bark_noise2(p: vec2<f32>) -> f32 {
    let c = floor(p);
    let f = fract(p);
    let w = f * f * (3.0 - 2.0 * f);
    return mix(mix(bark_hash(c), bark_hash(c + vec2(1.0, 0.0)), w.x),
        mix(bark_hash(c + vec2(0.0, 1.0)), bark_hash(c + vec2(1.0)), w.x), w.y);
}

// Signed distance to the nearest column boundary, plus the column's seed.
// Dividing the squared-distance difference gives shoulders of physical width,
// rather than hairlines whose widths depend on the distance between sites.
fn bark_column(p: vec2<f32>) -> vec2<f32> {
    let cell = floor(p);
    var first = 10.0;
    var second = 10.0;
    var nearest = vec2(0.0);
    var next = vec2(0.0);
    var seed = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let id = cell + vec2(f32(x), f32(y));
            let random = vec2(bark_hash(id), bark_hash(id + vec2(19.1, 3.7)));
            let offset = id + 0.2 + 0.6 * random - p;
            let distance = dot(offset, offset);
            if (distance < first) {
                second = first;
                next = nearest;
                first = distance;
                nearest = offset;
                seed = bark_hash(id + vec2(41.3, 11.9));
            } else if (distance < second) {
                second = distance;
                next = offset;
            }
        }
    }
    return vec2(0.5 * (second - first) / max(length(next - nearest), 0.001), seed);
}

// Uneven intervals, independently staggered per column. The short upward
// step at a plate's lower edge faces away from the overhead light; above it
// a lifted lip falls back onto a flat face. This is height, not painted shade.
fn bark_scale(along: f32, seed: f32) -> vec2<f32> {
    let cell = floor(along);
    var lower = cell - 1.0;
    var upper = cell + 2.0;
    for (var i = -1; i <= 1; i++) {
        let site = cell + f32(i);
        let cut = site + 0.15 + 0.7 * bark_hash(vec2(site, seed));
        if (cut <= along) { lower = max(lower, cut); }
        else { upper = min(upper, cut); }
    }
    let t = (along - lower) / (upper - lower);
    let face = smoothstep(0.0, 0.09, t) * (1.0 - smoothstep(0.86, 1.0, t));
    let lip = smoothstep(0.0, 0.065, t) * (1.0 - smoothstep(0.075, 0.3, t));
    let strength = mix(0.65, 1.3, bark_hash(vec2(lower, seed + 29.0)));
    return vec2(strength * (face + 0.18 * lip), face * (1.0 - lip));
}

fn bark_field_filtered(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32, footprint: f32) -> f32 {
    if (ridge_scale <= 0.0) { return 0.0; }
    let maturity = smoothstep(2.0, 5.0, 2.0 * radius / ridge_scale);
    // Girth continues to strengthen mature wood after the young-run fade.
    let girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale);
    let elongated = smoothstep(2.0, 6.0, plate_scale / ridge_scale);
    // Longer furrows retain short scales on their ridges, rather than making
    // every scale as tall as the furrow. Both lengths still come from the row.
    let spacing = clamp(plate_scale, ridge_scale * 1.5, ridge_scale * 2.0);
    let arc = circle * radius / ridge_scale;
    let character = bark_noise(dot(arc, vec2(0.13, -0.17)) + along / (spacing * 9.0));
    let run = spacing * mix(3.5, 8.0, elongated);
    let wander = vec2(
        bark_noise2(vec2(dot(arc, vec2(0.71, 0.53)), along / run)),
        bark_noise2(vec2(dot(arc, vec2(-0.61, 0.43)), along / (run * 0.83))));
    let ragged = vec2(
        bark_noise2(vec2(dot(arc, vec2(1.71, 1.53)), along / (spacing * 0.23))),
        bark_noise2(vec2(dot(arc, vec2(-1.61, 1.43)), along / (spacing * 0.19))));
    let column = bark_column(arc + 0.9 * (wander - vec2(0.5))
        + mix(0.32, 0.08, elongated) * (ragged - vec2(0.5)));
    let width = mix(0.045, 0.14, elongated) * mix(0.7, 1.3, character);
    let shoulder = smoothstep(width, width + mix(0.10, 0.20, elongated), column.x);
    let cross_wander = 0.55 * bark_noise2(vec2(dot(arc, vec2(2.13, -1.79)),
        along / (spacing * 1.7)))
        + 0.16 * bark_noise(dot(arc, vec2(5.3, 4.7)));
    let scale = bark_scale(along / spacing + cross_wander * mix(1.0, 0.3, elongated), column.y * 91.0);
    let plate = select(vec2(1.0), scale, plate_scale > 0.0);
    // Broad furrows survive successive plates; some shallow columns almost
    // close between breaks. Longer plate ratios reach deep persistent furrows.
    let depth = mix(0.095, 0.20, elongated) * mix(0.55, 1.35, character);
    let broken = mix(0.15, 1.0, bark_noise(along / (spacing * 2.1) + column.y * 73.0));
    let relief = shoulder * (depth * mix(broken, 1.0, elongated) + 0.05 * plate.x);
    // Finer flakes live only on the flat faces, never across a furrow or lip.
    let fine = bark_scale(along / (spacing * 0.19)
        + bark_noise(dot(arc, vec2(4.7, -3.1))), column.y * 173.0);
    let fine_filter = 1.0 - smoothstep(0.18, 0.7, footprint / (ridge_scale * 0.19));
    let flake = 0.012 * fine.x * plate.y * shoulder * shoulder * fine_filter;
    return ridge_scale * maturity * girth * (relief + flake);
}

// Camera-independent physical sampling for field contracts.
fn bark_field(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32) -> f32 {
    return bark_field_filtered(circle, along, radius, ridge_scale, plate_scale, 0.0);
}
