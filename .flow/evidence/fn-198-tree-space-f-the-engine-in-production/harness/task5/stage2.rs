//! fn-198 task 5 scratch: stage 2 alone (`executor::grow`), one tree, best of N.
use telperion_core::{pipeline::executor, presets::Preset};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let preset = match a[0].as_str() {
        "beech" => Preset::EuropeanBeech,
        "spruce" => Preset::NorwaySpruce,
        "oak" => Preset::OregonWhiteOak,
        _ => Preset::DatePalm,
    };
    let mut family = preset.parameters();
    family.age = a[1].parse().unwrap();
    let runs: usize = a.get(2).map_or(3, |r| r.parse().unwrap());
    let mut best = f64::MAX;
    for _ in 0..runs {
        let t = std::time::Instant::now();
        let g = executor::grow(&family).unwrap();
        best = best.min(t.elapsed().as_secs_f64() * 1e3);
        drop(g);
    }
    println!("{} {} stage 2 best {best:.0} ms", a[0], a[1]);
}
