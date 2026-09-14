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

// The plate network, the field's second primitive. Ridges are parallel: they
// cannot branch, cannot merge, and cannot give one plate an identity of its
// own. A cellular partition does all three. The circumference is partitioned
// by the same circle embedding the columns use, so the network crosses the
// angular wrap without a seam; each column is then cut along its run into
// plates at heights of its own, so no two neighbours break together.
//
// Uneven intervals along one column, staggered by the column's own seed. The
// two nearest sites give the distance to the boundary between them, in the
// same linear cell units the circumferential edge distance carries.
fn bark_run(along: f32, seed: f32) -> vec3<f32> {
    let base = floor(along);
    var first = 16.0;
    var second = 16.0;
    var nearest = 0.0;
    for (var i = -1; i <= 2; i++) {
        let index = base + f32(i);
        let site = index + 0.15 + 0.7 * bark_hash(vec2(index, seed));
        let distance = abs(along - site);
        if (distance < first) {
            second = first;
            first = distance;
            nearest = site;
        } else if (distance < second) {
            second = distance;
        }
    }
    return vec3(0.5 * (second - first), bark_hash(vec2(nearest, seed + 7.0)),
        along - nearest);
}

// Where the network's profile stands on average over a whole plate, measured
// over the field and pinned by the plate test. A trunk too far to resolve a
// plate converges here on both axes and the constant-height shortcut returns
// it, so the near and the far path agree at the boundary. A longer plate
// carries slightly more face than a round one, so one constant cannot be
// exact for every row: these are the midpoint of the shipped two, and the
// test holds both inside two hundredths of it.
const BARK_PLATE_FACE = 0.5715;
const BARK_PLATE_DOME = 0.3054;
const BARK_PLATE_RIM = 0.4387;
// How proud a plate stands of its furrow, as a fraction of its own width,
// and how much of that width the wall between the two takes.
const BARK_PLATE_DEPTH = 0.045;
const BARK_PLATE_WALL = 0.14;

fn bark_plate_mean(dome: f32, edge_lift: f32) -> f32 {
    return BARK_PLATE_FACE + dome * BARK_PLATE_DOME + edge_lift * BARK_PLATE_RIM;
}

// One plate's width across the run, in metres. Girth carries it, so an old
// trunk wears big plates and a young limb small ones off the same row.
fn bark_plate_size(cell_scale: f32, girth: f32) -> f32 {
    return cell_scale * girth;
}

