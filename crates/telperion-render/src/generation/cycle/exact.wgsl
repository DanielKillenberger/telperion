// fn-256 task 2: the contract's marked arithmetic held to the CPU's order
// throughout (`Arith::Exact`), built only where a test asks for it: each
// mark the held operation (`held.wgsl`).
fn exact_kept(x: f32) -> f32 {
    return held_kept(x);
}

fn exact_div(a: f32, b: f32) -> f32 {
    return held_div(a, b);
}
