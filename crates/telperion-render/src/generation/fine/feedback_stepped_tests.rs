//! The lit oak's fine roots on the GPU against the all-CPU run (fn-211
//! task 10; Astra's review of CONTRACT-A.md, section 4, its integration
//! proof): a small lit specimen, every grower's whole state after every
//! phase of every cycle and every cycle's lattice compared word for word.
use super::feedback_stepped::{Device, Taken};
use telperion_space::fixed::feedback::lattice::Grid;
use telperion_space::fixed::feedback::shed::{Lateral, Walked};
use telperion_space::fixed::feedback::Fault;
use telperion_space::{
    grow_stepped_on, oak, Expected, Feedback, Laid, LayJob, Light, PartitionOrder as Order,
    Request, Species, Stepped,
};

fn grouping(order: Order, chunk: usize) -> telperion_space::Grouping {
    telperion_space::Grouping {
        order,
        chunk,
        sizes: true,
        leaves: true,
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

/// The oak, and a neighbour that wakes what the oak leaves dormant in
/// its fine roots at 24: every PA tolerates a balance below 0.99, so they
/// shed inside (2,421 thresholds, 52 shed at cut 256); its laterals
/// straighten and erect, so relays carry pulls on; and they turn hard
/// towards 80 degrees below the horizon, so young wood eases onto the
/// ground and rests there; its internodes, insertions and tropisms vary
/// (fn-222, fn-224), so the device's lay reads their draws.
fn oaks() -> [Species; 3] {
    let mut neighbour = oak();
    for s in &mut neighbour.states {
        (s.form.internode_spread, s.form.insertion_spread) = (0.6, 0.3);
        s.form.tropism_spread = 0.8;
        s.tolerance = 0.99;
        s.straightening = 0.4;
        s.erection = 0.3;
        if s.form.elevation < std::f64::consts::FRAC_PI_2 {
            s.form.elevation = -1.4;
            s.form.tropism *= 3.0;
        }
    }
    // Lifespans run to fractions: units a lifespan cuts lay a share of
    // their nodes, each whole (fn-231).
    let mut fractional = oak();
    for s in fractional.states.iter_mut().filter(|s| s.lifespan > 0.0) {
        s.lifespan = s.lifespan * 1.37 + 0.29;
    }
    [oak(), neighbour, fractional]
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
        (2, 256.0, (Order::Forward, 1), (all, Taken::Forward, false)),
        (
            1,
            1_024.0,
            (Order::Forward, 7),
            (all, Taken::Forward, false),
        ),
    ];
    let oaks = oaks();
    let (mut retried, mut shed, mut grounded) = (0, 0, 0);
    for (k, cut, (order, chunk), taking) in cases {
        let species = &oaks[k];
        let expected = Expected::new(species, request.age, 0);
        let at = (&expected, cut, grouping(order, chunk));
        let cpu = grow_stepped_on(species, request, at, None, true).expect("the CPU's");
        let mut device = Device::new(&gpu, taking);
        let started = std::time::Instant::now();
        let on = grow_stepped_on(species, request, at, Some(&mut device), true).expect("the GPU's");
        let case = format!("oak {k}, cut {cut}, {order:?} by {chunk}, {taking:?}");
        println!(
            "{case}: {} roots; {} axes and {} phytomers laid ({} on the ground), \
             {} leaves, {} reads, {} decided, {} shed; {} dispatches, {} retries, \
             {} KB read, {:.1} s",
            on.roots,
            device.axes,
            device.phytomers,
            device.grounded,
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
        grounded += device.grounded;
    }
    assert!(retried > 0, "no attempt overflowed");
    assert!(shed > 0, "no fine root shed inside");
    assert!(grounded > 0, "no fine wood on the ground");
}

/// A device whose every call is timed: layout, D16, deposits, reads.
struct Timed<'d>(Device<'d>, [f64; 4]);

impl Timed<'_> {
    fn time<R>(&mut self, k: usize, f: impl FnOnce(&mut Device) -> R) -> R {
        let started = std::time::Instant::now();
        let r = f(&mut self.0);
        self.1[k] += started.elapsed().as_secs_f64();
        r
    }
}

