# fn-335 friction

## 9 Nov 2026, worker (Opus): the recorder's text log is large

- Doing: the R1 dry run, one tree per preset at seed 1001, one `REC` line per lateral with its subtree's per-age arrays.
- Slowed by: the spruce prints 810,017 records, 134 MB for one seed; twelve seeds would write about 1.6 GB to `raw/`, and the Python reader holds it all. The oak, birch and beech are 23, 22 and 3 MB.
- Cost: none yet, about a minute of reading; the counting read would pay it on every seed.
- Would remove it: once the protocol fixes the fine ages and the quantities, the probe prints only the frozen fields of the fine ages (spruce age 12 alone is 787k of the 810k records), or aggregates in Rust.

## 9 Nov 2026, worker (Opus): the shell hook and the local rustfmt

- Doing: appending a test with a heredoc, and restoring the crates after saving the patch.
- Slowed by: dcg refused `cat >> file <<EOF` under a home path and `git checkout -- crates`; the reverse `git apply -R` of the saved patch worked. `cargo fmt --check` flags files already on the branch (`examples/early.rs` and others), so the local rustfmt disagrees with the committed style and cannot check the patch's own files.
- Cost: about 3 minutes.
- Would remove it: nothing in the repository for the hook; the rustfmt mismatch is likely a local toolchain difference, reported, not specced.