// The network's relief in units of one plate's width, and the plate's own
// identity beside it. Faded to the mean on both axes: a plate too small to
// resolve costs no hash, carries no aliasing, and leaves the mean exactly.
fn bark_plate_field(arc: vec2<f32>, along: f32, ridge_scale: f32, girth: f32,
    footprint: vec2<f32>, wander: vec2<f32>, plate: vec4<f32>,
    identity: f32) -> vec2<f32> {
    let dome = plate.z;
    let edge_lift = plate.w;
    let mean = bark_plate_mean(dome, edge_lift);
    let size = bark_plate_size(plate.x, girth);
    if (size <= 0.0) { return vec2(mean, 0.5); }
    let run = size * (1.0 + max(plate.y, 0.0));
    // The circumferential lattice is the arc in plate widths; the axial one is
    // the run. Both footprints are measured in their own cell units.
    // Two footprints, because there are two bands. The plate itself leaves on
    // the cell it fills, which is a width across and a run along. Its walls
    // are measured in widths in both directions once the elongation below has
    // brought the cross-cut back, so their footprint is a width in both.
    let band = max(footprint.x / size, footprint.y / run);
    let pixel = max(footprint.x, footprint.y) / size;
    // A plate leaves the picture on its own band, converging to a mean the
    // colour range below knows, so nothing steps as a trunk recedes. The wall
    // inside it is integrated by its own edge rather than faded: a band that
    // fades on a footprint two renders disagree about is a band that aliases
    // between them, however exactly its mean is preserved.
    let retained = bark_pass(band);
    if (retained <= 0.0) { return vec2(mean, 0.5); }
    let lattice = arc * ridge_scale / size;
    let ring = bark_column(lattice + 0.5 * (wander - vec2(0.5)));
    // One cut across a whole column is a straight course, and a wall of them
    // is brickwork. Ragging the axial coordinate at the plate's own scale
    // bends each cut as it crosses its column, which is how a plate comes to
    // merge with the one beside it rather than sit in a row with it.
    let ragged = bark_noise2_filtered(vec2(dot(lattice, vec2(0.9, -0.7)),
        along * 1.3 / run), vec2(footprint.x * 1.14 / size, footprint.y * 1.3 / run));
    let cut = bark_run(along / run + 0.8 * (ragged - 0.5), ring.y * 137.0);
    // A furrow runs wherever either boundary is near; a face is the interior
    // both leave alone. That is what makes the network branch and merge. The
    // axial distance is measured in runs and the circumferential one in
    // widths, so the elongation brings the cross-cut back to the same metre:
    // a plate three times as long is not a plate with walls three times wide.
    let edge = min(ring.x, cut.x * (1.0 + max(plate.y, 0.0)));
    // The walls are as wide as the ridges' own shoulders. A furrow cut in a
    // third of that distance is a feature no footprint can integrate, and it
    // aliases on a trunk at four times the hero distance.
    let face = bark_edge(0.012, 0.012 + BARK_PLATE_WALL, edge, pixel);
    let ramp = clamp((edge - 0.012) / (0.012 + 2.0 * BARK_PLATE_WALL), 0.0, 1.0);
    let rim = face * (1.0 - bark_edge(0.012 + BARK_PLATE_WALL,
        0.012 + 2.2 * BARK_PLATE_WALL, edge, pixel));
    // What this plate keeps of its own: how proud it stands, and how far it
    // leans across its own run. A scale lifted at one edge is a plate leaning.
    let own = bark_hash(vec2(ring.y * 53.0, cut.y));
    let jitter = 2.0 * own - 1.0;
    let lean = (2.0 * fract(own * 71.7) - 1.0) * clamp(cut.z, -1.0, 1.0);
    let proud = 1.0 + identity * (0.40 * jitter + 0.75 * lean);
    let relief = face * proud + dome * ramp * face + edge_lift * rim * proud;
    // The identity is one value over a whole plate and nothing between two:
    // point-sampling it at a boundary is the one step in this field a box
    // filter cannot recover. Within a footprint of an edge it is the mean,
    // which is what a pixel straddling two plates actually averages to.
    let inside = clamp(edge / max(pixel, 1e-5), 0.0, 1.0);
    return vec2(mix(mean, relief, retained), mix(0.5, own, inside * retained));
}

// The identity of the plate under this fragment, on its own. Colour takes one
// value for the whole plate rather than one per shading cell, the way the
// low-frequency mottle does; at distance it is the mean and costs no hash.
fn bark_plate_identity(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, footprint: vec2<f32>, plate: vec4<f32>,
    identity: f32) -> f32 {
    if (plate.x <= 0.0 || ridge_scale <= 0.0 || radius <= ridge_scale) { return 0.5; }
    let girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale);
    let arc = circle * radius / ridge_scale;
    return bark_plate_field(arc, along, ridge_scale, girth, footprint, vec2(0.5),
        plate, identity).y;
}

