// A circle embedding preserves the metre arc metric and crosses the angular
// wrap continuously. Sites partition only the circumference, into columns;
// their slow axial drift is independent of the shorter scale breaks.
// Integral of smoothstep, including its constant tails.
fn bark_step_integral(t: f32) -> f32 {
    let x = clamp(t, 0.0, 1.0);
    return x * x * x * (1.0 - 0.5 * x) + max(t - 1.0, 0.0);
}

fn bark_edge(low: f32, high: f32, at: f32, footprint: f32) -> f32 {
    let span = high - low;
    // The intrinsic cubic edge has derivative variance span^2/20. A pixel
    // box has variance footprint^2/12; add only the missing variance. This
    // keeps already-resolved profiles exactly as authored, including near.
    let width = sqrt(max(footprint * footprint - 0.6 * span * span, 0.0)) / span;
    if (width < 0.001) { return smoothstep(low, high, at); }
    let t = (at - low) / span;
    return (bark_step_integral(t + 0.5 * width)
        - bark_step_integral(t - 0.5 * width)) / width;
}

// Box response for a sinusoidal band: sinc(pi * footprint). The polynomial
// avoids a sine in the roughness estimate (error <0.003 over [0, 1/2]).
fn bark_box(footprint: f32) -> f32 {
    let x = 3.14159265 * min(footprint, 1.0);
    let x2 = x * x;
    return max(0.0, 1.0 - x2 / 6.0 + x2 * x2 / 120.0 - x2 * x2 * x2 / 5040.0);
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
        let width = 0.15;
        let support = footprint / (upper - lower);
        let face = bark_edge(0.045 - max(0.045, width), 0.045 + max(0.045, width), t, support)
            * (1.0 - bark_edge(0.93 - max(0.07, width), 0.93 + max(0.07, width), t, support));
        let lip = bark_edge(0.0325 - max(0.0325, width), 0.0325 + max(0.0325, width), t, support)
            * (1.0 - bark_edge(0.1875 - max(0.1125, width), 0.1875 + max(0.1125, width), t, support));
        let strength = mix(0.65, 1.3, bark_hash(vec2(lower, seed + 29.0)));
        result += vec2(strength * (face + 0.18 * lip), face * (1.0 - lip));
    }
    return result;
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

// The same nominal depth and integrated face bands anchor colour and the
// constant-height shortcut. Their units become metres only at the end.
const BARK_DEPTH_RANGE = vec2(0.55, 1.35);

fn bark_depth(elongated: f32) -> f32 {
    return mix(0.095, 0.20, elongated);
}

fn bark_ridge_mean(elongated: f32, furrow: f32) -> f32 {
    return 0.7 * bark_depth(elongated) * 0.95 * mix(0.575, 1.0, elongated) * furrow;
}

fn bark_colour_range(radius: f32, ridge_scale: f32, plate_scale: f32,
    furrow: f32) -> vec2<f32> {
    if (ridge_scale <= 0.0 || radius <= ridge_scale) { return vec2(0.0); }
    let maturity = smoothstep(2.0, 5.0, 2.0 * radius / ridge_scale);
    let girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale);
    let elongated = smoothstep(2.0, 6.0, plate_scale / ridge_scale);
    let ridge_mean = bark_ridge_mean(elongated, furrow);
    let face_bands = 0.05 * 0.7 * 0.89 + 0.012 * 0.89 * 0.75 * 0.5;
    let mean = ridge_mean + 0.05 * 0.7 * 0.89 + 0.012 * 0.89 * 0.75 * 0.5;
    let amplitude = bark_depth(elongated) * BARK_DEPTH_RANGE.y * furrow + face_bands;
    return ridge_scale * maturity * girth * vec2(mean, amplitude);
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
    // One scale must leave both directions together, including its furrow.
    let plate_pixel = max(pixel.y, pixel.x * mix(1.5, 0.45, elongated));
    let scale_pixel = max(pixel.x, pixel.y);
    let ridge_mean = bark_ridge_mean(elongated, furrow);
    // Every shorter band has also vanished here. Avoid evaluating invisible
    // sites, especially when differentiating the height on distant runs.
    if (scale_pixel >= 1.0) {
        return bark_colour_range(radius, ridge_scale, plate_scale, furrow).x;
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
    let shoulder = bark_edge(width, width + edge, column.x, column_pixel);
    let cross_wander = 0.55 * bark_noise2_filtered(vec2(dot(arc, vec2(2.13, -1.79)),
        along / (spacing * 1.7)), pixel * vec2(2.79, 1.0 / 1.7))
        + 0.16 * bark_noise_filtered(dot(arc, vec2(5.3, 4.7)), pixel.x * 7.09);
    let scale_at = along / spacing + cross_wander * mix(1.0, 0.3, elongated);
    let scale = bark_scale(scale_at, column.y * 91.0, plate_pixel);
    let plate = select(vec2(1.0), scale, plate_scale > 0.0);
    // Broad furrows survive successive plates; some shallow columns almost
    // close between breaks. Longer plate ratios reach deep persistent furrows.
    let depth = bark_depth(elongated) * mix(BARK_DEPTH_RANGE.x, BARK_DEPTH_RANGE.y, character) * furrow;
    let broken = mix(0.15, 1.0,
        bark_noise_filtered(along / (spacing * 2.1) + column.y * 73.0, max(pixel.y / 2.1, pixel.x)));
    let ridge = mix(ridge_mean, shoulder * depth * mix(broken, 1.0, elongated),
        bark_pass(scale_pixel));
    let plate_relief = mix(0.7 * 0.89, shoulder * plate.x, bark_pass(scale_pixel));
    // Finer flakes live only on the flat faces, never across a furrow or lip.
    let flake = bark_flakes(arc, along, spacing, pixel, column.y, plate.y, shoulder);
    return ridge_scale * maturity * girth * (ridge + 0.05 * plate_relief + flake);
}

// Camera-independent physical sampling for field contracts.
fn bark_field(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32) -> f32 {
    return bark_field_filtered(circle, along, radius, ridge_scale, plate_scale, vec2(0.0), 1.0);
}
