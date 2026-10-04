//! fn-200: branches sag under the load they carry, as a beam bends, and
//! wood that bends onto the ground rests on it. The trunk reaching the
//! ground is still an error.
mod walk;
use std::f64::consts::{FRAC_PI_2, PI};
use telperion_space::{Axis, Error, Origin, Species, Structure, Vec3};

fn rise(a: Vec3, b: Vec3) -> f64 {
    let run = b - a;
    (run.z / run.length()).asin()
}

/// Limbs leaving level, turning up towards their tips.
fn level_limbs(sag: f64) -> Species {
    let mut species = walk::species();
    let limb = &mut species.states[1];
    // Plain monopodial limbs: no stop, relay or change of PA, so every
    // limb is one axis from the trunk to its tip.
    limb.insertion = FRAC_PI_2;
    limb.straightening = 0.0;
    limb.abortion = 0.0;
    limb.relay = 0.0;
    limb.viability = 1.0;
    limb.next = None;
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

/// Each internode's rise along a limb, base first.
fn rises(limb: &Axis) -> Vec<f64> {
    let mut from = limb.base;
    limb.phytomers
        .iter()
        .map(|p| {
            let r = rise(from, p.tip);
            from = p.tip;
            r
        })
        .collect()
}

/// R2: a loaded branch bends down at its base more than at its tip, and
/// its tip still turns towards its PA's elevation: the extra downward turn
/// sag puts at the first joint exceeds that at the last, and the sagged
/// limb ends rising more steeply than it starts.
#[test]
fn a_loaded_branch_droops_at_its_base_more_than_at_its_tip() {
    let plain = walk::tree(&level_limbs(0.0), 1).unwrap();
    let sagged = walk::tree(&level_limbs(1e-4), 1).unwrap();
    // A free tip: nothing borne at the limb's last node.
    let free = |i: usize| {
        let last = plain.axes[i].phytomers.len() - 1;
        !plain.axes.iter().any(|a| {
            matches!(a.origin, Origin::Lateral { parent, node, .. } if parent == i && node == last)
        })
    };
    let (mut drooped, mut upturned, mut seen) = (0, 0, 0);
    for (i, limb) in limbs(&plain).filter(|&(i, _)| free(i)) {
        let (a, b) = (rises(limb), rises(&sagged.axes[i]));
        let n = a.len();
        let bent = |j: usize| (a[j + 1] - a[j]) - (b[j + 1] - b[j]);
        seen += 1;
        drooped += usize::from(bent(0) > bent(n - 2).max(0.0));
        upturned += usize::from(b[n - 1] > b[0]);
    }
    assert!(seen > 3, "{seen} limbs");
    assert!(
        drooped * 4 >= seen * 3,
        "{drooped} of {seen} bend more at the base"
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

/// An unloaded tip bends by nothing: a limb bearing no laterals turns its
/// last internode towards its elevation, however much its base sags.
#[test]
fn an_unloaded_tip_keeps_its_tropism() {
    let mut species = level_limbs(0.02);
    species.states[1].zones[0].lateral = vec![0.0; 3];
    let tree = walk::tree(&species, 1).unwrap();
    let bears = |i: usize| tree.axes.iter().any(|a| a.origin.parent() == Some(i));
    let mut seen = 0;
    for (_, limb) in limbs(&tree).filter(|&(i, _)| !bears(i)) {
        let p = &limb.phytomers;
        let n = p.len();
        let (before, last) = (
            rise(p[n - 3].tip, p[n - 2].tip),
            rise(p[n - 2].tip, p[n - 1].tip),
        );
        if before < 0.4 {
            assert!(
                last > before,
                "the last internode turns down: {before} to {last}"
            );
            seen += 1;
        }
    }
    assert!(seen > 3, "{seen} limbs below their elevation");
}

/// A moment however small bends by degree: no cutoff below which it
/// stops bending.
#[test]
fn a_vanishing_moment_bends_by_degree() {
    let at = |pipe: f64| {
        let mut species = level_limbs(1e-4);
        species.states[1].form.pipe = pipe;
        let tree = walk::tree(&species, 1).unwrap();
        tree.axes
            .iter()
            .flat_map(|a| a.phytomers.iter().map(|p| p.tip))
            .collect::<Vec<_>>()
    };
    let (a, b) = (at(1.000001e-6), at(0.999999e-6));
    let moved = a
        .iter()
        .zip(&b)
        .map(|(p, q)| (*p - *q).length())
        .fold(0.0, f64::max);
    assert!(moved < 1e-3, "{moved} m");
}
