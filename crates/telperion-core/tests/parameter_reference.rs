//! `docs/parameters.md` and the browser's parameter metadata are the catalogue
//! rendered, never copies kept by hand. Run with `TELPERION_WRITE_REFERENCE=1`
//! to rewrite them from the catalogue.
use std::path::PathBuf;
use telperion_core::catalogue;

/// Rewrites `file` on request, then holds it to what the catalogue renders.
fn rendered(file: &str, rendered: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(file);
    if std::env::var_os("TELPERION_WRITE_REFERENCE").is_some() {
        std::fs::write(&path, rendered).expect("the generated file is writable");
    }
    let written = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        written == rendered,
        "{file} is not what the catalogue renders; rerun this test with \
         TELPERION_WRITE_REFERENCE=1"
    );
}

#[test]
fn every_row_is_in_the_reference() {
    rendered("docs/parameters.md", &catalogue::reference());
}

#[test]
fn the_browser_reads_the_catalogue() {
    rendered("src/browser/parameters.generated.ts", &catalogue::browser());
}
