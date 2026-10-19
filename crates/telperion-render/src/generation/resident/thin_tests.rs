//! fn-281: the oak at 80 at the seeds whose CPU conversion drew a head at
//! radius zero above a relay's join. The device runs no `validate_solved`,
//! so this holds its dress to it: no node at radius zero. Its node count
//! is printed beside the CPU conversion's; they differ by a few nodes in
//! 2.3 million at 80 before fn-281 as after it (`.flow/evidence/fn-281-*/
//! FIX.md`), which the young agreement test does not reach. About 6 GB of
//! host memory and a 4 GiB card:
//! `cargo test --profile ci -p telperion-render thin_tests -- --ignored`.
use super::super::cycle::tree::{self, Kernels};
use super::agree_tests::drawn_nodes;
use telperion_core::pipeline::executor;

#[test]
#[ignore = "fn-281: the oak at 80 on the device and the CPU, four seeds"]
fn the_oak_at_80_draws_no_radius_zero_node_on_the_device() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    // Held to the device's budget, as a build is (`build::crowned`).
    let renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = super::super::Generator::new(&renderer).unwrap();
    let _capped = gpu.ledger.capped(gen.device_budget());
    for seed in [3, 7, 30, 39] {
        let mut f = telperion_core::presets::by_identity("oregon-white-oak").unwrap();
        (f.age, f.skeleton.seed) = (80.0, seed);
        let growth = executor::growth(&f).unwrap();
        let (dressed, _) = pollster::block_on(tree::drawn(&gpu, &k, (&growth, 0))).unwrap();
        let nodes = drawn_nodes(&gpu, &dressed);
        drop(dressed);
        let zero = nodes.iter().filter(|n| !(n.radii[0] > 0.0)).count();
        let cpu = executor::grow(&f)
            .unwrap()
            .expansion()
            .unwrap()
            .tree()
            .nodes
            .len()
            - 1;
        println!(
            "seed {seed}: device {} nodes, {zero} at radius zero; cpu {cpu}",
            nodes.len()
        );
        assert_eq!(zero, 0, "seed {seed}: nodes at radius zero on the device");
    }
}
