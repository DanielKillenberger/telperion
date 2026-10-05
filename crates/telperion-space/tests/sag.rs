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

/// Heavily sagging limbs that reach the ground.
fn grounded() -> Species {
    level_limbs(5.0)
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
    let tree = walk::tree(&grounded(), 1).unwrap();
    let tips: Vec<f64> = tree
        .axes
        .iter()
        .flat_map(|a| a.phytomers.iter().map(|p| p.tip.z))
        .collect();
    let lowest = tips.iter().copied().fold(f64::MAX, f64::min);
    assert!(lowest >= 0.0, "a vertex {lowest} m below the ground");
    let resting = tips.iter().filter(|&&z| z < 0.01).count();
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
        // A limb lying on the ground has no rise to turn.
        if before < 0.4 && p[n - 1].tip.z > 0.05 {
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

/// The sharpest joint where a limb meets the ground, and how many limbs
/// reach it.
fn landing(species: &Species) -> (f64, usize) {
    let tree = walk::tree(species, 1).unwrap();
    let (mut sharpest, mut landed) = (0.0f64, 0);
    for axis in tree.axes.iter().filter(|a| a.pa > 0) {
        let r = rises(axis);
        let lowest = axis
            .phytomers
            .iter()
            .map(|p| p.tip.z)
            .fold(f64::MAX, f64::min);
        if lowest > 0.01 {
            continue;
        }
        landed += 1;
        // The joint where the wood meets the ground: the step that lands
        // against the one before it.
        if let Some(k) = axis.phytomers.iter().position(|p| p.tip.z <= 1e-9) {
            if k > 0 {
                sharpest = sharpest.max((r[k] - r[k - 1]).abs());
            }
        }
    }
    (sharpest, landed)
}

/// Wood lands on the ground without a kink: along a branch that comes
/// down to rest under sags from the spruce's up, no joint near the ground
/// turns it by more than 0.8 rad. Under the bent-lever sag any wood that
/// reaches the ground arrives near vertical and bends where it lands, as
/// a heavy rope does (0.43 to 0.72 rad measured); the bound guards against
/// regressions, and the stills judge the look (host, 2026-10-05, fn-203
/// decision 4).
#[test]
fn wood_lands_on_the_ground_without_a_corner() {
    for sag in [6e-4, 1e-3, 2e-3] {
        let (sharpest, landed) = landing(&level_limbs(sag));
        println!("sag {sag}: {landed} land, sharpest {sharpest:.3} rad");
        assert!(landed > 3, "sag {sag}: {landed} branches reach the ground");
        assert!(
            sharpest < 0.8,
            "sag {sag}: a corner of {sharpest} rad at the ground"
        );
    }
}

/// Under an extreme sag a branch hangs near vertical where it meets the
/// ground and bends sharply there, as a heavy rope reaching a floor does,
/// within decision 4's one bound of 0.8 rad (host, 2026-10-05, fn-203;
/// measured 0.66).
#[test]
fn heavy_wood_meets_the_ground_with_a_bounded_bend() {
    let (sharpest, landed) = landing(&grounded());
    println!("sag 5: {landed} land, sharpest {sharpest:.3} rad");
    assert!(landed > 3, "{landed} branches reach the ground");
    assert!(sharpest < 0.8, "a bend of {sharpest} rad at the ground");
}

/// The sag walk over limbs that come down onto the ground changes the
/// tree by degree: landing is a slope, not a jump.
#[test]
fn landing_on_the_ground_walks_by_degree() {
    // Limbs bear limbs too, so some stand near straight down, where
    // tropism weakens with their lean. A little wander keeps any limb
    // from standing exactly on the vertical, an unstable balance from
    // which an axis escapes at a rate no walk can resolve.
    fn landing(sag: f64) -> Species {
        let mut s = level_limbs(sag);
        s.states[1].form.wander = 0.2;
        s
    }
    let mut setting = walk::setting(
        "limb sag onto the ground".into(),
        0.0,
        // Up to the spruce's heaviest (fn-203 R3).
        6e-4,
        false,
        |s, v| *s = landing(v),
    );
    setting.stretch = walk::sag_bend(&walk::tree(&landing(0.0), 1).unwrap(), 1);
    let steps = walk::walk(&setting, 1);
    let worst = steps
        .iter()
        .max_by(|a, b| a.slope.total_cmp(&b.slope))
        .unwrap();
    assert!(worst.slope < 30.0, "{} in {}", worst.slope, worst.what);
    // Landing's steepest step is steep, not a jump: split finer it shrinks
    // eightfold a level once its split resolves the landing (five levels).
    let changes = walk::refine(&setting, 1, worst.from, worst.to, 5, 8);
    assert!(changes[5] < changes[0] / 100.0, "a jump: {changes:?}");
}

/// A vertical trunk bearing a one-sided load bends towards it: a moment
/// on vertical wood has an axis to turn about.
#[test]
fn a_vertical_trunk_bends_under_a_one_sided_crown() {
    let at = |sag: f64| {
        let mut species = walk::species();
        species.states[0].form.sag = sag;
        walk::tree(&species, 1).unwrap()
    };
    let top = |t: &Structure| t.axes[0].phytomers.last().unwrap().tip;
    let (plain, sagged) = (top(&at(0.0)), top(&at(1e-5)));
    let moved = (plain - sagged).length();
    assert!(moved > 1e-3, "the trunk's top moved {moved} m");
}

/// A short lateral swept through straight down near the ground lands by
/// degree: no lean too small to see sends it one way or the other.
#[test]
fn a_lateral_swept_through_straight_down_lands_by_degree() {
    use telperion_space::{grow, Form, NodeLaw, PaState, Request, Zone};
    let at = |insertion: f64| {
        let unit = |lateral: Vec<f64>| Zone {
            nodes: NodeLaw::Uniform { min: 1, max: 1 },
            buds: 1,
            dormant: vec![0.0; lateral.len()],
            delay: 0.0,
            rate: 0.0,
            lateral,
        };
        let mut trunk = walk::species().states[0].clone();
        trunk.lifespan = 1;
        trunk.zones = vec![unit(vec![0.0, 1.0])];
        trunk.internode = 1.0;
        trunk.form = Form {
            tropism: std::f64::consts::LN_2,
            elevation: 0.0,
            ..Form::default()
        };
        let mut lateral = PaState {
            lifespan: 1,
            zones: vec![unit(vec![0.0, 0.0])],
            internode: 0.4,
            insertion,
            form: Form::default(),
            ..trunk.clone()
        };
        lateral.next = None;
        trunk.next = None;
        let species = Species {
            states: vec![trunk, lateral],
        };
        let tree = grow(
            &species,
            Request {
                age: 2,
                seed: 1,
                budget: 100,
            },
        )
        .unwrap();
        tree.axes[1].phytomers[0].tip
    };
    let centre = 3.0 * std::f64::consts::FRAC_PI_4;
    let jump = (at(centre + 1e-7) - at(centre - 1e-7)).length();
    assert!(jump < 1e-4, "the tip moves {jump} m");
}

/// fn-203 review: the ground takes over a loaded phytomer's load by
/// degree as its end comes down to the ground, though what it carries is
/// a further axis. A lateral leaning down from a pole, carried on by a
/// continuation, is swept through the height at which its end reaches the
/// ground: its end moves in proportion.
#[test]
fn the_ground_takes_a_carried_load_by_degree() {
    use telperion_space::{grow, Form, NodeLaw, PaState, Request, Zone};
    let unit = |lateral: Vec<f64>| Zone {
        nodes: NodeLaw::Uniform { min: 1, max: 1 },
        buds: 1,
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    };
    let at = |height: f64| {
        let mut pole = walk::species().states[0].clone();
        pole.lifespan = 1;
        pole.next = None;
        pole.zones = vec![unit(vec![0.0, 1.0, 0.0])];
        pole.internode = height;
        pole.form = Form::default();
        let lateral = PaState {
            lifespan: 1,
            next: Some(2),
            zones: vec![unit(vec![0.0, 0.0, 0.0])],
            internode: 1.0,
            insertion: 3.0 * std::f64::consts::FRAC_PI_4,
            form: Form {
                sag: 0.02,
                ..Form::default()
            },
            ..pole.clone()
        };
        let on = PaState {
            next: None,
            form: Form::default(),
            ..lateral.clone()
        };
        let species = Species {
            states: vec![pole, lateral, on],
        };
        let request = Request {
            age: 3,
            seed: 1,
            budget: 100,
        };
        let tree = grow(&species, request).unwrap();
        tree.axes[1].phytomers[0].tip
    };
    let _ = Request {
        age: 1,
        seed: 1,
        budget: 1,
    };
    let (low, high, steps) = (0.6, 0.8, 400);
    let step = (high - low) / f64::from(steps);
    let mut steepest = 0.0f64;
    let mut previous = at(low);
    for i in 1..=steps {
        let next = at(low + step * f64::from(i));
        steepest = steepest.max((next - previous).length() / step);
        previous = next;
    }
    assert!(
        steepest < 30.0,
        "the end moves {steepest} m per m of height"
    );
}
