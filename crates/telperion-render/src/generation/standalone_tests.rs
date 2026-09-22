use super::*;

#[test]
fn standalone_rejects_resident_before_generation_and_delivers_owned_cpu_mesh() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let generator = Generator::for_cpu_output(gpu).unwrap();
    let mut family = Family::default();
    family.canopy.size = f64::NAN;
    let error = generator
        .prepare(&family, Delivery::Resident)
        .err()
        .unwrap();
    assert!(error
        .to_string()
        .contains("standalone generation requires CPU delivery"));
    family.canopy.size = 0.1;
    family.skeleton.attractors = 16;
    family.canopy.short_shoot_spacing = 0.0;
    family.canopy.limb_clumping = 0.0;
    let prepared = generator.prepare(&family, Delivery::Cpu).unwrap();
    assert_eq!(prepared.backend, Backend::Gpu);
    assert!(prepared.cpu_mesh().is_some());
    assert!(generator.wood.is_none());
    assert!(prepared.count() > 0);
    assert_eq!(
        generator.read_instances(&prepared).unwrap().len(),
        prepared.count()
    );
    assert!(prepared.bounds().min.x.is_finite());
}
