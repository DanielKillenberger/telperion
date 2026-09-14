//! The acceptance instrument must be able to fail the picture the owner
//! rejected. Two synthetic crops stand for the two answers: a lattice of
//! identical cells with straight mortar between them, which is what "looks
//! like armor plating" means, and a jittered cellular network with cells of
//! several sizes and boundaries that bend, which is what bark is. A score
//! that cannot tell those apart cannot judge a round, so this is the test
//! that guards every number the hill climb was steered by.
use telperion_render::{measure_frame, Structure, CROP};

/// One deterministic value per input, with no crate dependency and no state.
fn hash(x: f64, y: f64) -> f64 {
    let value = (x * 127.1 + y * 311.7).sin() * 43_758.545_312;
    value - value.floor()
}

fn image(shade: impl Fn(f64, f64) -> f64) -> Vec<u8> {
    let mut rgba = vec![255; CROP * CROP * 4];
    for y in 0..CROP {
        for x in 0..CROP {
            let value = (shade(x as f64, y as f64).clamp(0.0, 1.0) * 255.0) as u8;
            for channel in 0..3 {
                rgba[(y * CROP + x) * 4 + channel] = value;
            }
        }
    }
    rgba
}

/// A wall: cells 40 wide and 20 high, every one the same size, the mortar
/// between them straight and on two angles only.
fn brickwork() -> Vec<u8> {
    image(|x, y| {
        let across = (x % 40.0 - 20.0).abs();
        let along = (y % 20.0 - 10.0).abs();
        if across > 17.0 || along > 7.5 {
            0.1
        } else {
            0.9
        }
    })
}

/// Bark: sites on a jittered lattice, each with a size of its own, the whole
/// field warped by a long wave so no boundary runs straight. The furrow is
/// where the two nearest sites are equally far, which is what makes three
/// arms meet at a junction rather than four.
fn network(seed: f64) -> Vec<u8> {
    image(move |x, y| {
        let warp = 6.0 * ((y / 47.0).sin() + (x / 61.0).cos());
        let (px, py) = ((x + warp) / 34.0, (y + warp * 0.7) / 26.0);
        let (mut first, mut second) = (f64::INFINITY, f64::INFINITY);
        for dy in -1..=1 {
            for dx in -1..=1 {
                let cell = (px.floor() + f64::from(dx), py.floor() + f64::from(dy));
                let site = (
                    cell.0 + 0.15 + 0.7 * hash(cell.0, cell.1 + seed),
                    cell.1 + 0.15 + 0.7 * hash(cell.0 + 19.0, cell.1 + seed),
                );
                // Cells of one size are a wall; a size per site is a tree.
                let weight = 0.7 + 0.9 * hash(cell.0 + 7.0, cell.1 + 31.0 + seed);
                let distance = ((site.0 - px).powi(2) + (site.1 - py).powi(2)).sqrt() / weight;
                if distance < first {
                    second = first;
                    first = distance;
                } else if distance < second {
                    second = distance;
                }
            }
        }
        if second - first < 0.12 {
            0.1
        } else {
            0.9
        }
    })
}

fn measured(rgba: &[u8]) -> Structure {
    measure_frame(rgba, CROP, CROP).expect("the crop is the whole image")
}

#[test]
fn the_score_separates_bark_from_brickwork() {
    let wall = measured(&brickwork());
    let bark = measured(&network(0.0));
    let other_bark = measured(&network(5.0));
    eprintln!("brickwork {wall:?}");
    eprintln!("bark {bark:?}");
    // Every structure component sees the difference on its own, which is what
    // keeps the weighted distance from resting on one of them.
    assert!(
        bark.orientation_entropy > wall.orientation_entropy + 0.1,
        "a wall's edges lie on two angles: {} against {}",
        bark.orientation_entropy,
        wall.orientation_entropy
    );
    assert!(
        bark.area_variation > wall.area_variation + 0.15,
        "a wall's cells are all one size: {} against {}",
        bark.area_variation,
        wall.area_variation
    );
    assert!(
        bark.furrow_curvature > wall.furrow_curvature + 0.05,
        "a wall's mortar runs straight: {} against {}",
        bark.furrow_curvature,
        wall.furrow_curvature
    );
    assert!(
        bark.junction_arms < wall.junction_arms - 0.2,
        "a lattice crosses four ways, a network meets three: {} against {}",
        bark.junction_arms,
        wall.junction_arms
    );
    // And the distance the rounds are judged by orders them the same way: two
    // draws of the same network are nearer each other than either is to a wall.
    let between_barks = bark.distance(&other_bark);
    let to_the_wall = bark.distance(&wall);
    eprintln!("bark-to-bark {between_barks:.4}, bark-to-wall {to_the_wall:.4}");
    assert!(
        between_barks * 3.0 < to_the_wall,
        "the score cannot tell a wall from a tree: {between_barks:.4} against {to_the_wall:.4}"
    );
    assert_eq!(bark.distance(&bark), 0.0, "a still differs from itself");
}
