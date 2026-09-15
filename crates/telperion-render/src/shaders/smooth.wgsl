// Smooth bark, the other half of the catalogue's trunks. A beech or a birch
// has no furrow to shade: what the eye reads on it is colour at two scales,
// lichen patches and lenticel dashes. Both are spots about sites scattered in
// the space the bark passes through - the circle embedding in metres and the
// run along the wood - so, like the plate network, neither has an angular
// seam to hide. A spot's site may stand off the surface, and then the surface
// cuts it small: that is where the spread of sizes comes from.
//
// Both are footprint-faded on both axes. A spot too small to resolve costs no
// hash and returns the mean its field carries, so a distant trunk converges
// on one colour instead of shimmering, and both enter the colour as a mix at
// a weight that does not depend on the height, which keeps the fn-29 rule.

// The soft rim a spot is drawn with, in its own radii.
const SMOOTH_RIM = 0.15;

// A spot's soft disc: one at its site, nought past its rim, the rim widened
// by the footprint the way every band of the bark field is.
fn smooth_disc(distance: f32, pixel: f32) -> f32 {
    return 1.0 - bark_edge(1.0 - SMOOTH_RIM, 1.0 + SMOOTH_RIM, distance, pixel);
}

// Lichen patches are spheres about sites anywhere in their cells, reaching
// past them, so the twenty-seven cells about this point are read. The layer
// is read once a fragment, like the mottle; the relief never reads it.
const LICHEN_REACH = 0.62; // the largest patch's radius, in cells
const LICHEN_SMALL = 0.35; // the smallest patch against the largest
// What one octave averages to is 1 - exp(-rate * share): patches overlap as
// independent covers do. The rate is measured over a sweep of rings of the
// field and pinned by the smooth means test, as the plate means are.
const LICHEN_RATE = 0.31;

fn lichen_octave(p: vec3<f32>, pixel: f32, share: f32) -> f32 {
    let base = floor(p);
    var cover = 0.0;
    for (var z = -1; z <= 1; z++) {
        for (var y = -1; y <= 1; y++) {
            for (var x = -1; x <= 1; x++) {
                let id = base + vec3(f32(x), f32(y), f32(z));
                let a = bark_hash(id.xy + vec2(23.0, 41.0) * id.z + vec2(7.3, 1.9));
                let b = bark_hash(id.xy + vec2(13.0, 31.0) * id.z + vec2(61.7, 17.3));
                let present = smooth_presence(share, fract(a * 91.7 + b * 13.9));
                let site = id + vec3(a, b, fract(a * 43.7 + b * 71.3));
                let radius = LICHEN_REACH * mix(LICHEN_SMALL, 1.0, fract(a * 17.3 + b * 31.1));
                // A patch is a sphere drawn out along its own three axes, so
                // no two cut the bark to the same round disc.
                let axes = mix(vec3(0.8), vec3(1.25),
                    fract(vec3(a * 53.3 + b * 7.1, a * 11.9 + b * 61.7, a * 37.1 + b * 23.9)));
                let distance = length((p - site) * axes) / radius;
                // Most of the twenty-seven cannot reach: the filtered rim is
                // worked out only for a patch whose rim this pixel can touch.
                if (present > 0.0 && distance < 1.0 + SMOOTH_RIM + pixel / radius) {
                    let shade = mix(0.55, 1.0, fract(a * 29.3 + b * 57.1));
                    cover = max(cover, smooth_disc(distance, pixel / radius) * shade * present);
                }
            }
        }
    }
    return cover;
}

fn lichen_mean(share: f32) -> f32 {
    return 1.0 - exp(-LICHEN_RATE * smooth_share(share));
}

// One octave of patches over cells of `scale` metres, faded to its mean as a
// patch drops under two pixels or the wood under it grows too thin to hold
// one across, which `thin` says. The outline is the sphere's own cut: a warp
// to fray it cost the birch's whole tree 0.07 ms and the spots read round.
fn lichen_layer(arc: vec2<f32>, along: f32, footprint: vec2<f32>, scale: f32,
    share: f32, salt: f32, thin: f32) -> f32 {
    let mean = lichen_mean(share);
    let pixel = max(footprint.x, footprint.y) / scale;
    let retained = bark_pass(pixel / LICHEN_REACH) * thin;
    if (retained <= 0.0) { return mean; }
    let p = vec3(arc, along) / scale + salt;
    return mix(mean, lichen_octave(p, pixel, share), retained);
}

// Lichen over this fragment, two octaves: patches at the row's scale and the
// small spots at two fifths of it, combined as two independent covers are.
fn lichen(circle: vec2<f32>, along: f32, radius: f32, footprint: vec2<f32>) -> f32 {
    let scale = u.lichen_detail.x;
    let share = u.lichen_detail.y;
    let arc = circle * radius;
    let thin = smooth_thin(footprint, radius);
    let large = lichen_layer(arc, along, footprint, scale, share, 0.0, thin);
    let small = lichen_layer(arc, along, footprint, 0.4 * scale, share, 17.0, thin);
    return large + small - large * small;
}

