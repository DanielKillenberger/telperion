//! One busy cycle recorded on the device, in the council's phase order:
//! light, balance totals, shedding, surviving allocation totals, the
//! vigour split, the normalisation, fate and births.
use crate::bufs::{Binds, Bufs, Kits, Sizes};
use crate::gpu::{Gpu, Params, Recording};
use crate::tree::Blocks;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hier {
    /// Ancestor walks with integer atomics: a fixed dispatch count.
    A,
    /// Bounded subtrees in workgroups, block levels as boundaries.
    B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sweep {
    /// One workgroup a direction, looping the layers with barriers.
    Wg,
    /// One dispatch a layer.
    Layer,
}

fn groups(k: usize) -> [u32; 3] {
    [k.div_ceil(256) as u32, 1, 1]
}

fn p(a: [u32; 4]) -> Params {
    Params {
        a: [a[0], a[1], a[2], a[3], 0, 0, 0, 0],
        f: [0.0; 8],
    }
}

pub struct Ctx<'a> {
    pub gpu: &'a Gpu,
    pub kits: &'a Kits,
    pub bufs: &'a Bufs,
    pub sizes: &'a Sizes,
    pub blocks: &'a Blocks,
    pub light_wg: &'a wgpu::BindGroup,
    pub light_layer: &'a wgpu::BindGroup,
}

/// The buffers every run starts from: integer sums, shares and flags at
/// none, the box at its empty extremes.
pub fn reset(c: &Ctx, r: &mut Recording) {
    for name in [
        "acc_bal",
        "liv_acc",
        "acc_alloc",
        "incoming",
        "bud",
        "gacc",
        "quanta",
        "flags",
    ] {
        r.encoder.clear_buffer(c.bufs.get(name), 0, None);
    }
    let empty: [i32; 6] = [i32::MAX, i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN];
    c.gpu
        .queue
        .write_buffer(c.bufs.get("box"), 0, bytemuck::cast_slice(&empty));
}

pub fn light(c: &Ctx, r: &mut Recording, b: &Binds, sweep: Sweep, isolated: bool) {
    let (gpu, s, k) = (c.gpu, c.sizes, c.kits);
    let lit = |bg| (&k.light, bg);
    let mut pf = p([s.leaves as u32, s.tips as u32, s.dirs as u32, 0]);
    pf.f[0] = s.extinction;
    let bg = match sweep {
        Sweep::Wg => c.light_wg,
        Sweep::Layer => c.light_layer,
    };
    r.run(
        gpu,
        lit(bg),
        "span",
        "light.box",
        groups(s.leaves + s.tips),
        pf,
    );
    r.run(
        gpu,
        lit(bg),
        "deposit",
        "light.deposit",
        groups(s.leaves),
        pf,
    );
    r.run(
        gpu,
        lit(bg),
        "densities",
        "light.deposit",
        groups(s.nodes),
        pf,
    );
    // Swept whatever the extinction, so an unlit preset shows what its
    // lattice would cost lit; `combine` holds an unshaded lattice at one,
    // as the CPU does.
    match sweep {
        Sweep::Wg => r.run(
            gpu,
            lit(bg),
            "sweep_wg",
            "light.sweep",
            [s.dirs as u32, 1, 1],
            pf,
        ),
        Sweep::Layer => {
            for layer in (0..s.nz as u32).rev() {
                let mut q = pf;
                q.a[3] = layer;
                r.run(
                    gpu,
                    lit(bg),
                    "sweep_layer",
                    "light.sweep",
                    groups(s.dirs * s.layer),
                    q,
                );
            }
        }
    }
    r.run(gpu, lit(bg), "combine", "light.sweep", groups(s.nodes), pf);
    r.run(
        gpu,
        lit(&b.light_read),
        "read",
        "light.read",
        groups(s.tips),
        pf,
    );
    if isolated {
        // Read again on the CPU's lattice, into `reads_cpu`, with the
        // product kept from fusing where FMA_PROBE is set.
        let mut q = pf;
        q.a[4] = u32::from(std::env::var_os("FMA_PROBE").is_some());
        r.run(
            gpu,
            lit(bg),
            "read",
            "light.read (CPU lattice)",
            groups(s.tips),
            q,
        );
    }
}

