//! fn-255's checks on the one cycle (DESIGN.md section 1, CYCLE-COST.md).
use super::*;
use crate::light::Light;

fn request(age: u32, seed: u64, extinction: f64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light {
            extinction,
            sky: 0.5,
        },
    }
}

/// The one cycle with every axis open and none retired: the reference
/// its working set and its retirement are held to.
fn reference(species: &Species, r: Request) -> Structure {
    run(species, r, false, Working::reference())
        .unwrap()
        .structure
}

/// The most a phytomer's radius moves when retirement freezes the girth
/// of what closed (fn-270 R4, the host's ruling: a frozen subtree's wood
/// stops ripening): measured at most 2.8e-6 on these trees, 5.4e-5 on
/// the shipped presets (Telperion).
const SHIFT: f64 = 1e-4;

/// The working set's frozen subtrees and the subtrees retired as the tree
/// grows leave every sum, and so the tree, the reference's to the bit but
/// for the girth retirement freezes, where many are retired: the oak
/// (which reads light and keeps leaf girth), the spruce and the beech.
#[test]
fn the_working_set_and_retirement_leave_the_tree_as_it_was() {
    for (species, k) in [
        (crate::oak(), 0.5),
        (crate::spruce(), 0.0),
        (crate::beech(), 0.0),
    ] {
        let r = request(40, 7, k);
        let cycled = grow_census(&species, r).unwrap();
        let retired = cycled.census.expect("a census").retired;
        assert!(retired > 100, "{retired} retired");
        let apart = super::girth_apart(&cycled.structure, &reference(&species, r));
        println!("{retired} retired, girth apart {apart:?}");
        assert!(
            apart.is_some_and(|(_, most)| most <= SHIFT),
            "{retired} retired, girth apart {apart:?}"
        );
    }
}

/// At zero and arbitrarily near zero the same code runs: the tree at no
/// extinction is the tree at 1e-30, and with no response to shade the
/// tree with 1e-30 of it.
#[test]
fn zero_and_near_zero_grow_the_same_tree() {
    let species = crate::oak();
    let tree = |s: &Species, k: f64| grow_cycle(s, request(16, 3, k)).unwrap().structure;
    assert!(tree(&species, 0.0) == tree(&species, 1e-30), "extinction");
    let response = |v: f64| {
        let mut s = species.clone();
        for state in &mut s.states {
            (state.shade_hazard, state.shade_size, state.balance_hazard) = (v, v, v);
        }
        s
    };
    assert!(
        tree(&response(0.0), 0.5) == tree(&response(1e-30), 0.5),
        "response"
    );
}

/// The quantised deposit is the same lattice in any order of the leaves,
/// and a node past its u32 of quanta is refused by name.
#[test]
fn the_quantised_deposit_is_order_free_and_refuses_overflow() {
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let leaves: Vec<(Vec3, f64)> = (0..500)
        .map(|k| {
            let t = f64::from(k);
            (
                Vec3::new(
                    (t * 0.37).sin() * 3.0,
                    (t * 0.71).cos() * 3.0,
                    2.0 + (t * 0.13).sin(),
                ),
                0.0018 * (1.0 + (t * 0.3).cos()),
            )
        })
        .collect();
    let reads: Vec<Vec3> = leaves
        .iter()
        .map(|l| l.0 - Vec3::new(0.0, 0.0, 0.3))
        .collect();
    let forward = Field::quantised(&light, &leaves, &reads, 1).unwrap();
    let mut reversed = leaves.clone();
    reversed.reverse();
    let back = Field::quantised(&light, &reversed, &reads, 1).unwrap();
    for &r in &reads {
        assert_eq!(forward.at(r).to_bits(), back.at(r).to_bits());
    }
    let heavy = vec![(Vec3::new(0.0, 0.0, 1.0), 3000.0); 2];
    let refused = Field::quantised(&light, &heavy, &[], 1)
        .err()
        .expect("refused");
    assert!(
        format!("{refused}").starts_with("light.deposit"),
        "{refused}"
    );
}

