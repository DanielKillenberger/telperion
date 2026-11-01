// The contract's marked arithmetic, kept apart: a product or quotient is
// passed through its bits XORed with a uniform zero the driver cannot see
// through, so it is never fused into the add that takes it. The same
// program on the same input otherwise answers two ways, a rounding step
// apart, from one run to the next (fn-320 FUSED.md).
fn exact_kept(x: f32) -> f32 {
    return bitcast<f32>(bitcast<u32>(x) ^ exact_zero());
}

fn exact_div(a: f32, b: f32) -> f32 {
    return exact_kept(a / b);
}
