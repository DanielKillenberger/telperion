// The crown: the same flat clay and the same hemisphere as the wood, so a leaf
// and the branch it sits on are read under one light. The placement is not fed
// in per instance any more: this draw is one level of the element, and its
// instances are the leaves selection put in that level's list, so each one
// looks its own placement up through the list.

struct Uniforms {
    view_projection: mat4x4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
    clay: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

/// The crown's placements, as the core packed them, and the list of the ones
/// this draw's level was given. The list is bound at the level's own offset.
@group(1) @binding(0) var<storage, read> placements: array<mat4x4<f32>>;
@group(1) @binding(1) var<storage, read> list: array<u32>;

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vertex(
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
) -> Varying {
    let placement = placements[list[instance]];
    var out: Varying;
    out.clip = u.view_projection * placement * vec4<f32>(position, 1.0);
    out.normal = (placement * vec4<f32>(normal, 0.0)).xyz;
    return out;
}

@fragment
fn fragment(in: Varying, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // A leaf has no back. Nothing is culled, so the face the eye sees takes the
    // light; without the flip half the crown would read as holes.
    let n = normalize(select(-in.normal, in.normal, front));
    let hemisphere = mix(u.ground.rgb, u.sky.rgb, 0.5 + 0.5 * n.y);
    return vec4<f32>(u.clay.rgb * hemisphere, 1.0);
}
