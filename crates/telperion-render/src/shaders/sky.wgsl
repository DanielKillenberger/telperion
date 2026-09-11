// The sky: one triangle over the whole frame, drawn before anything else and
// writing no depth, so it is what is left wherever no surface stands. The
// gradient runs from the horizon to the zenith along the direction the pixel
// looks, not along the frame, so it holds still when the camera tilts.

struct Sky {
    @builtin(position) clip: vec4<f32>,
    @location(0) ndc: vec2<f32>,
};

@vertex
fn vertex(@builtin(vertex_index) index: u32) -> Sky {
    // One triangle that covers the frame: (-1,-1), (3,-1), (-1,3).
    let corner = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u)) * 2.0 - 1.0;
    var out: Sky;
    out.clip = vec4<f32>(corner, 1.0, 1.0);
    out.ndc = corner;
    return out;
}

@fragment
fn fragment(in: Sky) -> @location(0) vec4<f32> {
    let direction = normalize(
        u.ray_forward.xyz + in.ndc.x * u.ray_right.xyz + in.ndc.y * u.ray_up.xyz,
    );
    // The square root pulls the horizon's colour up off the skyline, which is
    // how a sky reads: most of its change is in the first few degrees.
    let height = sqrt(clamp(direction.y, 0.0, 1.0));
    return vec4<f32>(tone(mix(u.sky_horizon.rgb, u.sky_zenith.rgb, height)), 1.0);
}