// How deep the network cuts, in the units the rest of the field is written in:
// a plate stands a fixed fraction of its own width proud of its furrow, and a
// width is metres, so the ridge scale converts it.
fn bark_plate_depth(plate: vec4<f32>, ridge_scale: f32, girth: f32) -> f32 {
    return BARK_PLATE_DEPTH * bark_plate_size(plate.x, girth) / max(ridge_scale, 1e-6);
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
    furrow: f32, plate: vec4<f32>) -> vec2<f32> {
    if (ridge_scale <= 0.0 || radius <= ridge_scale) { return vec2(0.0); }
    let maturity = smoothstep(2.0, 5.0, 2.0 * radius / ridge_scale);
    let girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale);
    let elongated = smoothstep(2.0, 6.0, plate_scale / ridge_scale);
    let ridge_mean = bark_ridge_mean(elongated, furrow);
    let face_bands = 0.05 * 0.7 * 0.89 + 0.012 * 0.89 * 0.75 * 0.5;
    // The network sits on top of the ridges, so the untinted face is the mean
    // of both. A plate face rises above it and a furrow floor falls below it,
    // which is what makes the existing crest and fissure tints follow the
    // structure rather than the ridge alone.
    let cut = bark_plate_depth(plate, ridge_scale, girth);
    let mean = ridge_mean + face_bands + cut * bark_plate_mean(plate.z, plate.w);
    let amplitude = bark_depth(elongated) * BARK_DEPTH_RANGE.y * furrow + face_bands
        + cut * bark_plate_mean(plate.z, plate.w);
    return ridge_scale * maturity * girth * vec2(mean, amplitude);
}

fn bark_field_filtered(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32, footprint: vec2<f32>, furrow: f32,
    plate: vec4<f32>, identity: f32) -> f32 {
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
    let wander = vec2(
        bark_noise2_filtered(vec2(dot(arc, vec2(0.71, 0.53)), along / run),
            vec2(pixel.x * 0.89, footprint.y / run)),
        bark_noise2_filtered(vec2(dot(arc, vec2(-0.61, 0.43)), along / (run * 0.83)),
            vec2(pixel.x * 0.75, footprint.y / (run * 0.83))));
    // Plates are the coarsest band the field carries and outlive every other,
    // so the network is evaluated before the fine-scale shortcut below and
    // survives it. The same wander drifts both, because it is one bark.
    let network = bark_plate_field(arc, along, ridge_scale, girth, footprint,
        wander, plate, identity).x;
    let plates = bark_plate_depth(plate, ridge_scale, girth) * network;
    let face_bands = 0.05 * 0.7 * 0.89 + 0.012 * 0.89 * 0.75 * 0.5;
    // Every shorter band has also vanished here. Avoid evaluating invisible
    // sites, especially when differentiating the height on distant runs.
    if (scale_pixel >= 1.0) {
        return ridge_scale * maturity * girth * (ridge_mean + face_bands + plates);
    }
    let character = bark_noise_filtered(dot(arc, vec2(0.13, -0.17)) + along / (spacing * 9.0),
        pixel.x * 0.22 + pixel.y / 9.0);
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
    let scale_plate = select(vec2(1.0), scale, plate_scale > 0.0);
    // Broad furrows survive successive plates; some shallow columns almost
    // close between breaks. Longer plate ratios reach deep persistent furrows.
    let depth = bark_depth(elongated) * mix(BARK_DEPTH_RANGE.x, BARK_DEPTH_RANGE.y, character) * furrow;
    let broken = mix(0.15, 1.0,
        bark_noise_filtered(along / (spacing * 2.1) + column.y * 73.0, max(pixel.y / 2.1, pixel.x)));
    let ridge = mix(ridge_mean, shoulder * depth * mix(broken, 1.0, elongated),
        bark_pass(scale_pixel));
    let plate_relief = mix(0.7 * 0.89, shoulder * scale_plate.x, bark_pass(scale_pixel));
    // Finer flakes live only on the flat faces, never across a furrow or lip.
    let flake = bark_flakes(arc, along, spacing, pixel, column.y, scale_plate.y, shoulder);
    return ridge_scale * maturity * girth * (ridge + 0.05 * plate_relief + flake + plates);
}

// Camera-independent physical sampling for field contracts, with the plate
// network left out: the ridges' own arithmetic, unchanged since fn-26.
fn bark_field(circle: vec2<f32>, along: f32, radius: f32,
    ridge_scale: f32, plate_scale: f32) -> f32 {
    return bark_field_filtered(circle, along, radius, ridge_scale, plate_scale,
        vec2(0.0), 1.0, vec4(0.0), 0.0);
}
