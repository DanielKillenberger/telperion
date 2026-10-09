# F4 merged into fn-219 (2026-10-09)

fn-219's branch (off fn-218 at `9ebbb3eb`) took F4 (`fn-198-tree-space-f-the-engine-in-production`, with fn-238's running-total size solve).

## Conflict

Only `crates/telperion-core/tests/catalogue_identity.rs`. fn-218 had re-pinned the family and walk digests for Laurelin's rows; fn-238 had re-pinned the walk digest.

- **Header:** both histories kept, fn-218's paragraph then fn-238's, and a paragraph for this merge.
- **Walk digest:** fn-218's `6_900_362_327_311_276_930` and fn-238's `18_193_907_904_263_486_605` became `4_122_176_118_329_417_092`, computed from the merged tree: fn-238's solve over fn-218's rows.
- **Family, override and refusal:** fn-218's values, unchanged; the merged tree gives them.

## Engine presets' grown trees (`space_hash`)

`raw/merge-f4/hash-after.txt`, built on the merged tree with `crates/telperion-render/examples/space_hash.rs`. All 16 (beech, spruce, oak, palm at 40 and 80, seeds 1 and 7) equal MERGE-231.md's and MERGE-217.md's, node counts included. The walk digest was pinned only after they matched.
