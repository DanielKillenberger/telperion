"""fn-291 task 2: a throwaway traced wasm build of the CPU entry.

`apply <root> <save-dir>` saves the files it edits into <save-dir> and
patches them so every wood reservation of at least 16 MiB, the end of
growth and (task 5, where the chunks reserve under 16 MiB each) the end of
the wood's mesh step, records (where, MiB requested, Wasm MiB before, Wasm MiB after) into
a static the entry's metadata returns as `trace`. `revert <root> <save-dir>`
copies the saved files back. Build the binding into a scratch target between
the two, copy it over a scratch copy of dist/ as telperion.wasm and run
`probe-dist.mjs --case <scratch dist> ...`; the crate is never committed
patched.
"""
import shutil
import sys
from pathlib import Path

FILES = {
    "crates/telperion-core/src/lib.rs": [(
        "pub mod pipeline;\n",
        "pub mod pipeline;\n"
        "#[doc(hidden)]\n"
        "pub static WASM_TRACE: std::sync::Mutex<Vec<(String, u64, u64, u64)>> =\n"
        "    std::sync::Mutex::new(Vec::new());\n"
        "#[doc(hidden)]\n"
        "pub fn wasm_mib() -> u64 {\n"
        "    #[cfg(target_arch = \"wasm32\")]\n"
        "    return core::arch::wasm32::memory_size(0) as u64 * 65536 / 1048576;\n"
        "    #[cfg(not(target_arch = \"wasm32\"))]\n"
        "    0\n"
        "}\n"
        "#[doc(hidden)]\n"
        "pub fn wasm_mark(at: String, mib: u64, before: u64) {\n"
        "    if let Ok(mut t) = WASM_TRACE.lock() { t.push((at, mib, before, wasm_mib())); }\n"
        "}\n",
    )],
    "crates/telperion-core/src/pipeline/surface.rs": [(
        "fn reserved<T>(n: usize) -> Result<Vec<T>> {\n"
        "    let mut out = Vec::new();\n"
        "    out.try_reserve_exact(n)\n"
        "        .map_err(|_| Error::ResourceLimit(\"surface allocation\"))?;\n",
        "#[track_caller]\n"
        "fn reserved<T>(n: usize) -> Result<Vec<T>> {\n"
        "    let at = core::panic::Location::caller();\n"
        "    let (mib, before) = ((n * size_of::<T>()) as u64 >> 20, crate::wasm_mib());\n"
        "    let mut out = Vec::new();\n"
        "    let r = out.try_reserve_exact(n);\n"
        "    if mib >= 16 { crate::wasm_mark(format!(\"{}:{}{}\", at.file().rsplit('/').next().unwrap_or(\"\"), at.line(), if r.is_err() { \" FAILED\" } else { \"\" }), mib, before); }\n"
        "    r.map_err(|_| Error::ResourceLimit(\"surface allocation\"))?;\n",
    )],
    "crates/telperion-core/src/pipeline.rs": [(
        "    let inputs = Inputs::of(family, grown_envelope(&skeleton.tree));\n",
        "    crate::wasm_mark(\"grown\".into(), 0, crate::wasm_mib());\n"
        "    let inputs = Inputs::of(family, grown_envelope(&skeleton.tree));\n",
    )],
    "crates/telperion-core/src/pipeline/drawn.rs": [(
        "    let wood = wood?;\n",
        "    crate::wasm_mark(\"wood\".into(), 0, crate::wasm_mib());\n    let wood = wood?;\n",
    )],
    "crates/telperion-wasm/src/generate.rs": [(
        "    let meta = json!({\n",
        "    let trace: Vec<_> = telperion_core::WASM_TRACE.lock().map(|mut t| std::mem::take(&mut *t)).unwrap_or_default();\n"
        "    let meta = json!({\n        \"trace\": trace,\n",
    )],
}


def apply(root: Path, save: Path) -> None:
    save.mkdir(parents=True, exist_ok=True)
    for rel, edits in FILES.items():
        path = root / rel
        shutil.copy2(path, save / rel.replace("/", "__"))
        text = path.read_text()
        for old, new in edits:
            if text.count(old) != 1:
                raise SystemExit(f"{rel}: anchor not found once")
            text = text.replace(old, new)
        path.write_text(text)


def revert(root: Path, save: Path) -> None:
    for rel in FILES:
        shutil.copy2(save / rel.replace("/", "__"), root / rel)


if __name__ == "__main__":
    {"apply": apply, "revert": revert}[sys.argv[1]](Path(sys.argv[2]), Path(sys.argv[3]))
