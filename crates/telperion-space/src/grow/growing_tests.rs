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

/// A growing phytomer holds 72 bytes, the tree's 136 (fn-358).
#[test]
fn a_growing_phytomer_holds_what_growth_reads() {
    use std::mem::size_of;
    assert_eq!(size_of::<crate::structure::Growing>(), 72);
    assert_eq!(size_of::<crate::structure::Phytomer>(), 136);
}

/// The unlaid oak, beech and spruce in light, pinned at `8d15b662c`,
/// before the grower held its phytomers narrow; re-pinned by fn-359 where
/// a fade below one nests in another (oak 1, beech 1, spruce 1 and 7).
#[test]
fn a_growing_phytomer_is_read_as_it_was() {
    let trees = [
        (
            "oak",
            oak(),
            20,
            [10364051823450299320, 10572993160139470314],
        ),
        (
            "beech",
            beech(),
            30,
            [2408497720773702210, 11778063828277882196],
        ),
        (
            "spruce",
            spruce(),
            30,
            [10160230377130830586, 13617150108993612047],
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
