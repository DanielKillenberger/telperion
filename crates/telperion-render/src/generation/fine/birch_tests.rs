//! fn-263 R1: the silver birch at 30, seed 42, grown on both executors
//! stage by stage, naming the first stage and the first axis by lineage
//! where the device's wood stands apart from the CPU's, and by how much.
//! Growth is held by lineage exactly and its presences within
//! `PRESENCES`; girth and the first lay are read from the birch with its
//! sag set to none, whose wood is laid once; the sag from the birch as it
//! ships.
use super::preset_agree_tests::BOUND;
use super::realized_tests::{grown_as_on_the_cpu, request, settled_against, Apart};
use super::roots::kernels;
use telperion_core::presets::by_identity;
use telperion_space::{grow_roots, root_axes, Plan, Species};

const AT: (u32, u64) = (30, 42);

fn birch() -> Species {
    let family = by_identity("silver-birch").unwrap();
    family.applied().space.species.clone().unwrap()
}

/// How far `species` stands apart on the device, settled as production
/// settles it.
fn settled(gpu: &crate::Gpu, k: &super::roots::Kernels, species: &Species) -> Apart {
    let plan = Plan::default();
    let cpu = grow_roots(species, request(AT.0, AT.1), plan).unwrap();
    let want = root_axes(&cpu);
    settled_against(gpu, k, (species, AT, plan), &want, (&[], 64))
}

fn over(a: &Apart) -> bool {
    a.most > BOUND.most || a.rms() > BOUND.rms
}

#[test]
fn the_birch_grows_the_same_wood_on_the_device_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let species = birch();
    // Growth: every root's axes by lineage, their presences and livings.
    let presences = grown_as_on_the_cpu(&gpu, &k, "silver-birch", &species, AT);
    println!("growth: axes by lineage exact, presences {presences:.2e} apart");
    let mut unsagged = species.clone();
    for s in &mut unsagged.states {
        s.form.sag = 0.0;
    }
    let stages = [("girth and lay", &unsagged), ("sag", &species)];
    let mut parted = None;
    for (stage, species) in stages {
        let a = settled(&gpu, &k, species);
        println!(
            "{stage}: {} phytomers, tips {:.2e} most {:.2e} RMS, radii {:.2e} m, first apart {:?}",
            a.phytomers,
            a.most,
            a.rms(),
            a.radius_m,
            a.first
        );
        assert_eq!(a.statuses, 0, "{stage}: a chain's status");
        assert!(a.radius_m <= BOUND.radius, "girth: radii {:.2e} m", a.radius_m);
        if parted.is_none() && over(&a) {
            parted = Some((stage, a));
        }
    }
    if let Some((stage, a)) = parted {
        panic!(
            "the birch parts at {stage}: tips {:.2e} most {:.2e} RMS, first {:?}",
            a.most,
            a.rms(),
            a.first
        );
    }
}
