//! The device's sizing programs (`vm.wgsl`) compute what the host's
//! evaluation of the same expressions does, over every operation; a
//! flag raised at a point leaves its uniform words none; and the host's
//! bounds of an expression hold every value it takes in them.
use super::super::io;
use super::vm::{program, Dst, Kernels, E};

/// Expressions over every operation, the census's words and constants
/// past 24 bits.
fn expressions() -> Vec<E> {
    let (a, b, z) = (E::At(1), E::At(2), E::At(0));
    vec![
        a.clone().add(b.clone()),
        a.clone().sub(b.clone()),
        b.clone().sub(a.clone()),
        a.clone().mul(E::C(3)),
        a.clone().div(E::C(64)),
        a.clone().div(z.clone()),
        a.clone().min(b.clone()).max(E::C(5)),
        a.clone().gt(b.clone()).sel(E::C(1 << 30), E::C(17)),
        b.clone()
            .gt(a.clone())
            .sel(a.clone().div(E::C(256)), b.clone().mul(E::C(2))),
        E::C(65535 * 64).add(a.clone().sub(E::C(1 << 25)).min(E::C(1 << 26))),
        z.gt(E::C(0)).sel(E::C(0), a.add(b).div(E::C(65535))),
    ]
}

#[test]
fn the_sizing_programs_compute_what_the_host_does() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let census = [0u32, 100_000, 3_999_999, 7, 1 << 28];
    let exprs = expressions();
    let mut code: Vec<u32> = vec![0; exprs.len()];
    for (j, e) in exprs.iter().enumerate() {
        code[j] = code.len() as u32;
        code.extend(program(Dst::Cfg(j as u32), e));
    }
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let store = |label, words: &[u32]| {
        io::buffer(
            &gpu,
            label,
            (words.len().max(4) * 4) as u64,
            usage,
            bytemuck::cast_slice(words),
        )
        .unwrap()
    };
    let (code_b, census_b) = (store("vm code", &code), store("vm census", &census));
    let (cfgs, args) = (
        store("vm cfgs", &vec![0; exprs.len()]),
        store("vm args", &[0; 4]),
    );
    let run = |words: [u32; 8]| {
        let cfg = io::buffer(
            &gpu,
            "vm cfg",
            32,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&words),
        )
        .unwrap();
        let bind = k.pass.bind(&gpu, &[&cfg, &code_b, &census_b, &cfgs, &args]);
        let mut e = gpu.encoder(&Default::default());
        k.pass.run(&mut e, &bind, 0, 1);
        gpu.submit([e.finish()]);
        bytemuck::cast_slice::<u8, u32>(&io::read(&gpu, &cfgs, cfgs.size()).unwrap()).to_vec()
    };
    let n = exprs.len() as u32;
    let want: Vec<u32> = exprs.iter().map(|e| e.eval(&census)).collect();
    assert_eq!(
        &run([0, n, 0, 0, 0, 0, 0, 0])[..want.len()],
        &want[..],
        "the device's words are the host's"
    );
    // The census's word 3 as a flag, raised: every word the point sets
    // is none.
    assert!(
        run([0, n, 3, 1, 0, 0, 0, 0])[..want.len()]
            .iter()
            .all(|&w| w == 0),
        "a raised flag leaves no word"
    );
}

/// Expressions as the plan sizes dispatches: a range's share of a count,
/// a difference of counts that only grow, a level's width under a guard.
fn sizes() -> Vec<E> {
    let (a, b, c) = (E::At(0), E::At(1), E::At(2));
    vec![
        a.clone().sub(E::C(1000)).min(E::C(500)).div(E::C(64)),
        a.clone().sub(b.clone()).div(E::C(64)),
        c.clone().gt(E::C(3)).sel(a.clone().sub(b.clone()), E::C(0)),
        a.mul(c).add(b).max(E::C(7)),
    ]
}

#[test]
fn the_hosts_bounds_hold_every_value() {
    let (lo, hi) = ([900, 200, 1], [5000, 400, 9]);
    for e in sizes() {
        let (least, most) = e.span(&lo, &hi);
        for x in (lo[0]..=hi[0]).step_by(97) {
            for y in (lo[1]..=hi[1]).step_by(13) {
                for z in lo[2]..=hi[2] {
                    let v = e.eval(&[x, y, z]);
                    assert!(
                        (least..=most).contains(&v),
                        "{e:?} at {:?}: {v} outside {least}..={most}",
                        [x, y, z]
                    );
                }
            }
        }
    }
}
