//! Agreement with the CPU cycle's phase outputs: integers to the bit,
//! reals by their bit-equal share and their largest differences.
use crate::bufs::{Bufs, Sizes};
use crate::cap::Cap;
use crate::cycle::Hier;
use crate::gpu::Gpu;
use crate::tree::{fixed, fixed_sums};
use telperion_space::contract::{exp2, log2, INF};

pub fn floats(name: &str, dev: &[f32], cpu: &[f32]) {
    assert_eq!(dev.len(), cpu.len(), "{name}: lengths");
    let (mut exact, mut abs, mut rel, mut worst) = (0usize, 0f64, 0f64, 0usize);
    for (k, (&d, &c)) in dev.iter().zip(cpu).enumerate() {
        if d.to_bits() == c.to_bits() || (d.is_nan() && c.is_nan()) {
            exact += 1;
            continue;
        }
        let a = (f64::from(d) - f64::from(c)).abs();
        let r = if c != 0.0 { a / f64::from(c).abs() } else { a };
        if a > abs || a.is_nan() {
            worst = k;
        }
        abs = abs.max(a);
        rel = rel.max(r);
    }
    println!(
        "  {name}: bit-equal {exact}/{} ({:.4}%); max abs {abs:.3e}, max rel {rel:.3e} (worst at {worst})",
        dev.len(),
        100.0 * exact as f64 / dev.len().max(1) as f64
    );
}

pub fn ints<T: PartialEq + Copy + std::fmt::Debug>(name: &str, dev: &[T], cpu: &[T]) {
    assert_eq!(dev.len(), cpu.len(), "{name}: lengths");
    let wrong: Vec<usize> = (0..dev.len()).filter(|&k| dev[k] != cpu[k]).collect();
    let first = wrong
        .first()
        .map(|&k| format!(" (first at {k}: {:?} against {:?})", dev[k], cpu[k]));
    println!(
        "  {name}: {} of {} differ{}",
        wrong.len(),
        dev.len(),
        first.unwrap_or_default()
    );
}

fn wide(v: &[u32]) -> Vec<u64> {
    v.chunks(2)
        .map(|w| u64::from(w[0]) | (u64::from(w[1]) << 32))
        .collect()
}

/// `allocation::drawn` on the host, by the contract's operations.
fn drawn(v: f32, p: f32, psi: f32) -> f32 {
    if p <= 0.0 || psi == 0.0 {
        return 1.0;
    }
    if v <= 0.0 {
        return if psi > 0.0 { 0.0 } else { INF };
    }
    exp2(psi * (log2(v) - log2(p)))
}

pub fn hierarchy(gpu: &Gpu, bufs: &Bufs, cap: &Cap, s: &Sizes, hier: Hier) {
    let n = s.n;
    let rd = |name: &str, k: usize| gpu.read::<f32>(bufs.get(name), k);
    let ru = |name: &str, k: usize| gpu.read::<u32>(bufs.get(name), k);
    let parent = cap.u32s("parent");
    // The balance's own values, as the CPU's `totals` takes them.
    let (bs, run, light, wood, pa) = (
        cap.f32s("base_scale"),
        cap.f32s("running"),
        cap.f32s("light"),
        cap.f32s("wood"),
        cap.u32s("pa"),
    );
    let upkeep = cap.f32s("pa_upkeep");
    let mut live = vec![false; n];
    for a in cap.u32s("live") {
        live[a as usize] = true;
    }
    let own_a: Vec<f32> = (0..n)
        .map(|i| {
            if live[i] {
                bs[i] * run[i] * light[i]
            } else {
                0.0
            }
        })
        .collect();
    let own_b: Vec<f32> = (0..n).map(|i| upkeep[pa[i] as usize] * wood[i]).collect();
    println!("balance totals ({hier:?})");
    ints("own light (device against host)", &rd("own_a", n), &own_a);
    ints("own upkeep (device against host)", &rd("own_b", n), &own_b);
    if hier == Hier::A {
        let acc = wide(&ru("acc_bal", 4 * n));
        let (a, b): (Vec<u64>, Vec<u64>) = acc.chunks(2).map(|w| (w[0], w[1])).unzip();
        ints(
            "light, integer sums against the host's",
            &a,
            &fixed_sums(&parent, &own_a),
        );
        ints(
            "upkeep, integer sums against the host's",
            &b,
            &fixed_sums(&parent, &own_b),
        );
    }
    floats(
        "light against the CPU's f32",
        &rd("tot_a", n),
        &cap.f32s("cpu_total_light"),
    );
    floats(
        "upkeep against the CPU's f32",
        &rd("tot_b", n),
        &cap.f32s("cpu_total_cost"),
    );
    ints("living", &ru("living", n), &cap.u32s("cpu_living"));
    println!("shedding ({hier:?}, on the CPU's totals)");
    let cpu_draws = cap.u64s("cpu_shed_draw");
    let dev_draws = wide(&ru("sdraws", 2 * n));
    let decided: Vec<usize> = (0..n).filter(|&i| cpu_draws[i] != 0).collect();
    let pickd = |v: &[u64]| decided.iter().map(|&i| v[i]).collect::<Vec<_>>();
    ints(
        "shed draws of the deciding laterals",
        &pickd(&dev_draws),
        &pickd(&cpu_draws),
    );
    ints("shed", &ru("shed", n), &cap.u32s("cpu_shed"));
    floats("kept", &rd("kept", n), &cap.f32s("cpu_kept"));
    floats("memory", &rd("mem_out", n), &cap.f32s("cpu_memory"));
    println!("surviving allocation totals ({hier:?}, on the CPU's lit presences)");
    let lit = cap.f32s("cpu_lit");
    if hier == Hier::A {
        let acc = wide(&ru("acc_alloc", 4 * n));
        let a: Vec<u64> = acc.chunks(2).map(|w| w[0]).collect();
        ints(
            "integer sums against the host's",
            &a,
            &fixed_sums(&parent, &lit),
        );
    }
    floats(
        "subtree light against the CPU's f32",
        &rd("sub", n),
        &cap.f32s("cpu_sub"),
    );
    println!("vigour split ({hier:?}, on the CPU's subtree light)");
    let incoming = match hier {
        Hier::A => rd("rest", n),
        Hier::B => rd("incoming", n),
    };
    floats("incoming", &incoming, &cap.f32s("cpu_incoming"));
    floats("bud vigour", &rd("bud", n), &cap.f32s("cpu_bud"));
}

