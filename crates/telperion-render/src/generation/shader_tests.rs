//! Every generation shader parsed and validated on the CPU by naga, so a
//! reserved word or a function that can end without returning fails an
//! ordinary test before any GPU is asked for (fn-209 FRICTION.md, task 2:
//! four locked GPU runs were spent finding them).
use std::path::Path;

fn check(name: &str, source: &str) -> Result<(), String> {
    let module = naga::front::wgsl::parse_str(source)
        .map_err(|e| format!("{name}: {}", e.emit_to_string(source)))?;
    let flags = naga::valid::ValidationFlags::all();
    let caps = naga::valid::Capabilities::all();
    naga::valid::Validator::new(flags, caps)
        .validate(&module)
        .map_err(|e| format!("{name}: {}", e.emit_to_string(source)))?;
    Ok(())
}

#[test]
fn every_generation_shader_parses_and_validates() {
    let failed: Vec<String> = super::sources::all()
        .iter()
        .filter_map(|(name, source)| check(name, source).err())
        .collect();
    assert!(failed.is_empty(), "{}", failed.join("\n\n"));
}

#[test]
fn every_generation_shader_file_is_checked() {
    let all: Vec<String> = super::sources::all().into_iter().map(|s| s.1).collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generation");
    let mut files = Vec::new();
    let mut open = vec![root];
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
    let unchecked: Vec<String> = files
        .iter()
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

#[test]
fn a_reserved_word_fails_the_parse() {
    assert!(check("probe", "fn f() { let target = 1u; }").is_err());
    assert!(check("probe", "fn f() -> u32 { let x = 1u; return x; }").is_ok());
    assert!(check("probe", "fn f(x: u32) -> u32 { if x > 0u { return 1u; } }").is_err());
}
