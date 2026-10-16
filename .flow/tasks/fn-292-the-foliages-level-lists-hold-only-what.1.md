---
satisfies: [R1]
---
# fn-292-the-foliages-level-lists-hold-only-what.1 Lay the foliage's level lists end to end, each level's start read by its draw

## Description
One list of `instances` words replaces one list per level; the scatter writes each leaf at its level's start (the sum of the lower levels' final counts) plus its offset, and each level's start word into a start buffer whose slots sit at `min_storage_buffer_offset_alignment`. The draw group binds the whole list and the start buffer with the dynamic offset; the foliage vertex shader reads `list[start + instance_index]`. Leaf order within a level is unchanged.

**Size:** M
**Files:** `crates/telperion-render/src/select.rs`, `crates/telperion-render/src/select/bind.rs`, `crates/telperion-render/src/select/frame.rs`, `crates/telperion-render/src/shaders/select.wgsl`, `crates/telperion-render/src/shaders/foliage.wgsl`, `crates/telperion-render/src/foliage.rs`, `crates/telperion-render/src/submit.rs`, tests
**Touches:** [crates/telperion-render/src/select.rs, crates/telperion-render/src/select/**, crates/telperion-render/src/shaders/select.wgsl, crates/telperion-render/src/shaders/foliage.wgsl, crates/telperion-render/src/shaders/shadow.wgsl, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/submit.rs, crates/telperion-render/tests/submit.rs, crates/telperion-render/src/generation/resident/stalk_tests.rs, crates/telperion-render/src/generation/resident/**_tests.rs]

### Approach
- `sizes()` (`select.rs:65-76`): lists = align(instances × 4); a new start buffer = levels × alignment; scratch unchanged. Keep the buffer labels ("foliage level lists", "foliage selection scratch") and add one for the starts so the ledger names it.
- `scatter` in `select.wgsl`: start = sum of `counts[0..level)` (at most 16 reads; counts are final after `prefix`); the leaf with index 0 writes every level's start slot. Keep `leaf_index` / row folding for > 65,535 groups.
- `select/bind.rs`: the draw layout's list binding loses `has_dynamic_offset` and its `stride` window; a new start binding takes the dynamic offset. `bind_level` passes `alignment × level`; the leaf view (`bind_leaf`) and the shadow pass (`foliage.rs` `draw_shadow`, which shares the layout and reads only placements) bind slot 0 / a valid offset.
- `frame::Uniforms.stride` and foliage `frame.stride` (`select.rs:~420-428`): repurpose or remove; check both consumers.
- `submit::fits` (`submit.rs:60-120`) judges the new sizes by the same names.
- **First, on the base (its own commit, before the layout changes):** an ignored measurement test (beside `generation/resident/drawn_memory_tests.rs`) that builds every shipped preset at its default age, seed 1, renders the hero pose and the close pose at fixed sizes, reads each level's list back in its drawn sequence (a new test-only readback that copies the lists and counts and slices level k at its offset for `counts[k]` words: `k × stride` on the base, the level's start after; `chosen()` is not used, since it returns membership by placement and drops the order), and prints per preset, view and level the count and a hash of the ordered ids. Run it on the base, keep the output in `raw/before/levels.txt`; after the change it must print the same lines.
- `chosen()` / `counted` test helpers (`select.rs:~485-499`) read each level from its start (read the start buffer back).

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/select.rs` — sizes, setup, reset, frame encode, bind_level, chosen
- `crates/telperion-render/src/shaders/select.wgsl` — select / prefix / scatter
- `crates/telperion-render/src/select/bind.rs` — compute and draw layouts
- `crates/telperion-render/src/shaders/foliage.wgsl:12,49-54` — the list read
- `crates/telperion-render/src/foliage.rs:~320-356` — draws and shadow

**Optional**:
- `crates/telperion-render/src/select/tests.rs` — compaction tests to extend
- `crates/telperion-render/tests/submit.rs:~168-195` — admission names

### Key context
- WebGPU core has no indirect first instance; the start must reach the shader through a binding.
- The unseen bucket is counter index `levels`; it has a scratch slot and no list.
- The renderer's still is the proof of identity: render the base and the change at the same views and compare pixels (task 2 does every preset).
## Acceptance
- [ ] Lists sized `align(instances × 4)` once, not per level; a start buffer of `levels` aligned slots; `select/tests.rs`'s size test and `tests/submit.rs`'s admission cases updated to the new sizes and names.
- [ ] Every existing selection test passes reading each level from its start (`compaction_preserves_placement_order_at_every_level`, the > 65,535-group crown, the forced level, inflorescences, thin rachis).
- [ ] New test: a frame where one level gets no leaves, its start equals the next level's and the other levels' lists are unchanged; and a forced level starts at 0.
- [ ] The shadow pass and the leaf view draw with the new layout (`tests/shadow.rs`, `tests/thinning.rs`, `tests/conformance.rs`, `tests/walk.rs` green).
- [ ] Early proof: the ordered-leaf-id measurement prints identical lines before (`raw/before/levels.txt`, run on the base commit) and after (`raw/after/levels.txt`) for every preset, hero and close; the spruce's and Laurelin's whole and close stills are pixel-identical to `raw/before/`. Either failing stops task 2.
- [ ] Focused crate tests green; no full gate here (task 2).
## Done summary
The foliage's level lists are now one list of a word per leaf, levels end to end: the scatter writes each leaf at its level's start (the sum of the lower levels' final counts) and each start word into a "foliage level starts" buffer of `min_storage_buffer_offset_alignment` slots, and the draw binds that word at the level's dynamic offset so the vertex shader reads `list[start + instance]`. The early proof passes: the ordered leaf ids per preset, view, layer and level are identical before and after (raw/before/levels.txt vs raw/after/levels.txt, 90 lines), and the spruce's and Laurelin's whole, mid and close stills are byte-identical to raw/before.

- baseline: green (both Quick commands, pre-layout-change, at 8fb9fa759)
- measurement test committed first on the base layout (8fb9fa759, `the_drawn_levels`, ignored); layout change in 47805c6d1
- tests: `the_levels_share_one_list_the_crown_fills_once`, `an_empty_level_starts_where_the_next_does_and_a_forced_one_at_zero` (R1 empty level, forced level); `tests/submit.rs` cases for "foliage level lists" and "foliage level starts"; shadow, thinning, conformance, walk and stalk_tests green
- the start buffer is judged last in `submit::fits`, after the scratch, so the counter and argument cases keep naming their own buffers
- friction: two entries in FRICTION.md (cargo fmt touching an unformatted base file; finalize refusing a zero-findings draw)

stage: impl-review - ran [2026-10-16T12:51Z..2026-10-16T12:55Z]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8fb9fa759e5d6ef5cd6e43c8e283c4bb8b58403b, 47805c6d1405c7bf500f7201d114b12a4caa0363
- Tests: cargo test --profile ci -p telperion-render --lib select, cargo test --profile ci -p telperion-render --test submit, cargo test --profile ci -p telperion-render --test shadow --test thinning --test conformance --test walk, cargo test --profile ci -p telperion-render --lib the_drawn_levels -- --ignored (before/after identical)
- PRs: