use telperion_core::{
    branching::Specimen, foliage, presets::Preset, specimen::SpecimenView, surface,
};
#[test]
fn specimen_view_scrubs_advances_and_rebuilds_one_chronicle() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 20.0;
    f.skeleton.envelope.height = 4.0;
    f.skeleton.attractors = 40;
    let mut view = SpecimenView::build(&f).unwrap();
    for age in [10.0, 22.0, 5.5, 23.25] {
        view.seek(age).unwrap();
        assert_eq!(
            view.frontier(),
            if age <= 20.0 {
                if age == 5.5 {
                    22.0
                } else {
                    20.0
                }
            } else {
                age
            }
        );
        let mut fresh = f.clone();
        fresh.age = age;
        let read = Specimen::build(&fresh).unwrap().read().unwrap();
        let wood = surface::build(&read.tree, read.surface_height, &fresh.surface).unwrap();
        let element = foliage::build_element(fresh.element).unwrap();
        let placed = foliage::Instances {
            matrices: read.placements.iter().map(|p| p.transform).collect(),
        };
        let instances = foliage::cull(&placed, &element, read.envelope, fresh.shell_depth).unwrap();
        let mesh = view.mesh().unwrap();
        assert_eq!(mesh.wood, wood);
        assert_eq!(mesh.foliage.instances, instances);
    }
    let saved = view.mesh().unwrap();
    assert!(view.seek(-1.0).is_err());
    assert_eq!(view.mesh().unwrap().wood, saved.wood);
}

#[test]
fn scrubbed_view_can_advance_beyond_its_old_history_window() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 12.0;
    f.skeleton.envelope.height = 4.0;
    f.skeleton.attractors = 40;
    let mut view = SpecimenView::build(&f).unwrap();
    view.seek(5.0).unwrap();
    view.seek(10_013.0).unwrap();
    assert_eq!(view.frontier(), 10_013.0);
    view.seek(10_012.0).unwrap();
    f.age = 10_012.0;
    let fresh = SpecimenView::build(&f).unwrap().mesh().unwrap();
    assert_eq!(view.mesh().unwrap().wood, fresh.wood);
    assert_eq!(
        view.mesh().unwrap().foliage.instances,
        fresh.foliage.instances
    );
}
