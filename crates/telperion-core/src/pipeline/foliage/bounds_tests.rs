use super::*;
use crate::rng::Rng;

fn exact(instances: &Instances, element: &Element) -> Result<Option<Bounds>> {
    instances.validate()?;
    element.validate()?;
    let mut out: Option<Bounds> = None;
    for m in instances.matrices() {
        for vertex in &element.positions {
            let p = transform_point(&m, *vertex);
            if !p.is_finite() || [p.x, p.y, p.z].iter().any(|v| !(*v as f32).is_finite()) {
                return Err(Error::ResourceLimit("foliage bounds overflow"));
            }
            match &mut out {
                Some(b) => b.include(p),
                None => out = Some(Bounds { min: p, max: p }),
            }
        }
    }
    Ok(out)
}

fn bits(bounds: Option<Bounds>) -> Option<[u64; 6]> {
    bounds.map(|b| [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z].map(f64::to_bits))
}

#[test]
fn exact_bounds_match_randomized_curved_elements_and_packed_frames() {
    let mut rng = Rng::new(91);
    for thin in [false, true] {
        let element = build_element(ElementParams {
            width: if thin { 0.0001 } else { 0.07 },
            cup: 0.8,
            curl: 0.9,
            axial_segments: 32,
            ..ElementParams::default()
        })
        .unwrap();
        for scale in [0., 0.00001, 1., 65000.] {
            let mut instances = Instances::new(Reference::spanning(
                Vec3::new(-10., -10., -10.),
                Vec3::new(10., 10., 10.),
            ));
            for i in 0..256 {
                let axis = Vec3::new(rng.range(-1., 1.), rng.range(-1., 1.), rng.range(-1., 1.))
                    .normalized();
                let angle = rng.range(-1000., 1000.);
                let x = Vec3::X.rotate(axis, angle) * scale;
                let y = Vec3::Y.rotate(axis, angle) * scale;
                let z = Vec3::Z.rotate(axis, angle) * scale;
                let matrix = [
                    x.x,
                    x.y,
                    x.z,
                    0.,
                    y.x,
                    y.y,
                    y.z,
                    0.,
                    z.x,
                    z.y,
                    z.z,
                    0.,
                    rng.range(-10., 10.),
                    rng.range(-10., 10.),
                    rng.range(-10., 10.),
                    1.,
                ]
                .map(|v| v as f32);
                instances.push(&matrix);
                if i % 2 == 0 {
                    instances.leaves.last_mut().unwrap()[2] ^= 0x8000_0000;
                }
            }
            assert_eq!(
                bits(instances.bounds(&element).unwrap()),
                bits(exact(&instances, &element).unwrap())
            );
            instances.leaves.reverse();
            assert_eq!(
                bits(instances.bounds(&element).unwrap()),
                bits(exact(&instances, &element).unwrap())
            );
        }
    }
}

#[test]
fn bounds_keep_empty_signed_zero_and_overflow_contracts() {
    let mut instances = Instances::new(Reference::default());
    instances.leaves.push([0, 0, 0x3c00_0000]);
    for positions in [
        vec![],
        vec![Vec3::new(-0., 0., -0.)],
        vec![Vec3::new(f32::MAX as f64, 0., 0.)],
    ] {
        let element = Element {
            positions,
            ..Element::default()
        };
        for scale in [
            0x3c00_0000,
            0x7bff_0000,
            0x7c00_0000,
            0x7e00_0000,
            0xbc00_0000,
        ] {
            instances.leaves[0][2] = scale;
            match (instances.bounds(&element), exact(&instances, &element)) {
                (Ok(a), Ok(b)) => assert_eq!(bits(a), bits(b)),
                (Err(a), Err(b)) => assert_eq!(a, b),
                pair => panic!("bounds result mismatch: {pair:?}"),
            }
        }
    }
    let e = build_element(ElementParams::default()).unwrap();
    assert_eq!(Instances::default().bounds(&e).unwrap(), None);
    instances.reference.min.x = f64::MAX;
    assert_eq!(instances.bounds(&e), exact(&instances, &e));
}

#[test]
fn transformed_box_encloses_points_with_identical_sum_order() {
    let mut rng = Rng::new(191);
    for exponent in [-120, -20, 0, 20, 120] {
        let scale = 2_f64.powi(exponent);
        for _ in 0..128 {
            let lo = Vec3::new(
                rng.range(-1., 0.) * scale,
                rng.range(-1., 0.) * scale,
                rng.range(-1., 0.) * scale,
            );
            let hi = Vec3::new(
                rng.range(0., 1.) * scale,
                rng.range(0., 1.) * scale,
                rng.range(0., 1.) * scale,
            );
            let local = Bounds { min: lo, max: hi };
            let matrix = std::array::from_fn(|_| rng.range(-2., 2.) as f32);
            let b = local.transformed(&matrix);
            for _ in 0..16 {
                let p = Vec3::new(
                    rng.range(lo.x, hi.x),
                    rng.range(lo.y, hi.y),
                    rng.range(lo.z, hi.z),
                );
                assert!(b.contains(transform_point(&matrix, p)));
            }
        }
    }
}
