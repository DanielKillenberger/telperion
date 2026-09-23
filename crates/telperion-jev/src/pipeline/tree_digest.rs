/// A digest of every file under `src` and `data` of the crate at `root`,
/// in path order: the pipeline's code and its question sets, cases and
/// tables. Shared by the build script, which bakes it in as `BUILD_ID`, and
/// by the test that checks the baked value against the tree.
pub fn tree_digest(root: &std::path::Path) -> String {
    use sha2::{Digest, Sha256};
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    let mut files = Vec::new();
    walk(&root.join("src"), &mut files);
    walk(&root.join("data"), &mut files);
    let mut named: Vec<(String, std::path::PathBuf)> = files
        .into_iter()
        .filter_map(|p| {
            let rel = p.strip_prefix(root).ok()?.to_string_lossy().replace('\\', "/");
            Some((rel, p))
        })
        .collect();
    named.sort();
    let mut hasher = Sha256::new();
    for (rel, path) in named {
        let bytes = std::fs::read(&path).unwrap_or_default();
        hasher.update(rel.as_bytes());
        hasher.update([0]);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    hasher
        .finalize()
        .iter()
        .take(12)
        .map(|b| format!("{b:02x}"))
        .collect()
}
