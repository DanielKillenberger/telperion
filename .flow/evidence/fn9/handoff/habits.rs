use telperion_core::{branching::{generate,SkeletonParams,BranchHabit,SpreadingHabit,TieredHabit},radius::RadiusParams,envelope::Envelope,bias::BiasParams};
fn main(){
 for (id,habit,e) in [
 ("oak",BranchHabit::Spreading(SpreadingHabit::default()),Envelope{height:20.,crown_base:0.2,spread:0.55,fullness:0.55,shoulder:2.2}),
 ("spruce",BranchHabit::Tiered(TieredHabit::default()),Envelope{height:16.,crown_base:0.12,spread:0.3,fullness:0.18,shoulder:1.2})] {
 let p=SkeletonParams{habit,envelope:e,bias:BiasParams::NONE,..Default::default()};
 let tree=generate(&p,RadiusParams::default()).unwrap().tree;
 eprintln!("{} nodes {} crossover {} complete {}",id,tree.nodes.len(),tree.crossover,tree.diagnostics.complete());
 for (i,n) in tree.nodes.iter().enumerate().skip(1){let p=&tree.nodes[n.parent.unwrap() as usize]; println!("{} {} {} {} {} {} {} {} {} {}",id,i,p.position.x,p.position.y,p.position.z,n.position.x,n.position.y,n.position.z,n.start_radius,n.radius);}
 }
}
