//! fn-223 R5: a short shoot's cluster on the real path, its tip drawn by
//! the CPU above the cut or by the device in a fine root, against the
//! whole tree's on the CPU: the same clusters by their tip's stream key,
//! each of its leaves, at its tip within fn-125's screen and fanned about
//! its heading within a frame's.
use super::super::{io, Generator};
use super::resident::{build, Tools};
use super::roots::kernels;
use super::stations::GW;
use std::collections::HashMap;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;

/// fn-125's screen: metres, and radians for a frame.
const MOST: f64 = 50e-6;
const TURN: f64 = 0.01;
/// The beech's short shoots' age.
const SHORT: usize = 14;

/// A cluster as a segment states it: its leaves, tip, heading and spread.
type Seated = (u32, Vec3, Vec3, f64);

#[test]
fn clusters_match_across_the_seam() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut family = telperion_core::presets::by_identity("european-beech").unwrap();
    (family.age, family.skeleton.seed) = (15.0, 1);
    let species = family.space.species.as_mut().unwrap();
    (
        species.states[SHORT].cluster_leaves,
        species.states[SHORT].cluster_spread,
    ) = (3.0, 0.6);
    assert!(executor::apart(&family).is_ok());
    let x = executor::grow(&family).unwrap().expansion().unwrap();
    let whole = x.stations().unwrap().unwrap();
    let theirs: HashMap<u32, Seated> = (whole.segments.iter())
        .filter_map(|s| Some((s.key, (s.count, s.endpoints[0], s.frame[0], s.cluster?))))
        .collect();
    assert_eq!(theirs.len(), x.tree().clusters.len());
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    let gen = pollster::block_on(make).unwrap();
    let k = kernels(&gpu);
    let built = pollster::block_on(build(&gpu, &gen, &k, &Tools::new(&gpu), &family)).unwrap();
    let mut mine: Vec<(u32, Seated)> = (built.cpu_segments.iter())
        .filter_map(|s| Some((s.key, (s.count, s.endpoints[0], s.frame[0], s.cluster?))))
        .collect();
    let above = mine.len();
    let words = built.device.segments as usize * GW;
    let raw = io::read(&gpu, &built.device.buffer, (words.max(4) * 4) as u64).unwrap();
    let f = |w: u32| f64::from(f32::from_bits(w));
    let v = |g: &[u32]| Vec3::new(f(g[0]), f(g[1]), f(g[2]));
    for g in bytemuck::cast_slice::<u8, u32>(&raw).chunks(GW) {
        if f(g[21]) < 0.0 {
            mine.push((g[27], (g[25], v(&g[0..3]), v(&g[8..11]), f(g[22]))));
        }
    }
    println!(
        "{above} clusters above the cut, {} in fine roots",
        mine.len() - above
    );
    assert!(
        above > 0 && mine.len() > above,
        "the clusters stand on one side"
    );
    assert_eq!(mine.len(), theirs.len(), "a cluster on one path alone");
    let (mut apart, mut turned) = (0.0f64, 0.0f64);
    for (key, (leaves, at, heading, spread)) in mine {
        let (n, tip, h, s) = theirs[&key];
        assert_eq!(leaves, n);
        assert!((spread - s).abs() < 1e-6);
        apart = apart.max(at.distance(tip));
        turned = turned.max(heading.dot(h).clamp(-1.0, 1.0).acos());
    }
    println!("tips apart {apart:.2e} m, headings {turned:.2e} rad");
    assert!(apart <= MOST && turned <= TURN);
    let resident = built.prepared.resident.as_ref().unwrap();
    assert_eq!(resident.count, whole.count, "leaves");
}
