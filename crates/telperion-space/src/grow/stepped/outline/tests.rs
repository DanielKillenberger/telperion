//! R3 and R4 in the executor (fn-226 task 3): where no bud reads light
//! the two models are one; before the first outline every bud reads
//! light 1; and the history runs one way, the structure above the same
//! whatever its roots grow.
use super::super::grow_outline;
use super::*;
use crate::light::Light;

fn request(age: u32, light: Light) -> Request {
    Request {
        age,
        seed: 7,
        budget: 20_000_000,
        light,
    }
}

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// A species none of whose buds read light builds today's tree.
#[test]
fn where_no_bud_reads_light_the_models_are_one() {
    let mut species = crate::oak();
    for s in &mut species.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    let r = request(24, LIT);
    let today = crate::grow::grow(&species, r).unwrap();
    let outlined = grow_outline(&species, r).unwrap();
    assert!(outlined.grown > 0);
    assert_eq!(
        crate::fine::digest(&outlined.structure),
        crate::fine::digest(&today)
    );
}

/// A species whose buds read light takes R3 and R4 under no shade too,
/// every bud at light 1: the limit of its own as extinction falls, so
/// the least extinction contract A holds builds the same tree.
#[test]
fn under_no_shade_a_lit_species_is_its_own_limit() {
    let species = crate::oak();
    let none = grow_outline(&species, request(24, Light::NEUTRAL)).unwrap();
    let least = Light {
        extinction: 1e-12,
        sky: 0.5,
    };
    let faint = grow_outline(&species, request(24, least)).unwrap();
    assert!(none.roots > 0 && !none.growth.is_empty(), "R3 and R4 ran");
    assert_eq!(
        crate::fine::digest(&none.structure),
        crate::fine::digest(&faint.structure)
    );
}

/// The bootstrap: no outline before the first cycle's, and a bud with
/// none reads light 1, whatever it read before.
#[test]
fn before_the_first_outline_every_bud_reads_light_one() {
    let species = crate::oak();
    let mut x = Executor::outlined(&species, request(12, LIT))
        .unwrap()
        .expect("light is read");
    let mut a = x.above();
    assert!(a.history.outline(0).is_none());
    for cycle in 1..=6 {
        x.above_cycle(cycle, &mut a).unwrap();
    }
    let o = a.history.outline(5).expect("an outline");
    assert!(o.volume > 0 && o.mean < ONE32, "the crown shades");
    read(&mut x.coarse, Some(o), 1);
    let lights = |g: &Grower| -> Vec<f64> {
        let pencils = &g.sketch.as_ref().unwrap().pencils;
        g.live.iter().map(|l| pencils[l.axis].light).collect()
    };
    assert!(lights(&x.coarse).iter().any(|&l| l < 1.0));
    read(&mut x.coarse, None, 1);
    assert!(lights(&x.coarse).iter().all(|&l| l == 1.0));
}

/// One way: the structure above and its history are made before any
/// root grows, and growing the roots ahead leaves both as they were;
/// the roots grown or left as handed over, the structure above is the
/// same.
#[test]
fn the_history_runs_one_way() {
    let species = crate::oak();
    let r = request(24, LIT);
    let above = |grow_roots: bool| {
        let mut x = Executor::outlined(&species, r).unwrap().unwrap();
        let mut a = x.above();
        for cycle in 1..=r.age {
            x.above_cycle(cycle, &mut a).unwrap();
        }
        let before = format!("{:?}{:?}", x.coarse.axes, a.history.outlines);
        let roots: usize = x.items.iter().map(|i| i.g.axes.len()).sum();
        if grow_roots {
            x.ahead(&a.history, &mut Vec::new()).unwrap();
        }
        let grown: usize = x.items.iter().map(|i| i.g.axes.len()).sum();
        let after = format!("{:?}{:?}", x.coarse.axes, a.history.outlines);
        assert_eq!(before, after, "the roots reach nothing above");
        (after, roots, grown)
    };
    let (grown_ahead, roots, grown) = above(true);
    let (left, ..) = above(false);
    assert!(
        roots > 0 && grown > roots,
        "{roots} axes handed, {grown} grown"
    );
    assert_eq!(grown_ahead, left);
}
