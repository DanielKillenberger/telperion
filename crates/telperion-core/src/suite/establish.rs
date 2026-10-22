//! fn-275 R2, R2a, R3: a rosette's crown and stem establish together. Its
//! fronds are made small while the crown is young; its stem stays at the
//! ground and widens until the crown is established, then rises. A tree
//! with no rosette never reads the rows.
use telperion_core::{
    foliage::{CanopyParams, Crown, Instances},
    math::Vec3,
    pipeline::executor,
    presets::{Family, Preset},
    tree::{Node, Tree},
};

fn family(id: &str, seed: u32, age: f64) -> Family {
    let mut f = Preset::from_id(id)
        .unwrap_or_else(|| panic!("{id} is a preset"))
        .parameters();
    f.skeleton.seed = seed;
    f.age = age;
    f
}

/// The tree as the engine grows it, and its crown drawn at its whole age.
fn grown(f: &Family) -> (Tree, Instances) {
    let mut tree = executor::grow(f)
        .unwrap()
        .expansion()
        .unwrap()
        .tree()
        .clone();
    tree.year = executor::growth(f).unwrap().year(1.0);
    let leaves = executor::expand(tree.clone(), f)
        .unwrap()
        .mesh()
        .unwrap()
        .foliage
        .instances;
    (tree, leaves)
}

fn without(mut f: Family) -> Family {
    f.canopy.frond_establishment = 0.0;
    f
}

/// The wood the engine grew: the tree without its clothing's leaf bases
/// and frond axes (fn-283), which stand out of the stem and down past it.
fn grew(tree: &Tree) -> &[Node] {
    &tree.nodes[..tree.clothing.unwrap_or(tree.nodes.len())]
}

fn top(tree: &Tree) -> f64 {
    grew(tree)
        .iter()
        .map(|n| n.position.y)
        .fold(f64::MIN, f64::max)
}

/// R2a: a young palm's stem stands at the ground, narrow, under its crown,
/// and none of it below; without the rows the same palm's stem has risen.
/// A crown with no rosette is the tree it was.
#[test]
fn a_young_palms_stem_waits_at_the_ground() {
    for age in [1.0, 3.0] {
        let palm = family("date-palm", 1, age);
        let (tree, _) = grown(&palm);
        let (plain, _) = grown(&without(palm.clone()));
        let low = grew(&tree)
            .iter()
            .map(|n| n.position.y)
            .fold(f64::MAX, f64::min);
        assert!(low >= -1e-9, "{age}: the stem below the ground at {low}");
        assert!(
            top(&tree) < 0.2 && top(&plain) > 0.3,
            "{age}: {} against {}",
            top(&tree),
            top(&plain)
        );
        let girth = |t: &Tree| grew(t).iter().map(|n| n.radius).fold(0.0f64, f64::max);
        assert!(
            girth(&tree) < 0.5 * girth(&plain),
            "{age}: {} against {}",
            girth(&tree),
            girth(&plain)
        );
    }
    let oak = family("oregon-white-oak", 1, 12.0);
    let mut stated = oak.clone();
    stated.canopy.frond_establishment = 6.75;
    stated.canopy.frond_early_share = 0.1;
    let (a, b) = (grown(&stated), grown(&oak));
    assert_eq!(
        (a.0.nodes, a.1.leaves),
        (b.0.nodes, b.1.leaves),
        "a tree with no rosette moved"
    );
}

