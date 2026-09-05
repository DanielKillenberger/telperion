use telperion_core::envelope::{distance_to_profile, Envelope};
#[test]
fn nearest_profile_distance_preserves_scale_and_projection() {
    let profile = [[0., 0.], [0., 1.]];
    for r in [0., 1e-200, 1e-160, 1e-100, 0.25, 2., 1e100, 1e160, 1e200] {
        assert_eq!(distance_to_profile(&profile, r, 0.5), r);
    }
    let profile = Envelope::default().profile();
    for (r, y) in [(0., -1.), (2., 12.), (10., 25.), (0., 24.), (7., 8.)] {
        let expected = profile
            .windows(2)
            .map(|p| {
                let dx = p[1][0] - p[0][0];
                let dy = p[1][1] - p[0][1];
                let t =
                    (((r - p[0][0]) * dx + (y - p[0][1]) * dy) / (dx * dx + dy * dy)).clamp(0., 1.);
                (r - p[0][0] - t * dx).hypot(y - p[0][1] - t * dy)
            })
            .fold(f64::INFINITY, f64::min);
        assert!((distance_to_profile(&profile, r, y) - expected).abs() <= expected * 1e-15 + 1e-15);
    }
    assert_eq!(distance_to_profile(&[], 1., 2.), f64::INFINITY);
    assert_eq!(distance_to_profile(&[[1., 2.], [1., 2.]], 4., 6.), 5.);
}
