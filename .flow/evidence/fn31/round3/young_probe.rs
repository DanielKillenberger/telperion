use telperion_core::{
    branching::Specimen,
    presets::Preset,
    tree::{BudFate, NodeKind},
};
fn main() {
    for p in [
        Preset::Ordinary,
        Preset::Telperion,
        Preset::Laurelin,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
    ] {
        for age in [1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 14.1, 26.7] {
            let mut f = p.parameters();
            f.skeleton.seed = 7;
            f.age = age;
            let s = Specimen::build(&f).unwrap();
            let t = s.tree();
            let leaves = s.placements().unwrap();
            let lateral = t
                .nodes
                .iter()
                .filter(|n| n.shoot.bud_fate == BudFate::Lateral)
                .count();
            let structural = t
                .nodes
                .iter()
                .filter(|n| n.kind == NodeKind::Structural && n.shoot.bud_fate == BudFate::Lateral)
                .count();
            let h = t.nodes.iter().map(|n| n.position.y).fold(0., f64::max);
            let spread = t
                .nodes
                .iter()
                .map(|n| n.position.x.hypot(n.position.z))
                .fold(0., f64::max);
            let upper = leaves
                .iter()
                .filter(|p| f64::from(p.transform[13]) > h * 0.5)
                .count();
            println!("{p:?} {age} nodes={} laterals={lateral} structural={structural} leaves={} upper={upper} height={h} spread={spread}",t.nodes.len(),leaves.len());
            if p == Preset::Ordinary && age == 2. {
                for n in &t.nodes {
                    println!(
                        "NODE {:?} {:?} radius={} start={} pos={:?}",
                        n.kind, n.shoot.bud_fate, n.radius, n.start_radius, n.position
                    );
                }
            }
        }
    }
}
