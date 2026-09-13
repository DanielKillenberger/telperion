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
fn bark_scale(along: f32, seed: f32, footprint: f32) -> vec2<f32> {
    let cell = floor(along);
    var result = vec2(0.0);
    // Include neighbours whose edge support crosses the cut. Widening only
    // the selected plate shrinks its face and changes its mean with resolution.
    for (var i = -2; i <= 1; i++) {
        let site = cell + f32(i);
        let lower = site + 0.15 + 0.7 * bark_hash(vec2(site, seed));
        let upper = site + 1.15 + 0.7 * bark_hash(vec2(site + 1.0, seed));
        let t = (along - lower) / (upper - lower);
        let width = max(0.15, min(footprint, 0.5) / (upper - lower));
        let face = smoothstep(0.045 - max(0.045, width), 0.045 + max(0.045, width), t)
            * (1.0 - smoothstep(0.93 - max(0.07, width), 0.93 + max(0.07, width), t));
        let lip = smoothstep(0.0325 - max(0.0325, width), 0.0325 + max(0.0325, width), t)
            * (1.0 - smoothstep(0.1875 - max(0.1125, width), 0.1875 + max(0.1125, width), t));
        let strength = mix(0.65, 1.3, bark_hash(vec2(lower, seed + 29.0)));
        result += vec2(strength * (face + 0.18 * lip), face * (1.0 - lip));
    }
    return result;
}

// A wavelength has no contrast left at two pixels. Noise is centred on its
// mean as it leaves the pass band; this also filters coordinate warps.
fn bark_pass(footprint: f32) -> f32 {
    return 1.0 - smoothstep(0.25, 0.5, footprint);
}

fn bark_noise_filtered(t: f32, footprint: f32) -> f32 {
    return 0.5 + (bark_noise(t) - 0.5) * bark_pass(footprint);
}

fn bark_noise2_filtered(p: vec2<f32>, footprint: vec2<f32>) -> f32 {
    return 0.5 + (bark_noise2(p) - 0.5) * bark_pass(max(footprint.x, footprint.y));
}

fn bark_flakes(arc: vec2<f32>, along: f32, spacing: f32, pixel: vec2<f32>,
    seed: f32, face: f32, shoulder: f32) -> f32 {
    let mean = 0.89 * 0.75 * 0.5;
    let width = pixel.y / 0.19 + pixel.x * 5.64;
    let band = bark_pass(max(pixel.x / 0.19, width));
    if (band <= 0.0) { return 0.012 * mean; }
    let at = along / (spacing * 0.19)
        + bark_noise_filtered(dot(arc, vec2(4.7, -3.1)), pixel.x * 5.64);
    let fine = bark_scale(at, seed * 173.0, width);
    return 0.012 * mix(mean, fine.x * face * shoulder * shoulder, band);
}

fn bark_field_filtered(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32, footprint: vec2<f32>, furrow: f32) -> f32 {
    if (ridge_scale <= 0.0 || radius <= ridge_scale) { return 0.0; }
    let maturity = smoothstep(2.0, 5.0, 2.0 * radius / ridge_scale);
    // Girth continues to strengthen mature wood after the young-run fade.
    let girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale);
    let elongated = smoothstep(2.0, 6.0, plate_scale / ridge_scale);
    // Longer furrows retain short scales on their ridges, rather than making
    // every scale as tall as the furrow. Both lengths still come from the row.
    let spacing = clamp(plate_scale, ridge_scale * 1.5, ridge_scale * 2.0);
    let arc = circle * radius / ridge_scale;
    let pixel = footprint / vec2(ridge_scale, spacing);
    let run = spacing * mix(3.5, 8.0, elongated);
    let column_pixel = max(pixel.x, pixel.y * 0.8);
    let ridge_mean = 0.7 * mix(0.095, 0.20, elongated) * 0.95 * mix(0.575, 1.0, elongated) * furrow;
    // Every shorter band has also vanished here. Avoid evaluating invisible
    // sites, especially when differentiating the height on distant runs.
    if (max(column_pixel, footprint.y / run) >= 0.5) {
        return ridge_scale * maturity * girth * (ridge_mean + 0.05 * 0.7 * 0.89 + 0.012 * 0.89 * 0.75 * 0.5);
    }
    let character = bark_noise_filtered(dot(arc, vec2(0.13, -0.17)) + along / (spacing * 9.0),
        pixel.x * 0.22 + pixel.y / 9.0);
    let wander = vec2(
        bark_noise2_filtered(vec2(dot(arc, vec2(0.71, 0.53)), along / run),
            vec2(pixel.x * 0.89, footprint.y / run)),
        bark_noise2_filtered(vec2(dot(arc, vec2(-0.61, 0.43)), along / (run * 0.83)),
            vec2(pixel.x * 0.75, footprint.y / (run * 0.83))));
    let ragged = vec2(
        bark_noise2_filtered(vec2(dot(arc, vec2(1.71, 1.53)), along / (spacing * 0.23)),
            pixel * vec2(2.3, 1.0 / 0.23)),
        bark_noise2_filtered(vec2(dot(arc, vec2(-1.61, 1.43)), along / (spacing * 0.19)),
            pixel * vec2(2.16, 1.0 / 0.19)));
    let column = bark_column(arc + 0.9 * (wander - vec2(0.5))
        + mix(0.32, 0.08, elongated) * (ragged - vec2(0.5)));
    let width = mix(0.045, 0.14, elongated) * mix(0.7, 1.3, character) * furrow;
    // Both axial wander and foreshortened arc length widen a column outline.
    let edge = mix(0.04, mix(0.28, 0.32, elongated), furrow);
    let shoulder = smoothstep(width, width + max(edge, 2.0 * column_pixel), column.x);
    let cross_wander = 0.55 * bark_noise2_filtered(vec2(dot(arc, vec2(2.13, -1.79)),
        along / (spacing * 1.7)), pixel * vec2(2.79, 1.0 / 1.7))
        + 0.16 * bark_noise_filtered(dot(arc, vec2(5.3, 4.7)), pixel.x * 7.09);
    let plate_pixel = max(pixel.y, pixel.x * mix(1.5, 0.45, elongated));
    let scale_at = along / spacing + cross_wander * mix(1.0, 0.3, elongated);
    let scale = bark_scale(scale_at, column.y * 91.0, plate_pixel);
    let plate = select(vec2(1.0), scale, plate_scale > 0.0);
    // Broad furrows survive successive plates; some shallow columns almost
    // close between breaks. Longer plate ratios reach deep persistent furrows.
    let depth = mix(0.095, 0.20, elongated) * mix(0.55, 1.35, character) * furrow;
    let broken = mix(0.15, 1.0,
        bark_noise_filtered(along / (spacing * 2.1) + column.y * 73.0, max(pixel.y / 2.1, pixel.x)));
    let ridge = mix(ridge_mean, shoulder * depth * mix(broken, 1.0, elongated),
        bark_pass(max(column_pixel, footprint.y / run)));
    let plate_relief = mix(0.7 * 0.89, shoulder * plate.x, bark_pass(max(column_pixel, plate_pixel)));
    // Finer flakes live only on the flat faces, never across a furrow or lip.
    let flake = bark_flakes(arc, along, spacing, pixel, column.y, plate.y, shoulder);
    return ridge_scale * maturity * girth * (ridge + 0.05 * plate_relief + flake);
}

// Camera-independent physical sampling for field contracts.
fn bark_field(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32) -> f32 {
    return bark_field_filtered(circle, along, radius, ridge_scale, plate_scale, vec2(0.0), 1.0);
}
