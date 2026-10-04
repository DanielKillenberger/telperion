//! fn-200: branches sag under the load they carry, as a beam bends, and
//! wood that bends onto the ground rests on it. The trunk reaching the
//! ground is still an error.
mod walk;
use std::f64::consts::{FRAC_PI_2, PI};
use telperion_space::{Axis, Error, Species, Structure, Vec3};

fn rise(a: Vec3, b: Vec3) -> f64 {
    let run = b - a;
    (run.z / run.length()).asin()
}

/// The rise of a limb's first and last internodes.
fn ends(limb: &Axis) -> (f64, f64) {
    let p = &limb.phytomers;
    let n = p.len();
    (rise(limb.base, p[0].tip), rise(p[n - 2].tip, p[n - 1].tip))
}

/// Limbs leaving level, turning up towards their tips.
fn level_limbs(sag: f64) -> Species {
    let mut species = walk::species();
    let limb = &mut species.states[1];
    limb.insertion = FRAC_PI_2;
    limb.straightening = 0.0;
    limb.form.tropism = 1.5;
    limb.form.elevation = 0.4;
    limb.form.sag = sag;
    species
}

fn limbs(tree: &Structure) -> impl Iterator<Item = (usize, &Axis)> {
    tree.axes
        .iter()
        .enumerate()
        .filter(|(_, a)| a.pa == 1 && a.phytomers.len() > 3)
}

/// R2: a loaded branch droops at its base more than at its tip, and its
/// tip still turns towards its PA's elevation.
#[test]
fn a_loaded_branch_droops_at_its_base_more_than_at_its_tip() {
    let plain = walk::tree(&level_limbs(0.0), 1).unwrap();
    let sagged = walk::tree(&level_limbs(0.02), 1).unwrap();
    let (mut drooped, mut upturned, mut seen) = (0, 0, 0);
    for (i, limb) in limbs(&plain) {
        let (base, tip) = ends(limb);
        let (base2, tip2) = ends(&sagged.axes[i]);
        seen += 1;
        drooped += usize::from(base - base2 > (tip - tip2).max(0.0) + 0.05);
        upturned += usize::from(tip2 > base2 + 0.1);
    }
    assert!(seen > 3, "{seen} limbs");
    assert!(
        drooped * 4 >= seen * 3,
        "{drooped} of {seen} droop at the base"
    );
    assert!(
        upturned * 4 >= seen * 3,
        "{upturned} of {seen} tips turn up"
    );
}

/// R1: sag only bends: the same axes, phytomers and girth.
#[test]
fn sag_bends_and_grows_no_other_tree() {
    let plain = walk::tree(&level_limbs(0.0), 1).unwrap();
    let sagged = walk::tree(&level_limbs(0.02), 1).unwrap();
    assert!(plain != sagged, "sag bends the limbs");
    let wood = |t: &Structure| {
        t.axes
            .iter()
            .flat_map(|a| a.phytomers.iter().map(move |p| (a.lineage, p.radius)))
            .collect::<Vec<_>>()
    };
    assert_eq!(wood(&plain), wood(&sagged));
}

/// R3: wood that sags onto the ground rests on it and runs along it.
#[test]
fn sagging_wood_rests_on_the_ground() {
    let tree = walk::tree(&level_limbs(5.0), 1).unwrap();
    let tips: Vec<f64> = tree
        .axes
        .iter()
        .flat_map(|a| a.phytomers.iter().map(|p| p.tip.z))
        .collect();
    let lowest = tips.iter().copied().fold(f64::MAX, f64::min);
    assert!(lowest >= 0.0, "a vertex {lowest} m below the ground");
    let resting = tips.iter().filter(|&&z| z == 0.0).count();
    assert!(resting > 0, "no wood rests on the ground");
}

/// R3: a lateral driven straight down rests on the ground too.
#[test]
fn a_lateral_pointing_down_rests_on_the_ground() {
    let mut species = walk::species();
    species.states[1].insertion = PI;
    species.states[1].straightening = 0.0;
    let tree = walk::tree(&species, 1).unwrap();
    assert!(tree
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .all(|p| p.tip.z >= 0.0));
}

/// R3: the trunk reaching the ground is still refused, and names the
/// axis's PA, its birth and its base.
#[test]
fn a_trunk_into_the_ground_is_an_error() {
    let mut species = walk::species();
    species.states[0].form.tropism = 1.0;
    species.states[0].form.elevation = -FRAC_PI_2;
    match walk::tree(&species, 1) {
        Err(Error::BelowGround {
            axis,
            pa,
            birth,
            base,
            ..
        }) => {
            assert_eq!((axis, pa, birth), (0, 0, 0));
            assert!(base.abs() < 1e-9, "{base}");
        }
        other => panic!("{other:?}"),
    }
}
