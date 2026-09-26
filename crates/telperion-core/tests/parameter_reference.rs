//! `docs/parameters.md` is the catalogue rendered, never a copy kept by hand.
//! Run with `TELPERION_WRITE_REFERENCE=1` to rewrite it from the catalogue.
use std::path::PathBuf;
use telperion_core::catalogue;

#[test]
fn every_row_is_in_the_reference() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/parameters.md");
    let rendered = catalogue::reference();
    if std::env::var_os("TELPERION_WRITE_REFERENCE").is_some() {
        std::fs::write(&path, &rendered).expect("docs/parameters.md is writable");
    }
    let written = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        written == rendered,
        "docs/parameters.md is not the catalogue's reference; rerun this test \
         with TELPERION_WRITE_REFERENCE=1"
    );
}
