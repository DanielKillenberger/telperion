//! fn-275 R2, R2a, R3: a rosette's crown and stem establish together. Its
//! fronds are made small while the crown is young; its stem stays at the
//! ground and widens until the crown is established, then rises. A tree
//! with no rosette never reads the rows.
use telperion_core::{
    foliage::Instances,
    pipeline::executor,
    presets::{Family, Preset},
    tree::Tree,
};

fn family(id: &str, seed: u32, age: f64) -> Family {
    let mut f = Preset::from_id(id).unwrap_or_else(|| panic!("{id} is a preset")).parameters();
    f.skeleton.seed = seed;
    f.age = age;
    f
}

/// The tree as the engine grows it, and its crown drawn at its whole age.
fn grown(f: &Family) -> (Tree, Instances) {
    let mut tree = executor::grow(f).unwrap().expansion().unwrap().tree().clone();
    tree.year = executor::growth(f).unwrap().year(1.0);
    let leaves = executor::expand(tree.clone(), f).unwrap().mesh().unwrap().foliage.instances;
    (tree, leaves)
}

fn without(mut f: Family) -> Family {
    f.canopy.frond_establishment = 0.0;
    f
}

fn top(tree: &Tree) -> f64 {
    tree.nodes.iter().map(|n| n.position.y).fold(f64::MIN, f64::max)
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
        let low = tree.nodes.iter().map(|n| n.position.y).fold(f64::MAX, f64::min);
        assert!(low >= -1e-9, "{age}: the stem below the ground at {low}");
        assert!(top(&tree) < 0.2 && top(&plain) > 0.3, "{age}: {} against {}", top(&tree), top(&plain));
        let girth = |t: &Tree| t.nodes.iter().map(|n| n.radius).fold(0.0f64, f64::max);
        assert!(girth(&tree) < 0.5 * girth(&plain), "{age}: {} against {}", girth(&tree), girth(&plain));
    }
    let oak = family("oregon-white-oak", 1, 12.0);
    let mut stated = oak.clone();
    stated.canopy.frond_establishment = 6.75;
    stated.canopy.frond_early_share = 0.1;
    let (a, b) = (grown(&stated), grown(&oak));
    assert_eq!((a.0.nodes, a.1.leaves), (b.0.nodes, b.1.leaves), "a tree with no rosette moved");
}

/// R2, R3: the fronds a young palm made are small; at its shipped age the
/// palm stands as tall as before (21.59 m, its pin), its stem whole in
/// girth above where it was laid while establishing (1.56 m measured).
#[test]
fn the_shipped_palm_keeps_its_height_and_narrows_only_its_foot() {
    let young = family("date-palm", 1, 3.0);
    let mean = |c: &Instances| c.leaves.iter().map(|l| c.reference.scale(*l)).sum::<f64>() / c.leaves.len() as f64;
    let ((_, small), (_, whole)) = (grown(&young), grown(&without(young.clone())));
    assert_eq!(small.leaves.len(), whole.leaves.len(), "establishment moved a frond's count");
    assert!(mean(&small) < 0.6 * mean(&whole), "{} against {}", mean(&small), mean(&whole));
    let palm = family("date-palm", 7, 40.0);
    let ((tree, _), (plain, _)) = (grown(&palm), grown(&without(palm.clone())));
    assert_eq!(tree.nodes.len(), plain.nodes.len(), "establishment moved a node's count");
    let narrowed = tree.nodes.iter().zip(&plain.nodes).filter(|(a, b)| a.radius != b.radius);
    let foot = narrowed.map(|(a, _)| a.position.y).fold(0.0f64, f64::max);
    println!("the palm at 40: stem top {:.3} m, narrowed to {foot:.3} m", top(&tree));
    assert!((top(&tree) - 14.885).abs() < 0.15, "the stem's top at {}", top(&tree));
    assert!(foot < 2.0, "the stem narrowed up to {foot} m");
}