// Lenticels are short dashes across the wood. A dash's site stands anywhere
// across the arc of its cell - a site held near its cell's middle would thin
// the dashes wherever the trunk's circle runs along a cell edge - so the nine
// cells about this point across the arc are read. Along the wood a dash stays
// inside its own row of cells, so one row is enough: nine hashes, where a
// full search reads twenty-seven, which matters because the dash also cuts a
// groove into the relief, and the relief is read a dozen times a pixel.
const LENTICEL_REACH = 0.8; // the longest dash's half-length, in cells across
const LENTICEL_ROW = 0.3; // a dash's half-height, at most, in cells along
const LENTICEL_JITTER = 0.15; // how far a site wanders along its row
const LENTICEL_THIN = 0.06; // a dash's half-height against its whole length
const LENTICEL_SMALL = 0.4; // the shortest dash against the longest
const LENTICEL_SHARE = 0.8; // the share of cells holding a dash
// How deep the groove a dash cuts, against the dash's own half-height. The
// groove is a bowl across the whole dash rather than the dash's rim, so its
// walls lean no steeper than the relief's own and the shading integrates.
const LENTICEL_DEPTH = 0.5;
// A bowl fills this much of what the dash's disc fills, over the same cell.
const LENTICEL_BOWL = 0.3;

// The dash over this point, as filtered values in 0..1: the dash as colour
// reads it, and the bowl its groove is cut to. `row` is the lenticel row:
// rows per metre, the longest dash, strength and tint.
fn lenticel_dash(circle: vec2<f32>, along: f32, radius: f32, footprint: vec2<f32>,
    row: vec4<f32>) -> vec2<f32> {
    let half = 0.5 * row.y;
    let thin = LENTICEL_THIN * row.y;
    let across = half / LENTICEL_REACH;
    let pitch = max(1.0 / max(row.x, 1e-3), thin / LENTICEL_ROW);
    // A spheroid of radii half, half and thin, one to a cell of across,
    // across and pitch, fills this much of it before its size and presence;
    // dashes that overlap cover as independent covers do.
    let sizes = (1.0 - pow(LENTICEL_SMALL, 4.0)) / (4.0 * (1.0 - LENTICEL_SMALL));
    let filled = smooth_share(LENTICEL_SHARE) * sizes * 4.18879
        * LENTICEL_REACH * LENTICEL_REACH * (thin / pitch)
        * (1.0 + 0.6 * SMOOTH_RIM * SMOOTH_RIM);
    let mean = vec2(1.0 - exp(-filled), 1.0 - exp(-LENTICEL_BOWL * filled));
    let extent = max(footprint.x / half, footprint.y / thin);
    let retained = bark_pass(0.5 * extent) * smooth_thin(footprint, radius);
    if (retained <= 0.0) { return mean; }
    let p = vec3(circle * radius / across, along / pitch);
    let base = floor(p);
    let tangent = vec2(-circle.y, circle.x);
    var dash = vec2(0.0);
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let id = base + vec3(f32(x), f32(y), 0.0);
            let a = bark_hash(id.xy + vec2(29.0, 13.0) * id.z + vec2(3.1, 47.9));
            let b = bark_hash(id.xy + vec2(17.0, 37.0) * id.z + vec2(71.3, 23.7));
            let site = id + vec3(a, b, 0.5 + 2.0 * LENTICEL_JITTER * (fract(a * 43.7 + b * 71.3) - 0.5));
            let offset = (site - p) * vec3(across, across, pitch);
            let size = mix(LENTICEL_SMALL, 1.0, fract(a * 17.3 + b * 31.1));
            let distance = length(vec3(dot(offset.xy, tangent) / half,
                dot(offset.xy, circle) / half, offset.z / thin)) / size;
            let present = smooth_presence(LENTICEL_SHARE, fract(a * 91.7 + b * 13.9));
            if (present > 0.0 && distance < 1.0 + SMOOTH_RIM + extent / size) {
                let bowl = 1.0 - smoothstep(0.0, 1.0 + SMOOTH_RIM, distance);
                dash = max(dash, vec2(smooth_disc(distance, extent / size), bowl) * present);
            }
        }
    }
    return mix(mean, dash, retained);
}

// The groove a dash cuts into the relief, in metres, so the cavity and the
// shaded normal the field already carries darken it with no term of its own.
fn lenticel_groove(circle: vec2<f32>, along: f32, radius: f32, footprint: vec2<f32>) -> f32 {
    let bowl = lenticel_dash(circle, along, radius, footprint, u.lenticel).y;
    return LENTICEL_DEPTH * LENTICEL_THIN * u.lenticel.y * u.lenticel.z * bowl;
}

// How much of a spot across the wood a pixel can still hold on wood of this
// radius. Once a pixel's arc is half the radius the whole visible side of a
// twig is a few pixels, and a spot across it is averaged by the pixel anyway:
// the spot field returns its mean there and spends no hash on it, which is
// most of the wood in a whole tree.
fn smooth_thin(footprint: vec2<f32>, radius: f32) -> f32 {
    return bark_pass(footprint.x / max(radius, 1e-6));
}

// Smooth bark's colour for this fragment's wood, before the relief tints it:
// the inner bark a peeled strip shows, the lenticel's tint, and lichen over
// both. Each is a mix at a weight the height never enters, so the colour map
// stays affine in the filtered height and the relief's tints ride on top.
fn smooth_colour(albedo: vec3<f32>, cover: vec3<f32>) -> vec3<f32> {
    let peeled = mix(albedo, u.peel.rgb, cover.z);
    let marked = peeled * (1.0 + u.lenticel.w * cover.y);
    return mix(marked, u.lichen.rgb, cover.x);
}
