//! The release film's declared path (fn-272 R1, R1a): the value file the
//! film command reads parses, and lays out R1a's nine segments over 35 s.
// The film command uses what a test does not.
#[allow(dead_code)]
#[path = "../examples/film/path.rs"]
mod path;

#[test]
fn the_release_path_is_r1as_nine_segments_over_35_seconds() {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/film/release.json");
    let path = path::Path::read(&file).unwrap();
    assert_eq!(path.segments.len(), 9);
    let seconds: f64 = path.segments.iter().map(|s| s.seconds + s.hold).sum();
    assert_eq!(seconds, 35.0);
    let frames = path.frames();
    assert_eq!(frames.len(), 35 * path.fps as usize);
    // The shot opens at the first segment's start and every segment ends
    // where its motion does.
    assert_eq!(frames[0].along, 0.0);
    for k in 0..9 {
        assert_eq!(frames.iter().rev().find(|f| f.segment == k).unwrap().along, 1.0);
    }
}
