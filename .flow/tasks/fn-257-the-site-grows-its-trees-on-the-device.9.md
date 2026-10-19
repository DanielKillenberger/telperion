---
satisfies: [R7]
---
# fn-257-the-site-grows-its-trees-on-the-device.9 Growth's tables narrowed losslessly where they are created

## Description
The owner (spec, "Compact from the start"): during growth only lossless narrowing (counts, indices, flags and ids in the narrowest width their range allows, packed bit-fields, padding removed), the trees byte-identical, each preset's peak and warm time measured before and after; growth's own floats not quantised here, their saving and risk reported for fn-285. Growth's tables are every build's peak (LAYOUT.md: the spruce's 1,572 MB, its pencils 275 MB, axes 227, counts 211, units 153, phytomers laid 145).

**Touches:** WIDER: `crates/telperion-render/src/generation/cycle/` (the state tables' layout in `state.rs`, `columns.rs`, the census and the kernels that read the narrowed columns), its tests; evidence `TABLES.md`.

Order:
1. **Inventory first (`TABLES.md`):** each growth table: rows, words a row, bytes at the spruce's 40 and Telperion's 82 peaks; each column's type, its range (from its producer: an index under the table's rows, a flag, a count under a stated bound, a float), the narrowest lossless width, and the saving. Floats: what a half or a quantised width would save, and which kernels' arithmetic would read it (reported for fn-285, not taken).
2. **Then the narrowing, largest saving first:** each step one table, every reader and writer of its columns changed together, every shipped preset's tree byte-identical (the agreement and repeat tests), the gate's device tests green; stop when the next step saves under 2 % of the spruce's peak.

## Acceptance
- `TABLES.md`: the inventory; each step taken with the spruce's and Telperion's peak and warm `tree::drawn` before and after (paired, warm); the float quantisation's saving and risk for fn-285.
- Every shipped preset's tree byte-identical to the base at seeds 1, 7, 42; harness build times no worse.

## Done summary
A zone's node order written a byte a node (spruce peak 1,624.5 -> 1,586.8 MB, Telperion 2,740.3 -> 2,690.0), every preset byte-identical at seeds 1, 7, 42, warm time unchanged; TABLES.md inventories every growth table; the next lossless step is under 2 % and stops; float quantisation reported for fn-285. Codex SHIP after a recorded round reset.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4dc918a82
- Tests: drawn_hashes 24 trees, drawn_warm paired 16 runs
- PRs: