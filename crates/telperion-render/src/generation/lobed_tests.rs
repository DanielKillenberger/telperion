//! fn-212 task 11 (host, 2026-10-08, finding 8 (a)): lobed wood's ring
//! vertices laid on the device as the CPU's wood lays them, so seated
//! leaves on lobed wood take the same path as on round wood and lobe
//! depth is a degree, never a switch between ways of building.
use super::{io, Generator, Metrics};
use crate::Renderer;
use telperion_core::pipeline::executor::{self, Expansion};
use telperion_core::presets::Family;

/// The spruce at `age`, its wood two lobes a tenth deep, twisting at
/// `twist` turns over its height.
pub(super) fn lobed(age: u32, twist: f64) -> Family {
    let mut family = telperion_core::presets::by_identity("norway-spruce").unwrap();
    family.age = f64::from(age);
    family.surface.lobes = 2;
    family.surface.lobe_depth = 0.1;
    family.surface.twist_rate = twist;
    family
}

/// The device's ring vertices of `x`'s wood against the CPU's own build of
/// it, vertex by vertex: the most apart and the root mean square, metres.
fn apart(g: &Generator, x: &Expansion) -> (f64, f64, usize) {
    let p = x.compact().unwrap();
    assert!(p.qualified(), "lobed rings are laid on the device");
    let n = u64::from(p.vertices);
    let wood = pollster::block_on(g.emit_positions(p, &mut Metrics::default()));
    let wood = wood.unwrap().expect("the device's ring positions");
    let raw = io::read(&g.gpu, &wood.positions, n * 12).unwrap();
    let gpu: &[f32] = bytemuck::cast_slice(&raw);
    let cpu = x.wood().unwrap().positions;
    assert_eq!(gpu.len(), cpu.len(), "the vertex layouts");
    let (mut most, mut squared) = (0.0f64, 0.0f64);
    for (a, b) in gpu.chunks(3).zip(cpu.chunks(3)) {
        let d = (0..3).map(|k| f64::from(a[k] - b[k]).powi(2)).sum::<f64>();
        (most, squared) = (most.max(d.sqrt()), squared + d);
    }
    (most, (squared / n.max(1) as f64).sqrt(), n as usize)
}

fn generator() -> Option<Generator> {
    let gpu = crate::generation::test_gpu()?;
    Some(Generator::new(&Renderer::new(gpu, crate::STILL_FORMAT)).unwrap())
}

/// The compact path's position screen (the deleted `position_probe.rs`'s,
/// F3): 50 micrometres at most, 10 in the root mean square.
fn screened(label: &str, (most, rms, n): (f64, f64, usize)) {
    println!("LOBED {label}: {n} vertices, most {most:.3e} m, rms {rms:.3e} m");
    assert!(n > 0 && most <= 50e-6 && rms <= 10e-6, "{label}");
}

#[test]
fn lobed_wood_is_laid_on_the_device_as_on_the_cpu() {
    let Some(g) = generator() else {
        return;
    };
    for twist in [0.0, 0.5] {
        let x = executor::grow(&lobed(12, twist))
            .unwrap()
            .expansion()
            .unwrap();
        screened(&format!("spruce 12, twist {twist}"), apart(&g, &x));
    }
}

/// The spruce at 40 and 80 (`cargo test --profile ci -p telperion-render
/// lobed_at_40_and_80 -- --ignored --nocapture`, under the GPU lock).
#[test]
#[ignore = "device limits: a tree at 80 on the device"]
fn lobed_at_40_and_80() {
    let Some(g) = generator() else {
        return;
    };
    for age in [40, 80] {
        let x = executor::grow(&lobed(age, 0.5))
            .unwrap()
            .expansion()
            .unwrap();
        screened(&format!("spruce {age}"), apart(&g, &x));
    }
}
