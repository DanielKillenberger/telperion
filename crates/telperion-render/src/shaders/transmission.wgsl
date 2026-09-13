// The thin blade's single-pass transmission, under the same sun as its surface.
fn transmitted(normal: vec3<f32>, to_eye: vec3<f32>, to_sun: vec3<f32>,
    tint: vec3<f32>, strength: f32, thickness: f32, visibility: f32) -> vec3<f32> {
    let through_face = max(dot(-normal, to_sun), 0.0);
    let toward_eye = max(dot(to_eye, -to_sun), 0.0);
    return tint * (strength * exp(-thickness) * visibility * through_face
        * toward_eye * toward_eye);
}
