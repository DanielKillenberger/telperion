//! The lit oak's fine roots on the GPU against the all-CPU run (fn-211
//! task 10; Astra's review of CONTRACT-A.md, section 4, its integration
//! proof): a small lit specimen spanning two scheduled re-lays, every
//! grower's whole state after every phase of every cycle and every
//! cycle's lattice compared word for word.
use super::feedback_stepped::{Device, Taken};
use telperion_space::{
    grow_stepped_on, oak, Expected, Light, PartitionOrder as Order, Request, Species, Stepped,
};

fn grouping(order: Order, chunk: usize) -> telperion_space::Grouping {
    telperion_space::Grouping {
        order,
        chunk,
        sizes: true,
        leaves: true,
        girth: true,
    }
}

fn same(cpu: &Stepped, gpu: &Stepped, case: &str) {
    assert_eq!(gpu.phases, cpu.phases, "{case}: the phases");
    let first = (gpu.states.iter().zip(&cpu.states)).position(|(g, c)| g != c);
    assert_eq!(first, None, "{case}: the first cycle whose state differs");
    assert_eq!(gpu.lattices, cpu.lattices, "{case}: the lattices");
    assert_eq!(
        telperion_space::digest(&gpu.structure),
        telperion_space::digest(&cpu.structure),
        "{case}: the tree"
    );
}

/// The oak, and the oak whose every PA tolerates a balance below 0.99:
/// at 24 the oak's fine roots decide by D16 and never shed, and the
/// second's shed inside them (2,421 thresholds, 52 shed at cut 256).
fn oaks() -> [Species; 2] {
    let mut tolerant = oak();
    for s in &mut tolerant.states {
        s.tolerance = 0.99;
    }
    [oak(), tolerant]
}

#[test]
fn the_lit_fine_roots_on_the_device_are_the_cpus() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let request = Request {
        age: 24,
        seed: 7,
        budget: 20_000_000,
        light: Light {
            extinction: 0.5,
            sky: 0.5,
        },
    };
    let all = usize::MAX;
    let cases = [
        (0, 256.0, (Order::Forward, 1), (all, Taken::Forward, false)),
        (
            0,
            64.0,
            (Order::Shuffled(3), 64),
            (4_096, Taken::Scrambled, true),
        ),
        (1, 256.0, (Order::Reverse, 7), (777, Taken::Reverse, true)),
        (
            1,
            1_024.0,
            (Order::Forward, 7),
            (all, Taken::Forward, false),
        ),
    ];
    let oaks = oaks();
    let (mut retried, mut shed) = (0, 0);
    for (k, cut, (order, chunk), taking) in cases {
        let species = &oaks[k];
        let expected = Expected::new(species, request.age, 0);
        let at = (&expected, cut, grouping(order, chunk));
        let cpu = grow_stepped_on(species, request, at, None).expect("the CPU's");
        let mut device = Device::new(&gpu, taking);
        let started = std::time::Instant::now();
        let on = grow_stepped_on(species, request, at, Some(&mut device)).expect("the GPU's");
        let case = format!("oak {k}, cut {cut}, {order:?} by {chunk}, {taking:?}");
        println!(
            "{case}: {} roots; {} leaves, {} reads, {} decided, {} shed; \
             {} dispatches, {} retries, {} KB read, {:.1} s",
            on.roots,
            device.leaves,
            device.reads,
            device.decided,
            device.shed,
            device.dispatches,
            device.retries,
            device.read_bytes / 1024,
            started.elapsed().as_secs_f64()
        );
        assert!(
            on.roots > 100 && device.decided > 1_000,
            "{case}: {} roots",
            on.roots
        );
        assert_eq!(on.lattices.len() + 1, request.age as usize);
        same(&cpu, &on, &case);
        (retried, shed) = (retried + device.retries, shed + device.shed);
    }
    assert!(retried > 0, "no attempt overflowed");
    assert!(shed > 0, "no fine root shed inside");
}
