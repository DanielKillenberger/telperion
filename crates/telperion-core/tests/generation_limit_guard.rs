//! Deliberately small lexical guard: review limit-shaped expressions, not every number.
use std::{collections::BTreeMap, fs, path::Path};

fn tokens(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        if bytes[i].is_ascii_whitespace() {
            i += 1;
        } else if bytes[i..].starts_with(b"//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
        } else if bytes[i..].starts_with(b"/*") {
            i += 2;
            let mut depth = 1;
            while i < bytes.len() && depth > 0 {
                if bytes[i..].starts_with(b"/*") {
                    depth += 1;
                    i += 2;
                } else if bytes[i..].starts_with(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
        } else if bytes[i] == b'"' {
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i += 2;
                } else if bytes[i] == b'"' {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            out.push("STRING".into());
        } else if bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' {
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            out.push(source[start..i].into());
        } else {
            i += 1;
            out.push(source[start..i].into());
        }
    }
    out
}

fn close(t: &[String], start: usize, left: &str, right: &str) -> usize {
    let mut depth = 0;
    for (i, token) in t.iter().enumerate().skip(start) {
        if token == left {
            depth += 1;
        }
        if token == right {
            depth -= 1;
            if depth == 0 {
                return i;
            }
        }
    }
    t.len() - 1
}

fn production_tokens(source: &str) -> Vec<String> {
    let mut t = tokens(source);
    let marker = ["#", "[", "cfg", "(", "test", ")", "]"];
    while let Some(start) = t
        .windows(marker.len())
        .position(|s| s.iter().map(String::as_str).eq(marker))
    {
        let mut nested = 0;
        let body = (start + marker.len()..t.len()).find(|&i| {
            match t[i].as_str() {
                "(" | "[" => nested += 1,
                ")" | "]" => nested -= 1,
                "{" | ";" | "," if nested == 0 => return true,
                _ => {}
            }
            false
        });
        let end = body
            .map(|i| {
                if t[i] == "{" {
                    close(&t, i, "{", "}")
                } else {
                    i
                }
            })
            .unwrap_or(t.len() - 1);
        t.drain(start..=end);
    }
    t
}

fn candidates(source: &str) -> Vec<String> {
    let t = production_tokens(source);
    let named = |s: &str| {
        s.chars().any(|c| c.is_ascii_uppercase())
            && s.chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    };
    let loop_names: Vec<_> = t
        .iter()
        .enumerate()
        .filter(|(_, s)| *s == "for" || *s == "while")
        .flat_map(|(i, _)| t[i..].iter().take_while(|s| *s != "{"))
        .filter(|s| named(s))
        .collect();
    let mut sites = Vec::new();
    for i in 0..t.len() {
        let mut end = None;
        if t[i] == "."
            && i + 2 < t.len()
            && matches!(
                t[i + 1].as_str(),
                "min" | "max" | "clamp" | "take" | "truncate" | "with_limit"
            )
            && t[i + 2] == "("
        {
            let last = close(&t, i + 2, "(", ")");
            if t[i + 3..last]
                .iter()
                .any(|s| s.starts_with(|c: char| c.is_ascii_digit()) || named(s))
            {
                end = Some(last);
            }
        } else if t[i] == "const" && i + 1 < t.len() {
            let name = &t[i + 1];
            if ["CAP", "BUDGET", "LIMIT", "CEILING", "MAX_", "MIN_"]
                .iter()
                .any(|word| name.contains(word))
                || loop_names.contains(&name)
            {
                end = (i..t.len()).find(|&j| t[j] == ";");
            }
        } else if t[i] == "for" {
            if let Some(last) = (i..t.len()).find(|&j| t[j] == "{") {
                let fixed = (i..last.saturating_sub(2)).any(|j| {
                    let bound = j + 2 + usize::from(t[j + 2] == "=");
                    t[j] == "."
                        && t[j + 1] == "."
                        && bound < last
                        && t[bound..last]
                            .iter()
                            .any(|s| s.starts_with(|c: char| c.is_ascii_digit()) || named(s))
                });
                let array = t[i..last].windows(2).any(|w| w[0] == "in" && w[1] == "[");
                if fixed && !array {
                    end = Some(last);
                }
            }
        } else if t[i] == "if" || t[i] == "while" {
            if let Some(last) = (i..t.len()).find(|&j| t[j] == "{") {
                let comparison = t[i..last].iter().any(|s| matches!(s.as_str(), "<" | ">"));
                let literal = t[i..last]
                    .iter()
                    .any(|s| s.starts_with(|c: char| c.is_ascii_digit()));
                let body_end = close(&t, last, "{", "}");
                let stops = t[last..=body_end]
                    .iter()
                    .any(|s| s == "break" || s == "return");
                if comparison && literal && (stops || t[i] == "while") {
                    end = Some(last);
                }
            }
        }
        if let Some(end) = end {
            sites.push(t[i..=end].join(" "));
        }
    }
    sites
}

