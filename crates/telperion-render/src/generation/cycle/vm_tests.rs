//! The device's sizing programs (`vm.wgsl`) compute what the host's
//! evaluation of the same expressions does, over every operation.
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
        a.clone().floor(E::C(7)),
        a.clone().floor(z.clone()),
        a.clone().min(b.clone()).max(E::C(5)),
        a.clone().gt(b.clone()).sel(E::C(1 << 30), E::C(17)),
        b.clone().gt(a.clone()).sel(a.clone().div(E::C(256)), b.clone().mul(E::C(2))),
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
    let store = |label, words: &[u32]| io::buffer(&gpu, label, (words.len().max(4) * 4) as u64, usage, bytemuck::cast_slice(words)).unwrap();
    let (code_b, census_b) = (store("vm code", &code), store("vm census", &census));
    let (cfgs, args) = (store("vm cfgs", &vec![0; exprs.len()]), store("vm args", &[0; 4]));
    let cfg = io::buffer(&gpu, "vm cfg", 16, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&[0u32, exprs.len() as u32, 0, 0])).unwrap();
    let bind = k.pass.bind(&gpu, &[&cfg, &code_b, &census_b, &cfgs, &args]);
    let mut e = gpu.encoder(&Default::default());
    k.pass.run(&mut e, &bind, 0, 1);
    gpu.submit([e.finish()]);
    let got: Vec<u32> = bytemuck::cast_slice(&io::read(&gpu, &cfgs, cfgs.size()).unwrap()).to_vec();
    let want: Vec<u32> = exprs.iter().map(|e| e.eval(&census)).collect();
    assert_eq!(&got[..want.len()], &want[..], "the device's words are the host's");
}
