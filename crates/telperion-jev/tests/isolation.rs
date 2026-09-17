use std::path::PathBuf;

use telperion_jev::isolation::{endpoint_marker, scan, IsolationHit};
use telperion_jev::ENDPOINT;

#[test]
fn generation_path_does_not_import_the_caller_or_name_the_endpoint() {
    assert_eq!(endpoint_marker(), ENDPOINT);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let hits = scan(root);
    assert!(
        hits.is_empty(),
        "{}",
        hits.iter()
            .map(|hit| hit.message())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn failure_names_the_file_and_the_rule() {
    let hit = IsolationHit {
        file: PathBuf::from("crates/telperion-core/src/tree.rs"),
        marker: ENDPOINT.to_string(),
    };
    let message = hit.message();
    assert!(
        message.contains("crates/telperion-core/src/tree.rs"),
        "{message}"
    );
    assert!(
        message.contains("Jev never runs in generation, rendering, presets or the browser source"),
        "{message}"
    );
}

#[test]
fn a_render_subdirectory_of_the_browser_source_is_scanned() {
    let root = std::env::temp_dir().join(format!(
        "jev-isolation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let dir = root.join("src/browser/render");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("bindings.ts"),
        format!("const endpoint = \"{ENDPOINT}\";\n"),
    )
    .unwrap();

    let hits = scan(&root);
    std::fs::remove_dir_all(&root).ok();

    assert_eq!(
        hits,
        vec![IsolationHit {
            file: PathBuf::from("src/browser/render/bindings.ts"),
            marker: ENDPOINT.to_string(),
        }]
    );
}
