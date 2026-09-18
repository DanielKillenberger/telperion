//! The fixed-specimen cache the core tests draw on: a key miss generates and
//! stores, a stored specimen that is not the fresh one fails by its path, a
//! family that is not a shipped table is never stored, and the cache
//! directory is ignored by git.
mod specimens;
use std::{fs, path::Path, process::Command};

use telperion_core::presets::{Family, Preset};

fn ordinary(seed: u32) -> Family {
    let mut f = Preset::Ordinary.parameters();
    f.skeleton.seed = seed;
    f
}

#[test]
fn a_key_miss_generates_and_stores_and_the_next_read_is_the_stored_specimen() {
    let family = ordinary(76_001);
    let at = specimens::path(&family).expect("a shipped table at a seed is fixed");
    assert_eq!(at.parent(), Some(specimens::directory().as_path()));
    let _ = fs::remove_file(&at);
    let fresh = specimens::tree(&family);
    assert!(at.is_file(), "{} was not stored on the miss", at.display());
    assert_eq!(
        specimens::tree(&family),
        fresh,
        "the stored specimen is not the fresh one"
    );
}

#[test]
fn a_family_that_is_not_a_shipped_table_is_built_and_never_stored() {
    let mut family = ordinary(76_002);
    family.skeleton.growth.max_nodes = Some(2_000);
    assert_eq!(specimens::path(&family), None);
    assert!(specimens::tree(&family).nodes.len() <= 2_000);
}

#[test]
fn a_stored_specimen_that_differs_from_the_fresh_one_fails_by_its_path() {
    let at = Path::new("target/tmp/specimens/0/tree-7-0.bin");
    assert_eq!(specimens::verify(&1, &1, at), Ok(()));
    assert_eq!(
        specimens::verify(&1, &2, at),
        Err("target/tmp/specimens/0/tree-7-0.bin loaded back as a different specimen".into())
    );
}

#[test]
fn the_cache_directory_is_ignored_by_git() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let directory = specimens::directory();
    let inside = directory.starts_with(repo.canonicalize().unwrap());
    let ignored = Command::new("git")
        .args(["-C", repo.to_str().unwrap(), "check-ignore", "-q"])
        .arg(&directory)
        .status()
        .expect("git runs")
        .success();
    assert!(!inside || ignored, "{} is not ignored", directory.display());
}
