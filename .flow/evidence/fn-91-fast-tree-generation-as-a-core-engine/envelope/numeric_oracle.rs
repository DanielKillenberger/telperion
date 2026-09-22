use telperion_core::rng::Rng;
fn main() {
    let mut rng = Rng::new(9112);
    for i in 0..8192 {
        let shoulder = match i % 4 { 0 => 1.25, 1 => 8.0, _ => rng.range(1.25, 8.0) };
        let p = match i % 8 {
            0 => (i % 255 + 1) as f64 / 256.0,
            1 => ((i % 255 + 1) as f64 / 256.0).next_down(),
            2 => ((i % 255 + 1) as f64 / 256.0).next_up(),
            3 => (255.0_f64 / 256.0).next_down(),
            4 => 1e-300,
            _ => rng.range(0.0, 255.0 / 256.0),
        };
        let inner = libm::pow(p, shoulder);
        let shape = libm::pow((1.0-inner).max(0.0), 1.0/shoulder);
        println!("{shoulder:.17e},{p:.17e},{inner:.17e},{shape:.17e}");
    }
}
