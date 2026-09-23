//! Bakes the pipeline's build identity into the crate (fn-131).

include!("src/pipeline/tree_digest.rs");

fn main() {
    let root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=data");
    println!("cargo:rustc-env=TELPERION_JEV_BUILD={}", tree_digest(&root));
}
