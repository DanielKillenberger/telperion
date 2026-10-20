// The frond curve (`leaflet::rachis`), the one every module that draws a
// frond calls: the point `t` of the way along the rachis that leaves
// `point` on `heading`, `length` long, bent out of that line by its `arch`
// toward `lift`, and the rachis's direction there.
struct Rachis { at: vec3<f32>, run: vec3<f32> }
fn rachis(point: vec3<f32>, heading: vec3<f32>, lift: vec3<f32>, length: f32, arch: f32, t: f32) -> Rachis {
    let at = point + heading * (length * t) + lift * (arch * length * t * t);
    let run = normalize(heading + lift * (2.0 * arch * t));
    return Rachis(at, run);
}
