//! The pins a species' catalogue folder records.
//!
//! The identity pins, the leaf band and the growth reference are held twice
//! while the catalogue lands: once in these tests' own source and once in
//! `catalogue/<id>/pins.json`. Each test asserts the two agree and names the
//! species that disagrees. fn-77 deletes the source copies and leaves the file
//! as the only home.
#![allow(dead_code)]
use std::path::PathBuf;

use serde_json::Value;

/// The catalogue directory, resolved from the crate rather than the cwd.
pub fn catalogue() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalogue")
}

/// One species' pins record.
pub fn pins(species: &str) -> Value {
    let path = catalogue().join(species).join("pins.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{species}: {} unreadable: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("{species}: {} is not JSON: {error}", path.display()))
}

/// A whole number the record holds under `field`.
pub fn whole(record: &Value, field: &str, species: &str) -> usize {
    record[field]
        .as_u64()
        .unwrap_or_else(|| panic!("{species}: pins.json {field} is not a whole number"))
        as usize
}

/// A number the record holds under `field`.
pub fn number(record: &Value, field: &str, species: &str) -> f64 {
    record[field]
        .as_f64()
        .unwrap_or_else(|| panic!("{species}: pins.json {field} is not a number"))
}

/// Three numbers the record holds under `field`.
pub fn triple(record: &Value, field: &str, species: &str) -> [f64; 3] {
    let values = record[field]
        .as_array()
        .unwrap_or_else(|| panic!("{species}: pins.json {field} is not a list"));
    assert_eq!(
        values.len(),
        3,
        "{species}: pins.json {field} is not three numbers"
    );
    let mut out = [0.0; 3];
    for (slot, value) in out.iter_mut().zip(values) {
        *slot = value
            .as_f64()
            .unwrap_or_else(|| panic!("{species}: pins.json {field} holds a non-number"));
    }
    out
}

/// A u64 hash the record holds under `field`. A hash does not survive a JSON
/// double, so the file writes it as digits and this reads them back.
pub fn hash(record: &Value, field: &str, species: &str) -> u64 {
    record[field]
        .as_str()
        .unwrap_or_else(|| panic!("{species}: pins.json {field} is not written as digits"))
        .parse()
        .unwrap_or_else(|error| panic!("{species}: pins.json {field} is not a u64: {error}"))
}
