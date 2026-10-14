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
pub(crate) fn shipped() -> Vec<(&'static str, String)> {
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

/// The storage buffers a stage binds past the device's per-stage limit,
/// the one `Gpu::request` asks for (WebGPU's default 8, within Chrome's
/// 16; fn-237 R14, fn-266 FRICTION.md). A generation pass binds every
/// buffer its module declares in one layout (`io::Pass`), so `whole`
/// counts the module; a drawing pipeline's layout holds what its entry
/// uses.
fn storage_over_limit(name: &str, source: &str, whole: bool) -> Option<String> {
    let module = naga::front::wgsl::parse_str(source).ok()?;
    let flags = naga::valid::ValidationFlags::all();
    let info = naga::valid::Validator::new(flags, capabilities())
        .validate(&module)
        .ok()?;
    let storage: Vec<_> = (module.global_variables.iter())
        .filter(|(_, g)| matches!(g.space, naga::AddressSpace::Storage { .. }))
        .map(|(h, _)| h)
        .collect();
    let limit = wgpu::Limits::default().max_storage_buffers_per_shader_stage as usize;
    let mut counts = vec![("module".to_string(), storage.len())];
    if !whole {
        counts = (module.entry_points.iter().enumerate())
            .map(|(i, e)| {
                let used = info.get_entry_point(i);
                (
                    e.name.clone(),
                    storage.iter().filter(|&&h| !used[h].is_empty()).count(),
                )
            })
            .collect();
    }
    let over: Vec<String> = (counts.into_iter())
        .filter(|(_, n)| *n > limit)
        .map(|(at, n)| format!("{name} {at}: {n} storage buffers, the limit is {limit}"))
        .collect();
    (!over.is_empty()).then(|| over.join("\n"))
}

#[test]
fn every_shader_holds_the_storage_buffer_limit() {
    let drawing = crate::shaders::all().into_iter().map(|s| (s, false));
    let generation = crate::generation::sources::all()
        .into_iter()
        .map(|s| (s, true));
    let over: Vec<String> = (drawing.chain(generation))
        .filter_map(|((name, source), whole)| storage_over_limit(name, &source, whole))
        .collect();
    assert!(over.is_empty(), "{}", over.join("\n"));
    let limit = wgpu::Limits::default().max_storage_buffers_per_shader_stage as usize;
    let module = |n: usize| {
        let globals: String = (0..n)
            .map(|i| {
                format!("@group(0) @binding({i}) var<storage, read_write> b{i}: array<u32>;\n")
            })
            .collect();
        let uses: String = (0..n).map(|i| format!("b{i}[0] = 1u;")).collect();
        format!("{globals}@compute @workgroup_size(1) fn main() {{ {uses} }}")
    };
    for whole in [true, false] {
        assert!(storage_over_limit("at the limit", &module(limit), whole).is_none());
        assert!(storage_over_limit("past it", &module(limit + 1), whole).is_some());
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

/// Every shipped module through Tint itself, the WGSL compiler in Chrome
/// (Dawn's `tint`, `--format spirv` an entry, as Dawn makes a pipeline on
/// Vulkan), for what naga accepts and Chrome refuses (fn-237 R14): a NaN
/// constant, a barrier under non-uniform control, `|` beside `<<` without
/// parentheses. A release check, since the binary is fetched, not built
/// (docs/gpu-tests.md): `TINT=<dawn>/bin/tint cargo test --profile ci -p
/// telperion-render every_shader_passes_tint -- --ignored`.
#[test]
#[ignore = "release check: Tint itself, TINT=<Dawn's tint>"]
fn every_shader_passes_tint() {
    let tint = std::env::var("TINT").expect("TINT: Dawn's tint binary (docs/gpu-tests.md)");
    let dir = std::env::temp_dir().join(format!("telperion-tint-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let refused = |name: &str, source: &str| -> Vec<String> {
        let file = dir.join(format!("{}.wgsl", name.replace(' ', "-")));
        std::fs::write(&file, source).unwrap();
        let module = naga::front::wgsl::parse_str(source).expect("naga parses it");
        (module.entry_points.iter())
            .filter_map(|e| {
                let out = std::process::Command::new(&tint)
                    .args(["--format", "spirv", "-o", "/dev/null", "--ep", &e.name])
                    .arg(&file)
                    .output()
                    .expect("TINT runs");
                let said = String::from_utf8_lossy(&out.stderr);
                (!out.status.success()).then(|| format!("{name} {}: {said}", e.name))
            })
            .collect()
    };
    let head = "@group(0) @binding(0) var<storage, read_write> grid: array<u32>;\n";
    for (fault, faulty, sound) in [
        (
            "a NaN constant",
            "fn main() { grid[0] = bitcast<u32>(bitcast<f32>(0x7fc00000u)); }",
            "fn main() { grid[0] = bitcast<u32>(bitcast<f32>(0x3f800000u)); }",
        ),
        (
            "a barrier under a storage bound",
            "fn main() { for (var i = 0u; i < grid[1]; i++) { workgroupBarrier(); } }",
            "fn main() { for (var i = 0u; i < 4u; i++) { workgroupBarrier(); } }",
        ),
        (
            "or beside shift",
            "fn main() { grid[0] = grid[1] | grid[2] << 2u; }",
            "fn main() { grid[0] = grid[1] | (grid[2] << 2u); }",
        ),
    ] {
        let entry = "@compute @workgroup_size(64) ";
        let faulty = refused(fault, &format!("{head}{entry}{faulty}"));
        assert_eq!(faulty.len(), 1, "{fault}: Tint did not refuse it");
        assert!(
            refused(fault, &format!("{head}{entry}{sound}")).is_empty(),
            "{fault}: the sound twin"
        );
    }
    let failed: Vec<String> = (shipped().iter())
        .flat_map(|(name, source)| refused(name, source))
        .collect();
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(failed.is_empty(), "Tint refuses:\n{}", failed.join("\n"));
}
