//! Times the public wood build (parallel where admitted) on one grown
//! skeleton: one warm-up, then N timed builds; prints each sample.
use std::time::Instant;
use telperion_core::{pipeline, presets::Preset, surface};
fn main() {
    let id = std::env::args().nth(1).unwrap();
    let n: usize = std::env::args().nth(2).map_or(9, |s| s.parse().unwrap());
    let f = Preset::from_id(&id).unwrap().parameters();
    let tree = pipeline::skeleton(&f).unwrap().tree;
    let h = f.skeleton.envelope.height;
    let _ = surface::build(&tree, h, &f.surface).unwrap();
    let mut ms = Vec::new();
    for _ in 0..n {
        let t = Instant::now();
        let w = surface::build(&tree, h, &f.surface).unwrap();
        ms.push(t.elapsed().as_secs_f64() * 1000.0);
        std::hint::black_box(w);
    }
    println!("{{\"id\":\"{id}\",\"ms\":{ms:?}}}");
}
