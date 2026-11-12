//! What the grower holds of a phytomer is read as it was (fn-358): the
//! rough layout keeps every phytomer where it laid it, its frame, scale,
//! rank, size and grown share, and every later frame, stand, leaf and
//! light reads them. The unlaid tree (`sketch`) carries them out of the
//! grower as the layout left them, so a field held at a lesser width, or
//! dropped, that growth read differently moves its digest.
use super::cycle::tree_digest;
use crate::{beech, oak, spruce, Light, Request};

/// Leaves at every angle under the standard overcast sky.
const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// The unlaid oak, beech and spruce in light, pinned at `8d15b662c`,
/// before the grower held its phytomers narrow.
#[test]
fn a_growing_phytomer_is_read_as_it_was() {
    let trees = [
        (
            "oak",
            oak(),
            20,
            [6125320244909560580, 10572993160139470314],
        ),
        (
            "beech",
            beech(),
            30,
            [14165751050450917155, 11778063828277882196],
        ),
        (
            "spruce",
            spruce(),
            30,
            [17982918076840580537, 14152319693416367426],
        ),
    ];
    for (name, species, age, pins) in trees {
        for (seed, pin) in [1, 7].into_iter().zip(pins) {
            let request = Request {
                age,
                seed,
                light: LIT,
            };
            let rough = super::sketch(&species, request).unwrap();
            assert_eq!(tree_digest(&rough), pin, "{name} seed {seed}");
        }
    }
}
