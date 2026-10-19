//! The release film's corner map labels (fn-272 R1b; owner, 2026-10-19:
//! the beech, Laurelin and Telperion stand almost on one point and their
//! names collided): every label clear of every other, a label moved off
//! its dot joined to it, and the dots where the projection puts them.
// The film command uses what a test does not.
#[allow(dead_code)]
#[path = "../examples/film/map.rs"]
mod map;
#[allow(dead_code)]
#[path = "../examples/film/overlay.rs"]
mod overlay;

use overlay::overlap;
use telperion_core::presets::{Family, Preset, CATALOGUE};

#[test]
fn no_label_of_the_shipped_map_overlaps_another() {
    let shipped: Vec<(&str, Family)> = CATALOGUE
        .iter()
        .map(|&(_, id, _, _)| {
            let mut f = Preset::from_id(id).expect("a shipped id").parameters();
            f.age = 40.0;
            (id, f)
        })
        .collect();
    let map = map::Map::of(&shipped).unwrap();
    let names: Vec<(&str, &str)> = CATALOGUE.iter().map(|e| (e.1, e.2)).collect();
    for (w, h) in [(1280, 720), (1920, 1080)] {
        let o = overlay::Overlay::new(&map, &names, w, h);
        assert_eq!(o.labels.len(), shipped.len());
        let [px, py, pw, ph] = o.panel();
        for a in &o.labels {
            let f = a.frame;
            assert!(
                f[0] >= px && f[1] >= py && f[0] + f[2] <= px + pw && f[1] + f[3] <= py + ph,
                "{} leaves the panel",
                a.text
            );
            for row in o.axis_rows() {
                assert!(
                    !overlap(f, row),
                    "{} covers an axis name at {w}x{h}",
                    a.text
                );
            }
        }
        for (i, a) in o.labels.iter().enumerate() {
            for b in &o.labels[i + 1..] {
                assert!(
                    !overlay::overlap(a.frame, b.frame),
                    "{} and {} overlap at {w}x{h}",
                    a.text,
                    b.text
                );
            }
        }
        // The three that stand together: at most one keeps its own place.
        let close = ["European beech", "Laurelin", "Telperion"];
        let kept = o
            .labels
            .iter()
            .filter(|l| close.iter().any(|c| l.text.contains(c)) && !l.moved(f64::from(h / 720)))
            .count();
        assert!(
            kept <= 1,
            "{kept} of the beech, Laurelin and Telperion labels sit at their dots"
        );
    }
}
