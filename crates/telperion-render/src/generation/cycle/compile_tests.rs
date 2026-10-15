//! Bisecting a module's cold compile (fn-235's method, DESIGN.md section
//! 9 item 3): one module's WGSL, edited by the replacements a file names,
//! made into one entry's pipeline natively with the driver's disk cache
//! off (`__GL_SHADER_DISK_CACHE=0`), one variant a process.
//! `CYCLE_BISECT_MODULE` names the module (units, layout, place),
//! `CYCLE_BISECT_ENTRY` the entry, and `CYCLE_BISECT_EDITS` a file of
//! edits, each `old` then a line `====` then `new`, edits apart by a line
//! `%%%%`.
use super::super::io;
use std::time::Instant;

/// A module's WGSL and which of its storage bindings it only reads.
fn module(name: &str) -> (String, Vec<bool>) {
    let [three, four] = [
        [true; 3]
            .iter()
            .chain(&[false; 5])
            .copied()
            .collect::<Vec<_>>(),
        [true; 4].iter().chain(&[false; 4]).copied().collect(),
    ];
    match name {
        "units" => (super::source(), three.clone()),
        "layout" => (super::layout_source(), three),
        "light" => (
            super::light_source(),
            [true].iter().chain(&[false; 6]).copied().collect(),
        ),
        "sums" => (super::hier_cycle_source(), four),
        "place" => (
            super::settle_device::place_source(),
            vec![true, true, false, false, false],
        ),
        "settle" => (
            super::settle_device::source(),
            vec![true, true, true, false, false, false, true, true],
        ),
        other => panic!("no module {other}"),
    }
}

fn edited(mut source: String, edits: &str) -> String {
    for edit in edits.split("\n%%%%\n").filter(|e| !e.trim().is_empty()) {
        let (old, new) = edit.split_once("\n====\n").expect("an edit's old and new");
        assert!(source.contains(old), "an edit that finds nothing: {old}");
        source = source.replacen(old, new, 1);
    }
    source
}

#[test]
#[ignore = "a measurement"]
fn a_module_is_made_cold() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let name = std::env::var("CYCLE_BISECT_MODULE").unwrap_or_else(|_| "units".into());
    let entry = std::env::var("CYCLE_BISECT_ENTRY").unwrap_or_else(|_| "cycle_main".into());
    let edits = std::env::var("CYCLE_BISECT_EDITS")
        .map(|p| std::fs::read_to_string(p).expect("the edits"))
        .unwrap_or_default();
    let (source, reads) = module(&name);
    let source = edited(source, &edits);
    let started = Instant::now();
    let pass = io::Pass::new(&gpu, source, &reads, &[entry.as_str()]);
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("made");
    println!(
        "BISECT {name} {entry}: {:.2} s",
        started.elapsed().as_secs_f64()
    );
    drop(pass);
}

/// The cycle's every module made cold (DESIGN.md section 9 item 3), one
/// way a process (`CYCLE_COMPILE`): one module after another (`seq`),
/// every module at once on its own thread (`modules`), the units'
/// entries each at once on its own thread (`units`), or every program a
/// page's builds dispatch, made as a page makes them (`generator`:
/// `Generator::new`, then `prepare_programs`; fn-256 task 4).
#[test]
#[ignore = "a measurement"]
fn the_cycle_is_made_cold() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let way = std::env::var("CYCLE_COMPILE").unwrap_or_else(|_| "seq".into());
    let done = |started: Instant| {
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("made");
        started.elapsed().as_secs_f64()
    };
    let makers: [(&str, &(dyn Fn() + Sync)); 9] = [
        ("units and wake", &|| {
            drop(super::device::Kernels::new(&gpu))
        }),
        ("layout", &|| drop(super::lay_device::Kernels::new(&gpu))),
        ("hierarchy", &|| drop(super::hier_build::Kernels::new(&gpu))),
        ("light", &|| drop(super::light_device::Kernels::new(&gpu))),
        ("glue", &|| drop(super::glue::Kernels::new(&gpu))),
        ("sizing", &|| drop(super::vm::Kernels::new(&gpu))),
        ("retirement", &|| {
            drop(super::retire_device::Kernels::new(&gpu))
        }),
        ("settle", &|| drop(super::settle_device::Kernels::new(&gpu))),
        ("scan", &|| {
            drop(io::Pass::new(
                &gpu,
                super::hier_scan_source(),
                &[false; 3],
                &["scan_up", "scan_down"],
            ))
        }),
    ];
    let started = Instant::now();
    if way == "parts" {
        let parts: [(&str, &dyn Fn()); 6] = [
            ("build", &|| drop(super::build::Kit::new(&gpu))),
            ("settle", &|| drop(super::settle_device::Kernels::new(&gpu))),
            ("chains", &|| drop(super::chains::Kernels::new(&gpu))),
            ("drawn", &|| drop(super::drawn::Kernels::new(&gpu))),
            ("curve", &|| drop(crate::curve::passes(&gpu).len())),
            ("tree in all", &|| drop(super::tree::Kernels::new(&gpu))),
        ];
        for (name, make) in parts {
            let at = Instant::now();
            make();
            println!("MADE {name}: {:.2} s", done(at));
        }
        return;
    }
    if way == "generator" {
        let renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        let at = Instant::now();
        let gen = super::super::Generator::new(&renderer).expect("a generator");
        println!("MADE the generator's own passes: {:.2} s", done(at));
        pollster::block_on(gen.prepare_programs()).expect("programs");
        println!("MADE generator in all: {:.2} s", done(at));
        return;
    }
    match way.as_str() {
        "seq" => {
            for (name, make) in makers {
                let at = Instant::now();
                make();
                println!("MADE {name}: {:.2} s", done(at));
            }
        }
        "modules" => std::thread::scope(|s| {
            for (_, make) in makers {
                s.spawn(make);
            }
        }),
        _ => std::thread::scope(|s| {
            let (source, reads) = module("units");
            for entry in super::device::ENTRIES {
                let (source, reads, gpu) = (source.clone(), reads.clone(), &gpu);
                s.spawn(move || drop(io::Pass::new(gpu, source, &reads, &[entry])));
            }
        }),
    }
    println!("MADE {way} in all: {:.2} s", done(started));
}
