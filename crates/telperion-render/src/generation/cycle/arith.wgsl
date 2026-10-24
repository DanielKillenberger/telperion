// The contract's marked arithmetic, plain: a product an add takes and a
// quotient, each the operation WGSL writes, so the driver may fuse or
// round them otherwise than the CPU (fn-256 UNITS.md). The device's cycle
// can build the same marks held to the CPU's order instead, for its
// tests (`cycle/exact.wgsl`).
fn exact_kept(x: f32) -> f32 {
    return x;
}

fn exact_div(a: f32, b: f32) -> f32 {
    return a / b;
}
