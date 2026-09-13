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
    /// This leaf's own colour offsets and its depth into the crown. Every
    /// vertex of one instance carries the same three, so nothing here varies
    /// across a leaf: it is per leaf, worked out once where the leaf stands.
    @location(2) leaf: vec3<f32>,
    @location(3) coord: vec2<f32>,
};

/// How deep this leaf stands inside the crown: one at the centre of the
/// ellipsoid the placements fill, nought at its shell and outside it. The
/// placement's own position is what is measured, so a leaf reads one depth all
/// over and does not change it when its level changes.
fn depth_in_crown(position: vec3<f32>) -> f32 {
    if (u.crown_centre.w < 0.5) {
        return 0.0;
    }
    let offset = (position - u.crown_centre.xyz) / max(u.crown_radii.xyz, vec3<f32>(1e-6));
    return 1.0 - clamp(length(offset), 0.0, 1.0);
}

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
    out.leaf = vec3<f32>(
        mix(u.leaf_variation.x, u.leaf_variation.y, offsets.x),
        1.0 + mix(u.leaf_variation.z, u.leaf_variation.w, offsets.y),
        depth_in_crown(placement[3].xyz),
    );
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
    let colour = clamp(hue_shift(face, in.leaf.x) * in.leaf.y
        * vein_tone(in.coord), vec3<f32>(0.0), vec3<f32>(1.0));
    // Deep in the crown there is no sky to see: thousands of leaves stand
    // between this one and it, and what is left reads as a shaded mass rather
    // than as speckle. The sun is not attenuated - what reaches through is
    // the dapple the shared normal-offset comparison kernel reads.
    let shaded = ambient(n) * (1.0 - u.leaf_front.w * in.leaf.z);
    // One comparison result gates reflection and transmission alike. The
    // original face normal still offsets the receiver, as before this term.
    let visibility = sunlight(in.world, n);
    let direct = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * visibility;
    let through = u.sun.rgb * transmitted(n, normalize(u.eye.xyz - in.world),
        u.sun_direction.xyz, u.transmission.rgb, u.transmission.w,
        u.leaf_detail.w, visibility);
    return vec4<f32>(tone(colour * (shaded + direct) + through), 1.0);
}
