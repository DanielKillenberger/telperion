// The crown: the material row's two leaf faces, each leaf turned a little off
// the row's own colour by its seeded offset, darkened by how deep into the mass
// it stands, under the same sun and the same sky as the wood. The placement is
// not fed in per instance: this draw is one level of the element, and its
// instances are the leaves selection put in that level's list, so each one
// looks its own placement - and its own identity - up through the list.

/// The crown's placements, as the core packed them, and the list of the ones
/// this draw's level was given. The list is bound at the level's own offset.
@group(1) @binding(0) var<storage, read> placements: array<mat4x4<f32>>;
@group(1) @binding(1) var<storage, read> list: array<u32>;

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
    // One seed and crown depth per placement. Carry the seed once for both
    // colour variation and mottle, instead of adding another vertex varying.
    @location(2) @interpolate(flat) leaf: vec3<f32>,
    @location(3) coord: vec2<f32>,
};

@vertex
fn vertex(
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) coord: vec2<f32>,
) -> Varying {
    let id = list[instance];
    let placement = placements[id];
    let world = placement * vec4<f32>(position, 1.0);
    let offsets = vary(id);
    var out: Varying;
    out.clip = u.view_projection * world;
    out.normal = (placement * vec4<f32>(normal, 0.0)).xyz;
    out.world = world.xyz;
    // Restore the side before interpolation, then fold in the fragment.
    // Shared coarse triangles can cross the midrib without erasing it.
    out.coord = vec2<f32>(coord.x, coord.y * sign(position.x));
    out.leaf = vec3<f32>(offsets, depth_in_crown(placement[3].xyz));
    return out;
}

// Vein widths are in leaf coordinates; a pixel footprint filters the lines
// on every level, without a level-dependent phase or per-triangle randomness.
fn vein_tone(coord: vec2<f32>) -> f32 {
    let across = abs(coord.y);
    let width = max(fwidth(coord.y), 0.001);
    let midrib = 1.0 - smoothstep(0.015, 0.015 + width, across);
    let phase = coord.x * u.leaf_detail.x - 0.65 * across;
    let pair_distance = abs(fract(phase + 0.5) - 0.5);
    let pair_width = max(fwidth(phase), 0.001);
    let secondary = (1.0 - smoothstep(0.018, 0.018 + pair_width, pair_distance))
        * (1.0 - smoothstep(0.65, 0.95, across))
        * (1.0 - smoothstep(0.25, 0.75, pair_width));
    let margin = smoothstep(0.86 - width, 1.0, across);
    let vein = max(midrib, secondary * 0.6);
    return 1.0 + u.leaf_detail.y * (1.0 - u.leaf_detail.z)
        * (0.8 * vein - 0.25 * margin);
}

@fragment
fn fragment(in: Varying, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // A leaf has no back. Nothing is culled, so the face the eye sees takes the
    // light; without the flip half the crown would read as holes.
    let n = normalize(select(-in.normal, in.normal, front));
    if (is_clay()) {
        return vec4<f32>(u.clay.rgb * clay_light(n), 1.0);
    }
    // A leaf is paler underneath, and the eye is shown whichever face it is
    // looking at; the seeded offset is the leaf's own and applies to both.
    let face = select(u.leaf_back.rgb, u.leaf_front.rgb, front);
    let scale = u.leaf_colour_detail.x;
    let pixel = fwidth(in.coord);
    let veins = vein_tone(in.coord);
    var modulation = 1.0;
    if (scale > 0.0 && u.leaf_colour_detail.y > 0.0) {
        let mottle = bark_noise2_filtered(in.coord * scale + in.leaf.xy * 37.0, pixel * scale);
        modulation += u.leaf_colour_detail.y * (2.0 * mottle - 1.0);
    }
    let edge = max(abs(in.coord.y), in.coord.x);
    let width = max(pixel.x, pixel.y);
    let margin = smoothstep(1.0 - u.margin.w - width, 1.0 + width, edge)
        * select(0.0, 1.0, u.margin.w > 0.0);
    let hue = mix(u.leaf_variation.x, u.leaf_variation.y, in.leaf.x);
    let brightness = 1.0 + mix(u.leaf_variation.z, u.leaf_variation.w, in.leaf.y);
    let colour = clamp(hue_shift(face, hue) * brightness
        * veins * modulation + margin * u.margin.rgb,
        vec3<f32>(0.0), vec3<f32>(1.0));
    // Deep in the crown there is no sky to see: thousands of leaves stand
    // between this one and it, and what is left reads as a shaded mass rather
    // than as speckle. The sun is not attenuated - what reaches through is
    // the dapple the shared normal-offset comparison kernel reads.
    let shaded = occluded_ambient(n, in.leaf.z) * (1.0 - u.leaf_front.w * in.leaf.z);
    // One comparison result gates reflection and transmission alike. The
    // original face normal still offsets the receiver, as before this term.
    let visibility = sunlight(in.world, n);
    let direct = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * visibility;
    let through = u.sun.rgb * transmitted(n, normalize(u.eye.xyz - in.world),
        u.sun_direction.xyz, u.transmission.rgb, u.transmission.w,
        u.leaf_detail.w, visibility);
    let gloss = u.leaf_colour_detail.z;
    var cuticle = 0.0;
    // A fully shadowed or backlit face has no reflected sun to glint.
    if (front && gloss > 0.0 && any(direct > vec3<f32>(0.0))) {
        let half_way = normalize(normalize(u.eye.xyz - in.world) + u.sun_direction.xyz);
        cuticle = gloss * pow(max(dot(n, half_way), 0.0), exp2(3.0 + 5.0 * gloss));
    }
    return vec4<f32>(tone(colour * (shaded + direct) + through + direct * cuticle), 1.0);
}