/// The f32 survival the device reads: the table's viability, shaded in
/// f32; whole without shade or in full light.
#[test]
fn survival_reads_the_tables_f32() {
    use crate::contract::decide::shaded;
    assert_eq!(shaded(0.995, 1.0, 1.0), 0.995);
    assert_eq!(shaded(0.995, 0.0, 0.3), 0.995);
    let p = shaded(0.995, 1.0, 0.5);
    assert!(
        p < 0.995 && (f64::from(p) - 0.995f64.powf(2.0)).abs() < 1e-6,
        "{p}"
    );
}

/// How far the one cycle's tree lies from today's whole-tree engine, by
/// lineage, under the spike's contracts (f32 viability, the one reduction
/// order, the integer deposit): printed for the evidence, not bounded.
#[test]
#[ignore]
fn agreement_with_todays_tree() {
    for (name, species, k) in [
        ("oak", crate::oak(), 0.5),
        ("beech", crate::beech(), 0.0),
        ("spruce", crate::spruce(), 0.0),
    ] {
        for (age, seed) in [(24, 7), (40, 1)] {
            let r = request(age, seed, k);
            let today = crate::grow::whole(&species, r).unwrap();
            let cycled = grow_cycle(&species, r).unwrap();
            let a = crate::agree::agreement(&today, &cycled.structure);
            eprintln!("AGREE {name} {age} {seed}: {a:?}");
        }
    }
}

/// No light at all leaves no survival under any shade hazard, certain
/// survival certain, and none without a hazard (review round 1).
#[test]
fn survival_in_the_dark() {
    use crate::contract::decide::shaded;
    assert_eq!(shaded(0.995, 0.001, 0.0), 0.0);
    assert_eq!(shaded(1.0, 0.5, 0.0), 1.0);
    assert_eq!(shaded(0.9, 0.0, 0.0), 0.9);
}

/// Buds that wake and stop in their waking cycle, beside bearers that
/// grow on: the oak (whose leaf girth keeps retired records) with every
/// zone's laterals also sleeping and waking, and every lateral age
/// aborting often, retires them and still grows the reference tree but
/// for the girth retirement freezes (review round 1: a woken bud's kept
/// share reads its bearer's next unit).
#[test]
fn woken_buds_that_stop_at_once_retire_as_the_reference_has_them() {
    let mut species = crate::oak();
    for (pa, state) in species.states.iter_mut().enumerate() {
        if pa > 0 {
            state.abortion = state.abortion.max(0.4);
        }
        for zone in &mut state.zones {
            zone.dormant = zone.lateral.iter().map(|&p| (p * 0.5).min(0.5)).collect();
            zone.rate = 0.6;
        }
    }
    for seed in 1..=4 {
        let r = request(22, seed, 0.5);
        let cycled = grow_census(&species, r).unwrap();
        let retired = cycled.census.expect("a census").retired;
        assert!(retired > 10, "seed {seed}: {retired} retired");
        let apart = super::girth_apart(&cycled.structure, &reference(&species, r));
        println!("seed {seed}: {retired} retired, girth apart {apart:?}");
        assert!(
            apart.is_some_and(|(_, most)| most <= SHIFT),
            "seed {seed}: {retired} retired, girth apart {apart:?}"
        );
    }
}

/// The grown capture holds every phytomer the tree grew, none retired,
/// each with its layout's words, and a pencil an axis laid (fn-256 task 2).
#[test]
fn the_grown_capture_holds_every_phytomer() {
    for (species, age) in [
        (crate::beech(), 8),
        (crate::spruce(), 10),
        (crate::oak(), 9),
    ] {
        let c = grow_capture_grown(&species, request(age, 42, 0.5)).unwrap();
        let u32s = |name: &str| -> Vec<u32> {
            let b = c.array(name).expect(name).1;
            b.chunks(4)
                .map(|w| u32::from_le_bytes(w.try_into().unwrap()))
                .collect()
        };
        let held: u64 = u32s("g_phyt_n").iter().map(|&n| u64::from(n)).sum();
        let grown = u64::from_le_bytes(c.array("g_grown").unwrap().1.try_into().unwrap());
        assert_eq!(held, grown, "every phytomer grown is held");
        assert_eq!(u32s("g_phyt_axis").len() as u64, held);
        assert_eq!(u32s("sk_phyts").len() as u64, 13 * held);
        let axes = u32s("g_pa").len();
        assert_eq!(u32s("sk_axes").len(), 11 * axes);
        assert_eq!(u32s("sk_pencils").len() % 63, 0);
    }
}