/// R2, R3: the fronds a young palm made are small, its crown narrow; at its shipped age the
/// palm stands as tall as before (21.59 m, its pin), its stem whole in
/// girth above where it was laid while establishing (1.56 m measured).
#[test]
fn the_shipped_palm_keeps_its_height_and_narrows_only_its_foot() {
    let young = family("date-palm", 1, 3.0);
    // How far out its leaves stand: their leaflets' length is the blade's
    // form (R2b), the frond's size its spread.
    let spread = |c: &Instances| {
        c.leaves
            .iter()
            .map(|l| c.reference.position(*l))
            .map(|p| p.x.hypot(p.z))
            .fold(0.0, f64::max)
    };
    let ((_, small), (_, whole)) = (grown(&young), grown(&without(young.clone())));
    // R2b: its young fronds carry fewer leaflets, so fewer leaves.
    assert!(
        small.leaves.len() < whole.leaves.len(),
        "{} against {}",
        small.leaves.len(),
        whole.leaves.len()
    );
    assert!(
        spread(&small) < 0.6 * spread(&whole),
        "{} against {}",
        spread(&small),
        spread(&whole)
    );
    let palm = family("date-palm", 7, 40.0);
    let ((tree, _), (plain, _)) = (grown(&palm), grown(&without(palm.clone())));
    assert_eq!(
        tree.nodes.len(),
        plain.nodes.len(),
        "establishment moved a node's count"
    );
    let narrowed = tree
        .nodes
        .iter()
        .zip(&plain.nodes)
        .filter(|(a, b)| a.radius != b.radius);
    let foot = narrowed.map(|(a, _)| a.position.y).fold(0.0f64, f64::max);
    println!(
        "the palm at 40: stem top {:.3} m, narrowed to {foot:.3} m",
        top(&tree)
    );
    assert!(
        (top(&tree) - 14.885).abs() < 0.15,
        "the stem's top at {}",
        top(&tree)
    );
    assert!(foot < 2.0, "the stem narrowed up to {foot} m");
}

/// R2a (review): a stem drawn within its year, its shoot part grown, is
/// lowered by the height each tip has reached: it never dips below the
/// wood it stands on nor below the ground.
#[test]
fn a_stem_drawn_within_its_year_never_dips() {
    use telperion_core::pipeline::growth;
    let palm = family("date-palm", 1, 4.0);
    let params = growth::Params::of(&palm).unwrap();
    let mut state = growth::begin(&params, 1, 4.0).unwrap();
    for at in [0.1, 0.3, 0.9, 1.05, 1.3, 2.02, 3.5] {
        growth::grow_to(&mut state, at).unwrap();
        let tree = growth::settle(&state, at).unwrap();
        for n in &tree.nodes {
            let below = n.parent.map_or(0.0, |p| tree.nodes[p as usize].position.y);
            assert!(
                n.position.y >= below - 1e-6,
                "{at}: a node at {} below its parent at {below}",
                n.position.y
            );
        }
    }
}

/// R2a (review): rows that make the first stem thinner than the thinnest
/// wood drawn keep the whole stem.
#[test]
fn a_thin_first_stem_keeps_the_stem() {
    let mut palm = family("date-palm", 1, 12.0);
    (
        palm.canopy.frond_early_share,
        palm.canopy.frond_establishment,
    ) = (0.0, 10.0);
    let ((tree, _), (plain, _)) = (grown(&palm), grown(&without(palm.clone())));
    assert_eq!(grew(&tree).len(), grew(&plain).len(), "the stem was cut");
}

