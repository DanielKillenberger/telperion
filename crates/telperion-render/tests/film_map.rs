//! The release film's corner map (fn-272 R1b): a walk's frames project
//! onto the segment between its ends' dots, its ends onto the dots.
// The film command uses what a test does not.
#[allow(dead_code)]
#[path = "../examples/film/map.rs"]
mod map;

use telperion_core::presets::{Family, Preset, CATALOGUE};

fn shipped(age: f64) -> Vec<(&'static str, Family)> {
    CATALOGUE
        .iter()
        .map(|&(_, id, _, _)| {
            let mut f = Preset::from_id(id).expect("a shipped id").parameters();
            f.age = age;
            (id, f)
        })
        .collect()
}

/// Every walk between two shipped species at 40 (and the film's seedling
/// walk at 1): t = 0 and 1 land on the ends' dots, and each frame between
/// lies on the segment joining them, a share t of the way along.
#[test]
fn a_walks_frames_project_onto_the_segment_between_its_ends() {
    let map = map::Map::of(&shipped(40.0)).unwrap();
    assert!(map.share > 0.0 && map.share <= 1.0, "{}", map.share);
    for age in [1.0, 40.0] {
        let species = shipped(age);
        for (i, (ia, a)) in species.iter().enumerate() {
            for (ib, b) in &species[i + 1..] {
                let (p, q) = (map.dot(ia).unwrap(), map.dot(ib).unwrap());
                let length = (q[0] - p[0]).hypot(q[1] - p[1]);
                for k in 0..=4 {
                    let t = f64::from(k) / 4.0;
                    let at = map.project(&map::walked(a, b, t).unwrap()).unwrap();
                    let on = [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t];
                    let off = (at[0] - on[0]).hypot(at[1] - on[1]);
                    // Whole-count rows round at every frame, by half a count at
                    // most: 0.45 % of a segment at worst on today's species.
                    let tolerance = if k == 0 || k == 4 {
                        1e-12
                    } else {
                        0.01 * length
                    };
                    assert!(
                        off <= tolerance,
                        "{ia} -> {ib} at {age}, t {t}: {off} off the segment of {length}"
                    );
                }
            }
        }
    }
}
