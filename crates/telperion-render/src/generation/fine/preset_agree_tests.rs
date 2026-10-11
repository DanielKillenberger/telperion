//! fn-254 R4 check 4: every shipped preset whose fine roots grow on the
//! GPU, its device-grown wood against the CPU executor's by lineage: the
//! kept axes the same (no wood unmatched, every branch surviving alike),
//! tips and radii within the measured bounds, and the crown's height and
//! spread. The presets the CPU grows whole (`executor::apart`) are listed
//! by name and their checks deferred to fn-256 (R4a).
use super::realized_tests::{settled_against, Apart};
use super::roots::kernels;
use telperion_core::pipeline::executor;
use telperion_core::presets::{by_identity, CATALOGUE};
use telperion_space::{grow_roots, root_axes, Plan};

/// How far the device's wood may stand from the CPU's (m): a tip at most
/// and in RMS, a radius, and the crown's height and spread. Measured over
/// the shipped GPU presets at their own ages and seeds, Telperion's and
/// Laurelin's supernatural overrides applied as production grows them
/// (fn-254, on F4 after fn-248): tips 8.8e-4 at most (telperion) and
/// 1.8e-5 RMS (laurelin), radii 8.5e-7, crowns 2.6e-6; bound at about
/// twice. Relative radius gaps reach 3.9e-2, all on roots under a
/// micrometre thick, so a radius is held in metres.
pub(super) const BOUND: Bound = Bound {
    most: 1.75e-3,
    rms: 3.5e-5,
    radius: 2e-6,
    crown: 1.5e-5,
};

pub(super) struct Bound {
    pub most: f64,
    pub rms: f64,
    pub radius: f64,
    pub crown: f64,
}

/// The presets the CPU grows whole, by name, and why (R4a).
const ON_THE_CPU: [(&str, &str); 3] = [
    ("ordinary", "it reads light"),
    ("oregon-white-oak", "it reads light"),
    ("date-palm", "its leaves stand on no regular twig stations"),
];

#[test]
fn every_gpu_preset_grows_on_the_device_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let mut grown = 0;
    for &(_, id, _, _) in CATALOGUE {
        let family = by_identity(id).unwrap();
        if let Err(why) = executor::apart(&family) {
            let listed = ON_THE_CPU.iter().find(|(name, _)| *name == id);
            assert!(listed.is_some(), "{id} grows on the CPU ({why}) unlisted");
            println!("{id}: on the CPU ({why}); its checks deferred to fn-256");
            continue;
        }
        // The species as production grows it: its supernatural overrides
        // applied (fn-254 review).
        let species = family.applied().space.species.clone().unwrap();
        let at = (family.age.round() as u32, u64::from(family.skeleton.seed));
        let plan = Plan::default();
        let cpu = grow_roots(&species, super::realized_tests::request(at.0, at.1), plan).unwrap();
        let want = root_axes(&cpu);
        let a: Apart = settled_against(&gpu, &k, (&species, at, plan), &want, (&[], 64));
        println!(
            "{id} {at:?}: {} phytomers, tips {:.2e} most {:.2e} RMS, radii {:.2e} rel {:.2e} m, crown {:?} {:.2e} apart",
            a.phytomers,
            a.most,
            a.rms(),
            a.radius,
            a.radius_m,
            a.crown,
            a.crown_apart()
        );
        assert_eq!(a.statuses, 0, "{id}: a chain's status");
        assert!(a.phytomers > 0, "{id}: nothing on the device");
        let b = BOUND;
        assert!(a.most <= b.most && a.rms() <= b.rms, "{id}: tips {a:?}");
        assert!(a.radius_m <= b.radius, "{id}: radii {:.2e} m", a.radius_m);
        assert!(a.crown_apart() <= b.crown, "{id}: crown {:?}", a.crown);
        grown += 1;
    }
    assert_eq!(grown + ON_THE_CPU.len(), CATALOGUE.len());
}
