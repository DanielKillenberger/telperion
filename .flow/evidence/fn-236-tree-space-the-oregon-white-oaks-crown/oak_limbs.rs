//! fn-236 scratch: the oak's major axes by PA at one age and seed.
use telperion_core::presets::values;
use telperion_space::{grow, Origin, Request};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let text = std::fs::read_to_string("crates/telperion-core/presets/oregon-white-oak.values").unwrap();
    let f = values::read(&text).unwrap();
    let sp = f.space.species.clone().unwrap();
    let r = Request { age: a[0].parse().unwrap(), seed: a[1].parse().unwrap(), budget: 20_000_000, light: f.space.light };
    let s = grow(&sp, r).unwrap();
    // per PA among 2..7: axes kept, total phytomers, and those of laterals off the stem
    for pa in [2usize, 3, 4, 6, 7] {
        let ax: Vec<_> = s.axes.iter().filter(|x| x.pa == pa).collect();
        let n: usize = ax.iter().map(|x| x.phytomers.len()).sum();
        let lat = ax.iter().filter(|x| matches!(x.origin, Origin::Lateral{..})).count();
        let rel = ax.iter().filter(|x| matches!(x.origin, Origin::Relay{..})).count();
        let reach = ax.iter().filter_map(|x| x.phytomers.last()).map(|p| p.tip.x.hypot(p.tip.y)).fold(0.0f64, f64::max);
        let live = ax.iter().filter(|x| x.apex_end.is_none()).count();
        println!("pa {pa}: axes {} (lateral {lat}, relay {rel}, live {live}), phytomers {n}, max tip reach {reach:.1} m", ax.len());
    }
    let lat_limbs: Vec<_> = s.axes.iter().filter(|x| x.pa == 6 && matches!(x.origin, Origin::Lateral{..})).collect();
    for x in lat_limbs.iter().take(40) {
        let p = x.phytomers.last().map(|p| (p.tip.x.hypot(p.tip.y), p.tip.z)).unwrap_or((0.0,0.0));
        let parent = if let Origin::Lateral{parent, ..} = x.origin { s.axes[parent].pa } else {99};
        println!("  limb born {} on pa {parent} at z {:.1}: {} phytomers, tip reach {:.1} m z {:.1}", x.birth, x.base.z, x.phytomers.len(), p.0, p.1);
    }
}