impl Feedback for Timed<'_> {
    fn lay(&mut self, species: &Species, jobs: &[LayJob]) -> Vec<Laid> {
        self.time(0, |d| d.lay(species, jobs))
    }

    fn shed(&mut self, items: &[(Vec<Lateral>, (bool, u64))]) -> Result<Vec<Vec<Walked>>, Fault> {
        self.time(1, |d| d.shed(items))
    }

    fn deposit(&mut self, grid: &Grid, leaves: &[([i64; 3], i64)]) -> Result<Vec<u128>, Fault> {
        self.time(2, |d| d.deposit(grid, leaves))
    }

    fn sample(&mut self, grid: &Grid, light: &[i64], at: &[[i64; 3]]) -> Vec<i64> {
        self.time(3, |d| d.sample(grid, light, at))
    }
}

/// The lit oak at 80 with its fine roots on the device, timed beside
/// today's whole-tree build and the stepped executor on the CPU, each
/// warm (the second of two); the device's tree checked against the
/// CPU's by every phase's digest and the tree's.
#[test]
#[ignore = "device limits: a timing run: the lit oak at 80, under the GPU lock"]
fn the_lit_oak_at_80_on_the_device_timed() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let species = oak();
    let seed = std::env::var("SEED").map_or(1, |s| s.parse().expect("a seed"));
    let request = Request {
        age: 80,
        seed,
        budget: 20_000_000,
        light: Light {
            extinction: 0.5,
            sky: 0.5,
        },
    };
    let expected = Expected::new(&species, request.age, 0);
    let at = (&expected, 1_024.0, grouping(Order::Forward, 1));
    let warm = |f: &mut dyn FnMut() -> f64| {
        f();
        f()
    };
    let today = warm(&mut || {
        let started = std::time::Instant::now();
        telperion_space::grow(&species, request).expect("today's");
        started.elapsed().as_secs_f64()
    });
    let mut cpu = None;
    let stepped = warm(&mut || {
        let started = std::time::Instant::now();
        cpu = Some(grow_stepped_on(&species, request, at, None, false).expect("the CPU's"));
        started.elapsed().as_secs_f64()
    });
    let mut gpu_tree = None;
    let mut parts = [0.0; 4];
    let mut counts = (0, 0, 0);
    let on = warm(&mut || {
        let mut device = Timed(
            Device::new(&gpu, (usize::MAX, Taken::Forward, false)),
            [0.0; 4],
        );
        let started = std::time::Instant::now();
        let s = grow_stepped_on(&species, request, at, Some(&mut device), false);
        let took = started.elapsed().as_secs_f64();
        gpu_tree = Some(s.expect("the GPU's"));
        parts = device.1;
        counts = (device.0.dispatches, device.0.read_bytes, device.0.phytomers);
        took
    });
    // Once more, each pass's time on the device itself.
    let mut kernels = [0.0; 4];
    if crate::generation::profile::start(&gpu) {
        let mut device = Device::new(&gpu, (usize::MAX, Taken::Forward, false));
        grow_stepped_on(&species, request, at, Some(&mut device), false).expect("the GPU's");
        for (label, start, end) in crate::generation::profile::finish(&gpu) {
            let k = ["lay", "shed", "deposit", "sample"]
                .iter()
                .position(|l| *l == label);
            kernels[k.expect("a pass of this device")] += (end - start) * 1e-9;
        }
    }
    let (cpu, gpu_tree) = (cpu.expect("run"), gpu_tree.expect("run"));
    let settle = |s: &Stepped| s.settle;
    println!(
        "oak lit 80 s{seed}, cut 1,024, {} roots: today {today:.2} s; stepped on the CPU \
         {stepped:.2} s (settle {:.2}); with the device {on:.2} s (settle {:.2}): layout \
         {:.2}, D16 {:.2}, deposits {:.2}, reads {:.2} s, of it on the device {:.3}, {:.3}, \
         {:.3}, {:.3} s; {} dispatches, {} MB read, {} phytomers laid",
        cpu.roots,
        settle(&cpu),
        settle(&gpu_tree),
        parts[0],
        parts[1],
        parts[2],
        parts[3],
        kernels[0],
        kernels[1],
        kernels[2],
        kernels[3],
        counts.0,
        counts.1 >> 20,
        counts.2,
    );
    assert_eq!(gpu_tree.phases, cpu.phases, "the phases");
    assert_eq!(
        telperion_space::digest(&gpu_tree.structure),
        telperion_space::digest(&cpu.structure),
        "the tree"
    );
}
