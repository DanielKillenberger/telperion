//! Leaf counts along a walk (fn-277 R1): each frame's family
//! `blend::families(a, b, t)` built by the direct build, as the film and
//! `headless --walk` build it, at `steps` evenly spaced shares from `t0`
//! to `t1`.
//!
//!   cargo run --profile ci -p telperion-core --example walk_leaves -- \
//!     <from> <to> <age> <seed> <t0> <t1> <steps>
//!
//! With `WALK_DIFF=1`, instead every catalogue row and engine setting that
//! differs between the frames at `t0` and `t1`. With `WALK_ISOLATE=1`,
//! the frame at `t1` built again with each whole-number row that stepped
//! since `t0` put back to its value there, one at a time.
use telperion_core::{blend, catalogue, mesh, presets::by_identity, Family};
use telperion_space::{settings, zone_settings};

fn end(id: &str, age: f64, seed: u32) -> Family {
    let mut f = by_identity(id).unwrap_or_else(|e| panic!("{id}: {e}"));
    (f.age, f.skeleton.seed) = (age, seed);
    f
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [from, to, age, seed, t0, t1, steps] = &args[..] else {
        panic!("usage: <from> <to> <age> <seed> <t0> <t1> <steps>");
    };
    let (age, seed) = (age.parse().unwrap(), seed.parse().unwrap());
    let (t0, t1, steps): (f64, f64, usize) = (t0.parse().unwrap(), t1.parse().unwrap(), steps.parse().unwrap());
    let (a, b) = (end(from, age, seed), end(to, age, seed));
    if std::env::var("WALK_DIFF").is_ok() {
        let (x, y) = (blend::families(&a, &b, t0).unwrap(), blend::families(&a, &b, t1).unwrap());
        return diff(&x, &y);
    }
    if std::env::var("WALK_ISOLATE").is_ok() {
        let (x, y) = (blend::families(&a, &b, t0).unwrap(), blend::families(&a, &b, t1).unwrap());
        return isolate(&x, &y);
    }
    println!("t\tleaves\theight\tfronds\tleaflets\trachis\tskirt");
    for i in 0..=steps {
        let t = t0 + (t1 - t0) * i as f64 / steps.max(1) as f64;
        let family = blend::families(&a, &b, t).unwrap();
        let tree = mesh::build(&family).unwrap();
        let h = tree.bounds.max.y - tree.bounds.min.y;
        let c = &family.canopy;
        println!(
            "{t:.5}\t{}\t{h:.3}\t{}\t{}\t{:.3}\t{}",
            tree.foliage_instances(),
            c.rosette_fronds,
            c.leaflet_count,
            c.rachis_length,
            c.skirt_fronds
        );
    }
}

/// Every row whose value differs between `x` and `y`.
fn diff(x: &Family, y: &Family) {
    for e in catalogue::entries() {
        let (u, v) = (e.get(x).number(), e.get(y).number());
        if u != v {
            println!("{}\t{u:?}\t{v:?}", e.path());
        }
    }
    let (Some(sx), Some(sy)) = (&x.space.species, &y.space.species) else { return };
    for (k, (p, q)) in sx.states.iter().zip(&sy.states).enumerate() {
        let (mut p, mut q) = (p.clone(), q.clone());
        for ((n, u), (_, v)) in settings(&mut p).into_iter().zip(settings(&mut q)) {
            if u != v {
                println!("space[{k}].{n}\t{u}\t{v}");
            }
        }
        for (r, (zp, zq)) in p.zones.iter_mut().zip(q.zones.iter_mut()).enumerate() {
            let (a, b) = (zone_settings(zp).unwrap_or_default(), zone_settings(zq).unwrap_or_default());
            for ((n, u), (_, v)) in a.into_iter().zip(b) {
                if u != v {
                    println!("space[{k}].zone[{r}].{n}\t{u}\t{v}");
                }
            }
        }
    }
}

fn leaves(f: &Family) -> usize {
    mesh::build(f).unwrap().foliage_instances()
}

/// `y` with each whole row that differs from `x` put back, one at a time.
fn isolate(x: &Family, y: &Family) {
    println!("before\t{}\nafter\t{}", leaves(x), leaves(y));
    let whole = |v: f64| v == v.trunc();
    for e in catalogue::entries() {
        let (Some(u), Some(v)) = (e.get(x).number(), e.get(y).number()) else { continue };
        if u != v && whole(u) && whole(v) {
            let mut f = y.clone();
            e.set(&mut f).put(u);
            println!("{} back to {u}\t{}", e.path(), leaves(&f));
        }
    }
    let (Some(sx), Some(_)) = (&x.space.species, &y.space.species) else { return };
    let mut f = y.clone();
    let s = f.space.species.as_mut().unwrap();
    for (k, state) in s.states.iter_mut().enumerate() {
        state.cluster_leaves = sx.states[k].cluster_leaves;
    }
    println!("space cluster_leaves back\t{}", leaves(&f));
}
