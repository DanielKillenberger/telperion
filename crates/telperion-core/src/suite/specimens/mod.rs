//! Fixed specimens generated once per test run. A fixed specimen is a shipped
//! table at some seed and nothing else moved; every test that grows one reads
//! its skeleton from a cache under the cargo target keyed on the family's
//! wire, its seed and the generator's sources, so a generator change or a
//! preset value change misses and a stale specimen is never served. A family
//! that is not a fixed specimen is grown directly: it is asserted once and
//! never shared.
#![allow(dead_code)]
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::OnceLock,
    time::{Duration, SystemTime},
};

use telperion_core::{
    branching, mesh, params,
    presets::{Family, Preset},
    tree::Tree,
};

/// The cache's root: a directory per sources digest under the crate's target
/// scratch, so entries of another source version are evicted rather than kept.
pub fn directory() -> PathBuf {
    target_scratch()
        .join("specimens")
        .join(format!("{:016x}", sources()))
}

/// `target/tmp`, the scratch cargo gives an integration test, found from this
/// test binary at `target/<profile>/deps/`.
fn target_scratch() -> PathBuf {
    let exe = std::env::current_exe().expect("the test binary has a path");
    let target = exe
        .ancestors()
        .nth(3)
        .expect("a binary under target/<profile>/deps");
    target.join("tmp")
}

/// The skeleton of a family, from the cache when the family is a fixed specimen.
pub fn tree(family: &Family) -> Tree {
    let Some(at) = path(family) else {
        return grow(family);
    };
    if let Some(stored) = load(&at) {
        return stored;
    }
    let _held = Lock::take(&at);
    if let Some(stored) = load(&at) {
        return stored;
    }
    let fresh = grow(family);
    store(&at, &fresh);
    let stored =
        load(&at).unwrap_or_else(|| panic!("{} was stored and cannot be read", at.display()));
    verify(&fresh, &stored, &at).unwrap_or_else(|reason| panic!("{reason}"));
    fresh
}

/// The whole mesh of a family, assembled on its skeleton from the cache.
pub fn mesh(family: &Family) -> mesh::TreeMesh {
    mesh::assemble(&tree(family), family).expect("the table builds a tree")
}

fn grow(family: &Family) -> Tree {
    branching::generate(&family.skeleton, family.radii)
        .expect("the table grows a tree")
        .tree
}

/// The cache path of a family's skeleton, or `None` when the family is not a
/// shipped table at a seed.
pub fn path(family: &Family) -> Option<PathBuf> {
    let metadata = params::metadata(family);
    let wire = metadata.to_string();
    let seed = family.skeleton.seed;
    let fixed = [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::EuropeanBeech,
        Preset::SilverBirch,
        Preset::Telperion,
        Preset::Laurelin,
    ]
    .into_iter()
    .any(|preset| {
        let mut table = preset.parameters();
        table.skeleton.seed = seed;
        params::metadata(&table) == metadata
    });
    fixed.then(|| directory().join(format!("tree-{seed}-{:016x}.bin", fnv(wire.as_bytes()))))
}

/// A stored specimen that is not the fresh one, by its path.
pub fn verify<T: PartialEq>(fresh: &T, stored: &T, at: &Path) -> Result<(), String> {
    (fresh == stored)
        .then_some(())
        .ok_or_else(|| format!("{} loaded back as a different specimen", at.display()))
}

fn load(at: &Path) -> Option<Tree> {
    let bytes = fs::read(at).ok()?;
    Some(
        bincode::deserialize(&bytes)
            .unwrap_or_else(|e| panic!("{} is not a specimen: {e}", at.display())),
    )
}

/// Written whole under another name and renamed into place: a reader sees the
/// specimen or nothing.
fn store(at: &Path, specimen: &Tree) {
    let root = at.parent().expect("a specimen path has a directory");
    evict_other_sources(root);
    fs::create_dir_all(root).expect("the specimen directory");
    let partial = at.with_extension(format!("part{}", std::process::id()));
    let bytes = bincode::serialize(specimen).expect("a specimen serializes");
    fs::write(&partial, bytes).expect("the specimen writes");
    fs::rename(&partial, at).expect("the specimen renames into place");
}

fn evict_other_sources(keep: &Path) {
    let Some(parent) = keep.parent() else { return };
    let Ok(entries) = fs::read_dir(parent) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.path() != keep {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

/// One builder per specimen across the test processes of a run: the first
/// process to miss creates the lock file and builds; a later one waits for the
/// specimen to appear. A lock older than a build could take is a crashed
/// builder's and is taken over.
struct Lock(PathBuf);
impl Lock {
    const STALE: Duration = Duration::from_secs(300);
    fn take(at: &Path) -> Self {
        let lock = at.with_extension("lock");
        fs::create_dir_all(at.parent().expect("a specimen path has a directory"))
            .expect("the specimen directory");
        loop {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&lock)
            {
                Ok(_) => return Self(lock),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                    let stale = fs::metadata(&lock)
                        .and_then(|m| m.modified())
                        .ok()
                        .and_then(|t| SystemTime::now().duration_since(t).ok())
                        .is_some_and(|age| age > Self::STALE);
                    if stale {
                        let _ = fs::remove_file(&lock);
                    } else if at.exists() {
                        return Self(PathBuf::new());
                    } else {
                        std::thread::sleep(Duration::from_millis(100));
                    }
                }
                Err(e) => panic!("{}: {e}", lock.display()),
            }
        }
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        if self.0.as_os_str().is_empty() {
            return;
        }
        let _ = fs::remove_file(&self.0);
    }
}

/// FNV-1a over the generator's sources: every Rust file under the crate's
/// `src`, its manifest and the workspace lock, by path.
fn sources() -> u64 {
    static DIGEST: OnceLock<u64> = OnceLock::new();
    *DIGEST.get_or_init(|| {
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        walk(&crate_root.join("src"), &mut files);
        files.push(crate_root.join("Cargo.toml"));
        files.push(crate_root.join("../../Cargo.lock"));
        files.sort();
        let mut hash = FNV_OFFSET;
        for file in files {
            hash = fnv_from(hash, file.to_string_lossy().as_bytes());
            hash = fnv_from(hash, &fs::read(&file).expect("a generator source reads"));
        }
        hash
    })
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir)
        .expect("the generator sources list")
        .flatten()
    {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

const FNV_OFFSET: u64 = 14695981039346656037;
fn fnv(bytes: &[u8]) -> u64 {
    fnv_from(FNV_OFFSET, bytes)
}
fn fnv_from(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash = (hash ^ u64::from(*byte)).wrapping_mul(1099511628211);
    }
    hash
}