/// R2b: a seedling's first fronds are entire. Each frond carries the
/// leaflets it was made with, segments of one blade that splits (host), by the establishment's course on a log
/// scale: one at the first frond, the mature count from the end of the
/// establishment years, by degree between (no frond's log count above the
/// last one's by more than 0.8, the steepest step measured at 0.73,
/// across a year's turn), and kept for the frond's life. The drawn
/// seedling carries them; without the rows every frond carries the
/// mature count.
#[test]
fn a_seedlings_fronds_are_entire_and_become_pinnate() {
    use telperion_core::tree::Year;
    let palm = family("date-palm", 1, 12.0);
    let (mature, years) = (palm.canopy.leaflet_count, palm.canopy.frond_establishment);
    let year = |age: f64| Year {
        age,
        budburst: 0.0,
        elongation: 226.0,
        expansion: 139.0,
    };
    let crown = |age: f64| Crown::new(Some(year(age)), &palm.canopy);
    let c = crown(12.0);
    let per_year = palm.canopy.rosette_fronds / palm.canopy.frond_life;
    let first = c.leaflets(0.0);
    assert!(
        (1.0..1.01).contains(&first),
        "the first frond carries {first} leaflets"
    );
    let established = (per_year * years.ceil()).ceil() as i64;
    let (mut last, mut steepest) = (first, 0.0f64);
    for s in 1..established + 8 {
        let n = c.leaflets(s as f64);
        assert!(
            n >= last,
            "frond {s} carries {n}, fewer than the one before at {last}"
        );
        steepest = steepest.max((n / last).ln());
        last = n;
        for age in [6.0, 6.0 + 1e-9, 40.0] {
            assert_eq!(
                crown(age).leaflets(s as f64),
                n,
                "frond {s} changed its leaflets at {age}"
            );
        }
    }
    println!("the steepest step: {steepest:.3} in the log count");
    assert!(steepest < 0.8, "a frond's log count stepped {steepest}");
    assert_eq!(
        c.leaflets(established as f64),
        mature,
        "a frond made once established"
    );
    // Its leaflets are strips of one blade (host): the first frond one
    // strap the blade's length; a mature frond its rows; and every
    // frond's strips reach about its blade's tip whether its draw gives
    // it the lesser or the greater whole count: measured 0.79 (at f 0.38,
    // its segments crowded toward its base, fn-275 R2c) to 1.21 blades
    // (the mature 1.48 m leaflet past a 7 m rachis), the two counts at
    // most 0.04 apart. And no leaflet's base stands off its frond (R2c):
    // within a width of another segment's centreline, measured 0.75 at
    // most where the frond draws no spine (0.42 on the mature frond).
    let blade = telperion_core::foliage::blade(&executor::element(palm.element).unwrap());
    let p = palm.canopy;
    let (strap, split) = c.segments(p, &p, 0.0, blade);
    let split = split.expect("a young frond splits");
    let (base, length) = split.segment(0, 1, p.rachis_length, p.size * blade);
    let reach = base + length * p.size * blade / p.rachis_length;
    assert!(
        (reach - 1.0).abs() < 0.01 && strap.leaflet_pitch < 0.1,
        "the strap reaches {reach} at {}",
        strap.leaflet_pitch
    );
    assert_eq!(
        c.segments(p, &p, established as f64, blade),
        (p, None),
        "a mature frond moved"
    );
    let element = executor::element(palm.element).unwrap();
    let width = 2.0
        * element
            .positions
            .iter()
            .map(|v| v.x.abs())
            .fold(0.0, f64::max);
    let mut widest = 0.0f64;
    for s in 0..established {
        let Some(split) = c.segments(p, &p, s as f64, blade).1 else {
            continue;
        };
        let n = c.leaflets(s as f64);
        let reach = |n: usize| {
            let segment = |k| split.segment(k, n, p.rachis_length, p.size * blade);
            let tip = |(b, l): (f64, f64)| b + l * p.size * blade / p.rachis_length;
            (0..n).map(|k| tip(segment(k))).fold(0.0, f64::max)
        };
        let (lo, hi) = (reach(n.floor() as usize), reach(n.ceil() as usize));
        println!(
            "frond {s}: f {:.3}, {n:.2} leaflets, reach {lo:.3} to {hi:.3} blades",
            split.f
        );
        assert!(
            (0.78..1.22).contains(&lo) && (0.78..1.22).contains(&hi),
            "frond {s} reaches {lo} to {hi}"
        );
        assert!(
            (hi - lo).abs() < 0.2,
            "frond {s} reaches {lo} or {hi} by its draw"
        );
        // Each whole count of leaflets and of spines its draws can give.
        let spines = c.segments(p, &p, s as f64, blade).0.acanthophylls;
        let mut gaps = [0.0f64; 3];
        for n in [n.floor(), n.ceil()].map(|n| n as usize) {
            for whole in [spines.floor(), spines.ceil()] {
                let [leaflet, spine] = gap(&c, &p, s as f64, (n, whole), (blade, width));
                let at = usize::from(whole > 0.0);
                gaps[at] = gaps[at].max(leaflet);
                gaps[2] = gaps[2].max(spine);
            }
        }
        println!(
            "frond {s}: gap {:.2} widths, {:.2} by a spine, spines {:.2} ({spines:.2} spines)",
            gaps[0], gaps[1], gaps[2]
        );
        widest = widest.max(gaps[0]);
    }
    let grown_gap = gap(
        &c,
        &p,
        established as f64,
        (p.leaflet_count as usize, p.acanthophylls),
        (blade, width),
    );
    println!(
        "the widest gap: {widest:.2} widths; the mature frond's {:.2}, its spines' {:.2}",
        grown_gap[0], grown_gap[1]
    );
    assert!(
        widest <= 1.0,
        "a leaflet's base stands {widest} widths off its frond"
    );
    let leaves = |age: f64, f: &Family| grown(&family_at(f, age)).1.leaves.len();
    let drawn = |age: f64| {
        let f = family_at(&palm, age);
        Crown::new(executor::growth(&f).unwrap().year(1.0), &f.canopy).slots()
    };
    let plain = without(palm.clone());
    for age in [1.0, 2.0, 3.0, 4.0] {
        let (young, whole) = (leaves(age, &palm), leaves(age, &plain));
        let fronds = drawn(age);
        println!("{age}: {fronds} fronds, {young} leaves, {whole} without the rows");
        assert_eq!(
            whole as f64,
            fronds as f64 * mature,
            "{age}: a frond without the rows"
        );
        assert!(young < whole, "{age}: {young} leaves against {whole}");
    }
    let one = leaves(1.0, &palm);
    assert!(
        one <= drawn(1.0) + 2,
        "the seedling at 1 bears {one} leaves"
    );
}

