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
    // Declared and unread: surface detail is drawn along the element's own
    // coordinate in the spec that follows this one.
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
    out.leaf = vec3<f32>(
        mix(u.leaf_variation.x, u.leaf_variation.y, offsets.x),
        1.0 + mix(u.leaf_variation.z, u.leaf_variation.w, offsets.y),
        depth_in_crown(placement[3].xyz),
    );
    return out;
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
    let colour = clamp(hue_shift(face, in.leaf.x) * in.leaf.y, vec3<f32>(0.0), vec3<f32>(1.0));
    // Deep in the crown there is no sky to see: thousands of leaves stand
    // between this one and it, and what is left reads as a shaded mass rather
    // than as speckle. The sun is not attenuated - what reaches through is
    // the dapple the shadow map already draws.
    let shaded = ambient(n) * (1.0 - u.leaf_front.w * in.leaf.z);
    return vec4<f32>(tone(colour * (shaded + key(n, in.world))), 1.0);
}
