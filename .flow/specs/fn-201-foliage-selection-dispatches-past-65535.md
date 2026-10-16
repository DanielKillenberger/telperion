# Foliage selection dispatches past 65,535 workgroups

## Goal

The renderer's foliage selection (`crates/telperion-render/src/select.rs`) dispatched one dimension of `instances / WORKGROUP` groups, and wgpu panics above 65,535 groups (about 4.2M instances): "dispatch group size dimension ([74572, 1, 1]) must be less or equal to 65535" (fn-194 FRICTION.md (archived: see .flow/evidence/ARCHIVE-MANIFEST.md, `git show 8dab10634:.flow/evidence/fn-194-tree-space-d-the-norway-spruce-as-a/FRICTION.md`), 2026-10-04). The spruce had to thin its needles to stay under it, and Astra failed it for missing needle mass. An obvious fix under AGENTS.md "Friction reports" (host, 2026-10-04): the dispatch only, no change to which leaves are selected, LOD or frame budget.

## Decisions

- Split the per-instance dispatches over x and y as `generation/io.rs` already does; the per-level prefix pass stays unchanged.

## Acceptance Criteria

- **R1:** Per-instance select passes split groups over x and y; shaders rebuild the flat index and skip past the end — met.
- **R2:** A test, red on the base, selects past 65,535 workgroups without a panic and matches a flat dispatch below the limit — met.
- **R3:** Every shipped preset renders byte-identically — met.
- **R4:** Workspace gate green — met.

## Result

`grid` in `select.rs` folds workgroups into y once x runs out; `select.wgsl` rebuilds the index from `global_invocation_id` and `num_workgroups`, and a group past the end returns whole so barriers stay uniform. Below the limit the grid is `(groups, 1)`. Test `a_crown_past_one_dispatch_dimension_is_selected_in_order` (65,536 groups of 256 plus 3 leaves) panicked on the base with "([65537, 1, 1])". Gate 1,020 passed. Codex SHIP in round 2.

## Open

None.

## Friction

- Codex reviewer's sandbox has only llvmpipe, so GPU tests skip and it judged R2/R3 on reading -> hand the reviewer the implementer's GPU test logs.
- `conformance.rs` anatomy check fired only on the crate run, not the focused `select::` run -> no fix; run `--test conformance` with focused tests when touching `select.rs`.
