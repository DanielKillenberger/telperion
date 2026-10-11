//! Every shipped WGSL module parsed and validated on the CPU by naga, as the
//! device builds it: the composed source with its defines, every entry point,
//! under the capabilities the device is granted. A reserved word, a function
//! that can end without returning (fn-209 FRICTION.md: four locked GPU runs)
//! or a local that shadows a builtin (fn-248 FRICTION.md: `floor` in
//! `dress.wgsl`, `length` in `stations.wgsl`, a GPU run each) fails an
//! ordinary test before any GPU is asked for (fn-260 R1).
use std::path::Path;

/// What naga may accept: the capabilities wgpu grants a device asking for
/// the features `device.rs` asks for, on a WebGPU-compliant adapter.
fn capabilities() -> naga::valid::Capabilities {
    wgpu_naga_bridge::features_to_naga_capabilities(
        wgpu::Features::TIMESTAMP_QUERY,
        wgpu::DownlevelFlags::compliant(),
    )
}

fn check(name: &str, source: &str) -> Result<(), String> {
    let module = naga::front::wgsl::parse_str(source)
        .map_err(|e| format!("{name}: {}", e.emit_to_string(source)))?;
    let flags = naga::valid::ValidationFlags::all();
    naga::valid::Validator::new(flags, capabilities())
        .validate(&module)
        .map_err(|e| format!("{name}: {}", e.emit_to_string(source)))?;
    Ok(())
}

/// Every source the device builds: the drawing passes and the generation
/// passes, the fine-root executor's included.
fn shipped() -> Vec<(&'static str, String)> {
    let mut all = crate::shaders::all();
    all.extend(crate::generation::sources::all());
    all
}

#[test]
fn every_shader_parses_and_validates() {
    let failed: Vec<String> = (shipped().iter())
        .filter_map(|(name, source)| check(name, source).err())
        .collect();
    assert!(failed.is_empty(), "{}", failed.join("\n\n"));
}

/// No WGSL file under `src` escapes the check: each is part of a source
/// it validates.
#[test]
fn every_shader_file_is_checked() {
    let all: Vec<String> = shipped().into_iter().map(|s| s.1).collect();
    let mut files = Vec::new();
    let mut open = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
    while let Some(dir) = open.pop() {
        for entry in std::fs::read_dir(dir).expect("readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                open.push(path);
            } else if path.extension().is_some_and(|e| e == "wgsl") {
                files.push(path);
            }
        }
    }
    let unchecked: Vec<String> = (files.iter())
        .filter(|f| {
            let text = std::fs::read_to_string(f).expect("readable");
            !all.iter().any(|s| s.contains(&text))
        })
        .map(|f| f.display().to_string())
        .collect();
    assert!(
        unchecked.is_empty(),
        "not in any checked source: {unchecked:?}"
    );
}

/// Each fixture fails the check, and the same function without its fault
/// passes it, so the fault is what fails.
#[test]
fn a_faulty_fixture_fails_the_check() {
    for (fault, faulty, sound) in [
        (
            "a reserved word",
            "fn f() { let target = 1u; }",
            "fn f() { let aim = 1u; }",
        ),
        (
            "a missing return",
            "fn f(x: u32) -> u32 { if x > 0u { return 1u; } }",
            "fn f(x: u32) -> u32 { if x > 0u { return 1u; } return 0u; }",
        ),
        (
            "a local shadowing `floor`",
            "fn f(x: f32) -> f32 { let floor = x; return floor(x) + floor; }",
            "fn f(x: f32) -> f32 { let low = x; return floor(x) + low; }",
        ),
        (
            "a local shadowing `length`",
            "fn f(v: vec3f) -> f32 { var length = 1.0; return length(v) * length; }",
            "fn f(v: vec3f) -> f32 { var scale = 1.0; return length(v) * scale; }",
        ),
    ] {
        let refused = check(fault, faulty).expect_err(fault);
        println!("{refused}");
        assert!(
            check(fault, sound).is_ok(),
            "{fault}: the sound twin failed"
        );
    }
}

/// Every shipped source written to `SHADERS=<dir>`, one file a source,
/// for a browser's compiler to check (fn-212 task 10: Tint refuses some
/// WGSL naga accepts). `cargo test --profile ci -p telperion-render
/// dump_shaders -- --ignored`.
#[test]
#[ignore]
fn dump_shaders() {
    let dir = std::path::PathBuf::from(std::env::var("SHADERS").expect("SHADERS: a directory"));
    std::fs::create_dir_all(&dir).unwrap();
    for (k, (name, source)) in shipped().iter().enumerate() {
        let file = format!("{k:02}-{}.wgsl", name.replace(' ', "-"));
        std::fs::write(dir.join(file), source).unwrap();
    }
}
