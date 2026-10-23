//! The release film's corner map (fn-272 R1b): each species projects onto
//! its dot, and a walk's frames stand a share along the segment between.
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

/// Every shipped species at 40 and 1 projects onto its own dot, and a
/// walk's frame stands a share t of the way between its ends' dots
/// (host, 2026-10-22: the map shows how far along the walk is).
#[test]
fn a_walks_frames_stand_a_share_along_the_segment_between_its_ends() {
    let map = map::Map::of(&shipped(40.0)).unwrap();
    assert!(map.share > 0.0 && map.share <= 1.0, "{}", map.share);
    for (id, f) in shipped(40.0) {
        let (at, dot) = (map.project(&f).unwrap(), map.dot(id).unwrap());
        assert!((at[0] - dot[0]).hypot(at[1] - dot[1]) <= 1e-12, "{id}");
    }
    let ids: Vec<&str> = CATALOGUE.iter().map(|&(_, id, _, _)| id).collect();
    for (i, a) in ids.iter().enumerate() {
        for b in &ids[i + 1..] {
            let (p, q) = (map.dot(a).unwrap(), map.dot(b).unwrap());
            for k in 0..=4 {
                let t = f64::from(k) / 4.0;
                let at = map.along(a, b, t).unwrap();
                let on = [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t];
                assert!((at[0] - on[0]).hypot(at[1] - on[1]) <= 1e-12, "{a} -> {b} at {t}");
            }
        }
    }
    assert!(map.along("no-such", ids[0], 0.5).is_err());
}
