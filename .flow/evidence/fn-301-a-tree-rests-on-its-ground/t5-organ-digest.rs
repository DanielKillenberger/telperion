
/// fn-301 .5 scratch: the device's organ words for every flowering preset
/// at ages 12 and its own, seeds 1, 7 and 42, hashed.
#[test]
#[ignore = "a scratch digest"]
fn fn301_organ_digest() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    for id in ["telperion", "laurelin"] {
        let own = Preset::from_id(id).unwrap().parameters().age;
        for age in [12.0, own] {
            for seed in [1u32, 7, 42] {
                let mut family = Preset::from_id(id).unwrap().parameters();
                (family.age, family.skeleton.seed) = (age, seed);
                let prepared = gen.prepare(&family).unwrap();
                let organ = prepared.organ.as_ref().expect("the tree flowers");
                let words = pollster::block_on(io::read_leaves_async(
                    &gpu,
                    organ.leaves.buffer(),
                    organ.count,
                ))
                .unwrap();
                println!("ORGAN {id} {age} {seed}: {} {}", organ.count, debug(&words));
            }
        }
    }
}
