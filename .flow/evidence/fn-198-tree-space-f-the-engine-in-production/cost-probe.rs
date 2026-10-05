//! Scratch (fn-198 measurement, not committed): one species, one seed, per
//! stage: growth stages, conversion, expansion prep, CPU wood alone, the
//! whole CPU mesh; counts and peak RSS.
//!   f_cost <species> <age> <seed>
#[path = "space/tree.rs"]
mod tree;
use std::time::Instant;
use telperion_core::{params, pipeline::executor, presets::Preset};
use telperion_space::{grow_staged, Light, Request, Stage};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (name, age, seed) = (a[0].as_str(), a[1].parse().unwrap(), a[2].parse::<u64>().unwrap());
    let lit = Light { extinction: 0.5, sky: 0.5 };
    let (species, trunk, preset, rows, light): (_, &[usize], _, _, _) = match name {
        "beech" => (telperion_space::beech(), &tree::BEECH_TRUNK, "european-beech",
            r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.018, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.006}}}}"#, Light::NEUTRAL),
        "spruce" => (telperion_space::spruce(), &tree::SPRUCE_TRUNK, "norway-spruce",
            r#"{"canopy": {"shortShootSpacing": 0, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.0032}}}}"#, Light::NEUTRAL),
        "oak" => (telperion_space::oak(), &tree::OAK_TRUNK, "oregon-white-oak",
            r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.06, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.008}}}}"#, lit),
        "palm" => (telperion_space::palm(), &tree::PALM_TRUNK, "date-palm", "{}", Light::NEUTRAL),
        _ => panic!("species"),
    };
    let order = [Stage::Grown, Stage::Sketched, Stage::Relaid, Stage::Lit, Stage::Settled, Stage::Laid];
    let mut spent = [0.0f64; 6];
    let started = Instant::now();
    let mut last = started;
    let mut relays: Vec<f64> = Vec::new();
    let structure = grow_staged(&species, Request { age, seed, budget: 20_000_000, light }, &mut |s| {
        let now = Instant::now();
        spent[order.iter().position(|&o| o == s).unwrap()] += (now - last).as_secs_f64();
        if s == Stage::Relaid { relays.push((now - last).as_secs_f64() * 1e3); }
        last = now;
    })
    .expect("grows");
    let grow_s = started.elapsed().as_secs_f64();
    let kept = structure.phytomer_count();
    let digest = hash(&structure);
    let axes = structure.axes.len();
    let t = Instant::now();
    let ptree = tree::convert(&structure, trunk);
    let convert_s = t.elapsed().as_secs_f64();
    drop(structure);
    let nodes = ptree.nodes.len();
    let root = ptree.nodes[0].radius;
    let (mut fine, mut all) = (0.0, 0.0);
    for n in &ptree.nodes[1..] {
        let l = (n.position - ptree.nodes[n.parent.unwrap() as usize].position).length();
        all += l;
        if n.radius < tree::STRUCTURAL * root {
            fine += l;
        }
    }
    let base = Preset::from_id(preset).unwrap().parameters();
    let rows: serde_json::Value = serde_json::from_str(rows).unwrap();
    let mut family = params::overlay(&base, &rows).unwrap();
    family.skeleton.seed = (seed ^ (seed >> 32)) as u32;
    let t = Instant::now();
    let x = executor::expand(ptree, &family).unwrap();
    let expand_s = t.elapsed().as_secs_f64();
    let clothed = x.tree().nodes.len();
    let t = Instant::now();
    let wood = x.wood().unwrap();
    let wood_s = t.elapsed().as_secs_f64();
    drop(wood);
    let t = Instant::now();
    let mesh = x.mesh().unwrap();
    let mesh_s = t.elapsed().as_secs_f64();
    let peak = std::fs::read_to_string("/proc/self/status").unwrap().lines()
        .find(|l| l.starts_with("VmHWM")).map(|l| l[6..].trim().to_string()).unwrap();
    let ms = |v: f64| v * 1e3;
    println!(
        "{name} age {age} seed {seed}: grow {:.1} ms [growth {:.1} rough {:.1} relay {:.1} light {:.1} settle {:.1} lay {:.1}] relays {:?}; convert {:.1}; expand-prep {:.1}; cpu-wood-alone {:.1}; cpu-mesh {:.1}; kept {kept} phytomers {axes} axes; nodes {nodes} clothed {clothed}; wood {:.2} km fine {:.2} km; wood tris {}; leaves {}; peak {peak}; hash {digest:016x}",
        ms(grow_s), ms(spent[0]), ms(spent[1]), ms(spent[2]), ms(spent[3]), ms(spent[4]), ms(spent[5]),
        relays.iter().map(|r| r.round() as i64).collect::<Vec<_>>(),
        ms(convert_s), ms(expand_s), ms(wood_s), ms(mesh_s), all / 1e3, fine / 1e3,
        mesh.wood_triangles(), mesh.foliage_instances()
    );
}

fn hash(tree: &telperion_space::Structure) -> u64 {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut h = DefaultHasher::new();
    let v = |h: &mut DefaultHasher, v: telperion_space::Vec3| [v.x, v.y, v.z].map(|c| c.to_bits()).hash(h);
    (tree.age, tree.pas).hash(&mut h);
    for axis in &tree.axes {
        (axis.lineage, axis.pa, axis.birth, axis.apex_end).hash(&mut h);
        format!("{:?}", axis.origin).hash(&mut h);
        axis.vigour.to_bits().hash(&mut h);
        v(&mut h, axis.base);
        v(&mut h, axis.heading);
        v(&mut h, axis.side);
        for p in &axis.phytomers {
            (p.cycle, p.radius.to_bits(), p.scale.to_bits()).hash(&mut h);
            v(&mut h, p.tip);
            v(&mut h, p.heading);
            v(&mut h, p.side);
        }
    }
    h.finish()
}
