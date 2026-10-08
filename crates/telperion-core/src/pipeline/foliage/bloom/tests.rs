//! Flowers by degree (fn-216): each floret keyed by its node's lineage, and
//! every count, share and time of year moving a flower without a pop.
use super::*;
use crate::tree::{Node, NodeIdentity};

/// A stem of `count` nodes a metre apart, each a bloom of `share` facing +X.
fn stem(count: u32, share: f64) -> Tree {
    let mut nodes = vec![Node::root()];
    let mut blooms = Vec::new();
    for i in 1..=count {
        let mut node = Node::root();
        node.position = Vec3::new(0.0, f64::from(i), 0.0);
        node.parent = Some(i - 1);
        node.identity = NodeIdentity::lineage(child(7, u64::from(i)));
        nodes.push(node);
        blooms.push(Bloom {
            node: i,
            share,
            side: Vec3::X,
        });
    }
    Tree {
        nodes,
        blooms,
        ..Tree::default()
    }
}

/// A hanging raceme of four florets, open at the window's middle.
fn raceme() -> BloomParams {
    BloomParams {
        florets: 4.0,
        rachis: 0.1,
        droop: 1.0,
        spread: 70.0,
        taper: 0.3,
        ..BloomParams::default()
    }
}

/// Each bloom's florets as transforms, before packing.
fn florets(tree: &Tree, p: &BloomParams, season: Option<f64>) -> Vec<Vec<Matrix>> {
    let now = now(p, season);
    tree.blooms
        .iter()
        .map(|b| {
            let mut out = Vec::new();
            inflorescence(tree, b, p, now, &mut out).unwrap();
            out
        })
        .collect()
}

/// The largest difference between two lists of transforms of one length.
fn moved(a: &[Matrix], b: &[Matrix]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .flatten()
        .zip(b.iter().flatten())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f32::max)
}

#[test]
fn a_floret_is_keyed_by_its_node_and_moves_with_no_other() {
    let p = raceme();
    let three = florets(&stem(3, 1.0), &p, None);
    let two = florets(&stem(2, 1.0), &p, None);
    assert_eq!(three[..2], two[..], "a bloom added moved another");
    assert_ne!(three[0], three[1], "two nodes drew the same flowers");
}

