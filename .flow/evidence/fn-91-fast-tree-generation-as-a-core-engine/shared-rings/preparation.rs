use std::time::Instant;
use telperion_core::{branching, foliage::{self,TwigPlacement}, presets::Preset, surface};
fn hash(bytes: impl IntoIterator<Item=u8>) -> u64 { bytes.into_iter().fold(14695981039346656037u64, |h,b| (h ^ u64::from(b)).wrapping_mul(1099511628211)) }
fn main() {
    for species in ["oregon-white-oak", "norway-spruce"] { for seed in [1,7] {
        let mut f=Preset::from_id(species).unwrap().parameters(); f.skeleton.seed=seed;
        let tree=branching::generate(&f.skeleton,f.radii).unwrap().tree;
        let twig=f.skeleton.twigs.resolved().unwrap().twig;
        let twig=Some(TwigPlacement {internode_length:twig.internode_length,stations_per_internode:twig.stations_per_internode});
        for sample in 0..4 {
            let start=Instant::now();
            let old=foliage::prepared::prepare_stations(&tree,f.skeleton.envelope,f.canopy,twig,&f.surface).unwrap().unwrap();
            let wood=surface::prepared::prepare(&tree,f.skeleton.envelope.height,&f.surface).unwrap().unwrap();
            let old_ms=start.elapsed().as_secs_f64()*1000.;
            let start=Instant::now();
            let shared=surface::prepared::prepare_with_contacts(&tree,f.skeleton.envelope.height,&f.surface).unwrap().unwrap();
            let borrowed=foliage::prepared::prepare_shared_stations(&shared,f.skeleton.envelope,f.canopy,twig).unwrap().unwrap();
            let new_ms=start.elapsed().as_secs_f64()*1000.;
            let wood_hash=hash(format!("{wood:?}").bytes());
            assert_eq!(wood_hash,hash(format!("{:?}",shared.surface()).bytes()));
            assert_eq!(old.count,borrowed.count); assert_eq!(old.ring_size,borrowed.ring_size);
            assert_eq!(old.segments.len(),borrowed.segments.len());
            let count=old.count;
            let mut contact_hash=14695981039346656037u64;
            for (mut a,b) in old.segments.into_iter().zip(borrowed.segments) {
                if let (Some(old_edges),Some(new_edges))=(a.contact,b.contact) {
                    for (i,j) in old_edges.into_iter().zip(new_edges) { for k in 0..old.ring_size as usize {
                        let x=old.rings[i as usize+k]; let y=&borrowed.rings[(j as usize+k)*3..][..3];
                        assert_eq!([x.x,x.y,x.z],[y[0] as f64,y[1] as f64,y[2] as f64]);
                        for byte in y.iter().flat_map(|v|v.to_le_bytes()) { contact_hash=(contact_hash ^ u64::from(byte)).wrapping_mul(1099511628211); }
                    }}
                } else { assert_eq!(a.contact,b.contact); }
                a.contact=b.contact; assert_eq!(format!("{a:?}"),format!("{b:?}"));
            }
            println!("{{\"species\":\"{species}\",\"seed\":{seed},\"sample\":{sample},\"old_ms\":{old_ms},\"shared_ms\":{new_ms},\"stations\":{count},\"wood_hash\":\"{wood_hash:016x}\",\"contact_hash\":\"{contact_hash:016x}\",\"old_ring_capacity\":{},\"map_bytes\":{}}}",old.rings.capacity()*24, shared.contact_bytes());
        }
    }}
}
