//! fn-270 R5a at placement: a leaf its wood kept a year before and keeps
//! no longer is drawn at the share left of the year, and one it keeps now
//! whole, whatever it kept a year before.
use super::tests::{expansion, fixture, leaves};
use super::*;
use telperion_core::foliage::{Reference, TwigPlacement};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;
use telperion_core::Family;

/// Each placed leaf's scale on one segment of 64 stations, its retention
/// `(was, now, fade)` where it has one.
fn scales(generator: &Generator, retention: Option<(u32, u32, f32)>) -> Vec<f64> {
    let tree = fixture(0.0);
    let f = Family::default();
    let twig = TwigPlacement { internode_length: 0.07, stations_per_internode: 3 };
    let mut p = expansion(&tree, &f, twig).stations().unwrap().unwrap();
    p.segments.truncate(1);
    p.count = 64;
    let segment = &mut p.segments[0];
    (segment.count, segment.first) = (64, 0);
    segment.retention = retention.map(|(was, now, fade)| (0, was, now, fade));
    let reference = Reference::spanning(Vec3::new(-2.0, -1.0, -2.0), Vec3::new(2.0, 3.0, 2.0));
    let element = executor::element(f.element).unwrap();
    let out = generator.compute(p, &leaves(&tree, &f), twig, &element, reference, &mut Default::default()).unwrap();
    let bytes = io::read(&generator.gpu, out.leaves.buffer(), u64::from(out.count) * 12).unwrap();
    let words: &[u32] = bytemuck::cast_slice(&bytes);
    words.chunks_exact(3).map(|l| reference.scale([l[0], l[1], l[2]])).collect()
}

#[test]
fn a_leaf_its_wood_keeps_no_longer_falls_by_the_year_on_the_device() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = crate::Renderer::new(gpu, crate::STILL_FORMAT);
    let generator = Generator::new(&renderer).unwrap();
    let whole = scales(&generator, None);
    assert!(whole.len() == 64 && whole.iter().all(|&w| w > 0.0));
    let all = u32::MAX;
    for (retention, share) in [((all, 0, 0.5), 0.5), ((all, all, 0.5), 1.0)] {
        let drawn = scales(&generator, Some(retention));
        assert_eq!(drawn.len(), whole.len());
        for (d, w) in drawn.iter().zip(&whole) {
            assert!((d - w * share).abs() <= 2e-3 * w, "{retention:?}: {d} against {w}");
        }
    }
}