pub fn normalise(gpu: &Gpu, bufs: &Bufs, cap: &Cap, s: &Sizes) {
    let n = s.n;
    println!("normalisation (on the CPU's vigours and presences)");
    let (bud, pres, pa) = (
        cap.f32s("cpu_bud"),
        cap.f32s("cpu_presence"),
        cap.u32s("pa"),
    );
    let psi = cap.f32s("pa_psi");
    let mut host = vec![0u64; 2 * s.groups];
    for i in (0..n).filter(|&i| pres[i] > 0.0) {
        let g = pa[i] as usize;
        host[2 * g] += fixed(pres[i]);
        host[2 * g + 1] += fixed(drawn(bud[i], pres[i], psi[g]) * pres[i]);
    }
    ints(
        "integer group sums against the host's",
        &wide(&gpu.read::<u32>(bufs.get("gacc"), 4 * s.groups)),
        &host,
    );
    floats(
        "group sums against the CPU's f32",
        &gpu.read::<f32>(bufs.get("sums"), 2 * s.groups),
        &cap.f32s("cpu_sums"),
    );
    floats(
        "sizes",
        &gpu.read::<f32>(bufs.get("sizes"), n),
        &cap.f32s("cpu_sizes"),
    );
}

pub fn light(gpu: &Gpu, bufs: &Bufs, cap: &Cap, s: &Sizes) {
    println!("light (deposit, sweep, read)");
    let bx: Vec<i32> = gpu.read(bufs.get("box"), 6);
    let (low, size) = (cap.i32s("grid_low"), cap.u32s("grid_size"));
    let dev_size: Vec<u32> = (0..3).map(|a| (bx[a + 3] - bx[a] + 1) as u32).collect();
    ints("box low", &bx[..3], &low);
    ints("box size", &dev_size, &size);
    println!(
        "  overflow flag: {}",
        gpu.read::<u32>(bufs.get("flags"), 1)[0]
    );
    ints(
        "quanta",
        &gpu.read::<u32>(bufs.get("quanta"), s.nodes),
        &cap.u32s("quanta"),
    );
    ints(
        "densities (bits)",
        &gpu.read::<u32>(bufs.get("density"), s.nodes),
        &bits(&cap.f32s("density")),
    );
    floats(
        "lattice (device sweep, f32) against the CPU's (f64, rounded once)",
        &gpu.read(bufs.get("lattice"), s.nodes),
        &cap.f32s("lattice"),
    );
    floats(
        "reads on the CPU's lattice",
        &gpu.read(bufs.get("reads_cpu"), s.tips),
        &cap.f32s("reads"),
    );
    floats(
        "reads on the device's lattice",
        &gpu.read(bufs.get("reads"), s.tips),
        &cap.f32s("reads"),
    );
}

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

pub fn fate(gpu: &Gpu, bufs: &Bufs, cap: &Cap, s: &Sizes) {
    let m = s.apexes;
    println!("fate and births ({m} apexes)");
    let (thr, draw) = (cap.u64s("cpu_fate_threshold"), cap.u64s("cpu_fate_draw"));
    let dthr = wide(&gpu.read::<u32>(bufs.get("thr"), 2 * m));
    ints(
        "survival draws",
        &wide(&gpu.read::<u32>(bufs.get("fdraws"), 2 * m)),
        &draw,
    );
    ints("survival thresholds", &dthr, &thr);
    let worst = (0..m).map(|k| dthr[k].abs_diff(thr[k])).max().unwrap_or(0);
    println!(
        "  largest threshold difference: {worst} of 2^53 ({:.3e})",
        worst as f64 / 2f64.powi(53)
    );
    let cpu_survives: Vec<u32> = (0..m).map(|k| u32::from(draw[k] < thr[k])).collect();
    ints(
        "survival decisions",
        &gpu.read::<u32>(bufs.get("survives"), m),
        &cpu_survives,
    );
    let births = cap.u32s("cpu_births");
    let first = gpu.read::<u32>(bufs.get("first"), m);
    let cpu_first = cap.u32s("cpu_first_birth");
    let born: Vec<usize> = (0..m).filter(|&k| births[k] > 0).collect();
    let pick = |v: &[u32]| born.iter().map(|&k| v[k]).collect::<Vec<_>>();
    ints(
        "first index of each apex's births",
        &pick(&first),
        &pick(&cpu_first),
    );
}
