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
    fissure: vec4<f32>, // fissure RGB offsets, strength
    crest: vec4<f32>, // crest RGB offsets, strength
    bark_colour_detail: vec4<f32>, // mottle scale, mottle strength, cavity strength, sky occlusion strength
    leaf_colour_detail: vec4<f32>, // mottle scale, mottle strength, cuticle gloss, reserved
    margin: vec4<f32>, // RGB offsets, width
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
    // Most whole-crown blades are smaller than a noise cell. Their exact
    // filtered value is already the mean; avoid four hashes for that value.
    if (retained <= 0.0) { return 0.5; }
    return 0.5 + (bark_noise2(p) - 0.5) * retained;
}

