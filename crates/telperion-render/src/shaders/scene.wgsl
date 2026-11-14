// The ground and the figure, under the same sky and the same sun the subject
// stands in, taking the subject's shadow where the map throws it across the
// floor. The floor takes the scene row's ground colour, turned toward its dry
// colour in patches, and fades into the horizon far off; the figure keeps the
// value it was built with and is never hazed, because a scale reference is
// not part of the weather. In the clay room both keep their own neutral
// values.

struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    /// The vertex's own value, and in `w` how much of it is the floor: the
    /// ground disc is painted by the scene row, the figure by nothing.
    @location(1) colour: vec4<f32>,
    @location(2) world: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) colour: vec4<f32>,
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
    if (is_clay()) {
        return vec4<f32>(in.colour.rgb * clay_light(n), 1.0);
    }
    let albedo = mix(in.colour.rgb, ground_albedo(in.world.xz), in.colour.a);
    let lit = albedo * (ambient(n) + key(n, in.world));
    // The horizon straight out from the eye over this place. Straight below
    // the eye there is no such direction, and the haze there is nought.
    let across = vec3<f32>(in.world.x - u.eye.x, 0.0, in.world.z - u.eye.z);
    let skyline = sky_radiance(across / max(length(across), 1e-6));
    return vec4<f32>(tone(mix(lit, skyline, floor_haze(in.world) * in.colour.a)), 1.0);
}

/// The ground's colour at a place: the row's ground, turned in broad patches
/// toward its dry colour by the share the row's variation allows. A
/// variation of nought is the flat ground colour.
fn ground_albedo(place: vec2<f32>) -> vec3<f32> {
    let share = smoothstep(0.25, 0.75, ground_field(place));
    return mix(u.ground_colour.rgb, u.ground_dry.rgb, u.ground_dry.w * share);
}

/// How far the floor has faded into the horizon at a place: aerial haze over
/// the row's distance, counted only past the subject's own distance from the
/// eye so the ground at its foot is as clear as the tree, and the disc's last
/// stretch faded whatever the distance so its rim draws no line across the
/// sky. A distance of nought hazes nothing.
fn floor_haze(world: vec3<f32>) -> f32 {
    let distance = u.atmosphere.y;
    let beyond = max(length(world.xz - u.eye.xz) - length(u.eye.xz), 0.0);
    let aerial = 1.0 - exp(-beyond / max(distance, 1e-3));
    let rim = smoothstep(0.6, 0.98, length(world.xz) / u.atmosphere.z);
    return select(0.0, max(aerial, rim), distance > 0.0);
}