fn sum(
    c: &Ctx,
    r: &mut Recording,
    (bg, label): (&wgpu::BindGroup, &str),
    hier: Hier,
    (two, living): (u32, u32),
) {
    let (gpu, n) = (c.gpu, c.sizes.n as u32);
    let kit = (&c.kits.sum, bg);
    match hier {
        Hier::A => {
            r.run(
                gpu,
                kit,
                "walk",
                label,
                groups(c.sizes.n),
                p([n, two, 0, living]),
            );
            r.run(
                gpu,
                kit,
                "unpack",
                label,
                groups(c.sizes.n),
                p([n, two, 0, living]),
            );
        }
        Hier::B => {
            for l in 0..c.blocks.levels as usize {
                let (s, e) = (c.blocks.level_start[l], c.blocks.level_start[l + 1]);
                r.run(
                    gpu,
                    kit,
                    "block",
                    label,
                    [e - s, 1, 1],
                    p([n, two, s, living]),
                );
            }
        }
    }
}

/// Top-down over the blocks, the root's level first.
fn down(
    c: &Ctx,
    r: &mut Recording,
    kit: (&crate::gpu::Kit, &wgpu::BindGroup),
    label: &str,
    a: [u32; 2],
) {
    for l in (0..c.blocks.levels as usize).rev() {
        let (s, e) = (c.blocks.level_start[l], c.blocks.level_start[l + 1]);
        r.run(
            c.gpu,
            kit,
            "block",
            label,
            [e - s, 1, 1],
            p([a[0], a[1], s, 0]),
        );
    }
}

pub fn hierarchy(c: &Ctx, r: &mut Recording, b: &Binds, hier: Hier, chained: bool) {
    let (gpu, s) = (c.gpu, c.sizes);
    let n = s.n as u32;
    let flat = (&c.kits.flat, &b.flat);
    r.run(
        gpu,
        flat,
        "own",
        "balance.own",
        groups(s.n),
        p([n, s.groups as u32, 0, 0]),
    );
    let bal = match hier {
        Hier::A => &b.sum_bal_a,
        Hier::B => &b.sum_bal_b,
    };
    sum(c, r, (bal, "balance.totals"), hier, (1, 1));
    let shed = (&c.kits.shed, &b.shed);
    match hier {
        Hier::A => {
            r.run(
                gpu,
                shed,
                "local",
                "shed",
                groups(s.n),
                p([n, s.cycle, 0, 0]),
            );
            r.run(
                gpu,
                shed,
                "walk",
                "shed",
                groups(s.n),
                p([n, s.cycle, 0, 0]),
            );
        }
        Hier::B => down(c, r, shed, "shed", [n, s.cycle]),
    }
    if chained {
        r.run(
            gpu,
            flat,
            "lit_of",
            "alloc.lit",
            groups(s.n),
            p([n, 0, 0, 0]),
        );
    }
    let alloc = match hier {
        Hier::A => &b.sum_alloc_a,
        Hier::B => &b.sum_alloc_b,
    };
    sum(c, r, (alloc, "alloc.totals"), hier, (0, 0));
    let split = (&c.kits.split, &b.split);
    match hier {
        Hier::A => {
            r.run(gpu, split, "factor", "split", groups(s.n), p([n, 1, 0, 0]));
            r.run(gpu, split, "walk", "split", groups(s.n), p([n, 1, 0, 0]));
        }
        Hier::B => down(c, r, split, "split", [n, 0]),
    }
    r.run(
        gpu,
        flat,
        "group_sums",
        "normalise",
        groups(s.n),
        p([n, s.groups as u32, 0, 0]),
    );
    r.run(
        gpu,
        flat,
        "size_of",
        "normalise",
        groups(s.n),
        p([n, s.groups as u32, 0, 0]),
    );
}

pub fn fate(c: &Ctx, r: &mut Recording, b: &Binds) {
    let (gpu, s) = (c.gpu, c.sizes);
    let f = (&c.kits.fate, &b.fate);
    let m = s.apexes as u32;
    let blocks = s.apexes.div_ceil(256) as u32;
    r.run(
        gpu,
        f,
        "fate",
        "fate",
        groups(s.apexes),
        p([m, s.before, blocks, 0]),
    );
    r.run(
        gpu,
        f,
        "scan_blocks",
        "births",
        [blocks, 1, 1],
        p([m, s.before, blocks, 0]),
    );
    r.run(
        gpu,
        f,
        "scan_sums",
        "births",
        [1, 1, 1],
        p([m, s.before, blocks, 0]),
    );
    r.run(
        gpu,
        f,
        "scan_add",
        "births",
        [blocks, 1, 1],
        p([m, s.before, blocks, 0]),
    );
}