#[test]
fn a_raceme_hangs_and_tapers() {
    let tree = stem(1, 1.0);
    let racemes = florets(&tree, &raceme(), None);
    let all = &racemes[0];
    assert_eq!(all.len(), 4);
    let y = |m: &Matrix| m[13];
    assert!(
        all.windows(2).all(|w| y(&w[1]) < y(&w[0])),
        "it does not hang"
    );
    let size = |m: &Matrix| (m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt();
    assert!(size(&all[3]) < size(&all[0]), "the tip is not smaller");
}

/// A walk across the floret count grows the next floret from nothing.
#[test]
fn a_walk_across_the_floret_count_has_no_pop() {
    let tree = stem(1, 1.0);
    let at = |florets| {
        let p = BloomParams {
            florets,
            ..raceme()
        };
        florets_of(&tree, &p)
    };
    let (before, after) = (at(4.0), at(4.01));
    assert_eq!(after.len(), before.len() + 1, "the fifth floret grew in");
    let size = |m: &Matrix| (m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt();
    assert!(size(after.last().unwrap()) < 2e-2 * size(&after[0]));
    let jumped = moved(&before, &after[..4]);
    assert!(jumped < 2e-3, "the others jumped {jumped}");
}

fn florets_of(tree: &Tree, p: &BloomParams) -> Vec<Matrix> {
    let mut all = florets(tree, p, None);
    all.remove(0)
}

/// A walk across the flowering share grows each bloom in past its draw.
#[test]
fn a_walk_across_the_share_has_no_pop() {
    let p = raceme();
    let mut last: Vec<Vec<Matrix>> = florets(&stem(40, 0.0), &p, None);
    assert!(last.iter().all(Vec::is_empty), "flowers at no share");
    for step in 1..=200 {
        let share = f64::from(step) / 200.0;
        let now = florets(&stem(40, share), &p, None);
        for (a, b) in last.iter().zip(&now) {
            if a.len() == b.len() {
                assert!(moved(a, b) < 0.06, "a bloom jumped at {share}");
            } else {
                // A bloom that appears is barely grown.
                let size = |m: &Matrix| (m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt();
                assert!(b.iter().all(|m| size(m) < 0.1), "a bloom popped at {share}");
            }
        }
        last = now;
    }
    assert!(
        last.iter().all(|b| b.len() == 4),
        "a whole share flowers every node"
    );
}

/// A walk across the petal count turns the petals by degree and grows the
/// next one from nothing (host decision 4).
#[test]
fn a_walk_across_the_petal_count_has_no_pop() {
    let at = |petals| element(&BloomParams { petals, ..raceme() }).unwrap();
    let five = at(5.0);
    let more = at(5.001);
    let n = five.positions.len() / 5;
    assert_eq!(more.positions.len(), 6 * n);
    let far = |a: &[Vec3], b: &[Vec3]| {
        a.iter()
            .zip(b)
            .map(|(p, q)| p.distance(*q))
            .fold(0.0, f64::max)
    };
    assert!(far(&five.positions, &more.positions[..5 * n]) < 1e-4);
    let sixth = &more.positions[5 * n..];
    assert!(
        sixth.iter().all(|p| p.length() < 1e-4),
        "the sixth petal popped"
    );
    assert_eq!(five.levels.len(), at(1.0).levels.len());
    five.validate().unwrap();
}

/// Out of season nothing flowers; through the window each floret grows in
/// and out by degree.
#[test]
fn the_season_scales_florets_in_and_out() {
    let tree = stem(1, 1.0);
    let p = raceme();
    let out = florets(&tree, &p, Some(p.opens + 0.5)).remove(0);
    assert!(out.is_empty(), "flowers out of season");
    let size = |m: &Matrix| (m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt();
    let total = |t: f64| -> f32 { florets(&tree, &p, Some(t)).remove(0).iter().map(size).sum() };
    let mut last = 0.0;
    for step in 0..=400 {
        let t = p.opens - 0.01 + f64::from(step) * (p.lasts + 0.02) / 400.0;
        let now = total(t);
        assert!((now - last).abs() < 0.15, "a jump of {} at {t}", now - last);
        last = now;
    }
    assert!(
        total(p.opens + 0.5 * p.lasts) > 0.0,
        "no flowers at the window's middle"
    );
}

#[test]
fn every_floret_packs_inside_its_box() {
    let tree = stem(5, 1.0);
    let packed = place(&tree, &raceme(), None).unwrap();
    assert_eq!(packed.len(), 20);
    let all = florets(&tree, &raceme(), None).concat();
    for (i, m) in all.iter().enumerate() {
        let at = packed.position(i);
        let d = Vec3::new(m[12] as f64, m[13] as f64, m[14] as f64).distance(at);
        assert!(d < 1e-3, "floret {i} moved {d} in packing");
    }
}

/// A positive petal curl flares the tips away from the floret's axis, the
/// way a horn's mouth opens (DESIGN.md 9.1); zero is today's petal.
#[test]
fn a_positive_curl_flares_the_petal_tips_away_from_the_axis() {
    let at = |petal_curl| {
        let p = BloomParams {
            open: 20.0,
            petal_curl,
            ..raceme()
        };
        element(&p).unwrap()
    };
    let flat = at(0.0);
    assert_eq!(
        format!("{flat:?}"),
        format!("{:?}", element(&raceme_open(20.0)).unwrap())
    );
    // The first petal's tip: of its vertices, the farthest from the base.
    let n = flat.positions.len() / 5;
    let tip = (0..n)
        .max_by(|&a, &b| {
            flat.positions[a]
                .length()
                .total_cmp(&flat.positions[b].length())
        })
        .unwrap();
    // The first petal opens toward +X, so its tip's x is how far out it is.
    let reach = |e: &Element| e.positions[tip].x;
    let (out, flat_out) = (reach(&at(0.8)), reach(&flat));
    assert!(out > 1.5 * flat_out, "flared {out}, flat {flat_out}");
    assert!(reach(&at(-0.8)) < reach(&flat));
}

fn raceme_open(open: f64) -> BloomParams {
    BloomParams { open, ..raceme() }
}

/// Stamens (fn-217 F2): a ring of filaments, each one quad from the
/// floret's centre, coloured as the eye by their coordinates; none at zero,
/// the last grown in by the count's fraction, and dropped at a level whose
/// deviation reaches their length.
#[test]
fn stamens_stand_in_a_ring_by_degree_and_leave_the_coarse_levels() {
    let at = |stamens| {
        let p = BloomParams {
            stamens,
            stamen_length: 0.006,
            stamen_width: 0.0004,
            ..raceme()
        };
        element(&p).unwrap()
    };
    let none = at(0.0);
    assert_eq!(
        format!("{none:?}"),
        format!("{:?}", element(&raceme()).unwrap())
    );
    let five = at(5.0);
    let (petals, added) = (
        none.positions.len(),
        five.positions.len() - none.positions.len(),
    );
    assert_eq!(added, 5 * 4);
    let ring = &five.positions[petals..];
    let reach = ring.iter().map(|v| v.length()).fold(0.0, f64::max);
    assert!((reach - 0.006).abs() < 1e-4, "reach {reach}");
    assert!(five.coords[2 * petals..].chunks(2).all(|c| c[0] == 0.0));
    // A sixth grows in from nothing.
    let more = at(5.001);
    let sixth = &more.positions[petals + 20..];
    assert!(
        sixth.iter().all(|v| v.length() < 1e-4),
        "the sixth stamen popped"
    );
    // The finest level keeps them; a level as coarse as a stamen is long
    // drops them, and the ladder still narrows to the finest.
    let finest = five.levels.last().unwrap();
    assert_eq!(
        &five.level_indices[finest.indices.start as usize..],
        &five.indices[..]
    );
    for (l, base) in five.levels.iter().zip(&none.levels) {
        let grown = (l.indices.end - l.indices.start) - (base.indices.end - base.indices.start);
        let kept = if base.deviation < 0.006f64.hypot(0.0002) {
            5 * 6
        } else {
            0
        };
        assert_eq!((grown, l.deviation), (kept, base.deviation));
    }
    five.validate().unwrap();
}

/// A stamen as wide as it is long reaches past its length: a level drops it
/// only where its error reaches the farthest corner (review, fn-217 task 3).
#[test]
fn a_wide_stamen_is_kept_to_its_reach_not_its_length() {
    let p = BloomParams {
        stamens: 1.0,
        stamen_length: 1e-4,
        stamen_width: 0.01,
        ..raceme()
    };
    let e = element(&p).unwrap();
    let base = element(&raceme()).unwrap();
    let reach = 1e-4f64.hypot(0.005);
    for (l, b) in e.levels.iter().zip(&base.levels) {
        let grown = (l.indices.end - l.indices.start) - (b.indices.end - b.indices.start);
        assert_eq!(
            grown,
            if b.deviation < reach { 6 } else { 0 },
            "deviation {}",
            b.deviation
        );
    }
    assert!(base
        .levels
        .iter()
        .any(|b| b.deviation > 1e-4 && b.deviation < reach));
}
