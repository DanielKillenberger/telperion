//! fn-301 .5 host ruling 1: on the base, the lowest bark of non-trunk wood,
//! min(centreline y - radius) over the drawn curve's points, per row.
use telperion_core::{mesh, presets};

fn main() {
    let rows: &[(&str, f64, u32)] = &[
        ("ordinary", 40., 7),
        ("oregon-white-oak", 40., 7),
        ("norway-spruce", 20., 42),
        ("ordinary", 40., 1),
        ("ordinary", 40., 42),
        ("oregon-white-oak", 40., 1),
        ("oregon-white-oak", 40., 42),
        ("norway-spruce", 20., 1),
        ("norway-spruce", 20., 7),
        ("norway-spruce", 40., 1),
    ];
    println!("preset\tage\tseed\tbark_low\tcentre_at_low\tradius_at_low\truns_below\tpoints_below");
    for &(id, age, seed) in rows {
        let mut f = presets::by_identity(id).unwrap();
        (f.age, f.skeleton.seed) = (age, seed);
        let m = mesh::build(&f).unwrap();
        let c = &m.curve;
        let (mut low, mut centre, mut radius, mut runs, mut points) = (f64::INFINITY, 0., 0., 0, 0);
        for run in c.runs.iter().filter(|r| !r.trunk) {
            let ps = &c.points[run.first as usize..(run.first + run.count) as usize];
            let mut under = false;
            for p in ps {
                let bark = p.centre.y - p.radius;
                if bark < 0. {
                    points += 1;
                    under = true;
                }
                if bark < low {
                    (low, centre, radius) = (bark, p.centre.y, p.radius);
                }
            }
            runs += usize::from(under);
        }
        println!("{id}\t{age}\t{seed}\t{low:.5}\t{centre:.5}\t{radius:.5}\t{runs}\t{points}");
    }
}
