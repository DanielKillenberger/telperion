// The room: ground and figure, each carrying its own flat value, under the same
// sky and the same sun the subject stands in - and taking the subject's shadow
// where the map throws it across the floor.

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

@group(2) @binding(0) var shadow_map: texture_depth_2d;
@group(2) @binding(1) var shadow_sampler: sampler_comparison;

/// How much of the sun reaches this point: one where it stands open, zero in
/// full shadow, and the four taps of the comparison sampler in between. A point
/// the map does not cover stands open - the map is fitted to the subject and
/// the shadow it throws, never to the whole 400 m floor.
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
    @location(1) colour: vec3<f32>,
    @location(2) world: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) colour: vec3<f32>,
) -> Varying {
    var out: Varying;
    out.clip = u.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.colour = colour;
    out.world = position;
    return out;
}

@fragment
fn fragment(in: Varying) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let hemisphere = mix(u.ground.rgb, u.sky.rgb, 0.5 + 0.5 * n.y);
    let key = u.sun.rgb * max(dot(n, u.sun_direction.xyz), 0.0) * sunlight(in.world);
    return vec4<f32>(in.colour * (hemisphere + key), 1.0);
}
