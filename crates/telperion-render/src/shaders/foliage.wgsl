// The crown: the same flat clay, the same sky and the same sun as the wood, so
// a leaf and the branch it sits on are read under one light. The placement is
// not fed in per instance any more: this draw is one level of the element, and
// its instances are the leaves selection put in that level's list, so each one
// looks its own placement up through the list.

struct Uniforms {
    view_projection: mat4x4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
    clay: vec4<f32>,
    light_view_projection: mat4x4<f32>,
    sun: vec4<f32>,
    sun_direction: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

/// The crown's placements, as the core packed them, and the list of the ones
/// this draw's level was given. The list is bound at the level's own offset.
@group(1) @binding(0) var<storage, read> placements: array<mat4x4<f32>>;
@group(1) @binding(1) var<storage, read> list: array<u32>;

@group(2) @binding(0) var shadow_map: texture_depth_2d;
@group(2) @binding(1) var shadow_sampler: sampler_comparison;

/// How much of the sun reaches this point: one where it stands open, zero in
/// full shadow, and the four taps of the comparison sampler in between. A point
/// the map does not cover stands open - the map is fitted to the subject and
/// the shadow it throws, never to the whole floor.
fn sunlight(world: vec3<f32>) -> f32 {
    let position = u.light_view_projection * vec4<f32>(world, 1.0);
    let ndc = position.xyz / position.w;
    let uv = vec2<f32>(0.5 + 0.5 * ndc.x, 0.5 - 0.5 * ndc.y);
    let outside = any(uv < vec2<f32>(0.0)) || any(uv > vec2<f32>(1.0))
        || ndc.z < 0.0 || ndc.z > 1.0;
    if (outside) {
        return 1.0;
    }
    return textureSampleCompareLevel(shadow_map, shadow_sampler, uv, ndc.z);
}

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
};

@vertex
fn vertex(
    @builtin(instance_index) instance: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    // Declared and unread: surface detail is drawn along the element's own
    // coordinate in the spec that follows this one.
    @location(2) coord: vec2<f32>,
) -> Varying {
    let placement = placements[list[instance]];
    let world = placement * vec4<f32>(position, 1.0);
    var out: Varying;
    out.clip = u.view_projection * world;
    out.normal = (placement * vec4<f32>(normal, 0.0)).xyz;
    out.world = world.xyz;
    return out;
}

@fragment
fn fragment(in: Varying, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // A leaf has no back. Nothing is culled, so the face the eye sees takes the
    // light; without the flip half the crown would read as holes.
    let n = normalize(select(-in.normal, in.normal, front));
    let hemisphere = mix(u.ground.rgb, u.sky.rgb, 0.5 + 0.5 * n.y);
    let key = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * sunlight(in.world);
    return vec4<f32>(u.clay.rgb * (hemisphere + key), 1.0);
}