fn sources(root: &Path, at: &Path, out: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(at).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap();
        if name == "tests"
            || name == "suite"
            || name == "suite.rs"
            || name == "examples"
            || name.ends_with("_tests.rs")
            || name == "tests.rs"
            || test_module(&path)
        {
            continue;
        }
        if path.is_dir() {
            sources(root, &path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            for site in candidates(&fs::read_to_string(&path).unwrap()) {
                out.push((relative.clone(), site));
            }
        }
    }
}

fn test_module(path: &Path) -> bool {
    let parent = path.parent().unwrap();
    let module = path.file_stem().unwrap().to_str().unwrap();
    let marker = ["#", "[", "cfg", "(", "test", ")", "]", "mod", module, ";"];
    [
        parent.with_extension("rs"),
        parent.join("mod.rs"),
        parent.join("lib.rs"),
    ]
    .iter()
    .filter_map(|p| fs::read_to_string(p).ok())
    .any(|s| {
        tokens(&s)
            .windows(marker.len())
            .any(|w| w.iter().map(String::as_str).eq(marker))
    })
}

fn current() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut out = Vec::new();
    for source in ["crates/telperion-core/src", "crates/telperion-wasm/src"] {
        sources(root, &root.join(source), &mut out);
    }
    out.sort();
    out
}

#[test]
fn generation_limits_are_classified() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let inventory: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("docs/generation-limits-inventory.json")).unwrap(),
    )
    .unwrap();
    let mut declared = BTreeMap::new();
    for entry in inventory.as_array().unwrap() {
        let path = entry["path"].as_str().unwrap().to_owned();
        let site = entry["site"].as_str().unwrap().to_owned();
        assert!(!entry["reason"].as_str().unwrap().trim().is_empty());
        assert!(matches!(
            entry["classification"].as_str().unwrap(),
            "parameter" | "numeric" | "algorithm" | "excluded-fn31"
        ));
        *declared.entry((path, site)).or_insert(0usize) += 1;
    }
    let mut actual = BTreeMap::new();
    for site in current() {
        *actual.entry(site).or_insert(0usize) += 1;
    }
    let unknown: Vec<_> = actual
        .iter()
        .filter(|(site, n)| declared.get(*site) != Some(*n))
        .collect();
    assert!(unknown.is_empty(), "Unclassified generation limit sites: {unknown:#?}. Put authored limits in ranges.rs and the parameter schema, or declare a reviewed numeric tolerance/nonlimiting algorithm in docs/generation-limits-inventory.json.");
    assert_eq!(
        actual, declared,
        "Remove stale generation-limit inventory sites"
    );
}

#[test]
fn new_literal_limits_are_detected() {
    for code in [
        "let n = requested.min(250_000);",
        "const NODE_CEILING: usize = 250_000;",
        "for i in 0..4096 { grow(i); }",
        "if nodes.len() >= 4096 { break; }",
        "for i in 0..count.saturating_mul(64) { grow(i); }",
        "while count < 4096 { grow(); }",
        "const STEPS: usize = 4096; for i in 0..STEPS { grow(i); }",
    ] {
        assert!(!candidates(code).is_empty(), "missed mutation: {code}");
    }
    assert!(
        candidates("// x.min(250_000)\n#[cfg(test)] mod tests { for i in 0..4096 {} }").is_empty()
    );
    assert_eq!(candidates("#[cfg(test)] fn fixture(a: u32, b: u32) -> u32 { a.min(10) } fn production() { work.min(250_000); }").len(), 1);
}

#[test]
#[ignore = "inventory review helper"]
fn list_candidates() {
    println!("{}", serde_json::to_string(&current()).unwrap());
}
