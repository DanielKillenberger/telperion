// What every lit shader is drawn under: the one uniform block, the sun's map
// and the terms all of them share. WGSL has no include, so this prelude is
// concatenated in front of each shader's own stages when the module is built.
//
// Nothing here names a tree. Every appearance value arrives as a number in the
// block below - the material row for the subject, the scene row for the sky,
// the sun and the ground - so a shader has nothing to branch on but the frame.

struct Uniforms {
    view_projection: mat4x4<f32>,
    /// The clay room's own hemisphere, and its one flat value. `clay.w` is 1
    /// when the frame being drawn is that room rather than the outdoors.
    sky: vec4<f32>,
    ground: vec4<f32>,
    clay: vec4<f32>,
    light_view_projection: mat4x4<f32>,
    shadow_filter: vec4<f32>, // radius in texels, inverse map edge
    shadow_offset: vec4<f32>, // normal displacement in metres
    sun: vec4<f32>,
    /// The direction towards the sun, unit.
    sun_direction: vec4<f32>,
    /// Where the eye stands, and the picture's own axes: a ray through the
    /// frame is the forward axis plus the two edges scaled by the pixel's
    /// place in it, which is how the sky knows which way a pixel looks.
    eye: vec4<f32>,
    ray_right: vec4<f32>,
    ray_up: vec4<f32>,
    ray_forward: vec4<f32>,
    /// The scene row: the sky it stands under and the ground it stands on.
    sky_zenith: vec4<f32>,
    sky_horizon: vec4<f32>,
    ground_colour: vec4<f32>,
    /// The material row: bark colour with its roughness in `w`, the leaf's two
    /// faces with the interior darkening amount in the front's `w`, and the
    /// hue and brightness offsets one leaf may take (low, high, low, high).
    bark: vec4<f32>,
    bark_detail: vec4<f32>,
    leaf_detail: vec4<f32>, // vein scale, contrast, roundness, thickness
    transmission: vec4<f32>, // tint, strength
    leaf_front: vec4<f32>,
    leaf_back: vec4<f32>,
    leaf_variation: vec4<f32>,
    /// The ellipsoid the crown's placements fill. `crown_centre.w` is 1 when a
    /// leaf is darkened by how deep in it stands, and 0 where there is no
    /// crown to be deep in - one leaf on its own is not an interior.
    crown_centre: vec4<f32>,
    crown_radii: vec4<f32>,
    /// The box the core quantised every leaf position against, which
    /// `leaf.wgsl` decodes the crown's three-word placements with.
    leaf_box_min: vec4<f32>,
    leaf_box_extent: vec4<f32>,
    fissure: vec4<f32>, // fissure RGB offsets, strength
    crest: vec4<f32>, // crest RGB offsets, strength
    bark_colour_detail: vec4<f32>, // mottle scale, mottle strength, cavity strength, sky occlusion strength
    leaf_colour_detail: vec4<f32>, // mottle scale, mottle strength, cuticle gloss, reserved
    margin: vec4<f32>, // RGB offsets, width
    /// The plate network: how wide one plate is across the run in metres
    /// before girth stretches it, how far it runs along, how far its face
    /// domes and how far its rim lifts off the furrow beside it.
    plate: vec4<f32>,
    /// What a plate keeps of its own, how far a furrow floor is darkened by
    /// its own crest against the sun, and how far the relief is given depth.
    bark_structure: vec4<f32>, // identity, directional occlusion, depth, reserved
    weathering: vec4<f32>, // RGB offsets, strength
    orientation: vec4<f32>, // RGB offsets, strength
    /// Young wood's own colour, and in `w` the radius below which wood takes
    /// it; zero is a row with no young wood.
    shoot: vec4<f32>,
    /// The leaf lit as a mass: the bend of its lighting normal toward the
    /// crown's outward direction, the sun's wrap past the terminator, the
    /// diffuse share of its transmission, and its sheen. All zero is a card.
    canopy: vec4<f32>,
    /// How much of the sky one crown radius of leaves takes from a leaf that
    /// reads it through the mass; the rest is reserved. Zero sees through.
    crown_shade: vec4<f32>,
    /// Smooth bark: a lichen patch's colour and how far it covers the bark,
    /// the size of the cells its patches scatter over and the share holding
    /// one, lenticel dash rows per metre, the longest dash, its strength and
    /// its tint, and the inner bark a peeled strip shows with how far strips
    /// curl away.
    lichen: vec4<f32>, // RGB, strength
    lichen_detail: vec4<f32>, // cell size, coverage, reserved, reserved
    lenticel: vec4<f32>, // rows per metre, length, strength, tint
    peel: vec4<f32>, // RGB, curl
    /// What the bark and the cuticle mirror of the sun at normal incidence,
    /// the foot of each material's one highlight.
    reflectance: vec4<f32>, // bark, leaf, reserved, reserved
    /// The grain below the relief: the bark's cell size in metres and its
    /// strength, the leaf's cells per leaf length and its strength.
    grain: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

@group(2) @binding(0) var shadow_map: texture_depth_2d;
@group(2) @binding(1) var shadow_sampler: sampler_comparison;

/// Whether this frame is the clay room rather than the outdoors. The room is
/// the neutral inspection the look must never be allowed to replace: no sun,
/// no material, no depth term and no tone map.
fn is_clay() -> bool {
    return u.clay.w > 0.5;
}

/// The normal-offset receiver averaged over a square of hardware comparisons.
/// Radius zero keeps one comparison (four taps with linear filtering); an
/// unsupported linear comparison becomes a point tap. Outside-map taps are
/// lit, including where a kernel straddles the fitted map's edge.
fn sunlight(world: vec3<f32>, normal: vec3<f32>) -> f32 {
    let receiver = world + normal * u.shadow_offset.x;
    let position = u.light_view_projection * vec4<f32>(receiver, 1.0);
    let ndc = position.xyz / position.w;
    let uv = vec2<f32>(0.5 + 0.5 * ndc.x, 0.5 - 0.5 * ndc.y);
    if (ndc.z < 0.0 || ndc.z > 1.0) {
        return 1.0;
    }
    let radius = i32(u.shadow_filter.x);
    var sum = 0.0;
    for (var y = -radius; y <= radius; y += 1) {
        for (var x = -radius; x <= radius; x += 1) {
            let tap = uv + vec2<f32>(f32(x), f32(y)) * u.shadow_filter.y;
            if (any(tap < vec2<f32>(0.0)) || any(tap > vec2<f32>(1.0))) {
                sum += 1.0;
            } else {
                sum += textureSampleCompareLevel(shadow_map, shadow_sampler, tap, ndc.z);
            }
        }
    }
    let width = f32(2 * radius + 1);
    return sum / (width * width);
}

/// The clay room's light: one neutral hemisphere and nothing else, so form
/// reads off the surface normal with no key for weak geometry to hide behind.
fn clay_light(n: vec3<f32>) -> vec3<f32> {
    return mix(u.ground.rgb, u.sky.rgb, 0.5 + 0.5 * n.y);
}

/// What a surface of this normal takes from everything that is not the sun:
/// the sky over it, and what the ground throws back under it.
fn ambient(n: vec3<f32>) -> vec3<f32> {
    let sky = 0.5 * (u.sky_zenith.rgb + u.sky_horizon.rgb);
    return mix(u.ground_colour.rgb * sky, sky, 0.5 + 0.5 * n.y);
}

/// How deep a point stands inside the crown: one at the centre of the
/// ellipsoid the placements fill, nought at its shell and outside it. The
/// leaf's placement supplies one depth at every level; wood uses its fragment.
fn depth_in_crown(position: vec3<f32>) -> f32 {
    if (u.crown_centre.w < 0.5) {
        return 0.0;
    }
    let offset = (position - u.crown_centre.xyz) / max(u.crown_radii.xyz, vec3<f32>(1e-6));
    return 1.0 - clamp(length(offset), 0.0, 1.0);
}

// Only the sky hemisphere is hidden by the crown. Keeping ambient() intact
// preserves the original arithmetic exactly for rows with zero occlusion.
fn occluded_ambient(n: vec3<f32>, depth: f32) -> vec3<f32> {
    let sky = 0.5 * (u.sky_zenith.rgb + u.sky_horizon.rgb);
    return ambient(n) - sky * (0.5 + 0.5 * n.y) * u.bark_colour_detail.w * depth;
}

/// The sun on a surface of this normal, shadowed by the map it threw.
fn key(n: vec3<f32>, world: vec3<f32>) -> vec3<f32> {
    return u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * sunlight(world, n);
}

/// A normal tilted by a height field's differences over one pixel, from the
/// screen-space derivatives of the surface: surface-gradient bump mapping,
/// which needs no tangent attribute and displaces no vertex. The determinant
/// handles either orientation of the screen axes. Relief cannot keep its full
/// shading slope at a grazing silhouette, so the tilt is blended out as the
/// surface turns away and large slopes cannot defeat visibility.
fn relief_normal(n: vec3<f32>, world: vec3<f32>, dx: vec3<f32>, dy: vec3<f32>,
    height_x: f32, height_y: f32) -> vec3<f32> {
    let rx = cross(dy, n);
    let ry = cross(n, dx);
    let det = dot(dx, rx);
    let facing = abs(dot(n, normalize(u.eye.xyz - world)));
    let gradient = height_x * rx + height_y * ry;
    let perturbed = normalize(n - gradient * sign(det) / max(abs(det), 1e-10));
    return normalize(mix(n, perturbed, smoothstep(0.0, 0.6, facing)));
}

/// Linear radiance to a value a display can hold: Narkowicz's fit of the ACES
/// curve, which rolls a sun of radiance three into white instead of clipping
/// the whole lit side of the tree to it. The target encodes sRGB on the way
/// out, so nothing here applies a gamma of its own.
///
/// The fit is of the curve with the standard exposure already in it, so the
/// frame is stopped down by that same exposure first; without it a bark of
/// reflectance 0.15 in full sun comes back the colour of sand.
fn tone(colour: vec3<f32>) -> vec3<f32> {
    let c = max(colour, vec3<f32>(0.0)) * 0.6;
    let mapped = (c * (2.51 * c + 0.03)) / (c * (2.43 * c + 0.59) + 0.14);
    return clamp(mapped, vec3<f32>(0.0), vec3<f32>(1.0));
}

/// Two unrelated values in 0..1 from one identity, by integer hash. The
/// identity is the leaf's placement index, which every level of the ladder
/// hands the shader alike, so a leaf keeps its own colour when its level
/// changes and the crown does not shimmer as the camera moves.
fn vary(id: u32) -> vec2<f32> {
    var h = id * 747796405u + 2891336453u;
    h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
    h = h ^ (h >> 22u);
    let second = (h ^ 61u) * 2654435761u;
    return vec2<f32>(f32(h >> 8u), f32(second >> 8u)) / 16777216.0;
}

/// A colour turned about the grey axis by a fraction of the colour circle:
/// Rodrigues' rotation, which needs no trip through HSV and leaves a grey
/// exactly where it was. Negative channels are clamped away by the caller.
fn hue_shift(colour: vec3<f32>, turns: f32) -> vec3<f32> {
    let axis = vec3<f32>(0.5773503);
    let angle = turns * 6.2831855;
    return colour * cos(angle)
        + cross(axis, colour) * sin(angle)
        + axis * dot(axis, colour) * (1.0 - cos(angle));
}

// Filtered value noise shared by wood relief and leaf/wood colour.
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

// A whole wavelength starts fading only below two pixels, reaching its mean
// at one. Profiles are averaged independently of this final band rejection.
fn bark_pass(footprint: f32) -> f32 {
    return 1.0 - smoothstep(0.5, 1.0, footprint);
}

fn bark_noise_filtered(t: f32, footprint: f32) -> f32 {
    // Alternating lattice values have a two-cell wavelength. Filtering the
    // final warped field averages the profile; fading the warp itself before
    // that wavelength is unresolved would move the coarse outlines.
    return 0.5 + (bark_noise(t) - 0.5) * bark_pass(footprint * 0.5);
}

fn bark_noise2_filtered(p: vec2<f32>, footprint: vec2<f32>) -> f32 {
    let retained = bark_pass(max(footprint.x, footprint.y) * 0.5);
    // A fully rejected band returns its exact mean; avoid four hashes
    // when the filtered value is already known.
    if (retained <= 0.0) { return 0.5; }
    return 0.5 + (bark_noise2(p) - 0.5) * retained;
}

// The lattice basis one value of the noise above is weighted by: a
// smoothstep rising over the cell before its point and falling over the
// cell after, and its antiderivative from far below.
fn bark_basis(v: f32) -> f32 {
    if (v <= -1.0 || v >= 1.0) { return 0.0; }
    let f = select(v, 1.0 + v, v < 0.0);
    let s = f * f * (3.0 - 2.0 * f);
    return select(1.0 - s, s, v < 0.0);
}

fn bark_basis_integral(v: f32) -> f32 {
    if (v <= -1.0) { return 0.0; }
    if (v >= 1.0) { return 1.0; }
    if (v < 0.0) {
        let f = 1.0 + v;
        return f * f * f * (1.0 - 0.5 * f);
    }
    return 0.5 + v - v * v * v * (1.0 - 0.5 * v);
}

// The most lattice points the box below spans on one axis: a box of three
// cells, past which the window stops growing and the value is the
// three-cell average about the point, within a third of the noise's
// deviation of the mean it converges to.
const BARK_BOX_REACH = 4;

// The noise above averaged over a box of `width` cells on each axis, with
// its gradient: the box integral of every lattice value's basis, exact, so a
// half-size draw reads the box mean of the four full-size samples it stands
// for and a term linear in this noise agrees across resolution by
// construction, and leaves only by that averaging. The window grows with
// the box, so the cost a pixel pays rises as the pixels fall; a box under a
// thousandth of a cell is the point sample, which is what it averages to.
fn bark_noise2_box(p: vec2<f32>, width: vec2<f32>) -> vec3<f32> {
    let w = clamp(width, vec2(0.0), vec2(f32(BARK_BOX_REACH - 1)));
    if (max(w.x, w.y) < 0.001) {
        let c = floor(p);
        let f = fract(p);
        let a = bark_hash(c);
        let b = bark_hash(c + vec2(1.0, 0.0));
        let d = bark_hash(c + vec2(0.0, 1.0));
        let e = bark_hash(c + vec2(1.0));
        let wx = f.x * f.x * (3.0 - 2.0 * f.x);
        let wy = f.y * f.y * (3.0 - 2.0 * f.y);
        let dwx = 6.0 * f.x * (1.0 - f.x);
        let dwy = 6.0 * f.y * (1.0 - f.y);
        let value = mix(mix(a, b, wx), mix(d, e, wx), wy);
        return vec3(value, mix(b - a, e - d, wy) * dwx, (mix(d, e, wx) - mix(a, b, wx)) * dwy);
    }
    let low = p - 0.5 * w;
    let high = p + 0.5 * w;
    let first = floor(low);
    let count = min(vec2<i32>(ceil(high) - first) + vec2(1), vec2(BARK_BOX_REACH));
    var weight_x: array<f32, BARK_BOX_REACH>;
    var weight_y: array<f32, BARK_BOX_REACH>;
    var slope_x: array<f32, BARK_BOX_REACH>;
    var slope_y: array<f32, BARK_BOX_REACH>;
    for (var i = 0; i < BARK_BOX_REACH; i++) {
        let site = first + f32(i);
        weight_x[i] = bark_basis_integral(high.x - site.x) - bark_basis_integral(low.x - site.x);
        weight_y[i] = bark_basis_integral(high.y - site.y) - bark_basis_integral(low.y - site.y);
        slope_x[i] = bark_basis(high.x - site.x) - bark_basis(low.x - site.x);
        slope_y[i] = bark_basis(high.y - site.y) - bark_basis(low.y - site.y);
    }
    var total = vec3(0.0);
    for (var y = 0; y < count.y; y++) {
        var row = vec2(0.0);
        for (var x = 0; x < count.x; x++) {
            let value = bark_hash(first + vec2(f32(x), f32(y)));
            row += vec2(weight_x[x], slope_x[x]) * value;
        }
        total += vec3(weight_y[y] * row.x, weight_y[y] * row.y, slope_y[y] * row.x);
    }
    let area = max(w.x, 0.001) * max(w.y, 0.001);
    return vec3(total.x / area, total.y / area, total.z / area);
}

// Whether one of a scattered set - a lichen patch, a lenticel, a peeled
// strip - is present, as a ramp on its own hash rather than a step, so a walk
// between two shares fades it in rather than popping it. At a share of
// nought none is present and at one every one is.
fn smooth_presence(share: f32, own: f32) -> f32 {
    return clamp((1.1 * share - own) / 0.1, 0.0, 1.0);
}

// The ramp's mean over every spot: the share, less half the ramp at each end.
fn smooth_share(share: f32) -> f32 {
    let x = 1.1 * share;
    if (x < 0.1) { return 5.0 * x * x; }
    let over = max(x - 1.0, 0.0);
    return x - 0.05 - 5.0 * over * over;
}
