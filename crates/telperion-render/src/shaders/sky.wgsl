// The sky: one triangle over the whole frame, drawn before anything else and
// writing no depth, so it is what is left wherever no surface stands. Its
// colour runs along the direction the pixel looks, not along the frame, so it
// holds still when the camera tilts.

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
    // A slow gradient across a wide sky stair-steps into bands at eight bits,
    // so each pixel is moved by its own hash, about one step from lowest to
    // highest. The pixel is the hash's only input: one frame, one picture.
    let pixel = vec2<u32>(in.clip.xy);
    let offset = vary(pixel.y * 65536u + pixel.x).x - 0.5;
    return vec4<f32>(tone(sky_radiance(direction)) * (1.0 + offset / 96.0), 1.0);
}