/// R2c (host): the widest gaps on frond `serial` drawn with `n` segments,
/// before any draw, its leaflets' and its spines': each base after the
/// first, to the nearest point of another segment's centreline, in that
/// segment's widths. A leaf element reaches `blade` metres and is `width`
/// metres across at a size of one, its spines a whole count. None where
/// it draws one segment.
fn gap(
    c: &Crown,
    p: &CanopyParams,
    serial: f64,
    (n, spines): (usize, f64),
    (blade, width): (f64, f64),
) -> [f64; 2] {
    let (young, split) = c.segments(*p, p, serial, blade);
    let young = CanopyParams {
        acanthophylls: spines,
        ..young
    };
    // Clear of the ground, so the curve is the parabola (fn-301).
    let clear = Vec3::Y * 100.0;
    let lines: Vec<_> =
        telperion_core::foliage::leaflets(clear, Vec3::X, (young, Some(0.0)), n, split)
            .map(|l| {
                let scale = young.size * l.share.unwrap_or(1.0);
                (l.at, l.at + l.axis * (blade * scale), width * scale)
            })
            .collect();
    let near = |q: Vec3, (a, b, _): (Vec3, Vec3, f64)| {
        let ab = b - a;
        let t = ((q - a).dot(ab) / ab.length_squared().max(1e-30)).clamp(0.0, 1.0);
        q.distance(a + ab * t)
    };
    let spine = |k: usize| (k as f64) < young.acanthophylls && young.acanthophyll_length > 0.0;
    let mut widest = [0.0f64; 2];
    for k in 1..n {
        let others = (0..n)
            .filter(|&j| j != k)
            .map(|j| near(lines[k].0, lines[j]));
        let at = usize::from(spine(k));
        widest[at] = widest[at].max(others.fold(f64::MAX, f64::min) / lines[k].2);
    }
    widest
}

fn family_at(f: &Family, age: f64) -> Family {
    Family { age, ..f.clone() }
}
