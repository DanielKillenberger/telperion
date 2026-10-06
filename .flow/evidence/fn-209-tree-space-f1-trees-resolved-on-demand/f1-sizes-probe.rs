use std::time::Instant;
use telperion_space::{beech, expected_counts, oak, palm, spruce, Axis, Phytomer};
fn main() {
    println!("Phytomer {} B, Axis {} B", std::mem::size_of::<Phytomer>(), std::mem::size_of::<Axis>());
    for (name, make) in [("beech", beech as fn() -> _), ("spruce", spruce), ("oak", oak), ("palm", palm)] {
        let s = make();
        let t = Instant::now();
        let table = expected_counts(&s, 80).unwrap();
        let total: f64 = (0..s.states.len()).map(|pa| table.total(pa)).sum();
        println!("{name}: closed form at 80 in {:.1} ms, expected phytomers {total:.0}", t.elapsed().as_secs_f64() * 1e3);
    }
}
