use telperion_core::{envelope::Envelope, rng::Rng};
fn main() {
    let points = Envelope::default().sample(64, &mut Rng::new(42)).unwrap();
    let values: Vec<f64> = points.into_iter().flat_map(|p| [p.x, p.y, p.z]).collect();
    println!("{values:?}");
}
